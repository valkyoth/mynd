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

## Foundational data and color

The model separates PixelLayout (channels, samples, nominal/storage depth,
packing, endian, planes, stride, alpha), ColorEncoding (model, primaries,
white point, transfer, matrix, range, intent, ICC, HDR/WCG, known state), and
ImageMetadata (Exif, XMP, text, timing, dimensions, bounded raw blocks).

Codec APIs preserve native declarations and unsupported valid profiles; they
never silently assume sRGB or execute metadata. ICC is a separate crate and
threat boundary. NaN/infinity and tone mapping have explicit policy.

Safe validated byte views and naturally typed caller buffers are the baseline;
forbid(unsafe_code) rules out arbitrary byte-to-pixel casts. Future unsafe SIMD
or reinterpretation belongs in a tiny optional audited adapter behind a scalar
reference.

## Resource accounting and execution

One non-Clone monotonic ledger is shared by parsers, child streams, metadata,
ICC, processing, and adapters. Reserve before allocation/output, commit actual
use, release only transient memory, count concurrent peak, and charge
caller-owned output. No preset is unlimited.

| Group | Counters |
| --- | --- |
| Input | total/compressed/probe/skipped bytes, seeks, seek distance |
| Shape | dimensions, pixels, rows/planes, frames, total frame pixels |
| Structure | chunks, scans, tables, palettes, IFDs, strips, tiles, nesting |
| Output | decoded/encoded bytes, metadata output, preserved unknown bytes |
| Memory | persistent state, scratch, snapshots, coefficients, ICC, peak |
| Work | entropy symbols, back-references, filters, IDCT, taps, composition |
| Metadata | raw/decompressed bytes, strings, warnings, Exif, ICC tags |
| Compatibility | admitted repairs/extensions, never safety relaxation |

DecodePlan, ScratchPlan, and EncodePlan validate and calculate maxima without
mutating output. APIs are tiered into borrowed inspection, caller-buffer
execution, and optional fallible-owned convenience.

## Processing, adapters, and sanitization

- Pull streaming is used only where valid; affine, vertical, and animation
  operations disclose cache/random-access/snapshot requirements.
- Scalar algorithms are authoritative. SIMD/GPU adapters are optional and
  differential-tested across widths, alignments, tails, channels, and paths.
