use core::convert::TryFrom;

use mynd_math::{MathError, checked_add_u64, checked_mul_u64};

use crate::{
    Dimensions, OutputLength, PixelLayoutError, PixelLayoutResult, PlaneLayout, SampleStorage,
    checked_plane_output_len,
};

/// The meaning of stored alpha samples.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AlphaAssociation {
    /// Color samples are independent of alpha.
    Straight,
    /// Color samples are already multiplied by alpha.
    Premultiplied,
}

/// Chroma sampling factors relative to full-resolution luma.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ChromaSubsampling {
    /// No chroma subsampling (4:4:4).
    Cs444,
    /// Half horizontal chroma resolution (4:2:2).
    Cs422,
    /// Half horizontal and vertical chroma resolution (4:2:0).
    Cs420,
    /// Half vertical chroma resolution (4:4:0).
    Cs440,
    /// Quarter horizontal chroma resolution (4:1:1).
    Cs411,
    /// Quarter horizontal and half vertical chroma resolution (4:1:0).
    Cs410,
}

impl ChromaSubsampling {
    const fn factors(self) -> (u8, u8) {
        match self {
            Self::Cs444 => (1, 1),
            Self::Cs422 => (2, 1),
            Self::Cs420 => (2, 2),
            Self::Cs440 => (1, 2),
            Self::Cs411 => (4, 1),
            Self::Cs410 => (4, 2),
        }
    }

    /// Returns the horizontal chroma divisor.
    #[must_use]
    pub const fn horizontal_divisor(self) -> u8 {
        self.factors().0
    }

    /// Returns the vertical chroma divisor.
    #[must_use]
    pub const fn vertical_divisor(self) -> u8 {
        self.factors().1
    }
}

/// Channel order for interleaved gray-and-alpha samples.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GrayAlphaOrder {
    /// Gray followed by alpha.
    GrayAlpha,
    /// Alpha followed by gray.
    AlphaGray,
}

/// Channel order for interleaved three-channel RGB samples.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RgbOrder {
    /// Red, green, blue.
    Rgb,
    /// Blue, green, red.
    Bgr,
}

/// Channel order for interleaved RGB-with-alpha samples.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RgbaOrder {
    /// Red, green, blue, alpha.
    Rgba,
    /// Blue, green, red, alpha.
    Bgra,
    /// Alpha, red, green, blue.
    Argb,
    /// Alpha, blue, green, red.
    Abgr,
}

/// Channel order for the interleaved chroma plane of a semi-planar layout.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ChromaOrder {
    /// Cb followed by Cr.
    CbCr,
    /// Cr followed by Cb.
    CrCb,
}

/// One logical plane required by a [`PixelLayout`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PixelPlane {
    sample: SampleStorage,
    channels: u8,
    horizontal_divisor: u8,
    vertical_divisor: u8,
}

impl PixelPlane {
    const fn new(
        sample: SampleStorage,
        channels: u8,
        horizontal_divisor: u8,
        vertical_divisor: u8,
    ) -> Self {
        Self {
            sample,
            channels,
            horizontal_divisor,
            vertical_divisor,
        }
    }

    /// Returns the shared storage domain for each channel in this plane.
    #[must_use]
    pub const fn sample(self) -> SampleStorage {
        self.sample
    }

    /// Returns the number of interleaved channels in this plane.
    #[must_use]
    pub const fn channels(self) -> u8 {
        self.channels
    }

    /// Returns the horizontal sampling divisor relative to the image.
    #[must_use]
    pub const fn horizontal_divisor(self) -> u8 {
        self.horizontal_divisor
    }

    /// Returns the vertical sampling divisor relative to the image.
    #[must_use]
    pub const fn vertical_divisor(self) -> u8 {
        self.vertical_divisor
    }

    /// Returns the exact logical dimensions sampled by this plane.
    pub fn dimensions(self, image: Dimensions) -> PixelLayoutResult<Dimensions> {
        let width = divided_ceil(image.width(), self.horizontal_divisor);
        let height = divided_ceil(image.height(), self.vertical_divisor);
        Dimensions::new(width, height).map_err(PixelLayoutError::Geometry)
    }

