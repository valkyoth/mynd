//! Boundary tests for explicit sample-storage domains.

use mynd_core::{BitOrder, ByteOrder, SampleClass, SampleStorage, StorageError, StorageUnit};

#[test]
fn integer_storage_rejects_zero_and_excess_bits() {
    assert_eq!(
        SampleStorage::new(SampleClass::UnsignedInteger, 0, StorageUnit::Byte),
        Err(StorageError::ZeroSampleBits)
    );
    assert_eq!(
        SampleStorage::new(SampleClass::UnsignedInteger, 9, StorageUnit::Byte),
        Err(StorageError::SampleBitsExceedStorage)
    );
    assert_eq!(
        SampleStorage::new(
            SampleClass::UnsignedInteger,
            17,
            StorageUnit::Word16(ByteOrder::LittleEndian),
        ),
        Err(StorageError::SampleBitsExceedStorage)
    );
    assert_eq!(
        SampleStorage::new(
            SampleClass::UnsignedInteger,
            65,
            StorageUnit::Packed(BitOrder::MostSignificantFirst),
        ),
        Err(StorageError::PackedSampleTooWide)
    );
}

#[test]
fn integer_storage_retains_significant_and_physical_widths() {
    let packed = SampleStorage::new(
        SampleClass::UnsignedInteger,
        12,
        StorageUnit::Packed(BitOrder::MostSignificantFirst),
    );
    assert_eq!(packed.map(SampleStorage::significant_bits), Ok(12));
    assert_eq!(packed.map(SampleStorage::storage_bits), Ok(12));
    assert_eq!(
        packed.map(SampleStorage::bit_order),
        Ok(Some(BitOrder::MostSignificantFirst))
    );

    let padded = SampleStorage::new(
        SampleClass::UnsignedInteger,
        10,
        StorageUnit::Word16(ByteOrder::LittleEndian),
    );
    assert_eq!(padded.map(SampleStorage::significant_bits), Ok(10));
    assert_eq!(padded.map(SampleStorage::storage_bits), Ok(16));
    assert_eq!(
        padded.map(SampleStorage::byte_order),
        Ok(Some(ByteOrder::LittleEndian))
    );
}

#[test]
fn signed_storage_requires_a_sign_and_value_bit() {
    assert_eq!(
        SampleStorage::new(
            SampleClass::SignedInteger,
            1,
            StorageUnit::Packed(BitOrder::LeastSignificantFirst),
        ),
        Err(StorageError::SignedSampleNeedsValueBit)
    );
    assert!(
        SampleStorage::new(
            SampleClass::SignedInteger,
            2,
            StorageUnit::Packed(BitOrder::LeastSignificantFirst),
        )
        .is_ok()
    );
}

#[test]
fn floating_storage_is_full_word_only() {
    for unit in [
        StorageUnit::Word16(ByteOrder::BigEndian),
        StorageUnit::Word32(ByteOrder::LittleEndian),
        StorageUnit::Word64(ByteOrder::BigEndian),
    ] {
        assert!(SampleStorage::new(SampleClass::FloatingPoint, unit_width(unit), unit).is_ok());
    }
    assert_eq!(
        SampleStorage::new(SampleClass::FloatingPoint, 8, StorageUnit::Byte),
        Err(StorageError::InvalidFloatingStorage)
    );
    assert_eq!(
        SampleStorage::new(
            SampleClass::FloatingPoint,
            15,
            StorageUnit::Word16(ByteOrder::BigEndian),
        ),
        Err(StorageError::InvalidFloatingStorage)
    );
    assert_eq!(
        SampleStorage::new(
            SampleClass::FloatingPoint,
            32,
            StorageUnit::Packed(BitOrder::MostSignificantFirst),
        ),
        Err(StorageError::InvalidFloatingStorage)
    );
}

#[test]
fn storage_accessors_do_not_invent_order() {
    let byte = SampleStorage::new(SampleClass::UnsignedInteger, 8, StorageUnit::Byte);
    assert_eq!(
        byte.map(SampleStorage::class),
        Ok(SampleClass::UnsignedInteger)
    );
    assert_eq!(byte.map(SampleStorage::byte_order), Ok(None));
    assert_eq!(byte.map(SampleStorage::bit_order), Ok(None));
}

#[test]
fn storage_errors_do_not_echo_inputs() {
    assert_eq!(
        StorageError::SampleBitsExceedStorage.to_string(),
        "sample bits exceed the selected storage unit"
    );
    assert_eq!(
        StorageError::InvalidFloatingStorage.to_string(),
        "floating samples require a full 16-, 32-, or 64-bit word"
    );
}

const fn unit_width(unit: StorageUnit) -> u8 {
    match unit {
        StorageUnit::Word16(_) => 16,
        StorageUnit::Word32(_) => 32,
        StorageUnit::Word64(_) => 64,
        StorageUnit::Packed(_) | StorageUnit::Byte => 0,
    }
}
