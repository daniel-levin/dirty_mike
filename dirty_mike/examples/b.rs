use dirty_mike::{exact::ExactMeasurements, intel::skl::*};
use polars::prelude::*;
use toml::Table;

fn main() {
    let s = include_str!("../Cargo.toml");

    let m = ExactMeasurements::<BadSpeculation, _>::measure_k(250, |_| {
        || {
            let _f = s.parse::<Table>().unwrap();
        }
    })
    .unwrap();

    let df = m.transpose();

    let lf = df.lazy().mean().collect();

    dbg!(lf);
}
