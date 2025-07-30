use perf_event::events::Hardware;
use perf_event::{Builder, Group};

#[derive(Debug)]
pub struct Topdown<T> {
    pub result: T,
    pub cpu_cycles: u64,
    pub instructions: u64,
    pub branch_misses: u64,
    pub cache_misses: u64,
    pub stalled_cycles_frontend: u64,
    pub stalled_cycles_backend: u64,
}

pub fn topdown<T, F: Fn() -> T>(f: F) -> std::io::Result<Topdown<T>> {
    let mut group = Group::new()?;
    let cpu_cycles = group.add(&Builder::new(Hardware::CPU_CYCLES))?;
    let instructions = group.add(&Builder::new(Hardware::INSTRUCTIONS))?;
    let branch_misses = group.add(&Builder::new(Hardware::BRANCH_MISSES))?;
    let cache_misses = group.add(&Builder::new(Hardware::CACHE_MISSES))?;
    // Try to add stalled cycles counters, fallback to 0 if not available
    let stalled_cycles_frontend = group
        .add(&Builder::new(Hardware::STALLED_CYCLES_FRONTEND))
        .unwrap_or_else(|_| {
            group.add(&Builder::new(Hardware::CPU_CYCLES)).unwrap() // Dummy counter
        });
    let stalled_cycles_backend = group
        .add(&Builder::new(Hardware::STALLED_CYCLES_BACKEND))
        .unwrap_or_else(|_| {
            group.add(&Builder::new(Hardware::CPU_CYCLES)).unwrap() // Dummy counter
        });

    group.enable()?;
    let result = f();
    group.disable()?;

    let counts = group.read()?;

    Ok(Topdown {
        result,
        cpu_cycles: counts[&cpu_cycles],
        instructions: counts[&instructions],
        branch_misses: counts[&branch_misses],
        cache_misses: counts[&cache_misses],
        stalled_cycles_frontend: counts
            .get(&stalled_cycles_frontend)
            .map(|ge| ge.value())
            .unwrap_or(0),
        stalled_cycles_backend: counts
            .get(&stalled_cycles_backend)
            .map(|ge| ge.value())
            .unwrap_or(0),
    })
}

impl<T> Topdown<T> {
    pub fn frontend_bound_percentage(&self) -> f64 {
        if self.cpu_cycles == 0 {
            return 0.0;
        }
        (self.stalled_cycles_frontend as f64 / self.cpu_cycles as f64) * 100.0
    }

    pub fn backend_bound_percentage(&self) -> f64 {
        if self.cpu_cycles == 0 {
            return 0.0;
        }
        (self.stalled_cycles_backend as f64 / self.cpu_cycles as f64) * 100.0
    }

    pub fn retiring_percentage(&self) -> f64 {
        if self.cpu_cycles == 0 {
            return 0.0;
        }
        let retiring_cycles = self
            .cpu_cycles
            .saturating_sub(self.stalled_cycles_frontend + self.stalled_cycles_backend);
        (retiring_cycles as f64 / self.cpu_cycles as f64) * 100.0
    }

    pub fn bad_speculation_percentage(&self) -> f64 {
        if self.cpu_cycles == 0 {
            return 0.0;
        }
        // Approximate bad speculation as branch misses impact
        let approx_bad_speculation = self.branch_misses * 10; // Rough penalty estimate
        let bad_spec_cycles = approx_bad_speculation.min(self.cpu_cycles);
        (bad_spec_cycles as f64 / self.cpu_cycles as f64) * 100.0
    }

    pub fn instructions_per_cycle(&self) -> f64 {
        if self.cpu_cycles == 0 {
            return 0.0;
        }
        self.instructions as f64 / self.cpu_cycles as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::SliceRandom;

    #[test]
    fn measure_branch_misses() {
        fn work(data: &[i32]) -> u64 {
            let mut sum = 0u64;
            for &value in data {
                if value >= 16384 {
                    sum = sum.wrapping_add(value as u64);
                }
            }
            sum
        }

        let mut shuffled_data: Vec<i32> = (0..32768).collect();
        let mut rng = rand::rng();
        shuffled_data.shuffle(&mut rng);

        let mut sorted_data = shuffled_data.clone();
        sorted_data.sort();

        let sorted_res = topdown(move || {
            work(&sorted_data);
        })
        .unwrap();

        let unsorted_res = topdown(move || {
            work(&shuffled_data);
        })
        .unwrap();

        assert!(unsorted_res.branch_misses > sorted_res.branch_misses * 100);
    }
}
