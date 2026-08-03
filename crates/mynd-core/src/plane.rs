use core::num::{NonZeroU32, NonZeroUsize};

use mynd_math::{checked_add_u64, checked_mul_u64, checked_u64_to_usize};

use crate::{GeometryError, GeometryResult};

/// A nonzero output-buffer length representable by the target's `usize`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OutputLength(NonZeroUsize);

impl OutputLength {
    /// Returns the validated number of bytes.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0.get()
    }
}

/// A nonempty, aligned plane extent in a caller-owned output buffer.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlaneLayout {
    offset: usize,
    row_bytes: NonZeroUsize,
    row_stride: NonZeroUsize,
    rows: NonZeroU32,
    alignment: NonZeroUsize,
    end: OutputLength,
}

impl PlaneLayout {
    /// Validates one plane and computes its exact last-row output extent.
    ///
    /// Inputs use `u64` so byte-derived fixed-width values are checked before
    /// becoming target-width offsets. Alignment is numeric divisibility and
    /// does not claim Rust allocator-layout validity.
    ///
    /// # Errors
    ///
    /// Rejects zero row bytes, rows, or alignment; a stride smaller than one
    /// used row; a misaligned offset or stride; arithmetic overflow; and any
    /// value or final extent not representable by `usize`.
    pub fn new(
        offset: u64,
        row_bytes: u64,
        row_stride: u64,
        rows: u32,
        alignment: u64,
    ) -> GeometryResult<Self> {
        validate_plane_fields(offset, row_bytes, row_stride, rows, alignment)?;
        let end = required_len_u64(offset, row_bytes, row_stride, rows)?;
        let offset = checked_u64_to_usize(offset).map_err(GeometryError::Arithmetic)?;
        let row_bytes = nonzero_usize(row_bytes, GeometryError::ZeroRowBytes)?;
        let row_stride = nonzero_usize(row_stride, GeometryError::StrideTooSmall)?;
        let alignment = nonzero_usize(alignment, GeometryError::ZeroAlignment)?;
        let rows = NonZeroU32::new(rows).ok_or(GeometryError::ZeroRows)?;
        let end = nonzero_usize(end, GeometryError::ZeroRowBytes)?;
        Ok(Self {
            offset,
            row_bytes,
            row_stride,
            rows,
            alignment,
            end: OutputLength(end),
        })
    }

    /// Returns the plane's byte offset.
    #[must_use]
    pub const fn offset(self) -> usize {
        self.offset
    }

    /// Returns the bytes used by each row.
    #[must_use]
    pub const fn row_bytes(self) -> usize {
        self.row_bytes.get()
    }

    /// Returns the distance between consecutive row starts.
    #[must_use]
    pub const fn row_stride(self) -> usize {
        self.row_stride.get()
    }

    /// Returns the nonzero row count.
    #[must_use]
    pub const fn rows(self) -> u32 {
        self.rows.get()
    }

    /// Returns the numeric row-start alignment.
    #[must_use]
    pub const fn alignment(self) -> usize {
        self.alignment.get()
    }

    /// Returns the exclusive end required to store the final used row.
    #[must_use]
    pub const fn end(self) -> OutputLength {
        self.end
    }
}

/// Validates an ordered, nonoverlapping set of planes and returns its extent.
///
/// Gaps are allowed. Offsets must be in nondecreasing order, and each plane
/// must begin at or after the preceding plane's exclusive end.
///
/// # Errors
///
/// Returns [`GeometryError::NoPlanes`],
/// [`GeometryError::PlanesOutOfOrder`], or [`GeometryError::PlaneOverlap`]
/// when the set violates the contract.
pub fn checked_plane_output_len(planes: &[PlaneLayout]) -> GeometryResult<OutputLength> {
    let mut planes = planes.iter();
    let first = planes.next().ok_or(GeometryError::NoPlanes)?;
    let mut previous_offset = first.offset();
    let mut previous_end = first.end();
    for plane in planes {
        if plane.offset() < previous_offset {
            return Err(GeometryError::PlanesOutOfOrder);
        }
        if plane.offset() < previous_end.get() {
            return Err(GeometryError::PlaneOverlap);
        }
        previous_offset = plane.offset();
        previous_end = plane.end();
    }
    Ok(previous_end)
}

pub(crate) const fn required_len_u64(
    offset: u64,
    row_bytes: u64,
    row_stride: u64,
    rows: u32,
) -> GeometryResult<u64> {
    if row_bytes == 0 {
        return Err(GeometryError::ZeroRowBytes);
    }
    if rows == 0 {
        return Err(GeometryError::ZeroRows);
    }
    if row_stride < row_bytes {
        return Err(GeometryError::StrideTooSmall);
    }
    let prior_rows = match (rows as u64).checked_sub(1) {
        Some(prior_rows) => prior_rows,
        None => return Err(GeometryError::ZeroRows),
    };
    let prior_bytes = match checked_mul_u64(row_stride, prior_rows) {
        Ok(prior_bytes) => prior_bytes,
        Err(error) => return Err(GeometryError::Arithmetic(error)),
    };
    let last_row = match checked_add_u64(offset, prior_bytes) {
        Ok(last_row) => last_row,
        Err(error) => return Err(GeometryError::Arithmetic(error)),
    };
    match checked_add_u64(last_row, row_bytes) {
        Ok(end) => Ok(end),
        Err(error) => Err(GeometryError::Arithmetic(error)),
    }
}

fn validate_plane_fields(
    offset: u64,
    row_bytes: u64,
    row_stride: u64,
    rows: u32,
    alignment: u64,
) -> GeometryResult<()> {
    if row_bytes == 0 {
        return Err(GeometryError::ZeroRowBytes);
    }
    if rows == 0 {
        return Err(GeometryError::ZeroRows);
    }
    if alignment == 0 {
        return Err(GeometryError::ZeroAlignment);
    }
    if row_stride < row_bytes {
        return Err(GeometryError::StrideTooSmall);
    }
    if offset.checked_rem(alignment) != Some(0) {
        return Err(GeometryError::OffsetMisaligned);
    }
    if row_stride.checked_rem(alignment) != Some(0) {
        return Err(GeometryError::StrideMisaligned);
    }
    Ok(())
}

fn nonzero_usize(value: u64, zero_error: GeometryError) -> GeometryResult<NonZeroUsize> {
    let value = checked_u64_to_usize(value).map_err(GeometryError::Arithmetic)?;
    NonZeroUsize::new(value).ok_or(zero_error)
}
