use dirty_mike_derive::Counter;

#[derive(Counter)]
struct LifetimeStruct<'a> {
    field: &'a str,
}

fn main() {}