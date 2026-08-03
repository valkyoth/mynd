//! Facade re-export coverage for the v0.4.0 geometry and checked-math surface.

use mynd::core::{Dimensions, ImageRect, PlaneLayout, checked_plane_output_len};
use mynd::math::{MathError, checked_add_u64, checked_range_u64};

#[test]
fn facade_reexports_checked_math() {
    assert_eq!(checked_add_u64(20, 22), Ok(42));
    assert_eq!(checked_range_u64(2, 4, 6), Ok(2..6));
    assert_eq!(checked_range_u64(2, 5, 6), Err(MathError::RangeOutOfBounds));
}

#[test]
fn facade_reexports_validated_geometry() -> Result<(), mynd::core::GeometryError> {
    let dimensions = Dimensions::new(640, 480)?;
    let rectangle = ImageRect::new(dimensions, 10, 20, 30, 40)?;
    let plane = PlaneLayout::new(0, 640, 640, 480, 1)?;

    assert_eq!(rectangle.area(), 1_200);
    assert_eq!(
        checked_plane_output_len(&[plane]).map(|length| length.get()),
        Ok(307_200)
    );
    Ok(())
}
