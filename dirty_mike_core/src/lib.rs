pub mod exact;

pub mod pe2 {
    pub use perf_event::*;
}

pub use dirty_mike_derive::{ExactCounter, Observations};

#[derive(Debug)]
pub struct MeasurementDefinition {
    pub name: &'static str,
}

pub trait Observations: Send + Sync + 'static {
    fn new(observations: &[u64]) -> Self;

    //fn fields() -> &'static [&'static MeasurementDefinition];

    //fn measurements(&self) -> &[u64];
}

pub trait Measurements: Send + Sync + 'static {
    fn from_observations(observations: &[u64]) -> Self;
}
