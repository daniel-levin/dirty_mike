use dirty_mike::{exact::ExactMeasurements, intel::skl::*};

fn main() {
    let s = include_str!("../Cargo.toml");

    let m1 = ExactMeasurements::<BranchMispredicts, _>::measure_k(25_000, |_| {
        || {
            let mut a = vec![];
            for c in s.chars() {
                if c != 'g' {
                    a.push(c);
                }
            }
        }
    })
    .unwrap();

    dbg!(m1.mean_timeslice());
    dbg!(m1.mean());

    let m2 = ExactMeasurements::<BranchMispredicts, _>::measure_k(25_000, |_| {
        || {
            let mut a = vec![];
            for (i, c) in s.chars().enumerate() {
                if c != 'g' {
                    a.push(c);
                }
            }
        }
    })
    .unwrap();

    dbg!(m2.mean_timeslice());
    dbg!(m2.mean());
}
