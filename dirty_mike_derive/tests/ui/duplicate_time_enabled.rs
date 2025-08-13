use dirty_mike_derive::Counter;

#[derive(Counter)]
struct DuplicateTimeEnabled {
    #[time_enabled]
    enabled1: std::time::Duration,

    #[time_enabled]
    enabled2: std::time::Duration,
}

fn main() {}