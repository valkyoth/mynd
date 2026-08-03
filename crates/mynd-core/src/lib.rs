#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Format-neutral image foundations for the `mynd` ecosystem.
//!
//! Version 0.2.0 provides validated dimensions, contained rectangles,
//! platform-representable plane layouts, and exact output lengths. Pixel
//! storage semantics remain outside this release.

mod dimensions;
mod error;
mod plane;
mod rect;

#[cfg(kani)]
mod proofs;

pub use dimensions::Dimensions;
pub use error::{GeometryError, GeometryResult};
pub use plane::{OutputLength, PlaneLayout, checked_plane_output_len};
pub use rect::ImageRect;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;
