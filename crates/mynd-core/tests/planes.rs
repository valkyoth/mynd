//! Boundary tests for row strides, plane extents, and output lengths.

use mynd_core::{GeometryError, PlaneLayout, checked_plane_output_len};
use mynd_math::{ArithmeticOperation, MathError};

#[test]
fn one_row_uses_no_trailing_stride_padding() {
    let layout = PlaneLayout::new(8, 6, 8, 1, 4);
    assert_eq!(layout.map(PlaneLayout::offset), Ok(8));
    assert_eq!(layout.map(PlaneLayout::row_bytes), Ok(6));
    assert_eq!(layout.map(PlaneLayout::row_stride), Ok(8));
    assert_eq!(layout.map(PlaneLayout::rows), Ok(1));
    assert_eq!(layout.map(PlaneLayout::alignment), Ok(4));
    assert_eq!(layout.map(|layout| layout.end().get()), Ok(14));
}

#[test]
fn last_row_extent_uses_stride_only_between_rows() {
    let layout = PlaneLayout::new(8, 6, 8, 3, 4);
    assert_eq!(layout.map(|layout| layout.end().get()), Ok(30));
}

#[test]
fn plane_rejects_zero_fields_before_arithmetic() {
    assert_eq!(
        PlaneLayout::new(0, 0, 1, 1, 1),
        Err(GeometryError::ZeroRowBytes)
    );
    assert_eq!(
        PlaneLayout::new(0, 1, 1, 0, 1),
        Err(GeometryError::ZeroRows)
    );
    assert_eq!(
        PlaneLayout::new(0, 1, 1, 1, 0),
        Err(GeometryError::ZeroAlignment)
    );
}

#[test]
fn plane_rejects_short_or_misaligned_strides_and_offsets() {
    assert_eq!(
        PlaneLayout::new(0, 5, 4, 1, 1),
        Err(GeometryError::StrideTooSmall)
    );
    assert_eq!(
        PlaneLayout::new(2, 4, 4, 1, 4),
        Err(GeometryError::OffsetMisaligned)
    );
    assert_eq!(
        PlaneLayout::new(0, 4, 6, 1, 4),
        Err(GeometryError::StrideMisaligned)
    );
}

#[test]
fn plane_reports_last_row_arithmetic_overflow() {
    assert_eq!(
        PlaneLayout::new(0, 1, u64::MAX, 2, 1),
        Err(GeometryError::Arithmetic(MathError::Overflow {
            operation: ArithmeticOperation::Add,
        }))
    );
}

#[test]
fn plane_set_accepts_adjacency_and_gaps() {
    let first = PlaneLayout::new(0, 4, 4, 2, 1);
    let adjacent = PlaneLayout::new(8, 2, 2, 2, 1);
    let gap = PlaneLayout::new(16, 3, 3, 1, 1);
    assert!(first.is_ok() && adjacent.is_ok() && gap.is_ok());
    let (Ok(first), Ok(adjacent), Ok(gap)) = (first, adjacent, gap) else {
        return;
    };
    assert_eq!(
        checked_plane_output_len(&[first, adjacent]).map(|length| length.get()),
        Ok(12)
    );
    assert_eq!(
        checked_plane_output_len(&[first, gap]).map(|length| length.get()),
        Ok(19)
    );
}

#[test]
fn plane_set_rejects_empty_overlap_and_out_of_order_inputs() {
    assert_eq!(checked_plane_output_len(&[]), Err(GeometryError::NoPlanes));
    let first = PlaneLayout::new(8, 4, 4, 2, 1);
    let overlap = PlaneLayout::new(10, 2, 2, 1, 1);
    let earlier = PlaneLayout::new(4, 2, 2, 1, 1);
    assert!(first.is_ok() && overlap.is_ok() && earlier.is_ok());
    let (Ok(first), Ok(overlap), Ok(earlier)) = (first, overlap, earlier) else {
        return;
    };
    assert_eq!(
        checked_plane_output_len(&[first, overlap]),
        Err(GeometryError::PlaneOverlap)
    );
    assert_eq!(
        checked_plane_output_len(&[first, earlier]),
        Err(GeometryError::PlanesOutOfOrder)
    );
}

#[test]
fn plane_errors_format_without_values() {
    assert_eq!(
        GeometryError::StrideTooSmall.to_string(),
        "row stride is smaller than row bytes"
    );
    assert_eq!(
        GeometryError::PlaneOverlap.to_string(),
        "plane extents overlap"
    );
}

#[cfg(target_pointer_width = "32")]
#[test]
fn output_extent_rejects_values_above_32_bit_usize() {
    assert_eq!(
        PlaneLayout::new(0, 1, u64::from(u32::MAX), 2, 1),
        Err(GeometryError::Arithmetic(MathError::ConversionOutOfRange))
    );
}
