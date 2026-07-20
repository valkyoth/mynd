# mynd Version Plan

Status: planned integration train. Each version is a separate auditable pass;
unfinished work moves forward rather than being silently bundled.

Patch releases contain fixes only. Before 1.0, the facade follows this train;
changed support crates use independent versions and unchanged crates are not
republished. Every release updates support, specification, release-note, crate
matrix, package, and pentest evidence.

## Phase A: governance and image model

| Version | Deliverable | Primary security review |
| --- | --- | --- |
| 0.1.0 | Virtual workspace, facade/core skeletons, licensing, no_std baseline, policies and plans | Package contents, no unexpected dependency/build script/I/O/unsafe |
| 0.2.0 | Finalize threat model, trust boundaries, attacker capabilities, and non-goals | Memory, CPU, metadata, decompression, animation, parser confusion |
| 0.3.0 | Specification ledger and corpus-provenance schema | Copyright, source integrity, fixture licensing |
| 0.4.0 | Disclosure flow, supported-version policy, advisory template | Private reporting and maintenance clarity |
| 0.5.0 | Release manifest and tag-delta pentest checklist | Simulated audit completeness |
| 0.6.0 | `mynd-math`: checked conversion, add, multiply, align, and range | Extrema, zero, overflow, truncation |
| 0.7.0 | Validated dimensions, stride, rectangle, and output length | Area/stride/rectangle overflow and zero dimensions |
| 0.8.0 | Sample, channel, packing, and endianness domains | Invalid and unsupported combinations |
| 0.9.0 | Color model, alpha mode, pixel format, common constants | Ambiguous alpha, channel/palette invariants |
| 0.10.0 | Checked immutable and mutable image views | Short buffers, strides, last row, overlap |
| 0.11.0 | Frame rectangles, timing, disposal, blend, canvas types | Off-canvas frames, timing and canvas overflow |
| 0.12.0 | Fallible owned image under `alloc`; first core audit | Reservation failure, initialized length, panic audit |

## Phase B: I/O and codec contracts

| Version | Deliverable | Primary security review |
| --- | --- | --- |
| 0.13.0 | Slice reader/writer, I/O error, exact reads | Partial/empty reads, cursor overflow |
| 0.14.0 | Little/big-endian integer I/O | Every-byte truncation, sign conversion |
| 0.15.0 | Bounded, subrange, and counting readers | Nested bound escape and offset overflow |
| 0.16.0 | Seek/read-at traits with slice implementations | Beyond-source and backward seeks |
| 0.17.0 | Fixed writer and transactional checkpoints | Partial output and rollback |
| 0.18.0 | LSB/MSB bit readers | Shift width, refill, one-bit truncation |
| 0.19.0 | LSB/MSB bit writers | Padding, transitions, exact length |
| 0.20.0 | Optional `std::io` adapters | Partial/interrupted I/O and position agreement |
| 0.21.0 | Format ID, hints, media types, bounded probing | Collisions, spoofing, ambiguous prefixes |
| 0.22.0 | Decode limits, work budget, conservative presets | Cross-budget bypass and conversion |
| 0.23.0 | Structured errors, warning sink, reports | Logging injection, offsets, bounded warnings |
| 0.24.0 | Incremental decoder/encoder contracts; architecture freeze | Stalls and consumed/produced invariants |

## Phase C: BMP

