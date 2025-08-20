use dirty_mike::Counter;
use rand::prelude::SliceRandom;
use std::time::Duration;

#[derive(Debug, Counter)]
pub struct BasicHardwareCounters {
    #[hardware(CPU_CYCLES)]
    pub cycles: u64,

    #[hardware(INSTRUCTIONS)]
    pub instructions: u64,

    #[hardware(BRANCH_MISSES)]
    pub branch_misses: u64,

    #[time_running]
    pub running: Duration,
}

#[inline(never)]
fn add_up_only_some_values(data: &[i32]) -> u64 {
    let mut sum = 0u64;
    for &value in data {
        if value >= 16384 {
            sum = sum.wrapping_add(value as u64);
        }
    }
    sum
}

fn main() {
    let data_size = 32768;
    let mut shuffled_data: Vec<i32> = (0..data_size).collect();
    let mut rng = rand::rng();
    shuffled_data.shuffle(&mut rng);

    let mut sorted_data = shuffled_data.clone();
    sorted_data.sort();

    println!("Branch Prediction Performance Comparison");
    println!(
        "Data size: {} integers ({} bytes)",
        data_size,
        data_size * 4
    );
    println!("Operation: Sum values >= 16384");
    println!();

    let (sorted_result, sorted_counters) =
        BasicHardwareCounters::measure(|| add_up_only_some_values(&sorted_data)).unwrap();

    let (unsorted_result, unsorted_counters) =
        BasicHardwareCounters::measure(|| add_up_only_some_values(&shuffled_data)).unwrap();

    println!("Results:");
    println!("  Sorted sum: {}", sorted_result);
    println!("  Unsorted sum: {}", unsorted_result);
    println!("  Results match: {}", sorted_result == unsorted_result);
    println!();

    println!("Performance metrics:");
    println!("Sorted data:");
    println!("  cycles: {}", sorted_counters.cycles);
    println!("  instructions: {}", sorted_counters.instructions);
    println!("  branch_misses: {}", sorted_counters.branch_misses);
    println!("  time running: {:?}", sorted_counters.running);
    println!(
        "  cycles/element: {:.2}",
        sorted_counters.cycles as f64 / data_size as f64
    );
    println!(
        "  instructions/element: {:.2}",
        sorted_counters.instructions as f64 / data_size as f64
    );
    println!(
        "  branch_misses/element: {:.4}",
        sorted_counters.branch_misses as f64 / data_size as f64
    );
    println!();

    println!("Unsorted data:");
    println!("  cycles: {}", unsorted_counters.cycles);
    println!("  instructions: {}", unsorted_counters.instructions);
    println!("  branch_misses: {}", unsorted_counters.branch_misses);
    println!("  time running: {:?}", unsorted_counters.running);
    println!(
        "  cycles/element: {:.2}",
        unsorted_counters.cycles as f64 / data_size as f64
    );
    println!(
        "  instructions/element: {:.2}",
        unsorted_counters.instructions as f64 / data_size as f64
    );
    println!(
        "  branch_misses/element: {:.4}",
        unsorted_counters.branch_misses as f64 / data_size as f64
    );
    println!();

    println!("Comparison (Sorted vs Unsorted):");
    let cycle_ratio = unsorted_counters.cycles as f64 / sorted_counters.cycles as f64;
    let branch_miss_ratio =
        unsorted_counters.branch_misses as f64 / sorted_counters.branch_misses as f64;
    let time_ratio =
        unsorted_counters.running.as_nanos() as f64 / sorted_counters.running.as_nanos() as f64;

    println!("  cycles (sorted): {}", sorted_counters.cycles);
    println!("  cycles (unsorted): {}", unsorted_counters.cycles);
    println!("  cycle ratio (unsorted/sorted): {:.2}x", cycle_ratio);
    println!();
    println!(
        "  branch_misses (sorted): {}",
        sorted_counters.branch_misses
    );
    println!(
        "  branch_misses (unsorted): {}",
        unsorted_counters.branch_misses
    );
    println!(
        "  branch miss ratio (unsorted/sorted): {:.1}x",
        branch_miss_ratio
    );
    println!();
    println!("  time (sorted): {:?}", sorted_counters.running);
    println!("  time (unsorted): {:?}", unsorted_counters.running);
    println!("  time ratio (unsorted/sorted): {:.2}x", time_ratio);
    println!();

    let sorted_ipc = sorted_counters.instructions as f64 / sorted_counters.cycles as f64;
    let unsorted_ipc = unsorted_counters.instructions as f64 / unsorted_counters.cycles as f64;
    println!("  IPC (sorted): {:.2}", sorted_ipc);
    println!("  IPC (unsorted): {:.2}", unsorted_ipc);

    println!();
    println!("Analysis:");
    println!(
        "  Sorted data has {:.1}x fewer branch misses",
        branch_miss_ratio
    );
    println!("  Sorted data is {:.2}x faster in cycles", cycle_ratio);
    println!(
        "  Sorted data is {:.2}x faster in wall-clock time",
        time_ratio
    );
    println!("  Branch prediction makes sorted data significantly more efficient");
}
