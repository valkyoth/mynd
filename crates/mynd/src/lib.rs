#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Security-first image codecs and processing for Rust.
//!
//! Version 0.3.0 exposes checked arithmetic foundations. Image-model and codec
//! APIs remain unavailable until their implementation and security milestones
//! pass.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

/// Format-neutral image foundations.
pub use mynd_core as core;

/// Checked integer arithmetic for image-data boundaries.
pub use mynd_math as math;