- Callers schedule parallel work after source, destination, and budget
  partitioning. Results stay deterministic.
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
| 0.5.0 | Sample, channel, packing, endianness, color-model, and alpha descriptors | Invalid-combination construction tests |
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
| 0.16.0 | Format IDs, media types, bounded probing, static registry | Collision, ambiguity, polyglot, and disabled-feature tests |
| 0.17.0 | Fallible owned storage and std::io adapters | Allocation-failure, interrupted-I/O, and feature-matrix tests |
| 0.18.0 | Foundation API freeze, fuzz harness primitives, Kani core suite | External design review and no-default/32-bit/WASM build matrix |
| 0.19.0 | Common codec crate template and decode-plan contract | A dummy codec proves limit, scratch, progress, and rollback invariants |
| 0.20.0 | BMP probe, file header, OS/2 and Windows DIB dispatch | Header-size confusion and offset corpus |
| 0.21.0 | BMP BI_RGB depths, palettes, padding, row orientation | 1/4/8/16/24/32-bit golden and truncation tests |
| 0.22.0 | BMP bitfields, alpha masks, top-down rules | Mask overlap/gap/full-width and signed-height tests |
| 0.23.0 | BMP RLE4/RLE8 | Escape, delta, padding, exact-output, and no-progress fuzzing |
| 0.24.0 | BMP V4/V5 color declarations and embedded-profile transport | Profile-range, overlap, and linked-profile no-I/O tests |
| 0.25.0 | BMP deterministic encoders and declared-dialect conformance | Round-trip, differential, corpus, and fuzz audit |
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
| 0.51.0 | GIF extensions, loop compatibility, raw/composited APIs, encoder | Normative/de-facto distinction and full audit |
| 0.52.0 | JPEG source map, marker/segment parser, frame/scan/table declarations | Marker mutation and segment-size fuzzing |
| 0.53.0 | JPEG Huffman entropy, stuffing, restart, bounded coefficients | Canonical table proofs and MCU accounting |
| 0.54.0 | Baseline scalar IDCT, sampling, upsampling, grayscale/YCbCr decode | Tolerance vectors and restart corpus |
| 0.55.0 | Extended sequential and 12-bit DCT processes | Precision and coefficient-range evidence |
| 0.56.0 | Progressive DC/AC and successive approximation | Scan-order/state-machine fuzzing |
| 0.57.0 | Lossless predictive JPEG process | Predictor, point transform, precision tests |
| 0.58.0 | JPEG arithmetic coding | Conditioning-table and arithmetic-state proofs |
| 0.59.0 | Differential and hierarchical processes | Frame dependency and reconstruction bounds |
| 0.60.0 | JFIF, Exif, ICC APP2 assembly, Adobe RGB/CMYK/YCCK | Segment reassembly, precedence, and color vectors |
| 0.61.0 | JPEG encoders by separately claimed process | Deterministic tables, rate/quality policy, round trips |
| 0.62.0 | Complete declared T.81 conformance and security audit | Reference software, official material, long fuzz campaign |
| 0.63.0 | WebP RIFF/VP8X container, chunk order, metadata | RFC 9649 conformance and size/padding fuzzing |
| 0.64.0 | VP8 Boolean decoder, partitions, headers, probabilities | Partition limits and arithmetic-state fuzzing |
| 0.65.0 | VP8 prediction, coefficients, inverse transforms, loop filters | Scalar reference differential tests |
| 0.66.0 | VP8 alpha and Y′CbCr-to-RGB pipeline | ALPH preprocessing and color tests |
| 0.67.0 | VP8L prefix coding, LZ77, color cache | Prefix/distance/cache proofs and bombs |
| 0.68.0 | VP8L transforms and complete lossless reconstruction | Transform recursion and meta-image limits |
| 0.69.0 | WebP animation, lossy/lossless encoders, full audit | RFC/reference differential and animation fuzzing |
| 0.70.0 | TIFF byte order, header, typed values, bounded IFD graph | Offset cycles, overlaps, entry/count multiplication |
| 0.71.0 | Baseline strips: bilevel, Gray, palette, RGB, uncompressed/PackBits | Strip-size and row-layout tests |
| 0.72.0 | TIFF LZW, Deflate, and horizontal predictors | Dialect policy and decompression bombs |
| 0.73.0 | TIFF CCITT RLE, Group 3, and Group 4 | Fax transition/run proofs and differential corpus |
| 0.74.0 | Tiles, planar layouts, multipage/SubIFD traversal | Tile geometry, IFD cycles, aggregate limits |
| 0.75.0 | TIFF YCbCr, CMYK, CIELab, alpha, ICC | Photometric/tag dependency and color vectors |
| 0.76.0 | Corrected JPEG-in-TIFF, Exif IFDs, admitted extensions | Old/new JPEG distinction and nested offsets |
| 0.77.0 | TIFF encoders and complete TIFF 6.0-profile audit | Big/little endian, strip/tile round trips, conformance freeze |
| 0.78.0 | Color-encoding model and format-specific precedence | No implicit-sRGB or ambiguous-alpha paths |
| 0.79.0 | Bounded ICC v2/v4 structural parser | Tag table, offset, overlap, and size fuzzing |
| 0.80.0 | ICC matrix/TRC and chromatic adaptation | ICC test profiles and deterministic PCS vectors |
| 0.81.0 | ICC LUT, mAB/mBA, intents, PCS Lab/XYZ | Interpolation and rendering-intent differential tests |
| 0.82.0 | YCbCr matrices, ranges, subsampling, and chroma siting | JPEG/WebP/TIFF reference vectors |
| 0.83.0 | CMYK, YCCK, Gray, Lab, and wide-gamut conversion | Black-generation policy and gamut tests |
| 0.84.0 | Straight/premultiplied alpha conversion | Zero-alpha, rounding, and invariant tests |
| 0.85.0 | Conversion planner, quantization, dithering, sample-depth changes | Declared loss and deterministic output |
| 0.86.0 | Crop, flip, rotate, transpose | In-place overlap and rectangle proofs |
| 0.87.0 | Checked affine geometry and border modes | Finite-matrix and coordinate-overflow proofs |
| 0.88.0 | Nearest and bilinear resampling | Pixel-center and edge-policy golden tests |
| 0.89.0 | Bicubic resampling | Coefficient normalization and overshoot policy |
| 0.90.0 | Lanczos3 resampling | Tap planning, ring-buffer limits, reference vectors |
| 0.91.0 | Porter-Duff compositing | Linear-domain and alpha invariants |
| 0.92.0 | Declared artistic blend modes | Formula, clamping, NaN, and interoperability tests |
| 0.93.0 | Optional isolated SIMD backends | Scalar differential, tail/alignment, Miri/sanitizer evidence |
| 0.94.0 | Streaming/tiled processing graph and processing audit | Scratch bounds, fusion equivalence, DoS benchmarks |
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

