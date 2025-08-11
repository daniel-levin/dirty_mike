use perf_event::ReadFormat;
use perf_event::events::Hardware;
use perf_event::{Builder, Group};
use std::time::Duration;

pub mod portable;
pub mod x86;
pub use dirty_mike_derive::Counter;

#[derive(Debug)]
pub struct Topdown<T> {
    pub result: T,
    pub cpu_cycles: Option<u64>,
    pub instructions: Option<u64>,
    pub branch_misses: Option<u64>,
    pub cache_misses: Option<u64>,
    pub stalled_cycles_frontend: Option<u64>,
    pub stalled_cycles_backend: Option<u64>,

    pub time_enabled: Option<Duration>,
    pub time_running: Option<Duration>,
}

pub fn topdown<T, F: Fn() -> T>(f: F) -> std::io::Result<Topdown<T>> {
    let mut gb = Group::builder();
    gb.read_format(ReadFormat::all());

    let mut group = gb.build_group()?;

    let cpu_cycles = group.add(&Builder::new(Hardware::CPU_CYCLES)).ok();
    let instructions = group.add(&Builder::new(Hardware::INSTRUCTIONS)).ok();
    let branch_misses = group.add(&Builder::new(Hardware::BRANCH_MISSES)).ok();
    let cache_misses = group.add(&Builder::new(Hardware::CACHE_MISSES)).ok();
    let stalled_cycles_frontend = group
        .add(&Builder::new(Hardware::STALLED_CYCLES_FRONTEND))
        .ok();
    let stalled_cycles_backend = group
        .add(&Builder::new(Hardware::STALLED_CYCLES_BACKEND))
        .ok();

    group.enable()?;
    let result = f();
    group.disable()?;

    let counts = group.read()?;

    Ok(Topdown {
        result,
        cpu_cycles: cpu_cycles.and_then(|c| counts.get(&c).map(|entry| entry.value())),
        instructions: instructions.and_then(|c| counts.get(&c).map(|entry| entry.value())),
        branch_misses: branch_misses.and_then(|c| counts.get(&c).map(|entry| entry.value())),
        cache_misses: cache_misses.and_then(|c| counts.get(&c).map(|entry| entry.value())),
        stalled_cycles_frontend: stalled_cycles_frontend
            .and_then(|c| counts.get(&c).map(|entry| entry.value())),
        stalled_cycles_backend: stalled_cycles_backend
            .and_then(|c| counts.get(&c).map(|entry| entry.value())),
        time_running: counts.time_running(),
        time_enabled: counts.time_enabled(),
    })
}

#[derive(Debug)]
pub struct TopdownStats {
    pub frontend_bound_percentage: Option<f64>,
    pub backend_bound_percentage: Option<f64>,
    pub retiring_percentage: Option<f64>,
    pub bad_speculation_percentage: Option<f64>,
    pub instructions_per_cycle: Option<f64>,
}

impl<T> Topdown<T> {
    pub fn stats(&self) -> TopdownStats {
        TopdownStats {
            frontend_bound_percentage: match (self.cpu_cycles, self.stalled_cycles_frontend) {
                (Some(cycles), Some(frontend)) if cycles > 0 => {
                    Some((frontend as f64 / cycles as f64) * 100.0)
                }
                _ => None,
            },
            backend_bound_percentage: match (self.cpu_cycles, self.stalled_cycles_backend) {
                (Some(cycles), Some(backend)) if cycles > 0 => {
                    Some((backend as f64 / cycles as f64) * 100.0)
                }
                _ => None,
            },
            retiring_percentage: match (
                self.cpu_cycles,
                self.stalled_cycles_frontend,
                self.stalled_cycles_backend,
            ) {
                (Some(cycles), Some(frontend), Some(backend)) if cycles > 0 => {
                    let retiring_cycles = cycles.saturating_sub(frontend + backend);
                    Some((retiring_cycles as f64 / cycles as f64) * 100.0)
                }
                _ => None,
            },
            bad_speculation_percentage: match (self.cpu_cycles, self.branch_misses) {
                (Some(cycles), Some(misses)) if cycles > 0 => {
                    let approx_bad_speculation = misses * 10; // Rough penalty estimate
                    let bad_spec_cycles = approx_bad_speculation.min(cycles);
                    Some((bad_spec_cycles as f64 / cycles as f64) * 100.0)
                }
                _ => None,
            },
            instructions_per_cycle: match (self.cpu_cycles, self.instructions) {
                (Some(cycles), Some(instructions)) if cycles > 0 => {
                    Some(instructions as f64 / cycles as f64)
                }
                _ => None,
            },
        }
    }
}