    /// Returns the exact bytes used by one tightly packed plane row.
    ///
    /// A final partial byte is charged in full. Row-stride padding is not
    /// included.
    ///
    /// # Errors
    ///
    /// Returns an arithmetic error if the row-bit calculation overflows.
    pub fn row_bytes(self, image: Dimensions) -> PixelLayoutResult<u64> {
        let samples = u64::from(divided_ceil(image.width(), self.horizontal_divisor));
        let samples = checked_mul_u64(samples, u64::from(self.channels))
            .map_err(PixelLayoutError::Arithmetic)?;
        let bits = checked_mul_u64(samples, u64::from(self.sample.storage_bits()))
            .map_err(PixelLayoutError::Arithmetic)?;
        let rounded = checked_add_u64(bits, 7).map_err(PixelLayoutError::Arithmetic)?;
        Ok(rounded / 8)
    }
}

/// A format-neutral pixel layout whose channel, alpha, chroma, and plane
/// relationships are fixed by its variant.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PixelLayout {
    /// One full-resolution gray plane.
    Gray {
        /// Gray sample storage.
        sample: SampleStorage,
    },
    /// One full-resolution interleaved gray/alpha plane.
    GrayAlphaInterleaved {
        /// Shared gray and alpha storage.
        sample: SampleStorage,
        /// Interleaved channel order.
        order: GrayAlphaOrder,
        /// Alpha association.
        alpha: AlphaAssociation,
    },
    /// Separate full-resolution gray and alpha planes, in that order.
    GrayAlphaPlanar {
        /// Gray-plane storage.
        gray: SampleStorage,
        /// Alpha-plane storage.
        alpha_sample: SampleStorage,
        /// Alpha association.
        alpha: AlphaAssociation,
    },
    /// One full-resolution interleaved RGB plane.
    RgbInterleaved {
        /// Shared channel storage.
        sample: SampleStorage,
        /// Interleaved channel order.
        order: RgbOrder,
    },
    /// Separate full-resolution R, G, and B planes, in that order.
    RgbPlanar {
        /// Shared channel storage.
        sample: SampleStorage,
    },
    /// One full-resolution interleaved RGB/alpha plane.
    RgbaInterleaved {
        /// Shared channel storage.
        sample: SampleStorage,
        /// Interleaved channel order.
        order: RgbaOrder,
        /// Alpha association.
        alpha: AlphaAssociation,
    },
    /// Separate full-resolution R, G, B, and alpha planes, in that order.
    RgbaPlanar {
        /// Shared RGB-plane storage.
        color: SampleStorage,
        /// Alpha-plane storage.
        alpha_sample: SampleStorage,
        /// Alpha association.
        alpha: AlphaAssociation,
    },
    /// Separate Y, Cb, and Cr planes, in that order.
    YcbcrPlanar {
        /// Luma-plane storage.
        luma: SampleStorage,
        /// Shared chroma-plane storage.
        chroma: SampleStorage,
        /// Chroma sampling factors.
        subsampling: ChromaSubsampling,
    },
    /// A full-resolution Y plane followed by one interleaved chroma plane.
    YcbcrSemiPlanar {
        /// Luma-plane storage.
        luma: SampleStorage,
        /// Chroma-plane channel storage.
        chroma: SampleStorage,
        /// Chroma sampling factors.
        subsampling: ChromaSubsampling,
        /// Interleaved chroma order.
        order: ChromaOrder,
    },
    /// Separate Y, Cb, Cr, and full-resolution alpha planes, in that order.
    YcbcrAlphaPlanar {
        /// Luma-plane storage.
        luma: SampleStorage,
        /// Shared chroma-plane storage.
        chroma: SampleStorage,
        /// Alpha-plane storage.
        alpha_sample: SampleStorage,
        /// Chroma sampling factors.
        subsampling: ChromaSubsampling,
        /// Alpha association.
        alpha: AlphaAssociation,
    },
    /// Y, interleaved chroma, and full-resolution alpha planes, in that order.
    YcbcrAlphaSemiPlanar {
        /// Luma-plane storage.
        luma: SampleStorage,
        /// Chroma-plane channel storage.
        chroma: SampleStorage,
        /// Alpha-plane storage.
        alpha_sample: SampleStorage,
        /// Chroma sampling factors.
        subsampling: ChromaSubsampling,
        /// Interleaved chroma order.
        order: ChromaOrder,
        /// Alpha association.
        alpha: AlphaAssociation,
    },
}

