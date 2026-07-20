#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Security-first image codecs and processing for Rust.
//!
//! Version 0.1.0 is the repository foundation. Codec APIs are not available
//! until their implementation, conformance, and security milestones pass.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

/// Format-neutral image foundations.
pub use mynd_core as core;
