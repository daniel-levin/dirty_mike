use perf_event::{Builder, events, Group, ReadFormat};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tracepoint = events::Tracepoint::with_name("sched/sched_switch")?;
    let cpu_cycles = events::Hardware::INSTRUCTIONS;

    let mut group = Group::new()?;

    let cc = group.add(&Builder::new(cpu_cycles))?;

    group.enable()?;
    println!("poes");
    group.disable()?;

    let counts = group.read()?;

    dbg!(counts);

    Ok(())
}
