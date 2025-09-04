use crate::Observations;
use perf_event::{Builder, Counter, ReadFormat, SampleFlag, events::Raw};
use std::{io, marker::PhantomData, sync::Arc};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ExactMeasurementsBuildError {
    #[error("cannot create group leader")]
    CannotCreateLeader(#[source] Arc<std::io::Error>),

    #[error("cannot attach follower 0x{0:x} to group leader")]
    CannotAttachFollower(u64, #[source] Arc<std::io::Error>),

    #[error("no events defined")]
    NoDefinedEvents,
}

#[derive(Debug, Error)]
pub enum ExactMeasurementsError {
    #[error("cannot take reading from group")]
    CannotReadGroup(#[source] io::Error),

    #[error("cannot disable group")]
    CannotDisableGroup(#[source] io::Error),

    #[error("cannot enable group")]
    CannotEnableGroup(#[source] io::Error),

    #[error(transparent)]
    CannotBuildGroup(#[from] ExactMeasurementsBuildError),
}

#[derive(Debug)]
pub struct ExactMeasurements<const N: usize, Obs: Observations<N>> {
    counters: [Counter; N],
    _pd: PhantomData<Obs>,
}

impl<const N: usize, Obs: Observations<N>> ExactMeasurements<N, Obs> {
    pub fn new() -> Result<Self, ExactMeasurementsBuildError> {
        let rf = ReadFormat::TOTAL_TIME_ENABLED
            | ReadFormat::TOTAL_TIME_RUNNING
            | ReadFormat::ID
            | ReadFormat::GROUP;

        let fields = Obs::fields();

        let mut leader = Builder::new(Raw::new(fields[0].code))
            .sample(SampleFlag::IDENTIFIER)
            .read_format(rf)
            .enable_on_exec(true)
            .exclude_kernel(true)
            .exclude_hv(true)
            .exclude_guest(true)
            .inherit(true)
            .pinned(true)
            .exclusive(true)
            .build_group()
            .map_err(Arc::new)
            .map_err(ExactMeasurementsBuildError::CannotCreateLeader)?;

        let mut counters: [Result<Option<Counter>, ExactMeasurementsBuildError>; N] =
            std::array::from_fn(|i| {
                if i == 0 {
                    Ok(None)
                } else {
                    let raw_follower_code = fields[i].code;
                    let mut fb = Builder::new(Raw::new(raw_follower_code));
                    let fb = fb
                        .inherit(true)
                        .exclude_kernel(true)
                        .exclude_hv(true)
                        .exclude_guest(true)
                        .sample(SampleFlag::IDENTIFIER)
                        .read_format(rf);

                    fb.attrs_mut().set_disabled(0);

                    let follower = leader.add(fb).map_err(|e| {
                        ExactMeasurementsBuildError::CannotAttachFollower(
                            raw_follower_code,
                            Arc::new(e),
                        )
                    })?;

                    Ok(Some(follower))
                }
            });

        counters[0] = Ok(Some(leader.into_counter()));

        for maybe_faulted in &counters {
            if let Err(e) = maybe_faulted {
                return Err(e.clone());
            }
        }

        Ok(Self {
            counters: counters.map(|c| c.unwrap().unwrap()),
            _pd: PhantomData,
        })
    }

    pub fn enable(&mut self) -> io::Result<()> {
        self.counters[0].enable_group()
    }

    pub fn disable(&mut self) -> io::Result<()> {
        self.counters[0].disable_group()
    }

    pub fn read(&mut self) -> io::Result<Obs> {
        let mut readings = [0; N];

        let counts = self.counters[0].read_group()?;

        for i in 0..N {
            readings[i] = counts[&self.counters[i]];
        }

        Ok(Obs::new(readings))
    }

    pub fn measure<T, F: FnOnce() -> T>(f: F) -> Result<(T, Obs), ExactMeasurementsError> {
        let mut me = Self::new().map_err(ExactMeasurementsError::CannotBuildGroup)?;

        me.enable()
            .map_err(ExactMeasurementsError::CannotEnableGroup)?;
        let t = f();
        me.disable()
            .map_err(ExactMeasurementsError::CannotDisableGroup)?;
        let obs = me.read().map_err(ExactMeasurementsError::CannotReadGroup)?;

        Ok((t, obs))
    }

    pub fn measure_k<T, G: Fn(usize) -> F, F: FnOnce() -> T>(
        k: usize,
        g: G,
    ) -> Result<Vec<(T, Obs)>, ExactMeasurementsError> {
        (0..k).map(|i| Self::measure(g(i))).collect()
    }
}
