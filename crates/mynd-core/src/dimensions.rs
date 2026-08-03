use core::num::NonZeroU32;

use mynd_math::{checked_mul_u64, checked_u64_to_usize};

use crate::{GeometryError, GeometryResult};

/// Nonzero two-dimensional image dimensions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Dimensions {
    width: NonZeroU32,
    height: NonZeroU32,
    pixel_count: u64,
}

impl Dimensions {
    /// Validates nonzero image dimensions and their exact pixel count.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::ZeroWidth`] or [`GeometryError::ZeroHeight`]
    /// for a zero axis. Arithmetic failure is reported explicitly rather than
    /// relying on the mathematical fact that two `u32` values fit in `u64`.
    pub const fn new(width: u32, height: u32) -> GeometryResult<Self> {
        let width = match NonZeroU32::new(width) {
            Some(width) => width,
            None => return Err(GeometryError::ZeroWidth),
        };
        let height = match NonZeroU32::new(height) {
            Some(height) => height,
            None => return Err(GeometryError::ZeroHeight),
        };
        let pixel_count = match checked_mul_u64(width.get() as u64, height.get() as u64) {
            Ok(pixel_count) => pixel_count,
            Err(error) => return Err(GeometryError::Arithmetic(error)),
        };
        Ok(Self {
            width,
            height,
            pixel_count,
        })
    }

    /// Returns the nonzero width.
    #[must_use]
    pub const fn width(self) -> u32 {
        self.width.get()
    }

    /// Returns the nonzero height.
    #[must_use]
    pub const fn height(self) -> u32 {
        self.height.get()
    }

    /// Returns the exact fixed-width number of pixels.
    #[must_use]
    pub const fn pixel_count(self) -> u64 {
        self.pixel_count
    }

    /// Converts the pixel count to the target's address width.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::Arithmetic`] when the count is not
    /// representable by `usize`, including large images on 32-bit targets.
    pub fn pixel_count_usize(self) -> GeometryResult<usize> {
        checked_u64_to_usize(self.pixel_count).map_err(GeometryError::Arithmetic)
    }
}
