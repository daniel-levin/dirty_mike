use dirty_mike_core::{Observations, ObservationsOutOfBounds};

#[derive(Debug, Observations)]
struct S {
    #[raw(0xab)]
    x: u64,

    #[raw(0xcd)]
    y: u64,
}

#[test]
fn can_instantiate_from_measurements() {
    let s = S::new(&[50, 100]).unwrap();

    assert_eq!(s.x, 50);
    assert_eq!(s.y, 100);
}

#[test]
fn too_small() {
    assert!(matches!(
        S::new(&[]),
        Err(ObservationsOutOfBounds {
            received: 0,
            expected: 2
        })
    ));
}

#[test]
fn too_large() {
    assert!(matches!(
        S::new(&[1, 2, 3]),
        Err(ObservationsOutOfBounds {
            received: 3,
            expected: 2
        })
    ));
}

#[test]
fn exact_measurements() {
    let s = S::new(&[50, 100]).unwrap();
    assert_eq!(s.measurements(), [50, 100]);
}
