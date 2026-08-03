# mynd 0.3.0 Release Notes

Status: release candidate; pentest PASS; awaiting GitHub CI and CodeQL.

This release introduces Mynd's first public implementation surface: checked,
constant-work integer arithmetic for values derived from hostile image input.
It does not add dimensions, layouts, buffers, allocation, parsing, decoding,
encoding, metadata, color, processing, or format support.

## Public API

The new dependency-free `mynd-math` 0.1.0 crate owns:

- exact `i64`, `u32`, `u64`, and `usize` conversions used at signed,
  fixed-width, and platform-width boundaries;
- checked `u64` and `usize` addition and multiplication;
- upward alignment to any nonzero numeric multiple, including non-powers of
  two, with zero and result overflow reported separately;
- checked half-open range construction that rejects end overflow and an end
  above its exclusive upper bound;
- allocation-free `MathError` and `ArithmeticOperation` enums.

`mynd` 0.3.0 re-exports this crate as `mynd::math`. `mynd-core` remains
byte-for-byte unchanged at 0.1.0 and is not republished.

## Security contract

No operation wraps, saturates, silently truncates, clamps, allocates, logs,
uses unsafe code, or depends on Cargo overflow/panic profiles. Error values do
not retain attacker-controlled operands. Numeric alignment deliberately does
not claim Rust memory-layout validity. Empty ranges are valid at the exclusive
upper bound, while an empty range beginning above that bound is rejected.

The exact invariants, exclusions, official Rust semantic sources, and
requirement-to-evidence mapping are recorded in `docs/math-primitives.md`.

## Verification

- 14 Rust tests cover facade exposure, signed/fixed/platform conversion
  boundaries, `u64`/`usize` extrema cross-products, const evaluation, exact and
  overflowing ranges, arbitrary non-power-of-two alignment, error formatting,
  every pair of `u8` alignment operands, and every reduced range tuple.
- 12 Kani harnesses prove full-width addition/range/conversion properties,
  complete reduced multiplication/alignment domains, and explicit maximum
  overflow domains without assumptions.
- Normal CI retains Rust 1.90.0 through 1.97.1 and Linux, Windows, FreeBSD,
  macOS, Android, and iOS target coverage. Kani 0.67.0 runs in a separate
  verification job.
- Runtime dependency policy remains first-party only; `mynd-math` itself has no
  dependency or feature-enabled code path.
- Package inspection confirmed that the external logo is not included, both
  licenses and the intended tests are present, and both archives build and
  test independently.

## Release requirements

- repository, Kani, supported-Rust, platform, latest-tool, dependency,
  RustSec, package, documentation, and SBOM gates pass;
- the exact-version pentest and remediation loop is complete with
  `security/pentest/v0.3.0.md` at `Status: PASS`;
- the final committed candidate passes GitHub CI, the Kani job, and CodeQL
  before tagging;
- publish `mynd-math` 0.1.0 before `mynd` 0.3.0; do not republish
  `mynd-core` 0.1.0.
