#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Format-neutral image foundations for the `mynd` ecosystem.
//!
//! Version 0.1.0 intentionally exposes no image model. Public types are added
//! in independently reviewed releases after their invariants and tests exist.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;
