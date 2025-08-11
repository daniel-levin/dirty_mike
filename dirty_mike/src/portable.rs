use crate::Counter;

#[derive(Debug, Counter)]
pub struct BasicHardwareCounters {
    #[hardware(CPU_CYCLES)]
    pub cycles: usize,

    #[hardware(INSTRUCTIONS)]
    pub instructions: usize,

    #[hardware(BRANCH_MISSES)]
    pub branch_misses: usize,
}
