use core::convert::TryFrom;

use mynd_math::MathError;

use crate::{Dimensions, GeometryError, ImageRect, PlaneLayout, plane::required_len_u64};

#[kani::proof]
fn dimensions_accept_exactly_nonzero_u8_axes() {
    let width = u32::from(kani::any::<u8>());
    let height = u32::from(kani::any::<u8>());
    match Dimensions::new(width, height) {
        Ok(dimensions) => {
            assert!(width != 0);
            assert!(height != 0);
            assert!(dimensions.width() == width);
            assert!(dimensions.height() == height);
            assert!(dimensions.pixel_count() == u64::from(width) * u64::from(height));
        }
        Err(GeometryError::ZeroWidth) => assert!(width == 0),
        Err(GeometryError::ZeroHeight) => assert!(width != 0 && height == 0),
        Err(_) => assert!(false),
    }
}

#[kani::proof]
fn pixel_count_matches_32_bit_address_model_for_all_u8_inputs() {
    let width = u32::from(kani::any::<u8>());
    let height = u32::from(kani::any::<u8>());
    if let Ok(dimensions) = Dimensions::new(width, height) {
        let fixed_width_fits = u32::try_from(dimensions.pixel_count()).is_ok();
        let target_model_fits = width.checked_mul(height).is_some();
        assert!(fixed_width_fits == target_model_fits);
    }
}

#[kani::proof]
fn pixel_count_32_bit_boundary_is_exact() {
    let exact = Dimensions::new(u16::MAX.into(), u16::MAX.into());
    let above = Dimensions::new(u32::MAX, 2);
    let maximum = Dimensions::new(u32::MAX, u32::MAX);
    match (exact, above, maximum) {
        (Ok(exact), Ok(above), Ok(maximum)) => {
            assert!(u32::try_from(exact.pixel_count()).is_ok());
            assert!(u32::try_from(above.pixel_count()).is_err());
            assert!(maximum.pixel_count() == u64::from(u32::MAX) * u64::from(u32::MAX));
        }
        _ => assert!(false),
    }
}

#[kani::proof]
fn image_rect_matches_reduced_containment_model() {
    let bound_width = u32::from(kani::any::<u8>());
    let bound_height = u32::from(kani::any::<u8>());
    let x = u32::from(kani::any::<u8>());
    let y = u32::from(kani::any::<u8>());
    let width = u32::from(kani::any::<u8>());
    let height = u32::from(kani::any::<u8>());
    let bounds = Dimensions::new(bound_width, bound_height);
    if let Ok(bounds) = bounds {
        let actual = ImageRect::new(bounds, x, y, width, height);
        let right = u64::from(x) + u64::from(width);
        let bottom = u64::from(y) + u64::from(height);
        let expected_success = width != 0
            && height != 0
            && right <= u64::from(bound_width)
            && bottom <= u64::from(bound_height);
        assert!(actual.is_ok() == expected_success);
        if let Ok(rectangle) = actual {
            assert!(u64::from(rectangle.right()) == right);
            assert!(u64::from(rectangle.bottom()) == bottom);
            assert!(rectangle.area() == u64::from(width) * u64::from(height));
        }
    }
}

#[kani::proof]
fn maximum_contained_rectangle_has_exact_ends() {
    let bounds = match Dimensions::new(u32::MAX, u32::MAX) {
        Ok(bounds) => bounds,
        Err(_) => return,
    };
    let rectangle = ImageRect::new(bounds, u32::MAX - 1, u32::MAX - 1, 1, 1);
    match rectangle {
        Ok(rectangle) => {
            assert!(rectangle.right() == u32::MAX);
            assert!(rectangle.bottom() == u32::MAX);
        }
        Err(_) => assert!(false),
    }
}

