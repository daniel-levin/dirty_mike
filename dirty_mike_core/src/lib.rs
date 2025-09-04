use thiserror::Error;

pub mod exact;

pub mod pe2 {
    pub use perf_event::*;
}

pub use dirty_mike_derive::Observations;

#[derive(Debug)]
pub struct MeasurementDefinition {
    pub name: &'static str,
}

#[derive(Debug, Error)]
#[error("observations out of bounds. ({received:}) received ({expected:}) expected")]
pub struct ObservationsOutOfBounds {
    pub received: usize,
    pub expected: usize,
}

pub trait Observations<const N: usize>: Sized + Send + Sync + 'static {
    fn new(observations: &[u64]) -> Result<Self, ObservationsOutOfBounds>;

    //fn fields() -> &'static [&'static MeasurementDefinition];

    fn measurements(&self) -> [u64; N];
}
