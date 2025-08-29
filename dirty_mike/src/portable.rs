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

#[cfg(test)]
mod tests {
    use crate::intel::skl::*;
    use rand::prelude::SliceRandom;

    #[test]
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

        let mut shuffled_data: Vec<i32> = (0..32_768).collect();
        let mut rng = rand::rng();
        shuffled_data.shuffle(&mut rng);

        let mut sorted_data = shuffled_data.clone();
        sorted_data.sort();

        let (_, sorted_res) = BadSpeculation::measure(|| work(&sorted_data)).unwrap();

        let (_, unsorted_res) = BadSpeculation::measure(|| work(&shuffled_data)).unwrap();

        dbg!(sorted_res);
        dbg!(unsorted_res);

        //assert!(sorted_res.branch_misses * 100 < unsorted_res.branch_misses);
    }
}
