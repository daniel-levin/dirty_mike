use perf_event::{Builder, events};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tracepoint = events::Tracepoint::with_name("sched/sched_switch")?;

    // Create a single event (not group) with sampling
    let event = Builder::new(tracepoint).context_switch(true).build()?;
    let mut sampled = event.sampled(128)?; // Create sampled version with 128-entry buffer
    
    sampled.enable()?;
    println!("Monitoring sched_switch events...");

    // Create work in a separate thread to generate context switches
    let work_handle = thread::spawn(|| {
        for i in 0..5 {
            println!("Worker thread iteration: {}", i);
            thread::sleep(Duration::from_millis(100));
            thread::yield_now();
        }
    });
    work_handle.join().unwrap();

    // Read events from the main thread
    for i in 0..10 {
        println!("Waiting for event {}...", i);
        
        // Use a timeout to avoid blocking forever
        match sampled.next_blocking(Some(Duration::from_millis(500))) {
            Some(sample) => {
                println!("Got event {}!", i);
                match sample.parse_record() {
                    Ok(record) => println!("Event record: {:?}", record),
                    Err(e) => println!("Failed to parse record: {}", e),
                }
            }
            None => {
                println!("Error or timeout waiting for event");
            }
        }
    }

    sampled.disable()?;
    
    println!("Done monitoring");
    Ok(())
}
