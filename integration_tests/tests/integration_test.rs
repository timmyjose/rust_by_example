extern crate integration_tests;

#[test]
fn test_add() {
    assert_eq!(integration_tests::add(1, 2), 3);
}