Establish claim discipline, checked representation, shared budgets, transactional I/O, and incremental contracts before parsing a real format.

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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Preserve the existing virtual workspace, facade/core skeletons, dual licensing, empty defaults, no_std boundary, policy set, and release tooling.
- Keep all image-model, parser, codec, and processing claims unavailable; the repository still has no implemented image behavior.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Current checks plus completed exact-commit pentest.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Reconcile README.md, FORMAT_SUPPORT.md, docs/IMPLEMENTATION_PLAN.md, SPEC_SOURCES.md, crate planning, and release automation with this normative sequence.
- Define machine-readable SPEC_MAPPING records with requirement ID, edition, disposition, implementation path, positive/negative tests, fuzz target, proof reference, and unsupported reason.
- Define corpus provenance records for source, license, hash, expected result, redistribution status, and specification relevance.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: No contradictions across README, support matrix, and normative plans.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Exhaustive extrema tests and Kani arithmetic proofs.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Zero/min/max, last-row, alignment, and 32-bit usize proofs.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.4.0 implementation stop reached. Run pentest for this exact commit.`

### v0.5.0 - Sample, channel, packing, endianness, color-model, and alpha descriptors

Status: Planned.

Context:

This is the exclusive foundations handoff for
sample, channel, packing, endianness, color-model, and alpha descriptors. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete sample, channel, packing, endianness, color-model, and alpha descriptors with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Sample, channel, packing, endianness, color-model, and alpha descriptors.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Separate PixelLayout, ColorEncoding, and ImageMetadata; nominal depth is distinct from storage width and neither sRGB nor alpha association is implicit.
- Represent native Gray/GA, RGB/BGR, RGBA/BGRA, CMYK, YCbCr, YCCK, Lab, indexed, planar/interleaved, endian, and alpha declarations before codecs freeze their APIs.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Invalid-combination construction tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.5.0 implementation stop reached. Run pentest for this exact commit.`

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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Short-buffer, row-boundary, alias-policy, and Miri tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Off-canvas and cumulative-duration tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Keep Malformed, Truncated, Unsupported, LimitExceeded, I/O, and internal-invariant failures distinct and never allocate attacker-controlled error strings.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Bounded formatting and terminal/log-injection tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Account independently for input, shape, structure, output, memory, work, metadata, and compatibility events.
- The ledger is non-Clone, monotonic, reservation-based, shared by child streams, aware of concurrent peak memory, and applies to caller-owned output; no preset is unlimited.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Budget-sharing and bypass property tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Introduce DecodePlan, ScratchPlan, and EncodePlan; planning validates and reserves maximum resources without mutating visible output.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Peak-memory accounting and failed-reservation tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Every-byte truncation and rollback tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Nested-bound escape, offset, and seek-cycle tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Define width-zero behavior, validate widths before shifts, and make refill transactional so truncation cannot partly advance semantic state.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Every-bit truncation, width, refill, and shift proofs.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Every step reports consumed bytes, produced units, and Progress, NeedInput, NeedOutput, or Done; (0, 0, Progress) is an invariant failure.
- Commit only at declared row/frame/checkpoint boundaries so callers receive either a known valid prefix or no visible partial output.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Chunk-boundary equivalence and zero-progress rejection.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Preserve valid unsupported metadata/profile payloads without executing or silently reinterpreting them, and debit nested decompression from the original ledger.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Offset/count validation without full metadata interpretation.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.15.0 implementation stop reached. Run pentest for this exact commit.`

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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Collision, ambiguity, polyglot, and disabled-feature tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Allocation-failure, interrupted-I/O, and feature-matrix tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Demonstrate no-default, 32-bit, wasm32, stack, and supported-target boundaries. Aesynx remains an architecture constraint until its target and CI runner exist.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: External design review and no-default/32-bit/WASM build matrix.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.18.0 implementation stop reached. Run pentest for this exact commit.`

