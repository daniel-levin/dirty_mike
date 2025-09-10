use num_bigint::BigUint;
use std::time::Duration;
use tabled::Tabled;
use thiserror::Error;

pub mod exact;

pub mod pe2 {
    pub use perf_event::*;
}

pub use dirty_mike_derive::Observation;

#[derive(Debug, Tabled)]
pub struct Timeslice {
    #[tabled(format = "{:#?}")]
    pub enabled: Duration,
    #[tabled(format = "{:#?}")]
    pub running: Duration,
}

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

#[derive(Debug, Error)]
#[error("invalid fraction {0}")]
pub struct InvalidFraction(pub f64);

#[derive(Debug)]
pub struct Experiment<const N: usize, Obs: Observation<N>, T> {
    pub results: Vec<T>,
    pub timeslices: Vec<Timeslice>,
    pub measurements: Vec<Obs>,
}

impl<const N: usize, Obs: Observation<N>, T> Experiment<N, Obs, T> {
    pub fn mean(&self) -> Obs {
        let mut counters = [BigUint::ZERO; N];

        for m in self.measurements.iter() {
            for (i, field_value) in m.measurements().iter().enumerate() {
                counters[i] += *field_value;
            }
        }

        for i in 0..N {
            counters[i] = counters[i].clone() / self.measurements.len();
        }

        Obs::new(counters.map(|c| c.try_into().unwrap()))
    }

    pub fn p(&self, fraction: f64) -> Result<(Timeslice, Obs), InvalidFraction> {
        Ok((self.p_timeslice(fraction)?, self.p_obs(fraction)?))
    }

    pub fn p_obs(&self, fraction: f64) -> Result<Obs, InvalidFraction> {
        if 0f64 <= fraction && fraction <= 1f64 {
            let pivot = (fraction * (self.measurements.len() - 1) as f64) as usize;

            let mut columns = [const { Vec::new() }; N];

            for m in self.measurements.iter() {
                for (i, u) in m.measurements().iter().enumerate() {
                    columns[i].push(*u);
                }
            }

            Ok(Obs::new(columns.map(|mut c| {
                c.sort();
                c[pivot]
            })))
        } else {
            Err(InvalidFraction(fraction))
        }
    }

    pub fn p_timeslice(&self, fraction: f64) -> Result<Timeslice, InvalidFraction> {
        if 0f64 <= fraction && fraction <= 1f64 {
            let pivot = (fraction * (self.measurements.len() - 1) as f64) as usize;

            let mut enableds = vec![];
            let mut runnings = vec![];

            for Timeslice { enabled, running } in self.timeslices.iter() {
                enableds.push(enabled);
                runnings.push(running);
            }

            enableds.sort();
            runnings.sort();

            Ok(Timeslice {
                enabled: *enableds[pivot],
                running: *runnings[pivot],
            })
        } else {
            Err(InvalidFraction(fraction))
        }
    }

    pub fn mean_timeslice(&self) -> Timeslice {
        let mut enabled_sum = BigUint::ZERO;
        let mut running_sum = BigUint::ZERO;

        for Timeslice { enabled, running } in self.timeslices.iter() {
            enabled_sum += enabled.as_nanos();
            running_sum += running.as_nanos();
        }

        let enabled: u64 = (enabled_sum / self.timeslices.len()).try_into().unwrap();
        let running: u64 = (running_sum / self.timeslices.len()).try_into().unwrap();

        Timeslice {
            enabled: Duration::from_nanos(enabled),
            running: Duration::from_nanos(running),
        }
    }
}

#[derive(Debug, Tabled)]
pub struct TabulatedObservation<'a, const N: usize, Obs: Observation<N> + Tabled> {
    #[tabled(inline)]
    pub timeslice: &'a Timeslice,

    #[tabled(inline)]
    pub measurement: &'a Obs,
}

impl<const N: usize, Obs: Observation<N> + Tabled, T> Experiment<N, Obs, T> {
    pub fn as_table(&self) -> tabled::Table {
        let mut tos = vec![];

        for (i, timeslice) in self.timeslices.iter().enumerate() {
            let measurement = &self.measurements[i];
            tos.push(TabulatedObservation {
                timeslice,
                measurement,
            });
        }

        tabled::Table::new(tos)
    }
}
