use crate::Observations;

#[derive(Debug, Observations)]
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