## Phase: Simple and lossless codecs

Prove the architecture on bounded formats while keeping dialects, profiles, trailing-data rules, and encoder claims explicit.

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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Freeze borrowed inspection, caller-buffer execution, and optional fallible-owned API tiers.
- Require plan-before-execute, deterministic semantic work units, cancellation boundaries, and no hidden allocation, I/O, threads, clock, TLS, dynamic registry, or global state.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: A dummy codec proves limit, scratch, progress, and rollback invariants.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Header-size confusion and offset corpus.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: 1/4/8/16/24/32-bit golden and truncation tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Mask overlap/gap/full-width and signed-height tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Escape, delta, padding, exact-output, and no-progress fuzzing.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Treat linked BMP profile paths as inert data; they can never trigger filesystem, network, environment, or platform access.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Profile-range, overlap, and linked-profile no-I/O tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.24.0 implementation stop reached. Run pentest for this exact commit.`

### v0.25.0 - BMP deterministic encoders and declared-dialect conformance

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp deterministic encoders and declared-dialect conformance. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp deterministic encoders and declared-dialect conformance with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: BMP deterministic encoders and declared-dialect conformance.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Round-trip, differential, corpus, and fuzz audit.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.25.0 implementation stop reached. Run pentest for this exact commit.`

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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Pixel count, wraparound, end-marker, trailing-data tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Reference-vector and encode/decode conformance.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Comment, whitespace, decimal overflow, token-length fuzzing.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Bit order, row padding, multi-image policy.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: MAXVAL scaling, 8/16-bit, truncation.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Sample scaling, token bombs, binary boundaries.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tuple types, depth, header termination, unknown fields.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Concatenated images and official-tool differential tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Exact-size arithmetic, RGBA16-BE, alpha semantics.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Freeze only BMP dialects and QOI, Netpbm, and farbfeld profiles demonstrated by specification mappings and evidence.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Cross-codec probe fuzzing, 32-bit memory tests, external delta review.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.35.0 implementation stop reached. Run pentest for this exact commit.`

## Phase: Complex formats

Implement PNG, GIF, JPEG, WebP, and TIFF as separate audit surfaces with format-local entropy machines and profile-specific evidence.

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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Pin ISO/IEC 15948:2004, the dated W3C PNG Third Edition Recommendation and errata, RFC 1950, and RFC 1951 before code.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Unknown-critical and CRC policy tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Full normative combination matrix.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: RFC vectors, Adler-32, bit truncation.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Reject illegal canonical lengths, oversubscribed or disallowed incomplete trees, alphabet violations, and table overflow; build tables in fixed caller storage.
- Enforce the 32 KiB Deflate window, distance 1..=produced, block termination, output/symbol limits, Adler-32, and safe overlapping copies.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tree proofs, distance/overlap fuzzing, output bombs.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Per-filter/sample golden vectors.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scaling, endian, tail-bit tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Pass geometry proofs and tiny-image corpus.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Palette/index/transparency conformance.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Apply explicit PNG color-chunk precedence and preserve valid unsupported ICC/HDR declarations instead of treating them as sRGB.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: ICC bombs, precedence matrix, HDR/WCG vectors.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Metadata decompression and safe-to-copy tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: APNG state, frame, timing, disposal, blend, streaming, and animation-bomb tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: PNG Third Edition emission, round-trip, filter, Deflate, metadata, and determinism tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Frame-sequence, rectangle, timing, disposal/blend, and decode/encode round trips.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- A failed PNG audit delays GIF; unresolved PNG behavior cannot be merged into the next release family.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Third Edition conformance, differential, streaming, fuzz review.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Block termination and palette bounds.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Keep GIF LZW format-local; do not reuse TIFF LZW policy or create a universal entropy decoder.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Dictionary/code-width proofs and fuzzing.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Exact pixels and four-pass geometry tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Snapshot caps and animation bomb corpus.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.50.0 implementation stop reached. Run pentest for this exact commit.`

