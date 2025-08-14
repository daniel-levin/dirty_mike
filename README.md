# `dirty_mike` - quick and dirty self-profiling for Rust.

> [It's gonna be a nice evening.](https://www.youtube.com/watch?v=HEQTUSmob1Q)

## Elevator pitch
`dirty_mike` is just `perf` for almost anything you can pass to a Rust closure.

1. Profile only specific parts of your application.
2. Run experiments quickly.
2. Build custom combinations of measurements for specific purposes.

```rust
use dirty_mike::Counter;

#[derive(Debug, Counter)]
pub struct CyclesAndTime {
    #[raw(0x13C)]
    pub cpu_clk_thread_unhalted_ref_xclk: u64,

    #[time_running]
    pub running: Duration,

    #[time_enabled]
    pub enabled: Duration,
}

let (_, counts) = CyclesAndTime::measure(|| {
    let mut x = vec![];

    for i in 0..1000 {
        x.push(format!("{}", i));
    }
})
.unwrap();

assert!(counts.cpu_clk_thread_unhalted_ref_xclk > 2000);
assert!(counts.running.as_nanos() > 0);
assert!(counts.enabled.as_nanos() > 0);
}

```

## Branch mispredict example
See `dirty_mike/examples/branch.rs`.
This compares

### Sample Output (Non-release Mode)
```
 cargo r --example branch
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/branch`
Branch Prediction Performance Comparison
Data size: 32768 integers (131072 bytes)
Operation: Sum values >= 16384

Results:
  Sorted sum: 402644992
  Unsorted sum: 402644992
  Results match: true

Performance metrics:
Sorted data:
  cycles: 744886
  instructions: 1753432
  branch_misses: 26
  time running: 222.627µs
  cycles/element: 22.73
  instructions/element: 53.51
  branch_misses/element: 0.0008

Unsorted data:
  cycles: 1371755
  instructions: 1753432
  branch_misses: 16600
  time running: 398.203µs
  cycles/element: 41.86
  instructions/element: 53.51
  branch_misses/element: 0.5066

Comparison (Sorted vs Unsorted):
  cycles (sorted): 744886
  cycles (unsorted): 1371755
  cycle ratio (unsorted/sorted): 1.84x

  branch_misses (sorted): 26
  branch_misses (unsorted): 16600
  branch miss ratio (unsorted/sorted): 638.5x

  time (sorted): 222.627µs
  time (unsorted): 398.203µs
  time ratio (unsorted/sorted): 1.79x

  IPC (sorted): 2.35
  IPC (unsorted): 1.28

Analysis:
  Sorted data has 638.5x fewer branch misses
  Sorted data is 1.84x faster in cycles
  Sorted data is 1.79x faster in wall-clock time
  Branch prediction makes sorted data significantly more efficient
```

## Example
See `dirty_mike/examples/basic.rs`.
This compares `serde_json` and `toml`'s parsing of an equivalent payload.

### Sample Output (Release Mode)

```
Data sizes:
  JSON: 5458 bytes
  TOML: 5222 bytes

Performance metrics:
JSON parsing:
  cycles/b: 22.68
  instructions/b: 35.53
  total cycles: 123772
  total instructions: 193915
  time running: 54.366us
  time enabled: 54.366us

TOML parsing:
  cycles/b: 80.86
  instructions/b: 126.45
  total cycles: 422265
  total instructions: 660299
  time running: 173.791us
  time enabled: 173.791us

Comparison (JSON vs TOML):
  cycles/b (JSON): 22.68
  cycles/b (TOML): 80.86
  instructions/b (JSON): 35.53
  instructions/b (TOML): 126.45
  time running (JSON): 54.366us
  time running (TOML): 173.79us
  IPC (JSON): 1.57
  IPC (TOML): 1.56
```