| Version | Deliverable | Primary security review |
| --- | --- | --- |
| 0.25.0 | BMP probe and file header | Near signatures, short input, pixel offset |
| 0.26.0 | DIB size dispatch and unknown reporting | Header-size confusion and huge declarations |
| 0.27.0 | OS/2 BITMAPCOREHEADER | Narrow conversions and palette sizing |
| 0.28.0 | Windows BITMAPINFOHEADER | Signed height, planes, compression/depth |
| 0.29.0 | OS/2 2.x headers and unsupported subtypes | Header overlap and reserved values |
| 0.30.0 | Windows V4/V5 headers | Profiles, masks, color-space confusion |
| 0.31.0 | Shared validation and row layout | Padding, negative height, source end |
| 0.32.0 | 24-bit BI_RGB decode to caller RGB/BGR | Bottom-up order, padding, stride, truncation |
| 0.33.0 | 32-bit uncompressed decode and alpha policy | Unused byte versus alpha |
| 0.34.0 | 1-bit indexed decode | Bit order, tail bits, palette bounds |
| 0.35.0 | 4-bit indexed decode | Odd width and nibble order |
| 0.36.0 | 8-bit indexed decode | Palette count and short arrays |
| 0.37.0 | 16-bit default RGB decode | Expansion, endian, odd truncation |
| 0.38.0 | 16-bit bitfield decode | Zero/overlap/non-contiguous masks |
| 0.39.0 | 32-bit bitfields and alpha mask | Full-width masks and shifts |
| 0.40.0 | Complete top-down and palette-alpha policy | Signed minimum and reversed rows |
| 0.41.0 | RLE8 decode | Escapes, deltas, rows, expansion |
| 0.42.0 | RLE4 decode | Absolute padding, nibbles, odd count |
| 0.43.0 | Bounded profiles/metadata; embedded payload reporting | Nested offsets and overlap |
| 0.44.0 | Canonical uncompressed true-color/indexed encoder | Lengths, padding, palette count |
| 0.45.0 | RLE encoder and full BMP conformance/fuzz audit | Symmetry and malformed corpus marathon |

## Phase D: TGA

| Version | Deliverable | Primary security review |
| --- | --- | --- |
| 0.46.0 | Probe and 18-byte header | Weak signatures, type, ID length |
| 0.47.0 | 24-bit true-color decode | BGR, orientation, row truncation |
| 0.48.0 | 32-bit true-color and alpha | Attribute bits and output mismatch |
| 0.49.0 | 15/16-bit true-color | Expansion, attribute, endian |
| 0.50.0 | 8/16-bit grayscale | Depth combinations and luma-alpha |
| 0.51.0 | Color-mapped decode | Map origin, entry width, indices |
| 0.52.0 | Origins and supported interleaving | Coordinate mapping and duplicate rows |
| 0.53.0 | Generic bounded RLE packet parser | Zero progress and packet arithmetic |
| 0.54.0 | True-color RLE decode | Image/row bounds and truncation |
| 0.55.0 | Grayscale/indexed RLE decode | Lookup and raw/run packet bounds |
| 0.56.0 | Bounded image-ID handling | Non-text logging and preservation |
| 0.57.0 | TGA 2.0 footer and extension area | Offsets, sizes, contradictory metadata |
| 0.58.0 | Developer area, correction table, thumbnail | Counts, nested offsets, overlap |
| 0.59.0 | Canonical raw/RLE encoders | Packets, origins, deterministic output |
| 0.60.0 | Legacy compatibility and complete TGA audit | Cross-scanline RLE and differential corpus |

## Phase E: GIF decoding

| Version | Deliverable | Primary security review |
| --- | --- | --- |
| 0.61.0 | GIF87a/89a signature and version | Near signatures and truncation |
| 0.62.0 | Logical screen descriptor | Canvas area and packed/reserved bits |
| 0.63.0 | Global color table | Table length, truncation, no-allocation path |
| 0.64.0 | Incremental sub-block reader | Missing terminator, total bytes, chunking |
| 0.65.0 | Image descriptor and local table | Rectangle overflow and palette selection |
| 0.66.0 | LZW code extraction and LSB integration | Code size and bit transitions |
| 0.67.0 | Fixed LZW dictionary init/reset | Index bounds, clear behavior, stale state |
| 0.68.0 | Code-width growth and saturation | Exact transition and 12-bit maximum |
| 0.69.0 | Undefined-code special case and rejection | Previous state and forward references |
| 0.70.0 | Pixel accounting and exact frame size | Expansion bombs and no-progress loops |
| 0.71.0 | Four-pass deinterlacing | Tiny heights, duplicate/destination rows |
| 0.72.0 | Complete single-image decode | Palette indices and trailing data |
| 0.73.0 | Graphic Control Extension | Block size, reserved bits, terminator |
| 0.74.0 | Transparency index | Palette bounds and alpha conversion |
| 0.75.0 | Frame clipping and canvas placement | Rectangle addition and partial frames |
| 0.76.0 | Delay representation and timing policy | Zero and cumulative duration |
| 0.77.0 | Keep/do-not-dispose behavior | Retained canvas and ordering |
| 0.78.0 | Restore-to-background | Background selection and affected region |
| 0.79.0 | Restore-to-previous with bounded snapshots | Memory totals and snapshot lifetime |
| 0.80.0 | Raw-frame API | Local versus canvas coordinates |
| 0.81.0 | Composited-frame API | Disposal sequencing and reuse |
| 0.82.0 | Generic bounded extension state machine | Unknown labels and block bombs |
| 0.83.0 | Loops, comments, plain text, unknown extensions | Ordering and metadata limits |
| 0.84.0 | Full GIF decoder conformance/fuzz/differential audit | LZW state space and animation bombs |

