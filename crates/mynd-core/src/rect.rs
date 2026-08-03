use core::num::NonZeroU32;

use mynd_math::{checked_mul_u64, checked_u64_to_usize};

use crate::{Dimensions, GeometryError, GeometryResult};

/// A nonempty rectangle proven to be contained by retained image dimensions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ImageRect {
    bounds: Dimensions,
    x: u32,
    y: u32,
    width: NonZeroU32,
    height: NonZeroU32,
    right: u32,
    bottom: u32,
    area: u64,
}

impl ImageRect {
    /// Validates a nonempty rectangle within `bounds`.
    ///
    /// # Errors
    ///
    /// Returns a zero-axis error for a degenerate rectangle or
    /// [`GeometryError::RectangleOutOfBounds`] when an exclusive end overflows
    /// `u32` or exceeds `bounds`.
    pub const fn new(
        bounds: Dimensions,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> GeometryResult<Self> {
        let width = match NonZeroU32::new(width) {
            Some(width) => width,
            None => return Err(GeometryError::ZeroRectangleWidth),
        };
        let height = match NonZeroU32::new(height) {
            Some(height) => height,
            None => return Err(GeometryError::ZeroRectangleHeight),
        };
        let right = match exclusive_end(x, width.get()) {
            Ok(right) => right,
            Err(error) => return Err(error),
        };
        let bottom = match exclusive_end(y, height.get()) {
            Ok(bottom) => bottom,
            Err(error) => return Err(error),
        };
        if right > bounds.width() || bottom > bounds.height() {
            return Err(GeometryError::RectangleOutOfBounds);
        }
        let area = match checked_mul_u64(width.get() as u64, height.get() as u64) {
            Ok(area) => area,
            Err(error) => return Err(GeometryError::Arithmetic(error)),
        };
        Ok(Self {
            bounds,
            x,
            y,
            width,
            height,
            right,
            bottom,
            area,
        })
    }

    /// Returns the dimensions that contain this rectangle.
    #[must_use]
    pub const fn bounds(self) -> Dimensions {
        self.bounds
    }

    /// Returns the horizontal origin.
    #[must_use]
    pub const fn x(self) -> u32 {
        self.x
    }

    /// Returns the vertical origin.
    #[must_use]
    pub const fn y(self) -> u32 {
        self.y
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

    /// Returns the exclusive horizontal end.
    #[must_use]
    pub const fn right(self) -> u32 {
        self.right
    }

    /// Returns the exclusive vertical end.
    #[must_use]
    pub const fn bottom(self) -> u32 {
        self.bottom
    }

    /// Returns the exact fixed-width rectangle area.
    #[must_use]
    pub const fn area(self) -> u64 {
        self.area
    }

    /// Converts the rectangle area to the target's address width.
    ///
    /// # Errors
    ///
    /// Returns [`GeometryError::Arithmetic`] when the area is not
    /// representable by `usize`.
    pub fn area_usize(self) -> GeometryResult<usize> {
        checked_u64_to_usize(self.area).map_err(GeometryError::Arithmetic)
    }
}

const fn exclusive_end(origin: u32, extent: u32) -> GeometryResult<u32> {
    match origin.checked_add(extent) {
        Some(end) => Ok(end),
        None => Err(GeometryError::RectangleOutOfBounds),
    }
}
