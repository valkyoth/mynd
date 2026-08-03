use core::convert::TryFrom;

use crate::{
    ArithmeticOperation, MathError, checked_add_u64, checked_add_usize, checked_align_up_u64,
    checked_align_up_usize, checked_i64_to_u64, checked_mul_u64, checked_mul_usize,
    checked_range_u64, checked_range_usize, checked_u64_to_u32,
};

const fn overflow(operation: ArithmeticOperation) -> MathError {
    MathError::Overflow { operation }
}

#[kani::proof]
fn add_u64_matches_primitive() {
    let lhs = kani::any::<u64>();
    let rhs = kani::any::<u64>();
    match (lhs.checked_add(rhs), checked_add_u64(lhs, rhs)) {
        (Some(expected), Ok(actual)) => assert!(actual == expected),
        (
            None,
            Err(MathError::Overflow {
                operation: ArithmeticOperation::Add,
            }),
        ) => {}
        _ => assert!(false),
    }
}

#[kani::proof]
fn add_usize_matches_primitive() {
    let lhs = kani::any::<usize>();
    let rhs = kani::any::<usize>();
    match (lhs.checked_add(rhs), checked_add_usize(lhs, rhs)) {
        (Some(expected), Ok(actual)) => assert!(actual == expected),
        (
            None,
            Err(MathError::Overflow {
                operation: ArithmeticOperation::Add,
            }),
        ) => {}
        _ => assert!(false),
    }
}

#[kani::proof]
fn multiply_u64_matches_primitive_for_all_u8_inputs() {
    let lhs = u64::from(kani::any::<u8>());
    let rhs = u64::from(kani::any::<u8>());
    match (lhs.checked_mul(rhs), checked_mul_u64(lhs, rhs)) {
        (Some(expected), Ok(actual)) => assert!(actual == expected),
        (
            None,
            Err(MathError::Overflow {
                operation: ArithmeticOperation::Multiply,
            }),
        ) => {}
        _ => assert!(false),
    }
}

#[kani::proof]
fn multiply_usize_matches_primitive_for_all_u8_inputs() {
    let lhs = usize::from(kani::any::<u8>());
    let rhs = usize::from(kani::any::<u8>());
    match (lhs.checked_mul(rhs), checked_mul_usize(lhs, rhs)) {
        (Some(expected), Ok(actual)) => assert!(actual == expected),
        (
            None,
            Err(MathError::Overflow {
                operation: ArithmeticOperation::Multiply,
            }),
        ) => {}
        _ => assert!(false),
    }
}

#[kani::proof]
fn multiply_u64_maximum_by_all_u8_inputs() {
    let rhs = u64::from(kani::any::<u8>());
    match (u64::MAX.checked_mul(rhs), checked_mul_u64(u64::MAX, rhs)) {
        (Some(expected), Ok(actual)) => assert!(actual == expected),
        (
            None,
            Err(MathError::Overflow {
                operation: ArithmeticOperation::Multiply,
            }),
        ) => {}
        _ => assert!(false),
    }
}

#[kani::proof]
fn multiply_usize_maximum_by_all_u8_inputs() {
    let rhs = usize::from(kani::any::<u8>());
    match (
        usize::MAX.checked_mul(rhs),
        checked_mul_usize(usize::MAX, rhs),
    ) {
        (Some(expected), Ok(actual)) => assert!(actual == expected),
        (
            None,
            Err(MathError::Overflow {
                operation: ArithmeticOperation::Multiply,
            }),
        ) => {}
        _ => assert!(false),
    }
}

#[kani::proof]
fn align_u64_matches_primitive_for_all_u8_inputs() {
    let value = u64::from(kani::any::<u8>());
    let alignment = u64::from(kani::any::<u8>());
    let actual = checked_align_up_u64(value, alignment);
    match actual {
        Ok(aligned) => {
            assert!(alignment != 0);
            match value.checked_next_multiple_of(alignment) {
                Some(expected) => assert!(aligned == expected),
                None => assert!(false),
            }
        }
        Err(MathError::ZeroAlignment) => assert!(alignment == 0),
        Err(MathError::Overflow {
            operation: ArithmeticOperation::AlignUp,
        }) => {
            assert!(alignment != 0);
            assert!(value.checked_next_multiple_of(alignment).is_none());
        }
        Err(_) => assert!(false),
    }
}

#[kani::proof]
fn align_usize_matches_primitive_for_all_u8_inputs() {
    let value = usize::from(kani::any::<u8>());
    let alignment = usize::from(kani::any::<u8>());
    let actual = checked_align_up_usize(value, alignment);
    match actual {
        Ok(aligned) => {
            assert!(alignment != 0);
            match value.checked_next_multiple_of(alignment) {
                Some(expected) => assert!(aligned == expected),
                None => assert!(false),
            }
        }
        Err(MathError::ZeroAlignment) => assert!(alignment == 0),
        Err(MathError::Overflow {
            operation: ArithmeticOperation::AlignUp,
        }) => {
            assert!(alignment != 0);
            assert!(value.checked_next_multiple_of(alignment).is_none());
        }
        Err(_) => assert!(false),
    }
}

#[kani::proof]
fn range_u64_matches_checked_end_and_bound() {
    let start = kani::any::<u64>();
    let length = kani::any::<u64>();
    let upper_bound = kani::any::<u64>();
    let expected = match start.checked_add(length) {
        None => Err(overflow(ArithmeticOperation::Add)),
        Some(end) if end > upper_bound => Err(MathError::RangeOutOfBounds),
        Some(end) => Ok(start..end),
    };
    assert!(checked_range_u64(start, length, upper_bound) == expected);
}

#[kani::proof]
fn range_usize_matches_checked_end_and_bound() {
    let start = kani::any::<usize>();
    let length = kani::any::<usize>();
    let upper_bound = kani::any::<usize>();
    let expected = match start.checked_add(length) {
        None => Err(overflow(ArithmeticOperation::Add)),
        Some(end) if end > upper_bound => Err(MathError::RangeOutOfBounds),
        Some(end) => Ok(start..end),
    };
    assert!(checked_range_usize(start, length, upper_bound) == expected);
}

#[kani::proof]
fn u64_to_u32_matches_try_from() {
    let value = kani::any::<u64>();
    assert!(checked_u64_to_u32(value).is_ok() == u32::try_from(value).is_ok());
}

#[kani::proof]
fn i64_to_u64_matches_try_from() {
    let value = kani::any::<i64>();
    assert!(checked_i64_to_u64(value).is_ok() == u64::try_from(value).is_ok());
}