impl PixelLayout {
    /// Returns the exact number of planes required by the variant.
    #[must_use]
    pub const fn plane_count(self) -> u8 {
        match self {
            Self::Gray { .. }
            | Self::GrayAlphaInterleaved { .. }
            | Self::RgbInterleaved { .. }
            | Self::RgbaInterleaved { .. } => 1,
            Self::GrayAlphaPlanar { .. } | Self::YcbcrSemiPlanar { .. } => 2,
            Self::RgbPlanar { .. }
            | Self::YcbcrPlanar { .. }
            | Self::YcbcrAlphaSemiPlanar { .. } => 3,
            Self::RgbaPlanar { .. } | Self::YcbcrAlphaPlanar { .. } => 4,
        }
    }

    /// Returns the alpha association only for layouts that contain alpha.
    #[must_use]
    pub const fn alpha_association(self) -> Option<AlphaAssociation> {
        match self {
            Self::GrayAlphaInterleaved { alpha, .. }
            | Self::GrayAlphaPlanar { alpha, .. }
            | Self::RgbaInterleaved { alpha, .. }
            | Self::RgbaPlanar { alpha, .. }
            | Self::YcbcrAlphaPlanar { alpha, .. }
            | Self::YcbcrAlphaSemiPlanar { alpha, .. } => Some(alpha),
            Self::Gray { .. }
            | Self::RgbInterleaved { .. }
            | Self::RgbPlanar { .. }
            | Self::YcbcrPlanar { .. }
            | Self::YcbcrSemiPlanar { .. } => None,
        }
    }

    /// Returns chroma subsampling only for YCbCr layouts.
    #[must_use]
    pub const fn chroma_subsampling(self) -> Option<ChromaSubsampling> {
        match self {
            Self::YcbcrPlanar { subsampling, .. }
            | Self::YcbcrSemiPlanar { subsampling, .. }
            | Self::YcbcrAlphaPlanar { subsampling, .. }
            | Self::YcbcrAlphaSemiPlanar { subsampling, .. } => Some(subsampling),
            Self::Gray { .. }
            | Self::GrayAlphaInterleaved { .. }
            | Self::GrayAlphaPlanar { .. }
            | Self::RgbInterleaved { .. }
            | Self::RgbPlanar { .. }
            | Self::RgbaInterleaved { .. }
            | Self::RgbaPlanar { .. } => None,
        }
    }

    /// Returns one logical plane, or `None` when the index is out of range.
    #[must_use]
    pub const fn plane(self, index: u8) -> Option<PixelPlane> {
        match self {
            Self::Gray { sample } if index == 0 => Some(full(sample, 1)),
            Self::GrayAlphaInterleaved { sample, .. } if index == 0 => Some(full(sample, 2)),
            Self::GrayAlphaPlanar {
                gray, alpha_sample, ..
            } => planar_two(index, gray, alpha_sample),
            Self::RgbInterleaved { sample, .. } if index == 0 => Some(full(sample, 3)),
            Self::RgbPlanar { sample } => same_full(index, sample, 3),
            Self::RgbaInterleaved { sample, .. } if index == 0 => Some(full(sample, 4)),
            Self::RgbaPlanar {
                color,
                alpha_sample,
                ..
            } => rgba_planar(index, color, alpha_sample),
            Self::YcbcrPlanar {
                luma,
                chroma,
                subsampling,
            } => ycbcr_planar(index, luma, chroma, subsampling, None),
            Self::YcbcrSemiPlanar {
                luma,
                chroma,
                subsampling,
                ..
            } => ycbcr_semi(index, luma, chroma, subsampling, None),
            Self::YcbcrAlphaPlanar {
                luma,
                chroma,
                alpha_sample,
                subsampling,
                ..
            } => ycbcr_planar(index, luma, chroma, subsampling, Some(alpha_sample)),
            Self::YcbcrAlphaSemiPlanar {
                luma,
                chroma,
                alpha_sample,
                subsampling,
                ..
            } => ycbcr_semi(index, luma, chroma, subsampling, Some(alpha_sample)),
            _ => None,
        }
    }

