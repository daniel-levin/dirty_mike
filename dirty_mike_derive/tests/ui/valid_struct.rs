use dirty_mike_derive::Counter;

#[derive(Counter)]
struct MyCounter {
    #[hardware(INSTRUCTIONS)]
    instructions: u64,
    #[hardware(CACHE_MISSES)]
    cache_misses: u64,
    cycles: u64, // No attribute, should use default CPU_CYCLES
}

fn main() {}