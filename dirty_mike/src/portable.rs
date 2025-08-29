use crate::Counter;
use std::time::Duration;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intel::skl::*;
    use dirty_mike_core::Counter;
    use rand::prelude::SliceRandom;

    #[test]
    #[cfg_attr(feature = "ci", ignore = "requires perf_event_open capabilities")]
    fn measure_branch_misses() {
        fn work(data: &[i32]) -> u64 {
            let mut sum = 0u64;
            for &value in data {
                if value >= 16384 {
                    sum = sum.wrapping_add(value as u64);
                }
            }
            sum
        }

        let mut shuffled_data: Vec<i32> = (0..32768).collect();
        let mut rng = rand::rng();
        shuffled_data.shuffle(&mut rng);

        let mut sorted_data = shuffled_data.clone();
        sorted_data.sort();

        let (_, sorted_res) = MachineClears::measure(|| work(&sorted_data)).unwrap();

        let (_, unsorted_res) = BasicHardwareCounters::measure(|| work(&shuffled_data)).unwrap();

        dbg!(sorted_res);
        dbg!(unsorted_res);

        //assert!(sorted_res.branch_misses * 100 < unsorted_res.branch_misses);
    }
}
