//! Facade re-export coverage for the v0.5.0 storage and checked-math surface.

use mynd::core::{
    AlphaAssociation, Dimensions, ImageRect, PixelLayout, PlaneLayout, SampleClass, SampleStorage,
    StorageUnit, checked_plane_output_len,
};
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

#[test]
fn facade_reexports_validated_pixel_storage() -> Result<(), Box<dyn std::error::Error>> {
    let dimensions = Dimensions::new(2, 2)?;
    let sample = SampleStorage::new(SampleClass::UnsignedInteger, 8, StorageUnit::Byte)?;
    let layout = PixelLayout::RgbaPlanar {
        color: sample,
        alpha_sample: sample,
        alpha: AlphaAssociation::Straight,
    };
    let planes = [
        PlaneLayout::new(0, 2, 2, 2, 1)?,
        PlaneLayout::new(4, 2, 2, 2, 1)?,
        PlaneLayout::new(8, 2, 2, 2, 1)?,
        PlaneLayout::new(12, 2, 2, 2, 1)?,
    ];
    assert_eq!(layout.validate_planes(dimensions, &planes)?.get(), 16);
    Ok(())
}
