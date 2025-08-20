use dirty_mike::Counter;
use fxhash::FxHashMap;
use std::time::Duration;

#[derive(Debug, Counter, Default)]
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

fn benchmark_array_scan_int(data: &[(i32, String)], target: i32) -> BasicHardwareCounters {
    let (result, counters) = BasicHardwareCounters::measure(|| {
        let mut result = None;
        for (key, value) in data {
            if *key == target {
                result = Some(value.clone());
                break;
            }
        }
        result
    }).unwrap();
    
    std::hint::black_box(result);
    counters
}

fn benchmark_hashmap_lookup_int(data: &FxHashMap<i32, String>, target: i32) -> BasicHardwareCounters {
    let (result, counters) = BasicHardwareCounters::measure(|| {
        data.get(&target).cloned()
    }).unwrap();
    
    std::hint::black_box(result);
    counters
}

fn benchmark_array_scan_string(data: &[(String, String)], target: &str) -> BasicHardwareCounters {
    let (result, counters) = BasicHardwareCounters::measure(|| {
        let mut result = None;
        for (key, value) in data {
            if key == target {
                result = Some(value.clone());
                break;
            }
        }
        result
    }).unwrap();
    
    std::hint::black_box(result);
    counters
}

fn benchmark_binary_search_int(data: &[(i32, String)], target: i32) -> BasicHardwareCounters {
    let (result, counters) = BasicHardwareCounters::measure(|| {
        data.binary_search_by(|(key, _)| key.cmp(&target))
            .ok()
            .map(|idx| data[idx].1.clone())
    }).unwrap();
    
    std::hint::black_box(result);
    counters
}

fn benchmark_hashmap_lookup_string(data: &FxHashMap<String, String>, target: &str) -> BasicHardwareCounters {
    let (result, counters) = BasicHardwareCounters::measure(|| {
        data.get(target).cloned()
    }).unwrap();
    
    std::hint::black_box(result);
    counters
}

fn benchmark_binary_search_string(data: &[(String, String)], target: &str) -> BasicHardwareCounters {
    let (result, counters) = BasicHardwareCounters::measure(|| {
        data.binary_search_by(|(key, _)| key.as_str().cmp(target))
            .ok()
            .map(|idx| data[idx].1.clone())
    }).unwrap();
    
    std::hint::black_box(result);
    counters
}

