use perf_event::{Builder, ReadFormat, SampleFlag, events, events::Tracepoint};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sampler = Builder::new(Tracepoint::with_name("block/block_rq_complete")?)
        .any_pid()
        .one_cpu(1)
        .sample_period(1)
        .build()?
        .sampled(8192)?;

    sampler.enable()?;

    for i in 0..100 {
        match sampler.next_blocking(Some(std::time::Duration::from_millis(500))) {
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
