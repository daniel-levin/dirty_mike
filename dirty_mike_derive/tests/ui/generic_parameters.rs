use dirty_mike_derive::Counter;

#[derive(Counter)]
struct GenericStruct<T> {
    field: T,
}

fn main() {}