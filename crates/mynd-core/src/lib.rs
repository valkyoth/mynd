#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Format-neutral image foundations for the `mynd` ecosystem.
//!
//! Version 0.3.0 provides validated geometry plus explicit sample storage,
//! channel order, chroma subsampling, alpha association, and exact plane
//! relationships. Buffer access remains outside this release.

mod dimensions;
mod error;
mod pixel;
mod plane;
mod rect;
mod sample;

#[cfg(kani)]
mod proofs;

pub use dimensions::Dimensions;
pub use error::{
    GeometryError, GeometryResult, PixelLayoutError, PixelLayoutResult, StorageError, StorageResult,
};
pub use pixel::{
    AlphaAssociation, ChromaOrder, ChromaSubsampling, GrayAlphaOrder, PixelLayout, PixelPlane,
    RgbOrder, RgbaOrder,
};
pub use plane::{OutputLength, PlaneLayout, checked_plane_output_len};
pub use rect::ImageRect;
pub use sample::{BitOrder, ByteOrder, SampleClass, SampleStorage, StorageUnit};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;
