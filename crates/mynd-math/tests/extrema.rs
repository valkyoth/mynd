//! Extrema and conversion-boundary tests for `mynd-math`.

use mynd_math::{
    ArithmeticOperation, MathError, checked_add_u64, checked_add_usize, checked_align_up_u64,
    checked_align_up_usize, checked_i64_to_u64, checked_i64_to_usize, checked_mul_u64,
    checked_mul_usize, checked_u64_to_i64, checked_u64_to_u32, checked_u64_to_usize,
    checked_usize_to_u64,
};

const U64_EXTREMA: [u64; 4] = [0, 1, u64::MAX.saturating_sub(1), u64::MAX];
const USIZE_EXTREMA: [usize; 4] = [0, 1, usize::MAX.saturating_sub(1), usize::MAX];

const ADD_OVERFLOW: MathError = MathError::Overflow {
    operation: ArithmeticOperation::Add,
};
const MULTIPLY_OVERFLOW: MathError = MathError::Overflow {
    operation: ArithmeticOperation::Multiply,
};
const ALIGN_OVERFLOW: MathError = MathError::Overflow {
    operation: ArithmeticOperation::AlignUp,
};

#[test]
fn u64_add_extrema_match_the_primitive() {
    for lhs in U64_EXTREMA {
        for rhs in U64_EXTREMA {
            assert_eq!(
                checked_add_u64(lhs, rhs),
                lhs.checked_add(rhs).ok_or(ADD_OVERFLOW)
            );
        }
    }
}

#[test]
fn usize_add_extrema_match_the_primitive() {
    for lhs in USIZE_EXTREMA {
        for rhs in USIZE_EXTREMA {
            assert_eq!(
                checked_add_usize(lhs, rhs),
                lhs.checked_add(rhs).ok_or(ADD_OVERFLOW)
            );
        }
    }
}

#[test]
fn u64_multiply_extrema_match_the_primitive() {
    for lhs in U64_EXTREMA {
        for rhs in U64_EXTREMA {
            assert_eq!(
                checked_mul_u64(lhs, rhs),
                lhs.checked_mul(rhs).ok_or(MULTIPLY_OVERFLOW)
            );
        }
    }
}

#[test]
fn usize_multiply_extrema_match_the_primitive() {
    for lhs in USIZE_EXTREMA {
        for rhs in USIZE_EXTREMA {
            assert_eq!(
                checked_mul_usize(lhs, rhs),
                lhs.checked_mul(rhs).ok_or(MULTIPLY_OVERFLOW)
            );
        }
    }
}

#[test]
fn u64_alignment_extrema_match_the_primitive() {
    for value in U64_EXTREMA {
        for alignment in U64_EXTREMA {
            let expected = if alignment == 0 {
                Err(MathError::ZeroAlignment)
            } else {
                value
                    .checked_next_multiple_of(alignment)
                    .ok_or(ALIGN_OVERFLOW)
            };
            assert_eq!(checked_align_up_u64(value, alignment), expected);
        }
    }
}

#[test]
fn usize_alignment_extrema_match_the_primitive() {
    for value in USIZE_EXTREMA {
        for alignment in USIZE_EXTREMA {
            let expected = if alignment == 0 {
                Err(MathError::ZeroAlignment)
            } else {
                value
                    .checked_next_multiple_of(alignment)
                    .ok_or(ALIGN_OVERFLOW)
            };
            assert_eq!(checked_align_up_usize(value, alignment), expected);
        }
    }
}

#[test]
fn conversion_boundaries_are_exact() {
    let above_u32 = u64::from(u32::MAX).saturating_add(1);
    let i64_max_as_u64 = i64::MAX.unsigned_abs();
    let above_i64 = i64_max_as_u64.saturating_add(1);

    assert_eq!(checked_u64_to_u32(u64::from(u32::MAX)), Ok(u32::MAX));
    assert_eq!(
        checked_u64_to_u32(above_u32),
        Err(MathError::ConversionOutOfRange)
    );
    assert_eq!(checked_i64_to_u64(0), Ok(0));
    assert_eq!(checked_i64_to_u64(-1), Err(MathError::ConversionOutOfRange));
    assert_eq!(checked_u64_to_i64(i64_max_as_u64), Ok(i64::MAX));
    assert_eq!(
        checked_u64_to_i64(above_i64),
        Err(MathError::ConversionOutOfRange)
    );
    assert_eq!(checked_u64_to_usize(0), Ok(0));
    assert_eq!(checked_i64_to_usize(0), Ok(0));
    assert_eq!(
        checked_i64_to_usize(i64::MIN),
        Err(MathError::ConversionOutOfRange)
    );
    assert!(checked_usize_to_u64(usize::MAX).is_ok());
}

#[test]
fn operations_are_available_in_const_contexts() {
    const SUM: Result<u64, MathError> = checked_add_u64(7, 9);
    const PRODUCT: Result<usize, MathError> = checked_mul_usize(7, 9);
    const ALIGNED: Result<u64, MathError> = checked_align_up_u64(13, 6);

    assert_eq!(SUM, Ok(16));
    assert_eq!(PRODUCT, Ok(63));
    assert_eq!(ALIGNED, Ok(18));
}
