use dirty_mike_derive::Counter;

#[derive(Counter)]
struct InvalidHardware {
    #[hardware(INVALID_EVENT)]
    counter: u64,
}

fn main() {}