#[kani::proof]
fn plane_layout_matches_reduced_validation_model() {
    let offset = u64::from(kani::any::<u8>());
    let row_bytes = u64::from(kani::any::<u8>());
    let row_stride = u64::from(kani::any::<u8>());
    let rows = u32::from(kani::any::<u8>());
    let alignment = u64::from(kani::any::<u8>());
    let actual = PlaneLayout::new(offset, row_bytes, row_stride, rows, alignment);
    let expected = if row_bytes == 0 {
        Err(GeometryError::ZeroRowBytes)
    } else if rows == 0 {
        Err(GeometryError::ZeroRows)
    } else if alignment == 0 {
        Err(GeometryError::ZeroAlignment)
    } else if row_stride < row_bytes {
        Err(GeometryError::StrideTooSmall)
    } else if offset.checked_rem(alignment) != Some(0) {
        Err(GeometryError::OffsetMisaligned)
    } else if row_stride.checked_rem(alignment) != Some(0) {
        Err(GeometryError::StrideMisaligned)
    } else {
        let end = offset + row_stride * u64::from(rows - 1) + row_bytes;
        Ok(end)
    };
    match (actual, expected) {
        (Ok(layout), Ok(end)) => assert!(u64::try_from(layout.end().get()) == Ok(end)),
        (Err(actual), Err(expected)) => assert!(actual == expected),
        _ => assert!(false),
    }
}

#[kani::proof]
fn multirow_length_matches_32_bit_model_for_all_u8_inputs() {
    let offset = u32::from(kani::any::<u8>());
    let row_bytes = u32::from(kani::any::<u8>());
    let row_stride = u32::from(kani::any::<u8>());
    let rows = u32::from(kani::any::<u8>());
    let actual = required_len_u64(
        u64::from(offset),
        u64::from(row_bytes),
        u64::from(row_stride),
        rows,
    )
    .and_then(|end| {
        u32::try_from(end).map_err(|_| GeometryError::Arithmetic(MathError::ConversionOutOfRange))
    });
    let expected = required_len_u32_model(offset, row_bytes, row_stride, rows);
    assert!(actual == expected);
}

#[kani::proof]
fn one_row_length_matches_full_32_bit_usize_model() {
    let offset = kani::any::<u32>();
    let row_bytes = kani::any::<u32>();
    let actual = required_len_u64(
        u64::from(offset),
        u64::from(row_bytes),
        u64::from(row_bytes),
        1,
    )
    .and_then(|end| {
        u32::try_from(end).map_err(|_| GeometryError::Arithmetic(MathError::ConversionOutOfRange))
    });
    let expected = if row_bytes == 0 {
        Err(GeometryError::ZeroRowBytes)
    } else {
        offset
            .checked_add(row_bytes)
            .ok_or(GeometryError::Arithmetic(MathError::ConversionOutOfRange))
    };
    assert!(actual == expected);
}

#[kani::proof]
fn maximum_multirow_length_fails_the_32_bit_model() {
    let actual = required_len_u64(
        u64::from(u32::MAX),
        u64::from(u32::MAX),
        u64::from(u32::MAX),
        u32::MAX,
    )
    .and_then(|end| {
        u32::try_from(end).map_err(|_| GeometryError::Arithmetic(MathError::ConversionOutOfRange))
    });
    assert!(actual == Err(GeometryError::Arithmetic(MathError::ConversionOutOfRange)));
}

#[kani::proof]
fn last_row_32_bit_boundary_is_exact() {
    let exact = required_len_u64(0, 1, u64::from(u32::MAX - 1), 2).and_then(|end| {
        u32::try_from(end).map_err(|_| GeometryError::Arithmetic(MathError::ConversionOutOfRange))
    });
    let above = required_len_u64(0, 1, u64::from(u32::MAX), 2).and_then(|end| {
        u32::try_from(end).map_err(|_| GeometryError::Arithmetic(MathError::ConversionOutOfRange))
    });
    assert!(exact == Ok(u32::MAX));
    assert!(above == Err(GeometryError::Arithmetic(MathError::ConversionOutOfRange)));
}

fn required_len_u32_model(
    offset: u32,
    row_bytes: u32,
    row_stride: u32,
    rows: u32,
) -> Result<u32, GeometryError> {
    if row_bytes == 0 {
        return Err(GeometryError::ZeroRowBytes);
    }
    if rows == 0 {
        return Err(GeometryError::ZeroRows);
    }
    if row_stride < row_bytes {
        return Err(GeometryError::StrideTooSmall);
    }
    let overflow = || GeometryError::Arithmetic(MathError::ConversionOutOfRange);
    row_stride
        .checked_mul(rows - 1)
        .and_then(|prior| offset.checked_add(prior))
        .and_then(|last| last.checked_add(row_bytes))
        .ok_or_else(overflow)
}
