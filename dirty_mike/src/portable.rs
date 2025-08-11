use crate::Counter;

#[derive(Debug, Counter)]
pub struct BasicHardwareCounters {
    #[hardware(CPU_CYCLES)]
    pub cycles: usize,

    #[hardware(INSTRUCTIONS)]
    pub instructions: usize,
}
