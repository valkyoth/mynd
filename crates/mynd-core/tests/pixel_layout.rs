//! Relationship tests for pixel layouts, logical planes, chroma, and alpha.

use std::error::Error;

use mynd_core::{
    AlphaAssociation, ByteOrder, ChromaOrder, ChromaSubsampling, Dimensions, GeometryError,
    PixelLayout, PixelLayoutError, PlaneLayout, SampleClass, SampleStorage, StorageUnit,
};

#[test]
fn layout_variants_fix_plane_alpha_and_chroma_domains() -> Result<(), Box<dyn Error>> {
    let sample = u8_sample()?;
    let gray = PixelLayout::Gray { sample };
    let rgba = PixelLayout::RgbaPlanar {
        color: sample,
        alpha_sample: sample,
        alpha: AlphaAssociation::Premultiplied,
    };
    let ycbcr = PixelLayout::YcbcrSemiPlanar {
        luma: sample,
        chroma: sample,
        subsampling: ChromaSubsampling::Cs420,
        order: ChromaOrder::CbCr,
    };

    assert_eq!(gray.plane_count(), 1);
    assert_eq!(gray.alpha_association(), None);
    assert_eq!(gray.chroma_subsampling(), None);
    assert_eq!(rgba.plane_count(), 4);
    assert_eq!(
        rgba.alpha_association(),
        Some(AlphaAssociation::Premultiplied)
    );
    assert_eq!(rgba.chroma_subsampling(), None);
    assert_eq!(ycbcr.plane_count(), 2);
    assert_eq!(ycbcr.alpha_association(), None);
    assert_eq!(ycbcr.chroma_subsampling(), Some(ChromaSubsampling::Cs420));
    Ok(())
}

#[test]
fn every_chroma_domain_has_exact_ceil_dimensions() -> Result<(), Box<dyn Error>> {
    let sample = u8_sample()?;
    let image = Dimensions::new(7, 5)?;
    for (subsampling, horizontal, vertical, width, height) in [
        (ChromaSubsampling::Cs444, 1, 1, 7, 5),
        (ChromaSubsampling::Cs422, 2, 1, 4, 5),
        (ChromaSubsampling::Cs420, 2, 2, 4, 3),
        (ChromaSubsampling::Cs440, 1, 2, 7, 3),
        (ChromaSubsampling::Cs411, 4, 1, 2, 5),
        (ChromaSubsampling::Cs410, 4, 2, 2, 3),
    ] {
        let layout = PixelLayout::YcbcrPlanar {
            luma: sample,
            chroma: sample,
            subsampling,
        };
        let chroma = layout
            .plane(1)
            .ok_or(PixelLayoutError::PlaneCountMismatch)?;
        assert_eq!(subsampling.horizontal_divisor().get(), horizontal);
        assert_eq!(subsampling.vertical_divisor().get(), vertical);
        assert_eq!(
            chroma.horizontal_divisor(),
            subsampling.horizontal_divisor()
        );
        assert_eq!(chroma.vertical_divisor(), subsampling.vertical_divisor());
        assert_ne!(chroma.channels().get(), 0);
        assert_eq!(chroma.dimensions(image)?.width(), width);
        assert_eq!(chroma.dimensions(image)?.height(), height);
        assert_eq!(chroma.row_bytes(image)?, u64::from(width));
    }
    Ok(())
}

#[test]
fn packed_rows_round_only_the_final_partial_byte() -> Result<(), Box<dyn Error>> {
    let sample = SampleStorage::new(
        SampleClass::UnsignedInteger,
        1,
        StorageUnit::Packed(mynd_core::BitOrder::MostSignificantFirst),
    )?;
    let image = Dimensions::new(9, 1)?;
    let layout = PixelLayout::Gray { sample };
    let plane = layout
        .plane(0)
        .ok_or(PixelLayoutError::PlaneCountMismatch)?;
    assert_eq!(plane.row_bytes(image)?, 2);
    Ok(())
}

