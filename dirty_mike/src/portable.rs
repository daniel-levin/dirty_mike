use crate::Counter;

#[derive(Debug, Counter)]
pub struct BasicHardwareCounters {
    #[hardware(CPU_CYCLES)]
    pub cycles: u64,

    #[hardware(INSTRUCTIONS)]
    pub instructions: u64,

    #[hardware(BRANCH_MISSES)]
    pub branch_misses: u64,

    #[hardware(BRANCH_INSTRUCTIONS)]
    pub branch_instructions: u64,

    #[hardware(CACHE_MISSES)]
    pub cache_misses: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirty_mike_core::Counter;
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

        let mut shuffled_data: Vec<i32> = (0..32768).collect();
        let mut rng = rand::rng();
        shuffled_data.shuffle(&mut rng);

        let mut sorted_data = shuffled_data.clone();
        sorted_data.sort();

        let _sorted_res = BasicHardwareCounters::measure(|| work(&sorted_data)).unwrap();

        let _unsorted_res = BasicHardwareCounters::measure(|| work(&shuffled_data)).unwrap();
    }
}
