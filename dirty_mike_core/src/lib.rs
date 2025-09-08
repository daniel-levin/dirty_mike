pub mod exact;

pub mod pe2 {
    pub use perf_event::*;
}

pub use dirty_mike_derive::Observation;

/// The kind of PMC to schedule on the core.
#[derive(Debug)]
pub enum EventCode {
    /// Model-dependent codes.
    Raw(u64),

    /// Architecture-specific codes that normally refer to "fixed counters".
    Hardware(u64),
}

impl EventCode {
    /// The raw unsigned integer we're going to pass to `perf_event_open`.
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

/// Definition of a PMC which may be scheduled onto the core.
#[derive(Debug)]
pub struct MeasurementDefinition {
    pub name: &'static str,
    pub code: EventCode,
}

/// Normally automatically implemented with the [dirty_mike_derive::Observation] macro.
/// This defines a set of measurements which should be taken together in order to form a single,
/// coherent observation.
///
/// ```rust
/// #[derive(Debug, dirty_mike_core::Observation)]
/// pub struct MispredictsByCycle {
///     #[hardware(CPU_CYCLES)]
///     cycles: u64,
///
///     #[hardware(BRANCH_MISSES)]
///     branch_misses: u64,
///
///     #[raw(0x10e)]
///     pub uops_issued: u64,
/// }
/// ```
pub trait Observation<const N: usize>: Sized + Send + Sync + 'static {
    /// Create a single observation comprised of N measurements.
    fn new(measurements: [u64; N]) -> Self;

    /// Defines the meaning of this set of measurements when taken together.
    fn fields() -> &'static [MeasurementDefinition; N];

    /// The N measurements comprising this observation.
    fn measurements(&self) -> [u64; N];
}

#[derive(Debug)]
pub struct Experiment<const N: usize, Obs: Observation<N>, T> {
    pub results: Vec<T>,
    pub measurements: Vec<Obs>,
}

impl<const N: usize, Obs: Observation<N>, T> Experiment<N, Obs, T> {}
