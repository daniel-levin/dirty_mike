use dirty_mike_core::exact::*;

#[test]
fn test_builder() {
    let eb = ExactMeasurements::builder()
        .leader(0x13c)
        .follower(0x20010d)
        .follower(0x23c)
        .follower(0x3c)
        .build();

    assert!(eb.is_ok());
}
