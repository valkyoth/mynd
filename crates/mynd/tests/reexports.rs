//! Facade re-export coverage for the v0.3.0 checked-math surface.

use mynd::math::{MathError, checked_add_u64, checked_range_u64};

#[test]
fn facade_reexports_checked_math() {
    assert_eq!(checked_add_u64(20, 22), Ok(42));
    assert_eq!(checked_range_u64(2, 4, 6), Ok(2..6));
    assert_eq!(checked_range_u64(2, 5, 6), Err(MathError::RangeOutOfBounds));
}
