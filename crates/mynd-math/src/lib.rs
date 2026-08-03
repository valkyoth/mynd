#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Checked integer arithmetic for Mynd's untrusted image-data boundaries.
//!
//! The crate provides constant-work, allocation-free primitives for integer
//! conversion, addition, multiplication, alignment, and bounded ranges. Every
//! failure is explicit; no operation wraps, saturates, truncates, or panics.

mod convert;
mod error;
mod ops;
mod range;

#[cfg(kani)]
mod proofs;

pub use convert::{
    checked_i64_to_u64, checked_i64_to_usize, checked_u64_to_i64, checked_u64_to_u32,
    checked_u64_to_usize, checked_usize_to_u64,
};
pub use error::{ArithmeticOperation, MathError, MathResult};
pub use ops::{
    checked_add_u64, checked_add_usize, checked_align_up_u64, checked_align_up_usize,
    checked_mul_u64, checked_mul_usize,
};
pub use range::{checked_range_u64, checked_range_usize};
