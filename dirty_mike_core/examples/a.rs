use dirty_mike_core::exact::*;

fn main() -> anyhow::Result<()> {
    let exact = ExactMeasurements::builder()
        .measure(0xc0) // ins retired
        .measure(0x3c) // cycles
        .build()?;

    let o = exact.measure(|| {})?;

    dbg!(o);

    Ok(())
}