#[test]
fn interleaved_and_planar_rows_charge_exact_channel_counts() -> Result<(), Box<dyn Error>> {
    let sample = SampleStorage::new(
        SampleClass::UnsignedInteger,
        10,
        StorageUnit::Word16(ByteOrder::LittleEndian),
    )?;
    let image = Dimensions::new(3, 2)?;
    let interleaved = PixelLayout::RgbaInterleaved {
        sample,
        order: mynd_core::RgbaOrder::Bgra,
        alpha: AlphaAssociation::Straight,
    };
    let planar = PixelLayout::RgbaPlanar {
        color: sample,
        alpha_sample: sample,
        alpha: AlphaAssociation::Straight,
    };
    assert_eq!(
        interleaved
            .plane(0)
            .ok_or(PixelLayoutError::PlaneCountMismatch)?
            .row_bytes(image)?,
        24
    );
    for index in 0..planar.plane_count() {
        assert_eq!(
            planar
                .plane(index)
                .ok_or(PixelLayoutError::PlaneCountMismatch)?
                .row_bytes(image)?,
            6
        );
    }
    Ok(())
}

#[test]
fn concrete_planes_must_match_count_rows_and_used_bytes() -> Result<(), Box<dyn Error>> {
    let sample = u8_sample()?;
    let image = Dimensions::new(4, 2)?;
    let layout = PixelLayout::RgbInterleaved {
        sample,
        order: mynd_core::RgbOrder::Rgb,
    };
    let correct = PlaneLayout::new(0, 12, 16, 2, 4)?;
    assert_eq!(layout.validate_planes(image, &[correct])?.get(), 28);
    assert_eq!(
        layout.validate_planes(image, &[]),
        Err(PixelLayoutError::PlaneCountMismatch)
    );
    let wrong_rows = PlaneLayout::new(0, 12, 16, 1, 4)?;
    assert_eq!(
        layout.validate_planes(image, &[wrong_rows]),
        Err(PixelLayoutError::PlaneRowsMismatch)
    );
    let wrong_bytes = PlaneLayout::new(0, 13, 16, 2, 4)?;
    assert_eq!(
        layout.validate_planes(image, &[wrong_bytes]),
        Err(PixelLayoutError::PlaneRowBytesMismatch)
    );
    Ok(())
}

#[test]
fn matching_ycbcr_planes_preserve_geometry_overlap_checks() -> Result<(), Box<dyn Error>> {
    let sample = u8_sample()?;
    let image = Dimensions::new(5, 3)?;
    let layout = PixelLayout::YcbcrSemiPlanar {
        luma: sample,
        chroma: sample,
        subsampling: ChromaSubsampling::Cs420,
        order: ChromaOrder::CrCb,
    };
    let luma = PlaneLayout::new(0, 5, 5, 3, 1)?;
    let chroma = PlaneLayout::new(15, 6, 6, 2, 1)?;
    assert_eq!(layout.validate_planes(image, &[luma, chroma])?.get(), 27);
    let overlapping = PlaneLayout::new(14, 6, 6, 2, 1)?;
    assert_eq!(
        layout.validate_planes(image, &[luma, overlapping]),
        Err(PixelLayoutError::Geometry(GeometryError::PlaneOverlap))
    );
    Ok(())
}

#[test]
fn out_of_range_plane_indices_fail_closed() -> Result<(), Box<dyn Error>> {
    let layout = PixelLayout::Gray {
        sample: u8_sample()?,
    };
    assert!(layout.plane(0).is_some());
    assert!(layout.plane(1).is_none());
    assert!(layout.plane(u8::MAX).is_none());
    Ok(())
}

fn u8_sample() -> Result<SampleStorage, mynd_core::StorageError> {
    SampleStorage::new(SampleClass::UnsignedInteger, 8, StorageUnit::Byte)
}
