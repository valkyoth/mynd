# Validated Geometry and Plane Layout Contract

Status: v0.4.0 released geometry contract; extended, not replaced, by v0.5.0.

This document defines the format-neutral `mynd-core` 0.2.0 geometry boundary.
It is `no_std`, uses no allocation or unsafe Rust, and accepts no byte slice.
Format-specific representations remain governed by their authoritative
specifications when their implementation releases begin.

## Dimensions

`Dimensions::new(width, height)` accepts exactly two nonzero `u32` axes. It
retains the axes as `NonZeroU32` and stores their exact `u64` product. The
fixed-width pixel count therefore cannot overflow. `pixel_count_usize()` is a
separate fallible conversion and rejects counts that cannot be addressed on the
target.

This separation is intentional: a declaration can be structurally valid while
being too large for a 32-bit caller to address. Construction performs no
allocation and does not grant a future decode-limit decision.

## Contained rectangles

`ImageRect::new(bounds, x, y, width, height)` accepts only nonzero extents. Its
exclusive right and bottom ends must be representable by `u32` and no greater
than the retained bounding dimensions. The resulting type carries its bounds,
ends, and exact `u64` area, so callers do not need to repeat containment math.
`area_usize()` remains a fallible target-width conversion.

Validation order is stable:

1. zero rectangle width;
2. zero rectangle height;
3. exclusive-end overflow or containment failure.

## Plane layout

`PlaneLayout::new(offset, row_bytes, row_stride, rows, alignment)` takes
fixed-width byte quantities and validates them before conversion to `usize`.
Alignment means numeric divisibility of the plane offset and every row start;
it is not a claim that the value is valid for `core::alloc::Layout`.

Validation order is stable:

1. nonzero `row_bytes`;
2. nonzero `rows`;
3. nonzero `alignment`;
4. `row_stride >= row_bytes`;
5. `offset % alignment == 0`;
6. `row_stride % alignment == 0`;
7. checked last-row arithmetic;
8. target-`usize` conversion.

The exclusive required extent is exactly:

```text
offset + row_stride * (rows - 1) + row_bytes
```

Padding after the final used row is not charged. All operands and intermediate
results are checked. A successfully constructed plane therefore has nonzero,
target-representable fields and a nonzero `OutputLength`.

`checked_plane_output_len()` rejects an empty set, decreasing offsets, and
overlap. Adjacent planes and gaps are allowed. It returns the exclusive end of
the last plane in an ordered, nonoverlapping set.

## Explicit exclusions

The v0.4.0 API alone does not define pixel channels, sample storage,
endianness, packed bits, chroma subsampling, alpha association, palette
semantics, slice-backed views, allocation, format parsing, or decode/encode
limits. In particular, a standalone `PlaneLayout` remains storage-neutral.
Version 0.5.0 adds a separate validated pixel-to-plane relationship described
in [`pixel-storage.md`](pixel-storage.md); buffer access and later domains
remain excluded.

## Verification boundary

Rust tests cover zero/minimum/maximum dimensions, exact areas and exclusive
ends, containment failures, target-width conversion, last-row formulas,
alignment, stride errors, arithmetic overflow, ordered plane sets, gaps,
overlap, and 32-bit-only rejection paths.

Kani harnesses use no assumptions. They prove complete `u8` domains for
dimension acceptance and exact area, rectangle containment, plane validation,
multirow length, and the 32-bit pixel-count model; full `u32` one-row output
arithmetic; and explicit maximum and 32-bit boundary cases.
The reduced multi-field domains are stated rather than presented as full-width
exhaustion, because the corresponding unrestricted solver query is not part of
the release evidence.

| Requirement | Implementation | Rust evidence | Kani evidence |
| --- | --- | --- | --- |
| `GEO-DIM-01` Nonzero axes and exact fixed-width area | `dimensions.rs` | `geometry.rs` | dimension and pixel-count harnesses |
| `GEO-RECT-01` Nonempty retained-bounds containment | `rect.rs` | `geometry.rs` | reduced-domain and maximum-edge harnesses |
| `GEO-PLANE-01` Exact checked last-row extent | `plane.rs` | `planes.rs` | reduced multirow, full one-row, and boundary harnesses |
| `GEO-PLANE-02` Numeric alignment and stride validity | `plane.rs` | `planes.rs` | reduced validation harness |
| `GEO-SET-01` Ordered nonoverlapping plane sets | `plane.rs` | `planes.rs` | Rust tests; no dedicated Kani claim in v0.4.0 |
| `GEO-PORT-01` Target-width failure remains explicit | all geometry modules | native and 32-bit checks | 32-bit model and boundary harnesses |

## Authoritative language and verifier semantics

The representation choices follow the official Rust `core::num` documentation
for nonzero integer types and the official Kani proof-harness documentation:

- <https://doc.rust-lang.org/stable/core/num/index.html>
- <https://doc.rust-lang.org/core/num/struct.NonZero.html>
- <https://model-checking.github.io/kani/usage.html>
- <https://model-checking.github.io/kani/crates/doc/kani/index.html>

No image-format specification is applicable to this storage-neutral release.
Each future format crate must map its declarations to these primitives only
after pinning and reviewing that format's authoritative specification.
