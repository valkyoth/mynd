use core::fmt;

use mynd_math::MathError;

/// A failure while validating format-neutral image geometry or storage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum GeometryError {
    /// An image width was zero.
    ZeroWidth,
    /// An image height was zero.
    ZeroHeight,
    /// A rectangle width was zero.
    ZeroRectangleWidth,
    /// A rectangle height was zero.
    ZeroRectangleHeight,
    /// A rectangle was not fully contained by its declared dimensions.
    RectangleOutOfBounds,
    /// A plane row count was zero.
    ZeroRows,
    /// The number of bytes used by a plane row was zero.
    ZeroRowBytes,
    /// A plane alignment was zero.
    ZeroAlignment,
    /// A row stride was smaller than the bytes used by one row.
    StrideTooSmall,
    /// A plane offset was not divisible by its numeric alignment.
    OffsetMisaligned,
    /// A row stride was not divisible by its numeric alignment.
    StrideMisaligned,
    /// A plane set was empty.
    NoPlanes,
    /// Plane offsets were not in nondecreasing order.
    PlanesOutOfOrder,
    /// Two adjacent ordered plane extents overlapped.
    PlaneOverlap,
    /// Checked arithmetic or platform-width conversion failed.
    Arithmetic(MathError),
}

impl fmt::Display for GeometryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroWidth => formatter.write_str("image width must be nonzero"),
            Self::ZeroHeight => formatter.write_str("image height must be nonzero"),
            Self::ZeroRectangleWidth => formatter.write_str("rectangle width must be nonzero"),
            Self::ZeroRectangleHeight => formatter.write_str("rectangle height must be nonzero"),
            Self::RectangleOutOfBounds => {
                formatter.write_str("rectangle exceeds its image dimensions")
            }
            Self::ZeroRows => formatter.write_str("plane row count must be nonzero"),
            Self::ZeroRowBytes => formatter.write_str("plane row bytes must be nonzero"),
            Self::ZeroAlignment => formatter.write_str("plane alignment must be nonzero"),
            Self::StrideTooSmall => formatter.write_str("row stride is smaller than row bytes"),
            Self::OffsetMisaligned => formatter.write_str("plane offset is not aligned"),
            Self::StrideMisaligned => formatter.write_str("row stride is not aligned"),
            Self::NoPlanes => formatter.write_str("plane set must be nonempty"),
            Self::PlanesOutOfOrder => formatter.write_str("plane offsets are out of order"),
            Self::PlaneOverlap => formatter.write_str("plane extents overlap"),
            Self::Arithmetic(error) => write!(formatter, "geometry arithmetic failed: {error}"),
        }
    }
}

impl core::error::Error for GeometryError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Arithmetic(error) => Some(error),
            _ => None,
        }
    }
}

/// The result type returned by validated geometry primitives.
pub type GeometryResult<T> = Result<T, GeometryError>;