## Phase F: quantization, encoding, facade, processing, and CLI

| Version | Deliverable | Primary security review |
| --- | --- | --- |
| 0.85.0 | Exact palettes and bounded histogram | Indexing, unique colors, determinism |
| 0.86.0 | Deterministic median cut | Empty boxes and split arithmetic |
| 0.87.0 | Palette remap and ordered dither | Distance overflow and work accounting |
| 0.88.0 | GIF LZW encoder | Width transitions and dictionary reset |
| 0.89.0 | Single-frame GIF encoder | Table sizes, transparency, sub-blocks |
| 0.90.0 | Animated GIF encoder | Canvas/frame totals and disposal round trip |
| 0.91.0 | Feature-gated registry and bounded probing | Ambiguity, disabled formats, ordering |
| 0.92.0 | Static `AnyDecoder` dispatch | Variant/error/state confusion |
| 0.93.0 | Unified `decode_into` facade | Limit propagation and partial output |
| 0.94.0 | Fallible owned decode under `alloc` | Reservation and warning collection |
| 0.95.0 | Unified encoder/writer facade | Selection and partial output |
| 0.96.0 | Basic color and alpha conversion | Rounding, division, premultiplication |
| 0.97.0 | Crop, flip, rotate, normalize, nearest resize | Overlap, rectangles, work budget |
| 0.98.0 | CLI inspect/validate/decode/convert/frame/limits | Paths, defaults, terminal injection |

## Phase G: stabilization and 1.0 admission

| Version | Deliverable | Primary security review |
| --- | --- | --- |
| 0.99.0 | Audit every crate with no default features | Accidental std/alloc/macros/panics |
| 0.100.0 | Complete core/alloc/std and encode/decode matrix | Invalid feature combinations |
| 0.101.0 | Workspace panic/assertion audit | Unwrap, indexing, divide, build-mode differences |
| 0.102.0 | Arithmetic/offset/cast/stride audit | Every input-derived calculation |
| 0.103.0 | Calibrate limits on normal/hostile corpora | Bypass and unusable defaults |
| 0.104.0 | Automated every-byte truncation suite | Stalls, partial state, error consistency |
| 0.105.0 | Corpus provenance, conformance matrix, differential suite | Unsupported claims and disagreements |
| 0.106.0 | Extended fuzz campaign and coverage review | Unreached states and cycles |
| 0.107.0 | Kani: math, views, bits, clipping | Assumptions and unwind bounds |
| 0.108.0 | Kani: BMP/TGA RLE and GIF LZW | Dictionary, packet, output, row invariants |
| 0.109.0 | Miri, sanitizers, unsafe inventory, memory audit | Alias, initialization, SIMD boundaries |
| 0.110.0 | DoS/performance, reproducibility, API docs, RC1 | Release arithmetic and package contents |
| 0.111.0 | External delta pentest, RC2, API freeze decision | Entire attack surface and open findings |

## 1.0.0

1.0.0 is a separate admission decision, not the automatic successor of
0.111.0. It requires the criteria in `docs/IMPLEMENTATION_PLAN.md`, no unresolved
critical/high findings, an independent review, and maintainer commitment to the
stable API.

## Post-1.0 order

New codecs start independently and become facade defaults only after maturity:
PNM, QOI, PNG/APNG, classic JPEG, TIFF/Exif, WebP, JPEG-LS, JPEG XL, AVIF/HEIF,
then JPEG 2000 and specialist families. Each receives its own detailed version
track before implementation; no future codec expands the 1.0 audit surface.
