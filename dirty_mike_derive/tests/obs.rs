use dirty_mike_core::Observations;

#[derive(Debug, Observations)]
struct S {
    #[raw(0xab)]
    x: u64,

    #[raw(0xcd)]
    y: u64,
}

#[test]
fn can_instantiate_from_measurements() {
    let s = S::new(&[50, 100]);

    assert_eq!(s.x, 50);
    assert_eq!(s.y, 100);
}
