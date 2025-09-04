use crate::Observations;
use perf_event::{Builder, Counter, Group, ReadFormat, SampleFlag, events::Raw};
use std::{io, marker::PhantomData, sync::Arc, time::Duration};
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
    CannotDisable(#[source] io::Error),

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
}

/*
#[derive(Debug)]
pub struct ExactMeasurementsReading {
    pub time_running: Duration,
    pub time_enabled: Duration,
    pub counts: Vec<u64>,
}

#[derive(Debug, Default)]
pub struct ExactMeasurementsBuilder {
    raw_codes: Vec<u64>,
}

impl ExactMeasurementsBuilder {
    pub fn measure(mut self, raw_code: u64) -> Self {
        self.raw_codes.push(raw_code);
        self
    }

    pub fn build(self) -> Result<ExactMeasurements, ExactMeasurementsBuildError> {
        if self.raw_codes.is_empty() {
            return Err(ExactMeasurementsBuildError::NoDefinedEvents);
        }

        let rf = ReadFormat::TOTAL_TIME_ENABLED
            | ReadFormat::TOTAL_TIME_RUNNING
            | ReadFormat::ID
            | ReadFormat::GROUP;

        let leader_raw_code = self.raw_codes[0];

        let mut leader = Builder::new(Raw::new(leader_raw_code))
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
            .map_err(ExactMeasurementsBuildError::CannotCreateLeader)?;

        let mut followers = vec![];

        for raw_follower_code in &self.raw_codes[1..] {
            let mut fb = Builder::new(Raw::new(*raw_follower_code));
            let fb = fb
                .inherit(true)
                .exclude_kernel(true)
                .exclude_hv(true)
                .exclude_guest(true)
                .sample(SampleFlag::IDENTIFIER)
                .read_format(rf);

            fb.attrs_mut().set_disabled(0);

            let follower = leader.add(fb).map_err(|e| {
                ExactMeasurementsBuildError::CannotAttachFollower(*raw_follower_code, e)
            })?;

            followers.push(follower);
        }

        Ok(ExactMeasurements {
            leader: Box::new(leader),
            followers,
        })
    }
}
*/
