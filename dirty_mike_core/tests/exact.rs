use dirty_mike_core::exact::*;
use rand::prelude::SliceRandom;

fn work(data: &[i32]) -> u64 {
    let mut sum = 0u64;
    for &value in data {
        if value >= 16384 {
            sum = sum.wrapping_add(value as u64);
        }
    }
    sum
}

#[test]
fn test_builder() -> anyhow::Result<()> {
    // tma_frontend_bound on SKL
    // 0x13c = CPU_CLK_THREAD_UNHALTED.REF_XCLK
    // 0x19c = IDQ_UOPS_NOT_DELIVERED.CORE
    // 0x23c = CPU_CLK_THREAD_UNHALTED.ONE_THREAD_ACTIVE
    // 0x3c  = CPU_CLK_UNHALTED.THREAD_P_ANY
    let exact = ExactMeasurements::builder()
        .measure(0x13c)
        .measure(0x19c)
        .measure(0x23c)
        .measure(0x3c)
        .build()?;

    let mut shuffled_data: Vec<i32> = (0..320_768).collect();
    let mut rng = rand::rng();
    shuffled_data.shuffle(&mut rng);

    let mut sorted_data = shuffled_data.clone();
    sorted_data.sort();

    let handle = exact.start().unwrap();
    work(&shuffled_data);
    let measurements = handle.stop().unwrap();

    let exact2 = ExactMeasurements::builder()
        .measure(0x13c)
        .measure(0x19c)
        .measure(0x23c)
        .measure(0x3c)
        .build()?;

    let handle2 = exact2.start().unwrap();
    work(&sorted_data);
    let measurements2 = handle2.stop().unwrap();

    dbg!(measurements);
    dbg!(measurements2);

    Ok(())
}
