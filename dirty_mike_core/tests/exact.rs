use dirty_mike_core::{Observations, exact::*};

#[derive(Debug, Observations)]
struct S {
    #[hardware(CPU_CYCLES)]
    cpu_cycles: u64,

    #[hardware(BRANCH_MISSES)]
    branch_misses: u64,
}

#[test]
fn take_exact_measurements() {
    let m = ExactMeasurements::<_, S>::new();
}
