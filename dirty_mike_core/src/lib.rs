use polars::prelude::*;

pub mod exact;

pub mod pe2 {
    pub use perf_event::*;
}

pub use dirty_mike_derive::Observations;

#[derive(Debug)]
pub enum EventCode {
    Raw(u64),
    Hardware(u64),
}

impl EventCode {
    pub fn code(&self) -> u64 {
        match self {
            Self::Raw(c) | Self::Hardware(c) => *c,
        }
    }

    pub(crate) fn as_perf_event_builder(&self) -> pe2::Builder<'_> {
        match self {
            Self::Raw(c) => pe2::Builder::new(pe2::events::Raw::new(*c)),
            Self::Hardware(c) => pe2::Builder::new(pe2::events::Hardware(*c)),
        }
    }
}

#[derive(Debug)]
pub struct MeasurementDefinition {
    pub name: &'static str,
    pub code: EventCode,
}

pub trait Observations<const N: usize>: Sized + Send + Sync + 'static {
    fn new(observations: [u64; N]) -> Self;

    fn fields() -> &'static [MeasurementDefinition; N];

    fn measurements(&self) -> [u64; N];
}

#[derive(Debug)]
pub struct Experiment<const N: usize, Obs: Observations<N>, T> {
    pub results: Vec<T>,
    pub measurements: Vec<Obs>,
}
