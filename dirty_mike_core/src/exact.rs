use perf_event::Builder;
use perf_event::Counter;
use perf_event::Group;
use perf_event::ReadFormat;
use perf_event::SampleFlag;
use perf_event::events::Raw;
use std::io;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExactMeasurementsBuildError {
    #[error("cannot create group leader")]
    CannotCreateLeader(#[from] std::io::Error),

    #[error("cannot attach follower 0x{0:x} to group leader")]
    CannotAttachFollower(u64, #[source] std::io::Error),

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
}

#[derive(derive_more::Debug)]
pub struct ExactMeasurements {
    #[debug("opaque")]
    leader: Box<Group>,
    followers: Vec<Counter>,
}

impl ExactMeasurements {
    pub fn builder() -> ExactMeasurementsBuilder {
        ExactMeasurementsBuilder::default()
    }

    pub fn start(mut self) -> Result<ExactMeasurementsDropGuard, ExactMeasurementsError> {
        self.leader
            .enable()
            .map_err(ExactMeasurementsError::CannotEnableGroup)?;
        Ok(ExactMeasurementsDropGuard {
            leader: self.leader,
            followers: self.followers,
        })
    }

    pub fn measure<T, F: FnOnce() -> T>(
        self,
        f: F,
    ) -> Result<(T, ExactMeasurementsReading), ExactMeasurementsError> {
        let dg = self.start()?;
        let result = f();
        let measurements = dg.stop()?;
        Ok((result, measurements))
    }
}

#[derive(derive_more::Debug)]
pub struct ExactMeasurementsDropGuard {
    #[debug("opaque")]
    leader: Box<Group>,
    followers: Vec<Counter>,
}

#[derive(Debug)]
pub struct ExactMeasurementsReading {
    pub time_running: Duration,
    pub time_enabled: Duration,
    pub counts: Vec<u64>,
}

impl ExactMeasurementsDropGuard {
    pub fn stop(mut self) -> Result<ExactMeasurementsReading, ExactMeasurementsError> {
        self.leader
            .disable()
            .map_err(ExactMeasurementsError::CannotDisable)?;
        let measurements = self
            .leader
            .read()
            .map_err(ExactMeasurementsError::CannotReadGroup)?;

        let leader_val = measurements[&self.leader.as_counter()];
        let mut counts = vec![leader_val];

        for f in self.followers.iter() {
            counts.push(measurements[f]);
        }

        Ok(ExactMeasurementsReading {
            counts,
            time_running: measurements.time_running().unwrap(),
            time_enabled: measurements.time_enabled().unwrap(),
        })
    }
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
            .exclude_kernel(false)
            .exclude_hv(false)
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
                .exclude_kernel(false)
                .exclude_hv(false)
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