### v0.51.0 - GIF extensions, loop compatibility, raw/composited APIs, encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif extensions, loop compatibility, raw/composited apis, encoder. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif extensions, loop compatibility, raw/composited apis, encoder with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: GIF extensions, loop compatibility, raw/composited APIs, encoder.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Label Netscape looping as de facto interoperability behavior, not a GIF89a normative rule.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Normative/de-facto distinction and full audit.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.51.0 implementation stop reached. Run pentest for this exact commit.`

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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Pin ITU-T T.81 and corrigenda, then scope JFIF/T.871, Exif, ICC APP2, and Adobe conventions separately.
- Baseline plus progressive is not complete T.81: sequential, lossless, arithmetic, differential, hierarchical, precision, and encoder processes receive separate claims.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Marker mutation and segment-size fuzzing.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Keep JPEG Huffman and arithmetic state format-local while sharing only bounded bit I/O and proven canonical-table construction.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Canonical table proofs and MCU accounting.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tolerance vectors and restart corpus.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Precision and coefficient-range evidence.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scan-order/state-machine fuzzing.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Predictor, point transform, precision tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Conditioning-table and arithmetic-state proofs.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Frame dependency and reconstruction bounds.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Segment reassembly, precedence, and color vectors.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.60.0 implementation stop reached. Run pentest for this exact commit.`

### v0.61.0 - JPEG encoders by separately claimed process

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jpeg encoders by separately claimed process. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jpeg encoders by separately claimed process with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG encoders by separately claimed process.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Deterministic tables, rate/quality policy, round trips.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.61.0 implementation stop reached. Run pentest for this exact commit.`

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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- A failed JPEG audit delays WebP; the support matrix distinguishes every admitted T.81 decoder and encoder process.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Reference software, official material, long fuzz campaign.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Pin RFC 9649, RFC 6386, and the official VP8L bitstream specification before code.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: RFC 9649 conformance and size/padding fuzzing.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Partition limits and arithmetic-state fuzzing.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scalar reference differential tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: ALPH preprocessing and color tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Prefix/distance/cache proofs and bombs.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Transform recursion and meta-image limits.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.68.0 implementation stop reached. Run pentest for this exact commit.`

### v0.69.0 - WebP animation, lossy/lossless encoders, full audit

Status: Planned.

Context:

This is the exclusive complex formats handoff for
webp animation, lossy/lossless encoders, full audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete webp animation, lossy/lossless encoders, full audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: WebP animation, lossy/lossless encoders, full audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- A failed WebP audit delays TIFF; VP8, VP8L, ALPH, VP8X, animation, metadata, and unknown chunks require distinct evidence.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: RFC/reference differential and animation fuzzing.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.69.0 implementation stop reached. Run pentest for this exact commit.`

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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Pin TIFF 6.0, applicable Technical Notes, and corrected JPEG-in-TIFF rules. BigTIFF remains a separate unclaimed profile.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Offset cycles, overlaps, entry/count multiplication.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Strip-size and row-layout tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Make TIFF early/late LZW code-width behavior an explicit compatibility policy and never silently reuse GIF rules.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Dialect policy and decompression bombs.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Fax transition/run proofs and differential corpus.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tile geometry, IFD cycles, aggregate limits.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Photometric/tag dependency and color vectors.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Old/new JPEG distinction and nested offsets.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.76.0 implementation stop reached. Run pentest for this exact commit.`

### v0.77.0 - TIFF encoders and complete TIFF 6.0-profile audit

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff encoders and complete tiff 6.0-profile audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff encoders and complete tiff 6.0-profile audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF encoders and complete TIFF 6.0-profile audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Freeze separate claims for strips, tiles, planes, pages, compression families, predictors, JPEG variants, color spaces, ICC, and unknown tags.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Big/little endian, strip/tile round trips, conformance freeze.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.77.0 implementation stop reached. Run pentest for this exact commit.`

## Phase: Color and processing

Turn preserved native declarations into explicit deterministic plans with scalar references, bounded scratch, and disclosed loss.

### v0.78.0 - Color-encoding model and format-specific precedence

Status: Planned.

Context:

