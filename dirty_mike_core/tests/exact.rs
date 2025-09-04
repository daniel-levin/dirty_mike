use dirty_mike_core::{Observations, exact::*};
use rand::prelude::SliceRandom;

#[derive(Debug, Observations)]
struct S {
    #[raw(0x13c)]
    cpu_clk_unhalted_ref_xclk: u64,

    #[raw(0x20010d)]
    int_misc_recovery_cycles_any: u64,
}

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
fn take_exact_measurements() {
    let data_size = 32_768_00;
    let mut shuffled_data: Vec<i32> = (0..data_size).collect();
    let mut rng = rand::rng();
    shuffled_data.shuffle(&mut rng);

    let mut m = ExactMeasurements::<_, S>::new().unwrap();
    m.enable().unwrap();
    work(&shuffled_data);
    m.disable().unwrap();

    let r = m.read().unwrap();

    dbg!(r);
}
