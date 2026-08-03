# mynd 0.5.0 Release Notes

Status: released 2026-08-03; cumulative pentest, GitHub CI, and CodeQL PASS.

Release kind: Cumulative publication checkpoint

This release adds explicit format-neutral pixel-layout and sample-storage
domains. It does not add buffer access, allocation, format parsing, decoding,
encoding, metadata interpretation, color conversion, processing, or format
support.

## Public API

The first-party `mynd-core` 0.3.0 crate adds:

- `SampleStorage`, `SampleClass`, and `StorageUnit`, with explicit meaningful
  width, packed-bit order, and multi-byte word order;
- `PixelLayout` variants for gray, RGB, and YCbCr, with alpha-bearing variants
  structurally separated from non-alpha layouts;
- explicit interleaved channel order, straight or premultiplied alpha,
  planar/semi-planar organization, and 4:4:4, 4:2:2, 4:2:0, 4:4:0, 4:1:1,
  and 4:1:0 chroma domains;
- `PixelPlane` logical dimensions and checked exact tightly packed row bytes;
- concrete-plane validation that composes pixel relationships with the
  existing plane ordering, overlap, and output-length rules;
- allocation-free `StorageError` and `PixelLayoutError` classifications.

`mynd` 0.5.0 re-exports the API as `mynd::core`. The unchanged, dependency-free
`mynd-math` 0.1.0 crate is reused and is not republished.

## Security contract

A validated sample cannot have zero or excessive significant bits, an
undersized physical unit, a one-bit signed domain, or packed, byte-sized, or
padded floating storage. Alpha and chroma cannot be attached as independent
flags: only alpha-bearing variants contain alpha association, and only YCbCr
variants contain chroma sampling.

Layout variants fix plane count, order, channel count, and sampling divisors.
Odd chroma dimensions round upward. Row-byte multiplication and final-byte
rounding are checked, and concrete planes must match the exact logical height
and used bytes before overlap/output validation succeeds. The full invariants,
exclusions, Rust semantic sources, and requirement-to-evidence mapping are in
`docs/pixel-storage.md`.

## Explicit exclusions

The release does not define floating numeric behavior, transfer functions,
colorimetry, range conversion, chroma siting, palettes/indexes, CMYK, XYZ/Lab,
arbitrary channels, packed macropixels, tiled/swizzled storage, mixed-width
interleaved channels, or backing-buffer safety. Unsupported combinations have
no public layout variant and fail closed until separately admitted.

## Verification

- Rust tests cover sample extrema and validation order, physical-versus-
  significant widths, every admitted chroma factor with odd axes, final-byte
  rounding, channel row accounting, plane count/rows/bytes, overlap, facade
  exports, and out-of-range indices.
- Five new Kani harnesses use no assumptions and cover full `u8` sample-width
  domains plus full nonzero `u32` gray-row and all-variant chroma-axis
  relationships.
- Existing arithmetic, geometry, target-width, no_std, supported-Rust,
  platform, dependency, package, specification, and supply-chain gates remain
  mandatory.
- Runtime dependencies remain first-party only: `mynd-core` depends exactly on
  dependency-free `mynd-math` 0.1.0.

## Release requirements

- repository, Kani, supported-Rust, platform, latest-tool, dependency,
  RustSec, package, documentation, and SBOM gates pass;
- the cumulative external pentest covers the entire delta from published
  v0.4.0 and `security/pentest/v0.5.0.md` reaches `Status: PASS`;
- the final committed candidate passes GitHub CI, Kani, and CodeQL default
  setup before tagging;
- publish `mynd-core` 0.3.0 before `mynd` 0.5.0; do not republish
  `mynd-math` 0.1.0.

## Security review

The cumulative external pentest reported two related Low/Informational
hardening observations: chroma divisors were stored as plain integers, and the
full-domain Kani proof covered only 4:2:0. Channel counts and sampling divisors
are now `NonZeroU8`, Kani symbolically covers every chroma variant, and the
external retest is green.

The release gate also rejected a transient mismatched TIFF specification
response. The approved checksum remains unchanged; specification acquisition
now requests identity-encoded cache-revalidated bytes, and isolated recreation
uses bounded retries that still accept only the exact reviewed hash. Details
are recorded in `security/pentest/v0.5.0.md`.
