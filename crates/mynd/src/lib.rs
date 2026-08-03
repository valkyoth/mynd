#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Security-first image codecs and processing for Rust.
//!
//! Version 0.4.0 adds validated dimensions, contained rectangles, plane
//! layouts, and output lengths to the checked arithmetic foundations. Pixel
//! formats, buffer views, and codecs remain unavailable until their milestones
//! pass.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

/// Format-neutral image foundations.
pub use mynd_core as core;

/// Checked integer arithmetic for image-data boundaries.
pub use mynd_math as math;
