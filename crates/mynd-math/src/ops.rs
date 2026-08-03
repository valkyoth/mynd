use crate::{ArithmeticOperation, MathError, MathResult};

const fn overflow(operation: ArithmeticOperation) -> MathError {
    MathError::Overflow { operation }
}

/// Adds two `u64` values without wrapping or saturating.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] when the mathematical sum exceeds
/// [`u64::MAX`].
#[inline]
pub const fn checked_add_u64(lhs: u64, rhs: u64) -> MathResult<u64> {
    match lhs.checked_add(rhs) {
        Some(value) => Ok(value),
        None => Err(overflow(ArithmeticOperation::Add)),
    }
}

/// Adds two `usize` values without wrapping or saturating.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] when the mathematical sum exceeds
/// [`usize::MAX`].
#[inline]
pub const fn checked_add_usize(lhs: usize, rhs: usize) -> MathResult<usize> {
    match lhs.checked_add(rhs) {
        Some(value) => Ok(value),
        None => Err(overflow(ArithmeticOperation::Add)),
    }
}

/// Multiplies two `u64` values without wrapping or saturating.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] when the mathematical product exceeds
/// [`u64::MAX`].
#[inline]
pub const fn checked_mul_u64(lhs: u64, rhs: u64) -> MathResult<u64> {
    match lhs.checked_mul(rhs) {
        Some(value) => Ok(value),
        None => Err(overflow(ArithmeticOperation::Multiply)),
    }
}

/// Multiplies two `usize` values without wrapping or saturating.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] when the mathematical product exceeds
/// [`usize::MAX`].
#[inline]
pub const fn checked_mul_usize(lhs: usize, rhs: usize) -> MathResult<usize> {
    match lhs.checked_mul(rhs) {
        Some(value) => Ok(value),
        None => Err(overflow(ArithmeticOperation::Multiply)),
    }
}

/// Rounds `value` upward to a multiple of `alignment` in `u64` arithmetic.
///
/// Any nonzero alignment is accepted; callers must not infer a power-of-two
/// memory-layout guarantee. An already aligned value is returned unchanged.
///
/// # Errors
///
/// Returns [`MathError::ZeroAlignment`] when `alignment` is zero and
/// [`MathError::Overflow`] when the aligned result exceeds [`u64::MAX`].
#[inline]
pub const fn checked_align_up_u64(value: u64, alignment: u64) -> MathResult<u64> {
    if alignment == 0 {
        return Err(MathError::ZeroAlignment);
    }
    let remainder = match value.checked_rem(alignment) {
        Some(remainder) => remainder,
        None => return Err(MathError::ZeroAlignment),
    };
    if remainder == 0 {
        return Ok(value);
    }
    let padding = match alignment.checked_sub(remainder) {
        Some(padding) => padding,
        None => return Err(overflow(ArithmeticOperation::AlignUp)),
    };
    match value.checked_add(padding) {
        Some(aligned) => Ok(aligned),
        None => Err(overflow(ArithmeticOperation::AlignUp)),
    }
}

/// Rounds `value` upward to a multiple of `alignment` in `usize` arithmetic.
///
/// Any nonzero alignment is accepted; callers must not infer a power-of-two
/// memory-layout guarantee. An already aligned value is returned unchanged.
///
/// # Errors
///
/// Returns [`MathError::ZeroAlignment`] when `alignment` is zero and
/// [`MathError::Overflow`] when the aligned result exceeds [`usize::MAX`].
#[inline]
pub const fn checked_align_up_usize(value: usize, alignment: usize) -> MathResult<usize> {
    if alignment == 0 {
        return Err(MathError::ZeroAlignment);
    }
    let remainder = match value.checked_rem(alignment) {
        Some(remainder) => remainder,
        None => return Err(MathError::ZeroAlignment),
    };
    if remainder == 0 {
        return Ok(value);
    }
    let padding = match alignment.checked_sub(remainder) {
        Some(padding) => padding,
        None => return Err(overflow(ArithmeticOperation::AlignUp)),
    };
    match value.checked_add(padding) {
        Some(aligned) => Ok(aligned),
        None => Err(overflow(ArithmeticOperation::AlignUp)),
    }
}
