use core::ops::Range;

use crate::{MathError, MathResult, checked_add_u64, checked_add_usize};

/// Constructs a half-open `u64` range contained by an exclusive upper bound.
///
/// A zero-length range at `upper_bound` is valid. The function first proves
/// that `start + length` is representable and then proves the resulting end is
/// no greater than `upper_bound`.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] when `start + length` overflows and
/// [`MathError::RangeOutOfBounds`] when the resulting end exceeds
/// `upper_bound`.
#[inline]
pub const fn checked_range_u64(
    start: u64,
    length: u64,
    upper_bound: u64,
) -> MathResult<Range<u64>> {
    let end = match checked_add_u64(start, length) {
        Ok(end) => end,
        Err(error) => return Err(error),
    };
    if end > upper_bound {
        return Err(MathError::RangeOutOfBounds);
    }
    Ok(start..end)
}

/// Constructs a half-open `usize` range contained by an exclusive upper bound.
///
/// A zero-length range at `upper_bound` is valid. The function first proves
/// that `start + length` is representable and then proves the resulting end is
/// no greater than `upper_bound`.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] when `start + length` overflows and
/// [`MathError::RangeOutOfBounds`] when the resulting end exceeds
/// `upper_bound`.
#[inline]
pub const fn checked_range_usize(
    start: usize,
    length: usize,
    upper_bound: usize,
) -> MathResult<Range<usize>> {
    let end = match checked_add_usize(start, length) {
        Ok(end) => end,
        Err(error) => return Err(error),
    };
    if end > upper_bound {
        return Err(MathError::RangeOutOfBounds);
    }
    Ok(start..end)
}
