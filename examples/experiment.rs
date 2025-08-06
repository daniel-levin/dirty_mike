use perf_event::{Builder, events};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event = Builder::new(events::Software::CONTEXT_SWITCHES)
        .context_switch(true)
        .build()?;

    let mut sampled = event.sampled(128)?;

    sampled.enable()?;

    for i in 0..10 {
        println!("Waiting for event {}...", i);
        match sampled.next_blocking(Some(Duration::from_millis(500))) {
            Some(sample) => {
                println!(
                    "Got event {}! of type {} with size {}",
                    i,
                    sample.ty(),
                    sample.len()
                );
                match sample.parse_record() {
                    Ok(record) => println!("Event record: {:?}", record),
                    Err(e) => eprintln!("Failed to parse record: {}", e),
                }
            }
            None => {
                eprintln!("timeout");
            }
        }
    }

    sampled.disable()?;

    Ok(())
}
