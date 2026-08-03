//! Boundary tests for dimensions and contained rectangles.

use mynd_core::{Dimensions, GeometryError, ImageRect};

#[test]
fn dimensions_reject_each_zero_axis() {
    assert_eq!(Dimensions::new(0, 1), Err(GeometryError::ZeroWidth));
    assert_eq!(Dimensions::new(1, 0), Err(GeometryError::ZeroHeight));
    assert_eq!(Dimensions::new(0, 0), Err(GeometryError::ZeroWidth));
}

#[test]
fn minimum_and_maximum_dimensions_have_exact_pixel_counts() {
    let minimum = Dimensions::new(1, 1);
    assert_eq!(minimum.map(Dimensions::width), Ok(1));
    assert_eq!(minimum.map(Dimensions::height), Ok(1));
    assert_eq!(minimum.map(Dimensions::pixel_count), Ok(1));
    assert_eq!(
        Dimensions::new(u32::MAX, u32::MAX).map(Dimensions::pixel_count),
        Ok(u64::from(u32::MAX) * u64::from(u32::MAX))
    );
}

#[test]
fn pixel_count_conversion_is_target_width_aware() {
    let dimensions = Dimensions::new(u32::MAX, u32::MAX);
    assert!(dimensions.is_ok());
    let Ok(dimensions) = dimensions else {
        return;
    };
    #[cfg(target_pointer_width = "64")]
    assert!(dimensions.pixel_count_usize().is_ok());
    #[cfg(target_pointer_width = "32")]
    assert_eq!(
        dimensions.pixel_count_usize(),
        Err(GeometryError::Arithmetic(
            mynd_math::MathError::ConversionOutOfRange
        ))
    );
}

#[test]
fn full_image_and_last_pixel_rectangles_are_contained() {
    let bounds = Dimensions::new(8, 6);
    assert!(bounds.is_ok());
    let Ok(bounds) = bounds else {
        return;
    };
    let full = ImageRect::new(bounds, 0, 0, 8, 6);
    assert_eq!(full.map(ImageRect::bounds), Ok(bounds));
    assert_eq!(full.map(ImageRect::x), Ok(0));
    assert_eq!(full.map(ImageRect::y), Ok(0));
    assert_eq!(full.map(ImageRect::width), Ok(8));
    assert_eq!(full.map(ImageRect::height), Ok(6));
    assert_eq!(full.map(ImageRect::area), Ok(48));
    assert_eq!(full.and_then(ImageRect::area_usize), Ok(48));
    let last = ImageRect::new(bounds, 7, 5, 1, 1);
    assert_eq!(last.map(ImageRect::right), Ok(8));
    assert_eq!(last.map(ImageRect::bottom), Ok(6));
}

#[test]
fn rectangles_reject_zero_axes_and_each_out_of_bounds_end() {
    let bounds = Dimensions::new(8, 6);
    assert!(bounds.is_ok());
    let Ok(bounds) = bounds else {
        return;
    };
    assert_eq!(
        ImageRect::new(bounds, 0, 0, 0, 1),
        Err(GeometryError::ZeroRectangleWidth)
    );
    assert_eq!(
        ImageRect::new(bounds, 0, 0, 1, 0),
        Err(GeometryError::ZeroRectangleHeight)
    );
    assert_eq!(
        ImageRect::new(bounds, 8, 0, 1, 1),
        Err(GeometryError::RectangleOutOfBounds)
    );
    assert_eq!(
        ImageRect::new(bounds, 0, 6, 1, 1),
        Err(GeometryError::RectangleOutOfBounds)
    );
}

#[test]
fn maximum_coordinate_edges_do_not_wrap() {
    let bounds = Dimensions::new(u32::MAX, u32::MAX);
    assert!(bounds.is_ok());
    let Ok(bounds) = bounds else {
        return;
    };
    let last = ImageRect::new(bounds, u32::MAX - 1, u32::MAX - 1, 1, 1);
    assert_eq!(last.map(ImageRect::right), Ok(u32::MAX));
    assert_eq!(last.map(ImageRect::bottom), Ok(u32::MAX));
    assert_eq!(
        ImageRect::new(bounds, u32::MAX, 0, 1, 1),
        Err(GeometryError::RectangleOutOfBounds)
    );
}

#[test]
fn geometry_constructors_are_const_usable() {
    const DIMENSIONS: Result<Dimensions, GeometryError> = Dimensions::new(3, 2);
    const RECTANGLE: Result<ImageRect, GeometryError> = match DIMENSIONS {
        Ok(dimensions) => ImageRect::new(dimensions, 1, 0, 2, 2),
        Err(error) => Err(error),
    };
    assert_eq!(DIMENSIONS.map(Dimensions::pixel_count), Ok(6));
    assert_eq!(RECTANGLE.map(ImageRect::area), Ok(4));
}
