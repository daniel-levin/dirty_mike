use perf_event::{Builder, Group, events};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let group = Group::builder().context_switch(true).build_group()?;

    let ctr = group.into_counter();

    let mut sampled = ctr.sampled(128)?;

    sampled.enable()?;

    for i in 0..10 {
        println!("Waiting for event {}...", i);
        match sampled.next_blocking(Some(std::time::Duration::from_millis(500))) {
            Some(sample) => {
                println!("got sample of type {}", sample.ty());

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
