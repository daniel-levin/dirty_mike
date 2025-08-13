use crate::Counter;

#[derive(Debug, Counter)]
pub struct BasicHardwareCounters {
    #[hardware(CPU_CYCLES)]
    pub cycles: u64,

    #[hardware(INSTRUCTIONS)]
    pub instructions: u64,

    #[hardware(CACHE_MISSES)]
    pub cache_misses: u64,

    #[hardware(BRANCH_INSTRUCTIONS)]
    pub branch_instructions: u64,

    #[hardware(BRANCH_MISSES)]
    pub branch_misses: u64,

    #[hardware(REF_CPU_CYCLES)]
    pub ref_cpu_cycles: u64,
}
