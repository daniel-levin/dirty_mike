use dirty_mike_derive::Counter;

#[derive(Counter)]
union MyUnion {
    field1: u32,
    field2: f32,
}

fn main() {}