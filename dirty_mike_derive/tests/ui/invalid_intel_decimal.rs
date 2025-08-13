use dirty_mike_derive::Counter;

#[derive(Counter)]
struct InvalidIntelDecimal {
    #[intel(137, 255)]
    counter: u64,
}

fn main() {}