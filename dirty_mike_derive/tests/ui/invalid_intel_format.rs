use dirty_mike_derive::Counter;

#[derive(Counter)]
struct InvalidIntelFormat {
    #[intel(0x89)]
    counter: u64,
}

fn main() {}