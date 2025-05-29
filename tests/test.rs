use rust_crash_course::*;

use rust_crash_course::target;

// #[cfg(feature = "exercises")]
// use rust_crash_course::exercises as target;

#[test]
fn test_foo() {
    assert_eq!(target::foo(), 1);
}

/*
#[cfg(feature = "solutions")]
#[test]
fn test_bar() {
    solutions::foo();
}
*/
