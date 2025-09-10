use crate::Observation;

#[derive(Debug, Observation)]
pub struct Ipc {
    #[hardware(REF_CPU_CYCLES)]
    pub ref_cpu_cycles: u64,

    #[hardware(INSTRUCTIONS)]
    pub instructions: u64,
}

impl Ipc {
    pub fn ipc(&self) -> f64 {
        self.instructions as f64 / self.ref_cpu_cycles as f64
    }
}

#[derive(Debug, Observation)]
pub struct Basic {
    #[hardware(REF_CPU_CYCLES)]
    pub ref_cpu_cycles: u64,

    #[hardware(INSTRUCTIONS)]
    pub instructions_retired: u64,

    #[hardware(BRANCH_MISSES)]
    pub branch_misses: u64,

    #[hardware(CACHE_MISSES)]
    pub cache_misses: u64,
}
