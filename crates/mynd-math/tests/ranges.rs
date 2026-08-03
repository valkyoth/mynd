//! Range, alignment, and error-behavior tests for `mynd-math`.

use mynd_math::{
    ArithmeticOperation, MathError, checked_align_up_u64, checked_range_u64, checked_range_usize,
};

#[test]
fn u64_range_boundary_cases_are_distinct() {
    assert_eq!(checked_range_u64(0, 0, 0), Ok(0..0));
    assert_eq!(checked_range_u64(8, 0, 8), Ok(8..8));
    assert_eq!(checked_range_u64(3, 5, 8), Ok(3..8));
    assert_eq!(checked_range_u64(9, 0, 8), Err(MathError::RangeOutOfBounds));
    assert_eq!(
        checked_range_u64(u64::MAX, 1, u64::MAX),
        Err(MathError::Overflow {
            operation: ArithmeticOperation::Add,
        })
    );
}

#[test]
fn usize_range_boundary_cases_are_distinct() {
    assert_eq!(checked_range_usize(0, 0, 0), Ok(0..0));
    assert_eq!(checked_range_usize(8, 0, 8), Ok(8..8));
    assert_eq!(checked_range_usize(3, 5, 8), Ok(3..8));
    assert_eq!(
        checked_range_usize(9, 0, 8),
        Err(MathError::RangeOutOfBounds)
    );
    assert_eq!(
        checked_range_usize(usize::MAX, 1, usize::MAX),
        Err(MathError::Overflow {
            operation: ArithmeticOperation::Add,
        })
    );
}

#[test]
fn reduced_u8_domain_alignment_is_exhaustive() {
    for value in u8::MIN..=u8::MAX {
        for alignment in u8::MIN..=u8::MAX {
            let actual = checked_align_up_u64(u64::from(value), u64::from(alignment));
            if alignment == 0 {
                assert_eq!(actual, Err(MathError::ZeroAlignment));
            } else {
                let wide_value = u16::from(value);
                let wide_alignment = u16::from(alignment);
                let expected = wide_value
                    .div_ceil(wide_alignment)
                    .saturating_mul(wide_alignment);
                assert_eq!(actual, Ok(u64::from(expected)));
            }
        }
    }
}

#[test]
fn reduced_domain_ranges_are_exhaustive() {
    for start in 0_u8..=15 {
        for length in 0_u8..=15 {
            for upper_bound in 0_u8..=15 {
                let end = u16::from(start).saturating_add(u16::from(length));
                let actual =
                    checked_range_u64(u64::from(start), u64::from(length), u64::from(upper_bound));
                if end > u16::from(upper_bound) {
                    assert_eq!(actual, Err(MathError::RangeOutOfBounds));
                } else {
                    assert_eq!(actual, Ok(u64::from(start)..u64::from(end)));
                }
            }
        }
    }
}

#[test]
fn errors_format_without_operands() {
    assert_eq!(
        MathError::ConversionOutOfRange.to_string(),
        "integer conversion is out of range"
    );
    assert_eq!(
        MathError::Overflow {
            operation: ArithmeticOperation::Multiply,
        }
        .to_string(),
        "multiplication overflow"
    );
    assert_eq!(
        MathError::ZeroAlignment.to_string(),
        "alignment must be nonzero"
    );
    assert_eq!(
        MathError::RangeOutOfBounds.to_string(),
        "range exceeds its exclusive upper bound"
    );
}
