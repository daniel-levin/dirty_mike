use perf_event::Builder;
use perf_event::Counter;
use perf_event::Group;
use perf_event::ReadFormat;
use perf_event::SampleFlag;
use perf_event::events::Raw;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExactMeasurementsBuildError {
    #[error("cannot create group leader")]
    CannotCreateLeader(#[from] std::io::Error),

    #[error("cannot attach follower {0} to group leader")]
    CannotAttachFollower(u64, #[source] std::io::Error),
}

#[derive(derive_more::Debug)]
pub struct ExactMeasurements {
    #[debug("opaque")]
    leader: Group,
    followers: Vec<Counter>,
}

impl ExactMeasurements {
    pub fn builder() -> ExactMeasurementsBuilder {
        ExactMeasurementsBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ExactMeasurementsBuilder {
    leader_raw_code: u64,
    followers: Vec<u64>,
}

impl ExactMeasurementsBuilder {
    pub fn new(leader_raw_code: u64) -> Self {
        Self {
            leader_raw_code,
            followers: vec![],
        }
    }

    pub fn leader(mut self, leader_raw_code: u64) -> Self {
        self.leader_raw_code = leader_raw_code;
        self
    }

    pub fn follower(mut self, raw_follower_code: u64) -> Self {
        self.followers.push(raw_follower_code);
        self
    }

    pub fn build(self) -> Result<ExactMeasurements, ExactMeasurementsBuildError> {
        let rf = ReadFormat::TOTAL_TIME_ENABLED
            | ReadFormat::TOTAL_TIME_RUNNING
            | ReadFormat::ID
            | ReadFormat::GROUP;

        let mut leader = Builder::new(Raw::new(self.leader_raw_code))
            .sample(SampleFlag::IDENTIFIER)
            .read_format(rf)
            .enable_on_exec(true)
            .exclude_kernel(false)
            .exclude_hv(false)
            .exclude_guest(true)
            .inherit(true)
            .build_group()
            .map_err(ExactMeasurementsBuildError::CannotCreateLeader)?;

        let mut followers = vec![];

        for raw_follower_code in self.followers {
            let mut fb = Builder::new(Raw::new(raw_follower_code));
            let fb = fb
                .inherit(true)
                .exclude_kernel(false)
                .exclude_hv(false)
                .sample(SampleFlag::IDENTIFIER)
                .read_format(rf);

            fb.attrs_mut().set_disabled(0);

            let follower = leader.add(fb).map_err(|e| {
                ExactMeasurementsBuildError::CannotAttachFollower(raw_follower_code, e)
            })?;

            followers.push(follower);
        }

        Ok(ExactMeasurements { leader, followers })
    }
}