fn main() {
    const ARRAY_SIZE: usize = 1_200_000;
    const NUM_ITERATIONS: usize = 1000;
    
    // Integer key benchmarks
    let int_array: Vec<(i32, String)> = (0..ARRAY_SIZE as i32).map(|i| (i, format!("value_{}", i * 2))).collect();
    let int_sorted_array = int_array.clone(); // Already sorted since we generate 0..ARRAY_SIZE
    let int_hashmap: FxHashMap<i32, String> = int_array.iter().cloned().collect();
    
    // String key benchmarks
    let mut string_array: Vec<(String, String)> = (0..ARRAY_SIZE)
        .map(|i| (format!("key_{:06}", i), format!("value_{}", i))) // Zero-padded for proper string sorting
        .collect();
    string_array.sort_by(|a, b| a.0.cmp(&b.0));
    let string_sorted_array = string_array.clone();
    let string_hashmap: FxHashMap<String, String> = string_array.iter().cloned().collect();
    
    // Test with targets at different positions (beginning, middle, end)
    let test_positions = [0, ARRAY_SIZE / 2, ARRAY_SIZE - 1];
    
    println!("Benchmarking Array Scan vs Binary Search vs FxHashMap Lookup");
    println!("Array size: {}, Iterations per test: {}", ARRAY_SIZE, NUM_ITERATIONS);
    println!();
    
    for &pos in &test_positions {
        println!("=== Target at position {} ===", pos);
        
        // Integer benchmarks
        let int_target = pos as i32;
        let mut array_scan_total = BasicHardwareCounters::default();
        let mut binary_search_total = BasicHardwareCounters::default();
        let mut hashmap_lookup_total = BasicHardwareCounters::default();
        
        for _ in 0..NUM_ITERATIONS {
            let array_result = benchmark_array_scan_int(&int_array, int_target);
            array_scan_total.cycles += array_result.cycles;
            array_scan_total.instructions += array_result.instructions;
            array_scan_total.branch_misses += array_result.branch_misses;
            array_scan_total.running += array_result.running;
            
            let binary_result = benchmark_binary_search_int(&int_sorted_array, int_target);
            binary_search_total.cycles += binary_result.cycles;
            binary_search_total.instructions += binary_result.instructions;
            binary_search_total.branch_misses += binary_result.branch_misses;
            binary_search_total.running += binary_result.running;
            
            let hashmap_result = benchmark_hashmap_lookup_int(&int_hashmap, int_target);
            hashmap_lookup_total.cycles += hashmap_result.cycles;
            hashmap_lookup_total.instructions += hashmap_result.instructions;
            hashmap_lookup_total.branch_misses += hashmap_result.branch_misses;
            hashmap_lookup_total.running += hashmap_result.running;
        }
        
        println!("Integer Keys:");
        println!("  Array Scan      - Cycles: {:.1}, Instructions: {:.1}, Branch Misses: {:.1}, Time: {:?}",
                 array_scan_total.cycles as f64 / NUM_ITERATIONS as f64,
                 array_scan_total.instructions as f64 / NUM_ITERATIONS as f64,
                 array_scan_total.branch_misses as f64 / NUM_ITERATIONS as f64,
                 array_scan_total.running / NUM_ITERATIONS as u32);
        println!("  Binary Search   - Cycles: {:.1}, Instructions: {:.1}, Branch Misses: {:.1}, Time: {:?}",
                 binary_search_total.cycles as f64 / NUM_ITERATIONS as f64,
                 binary_search_total.instructions as f64 / NUM_ITERATIONS as f64,
                 binary_search_total.branch_misses as f64 / NUM_ITERATIONS as f64,
                 binary_search_total.running / NUM_ITERATIONS as u32);
        println!("  FxHashMap Lookup - Cycles: {:.1}, Instructions: {:.1}, Branch Misses: {:.1}, Time: {:?}",
                 hashmap_lookup_total.cycles as f64 / NUM_ITERATIONS as f64,
                 hashmap_lookup_total.instructions as f64 / NUM_ITERATIONS as f64,
                 hashmap_lookup_total.branch_misses as f64 / NUM_ITERATIONS as f64,
                 hashmap_lookup_total.running / NUM_ITERATIONS as u32);
        
        // String benchmarks
        let string_target = format!("key_{:06}", pos);
        let mut string_array_scan_total = BasicHardwareCounters::default();
        let mut string_binary_search_total = BasicHardwareCounters::default();
        let mut string_hashmap_lookup_total = BasicHardwareCounters::default();
        
        for _ in 0..NUM_ITERATIONS {
            let array_result = benchmark_array_scan_string(&string_array, &string_target);
            string_array_scan_total.cycles += array_result.cycles;
            string_array_scan_total.instructions += array_result.instructions;
            string_array_scan_total.branch_misses += array_result.branch_misses;
            string_array_scan_total.running += array_result.running;
            
            let binary_result = benchmark_binary_search_string(&string_sorted_array, &string_target);
            string_binary_search_total.cycles += binary_result.cycles;
            string_binary_search_total.instructions += binary_result.instructions;
            string_binary_search_total.branch_misses += binary_result.branch_misses;
            string_binary_search_total.running += binary_result.running;
            
            let hashmap_result = benchmark_hashmap_lookup_string(&string_hashmap, &string_target);
            string_hashmap_lookup_total.cycles += hashmap_result.cycles;
            string_hashmap_lookup_total.instructions += hashmap_result.instructions;
            string_hashmap_lookup_total.branch_misses += hashmap_result.branch_misses;
            string_hashmap_lookup_total.running += hashmap_result.running;
        }
        
        println!("String Keys:");
        println!("  Array Scan      - Cycles: {:.1}, Instructions: {:.1}, Branch Misses: {:.1}, Time: {:?}",
                 string_array_scan_total.cycles as f64 / NUM_ITERATIONS as f64,
                 string_array_scan_total.instructions as f64 / NUM_ITERATIONS as f64,
                 string_array_scan_total.branch_misses as f64 / NUM_ITERATIONS as f64,
                 string_array_scan_total.running / NUM_ITERATIONS as u32);
        println!("  Binary Search   - Cycles: {:.1}, Instructions: {:.1}, Branch Misses: {:.1}, Time: {:?}",
                 string_binary_search_total.cycles as f64 / NUM_ITERATIONS as f64,
                 string_binary_search_total.instructions as f64 / NUM_ITERATIONS as f64,
                 string_binary_search_total.branch_misses as f64 / NUM_ITERATIONS as f64,
                 string_binary_search_total.running / NUM_ITERATIONS as u32);
        println!("  FxHashMap Lookup - Cycles: {:.1}, Instructions: {:.1}, Branch Misses: {:.1}, Time: {:?}",
                 string_hashmap_lookup_total.cycles as f64 / NUM_ITERATIONS as f64,
                 string_hashmap_lookup_total.instructions as f64 / NUM_ITERATIONS as f64,
                 string_hashmap_lookup_total.branch_misses as f64 / NUM_ITERATIONS as f64,
                 string_hashmap_lookup_total.running / NUM_ITERATIONS as u32);
        
        println!();
    }
}
