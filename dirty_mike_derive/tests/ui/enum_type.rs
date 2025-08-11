use dirty_mike_derive::Counter;

#[derive(Counter)]
enum MyEnum {
    Variant1,
    Variant2(u32),
}

fn main() {}