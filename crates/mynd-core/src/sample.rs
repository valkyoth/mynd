use core::num::NonZeroU8;

use crate::{StorageError, StorageResult};

/// Byte order for storage units wider than one byte.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ByteOrder {
    /// Least-significant byte first.
    LittleEndian,
    /// Most-significant byte first.
    BigEndian,
}

/// Bit order within a byte-addressed packed sample stream.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BitOrder {
    /// The first sample begins at the most-significant available bit.
    MostSignificantFirst,
    /// The first sample begins at the least-significant available bit.
    LeastSignificantFirst,
}

/// The numeric class carried by a sample's stored bit pattern.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SampleClass {
    /// An unsigned integer sample.
    UnsignedInteger,
    /// A two's-complement signed integer sample.
    SignedInteger,
    /// An IEEE-style floating-point storage domain.
    ///
    /// This identifies storage only. Numeric behavior is not defined until
    /// the floating-sample contract milestone.
    FloatingPoint,
}

/// The physical unit containing one sample.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StorageUnit {
    /// Samples occupy a continuous packed-bit stream.
    Packed(BitOrder),
    /// Each sample occupies one byte.
    Byte,
    /// Each sample occupies a 16-bit word.
    Word16(ByteOrder),
    /// Each sample occupies a 32-bit word.
    Word32(ByteOrder),
    /// Each sample occupies a 64-bit word.
    Word64(ByteOrder),
}

/// A validated sample storage domain.
///
/// Significant bits may be narrower than an integer storage word. Packed
/// samples occupy exactly their significant-bit width. Floating-point samples
/// must occupy an entire 16-, 32-, or 64-bit word.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SampleStorage {
    class: SampleClass,
    significant_bits: NonZeroU8,
    unit: StorageUnit,
}

impl SampleStorage {
    /// Validates a numeric class, significant-bit count, and physical unit.
    ///
    /// # Errors
    ///
    /// Rejects zero bits, more than 64 packed bits, significant widths larger
    /// than a byte or word, one-bit signed samples, and floating-point storage
    /// that is packed, byte-sized, padded, or not 16/32/64 bits wide.
    pub fn new(class: SampleClass, significant_bits: u8, unit: StorageUnit) -> StorageResult<Self> {
        let significant_bits =
            NonZeroU8::new(significant_bits).ok_or(StorageError::ZeroSampleBits)?;
        let bits = significant_bits.get();
        let capacity = unit_capacity(unit, bits)?;
        if bits > capacity {
            return Err(StorageError::SampleBitsExceedStorage);
        }
        if class == SampleClass::SignedInteger && bits < 2 {
            return Err(StorageError::SignedSampleNeedsValueBit);
        }
        if class == SampleClass::FloatingPoint
            && (matches!(unit, StorageUnit::Packed(_) | StorageUnit::Byte) || bits != capacity)
        {
            return Err(StorageError::InvalidFloatingStorage);
        }
        Ok(Self {
            class,
            significant_bits,
            unit,
        })
    }

    /// Returns the sample's numeric class.
    #[must_use]
    pub const fn class(self) -> SampleClass {
        self.class
    }

    /// Returns the number of meaningful bits in the sample.
    #[must_use]
    pub const fn significant_bits(self) -> u8 {
        self.significant_bits.get()
    }

    /// Returns the physical storage unit.
    #[must_use]
    pub const fn unit(self) -> StorageUnit {
        self.unit
    }

    /// Returns the number of bits charged for each stored sample.
    #[must_use]
    pub const fn storage_bits(self) -> u8 {
        match self.unit {
            StorageUnit::Packed(_) => self.significant_bits.get(),
            StorageUnit::Byte => 8,
            StorageUnit::Word16(_) => 16,
            StorageUnit::Word32(_) => 32,
            StorageUnit::Word64(_) => 64,
        }
    }

    /// Returns the byte order when the unit is a multi-byte word.
    #[must_use]
    pub const fn byte_order(self) -> Option<ByteOrder> {
        match self.unit {
            StorageUnit::Word16(order)
            | StorageUnit::Word32(order)
            | StorageUnit::Word64(order) => Some(order),
            StorageUnit::Packed(_) | StorageUnit::Byte => None,
        }
    }

    /// Returns the packed-stream bit order, when applicable.
    #[must_use]
    pub const fn bit_order(self) -> Option<BitOrder> {
        match self.unit {
            StorageUnit::Packed(order) => Some(order),
            StorageUnit::Byte
            | StorageUnit::Word16(_)
            | StorageUnit::Word32(_)
            | StorageUnit::Word64(_) => None,
        }
    }
}

const fn unit_capacity(unit: StorageUnit, packed_bits: u8) -> StorageResult<u8> {
    match unit {
        StorageUnit::Packed(_) => {
            if packed_bits > 64 {
                Err(StorageError::PackedSampleTooWide)
            } else {
                Ok(packed_bits)
            }
        }
        StorageUnit::Byte => Ok(8),
        StorageUnit::Word16(_) => Ok(16),
        StorageUnit::Word32(_) => Ok(32),
        StorageUnit::Word64(_) => Ok(64),
    }
}
