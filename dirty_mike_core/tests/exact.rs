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
fn test_builder() {
    // tma_frontend_bound on SKL

    let exact = ExactMeasurements::builder()
        .leader(0x13c)
        .follower(0x19c)
        .follower(0x23c)
        .follower(0x3c)
        .build()
        .unwrap();

    let mut shuffled_data: Vec<i32> = (0..320_768).collect();
    let mut rng = rand::rng();
    shuffled_data.shuffle(&mut rng);

    let handle = exact.start().unwrap();
    work(&shuffled_data);
    let measurements = handle.stop().unwrap();

    dbg!(measurements);
}
