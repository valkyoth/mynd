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

/// A failure while validating sample storage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum StorageError {
    /// A sample declared zero significant bits.
    ZeroSampleBits,
    /// A packed sample exceeded the supported 64-bit storage domain.
    PackedSampleTooWide,
    /// Significant bits exceeded the selected physical storage unit.
    SampleBitsExceedStorage,
    /// A signed sample had no bit left for a value after its sign bit.
    SignedSampleNeedsValueBit,
    /// A floating sample did not occupy an entire 16-, 32-, or 64-bit word.
    InvalidFloatingStorage,
}

impl fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSampleBits => formatter.write_str("sample bits must be nonzero"),
            Self::PackedSampleTooWide => {
                formatter.write_str("packed samples cannot exceed 64 bits")
            }
            Self::SampleBitsExceedStorage => {
                formatter.write_str("sample bits exceed the selected storage unit")
            }
            Self::SignedSampleNeedsValueBit => {
                formatter.write_str("signed samples require a sign bit and a value bit")
            }
            Self::InvalidFloatingStorage => {
                formatter.write_str("floating samples require a full 16-, 32-, or 64-bit word")
            }
        }
    }
}

impl core::error::Error for StorageError {}

/// The result type returned by sample-storage validation.
pub type StorageResult<T> = Result<T, StorageError>;

/// A failure while relating a pixel layout to concrete planes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PixelLayoutError {
    /// The concrete plane count did not match the pixel layout.
    PlaneCountMismatch,
    /// A plane row count did not match its logical sampled height.
    PlaneRowsMismatch,
    /// A plane's used row bytes did not match its exact packed width.
    PlaneRowBytesMismatch,
    /// Geometry validation failed.
    Geometry(GeometryError),
    /// Checked row-size arithmetic failed.
    Arithmetic(MathError),
}

impl fmt::Display for PixelLayoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlaneCountMismatch => formatter.write_str("pixel layout plane count mismatch"),
            Self::PlaneRowsMismatch => formatter.write_str("pixel layout plane rows mismatch"),
            Self::PlaneRowBytesMismatch => {
                formatter.write_str("pixel layout plane row bytes mismatch")
            }
            Self::Geometry(error) => write!(formatter, "pixel layout geometry failed: {error}"),
            Self::Arithmetic(error) => write!(formatter, "pixel layout arithmetic failed: {error}"),
        }
    }
}

impl core::error::Error for PixelLayoutError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Geometry(error) => Some(error),
            Self::Arithmetic(error) => Some(error),
            Self::PlaneCountMismatch | Self::PlaneRowsMismatch | Self::PlaneRowBytesMismatch => {
                None
            }
        }
    }
}

impl From<GeometryError> for PixelLayoutError {
    fn from(error: GeometryError) -> Self {
        Self::Geometry(error)
    }
}

/// The result type returned by pixel-layout validation.
pub type PixelLayoutResult<T> = Result<T, PixelLayoutError>;
