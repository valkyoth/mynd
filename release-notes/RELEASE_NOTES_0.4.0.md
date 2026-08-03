# mynd 0.4.0 Release Notes

Status: release candidate; pentest PASS; awaiting GitHub CI and CodeQL.

This release adds storage-neutral geometry and plane-layout foundations. It
does not add pixel formats, buffer access, allocation, parsing, decoding,
encoding, metadata, color, processing, or format support.

## Public API

The first-party `mynd-core` 0.2.0 crate now owns:

- `Dimensions`, with nonzero `u32` axes, an exact `u64` pixel count, and a
  fallible target-`usize` conversion;
- `ImageRect`, which retains its bounding dimensions and proves nonzero extents
  and exclusive ends are contained;
- `PlaneLayout`, which validates nonzero row fields, numeric alignment,
  `stride >= row_bytes`, target-width representation, and the exact last-row
  extent;
- `OutputLength` and `checked_plane_output_len`, which represent a nonzero
  addressable extent and reject empty, out-of-order, or overlapping plane sets;
- allocation-free `GeometryError` values that do not retain input operands.

`mynd` 0.4.0 re-exports this API as `mynd::core`. The unchanged `mynd-math`
0.1.0 dependency is reused and is not republished.

## Security contract

Construction validates fixed-width inputs before target-width conversion. No
operation allocates, dereferences a buffer, wraps, saturates, silently
truncates, or uses unsafe Rust. Plane alignment means numeric divisibility and
does not claim Rust allocator-layout validity. Gaps between ordered planes are
allowed; overlap is rejected.

The exact invariants, validation order, exclusions, official Rust/Kani semantic
sources, and requirement-to-evidence mapping are recorded in
`docs/geometry-layout.md`.

## Verification

- 30 Rust tests cover arithmetic and facade behavior plus zero/min/max
  dimensions, exact areas, rectangle containment, last-row formulas, alignment,
  stride failures, overflow, ordered planes, gaps, overlap, and 32-bit-only
  rejection paths.
- 22 Kani harnesses cover the existing arithmetic properties and new dimension,
  rectangle, plane, last-row, and target-width properties without assumptions.
  Full and reduced symbolic domains are stated explicitly in the contract.
- CI covers Rust 1.90.0 through 1.97.1; Linux, Windows, FreeBSD, macOS, Android,
  and iOS; plus explicit 32-bit Linux and WASM compilation.
- Runtime dependencies remain first-party only. `mynd-core` depends exactly on
  the dependency-free `mynd-math` 0.1.0.
- Package gates verify licenses, intended source/tests, independent archive
  tests, and exclusion of the externally hosted logo.

## Release requirements

- repository, Kani, supported-Rust, platform, latest-tool, dependency,
  RustSec, package, documentation, and SBOM gates pass;
- the exact-version pentest and remediation loop completes with
  `security/pentest/v0.4.0.md` at `Status: PASS`;
- the final committed candidate passes GitHub CI, Kani, CodeQL default setup,
  and the strict v0.4.0 release gate before tagging;
- publish `mynd-core` 0.2.0 before `mynd` 0.4.0; do not republish
  `mynd-math` 0.1.0.

## Security review

The exact-version pentest reported one Low correctness-clarity issue in an
unreachable, misleading output-length error fallback. The generic fallback was
removed; the final nonzero extent is now constructed through checked
target-width arithmetic without panic or unsafe code. The external retest was
reported green on 2026-08-03, and `security/pentest/v0.4.0.md` records
`Status: PASS`.