    /// Validates concrete byte planes against the exact logical layout.
    ///
    /// Plane count, sampled row count, and tightly packed used row bytes must
    /// match. Stride padding and gaps between planes remain permitted by
    /// [`PlaneLayout`]; overlap and ordering remain forbidden.
    ///
    /// # Errors
    ///
    /// Rejects mismatched plane relationships, row-size arithmetic failure,
    /// or invalid concrete plane ordering and overlap.
    pub fn validate_planes(
        self,
        image: Dimensions,
        planes: &[PlaneLayout],
    ) -> PixelLayoutResult<OutputLength> {
        if planes.len() != usize::from(self.plane_count()) {
            return Err(PixelLayoutError::PlaneCountMismatch);
        }
        for (index, plane) in planes.iter().enumerate() {
            let index = u8::try_from(index)
                .map_err(|_| PixelLayoutError::Arithmetic(MathError::ConversionOutOfRange))?;
            let expected = self
                .plane(index)
                .ok_or(PixelLayoutError::PlaneCountMismatch)?;
            if plane.rows() != expected.dimensions(image)?.height() {
                return Err(PixelLayoutError::PlaneRowsMismatch);
            }
            let row_bytes = u64::try_from(plane.row_bytes())
                .map_err(|_| PixelLayoutError::Arithmetic(MathError::ConversionOutOfRange))?;
            if row_bytes != expected.row_bytes(image)? {
                return Err(PixelLayoutError::PlaneRowBytesMismatch);
            }
        }
        checked_plane_output_len(planes).map_err(PixelLayoutError::Geometry)
    }
}

const fn full(sample: SampleStorage, channels: u8) -> PixelPlane {
    PixelPlane::new(sample, channels, 1, 1)
}

const fn planar_two(index: u8, first: SampleStorage, second: SampleStorage) -> Option<PixelPlane> {
    match index {
        0 => Some(full(first, 1)),
        1 => Some(full(second, 1)),
        _ => None,
    }
}

const fn same_full(index: u8, sample: SampleStorage, count: u8) -> Option<PixelPlane> {
    if index < count {
        Some(full(sample, 1))
    } else {
        None
    }
}

const fn rgba_planar(index: u8, color: SampleStorage, alpha: SampleStorage) -> Option<PixelPlane> {
    match index {
        0..=2 => Some(full(color, 1)),
        3 => Some(full(alpha, 1)),
        _ => None,
    }
}

const fn ycbcr_planar(
    index: u8,
    luma: SampleStorage,
    chroma: SampleStorage,
    subsampling: ChromaSubsampling,
    alpha: Option<SampleStorage>,
) -> Option<PixelPlane> {
    let (horizontal, vertical) = subsampling.factors();
    match index {
        0 => Some(full(luma, 1)),
        1 | 2 => Some(PixelPlane::new(chroma, 1, horizontal, vertical)),
        3 => match alpha {
            Some(sample) => Some(full(sample, 1)),
            None => None,
        },
        _ => None,
    }
}

const fn ycbcr_semi(
    index: u8,
    luma: SampleStorage,
    chroma: SampleStorage,
    subsampling: ChromaSubsampling,
    alpha: Option<SampleStorage>,
) -> Option<PixelPlane> {
    let (horizontal, vertical) = subsampling.factors();
    match index {
        0 => Some(full(luma, 1)),
        1 => Some(PixelPlane::new(chroma, 2, horizontal, vertical)),
        2 => match alpha {
            Some(sample) => Some(full(sample, 1)),
            None => None,
        },
        _ => None,
    }
}

fn divided_ceil(value: u32, divisor: u8) -> u32 {
    value.div_ceil(u32::from(divisor))
}
