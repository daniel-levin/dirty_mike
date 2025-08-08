use perf_event::{Builder, events};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*let tp = events::Tracepoint::with_name("block/block_rq_complete")?;*/
    let tp = events::Hardware::CPU_CYCLES;

    let mut sampler = Builder::new(tp)
        .sample_period(1_000)
        .build()?
        .sampled(8192)?;

    sampler.enable()?;

    for i in 0..100 {
        match sampler.next_blocking(None) {
            Some(sample) => match sample.parse_record() {
                Ok(record) => {
                    println!("Event record {i}: {:#?}", &record);
                }
                Err(e) => eprintln!("Failed to parse record: {}", e),
            },
            None => {
                eprintln!("timeout");
            }
        }
    }

    sampler.disable()?;

    Ok(())
}
