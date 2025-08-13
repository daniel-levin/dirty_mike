use dirty_mike_derive::Counter;

#[derive(Counter)]
struct DuplicateTimeRunning {
    #[time_running]
    running1: std::time::Duration,

    #[time_running]
    running2: std::time::Duration,
}

fn main() {}