use perf_event::{Builder, events};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event = Builder::new(events::Software::CONTEXT_SWITCHES)
        .context_switch(true)
        .build()?;
    let mut sampled = event.sampled(128)?;

    sampled.enable()?;
    println!("Monitoring sched_switch events...");

    for i in 0..10 {
        println!("Waiting for event {}...", i);

        match sampled.next_record() {
            Some(sample) => {
                let x = sample.to_vec();
                println!(
                    "Got event {}! of type {} with size {}",
                    i,
                    sample.ty(),
                    x.len()
                );
                match sample.parse_record() {
                    Ok(record) => println!("Event record: {:?}", record),
                    Err(e) => println!("Failed to parse record: {}", e),
                }
            }
            None => {
                println!("Error or timeout waiting for event");
            }
        }
        thread::sleep(Duration::from_millis(100));
    }

    sampled.disable()?;

    println!("Done monitoring");
    Ok(())
}
