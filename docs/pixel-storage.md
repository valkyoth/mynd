# Pixel Layout And Sample Storage Contract

Status: released in v0.5.0 after cumulative pentest, GitHub CI, and CodeQL PASS.

This document defines the format-neutral `mynd-core` 0.3.0 storage boundary.
It is `no_std`, allocation-free, and safe Rust. It accepts no byte slice and
does not parse, decode, encode, render, or convert an image format.

## Sample storage

`SampleStorage::new(class, significant_bits, unit)` is the only constructor for
a sample storage domain. A successful value retains:

- one numeric class: unsigned integer, two's-complement signed integer, or a
  floating-point storage domain;
- a nonzero meaningful-bit count;
- one physical unit: a continuous packed-bit stream, one byte, or a 16-, 32-,
  or 64-bit word;
- bit order only for packed streams and byte order only for multi-byte words.

Integer significant width may be smaller than a byte or word. Packed integers
occupy exactly their declared width and are limited to 64 bits. Signed values
require at least a sign bit and one value bit. Floating storage must occupy a
complete 16-, 32-, or 64-bit word: packed, byte-sized, and padded floating
samples are rejected.

This release describes floating storage, not floating arithmetic. NaN,
infinity, signed zero, subnormal, rounding, FMA, tolerance, and backend behavior
belong exclusively to v0.5.1. Likewise, integer code-value interpretation,
transfer functions, range mapping, colorimetry, and conversion belong to later
milestones.

Validation order is stable:

1. nonzero significant bits;
2. packed-width or selected-unit capacity;
3. a signed value bit after the sign bit;
4. exact full-word floating storage.

## Pixel layout variants

`PixelLayout` variants encode valid relationships instead of accepting a bag
of independently combinable flags:

| Domain | Admitted organizations | Fixed relationship |
| --- | --- | --- |
| Gray | one plane | no chroma or alpha |
| Gray + alpha | interleaved or two planar planes | explicit order when interleaved; explicit alpha association |
| RGB | interleaved or R/G/B planar | explicit RGB/BGR order when interleaved; no chroma or alpha |
| RGB + alpha | interleaved or R/G/B/A planar | explicit four-channel order when interleaved; explicit alpha association |
| YCbCr | Y/Cb/Cr planar or Y plus interleaved chroma | explicit chroma sampling; explicit CbCr/CrCb semi-planar order |
| YCbCr + alpha | Y/Cb/Cr/A planar or Y/chroma/A semi-planar | full-resolution alpha and explicit alpha association |

Alpha association is either straight or premultiplied. It exists only in an
alpha-bearing layout variant. Chroma subsampling exists only in a YCbCr variant
and is one of 4:4:4, 4:2:2, 4:2:0, 4:4:0, 4:1:1, or 4:1:0. Planar channel order
is fixed by the variant documentation. A shared interleaved plane uses one
sample storage domain for all its channels; separate alpha planes may use a
different domain.

Unsupported combinations have no variant. This release intentionally excludes
indexed/palette pixels, CMYK, XYZ/Lab, arbitrary named channels, chroma siting,
packed macropixel orders such as YUY2, block/tiled/swizzled memory, mixed-width
interleaved channels, and format-specific aliases. A later release must add an
explicit typed variant and evidence before any such combination is accepted.

## Logical and concrete planes

`PixelLayout::plane_count()` and `plane(index)` expose the exact required
planes. `PixelPlane` retains its channel count and horizontal/vertical divisors
as `NonZeroU8`, plus its sample storage. A constructed plane therefore cannot
reach ceiling division with a zero channel count or divisor. Logical chroma
dimensions use integer ceiling division, so odd image axes never discard a
final chroma sample.

`PixelPlane::row_bytes()` computes the exact tightly packed used bytes:

```text
ceil(ceil(image_width / horizontal_divisor)
     * channels * storage_bits_per_sample / 8)
```

All multiplication and final-byte rounding use checked `u64` arithmetic.
Stride padding is not included in used row bytes.

`PixelLayout::validate_planes()` requires:

1. the exact plane count;
2. each plane's rows to equal its logical sampled height;
3. each plane's used row bytes to equal its exact tightly packed width;
4. the existing `PlaneLayout` ordering, non-overlap, and output-extent rules.

Stride padding and gaps between planes remain allowed. A valid result proves
only the declared pixel-to-byte geometry. It does not prove a backing buffer,
alignment suitable for allocation or typed dereference, format conformance,
resource-limit admission, or numeric/color correctness.

## Errors and security properties

`StorageError` and `PixelLayoutError` are allocation-free and do not retain or
format attacker-controlled operands. Validation performs no I/O, allocation,
pointer access, looping dependent on dimensions, saturating arithmetic, silent
truncation, or unsafe code. Plane validation loops only over the layout's fixed
maximum of four planes.

## Verification boundary

Rust tests cover zero and excessive widths, padded integer storage, signed and
floating constraints, order accessors, every chroma factor with odd axes,
packed final-byte rounding, interleaved and planar row accounting, exact plane
counts, row/height mismatch, overlap propagation, facade exports, and
out-of-range plane indices.

Kani harnesses use no assumptions. They prove byte and packed storage
validation across the full `u8` significant-width domain, full-word floating
validation, exact gray-u8 row bytes across the full nonzero `u32` width domain,
and nonzero ceiling dimensions for full `u32` axes and every
`ChromaSubsampling` variant. The chroma enum derives Kani's safe symbolic-value
support under verifier configuration, so a newly added variant automatically
enters the same proof.

| Requirement | Implementation | Rust evidence | Kani evidence |
| --- | --- | --- | --- |
| `PIX-SAMPLE-01` Only valid sample widths and unit/class combinations exist | `sample.rs` | `sample_storage.rs` | byte, packed, and float harnesses |
| `PIX-ALPHA-01` Alpha association exists only with alpha-bearing layouts | `pixel.rs` variants | `pixel_layout.rs` | encoded by enum variants |
| `PIX-CHROMA-01` Chroma factors exist only for YCbCr, remain nonzero, and round odd axes upward | `pixel.rs` | `pixel_layout.rs` | all-variant dimension harness |
| `PIX-PLANE-01` Organization fixes plane count, order, channels, and sampling | `pixel.rs` | `pixel_layout.rs` | enum and fixed-index construction |
| `PIX-ROW-01` Exact row bytes use physical storage width and checked arithmetic | `pixel.rs` | `pixel_layout.rs` | full-width gray-u8 harness |
| `PIX-BIND-01` Concrete planes match logical rows/bytes before geometry commit | `pixel.rs` | `pixel_layout.rs` | Rust relationship tests |

## Authoritative language semantics

No image-format specification is implemented or claimed by this release.
Generic representation and arithmetic choices use the official Rust standard
library documentation:

- <https://doc.rust-lang.org/stable/core/num/index.html>
- <https://doc.rust-lang.org/stable/std/primitive.u32.html#method.div_ceil>
- <https://doc.rust-lang.org/stable/std/primitive.u64.html#method.checked_mul>
- <https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html>
- <https://model-checking.github.io/kani/usage.html>
- <https://model-checking.github.io/kani/crates/doc/kani/derive.Arbitrary.html>

Format crates must separately pin and map their authoritative specifications
before translating format declarations into these types.
