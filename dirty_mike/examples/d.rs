use core::arch::x86_64::*;
use dirty_mike::{exact::ExactMeasurements, intel::skl::*, portable};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = include_str!("../../dirty_mike_core/examples/page.txt");

    let m1 = ExactMeasurements::<BadSpeculation, _>::measure_k(2500, |_| || {}).unwrap();

    let m2 = ExactMeasurements::<BadSpeculation, _>::measure_k(2500, |_| || {}).unwrap();

    dbg!(m1.p_timeslice(0f64)?);

    dbg!(m2.p_timeslice(0f64)?);
    dbg!(m2.p(0f64)?);
    dbg!(m2.p_timeslice(0.5f64)?);
    dbg!(m2.p(0.5f64)?);
    dbg!(m2.p_timeslice(1f64)?);
    dbg!(m2.p(1f64)?);

    Ok(())
}
