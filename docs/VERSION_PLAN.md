# mynd Release Plan To 1.0

Status: active planning document

Mynd is currently governance scaffolding, not an image engine. The repository
has a no_std facade/core skeleton, security and release policy, and verification
tooling, but no image model, parser, codec, processing algorithm, or Rust
test-bearing behavior. Roadmap entries are not support claims; only implemented
behavior recorded in FORMAT_SUPPORT.md may be advertised.

This plan is granular because every image byte is hostile input. Each handoff
must be small enough to implement, test, review, pentest, and stop cleanly
before tagging. Split work whenever one safe review pass is insufficient.
Schedule pressure never permits scope to roll silently into the next version.

```text
v0.N.0       primary release in family N
v0.N.P       explicitly scoped additive/evidence patch in family N
v1.0.0-rc.N exact versioned production candidate
v1.0.0       first serious production-ready mynd crate
```

## Normative roadmap correction

This sequence replaces the former v0.25.0-v0.111.0 roadmap. Pre-1.0 scope now
contains BMP, QOI, Netpbm, farbfeld, PNG/APNG, GIF, declared classic-JPEG
profiles, WebP, and declared TIFF 6.0 profiles. TGA and formats not expressly
admitted below are outside the 1.0 claim.

Version 0.2.0 reconciles this scope across README.md, FORMAT_SUPPORT.md,
docs/IMPLEMENTATION_PLAN.md, SPEC_SOURCES.md, crate planning, and release
automation before implementation proceeds. Until then, the narrower statement
wins and no codec support is claimed.

Each .x family has one exclusive outcome. Every named patch version has its own
artifacts, evidence, release notes, exact-commit pentest, clean retest, and tag
decision. A failed format audit delays later work; it cannot be hidden in the
next format.

## Conformance claims

“100% compliant” is never unconditional. A finite decoder may reject a
conforming image that exceeds documented limits; that is LimitExceeded, not
Malformed. Claims name an edition and profile and are split as follows:

| Claim | Required meaning |
| --- | --- |
| Structural | Every normative order, length, reserved bit, cross-field rule, checksum, and terminator in the profile maps to code and tests. |
| Decoder | Every conforming stream in the profile decodes unless an explicit resource limit is exceeded. |
| Encoder | Every emitted stream conforms; optional encoding processes are claimed separately. |
| Editor/transcoder | Unknown metadata, safe-to-copy rules, and information loss follow declared policy. |
| Color | Native samples, declarations, alpha association, intent, and output encoding remain explicit. |
| Operational safety | Malformed input ends within budgets without panic, hang, ambiguous partial success, hidden I/O, or unaccounted work. |

Every codec owns a machine-readable SPEC_MAPPING containing requirement ID,
edition, disposition, implementation path, positive and negative tests, fuzz
target, proof reference where applicable, and unsupported reason. A prose
ledger alone cannot authorize completion.

## Normative source baselines

Exact editions, acquisition records, hashes, errata, and redistribution rules
are pinned when implementation begins.

