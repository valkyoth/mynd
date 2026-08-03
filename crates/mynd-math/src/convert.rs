use core::convert::TryFrom;

use crate::{MathError, MathResult};

/// Converts `u64` to `u32` without truncation.
///
/// # Errors
///
/// Returns [`MathError::ConversionOutOfRange`] when `value` exceeds
/// [`u32::MAX`].
#[inline]
pub fn checked_u64_to_u32(value: u64) -> MathResult<u32> {
    u32::try_from(value).map_err(|_| MathError::ConversionOutOfRange)
}

/// Converts `u64` to `usize` without truncation.
///
/// This conversion is deliberately fallible so the same source remains safe
/// on 32-bit and 64-bit targets.
///
/// # Errors
///
/// Returns [`MathError::ConversionOutOfRange`] when `value` is not
/// representable by the target's `usize`.
#[inline]
pub fn checked_u64_to_usize(value: u64) -> MathResult<usize> {
    usize::try_from(value).map_err(|_| MathError::ConversionOutOfRange)
}

/// Converts `usize` to `u64` without truncation.
///
/// # Errors
///
/// Returns [`MathError::ConversionOutOfRange`] on a target whose `usize` can
/// represent values wider than `u64`.
#[inline]
pub fn checked_usize_to_u64(value: usize) -> MathResult<u64> {
    u64::try_from(value).map_err(|_| MathError::ConversionOutOfRange)
}

/// Converts a nonnegative `i64` value to `u64`.
///
/// # Errors
///
/// Returns [`MathError::ConversionOutOfRange`] for a negative value.
#[inline]
pub fn checked_i64_to_u64(value: i64) -> MathResult<u64> {
    u64::try_from(value).map_err(|_| MathError::ConversionOutOfRange)
}

/// Converts a nonnegative `i64` value to `usize` without truncation.
///
/// # Errors
///
/// Returns [`MathError::ConversionOutOfRange`] when `value` is negative or is
/// not representable by the target's `usize`.
#[inline]
pub fn checked_i64_to_usize(value: i64) -> MathResult<usize> {
    usize::try_from(value).map_err(|_| MathError::ConversionOutOfRange)
}

/// Converts `u64` to `i64` without truncation or sign change.
///
/// # Errors
///
/// Returns [`MathError::ConversionOutOfRange`] when `value` exceeds
/// [`i64::MAX`].
#[inline]
pub fn checked_u64_to_i64(value: u64) -> MathResult<i64> {
    i64::try_from(value).map_err(|_| MathError::ConversionOutOfRange)
}
