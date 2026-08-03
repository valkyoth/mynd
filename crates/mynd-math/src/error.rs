use core::fmt;

/// The checked operation that could not be represented by its output type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ArithmeticOperation {
    /// Checked integer addition.
    Add,
    /// Checked integer multiplication.
    Multiply,
    /// Rounding a value upward to an alignment multiple.
    AlignUp,
}

/// A failure from a checked Mynd arithmetic primitive.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum MathError {
    /// The source integer is not representable by the destination type.
    ConversionOutOfRange,
    /// The named arithmetic operation overflowed its output type.
    Overflow {
        /// The operation that overflowed.
        operation: ArithmeticOperation,
    },
    /// Alignment zero is invalid.
    ZeroAlignment,
    /// The computed half-open range exceeds its exclusive upper bound.
    RangeOutOfBounds,
}

impl fmt::Display for MathError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversionOutOfRange => formatter.write_str("integer conversion is out of range"),
            Self::Overflow { operation } => {
                write!(formatter, "{operation} overflow")
            }
            Self::ZeroAlignment => formatter.write_str("alignment must be nonzero"),
            Self::RangeOutOfBounds => {
                formatter.write_str("range exceeds its exclusive upper bound")
            }
        }
    }
}

impl fmt::Display for ArithmeticOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => formatter.write_str("addition"),
            Self::Multiply => formatter.write_str("multiplication"),
            Self::AlignUp => formatter.write_str("alignment"),
        }
    }
}

impl core::error::Error for MathError {}

/// The result type returned by checked Mynd arithmetic primitives.
pub type MathResult<T> = Result<T, MathError>;