This is the exclusive color and processing handoff for
color-encoding model and format-specific precedence. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete color-encoding model and format-specific precedence with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Color-encoding model and format-specific precedence.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Expand foundational declarations into precedence-aware executable color policy without silently changing preserved native samples.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: No implicit-sRGB or ambiguous-alpha paths.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.78.0 implementation stop reached. Run pentest for this exact commit.`

### v0.79.0 - Bounded ICC v2/v4 structural parser

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Pin ICC.1:2022/profile version 4.4, amendments, and an explicit v2 baseline; ICC is its own crate and threat boundary.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tag table, offset, overlap, and size fuzzing.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.79.0 implementation stop reached. Run pentest for this exact commit.`

### v0.80.0 - ICC matrix/TRC and chromatic adaptation

Status: Planned.

Context:

This is the exclusive color and processing handoff for
icc matrix/trc and chromatic adaptation. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete icc matrix/trc and chromatic adaptation with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: ICC matrix/TRC and chromatic adaptation.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: ICC test profiles and deterministic PCS vectors.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.80.0 implementation stop reached. Run pentest for this exact commit.`

### v0.81.0 - ICC LUT, mAB/mBA, intents, PCS Lab/XYZ

Status: Planned.

Context:

This is the exclusive color and processing handoff for
icc lut, mab/mba, intents, pcs lab/xyz. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete icc lut, mab/mba, intents, pcs lab/xyz with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: ICC LUT, mAB/mBA, intents, PCS Lab/XYZ.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Preserve unsupported valid profiles; broad v4 claims require matrix/TRC, LUT, mAB/mBA, parametric curves, adaptation, PCS, intents, and deterministic interpolation.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Interpolation and rendering-intent differential tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.81.0 implementation stop reached. Run pentest for this exact commit.`

### v0.82.0 - YCbCr matrices, ranges, subsampling, and chroma siting

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: JPEG/WebP/TIFF reference vectors.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.82.0 implementation stop reached. Run pentest for this exact commit.`

### v0.83.0 - CMYK, YCCK, Gray, Lab, and wide-gamut conversion

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Black-generation policy and gamut tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.83.0 implementation stop reached. Run pentest for this exact commit.`

### v0.84.0 - Straight/premultiplied alpha conversion

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Zero-alpha, rounding, and invariant tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.84.0 implementation stop reached. Run pentest for this exact commit.`

### v0.85.0 - Conversion planner, quantization, dithering, sample-depth changes

Status: Planned.

Context:

This is the exclusive color and processing handoff for
conversion planner, quantization, dithering, sample-depth changes. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete conversion planner, quantization, dithering, sample-depth changes with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Conversion planner, quantization, dithering, sample-depth changes.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Reject or canonicalize NaN/infinity and disclose tone mapping, gamut mapping, quantization, dithering, and all information loss.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Declared loss and deterministic output.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.85.0 implementation stop reached. Run pentest for this exact commit.`

### v0.86.0 - Crop, flip, rotate, transpose

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: In-place overlap and rectangle proofs.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.86.0 implementation stop reached. Run pentest for this exact commit.`

### v0.87.0 - Checked affine geometry and border modes

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Do not market arbitrary affine transforms as constant-memory streaming; require a tile cache or random-access source when necessary.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Finite-matrix and coordinate-overflow proofs.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.87.0 implementation stop reached. Run pentest for this exact commit.`

### v0.88.0 - Nearest and bilinear resampling

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Pixel-center and edge-policy golden tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.88.0 implementation stop reached. Run pentest for this exact commit.`

### v0.89.0 - Bicubic resampling

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Coefficient normalization and overshoot policy.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.89.0 implementation stop reached. Run pentest for this exact commit.`

### v0.90.0 - Lanczos3 resampling

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Tap planning, ring-buffer limits, reference vectors.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.90.0 implementation stop reached. Run pentest for this exact commit.`

### v0.91.0 - Porter-Duff compositing

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Linear-domain and alpha invariants.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.91.0 implementation stop reached. Run pentest for this exact commit.`

### v0.92.0 - Declared artistic blend modes

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Formula, clamping, NaN, and interoperability tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.92.0 implementation stop reached. Run pentest for this exact commit.`

### v0.93.0 - Optional isolated SIMD backends

Status: Planned.

Context:

This is the exclusive color and processing handoff for
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- The safe scalar implementation remains authoritative. Unsafe or external SIMD belongs in a tiny optional independently audited adapter.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scalar differential, tail/alignment, Miri/sanitizer evidence.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.93.0 implementation stop reached. Run pentest for this exact commit.`