| Family | Required baseline | Explicit scope |
| --- | --- | --- |
| PNG/APNG | [ISO/IEC 15948:2004](https://www.iso.org/standard/29581.html), [W3C PNG Third Edition](https://www.w3.org/TR/png-3/), its [errata](https://www.w3.org/2025/06/REC-PNG-20250624-errata), RFC 1950, RFC 1951 | APNG, cICP/HDR/WCG, eXIf, precedence, private/unknown chunks, editor rules |
| JPEG | [ITU-T T.81](https://www.itu.int/rec/T-REC-T.81/en), corrigenda, separately scoped JFIF/T.871, Exif, ICC APP2, Adobe conventions | Sequential, progressive, lossless, arithmetic, differential, hierarchical, precision, encoders |
| WebP | [RFC 9649](https://www.rfc-editor.org/info/rfc9649/), [RFC 6386](https://www.rfc-editor.org/rfc/rfc6386.html), [VP8L](https://developers.google.com/speed/webp/docs/webp_lossless_bitstream_specification) | VP8, VP8L, ALPH, VP8X, animation, metadata, unknown chunks, order |
| GIF | GIF87a and [GIF89a](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) | Normative blocks versus Netscape/de facto extensions |
| TIFF | TIFF 6.0, Technical Notes, corrected JPEG-in-TIFF rules | Strips, tiles, planes, pages, compression, predictors, JPEG variants, color, ICC, tags; BigTIFF is separate |
| BMP | Microsoft DIB documentation for every admitted OS/2 and Windows family | Dialects, masks, RLE, V4/V5 color, embedded profiles, inert linked paths |
| Netpbm | [PBM/PGM/PPM](https://netpbm.sourceforge.net/doc/pnm.html) and PAM documents | Plain/raw syntax, comments, concatenation, MAXVAL, 16-bit, PAM inclusion |
| QOI | Author's [QOI specification](https://phoboslab.org/log/2021/12/qoi-specification) | Pixel termination, marker, wraparound, hints, trailing data |
| farbfeld | [suckless definition](https://tools.suckless.org/farbfeld/) | RGBA16-BE, unassociated alpha, exact length, trailing data |
| Shared color and blending | Pinned ICC v2 and ICC.1:2022/v4.4 sources, sRGB, BT.601/709/2020, H.273, PQ/HLG when claimed, CIE XYZ/Lab, Porter-Duff, and the selected artistic-blend specification | Edition, numeric domain, rounding, alpha, gamut, tolerance, and unsupported operations |

## Format-profile decisions visible in release gates

- BMP gates decide BI_JPEG/BI_PNG embedded payloads and every admitted OS/2
  variant, including RLE24 and Huffman 1D, while linked profiles remain inert.
- GIF gates name Comment, Plain Text, Application, and unknown extensions and
  decide missing EOI/trailer, extra pixels, zero delay, and reserved disposal.
- JPEG gates cover DNL, DAC, restart reset, abbreviated tables, multiscan
  sequential streams, component limits, table redefinition, native
  samples/coefficients, and JFIF/Adobe interpretation. Decoder completeness
  never implies that every encoder process is claimed.
- WebP gates separate still/animated containers and name ANMF rectangles,
  blend/dispose, ALPH filter/compression, ICCP, EXIF, XMP, encoder
  quality/effort controls, and deterministic mode.
- TIFF gates name FillOrder, Orientation, ExtraSamples, SampleFormat, signed and
  IEEE-float samples, every predictor, YCbCr dependencies/siting, sparse strips,
  and overlapping strip/tile policy. BigTIFF stays separate.
- Netpbm gates define the exact header-to-raw-raster boundary, concatenated
  images, trailing material, and unknown PAM tuple policy.

## Fail-closed architecture

```text
ByteSource
  -> BoundedReader / SubrangeReader
    -> MsbBitReader or LsbBitReader
      -> format-local entropy machine
        -> bounded sample/coefficient sink
          -> validated output view
```

- Keep external positions fixed-width and convert to usize only after
  representability and source-bound checks.
- Check lengths, rectangles, strides, allocations, shifts, and output sizes
  before conversion or slicing.
- Validate bit widths before shifts; width zero is explicit and refill is
  transactional.
- Every loop has a structural bound or spends a monotonic work budget.
- Incremental steps report consumed/produced units and Progress, NeedInput,
  NeedOutput, or Done; zero/zero Progress is an invariant failure.
- Nonterminal EOF is Truncated; unsupported normative behavior is Unsupported;
  bad data is Malformed; exhaustion is LimitExceeded.
- Output commits at declared rows, frames, or checkpoints, yielding a known
  valid prefix or no visible partial output.
- Share bounded bit I/O and proven canonical tables, not a universal entropy
  decoder. Deflate, GIF/TIFF LZW, JPEG entropy, VP8, and VP8L stay format-local.
- Core has no hidden allocation, dynamic registry, TLS, globals, filesystem,
  network, environment, clock, threads, or runtime.

## Foundational data, color, and numeric contracts

The model separates PixelLayout (channels, samples, nominal/storage depth,
packing, endian, per-plane offsets/strides, chroma subsampling/siting, and
alpha), ColorEncoding (model, primaries, white point, transfer, matrix, range,
intent, ICC, HDR/WCG, known state), and ImageMetadata (Exif, XMP, text, timing,
dimensions, and bounded raw blocks).

Storage and computation domains explicitly cover packed U1/U2/U4, U8, U16,
required signed-integer working samples, indexed values plus palette-entry
layout, and F32. Interleaved and planar storage are distinct. F32 policy names
NaN, infinity, signed zero, subnormal, saturation, rounding, and conversion
behavior at every public boundary.

Codec APIs preserve native samples, declarations, and unsupported valid
profiles; they never silently assume sRGB or execute metadata. The shared
scalar color and ICC foundations land before complex codecs. Container releases
may transport profiles and apply precedence only through those shared APIs.

Numeric evidence uses four tiers:

| Tier | Scope |
| --- | --- |
| Bit-exact | Parsing, lossless codecs, integer color paths, and integer compositing |
| Normative-tolerance | JPEG IDCT and other specification-tolerance lossy reconstruction |
| Reference-tolerance | Floating ICC, resize, SIMD, and GPU paths |
| Backend/config deterministic | Heuristic encoders with a declared backend, settings, search, and seed policy |

Every algorithm records rounding, saturation, coefficient precision, permitted
FMA use, backend constraints, and comparison tolerance. Cross-platform
bit-identity is claimed only for the bit-exact tier.

Safe validated byte views and naturally typed caller buffers are the baseline;
forbid(unsafe_code) rules out arbitrary byte-to-pixel casts. Future unsafe SIMD,
reinterpretation, or arena support belongs in a tiny optional independently
audited adapter behind a scalar reference.
## Resource accounting, scratch, and execution

The resource ledger has three non-confusable components:

1. Monotonic cumulative counters for input, output, work, decoded pixels,
   metadata processed, warnings, diagnostics, and state transitions.
2. Live reservations for currently held scratch, coefficient buffers, tables,
   frame snapshots, and other memory. Only these reservations may be released.
3. Peak gauges recording maximum concurrent live memory and other declared
   high-water marks.

All components are non-Clone and parent-backed. Child streams and parallel
workers receive grants that cannot mint capacity; unused live grants return to
the parent without refunding cumulative work or output. Caller-owned output is
charged and no preset is unlimited.

| Group | Counters |
| --- | --- |
| Input | total/compressed/probe/skipped bytes, seeks, seek distance |
| Shape | dimensions, pixels, rows/planes, frames, total frame pixels |
| Structure | chunks, scans, progressive refinements, table entries/bytes, palettes, IFDs, strips, tiles, nesting, state transitions |
| Output | decoded/encoded bytes, metadata output, preserved unknown bytes |
| Memory | persistent state, typed scratch, snapshots, coefficients, ICC, live and peak |
| Work | entropy symbols, back-references, filters, IDCT, coefficient refinements, taps, composition |
| Metadata | compressed and decompressed bytes separately, strings, warnings, diagnostics, Exif, ICC tags |
| Compatibility | admitted repairs/extensions, never safety relaxation |

ScratchRequirements describes size, alignment, lifetime class,
persistent/transient class, and whether a region may alias output. Safe core
implementations use byte-oriented algorithms where suitable and codec-specific
typed workspaces such as GifWorkspace, DeflateWorkspace, and JpegWorkspace
where typed tables or coefficients are required. Generic byte arenas never
promise safe typed reinterpretation. Owned convenience storage is initialized;
the plan does not rely on exposing uninitialized Vec capacity.

DecodePlan, ScratchPlan, and EncodePlan validate, grant, and calculate maxima
without mutating output. APIs are tiered into borrowed inspection,
caller-buffer execution, and optional fallible-owned convenience.

## Metadata and selective decoding contract

Container milestones initially preserve bounded raw Exif, ICC, and XMP
transport. Structured v1 inspection is added later through a shared TIFF/Exif
IFD parser, selected Exif fields, opaque MakerNotes, bounded string values,
explicit thumbnail limits, and a declared XMP raw-packet versus bounded-XML
policy.

Exif orientation is metadata until a caller explicitly requests normalization.
Transcoders choose discard, inspect, preserve raw, parse selected namespaces,
or rewrite; conflict and precedence policy is explicit.

Decoder traits are frozen with optional planning hooks for metadata/header-only
termination, region-of-interest, reduced-resolution output, strip/tile
selection, progressive preview events, animation frame ranges, and
scale-during-decode. Support is format-specific and claimed only when its later
0.94.x handoff passes.
## Processing, adapters, and sanitization

- Resize and interpolation declare their color domain. Production defaults
  account for linear-light filtering, premultiplication before filtering alpha,
  zero/near-zero unpremultiplication, and gamut handling after interpolation.
- Pull streaming is used only where valid; affine, vertical, and animation
  operations disclose cache, random-access, or snapshot requirements.
- Scalar algorithms are authoritative. SIMD/GPU adapters are optional and
  differential-tested across widths, alignments, tails, channels, FMA policy,
  coefficient precision, and execution paths.
- Callers schedule parallel work after parent-backed source, destination, and
  budget grants. Results follow the declared numeric tier.
- Async stays outside parsers and supplies backpressure and cancellation.
- CLI/service output is transactional; metadata/path text is escaped; color,
  loss, and genuine zero-copy conditions are disclosed.
- Pixels are non-secret by default. Clearing initialized sensitive scratch is
  best effort under safe Rust; Drop, abort, and workspace profiles are never
  represented as guaranteed erasure.
## Platform contract

Libraries start no_std with empty defaults and explicit alloc/std layers. The
target intent covers Linux, Windows, BSD, macOS, Android, iOS, wasm32, and
preferably WASI/component builds. Stack ceilings are measured.

Aesynx remains an architecture constraint until a real target, allocator
behavior, and CI runner exist. APIs may not assume conventional processes,
filesystems, threads, wall clocks, or global allocation.

## Release and pentest rules

Every release has one outcome and the Status, Context, Goal, Deliverables,
Verification, and Exit criteria blocks below. Official/original sources,
limits, unsupported behavior, compatibility, loss, local/adversarial evidence,
release notes, crate metadata, package inspection, SBOM, exact-commit pentest,
CI, and CodeQL are mandatory.

Implementation completion is a stop, not tag authority. Before every milestone,
patch, RC, or stable tag:

- scripts/checks.sh, latest-crate/tool checks, Rust/feature/platform gates,
  cargo-deny, cargo-audit, and SBOM verification pass;
- mappings, support claims, limits, corpus records, and release notes match;
- security/pentest/vX.Y.Z.md records the exact reviewed 40-character commit,
  tester, date, scope, findings, residual risk, and PASS;
- root PENTEST.md findings are fixed and removed, then cleanly retested;
- CI and CodeQL default setup are green;
- strict release metadata and the version-specific gate pass;
- the tag is absent and will point to the approved report commit.

Any code, dependency, metadata, documentation, or package change invalidates
affected approval. At each exit, print the exact stop line and do not tag.

## Crate versioning

The mynd facade is the integration train. Changed support crates use independent
versions; unchanged crates are not republished. Update release-crates.toml,
the crate-version matrix, changelog, notes, package inventory, SBOM, and
pentest metadata atomically.

Crates enter only at their first handoff. Intended inward layering is
mynd-core, mynd-budget, mynd-io, mynd-metadata, mynd-color, mynd-codec,
one mynd-format crate per family, mynd-processing, and mynd. Optional outer
adapters include alloc/std, async, Rayon, WASM, GPU, and CLI.

## Milestone summary

| Version | Exclusive capability | Mandatory evidence |
| --- | --- | --- |
| 0.1.0 | Existing workspace, licenses, feature boundaries, release policy | Current checks plus completed exact-commit pentest |
| 0.2.0 | Unified scope, claim taxonomy, standards/errata ledger, corpus provenance schema | No contradictions across README, support matrix, and normative plans |
| 0.3.0 | Checked conversion/add/multiply/align/range primitives | Exhaustive extrema tests and Kani arithmetic proofs |
| 0.4.0 | Validated dimensions, rectangles, strides, planes, and output lengths | Zero/min/max, last-row, alignment, and 32-bit usize proofs |
| 0.5.0 | Explicit pixel layout and sample-storage domains | Invalid layout/sample/plane/chroma/alpha combinations are unrepresentable |
| 0.5.1 | Numeric determinism and floating-sample contract | Bit-exact/tolerance/backend tiers plus rounding, saturation, FMA, NaN, infinity, zero, and subnormal tests |
| 0.5.2 | Shared color and blending specification ledger | Pinned ICC, sRGB, BT.601/709/2020, H.273, HDR, CIE, Porter-Duff, and blend sources with claim scope |
| 0.5.3 | Scalar transfer, matrix/range, and alpha foundation | Integer/scalar reference vectors, premultiplication, range conversion, rounding, and native-sample preservation |
| 0.6.0 | Immutable/mutable image and plane views | Short-buffer, row-boundary, alias-policy, and Miri tests |
| 0.7.0 | Frames, timing, canvas, disposal, blend, and frame rectangles | Off-canvas and cumulative-duration tests |
| 0.8.0 | Allocation-free structured errors, warnings, reports, and offsets | Bounded formatting and terminal/log-injection tests |
| 0.9.0 | DecodeLimits, EncodeLimits, monotonic work/memory ledger | Budget-sharing and bypass property tests |
| 0.10.0 | Caller-owned scratch planner, arenas, buffer pools, and leases | Peak-memory accounting and failed-reservation tests |
| 0.11.0 | Slice reader/writer, exact reads, fixed output, checkpoints | Every-byte truncation and rollback tests |
| 0.12.0 | Endian I/O, subranges, counting, seek/read-at | Nested-bound escape, offset, and seek-cycle tests |
| 0.13.0 | MSB/LSB bit readers and writers | Every-bit truncation, width, refill, and shift proofs |
| 0.14.0 | Incremental decoder/encoder progress contracts | Chunk-boundary equivalence and zero-progress rejection |
| 0.15.0 | Metadata envelopes and bounded Exif/ICC/XMP header transport | Offset/count validation without full metadata interpretation |
| 0.15.1 | Bounded ICC v2/v4 structural parser | Tag counts/sizes/offsets/overlap, curves, LUT dimensions, recursion, opaque preservation, and fuzzing |
| 0.15.2 | ICC matrix/TRC and chromatic-adaptation engine | Parametric curves, PCS conversion, adaptation, rendering intent, deterministic scalar vectors, and limits |
| 0.15.3 | ICC LUT, mAB/mBA, PCS Lab/XYZ, and v4 execution | Element counts, interpolation, intent, recursion, numeric tolerances, and independent profile tests |
| 0.16.0 | Format IDs, media types, bounded probing, static registry | Collision, ambiguity, polyglot, and disabled-feature tests |
| 0.17.0 | Fallible owned storage and std::io adapters | Allocation-failure, interrupted-I/O, and feature-matrix tests |
| 0.18.0 | Foundation API freeze, fuzz harness primitives, Kani core suite | External design review and no-default/32-bit/WASM build matrix |
| 0.19.0 | Common codec crate template and decode-plan contract | A dummy codec proves limit, scratch, progress, and rollback invariants |
| 0.20.0 | BMP probe, file header, OS/2 and Windows DIB dispatch | Header-size confusion and offset corpus |
| 0.21.0 | BMP BI_RGB depths, palettes, padding, row orientation | 1/4/8/16/24/32-bit golden and truncation tests |
| 0.22.0 | BMP bitfields, alpha masks, top-down rules | Mask overlap/gap/full-width and signed-height tests |
| 0.23.0 | BMP RLE4/RLE8 | Escape, delta, padding, exact-output, and no-progress fuzzing |
| 0.24.0 | BMP V4/V5 color declarations and embedded-profile transport | Profile-range, overlap, and linked-profile no-I/O tests |
| 0.25.0 | BMP deterministic uncompressed encoders | Dialect-specific golden files, exact headers/padding, determinism, and round trips |
| 0.25.1 | BMP deterministic RLE4/RLE8 encoders | Escape/padding/delta policy, deterministic packets, bounded work, and decode/encode round trips |
| 0.25.2 | Complete declared BMP dialect audit | BI_RGB/bitfield/RLE encoders, OS/2 decisions, embedded BI_JPEG/BI_PNG policy, differential, corpus, and fuzz review |
| 0.26.0 | QOI structural parse and bounded decoder | Pixel count, wraparound, end-marker, trailing-data tests |
| 0.27.0 | QOI deterministic encoder | Reference-vector and encode/decode conformance |
| 0.28.0 | Bounded Netpbm tokenizer | Comment, whitespace, decimal overflow, token-length fuzzing |
| 0.29.0 | PBM P1/P4 decode/encode | Bit order, row padding, multi-image policy |
| 0.30.0 | PGM P2/P5 decode/encode | MAXVAL scaling, 8/16-bit, truncation |
| 0.31.0 | PPM P3/P6 decode/encode | Sample scaling, token bombs, binary boundaries |
| 0.32.0 | PAM P7, if the public claim is “Netpbm” | Tuple types, depth, header termination, unknown fields |
| 0.33.0 | Combined PNM/PAM stream and conformance audit | Concatenated images and official-tool differential tests |
| 0.34.0 | farbfeld decode and encode | Exact-size arithmetic, RGBA16-BE, alpha semantics |
| 0.35.0 | Simple-codec security and API freeze | Cross-codec probe fuzzing, 32-bit memory tests, external delta review |
| 0.36.0 | PNG source map, signature, chunk state machine, CRC, ordering | Unknown-critical and CRC policy tests |
| 0.37.0 | PNG IHDR and color-type/bit-depth validation | Full normative combination matrix |
| 0.38.0 | zlib wrapper plus stored/fixed-Huffman Deflate | RFC vectors, Adler-32, bit truncation |
| 0.39.0 | Dynamic Huffman and complete bounded Deflate window | Tree proofs, distance/overlap fuzzing, output bombs |
| 0.40.0 | PNG filters and noninterlaced core color decoding | Per-filter/sample golden vectors |
| 0.41.0 | Packed 1/2/4-bit and 16-bit PNG samples | Scaling, endian, tail-bit tests |
| 0.42.0 | Adam7 decode and progressive row events | Pass geometry proofs and tiny-image corpus |
| 0.43.0 | PNG palette, tRNS, bKGD, hIST, sBIT, sPLT | Palette/index/transparency conformance |
| 0.44.0 | PNG cHRM/gAMA/sRGB/iCCP/cICP/mDCV/cLLI and precedence | ICC bombs, precedence matrix, HDR/WCG vectors |
| 0.45.0 | PNG text, compressed text, eXIf, pHYs, tIME, unknown/private chunks | Metadata decompression and safe-to-copy tests |
| 0.46.0 | APNG decoding | APNG state, frame, timing, disposal, blend, streaming, and animation-bomb tests |
| 0.46.1 | PNG deterministic encoding | PNG Third Edition emission, round-trip, filter, Deflate, metadata, and determinism tests |
| 0.46.2 | APNG deterministic encoding | Frame-sequence, rectangle, timing, disposal/blend, and decode/encode round trips |
| 0.46.3 | Complete PNG/APNG conformance and security audit | Third Edition conformance, differential, streaming, fuzz review |
| 0.47.0 | GIF87a/89a structure, palettes, sub-blocks, descriptors | Block termination and palette bounds |
| 0.48.0 | GIF LZW | Dictionary/code-width proofs and fuzzing |
| 0.49.0 | GIF single-frame decode and deinterlace | Exact pixels and four-pass geometry tests |
| 0.50.0 | GIF GCE, transparency, frame composition, all disposal modes | Snapshot caps and animation bomb corpus |
| 0.51.0 | GIF named extensions and compatibility policy | Comment/Plain Text/Application/unknown blocks plus loop, EOI, trailer, delay, and disposal policy |
| 0.51.1 | GIF raw-frame and composited-frame APIs | Coordinate, palette, disposal sequencing, valid-prefix, snapshot, and frame-range tests |
| 0.51.2 | Exact palettes and bounded histogram | Unique-color limits, palette-entry layout, deterministic ordering, overflow, and caller-palette tests |
| 0.51.3 | Deterministic palette generation and remapping | Declared median-cut policy, palette remap, ordered/error-diffusion modes, budgets, and golden vectors |
| 0.51.4 | Single-frame GIF encoder | Caller/generated palette paths, table sizing, transparency, sub-blocks, LZW, and deterministic round trips |
| 0.51.5 | Animated GIF encoder | Canvas/frame totals, timing, loop extension, disposal, frame ranges, and animation round trips |
| 0.51.6 | Complete GIF conformance and security audit | Normative/de-facto separation, missing terminator policy, differential corpus, LZW fuzzing, and animation bombs |
| 0.52.0 | JPEG source map, marker/segment parser, frame/scan/table declarations | Marker mutation and segment-size fuzzing |
| 0.53.0 | JPEG Huffman entropy, stuffing, restart, bounded coefficients | Canonical table proofs and MCU accounting |
| 0.54.0 | Baseline scalar IDCT, sampling, upsampling, grayscale/YCbCr decode | Tolerance vectors and restart corpus |
| 0.55.0 | Extended sequential and 12-bit DCT processes | Precision and coefficient-range evidence |
| 0.56.0 | Progressive DC/AC and successive approximation | Scan-order/state-machine fuzzing |
| 0.57.0 | Lossless predictive JPEG process | Predictor, point transform, precision tests |
| 0.58.0 | JPEG arithmetic coding | Conditioning-table and arithmetic-state proofs |
| 0.59.0 | Differential and hierarchical processes | Frame dependency and reconstruction bounds |
| 0.60.0 | JFIF, Exif, ICC APP2 assembly, Adobe RGB/CMYK/YCCK | Segment reassembly, precedence, and color vectors |
| 0.61.0 | Baseline JPEG encoder | Deterministic valid baseline emission, quality controls, coefficient limits, and round trips |
| 0.61.1 | Progressive JPEG encoder | Scan scripts, successive approximation, deterministic tables, restart policy, and round trips |
| 0.61.2 | Extended-sequential and lossless JPEG encoders | Precision/process-specific valid emission, predictor/point-transform policy, and differential tests |
| 0.61.3 | Arithmetic, differential, and hierarchical JPEG encoder admission | Each process is either independently evidenced or explicitly unclaimed with no decoder-claim ambiguity |
| 0.62.0 | Complete declared T.81 conformance and security audit | Reference software, official material, long fuzz campaign |
| 0.63.0 | WebP RIFF/VP8X container, chunk order, metadata | RFC 9649 conformance and size/padding fuzzing |
| 0.64.0 | VP8 Boolean decoder, partitions, headers, probabilities | Partition limits and arithmetic-state fuzzing |
| 0.65.0 | VP8 prediction, coefficients, inverse transforms, loop filters | Scalar reference differential tests |
| 0.66.0 | VP8 alpha and Y′CbCr-to-RGB pipeline | ALPH preprocessing and color tests |
| 0.67.0 | VP8L prefix coding, LZ77, color cache | Prefix/distance/cache proofs and bombs |
| 0.68.0 | VP8L transforms and complete lossless reconstruction | Transform recursion and meta-image limits |
| 0.69.0 | WebP animation decoding | ANMF rectangles, duration, blend/dispose, frame limits, and animation fuzzing |
| 0.69.1 | VP8L deterministic encoder | Prefix/LZ/cache/transform validity, quality-effort controls, bounded search, determinism, and round trips |
| 0.69.2 | VP8 deterministic encoder | Prediction/partition/token validity, quality-effort controls, bounded heuristics, backend determinism, and differential tests |
| 0.69.3 | Animated WebP encoder | ANMF ordering/rectangles, mixed frame modes, blend/dispose, metadata, and round trips |
| 0.69.4 | Complete WebP conformance and security audit | RFC/VP8/VP8L mappings, still/animated split, ALPH/metadata, encoder modes, long fuzzing, and external review |
| 0.70.0 | TIFF byte order, header, typed values, bounded IFD graph | Offset cycles, overlaps, entry/count multiplication |
| 0.71.0 | Baseline strips: bilevel, Gray, palette, RGB, uncompressed/PackBits | Strip-size and row-layout tests |
| 0.72.0 | TIFF LZW, Deflate, and horizontal predictors | Dialect policy and decompression bombs |
| 0.73.0 | TIFF CCITT RLE, Group 3, and Group 4 | Fax transition/run proofs and differential corpus |
| 0.74.0 | Tiles, planar layouts, multipage/SubIFD traversal | Tile geometry, IFD cycles, aggregate limits |
| 0.75.0 | TIFF YCbCr, CMYK, CIELab, alpha, ICC | Photometric/tag dependency and color vectors |
| 0.76.0 | Corrected JPEG-in-TIFF, Exif IFDs, admitted extensions | Old/new JPEG distinction and nested offsets |
| 0.77.0 | TIFF baseline strip encoder | Big/little-endian uncompressed baseline strips, tags, exact sizes, and round trips |
| 0.77.1 | TIFF compressed-strip encoders | PackBits/LZW/Deflate/fax process validity, predictors, bounds, dialect policy, and round trips |
| 0.77.2 | TIFF tile, planar, and multipage encoders | Tile geometry, plane offsets, IFD/SubIFD graph, aggregate limits, determinism, and round trips |
| 0.77.3 | TIFF extended color and JPEG encoders | Photometric dependencies, ExtraSamples, SampleFormat, YCbCr/ICC, corrected JPEG rules, and claims |
| 0.77.4 | Complete declared TIFF 6.0-profile audit | Compression/profile matrix, sparse/overlap policy, differential corpus, fuzzing, conformance, and external review |
| 0.78.0 | Cross-format native-sample and rendered-color conformance | Codec declarations use the shared scalar color engine with no implicit sRGB or ambiguous alpha |
| 0.79.0 | Shared bounded TIFF/Exif IFD inspection | Offset graphs, entry counts, cycles, value bounds, MakerNote opacity, and fuzzing |
| 0.80.0 | Selected Exif fields, thumbnails, and orientation policy | Dimensions/timestamps/strings/thumbnails are bounded and orientation is never silently applied |
| 0.81.0 | XMP inspection and metadata conflict/rewrite policies | Raw-versus-bounded-XML decision, precedence, preserve/discard/rewrite, and decompression limits |
| 0.82.0 | YCbCr matrices, ranges, subsampling, and chroma siting | JPEG/WebP/TIFF reference vectors |
| 0.83.0 | CMYK, YCCK, Gray, Lab, and wide-gamut conversion | Black-generation policy and gamut tests |
| 0.84.0 | Straight/premultiplied alpha conversion | Zero-alpha, rounding, and invariant tests |
| 0.85.0 | Conversion planning, sample-depth changes, and advanced dithering | Declared information loss, numeric tier, scratch/work plan, and deterministic output |
| 0.86.0 | Crop, flip, rotate, transpose | In-place overlap and rectangle proofs |
| 0.87.0 | Checked affine geometry and border modes | Finite-matrix and coordinate-overflow proofs |
| 0.88.0 | Nearest and bilinear resampling | Pixel-center and edge-policy golden tests |
| 0.89.0 | Bicubic resampling | Coefficient normalization and overshoot policy |
| 0.90.0 | Lanczos3 resampling | Tap planning, ring-buffer limits, reference vectors |
| 0.91.0 | Porter-Duff compositing | Linear-domain and alpha invariants |
| 0.92.0 | Declared artistic blend modes | Formula, clamping, NaN, and interoperability tests |
| 0.93.0 | Optional isolated SIMD backends | Scalar differential, tail/alignment, Miri/sanitizer evidence |
| 0.94.0 | Streaming and tiled processing graph | Scratch bounds, fusion equivalence, cancellation, and honest random-access disclosure |
| 0.94.1 | Metadata/header-only, region, and frame-range decoding | Early termination, ROI bounds, valid-prefix semantics, frame selection, budgets, and format support matrix |
| 0.94.2 | Reduced-resolution and progressive-preview decoding | JPEG reduced IDCT, TIFF strip/tile selection, progressive events, scale-during-decode policy, and numeric evidence |
| 0.94.3 | Processing and selective-decoding security audit | Fusion equivalence, scratch/peak limits, cancellation, DoS benchmarks, differential results, and API freeze |
| 0.95.0 | Runtime-neutral async source/sink adapters | Backpressure, cancellation, partial-I/O tests |
| 0.95.1 | WASM/browser streaming adapters | wasm32-unknown-unknown, JS-size, memory-growth tests |
| 0.96.0 | Caller-provided parallel scheduling interface | Determinism, budget partition, cancellation |
| 0.96.1 | Optional Rayon/service adapter | No core dependency or automatic global pool |
| 0.97.0 | GPU-compatible descriptors and upload-layout hooks | Stable layout contract; no device ownership in core |
| 0.97.1 | Optional backend adapters | CPU/GPU differential results and synchronization policy |
| 0.98.0 | mynd-cli inspect and validate | Escaped metadata, bounded defaults, exit-code contract |
| 0.98.1 | CLI decode/encode/convert/frame operations | Transactional files and color-policy disclosure |
| 0.98.2 | CLI batch/service profiles | Aggregate budgets, cancellation, hostile filename tests |
| 0.99.0 | cargo-fuzz suites for every parser, entropy engine, metadata path, and dispatcher | Coverage report and minimized persistent corpus |
| 0.99.1 | Long-running fuzz and every-byte/bit truncation campaigns | No stalls, panics, or inconsistent terminal states |
| 0.99.2 | Kani math/view/bit/geometry proofs | Published assumptions and unwind bounds |
| 0.99.3 | Kani Deflate/LZW/Huffman/JPEG/WebP/TIFF state proofs | Output, dictionary, table, and progress invariants |
| 0.99.4 | Miri, sanitizers, 32-bit, WASM, feature-combination, and stack audit | All supported configurations pass |
| 0.99.5 | Official conformance, differential, color, performance, and DoS freeze | Every claim linked to evidence; no unexplained disagreement |
| 0.99.6 | Reproducible SBOM/package/provenance, external pentest, API freeze | No critical/high finding; clean retest |

## Phase: Foundations

Establish claims, explicit samples and numeric tiers, scalar color/ICC, typed scratch, three-part budgets, transactional I/O, and selective-decoder contracts before a real codec.

### v0.1.0 - Existing workspace, licenses, feature boundaries, release policy

Status: Implementation complete; awaiting pentest.

Context:

This is the exclusive foundations handoff for
existing workspace, licenses, feature boundaries, release policy. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete existing workspace, licenses, feature boundaries, release policy with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Existing workspace, licenses, feature boundaries, release policy.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Preserve the existing workspace, facade/core skeletons, dual licensing, empty defaults, no_std boundary, policies, and release tooling.
- Keep image-model, parser, codec, and processing claims unavailable; the repository still has no implemented image behavior.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Current checks plus completed exact-commit pentest.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.1.0 implementation stop reached. Run pentest for this exact commit.`

### v0.2.0 - Unified scope, claim taxonomy, standards/errata ledger, corpus provenance schema

Status: Planned.

Context:

This is the exclusive foundations handoff for
unified scope, claim taxonomy, standards/errata ledger, corpus provenance schema. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete unified scope, claim taxonomy, standards/errata ledger, corpus provenance schema with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Unified scope, claim taxonomy, standards/errata ledger, corpus provenance schema.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Reconcile README.md, FORMAT_SUPPORT.md, docs/IMPLEMENTATION_PLAN.md, docs/POST_1_0_CODEC_PLAN.md, SPEC_SOURCES.md, the crate graph, and release automation before implementation starts.
- Create machine-readable requirement mappings and licensed corpus provenance records.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: No contradictions across README, support matrix, and normative plans.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.2.0 implementation stop reached. Run pentest for this exact commit.`

### v0.3.0 - Checked conversion/add/multiply/align/range primitives

Status: Planned.

Context:

This is the exclusive foundations handoff for
checked conversion/add/multiply/align/range primitives. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete checked conversion/add/multiply/align/range primitives with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Checked conversion/add/multiply/align/range primitives.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Exhaustive extrema tests and Kani arithmetic proofs.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.3.0 implementation stop reached. Run pentest for this exact commit.`

### v0.4.0 - Validated dimensions, rectangles, strides, planes, and output lengths

Status: Planned.

Context:

This is the exclusive foundations handoff for
validated dimensions, rectangles, strides, planes, and output lengths. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete validated dimensions, rectangles, strides, planes, and output lengths with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Validated dimensions, rectangles, strides, planes, and output lengths.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Zero/min/max, last-row, alignment, and 32-bit usize proofs.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.4.0 implementation stop reached. Run pentest for this exact commit.`

### v0.5.0 - Explicit pixel layout and sample-storage domains

Status: Planned.

Context:

This is the exclusive foundations handoff for
explicit pixel layout and sample-storage domains. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete explicit pixel layout and sample-storage domains with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Explicit pixel layout and sample-storage domains.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Enumerate U1/U2/U4 packed, U8/U16, required signed working integers, indexed/palette layout, F32, interleaved/planar storage, plane offsets/strides, chroma siting, and alpha.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Invalid layout/sample/plane/chroma/alpha combinations are unrepresentable.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.5.0 implementation stop reached. Run pentest for this exact commit.`

### v0.5.1 - Numeric determinism and floating-sample contract

Status: Planned.

Context:

This is the exclusive foundations handoff for
numeric determinism and floating-sample contract. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete numeric determinism and floating-sample contract with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Numeric determinism and floating-sample contract.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Publish bit-exact, normative-tolerance, reference-tolerance, and backend/config-deterministic behavior with rounding, saturation, FMA, NaN, infinity, signed-zero, and subnormal policy.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Bit-exact/tolerance/backend tiers plus rounding, saturation, FMA, NaN, infinity, zero, and subnormal tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.5.1 implementation stop reached. Run pentest for this exact commit.`

### v0.5.2 - Shared color and blending specification ledger

Status: Planned.

Context:

This is the exclusive foundations handoff for
shared color and blending specification ledger. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete shared color and blending specification ledger with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Shared color and blending specification ledger.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Pin exact ICC, sRGB, BT.601/709/2020, H.273, HDR, CIE, Porter-Duff, and artistic-blend sources and supported subsets.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Pinned ICC, sRGB, BT.601/709/2020, H.273, HDR, CIE, Porter-Duff, and blend sources with claim scope.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.5.2 implementation stop reached. Run pentest for this exact commit.`

### v0.5.3 - Scalar transfer, matrix/range, and alpha foundation

Status: Planned.

Context:

This is the exclusive foundations handoff for
scalar transfer, matrix/range, and alpha foundation. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete scalar transfer, matrix/range, and alpha foundation with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Scalar transfer, matrix/range, and alpha foundation.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Provide shared scalar transfer, matrix/range, rounding, and alpha APIs before any complex codec claims rendered output.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Integer/scalar reference vectors, premultiplication, range conversion, rounding, and native-sample preservation.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.5.3 implementation stop reached. Run pentest for this exact commit.`

### v0.6.0 - Immutable/mutable image and plane views

Status: Planned.

Context:

This is the exclusive foundations handoff for
immutable/mutable image and plane views. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete immutable/mutable image and plane views with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Immutable/mutable image and plane views.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Short-buffer, row-boundary, alias-policy, and Miri tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.6.0 implementation stop reached. Run pentest for this exact commit.`

### v0.7.0 - Frames, timing, canvas, disposal, blend, and frame rectangles

Status: Planned.

Context:

This is the exclusive foundations handoff for
frames, timing, canvas, disposal, blend, and frame rectangles. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete frames, timing, canvas, disposal, blend, and frame rectangles with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Frames, timing, canvas, disposal, blend, and frame rectangles.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Off-canvas and cumulative-duration tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.7.0 implementation stop reached. Run pentest for this exact commit.`

### v0.8.0 - Allocation-free structured errors, warnings, reports, and offsets

Status: Planned.

Context:

This is the exclusive foundations handoff for
allocation-free structured errors, warnings, reports, and offsets. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete allocation-free structured errors, warnings, reports, and offsets with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Allocation-free structured errors, warnings, reports, and offsets.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Bounded formatting and terminal/log-injection tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.8.0 implementation stop reached. Run pentest for this exact commit.`

### v0.9.0 - DecodeLimits, EncodeLimits, monotonic work/memory ledger

Status: Planned.

Context:

This is the exclusive foundations handoff for
decodelimits, encodelimits, monotonic work/memory ledger. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete decodelimits, encodelimits, monotonic work/memory ledger with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: DecodeLimits, EncodeLimits, monotonic work/memory ledger.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Separate monotonic cumulative counters, releasable live reservations, and peak gauges. Include warnings, diagnostics, state transitions, table counts/bytes, progressive refinements, and decompressed metadata.
- Nested and parallel children use parent-backed grants that cannot mint capacity or refund cumulative work/output.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Budget-sharing and bypass property tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.9.0 implementation stop reached. Run pentest for this exact commit.`

### v0.10.0 - Caller-owned scratch planner, arenas, buffer pools, and leases

Status: Planned.

Context:

This is the exclusive foundations handoff for
caller-owned scratch planner, arenas, buffer pools, and leases. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete caller-owned scratch planner, arenas, buffer pools, and leases with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Caller-owned scratch planner, arenas, buffer pools, and leases.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Use format-neutral ScratchRequirements plus codec-specific typed workspaces. Safe Rust does not reinterpret arbitrary byte scratch or expose uninitialized Vec capacity.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Peak-memory accounting and failed-reservation tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.10.0 implementation stop reached. Run pentest for this exact commit.`

### v0.11.0 - Slice reader/writer, exact reads, fixed output, checkpoints

Status: Planned.

Context:

This is the exclusive foundations handoff for
slice reader/writer, exact reads, fixed output, checkpoints. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete slice reader/writer, exact reads, fixed output, checkpoints with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Slice reader/writer, exact reads, fixed output, checkpoints.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Every-byte truncation and rollback tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.11.0 implementation stop reached. Run pentest for this exact commit.`

### v0.12.0 - Endian I/O, subranges, counting, seek/read-at

Status: Planned.

Context:

This is the exclusive foundations handoff for
endian i/o, subranges, counting, seek/read-at. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete endian i/o, subranges, counting, seek/read-at with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Endian I/O, subranges, counting, seek/read-at.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Nested-bound escape, offset, and seek-cycle tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.12.0 implementation stop reached. Run pentest for this exact commit.`

### v0.13.0 - MSB/LSB bit readers and writers

Status: Planned.

Context:

This is the exclusive foundations handoff for
msb/lsb bit readers and writers. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete msb/lsb bit readers and writers with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: MSB/LSB bit readers and writers.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Every-bit truncation, width, refill, and shift proofs.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.13.0 implementation stop reached. Run pentest for this exact commit.`

### v0.14.0 - Incremental decoder/encoder progress contracts

Status: Planned.

Context:

This is the exclusive foundations handoff for
incremental decoder/encoder progress contracts. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete incremental decoder/encoder progress contracts with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Incremental decoder/encoder progress contracts.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Reserve plan hooks for header/metadata-only termination, ROI, reduced resolution, strip/tile and frame-range selection, progressive previews, and scale-during-decode.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Chunk-boundary equivalence and zero-progress rejection.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.14.0 implementation stop reached. Run pentest for this exact commit.`

### v0.15.0 - Metadata envelopes and bounded Exif/ICC/XMP header transport

Status: Planned.

Context:

This is the exclusive foundations handoff for
metadata envelopes and bounded exif/icc/xmp header transport. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete metadata envelopes and bounded exif/icc/xmp header transport with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Metadata envelopes and bounded Exif/ICC/XMP header transport.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Transport remains bounded and uninterpreted here; structured Exif/XMP inspection has dedicated later releases.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Offset/count validation without full metadata interpretation.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.15.0 implementation stop reached. Run pentest for this exact commit.`

### v0.15.1 - Bounded ICC v2/v4 structural parser

Status: Planned.

Context:

This is the exclusive foundations handoff for
bounded icc v2/v4 structural parser. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bounded icc v2/v4 structural parser with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Bounded ICC v2/v4 structural parser.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Keep ICC in its own threat boundary and preserve unsupported valid profiles without execution.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tag counts/sizes/offsets/overlap, curves, LUT dimensions, recursion, opaque preservation, and fuzzing.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.15.1 implementation stop reached. Run pentest for this exact commit.`

### v0.15.2 - ICC matrix/TRC and chromatic-adaptation engine

Status: Planned.

Context:

This is the exclusive foundations handoff for
icc matrix/trc and chromatic-adaptation engine. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete icc matrix/trc and chromatic-adaptation engine with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: ICC matrix/TRC and chromatic-adaptation engine.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Parametric curves, PCS conversion, adaptation, rendering intent, deterministic scalar vectors, and limits.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.15.2 implementation stop reached. Run pentest for this exact commit.`

### v0.15.3 - ICC LUT, mAB/mBA, PCS Lab/XYZ, and v4 execution

Status: Planned.

Context:

This is the exclusive foundations handoff for
icc lut, mab/mba, pcs lab/xyz, and v4 execution. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete icc lut, mab/mba, pcs lab/xyz, and v4 execution with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: ICC LUT, mAB/mBA, PCS Lab/XYZ, and v4 execution.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Element counts, interpolation, intent, recursion, numeric tolerances, and independent profile tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.15.3 implementation stop reached. Run pentest for this exact commit.`

### v0.16.0 - Format IDs, media types, bounded probing, static registry

Status: Planned.

Context:

This is the exclusive foundations handoff for
format ids, media types, bounded probing, static registry. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete format ids, media types, bounded probing, static registry with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Format IDs, media types, bounded probing, static registry.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Collision, ambiguity, polyglot, and disabled-feature tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.16.0 implementation stop reached. Run pentest for this exact commit.`

### v0.17.0 - Fallible owned storage and std::io adapters

Status: Planned.

Context:

This is the exclusive foundations handoff for
fallible owned storage and std::io adapters. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete fallible owned storage and std::io adapters with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Fallible owned storage and std::io adapters.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Allocation-failure, interrupted-I/O, and feature-matrix tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.17.0 implementation stop reached. Run pentest for this exact commit.`

### v0.18.0 - Foundation API freeze, fuzz harness primitives, Kani core suite

Status: Planned.

Context:

This is the exclusive foundations handoff for
foundation api freeze, fuzz harness primitives, kani core suite. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete foundation api freeze, fuzz harness primitives, kani core suite with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Foundation API freeze, fuzz harness primitives, Kani core suite.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Freeze selective-decoding hooks before codecs while every optional path remains unsupported until its 0.94.x evidence passes.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: External design review and no-default/32-bit/WASM build matrix.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.18.0 implementation stop reached. Run pentest for this exact commit.`


## Phase: Simple and lossless codecs

Prove the architecture on bounded formats while splitting encoders from audits and making every dialect and trailing-data policy explicit.

### v0.19.0 - Common codec crate template and decode-plan contract

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
common codec crate template and decode-plan contract. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete common codec crate template and decode-plan contract with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Common codec crate template and decode-plan contract.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: A dummy codec proves limit, scratch, progress, and rollback invariants.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.19.0 implementation stop reached. Run pentest for this exact commit.`

### v0.20.0 - BMP probe, file header, OS/2 and Windows DIB dispatch

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp probe, file header, os/2 and windows dib dispatch. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp probe, file header, os/2 and windows dib dispatch with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: BMP probe, file header, OS/2 and Windows DIB dispatch.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Record admitted OS/2 families and explicit RLE24, Huffman 1D, BI_JPEG, and BI_PNG decisions.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Header-size confusion and offset corpus.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.20.0 implementation stop reached. Run pentest for this exact commit.`

### v0.21.0 - BMP BI_RGB depths, palettes, padding, row orientation

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp bi_rgb depths, palettes, padding, row orientation. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp bi_rgb depths, palettes, padding, row orientation with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: BMP BI_RGB depths, palettes, padding, row orientation.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: 1/4/8/16/24/32-bit golden and truncation tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.21.0 implementation stop reached. Run pentest for this exact commit.`

### v0.22.0 - BMP bitfields, alpha masks, top-down rules

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp bitfields, alpha masks, top-down rules. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp bitfields, alpha masks, top-down rules with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: BMP bitfields, alpha masks, top-down rules.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Mask overlap/gap/full-width and signed-height tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.22.0 implementation stop reached. Run pentest for this exact commit.`

### v0.23.0 - BMP RLE4/RLE8

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp rle4/rle8. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp rle4/rle8 with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: BMP RLE4/RLE8.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Escape, delta, padding, exact-output, and no-progress fuzzing.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.23.0 implementation stop reached. Run pentest for this exact commit.`

### v0.24.0 - BMP V4/V5 color declarations and embedded-profile transport

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp v4/v5 color declarations and embedded-profile transport. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp v4/v5 color declarations and embedded-profile transport with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: BMP V4/V5 color declarations and embedded-profile transport.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Linked profile paths remain inert and never trigger filesystem or network access.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Profile-range, overlap, and linked-profile no-I/O tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.24.0 implementation stop reached. Run pentest for this exact commit.`

### v0.25.0 - BMP deterministic uncompressed encoders

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp deterministic uncompressed encoders. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp deterministic uncompressed encoders with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: BMP deterministic uncompressed encoders.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Dialect-specific golden files, exact headers/padding, determinism, and round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.25.0 implementation stop reached. Run pentest for this exact commit.`

### v0.25.1 - BMP deterministic RLE4/RLE8 encoders

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp deterministic rle4/rle8 encoders. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp deterministic rle4/rle8 encoders with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: BMP deterministic RLE4/RLE8 encoders.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Escape/padding/delta policy, deterministic packets, bounded work, and decode/encode round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.25.1 implementation stop reached. Run pentest for this exact commit.`

### v0.25.2 - Complete declared BMP dialect audit

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
complete declared bmp dialect audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete complete declared bmp dialect audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Complete declared BMP dialect audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Audit every admitted OS/2/Windows dialect and embedded-payload decision separately.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: BI_RGB/bitfield/RLE encoders, OS/2 decisions, embedded BI_JPEG/BI_PNG policy, differential, corpus, and fuzz review.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.25.2 implementation stop reached. Run pentest for this exact commit.`

### v0.26.0 - QOI structural parse and bounded decoder

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
qoi structural parse and bounded decoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete qoi structural parse and bounded decoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: QOI structural parse and bounded decoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Pixel count, wraparound, end-marker, trailing-data tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.26.0 implementation stop reached. Run pentest for this exact commit.`

### v0.27.0 - QOI deterministic encoder

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
qoi deterministic encoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete qoi deterministic encoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: QOI deterministic encoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Reference-vector and encode/decode conformance.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.27.0 implementation stop reached. Run pentest for this exact commit.`

### v0.28.0 - Bounded Netpbm tokenizer

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bounded netpbm tokenizer. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bounded netpbm tokenizer with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Bounded Netpbm tokenizer.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Define the exact final-whitespace to raw-raster transition with bounded lookahead.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Comment, whitespace, decimal overflow, token-length fuzzing.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.28.0 implementation stop reached. Run pentest for this exact commit.`

### v0.29.0 - PBM P1/P4 decode/encode

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
pbm p1/p4 decode/encode. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete pbm p1/p4 decode/encode with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PBM P1/P4 decode/encode.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Bit order, row padding, multi-image policy.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.29.0 implementation stop reached. Run pentest for this exact commit.`

### v0.30.0 - PGM P2/P5 decode/encode

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
pgm p2/p5 decode/encode. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete pgm p2/p5 decode/encode with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PGM P2/P5 decode/encode.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: MAXVAL scaling, 8/16-bit, truncation.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.30.0 implementation stop reached. Run pentest for this exact commit.`

### v0.31.0 - PPM P3/P6 decode/encode

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
ppm p3/p6 decode/encode. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete ppm p3/p6 decode/encode with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PPM P3/P6 decode/encode.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Sample scaling, token bombs, binary boundaries.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.31.0 implementation stop reached. Run pentest for this exact commit.`

### v0.32.0 - PAM P7, if the public claim is “Netpbm”

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
pam p7, if the public claim is “netpbm”. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete pam p7, if the public claim is “netpbm” with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PAM P7, if the public claim is “Netpbm”.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Unknown PAM tuple types are preserved or rejected by explicit profile policy.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tuple types, depth, header termination, unknown fields.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.32.0 implementation stop reached. Run pentest for this exact commit.`

### v0.33.0 - Combined PNM/PAM stream and conformance audit

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
combined pnm/pam stream and conformance audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete combined pnm/pam stream and conformance audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Combined PNM/PAM stream and conformance audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Declare concatenated-image iteration and trailing-material behavior.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Concatenated images and official-tool differential tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.33.0 implementation stop reached. Run pentest for this exact commit.`

### v0.34.0 - farbfeld decode and encode

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
farbfeld decode and encode. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete farbfeld decode and encode with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: farbfeld decode and encode.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Exact-size arithmetic, RGBA16-BE, alpha semantics.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.34.0 implementation stop reached. Run pentest for this exact commit.`

### v0.35.0 - Simple-codec security and API freeze

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
simple-codec security and api freeze. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete simple-codec security and api freeze with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Simple-codec security and API freeze.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Cross-codec probe fuzzing, 32-bit memory tests, external delta review.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.35.0 implementation stop reached. Run pentest for this exact commit.`


## Phase: Complex formats

Implement PNG, GIF, JPEG, WebP, and TIFF as separate audit surfaces; palette work precedes GIF encoding and broad encoder families are patch-split.

### v0.36.0 - PNG source map, signature, chunk state machine, CRC, ordering

Status: Planned.

Context:

This is the exclusive complex formats handoff for
png source map, signature, chunk state machine, crc, ordering. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete png source map, signature, chunk state machine, crc, ordering with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PNG source map, signature, chunk state machine, CRC, ordering.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Pin PNG Third Edition, its errata, ISO/IEC 15948:2004, RFC 1950, and RFC 1951.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Unknown-critical and CRC policy tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.36.0 implementation stop reached. Run pentest for this exact commit.`

### v0.37.0 - PNG IHDR and color-type/bit-depth validation

Status: Planned.

Context:

This is the exclusive complex formats handoff for
png ihdr and color-type/bit-depth validation. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete png ihdr and color-type/bit-depth validation with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PNG IHDR and color-type/bit-depth validation.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Full normative combination matrix.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.37.0 implementation stop reached. Run pentest for this exact commit.`

### v0.38.0 - zlib wrapper plus stored/fixed-Huffman Deflate

Status: Planned.

Context:

This is the exclusive complex formats handoff for
zlib wrapper plus stored/fixed-huffman deflate. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete zlib wrapper plus stored/fixed-huffman deflate with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: zlib wrapper plus stored/fixed-Huffman Deflate.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: RFC vectors, Adler-32, bit truncation.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.38.0 implementation stop reached. Run pentest for this exact commit.`

### v0.39.0 - Dynamic Huffman and complete bounded Deflate window

Status: Planned.

Context:

This is the exclusive complex formats handoff for
dynamic huffman and complete bounded deflate window. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete dynamic huffman and complete bounded deflate window with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Dynamic Huffman and complete bounded Deflate window.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tree proofs, distance/overlap fuzzing, output bombs.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.39.0 implementation stop reached. Run pentest for this exact commit.`

### v0.40.0 - PNG filters and noninterlaced core color decoding

Status: Planned.

Context:

This is the exclusive complex formats handoff for
png filters and noninterlaced core color decoding. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete png filters and noninterlaced core color decoding with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PNG filters and noninterlaced core color decoding.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Per-filter/sample golden vectors.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.40.0 implementation stop reached. Run pentest for this exact commit.`

### v0.41.0 - Packed 1/2/4-bit and 16-bit PNG samples

Status: Planned.

Context:

This is the exclusive complex formats handoff for
packed 1/2/4-bit and 16-bit png samples. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete packed 1/2/4-bit and 16-bit png samples with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Packed 1/2/4-bit and 16-bit PNG samples.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scaling, endian, tail-bit tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.41.0 implementation stop reached. Run pentest for this exact commit.`

### v0.42.0 - Adam7 decode and progressive row events

Status: Planned.

Context:

This is the exclusive complex formats handoff for
adam7 decode and progressive row events. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete adam7 decode and progressive row events with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Adam7 decode and progressive row events.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Pass geometry proofs and tiny-image corpus.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.42.0 implementation stop reached. Run pentest for this exact commit.`

### v0.43.0 - PNG palette, tRNS, bKGD, hIST, sBIT, sPLT

Status: Planned.

Context:

This is the exclusive complex formats handoff for
png palette, trns, bkgd, hist, sbit, splt. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete png palette, trns, bkgd, hist, sbit, splt with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PNG palette, tRNS, bKGD, hIST, sBIT, sPLT.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Palette/index/transparency conformance.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.43.0 implementation stop reached. Run pentest for this exact commit.`

### v0.44.0 - PNG cHRM/gAMA/sRGB/iCCP/cICP/mDCV/cLLI and precedence

Status: Planned.

Context:

This is the exclusive complex formats handoff for
png chrm/gama/srgb/iccp/cicp/mdcv/clli and precedence. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete png chrm/gama/srgb/iccp/cicp/mdcv/clli and precedence with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PNG cHRM/gAMA/sRGB/iCCP/cICP/mDCV/cLLI and precedence.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Perform bounded ICC transport and PNG precedence through shared color APIs; do not add a format-local ICC executor.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: ICC bombs, precedence matrix, HDR/WCG vectors.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.44.0 implementation stop reached. Run pentest for this exact commit.`

### v0.45.0 - PNG text, compressed text, eXIf, pHYs, tIME, unknown/private chunks

Status: Planned.

Context:

This is the exclusive complex formats handoff for
png text, compressed text, exif, phys, time, unknown/private chunks. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete png text, compressed text, exif, phys, time, unknown/private chunks with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PNG text, compressed text, eXIf, pHYs, tIME, unknown/private chunks.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Metadata decompression and safe-to-copy tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.45.0 implementation stop reached. Run pentest for this exact commit.`

### v0.46.0 - APNG decoding

Status: Planned.

Context:

This is the exclusive complex formats handoff for
apng decoding. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete apng decoding with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: APNG decoding.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: APNG state, frame, timing, disposal, blend, streaming, and animation-bomb tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.46.0 implementation stop reached. Run pentest for this exact commit.`

### v0.46.1 - PNG deterministic encoding

Status: Planned.

Context:

This is the exclusive complex formats handoff for
png deterministic encoding. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete png deterministic encoding with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: PNG deterministic encoding.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: PNG Third Edition emission, round-trip, filter, Deflate, metadata, and determinism tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.46.1 implementation stop reached. Run pentest for this exact commit.`

### v0.46.2 - APNG deterministic encoding

Status: Planned.

Context:

This is the exclusive complex formats handoff for
apng deterministic encoding. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete apng deterministic encoding with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: APNG deterministic encoding.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Frame-sequence, rectangle, timing, disposal/blend, and decode/encode round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.46.2 implementation stop reached. Run pentest for this exact commit.`

### v0.46.3 - Complete PNG/APNG conformance and security audit

Status: Planned.

Context:

This is the exclusive complex formats handoff for
complete png/apng conformance and security audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete complete png/apng conformance and security audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Complete PNG/APNG conformance and security audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- A failed PNG audit blocks GIF rather than moving unresolved scope forward.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Third Edition conformance, differential, streaming, fuzz review.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.46.3 implementation stop reached. Run pentest for this exact commit.`

### v0.47.0 - GIF87a/89a structure, palettes, sub-blocks, descriptors

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif87a/89a structure, palettes, sub-blocks, descriptors. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif87a/89a structure, palettes, sub-blocks, descriptors with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: GIF87a/89a structure, palettes, sub-blocks, descriptors.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Decide missing EOI/trailer, extra pixels, zero delay, and reserved disposal behavior.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Block termination and palette bounds.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.47.0 implementation stop reached. Run pentest for this exact commit.`

### v0.48.0 - GIF LZW

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif lzw. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif lzw with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: GIF LZW.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- GIF LZW remains format-local and cannot silently share TIFF LZW policy.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Dictionary/code-width proofs and fuzzing.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.48.0 implementation stop reached. Run pentest for this exact commit.`

### v0.49.0 - GIF single-frame decode and deinterlace

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif single-frame decode and deinterlace. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif single-frame decode and deinterlace with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: GIF single-frame decode and deinterlace.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Exact pixels and four-pass geometry tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.49.0 implementation stop reached. Run pentest for this exact commit.`

### v0.50.0 - GIF GCE, transparency, frame composition, all disposal modes

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif gce, transparency, frame composition, all disposal modes. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif gce, transparency, frame composition, all disposal modes with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: GIF GCE, transparency, frame composition, all disposal modes.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Snapshot caps and animation bomb corpus.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.50.0 implementation stop reached. Run pentest for this exact commit.`

### v0.51.0 - GIF named extensions and compatibility policy

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif named extensions and compatibility policy. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif named extensions and compatibility policy with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: GIF named extensions and compatibility policy.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Name Comment, Plain Text, Application, and unknown extensions separately; Netscape looping remains de facto behavior.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Comment/Plain Text/Application/unknown blocks plus loop, EOI, trailer, delay, and disposal policy.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.51.0 implementation stop reached. Run pentest for this exact commit.`

### v0.51.1 - GIF raw-frame and composited-frame APIs

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif raw-frame and composited-frame apis. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif raw-frame and composited-frame apis with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: GIF raw-frame and composited-frame APIs.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Coordinate, palette, disposal sequencing, valid-prefix, snapshot, and frame-range tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.51.1 implementation stop reached. Run pentest for this exact commit.`

### v0.51.2 - Exact palettes and bounded histogram

Status: Planned.

Context:

This is the exclusive complex formats handoff for
exact palettes and bounded histogram. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete exact palettes and bounded histogram with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Exact palettes and bounded histogram.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Support exact caller palettes before any lossy palette search.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Unique-color limits, palette-entry layout, deterministic ordering, overflow, and caller-palette tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.51.2 implementation stop reached. Run pentest for this exact commit.`

### v0.51.3 - Deterministic palette generation and remapping

Status: Planned.

Context:

This is the exclusive complex formats handoff for
deterministic palette generation and remapping. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete deterministic palette generation and remapping with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Deterministic palette generation and remapping.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Split exact palette, histogram, declared median-cut, remap, ordered dither, and optional error diffusion into bounded stages.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Declared median-cut policy, palette remap, ordered/error-diffusion modes, budgets, and golden vectors.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.51.3 implementation stop reached. Run pentest for this exact commit.`

### v0.51.4 - Single-frame GIF encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
single-frame gif encoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete single-frame gif encoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Single-frame GIF encoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Caller/generated palette paths, table sizing, transparency, sub-blocks, LZW, and deterministic round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.51.4 implementation stop reached. Run pentest for this exact commit.`

### v0.51.5 - Animated GIF encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
animated gif encoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete animated gif encoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Animated GIF encoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Canvas/frame totals, timing, loop extension, disposal, frame ranges, and animation round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.51.5 implementation stop reached. Run pentest for this exact commit.`

### v0.51.6 - Complete GIF conformance and security audit

Status: Planned.

Context:

This is the exclusive complex formats handoff for
complete gif conformance and security audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete complete gif conformance and security audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Complete GIF conformance and security audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Normative/de-facto separation, missing terminator policy, differential corpus, LZW fuzzing, and animation bombs.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.51.6 implementation stop reached. Run pentest for this exact commit.`

### v0.52.0 - JPEG source map, marker/segment parser, frame/scan/table declarations

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jpeg source map, marker/segment parser, frame/scan/table declarations. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jpeg source map, marker/segment parser, frame/scan/table declarations with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG source map, marker/segment parser, frame/scan/table declarations.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Include DNL, DAC, restart reset, abbreviated tables, multiscan sequential streams, component limits, and table redefinition.
- Keep native samples/coefficients separate from JFIF or Adobe color interpretation.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Marker mutation and segment-size fuzzing.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.52.0 implementation stop reached. Run pentest for this exact commit.`

### v0.53.0 - JPEG Huffman entropy, stuffing, restart, bounded coefficients

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jpeg huffman entropy, stuffing, restart, bounded coefficients. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jpeg huffman entropy, stuffing, restart, bounded coefficients with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG Huffman entropy, stuffing, restart, bounded coefficients.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- JPEG Huffman and arithmetic engines remain format-local.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Canonical table proofs and MCU accounting.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.53.0 implementation stop reached. Run pentest for this exact commit.`

### v0.54.0 - Baseline scalar IDCT, sampling, upsampling, grayscale/YCbCr decode

Status: Planned.

Context:

This is the exclusive complex formats handoff for
baseline scalar idct, sampling, upsampling, grayscale/ycbcr decode. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete baseline scalar idct, sampling, upsampling, grayscale/ycbcr decode with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Baseline scalar IDCT, sampling, upsampling, grayscale/YCbCr decode.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tolerance vectors and restart corpus.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.54.0 implementation stop reached. Run pentest for this exact commit.`

### v0.55.0 - Extended sequential and 12-bit DCT processes

Status: Planned.

Context:

This is the exclusive complex formats handoff for
extended sequential and 12-bit dct processes. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete extended sequential and 12-bit dct processes with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Extended sequential and 12-bit DCT processes.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Precision and coefficient-range evidence.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.55.0 implementation stop reached. Run pentest for this exact commit.`

### v0.56.0 - Progressive DC/AC and successive approximation

Status: Planned.

Context:

This is the exclusive complex formats handoff for
progressive dc/ac and successive approximation. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete progressive dc/ac and successive approximation with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Progressive DC/AC and successive approximation.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scan-order/state-machine fuzzing.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.56.0 implementation stop reached. Run pentest for this exact commit.`

### v0.57.0 - Lossless predictive JPEG process

Status: Planned.

Context:

This is the exclusive complex formats handoff for
lossless predictive jpeg process. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete lossless predictive jpeg process with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Lossless predictive JPEG process.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Predictor, point transform, precision tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.57.0 implementation stop reached. Run pentest for this exact commit.`

### v0.58.0 - JPEG arithmetic coding

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jpeg arithmetic coding. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jpeg arithmetic coding with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG arithmetic coding.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Conditioning-table and arithmetic-state proofs.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.58.0 implementation stop reached. Run pentest for this exact commit.`

### v0.59.0 - Differential and hierarchical processes

Status: Planned.

Context:

This is the exclusive complex formats handoff for
differential and hierarchical processes. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete differential and hierarchical processes with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Differential and hierarchical processes.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Frame dependency and reconstruction bounds.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.59.0 implementation stop reached. Run pentest for this exact commit.`

### v0.60.0 - JFIF, Exif, ICC APP2 assembly, Adobe RGB/CMYK/YCCK

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jfif, exif, icc app2 assembly, adobe rgb/cmyk/ycck. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jfif, exif, icc app2 assembly, adobe rgb/cmyk/ycck with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: JFIF, Exif, ICC APP2 assembly, Adobe RGB/CMYK/YCCK.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Interpret color only through the shared scalar engine and preserve native samples/coefficients independently.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Segment reassembly, precedence, and color vectors.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.60.0 implementation stop reached. Run pentest for this exact commit.`

### v0.61.0 - Baseline JPEG encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
baseline jpeg encoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete baseline jpeg encoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Baseline JPEG encoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Deterministic valid baseline emission, quality controls, coefficient limits, and round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.61.0 implementation stop reached. Run pentest for this exact commit.`

### v0.61.1 - Progressive JPEG encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
progressive jpeg encoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete progressive jpeg encoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Progressive JPEG encoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scan scripts, successive approximation, deterministic tables, restart policy, and round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.61.1 implementation stop reached. Run pentest for this exact commit.`

### v0.61.2 - Extended-sequential and lossless JPEG encoders

Status: Planned.

Context:

This is the exclusive complex formats handoff for
extended-sequential and lossless jpeg encoders. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete extended-sequential and lossless jpeg encoders with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Extended-sequential and lossless JPEG encoders.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Precision/process-specific valid emission, predictor/point-transform policy, and differential tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.61.2 implementation stop reached. Run pentest for this exact commit.`

### v0.61.3 - Arithmetic, differential, and hierarchical JPEG encoder admission

Status: Planned.

Context:

This is the exclusive complex formats handoff for
arithmetic, differential, and hierarchical jpeg encoder admission. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete arithmetic, differential, and hierarchical jpeg encoder admission with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Arithmetic, differential, and hierarchical JPEG encoder admission.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Decoder completeness never implies all encoder processes; unimplemented encoder modes remain explicit.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Each process is either independently evidenced or explicitly unclaimed with no decoder-claim ambiguity.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.61.3 implementation stop reached. Run pentest for this exact commit.`

### v0.62.0 - Complete declared T.81 conformance and security audit

Status: Planned.

Context:

This is the exclusive complex formats handoff for
complete declared t.81 conformance and security audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete complete declared t.81 conformance and security audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Complete declared T.81 conformance and security audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- A failed JPEG audit blocks WebP rather than moving unresolved scope forward.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Reference software, official material, long fuzz campaign.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.62.0 implementation stop reached. Run pentest for this exact commit.`

### v0.63.0 - WebP RIFF/VP8X container, chunk order, metadata

Status: Planned.

Context:

This is the exclusive complex formats handoff for
webp riff/vp8x container, chunk order, metadata. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete webp riff/vp8x container, chunk order, metadata with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: WebP RIFF/VP8X container, chunk order, metadata.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Separate still and animated RIFF behavior; name ICCP, EXIF, XMP, unknown chunks, and ordering.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: RFC 9649 conformance and size/padding fuzzing.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.63.0 implementation stop reached. Run pentest for this exact commit.`

### v0.64.0 - VP8 Boolean decoder, partitions, headers, probabilities

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8 boolean decoder, partitions, headers, probabilities. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8 boolean decoder, partitions, headers, probabilities with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: VP8 Boolean decoder, partitions, headers, probabilities.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Partition limits and arithmetic-state fuzzing.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.64.0 implementation stop reached. Run pentest for this exact commit.`

### v0.65.0 - VP8 prediction, coefficients, inverse transforms, loop filters

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8 prediction, coefficients, inverse transforms, loop filters. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8 prediction, coefficients, inverse transforms, loop filters with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: VP8 prediction, coefficients, inverse transforms, loop filters.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scalar reference differential tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.65.0 implementation stop reached. Run pentest for this exact commit.`

### v0.66.0 - VP8 alpha and Y′CbCr-to-RGB pipeline

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8 alpha and y′cbcr-to-rgb pipeline. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8 alpha and y′cbcr-to-rgb pipeline with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: VP8 alpha and Y′CbCr-to-RGB pipeline.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Name every ALPH filtering/compression mode and use shared YCbCr/color policy.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: ALPH preprocessing and color tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.66.0 implementation stop reached. Run pentest for this exact commit.`

### v0.67.0 - VP8L prefix coding, LZ77, color cache

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8l prefix coding, lz77, color cache. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8l prefix coding, lz77, color cache with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: VP8L prefix coding, LZ77, color cache.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Prefix/distance/cache proofs and bombs.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.67.0 implementation stop reached. Run pentest for this exact commit.`

### v0.68.0 - VP8L transforms and complete lossless reconstruction

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8l transforms and complete lossless reconstruction. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8l transforms and complete lossless reconstruction with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: VP8L transforms and complete lossless reconstruction.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Transform recursion and meta-image limits.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.68.0 implementation stop reached. Run pentest for this exact commit.`

### v0.69.0 - WebP animation decoding

Status: Planned.

Context:

This is the exclusive complex formats handoff for
webp animation decoding. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete webp animation decoding with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: WebP animation decoding.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Validate ANMF rectangles, duration, blend/dispose, and aggregate frames before payload decode.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: ANMF rectangles, duration, blend/dispose, frame limits, and animation fuzzing.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.69.0 implementation stop reached. Run pentest for this exact commit.`

### v0.69.1 - VP8L deterministic encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8l deterministic encoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8l deterministic encoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: VP8L deterministic encoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Expose bounded quality/effort controls and a declared deterministic mode.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Prefix/LZ/cache/transform validity, quality-effort controls, bounded search, determinism, and round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.69.1 implementation stop reached. Run pentest for this exact commit.`

### v0.69.2 - VP8 deterministic encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8 deterministic encoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8 deterministic encoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: VP8 deterministic encoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Expose bounded quality/effort controls and a declared backend/configuration determinism tier.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Prediction/partition/token validity, quality-effort controls, bounded heuristics, backend determinism, and differential tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.69.2 implementation stop reached. Run pentest for this exact commit.`

### v0.69.3 - Animated WebP encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
animated webp encoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete animated webp encoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Animated WebP encoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: ANMF ordering/rectangles, mixed frame modes, blend/dispose, metadata, and round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.69.3 implementation stop reached. Run pentest for this exact commit.`

### v0.69.4 - Complete WebP conformance and security audit

Status: Planned.

Context:

This is the exclusive complex formats handoff for
complete webp conformance and security audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete complete webp conformance and security audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Complete WebP conformance and security audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- A failed WebP audit blocks TIFF rather than moving unresolved scope forward.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: RFC/VP8/VP8L mappings, still/animated split, ALPH/metadata, encoder modes, long fuzzing, and external review.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.69.4 implementation stop reached. Run pentest for this exact commit.`

### v0.70.0 - TIFF byte order, header, typed values, bounded IFD graph

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff byte order, header, typed values, bounded ifd graph. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff byte order, header, typed values, bounded ifd graph with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF byte order, header, typed values, bounded IFD graph.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Include FillOrder, Orientation, ExtraSamples, SampleFormat, signed/IEEE-float samples, sparse strips, and overlapping range policy.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Offset cycles, overlaps, entry/count multiplication.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.70.0 implementation stop reached. Run pentest for this exact commit.`

### v0.71.0 - Baseline strips: bilevel, Gray, palette, RGB, uncompressed/PackBits

Status: Planned.

Context:

This is the exclusive complex formats handoff for
baseline strips: bilevel, gray, palette, rgb, uncompressed/packbits. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete baseline strips: bilevel, gray, palette, rgb, uncompressed/packbits with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Baseline strips: bilevel, Gray, palette, RGB, uncompressed/PackBits.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Strip-size and row-layout tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.71.0 implementation stop reached. Run pentest for this exact commit.`

### v0.72.0 - TIFF LZW, Deflate, and horizontal predictors

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff lzw, deflate, and horizontal predictors. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff lzw, deflate, and horizontal predictors with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF LZW, Deflate, and horizontal predictors.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Declare supported predictors; floating Predictor 3 is outside TIFF 6 unless separately admitted.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Dialect policy and decompression bombs.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.72.0 implementation stop reached. Run pentest for this exact commit.`

### v0.73.0 - TIFF CCITT RLE, Group 3, and Group 4

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff ccitt rle, group 3, and group 4. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff ccitt rle, group 3, and group 4 with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF CCITT RLE, Group 3, and Group 4.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Fax transition/run proofs and differential corpus.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.73.0 implementation stop reached. Run pentest for this exact commit.`

### v0.74.0 - Tiles, planar layouts, multipage/SubIFD traversal

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiles, planar layouts, multipage/subifd traversal. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiles, planar layouts, multipage/subifd traversal with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Tiles, planar layouts, multipage/SubIFD traversal.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tile geometry, IFD cycles, aggregate limits.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.74.0 implementation stop reached. Run pentest for this exact commit.`

### v0.75.0 - TIFF YCbCr, CMYK, CIELab, alpha, ICC

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff ycbcr, cmyk, cielab, alpha, icc. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff ycbcr, cmyk, cielab, alpha, icc with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF YCbCr, CMYK, CIELab, alpha, ICC.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Enumerate YCbCr coefficients, reference black/white, subsampling, positioning, and alpha dependencies.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Photometric/tag dependency and color vectors.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.75.0 implementation stop reached. Run pentest for this exact commit.`

### v0.76.0 - Corrected JPEG-in-TIFF, Exif IFDs, admitted extensions

Status: Planned.

Context:

This is the exclusive complex formats handoff for
corrected jpeg-in-tiff, exif ifds, admitted extensions. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete corrected jpeg-in-tiff, exif ifds, admitted extensions with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Corrected JPEG-in-TIFF, Exif IFDs, admitted extensions.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Old/new JPEG distinction and nested offsets.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.76.0 implementation stop reached. Run pentest for this exact commit.`

### v0.77.0 - TIFF baseline strip encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff baseline strip encoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff baseline strip encoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF baseline strip encoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Big/little-endian uncompressed baseline strips, tags, exact sizes, and round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.77.0 implementation stop reached. Run pentest for this exact commit.`

### v0.77.1 - TIFF compressed-strip encoders

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff compressed-strip encoders. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff compressed-strip encoders with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF compressed-strip encoders.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: PackBits/LZW/Deflate/fax process validity, predictors, bounds, dialect policy, and round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.77.1 implementation stop reached. Run pentest for this exact commit.`

### v0.77.2 - TIFF tile, planar, and multipage encoders

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff tile, planar, and multipage encoders. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff tile, planar, and multipage encoders with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF tile, planar, and multipage encoders.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tile geometry, plane offsets, IFD/SubIFD graph, aggregate limits, determinism, and round trips.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.77.2 implementation stop reached. Run pentest for this exact commit.`

### v0.77.3 - TIFF extended color and JPEG encoders

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff extended color and jpeg encoders. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff extended color and jpeg encoders with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF extended color and JPEG encoders.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Photometric dependencies, ExtraSamples, SampleFormat, YCbCr/ICC, corrected JPEG rules, and claims.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.77.3 implementation stop reached. Run pentest for this exact commit.`

### v0.77.4 - Complete declared TIFF 6.0-profile audit

Status: Planned.

Context:

This is the exclusive complex formats handoff for
complete declared tiff 6.0-profile audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete complete declared tiff 6.0-profile audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Complete declared TIFF 6.0-profile audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Freeze separate strip/tile/plane/page/compression/predictor/color/JPEG/tag claims; BigTIFF remains separate.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Compression/profile matrix, sparse/overlap policy, differential corpus, fuzzing, conformance, and external review.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.77.4 implementation stop reached. Run pentest for this exact commit.`


## Phase: Color, metadata, and processing

Re-audit rendered color through the early shared engine, add bounded structured metadata, then deliver declared-domain processing and selective decode.

### v0.78.0 - Cross-format native-sample and rendered-color conformance

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
cross-format native-sample and rendered-color conformance. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete cross-format native-sample and rendered-color conformance with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Cross-format native-sample and rendered-color conformance.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Re-run PNG/JPEG/WebP/TIFF rendered-color evidence through shared ICC, transfer, matrix/range, alpha, and numeric-tier APIs.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Codec declarations use the shared scalar color engine with no implicit sRGB or ambiguous alpha.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.78.0 implementation stop reached. Run pentest for this exact commit.`

### v0.79.0 - Shared bounded TIFF/Exif IFD inspection

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
shared bounded tiff/exif ifd inspection. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete shared bounded tiff/exif ifd inspection with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Shared bounded TIFF/Exif IFD inspection.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Share the bounded IFD engine with TIFF while MakerNotes stay opaque.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Offset graphs, entry counts, cycles, value bounds, MakerNote opacity, and fuzzing.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.79.0 implementation stop reached. Run pentest for this exact commit.`

### v0.80.0 - Selected Exif fields, thumbnails, and orientation policy

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
selected exif fields, thumbnails, and orientation policy. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete selected exif fields, thumbnails, and orientation policy with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Selected Exif fields, thumbnails, and orientation policy.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Exif orientation is metadata; only an explicit normalization transform changes pixels.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Dimensions/timestamps/strings/thumbnails are bounded and orientation is never silently applied.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.80.0 implementation stop reached. Run pentest for this exact commit.`

### v0.81.0 - XMP inspection and metadata conflict/rewrite policies

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
xmp inspection and metadata conflict/rewrite policies. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete xmp inspection and metadata conflict/rewrite policies with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: XMP inspection and metadata conflict/rewrite policies.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Choose raw XMP packet inspection or a separately bounded XML subset and define conflict precedence plus preserve/discard/rewrite policy.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Raw-versus-bounded-XML decision, precedence, preserve/discard/rewrite, and decompression limits.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.81.0 implementation stop reached. Run pentest for this exact commit.`

### v0.82.0 - YCbCr matrices, ranges, subsampling, and chroma siting

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
ycbcr matrices, ranges, subsampling, and chroma siting. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete ycbcr matrices, ranges, subsampling, and chroma siting with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: YCbCr matrices, ranges, subsampling, and chroma siting.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: JPEG/WebP/TIFF reference vectors.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.82.0 implementation stop reached. Run pentest for this exact commit.`

### v0.83.0 - CMYK, YCCK, Gray, Lab, and wide-gamut conversion

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
cmyk, ycck, gray, lab, and wide-gamut conversion. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete cmyk, ycck, gray, lab, and wide-gamut conversion with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: CMYK, YCCK, Gray, Lab, and wide-gamut conversion.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Black-generation policy and gamut tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.83.0 implementation stop reached. Run pentest for this exact commit.`

### v0.84.0 - Straight/premultiplied alpha conversion

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
straight/premultiplied alpha conversion. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete straight/premultiplied alpha conversion with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Straight/premultiplied alpha conversion.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Zero-alpha, rounding, and invariant tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.84.0 implementation stop reached. Run pentest for this exact commit.`

### v0.85.0 - Conversion planning, sample-depth changes, and advanced dithering

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
conversion planning, sample-depth changes, and advanced dithering. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete conversion planning, sample-depth changes, and advanced dithering with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Conversion planning, sample-depth changes, and advanced dithering.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Do not duplicate early GIF palette work; this release covers general conversion plans, depth changes, declared loss, and advanced dithering.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Declared information loss, numeric tier, scratch/work plan, and deterministic output.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.85.0 implementation stop reached. Run pentest for this exact commit.`

### v0.86.0 - Crop, flip, rotate, transpose

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
crop, flip, rotate, transpose. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete crop, flip, rotate, transpose with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Crop, flip, rotate, transpose.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: In-place overlap and rectangle proofs.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.86.0 implementation stop reached. Run pentest for this exact commit.`

### v0.87.0 - Checked affine geometry and border modes

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
checked affine geometry and border modes. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete checked affine geometry and border modes with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Checked affine geometry and border modes.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Finite-matrix and coordinate-overflow proofs.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.87.0 implementation stop reached. Run pentest for this exact commit.`

### v0.88.0 - Nearest and bilinear resampling

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
nearest and bilinear resampling. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete nearest and bilinear resampling with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Nearest and bilinear resampling.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Declare filtering domain, linear-light policy, premultiplication, zero-alpha handling, gamut behavior, rounding, and tolerance.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Pixel-center and edge-policy golden tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.88.0 implementation stop reached. Run pentest for this exact commit.`

### v0.89.0 - Bicubic resampling

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
bicubic resampling. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bicubic resampling with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Bicubic resampling.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Fix bicubic coefficients, FMA permission, overshoot, saturation, edge policy, and numeric tolerance.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Coefficient normalization and overshoot policy.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.89.0 implementation stop reached. Run pentest for this exact commit.`

### v0.90.0 - Lanczos3 resampling

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
lanczos3 resampling. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete lanczos3 resampling with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Lanczos3 resampling.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Fix Lanczos coefficient precision, normalization, FMA permission, edge policy, and reference tolerance.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tap planning, ring-buffer limits, reference vectors.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.90.0 implementation stop reached. Run pentest for this exact commit.`

### v0.91.0 - Porter-Duff compositing

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
porter-duff compositing. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete porter-duff compositing with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Porter-Duff compositing.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Pin the Porter-Duff source and declare linear/encoded domain, numeric tier, and premultiplication.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Linear-domain and alpha invariants.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.91.0 implementation stop reached. Run pentest for this exact commit.`

### v0.92.0 - Declared artistic blend modes

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
declared artistic blend modes. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete declared artistic blend modes with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Declared artistic blend modes.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Pin the blend specification and test clamping, NaN/infinity, gamut, and alpha.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Formula, clamping, NaN, and interoperability tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.92.0 implementation stop reached. Run pentest for this exact commit.`

### v0.93.0 - Optional isolated SIMD backends

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
optional isolated simd backends. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete optional isolated simd backends with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Optional isolated SIMD backends.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- The safe scalar path remains authoritative; unsafe acceleration requires a tiny optional independently audited adapter.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scalar differential, tail/alignment, Miri/sanitizer evidence.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.93.0 implementation stop reached. Run pentest for this exact commit.`

### v0.94.0 - Streaming and tiled processing graph

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
streaming and tiled processing graph. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete streaming and tiled processing graph with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Streaming and tiled processing graph.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scratch bounds, fusion equivalence, cancellation, and honest random-access disclosure.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.94.0 implementation stop reached. Run pentest for this exact commit.`

### v0.94.1 - Metadata/header-only, region, and frame-range decoding

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
metadata/header-only, region, and frame-range decoding. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete metadata/header-only, region, and frame-range decoding with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Metadata/header-only, region, and frame-range decoding.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Support is format-specific; decode-and-discard ROI fallback is permitted only with explicit budgets and disclosure.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Early termination, ROI bounds, valid-prefix semantics, frame selection, budgets, and format support matrix.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.94.1 implementation stop reached. Run pentest for this exact commit.`

### v0.94.2 - Reduced-resolution and progressive-preview decoding

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
reduced-resolution and progressive-preview decoding. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete reduced-resolution and progressive-preview decoding with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Reduced-resolution and progressive-preview decoding.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Scale-during-decode is admitted only where bitstream structure and numeric policy define it.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: JPEG reduced IDCT, TIFF strip/tile selection, progressive events, scale-during-decode policy, and numeric evidence.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.94.2 implementation stop reached. Run pentest for this exact commit.`

### v0.94.3 - Processing and selective-decoding security audit

Status: Planned.

Context:

This is the exclusive color, metadata, and processing handoff for
processing and selective-decoding security audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete processing and selective-decoding security audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Processing and selective-decoding security audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Fusion equivalence, scratch/peak limits, cancellation, DoS benchmarks, differential results, and API freeze.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.94.3 implementation stop reached. Run pentest for this exact commit.`


## Phase: Integration and assurance

Add optional adapters outside core, then freeze behavior through fuzzing, proofs, platform audits, reproducibility, and external review.

### v0.95.0 - Runtime-neutral async source/sink adapters

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
runtime-neutral async source/sink adapters. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete runtime-neutral async source/sink adapters with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Runtime-neutral async source/sink adapters.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Backpressure, cancellation, partial-I/O tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.95.0 implementation stop reached. Run pentest for this exact commit.`

### v0.95.1 - WASM/browser streaming adapters

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
wasm/browser streaming adapters. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete wasm/browser streaming adapters with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: WASM/browser streaming adapters.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: wasm32-unknown-unknown, JS-size, memory-growth tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.95.1 implementation stop reached. Run pentest for this exact commit.`

### v0.96.0 - Caller-provided parallel scheduling interface

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
caller-provided parallel scheduling interface. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete caller-provided parallel scheduling interface with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Caller-provided parallel scheduling interface.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Determinism, budget partition, cancellation.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.96.0 implementation stop reached. Run pentest for this exact commit.`

### v0.96.1 - Optional Rayon/service adapter

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
optional rayon/service adapter. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete optional rayon/service adapter with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Optional Rayon/service adapter.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: No core dependency or automatic global pool.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.96.1 implementation stop reached. Run pentest for this exact commit.`

### v0.97.0 - GPU-compatible descriptors and upload-layout hooks

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
gpu-compatible descriptors and upload-layout hooks. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gpu-compatible descriptors and upload-layout hooks with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: GPU-compatible descriptors and upload-layout hooks.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Stable layout contract; no device ownership in core.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.97.0 implementation stop reached. Run pentest for this exact commit.`

### v0.97.1 - Optional backend adapters

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
optional backend adapters. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete optional backend adapters with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Optional backend adapters.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: CPU/GPU differential results and synchronization policy.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.97.1 implementation stop reached. Run pentest for this exact commit.`

### v0.98.0 - mynd-cli inspect and validate

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
mynd-cli inspect and validate. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete mynd-cli inspect and validate with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: mynd-cli inspect and validate.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Escaped metadata, bounded defaults, exit-code contract.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.98.0 implementation stop reached. Run pentest for this exact commit.`

### v0.98.1 - CLI decode/encode/convert/frame operations

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
cli decode/encode/convert/frame operations. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete cli decode/encode/convert/frame operations with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: CLI decode/encode/convert/frame operations.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Transactional files and color-policy disclosure.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.98.1 implementation stop reached. Run pentest for this exact commit.`

### v0.98.2 - CLI batch/service profiles

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
cli batch/service profiles. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete cli batch/service profiles with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: CLI batch/service profiles.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Aggregate budgets, cancellation, hostile filename tests.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.98.2 implementation stop reached. Run pentest for this exact commit.`

### v0.99.0 - cargo-fuzz suites for every parser, entropy engine, metadata path, and dispatcher

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
cargo-fuzz suites for every parser, entropy engine, metadata path, and dispatcher. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete cargo-fuzz suites for every parser, entropy engine, metadata path, and dispatcher with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: cargo-fuzz suites for every parser, entropy engine, metadata path, and dispatcher.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Coverage report and minimized persistent corpus.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.99.0 implementation stop reached. Run pentest for this exact commit.`

### v0.99.1 - Long-running fuzz and every-byte/bit truncation campaigns

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
long-running fuzz and every-byte/bit truncation campaigns. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete long-running fuzz and every-byte/bit truncation campaigns with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Long-running fuzz and every-byte/bit truncation campaigns.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: No stalls, panics, or inconsistent terminal states.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.99.1 implementation stop reached. Run pentest for this exact commit.`

### v0.99.2 - Kani math/view/bit/geometry proofs

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
kani math/view/bit/geometry proofs. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete kani math/view/bit/geometry proofs with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Kani math/view/bit/geometry proofs.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Published assumptions and unwind bounds.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.99.2 implementation stop reached. Run pentest for this exact commit.`

### v0.99.3 - Kani Deflate/LZW/Huffman/JPEG/WebP/TIFF state proofs

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
kani deflate/lzw/huffman/jpeg/webp/tiff state proofs. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete kani deflate/lzw/huffman/jpeg/webp/tiff state proofs with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Kani Deflate/LZW/Huffman/JPEG/WebP/TIFF state proofs.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Output, dictionary, table, and progress invariants.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.99.3 implementation stop reached. Run pentest for this exact commit.`

### v0.99.4 - Miri, sanitizers, 32-bit, WASM, feature-combination, and stack audit

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
miri, sanitizers, 32-bit, wasm, feature-combination, and stack audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete miri, sanitizers, 32-bit, wasm, feature-combination, and stack audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Miri, sanitizers, 32-bit, WASM, feature-combination, and stack audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Measure stack ceilings and treat safe-Rust scratch clearing as best effort, never guaranteed erasure.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: All supported configurations pass.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.99.4 implementation stop reached. Run pentest for this exact commit.`

### v0.99.5 - Official conformance, differential, color, performance, and DoS freeze

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
official conformance, differential, color, performance, and dos freeze. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete official conformance, differential, color, performance, and dos freeze with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Official conformance, differential, color, performance, and DoS freeze.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Every claim linked to evidence; no unexplained disagreement.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.99.5 implementation stop reached. Run pentest for this exact commit.`

### v0.99.6 - Reproducible SBOM/package/provenance, external pentest, API freeze

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
reproducible sbom/package/provenance, external pentest, api freeze. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete reproducible sbom/package/provenance, external pentest, api freeze with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Reproducible SBOM/package/provenance, external pentest, API freeze.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Pin the advisory database commit/hash and separate offline reproducibility from online freshness.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: No critical/high finding; clean retest.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.99.6 implementation stop reached. Run pentest for this exact commit.`


## Phase: Production admission

Pentest and reproduce exact candidate archives, then promote without changing bytes or claims.

### v1.0.0-rc.1 - Exact versioned production candidate

Status: Planned.

Context:

This is the exclusive production admission handoff for
exact versioned production candidate. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete exact versioned production candidate with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Exact versioned production candidate.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Freeze exact crate archives; any byte, metadata, dependency, documentation, or package change requires a new candidate.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Pentest and reproduce the exact .crate archives.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v1.0.0-rc.1 implementation stop reached. Run pentest for this exact commit.`

### v1.0.0 - Byte-for-byte promotion of the approved candidate

Status: Planned.

Context:

This is the exclusive production admission handoff for
byte-for-byte promotion of the approved candidate. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete byte-for-byte promotion of the approved candidate with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Byte-for-byte promotion of the approved candidate.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update SPEC_MAPPING records, support claims, source/architecture records,
  corpus provenance, numeric tier/tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, and resource-accounting fixtures.
- Promote approved candidate artifacts byte-for-byte without rebuild or widened claims.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Signed checksums, SBOM, provenance, and stable support matrix.
- Audit arithmetic, offsets, loops, cumulative counters, live reservations,
  peak gauges, typed scratch, output, metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations,
  numeric tier/tolerance, and compatibility choices are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and release notes match the
  exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v1.0.0 implementation stop reached. Run pentest for this exact commit.`

## Post-1.0 admission

Post-1.0 formats require their own threat-boundary review, official source
ledger, granular versions, conformance profile, corpus provenance, resource
model, and exact-commit pentest. They never enter a 1.0 patch merely because
they share an image category.

The first later planning pass may separately evaluate TGA, AVIF/HEIF, JPEG XL,
JPEG 2000, JPEG-LS, JPEG XR, JPEG XS, OpenEXR, Radiance HDR, ICO/CUR, PSD/PSB,
DDS, KTX, SVG/rasterization, and emerging formats without treating one family
as an extension of another.
