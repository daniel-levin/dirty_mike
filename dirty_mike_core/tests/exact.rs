use dirty_mike_core::{Observation, exact::*};
use rand::prelude::SliceRandom;

#[derive(Debug, Observation)]
struct S {
    #[hardware(CPU_CYCLES)]
    cpu_cycles: u64,

    #[raw(0x20010d)]
    int_misc_recovery_cycles_any: u64,

    #[raw(0x2c2)]
    uops_retired_retire_slots: u64,

    #[raw(0x10e)]
    uops_issued_any: u64,
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
#[cfg_attr(feature = "ci", ignore = "requires perf_event_open capabilities")]
fn take_exact_measurements() {
    let data_size = 32_768_00;
    let mut shuffled_data: Vec<i32> = (0..data_size).collect();
    let mut rng = rand::rng();
    shuffled_data.shuffle(&mut rng);

    let obs = ExactMeasurements::<S, _>::measure_k(10, |_| {
        || {
            work(&shuffled_data);
        }
    })
    .unwrap();

    dbg!(obs);
}