### v0.94.0 - Streaming/tiled processing graph and processing audit

Status: Planned.

Context:

This is the exclusive color and processing handoff for
streaming/tiled processing graph and processing audit. Its API and attack-surface delta must be implemented,
tested, reviewed, and pentested independently. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete streaming/tiled processing graph and processing audit with bounded behavior, explicit claims, and
evidence sufficient for an exact-commit security decision.

Deliverables:

- Complete only the release-scoped capability: Streaming/tiled processing graph and processing audit.
- Define its contract, invariants, limits, errors, feature behavior,
  output-commit semantics, compatibility policy, and unsupported cases.
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Expose pull streaming only where valid and disclose operations requiring buffering, random access, or frame snapshots.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Scratch bounds, fusion equivalence, DoS benchmarks.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
- The pentest covers the new attack surface and inherited invariants; all
  critical/high findings are fixed and every fix receives a clean retest.
- CI and CodeQL default setup are green, the permanent report records PASS, and
  the version-specific release gate accepts the exact reviewed commit.
- `v0.94.0 implementation stop reached. Run pentest for this exact commit.`

## Phase: Integration and assurance

Add optional environment adapters outside the core, then freeze behavior through fuzzing, proofs, platform audits, reproducibility, and review.

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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Async stays outside parsers: adapters drive the same deterministic incremental machine with backpressure.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Backpressure, cancellation, partial-I/O tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: wasm32-unknown-unknown, JS-size, memory-growth tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Callers schedule work; partition sources, disjoint destinations, and budgets before dispatch while keeping deterministic results.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Determinism, budget partition, cancellation.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: No core dependency or automatic global pool.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Stable layout contract; no device ownership in core.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: CPU/GPU differential results and synchronization policy.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Never render input-derived text directly to terminals/logs and never provide an unlimited default.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Escaped metadata, bounded defaults, exit-code contract.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Write outputs transactionally and call a path zero-copy only when layout, color encoding, and alpha association are representation-compatible.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Transactional files and color-policy disclosure.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Aggregate budgets, cancellation, hostile filename tests.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Coverage report and minimized persistent corpus.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: No stalls, panics, or inconsistent terminal states.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Published assumptions and unwind bounds.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Output, dictionary, table, and progress invariants.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Safe-Rust clearing is best effort: caller buffers remain caller-owned, abort skips Drop, and consumer profiles override workspace profiles.
- Explicitly clear initialized sensitive scratch on normal return; guaranteed non-elidable erasure requires a separately audited optional boundary.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: All supported configurations pass.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Every claim linked to evidence; no unexplained disagreement.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Pin the advisory-database commit/hash used as release evidence and separate reproducible offline verification from online freshness checks.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: No critical/high finding; clean retest.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Freeze exact crate archives. Any code, dependency, metadata, documentation, or package-content change invalidates approval and requires a new RC.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Pentest and reproduce the exact .crate archives.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
- Update applicable SPEC_MAPPING records, support claims, architecture records,
  corpus provenance, and security documentation.
- Add focused positive, boundary, malformed, truncation, mutation, regression,
  and deterministic-behavior fixtures.
- Promote only byte-for-byte artifacts approved as an RC; do not rebuild or widen claims.
- Update changelog, release notes, crate versions, package inventory, SBOM, and
  the exact-version pentest-report scaffold.

Verification:

- Required release evidence: Signed checksums, SBOM, provenance, and stable support matrix.
- Audit all input-derived arithmetic, offsets, loops, allocations, scratch,
  output, metadata, and work accounting touched by this release.
- Run every applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  performance, and denial-of-service check introduced by this handoff.
- Run scripts/checks.sh, cargo deny, cargo audit, latest-crate/tool checks, and
  SBOM verification.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform-target
  gates applicable to changed crates.

Exit criteria:

- The stated capability is complete, documented, and the only new capability.
- Every support/conformance claim links to passing evidence; limitations and
  compatibility choices are explicit with no unexplained differential.
- Packages, dependencies, SBOM, source mappings, fixtures, and release notes
  match the exact candidate commit.
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
