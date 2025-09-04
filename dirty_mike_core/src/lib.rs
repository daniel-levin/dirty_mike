pub mod exact;

pub mod pe2 {
    pub use perf_event::*;
}

pub use dirty_mike_derive::Observations;

#[derive(Debug)]
pub struct MeasurementDefinition {
    pub name: &'static str,
    pub code: u64,
}

pub trait Observations<const N: usize>: Sized + Send + Sync + 'static {
    fn new(observations: [u64; N]) -> Self;

    fn fields() -> &'static [MeasurementDefinition];

    fn measurements(&self) -> [u64; N];
}
