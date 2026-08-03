# mynd Release Plan To 1.0

Status: active planning document

Mynd currently provides checked arithmetic and validated geometry foundations,
not an image engine. The repository
has a no_std facade/core skeleton, security and release policy, and verification
tooling, but no image model, parser, codec, processing algorithm, or Rust
test-bearing behavior. Roadmap entries are not support claims; only implemented
behavior recorded in FORMAT_SUPPORT.md may be advertised.

This plan is granular because every image byte is hostile input. Each handoff
must be small enough to implement, test, review, and stop cleanly before
tagging. Pentests are cumulative publication checkpoints under the cadence
below. Split work whenever one safe review pass is insufficient. Schedule
pressure never permits scope to roll silently into the next version.

## Release sizing invariant

An implementation handoff owns one independently reviewable parser state
machine, algorithm, format process, metadata policy, adapter command, or facade
surface. An evidence-only handoff owns one bounded campaign over an unchanged
implementation. A release may integrate prerequisites needed for that one
outcome, but it may not introduce a second independent state machine or defer
part of its security proof to a later tag.

Field groups remain together only when one normative transition table and one
test matrix cover them. Format audits may aggregate already implemented,
unchanged surfaces, but cannot add behavior. If implementation review, fuzz
triage, or proof work cannot finish as one coherent decision, the handoff must
split before code begins. Cumulative pentest findings remain mapped to the
handoff that introduced the affected surface. The summary and detailed
handoffs are machine-checked for identical, monotonically ordered version sets.

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
farbfeld is a required pre-1.0 codec: its v0.34.0 decode/encode handoff and
v0.35.0 simple-codec security audit must remain in the production support
matrix through the 1.0 candidate and byte-for-byte promotion.

Version 0.2.0 is a blocking reconciliation across README.md,
FORMAT_SUPPORT.md, docs/IMPLEMENTATION_PLAN.md, docs/POST_1_0_CODEC_PLAN.md,
SPEC_SOURCES.md, crate planning, and release automation. No implementation
milestone may begin while those documents describe the former architecture or
scope; until reconciliation passes, no codec support is claimed.

Each .x family has one exclusive outcome. Every named version has its own
artifacts, evidence, release notes, commit, green GitHub checks, and tag
decision. Only rows marked `Yes` in the Pentest column require an evolving
pentest report and clean retest, and only rows marked `Publish` in the
Crates.io column are published. A failed format audit delays later work; it
cannot be hidden in the next format.

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
| JPEG | [ITU-T T.81](https://www.itu.int/rec/T-REC-T.81/en), corrigenda, separately scoped JFIF/T.871, T.86 registrations, Exif, ICC APP2, Adobe conventions | Sequential, progressive, lossless, arithmetic, differential, hierarchical, precision, COM/APPn policy, encoders |
| WebP | [RFC 9649](https://www.rfc-editor.org/info/rfc9649/), [RFC 6386](https://www.rfc-editor.org/rfc/rfc6386.html), [VP8L](https://developers.google.com/speed/webp/docs/webp_lossless_bitstream_specification) | VP8, VP8L, ALPH, VP8X, animation, metadata, unknown chunks, order |
| GIF | GIF87a and [GIF89a](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) | Normative blocks versus Netscape/de facto extensions |
| TIFF | [TIFF 6.0](https://www.itu.int/itudoc/itu-t/com16/tiff-fx/docs/tiff6.pdf), the referenced 1988 T.4/T.6 fax editions, Adobe PageMaker/Photoshop Technical Notes, and corrected JPEG-in-TIFF rules | Strips, tiles, planes, pages, compression, TIFF Predictor 2, separately sourced Adobe floating-point Predictor 3, JPEG variants, orientation, calibrated color, ICC, tags; BigTIFF is separate |
| BMP | Microsoft GDI and Open Specifications for the core/INFO/V4/V5 families, original IBM documentation for every admitted OS/2 family, and pinned primary provenance for compatibility headers | Exact header sizes, envelopes, palettes, masks, RLE, V4/V5 color, embedded profiles, inert linked paths, and explicit unsupported dialects |
| Netpbm | [PBM/PGM/PPM](https://netpbm.sourceforge.net/doc/pnm.html) and [PAM](https://netpbm.sourceforge.net/doc/pam.html) | Plain/raw syntax, comments, concatenation, MAXVAL, 16-bit, source-defined BT.709 transfer semantics, linear-opacity PAM tuples, and PAM; PFM is explicitly outside the official Netpbm claim and pre-1.0 scope |
| QOI | Author's [QOI specification](https://phoboslab.org/log/2021/12/qoi-specification) | Pixel termination, marker, wraparound, hints, trailing data |
| farbfeld | [suckless definition](https://tools.suckless.org/farbfeld/) | RGBA16-BE, unassociated alpha, exact length, trailing data |
| Shared color and blending | Pinned ICC v2 and ICC.1:2022/v4.4 sources, the ICC 2025 adaptive-gain amendment and ISO 21496-1:2025 dependency, sRGB, BT.601/709/2020, H.273, PQ/HLG when claimed, CIE XYZ/Lab, Porter-Duff, and the selected artistic-blend specification | Edition, numeric domain, rounding, alpha, gamut, patent/admission status, tolerance, and unsupported operations |
| Exif and XMP | CIPA DC-008-Translation-2026 Exif 3.1, CIPA DC-010-2026, Adobe XMP Parts 1-3, XML 1.0 Fifth Edition, Namespaces in XML 1.0 Third Edition, RDF 1.1 XML Syntax, and BCP 47 | Selected Exif profile, raw XMP packets, bounded XML/RDF inspection, language alternatives, no external resolution, reconciliation, and transformation effects |

## Format-profile decisions visible in release gates

- BMP gates separately decide every header/compression combination, including
  BI_JPEG/BI_PNG embedded payloads, de-facto 52/56-byte headers, and every
  documented OS/2 variant including RLE24 and Huffman 1D; linked profiles
  remain inert.
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
  IEEE-float samples, TIFF Predictor 2, the separately sourced Adobe
  floating-point Predictor 3 profile, YCbCr dependencies/siting, sparse strips,
  and overlapping strip/tile policy. BigTIFF stays separate.
- Netpbm gates define the exact header-to-raw-raster boundary, concatenated
  images, trailing material, the source-defined BT.709 transfer/range semantics
  versus common linear/sRGB variants, linear alpha/opacity, and unknown PAM
  tuple policy. Variants require explicit compatibility mode and never inherit
  the official PGM/PPM claim. PFM is not an official Netpbm format and remains
  unclaimed before 1.0.

## BMP dialect admission contract

BMP is a family of file envelopes, DIB header revisions, palette layouts,
compression modes, and color/profile extensions. Support is never inferred
from a `BM` signature or a header that is merely large enough. The first DWORD
of the DIB header selects an exact, allow-listed layout; unknown, truncated,
oversized, or internally inconsistent sizes fail closed without falling back
to a shorter structure.

| Version | Dialect responsibility |
| --- | --- |
| 0.20.0 | Pin the Microsoft and IBM sources, define `.bmp` versus bare-DIB entry points, validate the 14-byte file envelope, and freeze the header/compression/depth support matrix. |
| 0.20.1 | Parse the 12-byte core family with unsigned dimensions, planes=1, allowed depths, and three-byte `RGBTRIPLE` palette entries. |
| 0.20.2 | Parse Windows 40-byte `BITMAPINFOHEADER`, 108-byte `BITMAPV4HEADER`, and 124-byte `BITMAPV5HEADER`; admit 52/56-byte V2/V3 compatibility headers only from pinned primary provenance. |
| 0.20.3 | Parse each admitted IBM OS/2 2.x extended header revision and record explicit decisions for bitmap arrays and icon/pointer signatures. |
| 0.21.0 | Decode each admitted header/depth/palette BI_RGB combination with dialect-correct row and orientation rules. |
| 0.22.0 | Decode version-correct external versus inline RGB/alpha masks and reject invalid overlap, width, placement, or compression combinations. |
| 0.23.0 | Decode only admitted RLE4/RLE8 combinations, with compressed top-down forms rejected. |
| 0.24.0 | Interpret V4/V5 calibrated color, rendering intent, embedded-profile ranges, and inert linked-profile data without external I/O. |
| 0.25.0-0.25.1 | Emit only explicitly selected canonical dialects; every encoder declares its header, masks, palette, orientation, compression, and profile policy. |
| 0.25.2 | Audit the complete cross-product and publish supported, unsupported, and rejected combinations, including OS/2 Huffman 1D/RLE24 and BI_JPEG/BI_PNG decisions. |

The pre-1.0 file API admits standalone `BM` bitmap files. Bare packed DIBs use
a distinct API because they lack `BITMAPFILEHEADER`. OS/2 bitmap arrays and
icon/pointer signatures (`BA`, `IC`, `CI`, `PT`, and `CP`) remain unsupported
unless v0.20.3 explicitly admits their container semantics. A shared numeric
compression value is interpreted only inside its admitted header family;
Windows, OS/2, and de-facto extensions never borrow one another's semantics.

The dialect matrix keys every claim by file envelope, DIB header name and exact
size, dimensions/planes/depth, palette entry width, mask placement,
compression, orientation, color/profile fields, decode tier, and encode tier.
Each cell links to a primary source, implementation module, positive and
negative fixtures, truncation cases, differential evidence, and a final
Supported or Unsupported decision. “Unknown BMP version” is always Unsupported,
never a best-effort parse.

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
  NeedOutput, Yielded, Done, or a typed terminal result; zero/zero Progress is
  an invariant failure. Yielded is resumable and distinct from LimitExceeded.
- Nonterminal EOF is Truncated; unsupported normative behavior is Unsupported;
  bad data is Malformed; exhaustion is LimitExceeded.
- Output commits at declared rows, frames, or checkpoints, yielding a known
  valid prefix or no visible partial output.
- Share bounded bit I/O and proven canonical tables, not a universal entropy
  decoder. One independently audited mynd-deflate engine and thin mynd-zlib
  wrapper serve PNG data/metadata and TIFF Deflate. GIF LZW, TIFF LZW, JPEG
  entropy, VP8 Boolean coding, and VP8L prefix coding stay dialect-local.
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
scalar color, ICC, and minimal animation source/source-over foundations land
before complex codecs. Container releases transport profiles and apply
precedence only through those shared APIs.

Numeric evidence uses four tiers:

| Tier | Scope |
| --- | --- |
| Bit-exact | Parsing, lossless codecs, integer color paths, and integer compositing |
| Normative-tolerance | JPEG IDCT and other specification-tolerance lossy reconstruction |
| Reference-tolerance | Floating ICC, resize, SIMD, and GPU paths |
| Backend/config deterministic | Heuristic encoders with a declared backend, settings, search, and seed policy |

Every algorithm records rounding, saturation, coefficient precision, permitted
FMA use, backend constraints, and comparison tolerance. Cross-platform
bit-identity is claimed only for the bit-exact tier. The shared animation kernel
covers source replacement, source-over, straight/premultiplied conversion,
exact integer rounding, zero alpha, and format-specific blend/disposal mapping;
the later Porter-Duff release adds the remaining operators.

Elementary floating-point math is an explicit `no_std` backend contract. The
plan records which powers, roots, and related operations exist in pure `core`,
which use deterministic software approximations, and which require an optional
libm-style adapter. Domain reduction, exceptional inputs, maximum error,
rounding, FMA policy, and backend selection are visible in conformance claims.
Callers may select the deterministic software backend where supported; a
backend change can never silently retain stronger evidence.

Safe validated byte views and naturally typed caller buffers are the baseline.
Every first-party Rust crate retains #![forbid(unsafe_code)]; arbitrary
byte-to-pixel casts, unsafe arenas, and first-party unsafe SIMD are outside the
contract. SIMD uses safe Rust or an explicitly audited optional external
backend with no transitive/default activation. Disabling acceleration always
retains the authoritative scalar implementation and its full conformance path.

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

## Source, sink, lifecycle, and output tiers

Planning negotiates explicit source capabilities (ForwardOnly, Seek, ReadAt,
KnownLength) and sink capabilities (ForwardOnly, Seek, WriteAt, Transactional).
Missing capabilities return RequiresCapability before output changes. Every
encoder returns a compositional EncodeLayoutPlan with independent fields:
LengthKnowledge (Exact, UpperBound, Unknown), EmissionStrategy (OnePass,
CountThenEmit, Backpatch, BufferedRegion), EncodeInputCapabilities (SinglePass,
Replayable, RandomAccess, stable frame/metadata sequence), SinkCapabilities,
ScratchRequirements, and CommitPolicy. Buffered regions have explicit maxima
charged to the live-memory ledger. Counting and emission both spend input/work
budgets and bind identical configuration, metadata decisions, numeric backend,
and source pixels. BMP sizes, PNG chunks, WebP RIFF lengths, TIFF offsets,
metadata insertion, and transcoders use this common contract; a forward-only
input cannot promise replay and a forward sink cannot promise rollback.

Planning is bound to inspected bytes and configuration. A source-bound
DecodeSessionPlan uses typestate retaining the exact inspection session, exact
retained plan-critical header bytes, an unforgeable adapter/session-generation
token, or a collision-resistant digest only after that dependency is
intentionally admitted. Ordinary non-cryptographic fingerprints are not
adversarial identity. Binding covers codec/profile declarations, source epoch
or prefix, limits, compatibility, native/rendered choice, output layout/color
encoding, numeric backend, metadata policy, and commit mode. A reusable
format-neutral LayoutPlan is distinct.

Execution revalidates binding before output changes and still validates every
bitstream structure; a plan is never authority to skip parser checks. Mutable
sources either promise a stable snapshot for the session or receive no
cross-reread consistency guarantee while remaining memory-safe and fail-closed.
A later inconsistent read never replaces retained plan-critical declarations.

Decode commit policy is physically enforceable: Atomic stages all changes;
CommittedRows and CommittedFrames validate a complete unit in scratch before
copying; CommittedRegions returns bounded region tokens for tiled/parallel
work. A separately named relaxed mode may leave uncommitted bytes unspecified.
Scratch and destination requirements are known during planning; insufficiency
fails before mutation and no mode silently downgrades.

Every public incremental decoder and encoder trait method returns one fixed,
allocation-free concrete alias established at v0.14.0. The normative shape is:

```rust
pub type DecodeStepReport<'a> = StepReport<StepState, CommitSet<'a>>;
pub type EncodeStepReport<'a> = StepReport<StepState, CommitSet<'a>>;

#[non_exhaustive]
pub enum StepState {
    Progress,
    NeedInput { minimum_additional: usize },
    NeedOutput(OutputNeed),
    Yielded,
    Done,
    Cancelled,
    LimitExceeded,
    Error,
}

#[non_exhaustive]
pub enum OutputNeed {
    Destination { minimum_additional: usize },
    CommitTokens { minimum_additional: usize },
}

#[non_exhaustive]
pub enum CommitSet<'a> {
    None,
    Bytes(ByteRange),
    Rows(RowRange),
    Frames(FrameRange),
    Regions(&'a [RegionToken]),
}
```

`StepReport` also contains exact input_consumed, output_produced, and
work_consumed fields. The aliases, explicit lifetime, fields, and all eventual
state/commit categories exist at v0.14.0; v0.14.1 and v0.14.2 enable their
behavior rather than redesigning the return type. The region slice borrows
caller-provided, plan-sized token storage. Execution reserves a token slot
before committing a region. Exhaustion returns
`NeedOutput(OutputNeed::CommitTokens { minimum_additional })`, preserves every
already-issued token in the reported slice, and commits no unreportable pixels.
No path allocates or grows a hidden collection. Region-token count/storage
spend limits, and tokens bind to the session/output generation so reset or
another destination rejects them. A linear “valid prefix” is never used for
disjoint regions.

Execution follows Probe -> InspectHeader -> Plan -> BindWorkspace -> Execute,
then terminates as Done, Cancelled, or Error. All three outcomes are sticky. A
post-terminal step returns a deterministic state error. Reset is explicit,
clears format-local state, and never restores source position, destination
contents, or cumulative budget. NeedInput/NeedOutput report exact
consumption/production, expose only committed output, and retain no borrow
beyond the call unless encoded by a lifetime. Compatibility warnings cannot
repair an Error. Done requires the format's terminator and trailing-data policy.
Animations and indefinite streams may refine plans only within original limits.
Each step also receives a caller-controlled work grant or maximum execution
quantum. Entropy loops, ICC CLUTs, resampling taps, animation composition, and
encoder searches expose deterministic cancellation checkpoints. Quantum
exhaustion returns Yielded { work_consumed }, reveals no output beyond a commit
boundary, and resumes without repeating or skipping semantic work. A zero grant
returns Yielded without touching input or output. LimitExceeded is sticky
global exhaustion. Cancelled is a terminal caller abort that requires explicit
Reset and never refunds cumulative budget. These distinctions bound
cancellation latency even for large valid inputs.

Codec output has two explicit tiers:

- decode_native returns native indices, planes, samples, coefficients where
  admitted, palettes, and color declarations.
- decode_rendered applies the shared color/conversion engine to a
  caller-selected pixel layout and color encoding.

Native codec conformance and rendered-color conformance are evidenced
separately so entropy/container failures cannot be confused with conversion
failures.

Forward-only probing uses a caller-owned bounded prefix buffer. Every candidate
sees identical bytes; the selected decoder inherits consumed prefix bytes
without seeking. NeedInput reports both minimum additional bytes and the
absolute cap. Physical bytes are charged once while repeated probe work is
charged each time. Ambiguous/rejected probing returns the intact prefix.
Seekable probing restores its original logical position or reports failure.

Before foundation candidate review, public types receive an explicit
reentrancy and auto-trait contract. Decoder, encoder, plan, workspace, budget,
view, registry, and transform types state intended `Send`/`Sync` behavior;
compile-time assertions enforce it. Plans may execute concurrently only with
independent workspaces, parent-backed budget grants, disjoint destination
regions, and exclusive scratch. Registries and reusable color transforms are
immutable and reentrant where claimed.

## Incremental contract sequencing invariants

The incremental contracts have a strict dependency order that release
renumbering must not weaken:

| Version | Prerequisite established |
| --- | --- |
| 0.12.3 | Source/session identity uses retained state or strong identity and defines mutable-source consistency. |
| 0.14.0 | Every trait fixes `DecodeStepReport<'a>`/`EncodeStepReport<'a>` aliases, declares all eventual non-exhaustive states and commit categories, and uses caller-planned region-token storage. |
| 0.14.1 | Resumable `Yielded` is distinct from terminal exhaustion and cancellation. |
| 0.14.2 | Commit modes integrate every `StepReport` state with planned staging and generation-bound tokens. |

Physical incremental commit modes cannot move back to 0.12.4 or any release
before `Yielded` exists. Planning may define generic scratch and binding
primitives earlier, but evidence for commits on every incremental outcome
belongs exclusively to 0.14.2.

## Facade stabilization sequencing invariants

The facade is exercised across every execution domain before its final public
API freeze:

| Version | Stabilization responsibility |
| --- | --- |
| 0.94.8 | Establish the audited facade candidate and integration baseline after representative synchronous codec paths exercise it. |
| 0.95.0-0.95.1 | Exercise lifetimes, pinning, backpressure, cancellation, buffer ownership, and browser memory behavior through async and WASM adapters. |
| 0.96.0-0.96.1 | Exercise scheduler ownership, budget partitioning, cancellation, and service integration through parallel adapters. |
| 0.97.0-0.97.1 | Exercise descriptor stability, row alignment, synchronization, and device-boundary ownership through GPU-facing adapters. |
| 0.98.0-0.98.7 | Exercise diagnostics, transactional output, command isolation, batch limits, and service profiles through CLI integration. |
| 0.98.8 | Resolve every cross-adapter facade issue, prohibit adapter-specific forks, rerun affected compatibility matrices, and establish the exact implementation admitted to assurance. |
| 0.99.0-0.99.17 | Fuzz, prove, and audit the exact reconciled v0.98.8 implementation; any implementation or public-API correction invalidates affected evidence and returns to reconciliation. |
| 0.99.18 | Verify and freeze the already-corrected facade, assemble the cumulative pentest handoff, and make no implementation or public-API correction. |

Every v0.95.x-v0.98.7 handoff records whether it exposed facade pressure. Any
required correction is reviewed, documented, compatibility-tested, and
included in the next cumulative pentest or becomes a v0.98.8 release blocker.
v0.98.8 exits
only with zero unresolved implementation or public-API issues and no adapter
forking or bypassing the shared facade. The v0.99.x assurance campaign applies
to that exact reconciled implementation. A later implementation or public-API
change invalidates affected evidence and requires reconciliation plus reruns;
v0.99.18 only verifies and freezes the already-corrected facade.

## Metadata and selective decoding contract

Container milestones initially preserve bounded raw Exif, ICC, and XMP
transport. Structured v1 inspection is added later through a shared TIFF/Exif
IFD parser, a CIPA Exif 3.1 selected-field profile, opaque MakerNotes, bounded
string values, explicit thumbnail limits, and independently reviewed XMP
packet, XML/Namespaces, and RDF/XMP data-model layers included in the next
cumulative pentest. CIPA DC-010-2026 binds
Exif/XMP reconciliation; no metadata identifier is dereferenced.

Exif orientation is metadata until a caller explicitly requests normalization.
Transcoders choose discard, inspect, preserve raw, parse selected namespaces,
or rewrite; conflict and precedence policy is explicit.

Every processing plan returns MetadataEffect values: Preserved, Rewritten,
Invalidated, or RequiresCallerDecision. Geometry, color, palette, and animation
operations explicitly account for pixel dimensions, orientation, thumbnails,
resolution/aspect, PNG safe-to-copy chunks, profiles/color declarations,
histograms/palette suggestions, hashes/previews, frame rectangles, and timing.
Stale metadata is never copied automatically.

Decoder traits receive a foundation candidate review with optional planning hooks for metadata/header-only
termination, region-of-interest, reduced-resolution output, strip/tile
selection, progressive preview events, animation frame ranges, and
scale-during-decode. Support is format-specific and claimed only when its later
0.94.x handoff passes. v0.94.8 establishes the audited facade candidate;
v0.95.x-v0.98.7 adapter evidence informs reviewed corrections, v0.98.8 closes
them before assurance begins, and the full public facade freezes only at
v0.99.18.

Selective decode names coordinate spaces: encoded/native image,
orientation-normalized image, animation canvas, and frame-local rectangle.
Frame selection distinguishes raw from composited frames. ROI plans state
chroma-subsampling expansion, resampling halos, and final cropping. Exif
orientation remains unapplied unless the caller explicitly selects oriented
coordinates.

## Processing, adapters, and sanitization

- Resize and interpolation declare their color domain. Production defaults
  account for linear-light filtering, premultiplication before filtering alpha,
  zero/near-zero unpremultiplication, and gamut handling after interpolation.
- Streaming and tiled filters declare halo regions for every plane and prove
  seam equivalence. Lanczos, bicubic, affine borders, chroma planes, and
  premultiplied-alpha filtering match whole-image, scanline-band, and disjoint
  tile execution within the declared numeric tier.
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
release notes, crate metadata, package inspection, SBOM, CI, and CodeQL remain
mandatory for every tag. Pentest history and crates.io publication are
mandatory only at publication checkpoints.

The cadence is authoritative:

| Range | GitHub tag | Pentest | Crates.io |
| --- | --- | --- | --- |
| Through `v0.4.0` | Every named version | Exact-version review | Every named version |
| `v0.5.0` through `v0.95.0` | Every named version | Cumulative at `v0.5.0`, then each fifth minor checkpoint: `v0.10.0`, `v0.15.0`, ... `v0.95.0` | Only the same checkpoint versions |
| After `v0.95.0` through `v0.99.18` | Every named version | Rolled into `v1.0.0-rc.1` | No interim publication |
| `v1.0.0-rc.1` | Yes | Cumulative production-candidate pentest | Publish prerelease |
| `v1.0.0` | Yes | Reuse RC evidence only for the defined byte-for-byte promotion; otherwise retest | Publish stable |

An interim row marked `No` is a **GitHub engineering checkpoint**. It receives
the normal security tests, review, documentation, commit, CI, CodeQL, and tag,
but it is neither externally pentested nor published to crates.io. Its release
notes must say so and name the next cumulative checkpoint. A Git tag requires
a commit, so "no publication commit" never means skipping the implementation
commit used by that tag.

A checkpoint pentest covers the complete delta since the preceding published
checkpoint: source, dependencies, generated artifacts, documentation claims,
packaging, and every carried-forward finding. Keep
`security/pentest/vX.Y.Z.md` for that checkpoint, repeat remediation and
retesting until `Status: PASS`, commit the complete candidate and report, and
wait for GitHub CI and CodeQL. If GitHub exposes a problem, record the
correction and relevant retest in the same report and repeat. Tag and publish
only the final green commit.

This gives crates.io consumers cumulative externally reviewed checkpoints;
interim GitHub tags deliberately have a lower assurance label and are not
represented as published releases. A critical/high vulnerability, compromise,
or defect in an already published artifact may trigger an out-of-cadence
pentest and patch publication. Record why the emergency exception was used.

The summary Pentest and Crates.io cells override generic security wording in a
detailed handoff. A pentest, clean retest, permanent report, or strict
publication-gate clause applies only when that row says `Yes`/`Publish`; a `No`
row carries its delta to the named checkpoint instead.

## Crate versioning

The mynd facade is the integration train. Changed support crates use independent
versions; unchanged crates are not republished. Support-crate versions may
advance in interim source tags, but only the latest required unpublished
versions are published in dependency order at the next checkpoint. Update
release-crates.toml, the crate-version matrix, changelog, notes, package
inventory, and SBOM for every tag; update pentest metadata only at a pentest
checkpoint.

Crates enter only at their first handoff. Intended inward layering is
mynd-math, mynd-core, mynd-budget, mynd-io, mynd-deflate, mynd-zlib,
mynd-metadata, mynd-icc, mynd-ifd, mynd-exif, mynd-color, mynd-codec, one
mynd-format crate per family, mynd-processing, and mynd. mynd-deflate is shared
only where the bitstream is RFC 1951; GIF/TIFF LZW remain separate. mynd-ifd
owns bounded graph/value mechanics while TIFF and Exif own their schemas.
mynd-math owns deterministic elementary approximations; any external libm
integration is optional and cannot enter the core dependency graph. Optional
outer adapters include alloc/std, async, Rayon, WASM, GPU, and CLI.

## Milestone summary

| Version | Exclusive capability | Mandatory evidence | Pentest | Crates.io |
| --- | --- | --- | --- | --- |
| 0.1.0 | Existing workspace, licenses, feature boundaries, release policy | Current checks plus completed pentest cycle | Yes (exact version) | Publish `0.1.0` |
| 0.2.0 | Unified scope, claim taxonomy, standards/errata ledger, corpus provenance schema | No contradictions across README, support matrix, and normative plans | Yes (exact version) | Publish `0.2.0` |
| 0.2.1 | Reproducible specification corpus and legal-disposition gate | Every source classified public/offline/manual, immutable hashes, clean recreation, package exclusion, and legal review | Yes (exact version) | Publish `0.2.1` |
| 0.3.0 | Checked conversion/add/multiply/align/range primitives | Exhaustive extrema tests and Kani arithmetic proofs | Yes (exact version) | Publish `0.3.0` |
| 0.4.0 | Validated dimensions, rectangles, strides, planes, and output lengths | Zero/min/max, last-row, alignment, and 32-bit usize proofs | Yes (exact version) | Publish `0.4.0` |
| 0.5.0 | Explicit pixel layout and sample-storage domains | Invalid layout/sample/plane/chroma/alpha combinations are unrepresentable | Yes (cumulative checkpoint) | Publish `0.5.0` |
| 0.5.1 | Numeric determinism and floating-sample contract | Bit-exact/tolerance/backend tiers plus rounding, saturation, FMA, NaN, infinity, zero, and subnormal tests | No; covered by `0.10.0` | Not published; next `0.10.0` |
| 0.5.2 | Shared color and blending specification ledger | Pinned ICC, sRGB, BT.601/709/2020, H.273, HDR, CIE, Porter-Duff, and blend sources with claim scope | No; covered by `0.10.0` | Not published; next `0.10.0` |
| 0.5.3 | Scalar transfer, matrix/range, and alpha foundation | Integer/scalar reference vectors, premultiplication, range conversion, rounding, and native-sample preservation | No; covered by `0.10.0` | Not published; next `0.10.0` |
| 0.5.4 | Minimal scalar animation composition kernel | Source/source-over, straight/premultiplied conversion, exact integer rounding, zero alpha, and format mapping | No; covered by `0.10.0` | Not published; next `0.10.0` |
| 0.5.5 | Explicit no_std elementary-math backend | Pure-core/software/optional backend selection, domains, maximum error, FMA policy, and cross-target vectors | No; covered by `0.10.0` | Not published; next `0.10.0` |
| 0.6.0 | Immutable/mutable image and plane views | Short-buffer, row-boundary, alias-policy, and Miri tests | No; covered by `0.10.0` | Not published; next `0.10.0` |
| 0.7.0 | Frames, timing, canvas, disposal, blend, and frame rectangles | Off-canvas and cumulative-duration tests | No; covered by `0.10.0` | Not published; next `0.10.0` |
| 0.8.0 | Allocation-free structured errors, warnings, reports, and offsets | Bounded formatting and terminal/log-injection tests | No; covered by `0.10.0` | Not published; next `0.10.0` |
| 0.9.0 | DecodeLimits, EncodeLimits, monotonic work/memory ledger | Budget-sharing and bypass property tests | No; covered by `0.10.0` | Not published; next `0.10.0` |
| 0.10.0 | Caller-owned scratch planner, arenas, buffer pools, and leases | Peak-memory accounting and failed-reservation tests | Yes (cumulative checkpoint) | Publish `0.10.0` |
| 0.11.0 | Slice reader/writer, exact reads, fixed output, checkpoints | Every-byte truncation and rollback tests | No; covered by `0.15.0` | Not published; next `0.15.0` |
| 0.12.0 | Endian I/O, subranges, counting, seek/read-at | Nested-bound escape, offset, and seek-cycle tests | No; covered by `0.15.0` | Not published; next `0.15.0` |
| 0.12.1 | Source/sink capability negotiation and execution lifecycle | Capability planning plus sticky Done/Cancelled/Error and non-restoring Reset tests | No; covered by `0.15.0` | Not published; next `0.15.0` |
| 0.12.2 | General encoder sizing and commit planning | Compositional length/strategy/input/sink/scratch/commit fields, replayability, two-pass identity, and pre-mutation failure | No; covered by `0.15.0` | Not published; next `0.15.0` |
| 0.12.3 | Source-bound decode session planning | Exact-session/header/token/digest identity, stable-snapshot policy, complete configuration binding, and pre-output revalidation | No; covered by `0.15.0` | Not published; next `0.15.0` |
| 0.13.0 | MSB/LSB bit readers and writers | Every-bit truncation, width, refill, and shift proofs | No; covered by `0.15.0` | Not published; next `0.15.0` |
| 0.14.0 | Incremental decoder/encoder progress contracts | Fixed `DecodeStepReport<'a>`/`EncodeStepReport<'a>` return aliases, all eventual non-exhaustive states/commit categories, caller-planned token storage, and exact accounting | No; covered by `0.15.0` | Not published; next `0.15.0` |
| 0.14.1 | Cooperative execution quantum and cancellation latency | Bounded work, Yielded/LimitExceeded/Cancelled semantics, deterministic resume, committed prefixes, and latency tests | No; covered by `0.15.0` | Not published; next `0.15.0` |
| 0.14.2 | Physically enforceable incremental decode commit modes | Planned atomic/unit/region staging, no downgrade, generation-bound tokens, and StepReport commits on every state | No; covered by `0.15.0` | Not published; next `0.15.0` |
| 0.15.0 | Metadata envelopes and bounded Exif/ICC/XMP header transport | Offset/count validation without full metadata interpretation | Yes (cumulative checkpoint) | Publish `0.15.0` |
| 0.15.1 | Bounded ICC v2/v4 structural parser | Tag counts/sizes/offsets/overlap, curves, LUT dimensions, recursion, opaque preservation, and fuzzing | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.15.2 | ICC matrix/TRC and chromatic-adaptation engine | Parametric curves, PCS conversion, adaptation, rendering intent, deterministic scalar vectors, and limits | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.15.3 | ICC v2 LUT pipelines and deterministic interpolation | LUT dimensions/elements, interpolation, PCS bounds, intent, numeric tolerance, and v2 profiles | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.15.4 | ICC v4 mAB/mBA and processing-element pipelines | Element counts/types/order, curves, matrices, CLUTs, recursion, interpolation, and v4 profiles | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.15.5 | ICC PCS Lab/XYZ, intent selection, and core execution audit | Core execution-profile matrix, PCS/intents/adaptation, preservable Unsupported profiles, and differential tests | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.15.6 | ICC adaptive-gain tag and type structural parsing | ICC 2025 amendment and ISO 21496-1:2025 provenance, exact bounds, offsets, counts, floats, CICP fields, and patent review | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.15.7 | ICC adaptive-gain execution admission | Piecewise-cubic evaluation, headroom interpolation, numeric limits and tolerances, or an explicit legally reviewed unclaimed decision | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.15.8 | Complete declared ICC profile audit | Amendment-inclusive profile matrix, unsupported preservation, conformance, differential, fuzzing, and freeze | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.15.9 | Bounded BCP 47 language-tag profile | RFC 5646 syntax/canonicalization, RFC 4647 matching, private-use and `x-default`, length/work bounds, and cross-metadata vectors | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.16.0 | Format IDs, media types, bounded probing, static registry | Collision, ambiguity, polyglot, and disabled-feature tests | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.16.1 | Non-destructive forward-only and seekable probing | Shared caller prefix, decoder inheritance, NeedInput minima/cap, one-time byte charging, and seek restoration | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.17.0 | Fallible owned storage and std::io adapters | Allocation-failure, interrupted-I/O, and feature-matrix tests | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.17.1 | Reentrancy, concurrency, and auto-trait contract | Send/Sync assertions, independent-workspace concurrency, disjoint output, immutable registry, and scratch ownership tests | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.18.0 | Foundation candidate review and representative-codec readiness | External design review, dummy lifecycle exercise, no-default/32-bit/WASM matrix, and documented evolvability | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.19.0 | Common codec crate template and decode-plan contract | A dummy codec proves limit, scratch, progress, and rollback invariants | No; covered by `0.20.0` | Not published; next `0.20.0` |
| 0.20.0 | BMP source ledger, file envelope, and dialect-matrix freeze | `.bmp`/bare-DIB separation, exact-size dispatch policy, primary-source provenance, and combination matrix | Yes (cumulative checkpoint) | Publish `0.20.0` |
| 0.20.1 | BMP 12-byte core-family headers and RGBTRIPLE palettes | Unsigned dimensions, planes/depth matrix, three-byte palette bounds, offsets, and truncation | No; covered by `0.25.0` | Not published; next `0.25.0` |
| 0.20.2 | BMP Windows INFO/V4/V5 and V2/V3 compatibility headers | Exact 40/108/124-byte dispatch, 52/56-byte provenance gate, field-boundary corpus, and no fallback | No; covered by `0.25.0` | Not published; next `0.25.0` |
| 0.20.3 | BMP OS/2 2.x extended headers and container decision | IBM-source revision matrix, palette/layout differences, compression namespace, and BA/IC/CI/PT/CP policy | No; covered by `0.25.0` | Not published; next `0.25.0` |
| 0.21.0 | BMP BI_RGB depths, palettes, padding, row orientation | Per-header depth/palette/stride/orientation matrix, 1/4/8/16/24/32-bit goldens, and truncation tests | No; covered by `0.25.0` | Not published; next `0.25.0` |
| 0.22.0 | BMP bitfields, alpha masks, top-down rules | External/inline mask placement, BI_ALPHABITFIELDS decision, overlap/gap/full-width, and signed-height tests | No; covered by `0.25.0` | Not published; next `0.25.0` |
| 0.23.0 | BMP RLE4/RLE8 | Header/depth admission, bottom-up enforcement, escape/delta/padding/exact-output, and no-progress fuzzing | No; covered by `0.25.0` | Not published; next `0.25.0` |
| 0.24.0 | BMP V4/V5 color declarations and embedded-profile transport | Calibrated/sRGB/profile/intent matrix, range/overlap validation, and linked-profile no-I/O tests | No; covered by `0.25.0` | Not published; next `0.25.0` |
| 0.25.0 | BMP deterministic uncompressed encoders | Explicit output-dialect policy, exact headers/masks/palettes/padding, determinism, and round trips | Yes (cumulative checkpoint) | Publish `0.25.0` |
| 0.25.1 | BMP deterministic RLE4/RLE8 encoders | Escape/padding/delta policy, deterministic packets, bounded work, and decode/encode round trips | No; covered by `0.30.0` | Not published; next `0.30.0` |
| 0.25.2 | Complete declared BMP dialect audit | Exhaustive envelope/header/depth/palette/mask/compression/orientation/profile matrix, OS/2 legacy decisions, embedded payload policy, and external review | No; covered by `0.30.0` | Not published; next `0.30.0` |
| 0.26.0 | QOI structural parse and bounded decoder | Magic, dimensions, channels, colorspace hint, pixel count, wraparound, end-marker, and trailing-data tests | No; covered by `0.30.0` | Not published; next `0.30.0` |
| 0.27.0 | QOI deterministic encoder | Reference-vector and encode/decode conformance | No; covered by `0.30.0` | Not published; next `0.30.0` |
| 0.28.0 | Bounded Netpbm tokenizer | Comment, whitespace, decimal overflow, token-length fuzzing | No; covered by `0.30.0` | Not published; next `0.30.0` |
| 0.29.0 | PBM P1/P4 decode/encode | Bit order, row padding, multi-image policy | No; covered by `0.30.0` | Not published; next `0.30.0` |
| 0.30.0 | PGM P2/P5 decode/encode | MAXVAL scaling, 8/16-bit, source-defined BT.709 transfer, linear/sRGB variant policy, and truncation | Yes (cumulative checkpoint) | Publish `0.30.0` |
| 0.31.0 | PPM P3/P6 decode/encode | Sample scaling, BT.709 primaries/transfer/range declaration, variant policy, token bombs, and binary boundaries | No; covered by `0.35.0` | Not published; next `0.35.0` |
| 0.32.0 | PAM P7, if the public claim is “Netpbm” | Tuple types, depth, linear opacity, header termination, unknown fields, and color/alpha declarations | No; covered by `0.35.0` | Not published; next `0.35.0` |
| 0.33.0 | Combined PNM/PAM stream and conformance audit | Concatenated images, official-tool differential tests, and explicit PFM exclusion | No; covered by `0.35.0` | Not published; next `0.35.0` |
| 0.34.0 | farbfeld decode and encode | Exact-size arithmetic, RGBA16-BE, alpha semantics | No; covered by `0.35.0` | Not published; next `0.35.0` |
| 0.35.0 | Simple-codec contract and security freeze | Cross-codec probe fuzzing, 32-bit memory tests, simple-codec contract freeze, and external delta review | Yes (cumulative checkpoint) | Publish `0.35.0` |
| 0.36.0 | PNG signature and bounded probing | signature, ambiguity, prefix ownership, and every-byte truncation | No; covered by `0.40.0` | Not published; next `0.40.0` |
| 0.36.1 | PNG chunk framing and CRC | length/type/data/CRC boundaries, overflow, and mutation fuzzing | No; covered by `0.40.0` | Not published; next `0.40.0` |
| 0.36.2 | PNG chunk-order state machine | critical/ancillary transition matrix and unknown-critical rejection | No; covered by `0.40.0` | Not published; next `0.40.0` |
| 0.37.0 | PNG IHDR and color-type/bit-depth validation | Full normative combination matrix | No; covered by `0.40.0` | Not published; next `0.40.0` |
| 0.38.0 | Bounded mynd-zlib wrapper | RFC 1950 header/dictionary/trailer rules, Adler-32, and truncation | No; covered by `0.40.0` | Not published; next `0.40.0` |
| 0.38.1 | Deflate stored blocks | RFC 1951 stored-block alignment/complement/output bounds and truncation | No; covered by `0.40.0` | Not published; next `0.40.0` |
| 0.38.2 | Deflate fixed-Huffman blocks | fixed tables, distance/overlap bounds, transactional bits, and fuzzing | No; covered by `0.40.0` | Not published; next `0.40.0` |
| 0.39.0 | Shared dynamic-Huffman and complete bounded mynd-deflate | Tree proofs, 32 KiB window, distance/overlap fuzzing, output bombs, and reusable crate audit | No; covered by `0.40.0` | Not published; next `0.40.0` |
| 0.40.0 | PNG row-filter reconstruction | all five filters at every admitted byte width with prior-row boundaries | Yes (cumulative checkpoint) | Publish `0.40.0` |
| 0.40.1 | PNG noninterlaced 8-bit core color decoding | grayscale, truecolor, grayscale-alpha, and truecolor-alpha golden vectors | No; covered by `0.45.0` | Not published; next `0.45.0` |
| 0.41.0 | Packed 1/2/4-bit and 16-bit PNG samples | Scaling, endian, tail-bit tests | No; covered by `0.45.0` | Not published; next `0.45.0` |
| 0.42.0 | Adam7 decode and progressive row events | Pass geometry proofs and tiny-image corpus | No; covered by `0.45.0` | Not published; next `0.45.0` |
| 0.43.0 | PNG PLTE and tRNS semantics | palette cardinality/index/transparency matrices and invalid combinations | No; covered by `0.45.0` | Not published; next `0.45.0` |
| 0.43.1 | PNG bKGD, hIST, sBIT, and sPLT chunks | per-chunk length/value/order matrices and bounded suggested palettes | No; covered by `0.45.0` | Not published; next `0.45.0` |
| 0.44.0 | PNG cHRM, gAMA, and sRGB declarations | PNG Third Edition value rules and declaration-precedence matrix | No; covered by `0.45.0` | Not published; next `0.45.0` |
| 0.44.1 | PNG iCCP transport and ICC precedence | profile-name/Deflate limits, ICC bombs, and conflict precedence | No; covered by `0.45.0` | Not published; next `0.45.0` |
| 0.44.2 | PNG cICP and HDR/WCG metadata | cICP, mDCV, and cLLI order/dependency/value rules with HDR vectors | No; covered by `0.45.0` | Not published; next `0.45.0` |
| 0.45.0 | PNG bounded text chunks | tEXt, zTXt, and iTXt keyword/language/compression/UTF-8 limits | Yes (cumulative checkpoint) | Publish `0.45.0` |
| 0.45.1 | PNG eXIf, pHYs, and tIME metadata | chunk-specific order, length, value, and metadata-transport tests | No; covered by `0.50.0` | Not published; next `0.50.0` |
| 0.45.2 | PNG unknown and private chunk policy | critical rejection, ancillary preservation, and safe-to-copy editor matrix | No; covered by `0.50.0` | Not published; next `0.50.0` |
| 0.46.0 | APNG control and frame-chunk sequencing | acTL/fcTL/fdAT order, sequence numbers, rectangles, timing, and limits | No; covered by `0.50.0` | Not published; next `0.50.0` |
| 0.46.1 | APNG frame decoding and composition | default-image cases, source/source-over, disposal, streaming, and bombs | No; covered by `0.50.0` | Not published; next `0.50.0` |
| 0.46.2 | PNG deterministic encoding | Third Edition emission, row filters, Deflate, metadata, and determinism | No; covered by `0.50.0` | Not published; next `0.50.0` |
| 0.46.3 | APNG deterministic encoding | frame sequencing, rectangles, timing, disposal/blend, and round trips | No; covered by `0.50.0` | Not published; next `0.50.0` |
| 0.46.4 | Complete PNG/APNG conformance and security audit | Third Edition mapping, conformance/differential corpus, and long fuzzing | No; covered by `0.50.0` | Not published; next `0.50.0` |
| 0.47.0 | GIF87a/89a structure, palettes, sub-blocks, descriptors | Logical-screen fields, color resolution/sort/background/aspect policy, block termination, and palette bounds | No; covered by `0.50.0` | Not published; next `0.50.0` |
| 0.48.0 | GIF LZW | Dictionary/code-width proofs and fuzzing | No; covered by `0.50.0` | Not published; next `0.50.0` |
| 0.49.0 | GIF single-frame decode and deinterlace | Exact pixels and four-pass geometry tests | No; covered by `0.50.0` | Not published; next `0.50.0` |
| 0.50.0 | GIF GCE, transparency, frame composition, all disposal modes | Snapshot caps and animation bomb corpus | Yes (cumulative checkpoint) | Publish `0.50.0` |
| 0.51.0 | GIF named-extension parsing | Comment, Plain Text, Application, and unknown extension boundaries | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.51.1 | GIF compatibility and termination policy | Netscape loop, EOI/trailer, extra pixels, delay, and disposal decisions | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.51.2 | GIF raw-frame and composited-frame APIs | coordinates, disposal sequencing, valid prefixes, and frame ranges | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.51.3 | GIF exact palettes and bounded histogram | unique-color limits, entry layout, ordering, overflow, and caller palettes | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.51.4 | GIF deterministic palette generation and remapping | median-cut policy, remap, bounded dithering, budgets, and goldens | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.51.5 | GIF LZW encoder | dictionary growth/reset/saturation, widths, end code, proofs, and round trips | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.51.6 | Single-frame GIF encoder | palette/table/transparency/sub-block integration and deterministic output | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.51.7 | Animated GIF encoder | canvas/frame limits, timing, loop, disposal, ranges, and round trips | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.51.8 | Complete GIF conformance and security audit | normative/de-facto matrix, differential corpus, fuzzing, and animation bombs | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.52.0 | JPEG marker and segment framing | standalone/length-bearing marker boundaries and size mutation fuzzing | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.52.1 | JPEG quantization and entropy-table declarations | DQT/DHT/DAC/DRI types, precision, counts, redefinition, and dependencies | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.52.2 | JPEG frame declarations | SOF process/precision/components/sampling/dimensions and DNL policy | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.52.3 | JPEG scan declarations and ordering | SOS selectors/ranges, multiscan/abbreviated-table state, and invalid transitions | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.53.0 | JPEG Huffman entropy and byte stuffing | canonical table proofs, marker boundaries, and every-bit truncation | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.53.1 | JPEG restart and bounded MCU coefficient accounting | restart reset/sequence, MCU geometry, coefficient bounds, and work budgets | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.54.0 | JPEG scalar IDCT and grayscale reconstruction | IDCT coefficient bounds, normative tolerance, grayscale blocks, restart corpus, and deterministic scalar vectors | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.54.1 | JPEG component sampling and bounded upsampling | Sampling factors, MCU geometry, edge extension, upsampling policy, limits, and reference vectors | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.54.2 | JPEG native YCbCr and rendered RGB output tiers | Native plane/coefficients versus shared-color rendering, JFIF declarations, tolerances, and error separation | No; covered by `0.55.0` | Not published; next `0.55.0` |
| 0.55.0 | Extended sequential and 12-bit DCT processes | Precision and coefficient-range evidence | Yes (cumulative checkpoint) | Publish `0.55.0` |
| 0.56.0 | JPEG progressive DC scans | first/refinement DC scan state, predictors, restart, and malformed order | No; covered by `0.60.0` | Not published; next `0.60.0` |
| 0.56.1 | JPEG progressive AC scans | spectral selection, EOB runs, coefficient bounds, and restart behavior | No; covered by `0.60.0` | Not published; next `0.60.0` |
| 0.56.2 | JPEG successive-approximation integration audit | complete scan-script matrix, native coefficients, work limits, and fuzzing | No; covered by `0.60.0` | Not published; next `0.60.0` |
| 0.57.0 | Lossless predictive JPEG process | Predictor, point transform, precision tests | No; covered by `0.60.0` | Not published; next `0.60.0` |
| 0.58.0 | JPEG arithmetic coding | Conditioning-table and arithmetic-state proofs | No; covered by `0.60.0` | Not published; next `0.60.0` |
| 0.59.0 | JPEG differential processes | Reference-frame dependencies, differential scan state, reconstruction bounds, and malformed graphs | No; covered by `0.60.0` | Not published; next `0.60.0` |
| 0.59.1 | JPEG hierarchical processes | Frame hierarchy, expansion, dependencies, reconstruction limits, native output, and differential evidence | No; covered by `0.60.0` | Not published; next `0.60.0` |
| 0.60.0 | JPEG JFIF and Adobe color declarations | T.871 APP0 plus Adobe RGB/CMYK/YCCK interpretation and precedence | Yes (cumulative checkpoint) | Publish `0.60.0` |
| 0.60.1 | JPEG Exif APP1 transport | identifier, length, nested offset, duplicate, and preservation policies | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.60.2 | JPEG ICC APP2 assembly and color precedence | chunk numbering/completeness/duplicates, profile limits, and color vectors | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.60.3 | JPEG COM and registered/unknown APPn policy | T.86 registry mapping, SPIFF decision, bounded COM/APPn preservation, duplicates, and inert metadata | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.61.0 | Baseline JPEG encoder | Deterministic valid baseline emission, quality controls, coefficient limits, and round trips | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.61.1 | Progressive JPEG encoder | Scan scripts, successive approximation, deterministic tables, restart policy, and round trips | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.61.2 | Extended-sequential JPEG encoder | precision/process-valid emission, tables, restart, and differential tests | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.61.3 | Lossless JPEG encoder | predictor/point-transform emission, precision, restart, and round trips | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.61.4 | Arithmetic JPEG encoder admission | independent arithmetic evidence or an explicit unclaimed decision | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.61.5 | Differential JPEG encoder admission | independent differential-process evidence or an explicit unclaimed decision | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.61.6 | Hierarchical JPEG encoder admission | independent hierarchy evidence or an explicit unclaimed decision | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.62.0 | Complete declared T.81 conformance and security audit | Reference software, official material, long fuzz campaign | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.63.0 | WebP RIFF framing and simple-file dispatch | RIFF/WEBP size, padding, VP8/VP8L simple payload, and trailing policy | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.63.1 | WebP VP8X feature and chunk-order state machine | feature bits, canvas, chunk multiplicity/order, and invalid combinations | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.63.2 | WebP ICCP, EXIF, XMP, and unknown chunks | bounded metadata, feature consistency, preservation, and unknown policy | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.64.0 | VP8 Boolean-decoder primitive | range/value normalization, refill, termination, and arithmetic-state fuzzing | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.64.1 | VP8 partition and frame-header parsing | partition sizes/counts, key/inter headers, segmentation, and bounds | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.64.2 | VP8 probability-update and token state | coefficient/mode probability updates, defaults, reset, and work accounting | No; covered by `0.65.0` | Not published; next `0.65.0` |
| 0.65.0 | VP8 prediction and coefficient reconstruction | Macroblock/reference bounds, prediction modes, token reconstruction, partition limits, and scalar differential tests | Yes (cumulative checkpoint) | Publish `0.65.0` |
| 0.65.1 | VP8 inverse transforms and reconstructed macroblocks | Transform arithmetic, coefficient ranges, clipping, prediction integration, and scalar reference vectors | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.65.2 | VP8 loop filtering and complete still reconstruction | Filter levels/edges, macroblock bounds, native YCbCr output, rendered output, and differential corpus | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.66.0 | WebP ALPH decoding | ALPH filter/compression/preprocessing modes, dimensions, and bombs | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.66.1 | VP8 native YCbCr and rendered RGB integration | matrix/range assumptions, alpha association, output tiers, and color vectors | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.67.0 | VP8L prefix-code parsing and decoding | simple/normal code validation, tables, symbols, and every-bit truncation | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.67.1 | VP8L LZ77 distance and copy engine | distance mapping, overlap, history/output bounds, progress, and bombs | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.67.2 | VP8L color-cache engine | cache-bit bounds, hashing, initialization, access, and deterministic vectors | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.68.0 | VP8L transform declarations and meta-prefix images | transform order/count, dimensions, recursion, prefix images, and limits | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.68.1 | VP8L predictor transform | all predictor modes, edge rules, modular arithmetic, and goldens | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.68.2 | VP8L color transform | transform-image geometry, delta arithmetic, bounds, and golden vectors | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.68.3 | VP8L subtract-green transform | modular channel arithmetic, order, and exact pixel vectors | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.68.4 | VP8L color-indexing transform | palette image, packing widths, dimension reduction, indexes, and bounds | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.68.5 | VP8L complete lossless reconstruction audit | all transforms plus prefix/LZ/cache integration, exact pixels, bombs, and fuzzing | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.69.0 | WebP animation decoding | ANIM background/loop plus ANMF rectangles, duration, blend/dispose, frame limits, and animation fuzzing | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.69.1 | VP8L deterministic encoder | Prefix/LZ/cache/transform validity, quality-effort controls, bounded search, determinism, and round trips | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.69.2 | VP8 deterministic encoder | Prediction/partition/token validity, quality-effort controls, bounded heuristics, backend determinism, and differential tests | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.69.3 | Animated WebP encoder | ANMF ordering/rectangles, mixed frame modes, blend/dispose, metadata, and round trips | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.69.4 | Complete WebP conformance and security audit | RFC/VP8/VP8L mappings, still/animated split, ALPH/metadata, encoder modes, long fuzzing, and external review | No; covered by `0.70.0` | Not published; next `0.70.0` |
| 0.70.0 | Shared bounded mynd-ifd graph and typed-value engine | count/offset arithmetic, cycles, overlaps, typed values, and fuzzing | Yes (cumulative checkpoint) | Publish `0.70.0` |
| 0.70.1 | TIFF 6.0 tag schema and dependency validation | required/defaulted tags, types/counts, duplicate policy, and dependencies | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.71.0 | TIFF baseline uncompressed strips | bilevel/Gray/palette/RGB strip geometry, FillOrder, and truncation | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.71.1 | TIFF PackBits strip decoding | packet boundaries, no-op bytes, row/strip output bounds, and bombs | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.72.0 | TIFF LZW decoding | TIFF code-width dialect, clear/end codes, dictionary limits, and bombs | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.72.1 | TIFF Deflate decoding | old/new compression-tag policy, zlib integration, and output limits | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.72.2 | TIFF horizontal Predictor 2 | integer sample widths, planar/contiguous rows, endian, and overflow | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.72.3 | TIFF floating-point Predictor 3 profile | Adobe Technical Note 3 byte reordering/differencing vectors and profile matrix | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.73.0 | TIFF CCITT modified-Huffman RLE | run tables, FillOrder, EOL policy, row bounds, and differential corpus | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.73.1 | TIFF CCITT Group 3 fax decoding | T4 options, 1D/2D transitions, EOL/RTC, damaged rows, and limits | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.73.2 | TIFF CCITT Group 4 fax decoding | T6 transitions, EOFB, reference-line bounds, and malformed streams | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.74.0 | TIFF tiled image layout | tile geometry, edge tiles, sparse/overlap policy, offsets, and byte counts | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.74.1 | TIFF planar image layout | plane ordering, per-plane strips/tiles, sample dependencies, and limits | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.74.2 | TIFF multipage and SubIFD traversal | next-IFD/SubIFD graphs, cycles, aggregate page limits, and valid prefixes | No; covered by `0.75.0` | Not published; next `0.75.0` |
| 0.75.0 | TIFF YCbCr samples and tag dependencies | Coefficients, reference black/white, subsampling, positioning, strip/tile geometry, and color vectors | Yes (cumulative checkpoint) | Publish `0.75.0` |
| 0.75.1 | TIFF CMYK and CIELab native samples | Photometric dependencies, signed/sample domains, planar layouts, declarations, and native golden vectors | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.75.2 | TIFF alpha, ICC, and rendered-color integration | ExtraSamples association, ICC precedence, shared rendering, and tolerances | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.75.3 | TIFF signed-integer and IEEE floating sample domains | SampleFormat/depth combinations, endian, NaN/infinity policy, and native output | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.75.4 | TIFF Orientation presentation policy | all eight values, native coordinates, opt-in normalization, region mapping, and metadata effects | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.75.5 | TIFF calibrated color declarations | WhitePoint, PrimaryChromaticities, TransferFunction, ReferenceBlackWhite, precedence, and unsupported combinations | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.76.0 | Corrected JPEG-in-TIFF decoding | old/new JPEG distinction, table ownership, strip/tile boundaries, and corpus | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.76.1 | TIFF Exif IFD integration | Exif/GPS/Interop graph namespaces, nested offsets, cycles, and limits | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.76.2 | TIFF admitted-extension profile freeze | each extension has pinned provenance and an explicit support disposition | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.0 | TIFF baseline uncompressed-strip encoder | endian tags/strips/exact sizes/determinism and round trips | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.1 | TIFF PackBits encoder | packet/run/row boundaries, exact lengths, determinism, and round trips | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.2 | TIFF LZW encoder | dialect widths, clear/end behavior, proofs, determinism, and round trips | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.3 | TIFF Deflate encoder | mynd-deflate/zlib integration, output limits, validity, and round trips | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.4 | TIFF horizontal Predictor 2 encoder | integer widths, endian, planar/contiguous rows, and round trips | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.5 | TIFF floating-point Predictor 3 encoder admission | Adobe-profile widths and vectors or an explicit unclaimed decision | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.6 | TIFF CCITT RLE encoder | run-code validity, FillOrder, row termination, and differential tests | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.7 | TIFF CCITT Group 3 encoder | T4 options, 1D/2D transitions, EOL/RTC, and differential tests | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.8 | TIFF CCITT Group 4 encoder | T6 transitions, EOFB, reference rows, and differential tests | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.9 | TIFF tiled-image encoder | tile geometry, edge tiles, offsets/byte counts, and deterministic output | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.10 | TIFF planar-image encoder | plane ordering, per-plane storage, dependencies, and round trips | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.11 | TIFF multipage and SubIFD encoder | IFD graph, next/SubIFD links, aggregate limits, and deterministic output | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.12 | TIFF extended sample/color encoder | photometric/SampleFormat/ExtraSamples/YCbCr/ICC dependencies and claims | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.13 | Corrected JPEG-in-TIFF encoder | new-style table ownership, strip/tile boundaries, validity, and round trips | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.77.14 | Complete declared TIFF profile audit | compression/layout/sample/color/extension matrix, conformance, fuzzing, and review | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.78.0 | Cross-format native-sample and color-declaration integration | decode_native consistency, declaration precedence, preserved profiles, and no premature rendered-color claim | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.79.0 | Shared bounded TIFF/Exif IFD inspection | Offset graphs, entry counts, cycles, value bounds, MakerNote opacity, and fuzzing | No; covered by `0.80.0` | Not published; next `0.80.0` |
| 0.80.0 | Selected bounded Exif field interpretation | dimensions, timestamps, strings, types/counts, encoding, and conflicts | Yes (cumulative checkpoint) | Publish `0.80.0` |
| 0.80.1 | Bounded Exif thumbnail extraction | thumbnail offsets/lengths, nested format limits, overlap, and bomb resistance | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.80.2 | Explicit Exif orientation policy | all eight orientations, coordinate mapping, opt-in transform, and metadata effect | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.80.3 | Exif 3.1 and Exif-for-XMP profile freeze | DC-008/DC-010 2026 mappings, selected/opaque/unsupported fields, container consistency, and differential audit | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.81.0 | XMP packet framing and bounded raw transport | xpacket boundaries, encoding, padding, read-only flag, exact preservation, truncation, and no external I/O | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.81.1 | Bounded XML 1.0 and Namespaces profile for XMP | token/nesting/name/attribute/text limits, encoding rules, disabled DTD/entities/resolvers, and malicious XML corpus | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.81.2 | Bounded RDF/XML and XMP data-model inspection | RDF productions, arrays/structures/qualifiers, namespaces, aliases, duplicate policy, and graph budgets | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.81.3 | XMP and legacy-metadata conflict/rewrite policy | XMP Part 3 plus CIPA DC-010 reconciliation, preserve/discard/rewrite, duplicates, and round trips | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.81.4 | Transformation-aware metadata effect planning | preserved/rewritten/invalidated/decision results for every operation | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.82.0 | YCbCr matrices, ranges, subsampling, and chroma siting | JPEG/WebP/TIFF reference vectors | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.83.0 | Gray, CMYK, and YCCK conversion | black-generation/Adobe policy, native-to-rendered vectors, and gamut limits | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.83.1 | CIELab and declared wide-gamut conversion | white-point/adaptation/range rules, out-of-gamut policy, and reference vectors | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.84.0 | Straight/premultiplied alpha conversion | Zero-alpha, rounding, and invariant tests | No; covered by `0.85.0` | Not published; next `0.85.0` |
| 0.85.0 | Explicit color-conversion planning | information-loss declaration, numeric tier, stages, scratch, and work plan | Yes (cumulative checkpoint) | Publish `0.85.0` |
| 0.85.1 | Sample-depth conversion | all admitted integer/float depths, scaling, rounding, saturation, and alpha | No; covered by `0.90.0` | Not published; next `0.90.0` |
| 0.85.2 | Deterministic advanced dithering | ordered/error-diffusion kernels, edge/error bounds, budgets, and goldens | No; covered by `0.90.0` | Not published; next `0.90.0` |
| 0.85.3 | Final cross-format rendered-color conformance audit | PNG/JPEG/WebP/TIFF profiles, precedence, tolerances, and differential results | No; covered by `0.90.0` | Not published; next `0.90.0` |
| 0.86.0 | Crop, flip, rotate, transpose | In-place overlap and rectangle proofs | No; covered by `0.90.0` | Not published; next `0.90.0` |
| 0.87.0 | Checked affine geometry and border modes | Finite-matrix and coordinate-overflow proofs | No; covered by `0.90.0` | Not published; next `0.90.0` |
| 0.88.0 | Nearest and bilinear resampling | Pixel-center and edge-policy golden tests | No; covered by `0.90.0` | Not published; next `0.90.0` |
| 0.89.0 | Bicubic resampling | Coefficient normalization and overshoot policy | No; covered by `0.90.0` | Not published; next `0.90.0` |
| 0.90.0 | Lanczos3 resampling | Tap planning, ring-buffer limits, reference vectors | Yes (cumulative checkpoint) | Publish `0.90.0` |
| 0.91.0 | Remaining Porter-Duff compositing operators | Shared source/source-over compatibility, remaining operators, linear domain, alpha, overlap, and invariants | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.92.0 | Declared artistic blend modes | formula/domain/clamping/NaN/alpha matrices and interoperability vectors | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.92.1 | Clipped pixel, span, and fill primitives | signed/overflowing coordinates, clipping, layout, alpha, and work budgets | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.92.2 | Rectangle and overlap-safe blit primitives | empty/degenerate rectangles, clipping, overlap directions, and alias policy | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.92.3 | Deterministic integer line primitives | octants, endpoints, degenerates, clipping symmetry, and golden rasters | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.92.4 | Deterministic integer circle and ellipse primitives | quadrants, degenerates, clipping symmetry, overflow, and golden rasters | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.92.5 | Bounded raster-drawing contract and security audit | layout/color/alpha/overlap/work tests and explicit support matrix | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.93.0 | Optional safe SIMD or audited external backends | Scalar differential, tail/alignment, dependency/unsafe-boundary, Miri, and sanitizer evidence | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.94.0 | Streaming and tiled processing graph | Scratch bounds, fusion equivalence, cancellation, and honest random-access disclosure | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.94.1 | Metadata- and header-only decoding | no-pixel paths, source-position policy, metadata budgets, and format matrix | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.94.2 | Region-selective decoding | coordinate spaces, chroma/halo planning, committed regions, and fallbacks | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.94.3 | Frame-range selective decoding | raw/composited selection, dependency closure, disposal state, and bombs | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.94.4 | Reduced-resolution and progressive-preview decoding | JPEG reduced IDCT, TIFF selection, progressive events, and numeric evidence | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.94.5 | Processing and selective-decoding contract freeze | fusion equivalence, peak limits, cancellation, DoS, and support matrix | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.94.6 | Unified borrowed inspection and decode_into facade | hints/mismatch, static dispatch, output tiers, limits, and disabled features | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.94.7 | Unified encoder and transcoding facade | capability plans, conversion orchestration, metadata effects, and transactions | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.94.8 | Fallible owned APIs and facade-candidate integration audit | allocation failure, representative codecs, feature matrix, and candidate baseline | No; covered by `0.95.0` | Not published; next `0.95.0` |
| 0.95.0 | Runtime-neutral async source/sink adapters | Backpressure, cancellation, partial-I/O tests | Yes (cumulative checkpoint) | Publish `0.95.0` |
| 0.95.1 | WASM/browser streaming adapters | wasm32-unknown-unknown, JS-size, memory-growth tests | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.96.0 | Caller-provided parallel scheduling interface | Determinism, budget partition, cancellation | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.96.1 | Optional Rayon/service adapter | No core dependency or automatic global pool | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.97.0 | GPU-compatible descriptors and upload-layout hooks | Stable layout contract; no device ownership in core | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.97.1 | Optional backend adapters | CPU/GPU differential results and synchronization policy | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.98.0 | mynd-cli inspect command | escaped bounded metadata, stable schema, hostile terminals, and exit codes | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.98.1 | mynd-cli validate command | strict/compatibility modes, bounded diagnostics, exit codes, and no output mutation | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.98.2 | mynd-cli decode command | transactional files, output-tier/color disclosure, limits, and cancellation | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.98.3 | mynd-cli encode command | format capability validation, metadata policy, transactions, and determinism | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.98.4 | mynd-cli convert command | explicit conversion plan, information-loss confirmation, and atomic replacement | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.98.5 | mynd-cli frame command | raw/composited selection, frame/range limits, filenames, and animation bombs | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.98.6 | mynd-cli bounded batch profile | aggregate budgets, hostile filenames, collision policy, and cancellation | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.98.7 | mynd-cli bounded service profile | request isolation, aggregate/live budgets, cancellation, and no ambient authority | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.98.8 | Cross-adapter facade reconciliation | Zero unresolved implementation or public-API issues and one exact assurance input | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.0 | cargo-fuzz harness and corpus integration | every parser/entropy/metadata/dispatcher target builds with provenance | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.1 | Long-running fuzz and truncation campaign | coverage report, minimized persistent corpus, and no stalls/panics | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.2 | Kani checked-arithmetic and geometry proofs | conversion/size/stride/rectangle assumptions and unwind bounds | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.3 | Kani view and buffer-state proofs | bounds, alias policy, commit visibility, and state invariants | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.4 | Kani byte- and bit-I/O proofs | cursor/refill/shift/rollback/progress invariants | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.5 | Kani Deflate and zlib state proofs | table/distance/window/checksum/output/progress invariants | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.6 | Kani GIF and TIFF LZW state proofs | dialect dictionary/width/reset/end/output/progress invariants | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.7 | Kani JPEG entropy state proofs | Huffman/arithmetic/stuffing/restart/coefficient/progress invariants | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.8 | Kani WebP entropy state proofs | VP8 Boolean and VP8L prefix/LZ/cache/output/progress invariants | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.9 | Kani TIFF fax and IFD state proofs | run/transition/reference-row and graph/count/progress invariants | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.10 | Miri audit | all supported feature sets and mutation/view/adapter paths pass Miri | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.11 | Sanitizer audit | address, leak, memory, and undefined-behavior sanitizer matrix | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.12 | Supported-Rust, target, and feature audit | Rust 1.90.0-1.97.1, targets, no-default, alloc/std, WASM, and combinations | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.13 | Stack and code-size audit | per-target stack ceilings, recursion absence, and binary-size budgets | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.14 | Official conformance and differential freeze | every format claim mapped with no unexplained reference disagreement | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.15 | Cross-format color conformance freeze | native/rendered profiles, precedence, tolerances, and reference vectors | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.16 | Performance and denial-of-service freeze | valid throughput plus hostile rejection-time/work/memory regression limits | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.17 | Reproducible package, SBOM, and provenance freeze | byte-reproducible archives, dependency identity, signatures, and attestations | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |
| 0.99.18 | Cumulative pentest handoff and final public API freeze | unchanged v0.98.8 input, complete evidence bundle, frozen scope, and no unresolved implementation defect | No; covered by `1.0.0-rc.1` | Not published; next `1.0.0-rc.1` |

## Phase: Foundations

Establish explicit sample/color/alpha primitives, source/sink negotiation, sticky incremental lifecycle, typed scratch, budgets, shared Deflate/ICC, and candidate-reviewed contracts.

### v0.1.0 - Existing workspace, licenses, feature boundaries, release policy

Status: Released 2026-07-25.

Context:

This is the exclusive foundations handoff for
existing workspace, licenses, feature boundaries, release policy. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete existing workspace, licenses, feature boundaries, release policy with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Existing workspace, licenses, feature boundaries, release policy.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Current checks plus completed pentest cycle.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.1.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.2.0 - Unified scope, claim taxonomy, standards/errata ledger, corpus provenance schema

Status: Released 2026-07-25.

Context:

This is the exclusive foundations handoff for
unified scope, claim taxonomy, standards/errata ledger, corpus provenance schema. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete unified scope, claim taxonomy, standards/errata ledger, corpus provenance schema with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Unified scope, claim taxonomy, standards/errata ledger, corpus provenance schema.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- This is blocking: no implementation handoff begins until every old-scope document and crate graph agrees with the normative roadmap.
- The reconciled image list and pre-1.0 implementation plan must name farbfeld,
  its dedicated codec boundary, original specification source, and v0.34.0
  decode/encode handoff explicitly.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: No contradictions across README, support matrix, and normative plans.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.2.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.2.1 - Reproducible specification corpus and legal-disposition gate

Status: Released 2026-07-27.

Context:

This governance handoff turns the source ledger into a reproducible,
fail-closed corpus before implementation relies on any normative text. It adds
no codec behavior.

Goal:

Make every normative, supplemental, errata, and conformance source available
through a legally reviewed tracked, offline, or manual acquisition path.

Deliverables:

- Classify every source as public, offline, or manual with publisher, exact
  edition, role, acquisition URL, terms, filename, size ceiling, and hash.
- Track only unmodified sources with explicit redistribution permission; keep
  restricted and unclear material ignored and never claim ownership.
- Provide allow-listed HTTPS fetch, candidate-lock, verification, read-only
  mode, exact-file-set, symlink, atomic-replacement, and package-exclusion gates.
- Record every normative dependency discovered inside an admitted source,
  including errata, XML/RDF dependencies, TIFF fax baselines, and patent-bearing
  or purchased references.
- Prove a clean checkout can recreate every automatic offline source while
  manual acquisition never accepts terms, uses credentials, or bypasses payment.
- Update source policy, release notes, SBOM inputs, and the cadence-appropriate release-evidence record.

Verification:

- Rebuild public and offline corpora from an empty destination, verify all
  hashes and modes, mutate one byte and one manifest field, and require closed
  failure without replacing the reviewed lock.
- Verify public terms and attribution records, review redirects and host
  allowlists, and confirm crate packages contain no specification documents.

Exit criteria:

- Every pre-1.0 requirement maps to a pinned source or an explicit blocked
  manual record; no implementation may begin with an unresolved normative
  dependency.
- Legal review approves every public copy and confirms offline/manual material
  is ignored; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, at a publication checkpoint the permanent report records PASS, and
  the exact-version release gate accepts the reviewed corpus.
- `v0.2.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.3.0 - Checked conversion/add/multiply/align/range primitives

Status: Released 2026-08-03.

Context:

This is the exclusive foundations handoff for
checked conversion/add/multiply/align/range primitives. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete checked conversion/add/multiply/align/range primitives with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Checked conversion/add/multiply/align/range primitives.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Exhaustive extrema tests and Kani arithmetic proofs.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.3.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.4.0 - Validated dimensions, rectangles, strides, planes, and output lengths

Status: Release candidate; pentest PASS; awaiting green GitHub CI and CodeQL.

Context:

This is the exclusive foundations handoff for
validated dimensions, rectangles, strides, planes, and output lengths. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete validated dimensions, rectangles, strides, planes, and output lengths with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Validated dimensions, rectangles, strides, planes, and output lengths.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Zero/min/max, last-row, alignment, and 32-bit usize proofs.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.4.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.5.0 - Explicit pixel layout and sample-storage domains

Status: Planned.

Context:

This is the exclusive foundations handoff for
explicit pixel layout and sample-storage domains. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete explicit pixel layout and sample-storage domains with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Explicit pixel layout and sample-storage domains.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Invalid layout/sample/plane/chroma/alpha combinations are unrepresentable.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.5.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.5.1 - Numeric determinism and floating-sample contract

Status: Planned.

Context:

This is the exclusive foundations handoff for
numeric determinism and floating-sample contract. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete numeric determinism and floating-sample contract with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Numeric determinism and floating-sample contract.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Bit-exact/tolerance/backend tiers plus rounding, saturation, FMA, NaN, infinity, zero, and subnormal tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.5.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.5.2 - Shared color and blending specification ledger

Status: Planned.

Context:

This is the exclusive foundations handoff for
shared color and blending specification ledger. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete shared color and blending specification ledger with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Shared color and blending specification ledger.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Pinned ICC, sRGB, BT.601/709/2020, H.273, HDR, CIE, Porter-Duff, and blend sources with claim scope.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.5.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.5.3 - Scalar transfer, matrix/range, and alpha foundation

Status: Planned.

Context:

This is the exclusive foundations handoff for
scalar transfer, matrix/range, and alpha foundation. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete scalar transfer, matrix/range, and alpha foundation with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Scalar transfer, matrix/range, and alpha foundation.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Integer/scalar reference vectors, premultiplication, range conversion, rounding, and native-sample preservation.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.5.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.5.4 - Minimal scalar animation composition kernel

Status: Planned.

Context:

This is the exclusive foundations handoff for
minimal scalar animation composition kernel. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete minimal scalar animation composition kernel with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Minimal scalar animation composition kernel.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- APNG and WebP must reuse this source/source-over kernel; GIF maps its skip/replace semantics without duplicating alpha math.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Source/source-over, straight/premultiplied conversion, exact integer rounding, zero alpha, and format mapping.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.5.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.5.5 - Explicit no_std elementary-math backend

Status: Planned.

Context:

This is the exclusive foundations handoff for explicit no_std elementary-math backend. Its API
and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release
cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete explicit no_std elementary-math backend with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Explicit no_std elementary-math backend.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Specify powers, roots, and every elementary operation needed by ICC curves, transfer functions, Lab, HDR, and resampling.
- Define domain reduction, exceptional inputs, maximum error, rounding, FMA use, and reproducible backend selection on WASM and targets without hardware floating point.
- Offer a deterministic software backend where supported; selecting another backend changes the recorded conformance evidence explicitly.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Pure-core/software/optional backend selection, domains, maximum error, FMA policy, and cross-target vectors.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.5.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.6.0 - Immutable/mutable image and plane views

Status: Planned.

Context:

This is the exclusive foundations handoff for
immutable/mutable image and plane views. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete immutable/mutable image and plane views with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Immutable/mutable image and plane views.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Short-buffer, row-boundary, alias-policy, and Miri tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.6.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.7.0 - Frames, timing, canvas, disposal, blend, and frame rectangles

Status: Planned.

Context:

This is the exclusive foundations handoff for
frames, timing, canvas, disposal, blend, and frame rectangles. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete frames, timing, canvas, disposal, blend, and frame rectangles with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Frames, timing, canvas, disposal, blend, and frame rectangles.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Off-canvas and cumulative-duration tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.7.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.8.0 - Allocation-free structured errors, warnings, reports, and offsets

Status: Planned.

Context:

This is the exclusive foundations handoff for
allocation-free structured errors, warnings, reports, and offsets. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete allocation-free structured errors, warnings, reports, and offsets with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Allocation-free structured errors, warnings, reports, and offsets.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Bounded formatting and terminal/log-injection tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.8.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.9.0 - DecodeLimits, EncodeLimits, monotonic work/memory ledger

Status: Planned.

Context:

This is the exclusive foundations handoff for
decodelimits, encodelimits, monotonic work/memory ledger. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete decodelimits, encodelimits, monotonic work/memory ledger with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: DecodeLimits, EncodeLimits, monotonic work/memory ledger.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Budget-sharing and bypass property tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.9.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.10.0 - Caller-owned scratch planner, arenas, buffer pools, and leases

Status: Planned.

Context:

This is the exclusive foundations handoff for
caller-owned scratch planner, arenas, buffer pools, and leases. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete caller-owned scratch planner, arenas, buffer pools, and leases with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Caller-owned scratch planner, arenas, buffer pools, and leases.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Peak-memory accounting and failed-reservation tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.10.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.11.0 - Slice reader/writer, exact reads, fixed output, checkpoints

Status: Planned.

Context:

This is the exclusive foundations handoff for
slice reader/writer, exact reads, fixed output, checkpoints. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete slice reader/writer, exact reads, fixed output, checkpoints with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Slice reader/writer, exact reads, fixed output, checkpoints.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Every-byte truncation and rollback tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.11.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.12.0 - Endian I/O, subranges, counting, seek/read-at

Status: Planned.

Context:

This is the exclusive foundations handoff for
endian i/o, subranges, counting, seek/read-at. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete endian i/o, subranges, counting, seek/read-at with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Endian I/O, subranges, counting, seek/read-at.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Nested-bound escape, offset, and seek-cycle tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.12.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.12.1 - Source/sink capability negotiation and execution lifecycle

Status: Planned.

Context:

This is the exclusive foundations handoff for
source/sink capability negotiation and execution lifecycle. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete source/sink capability negotiation and execution lifecycle with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Source/sink capability negotiation and execution lifecycle.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Planning returns RequiresCapability before binding workspace or modifying output.
- Lifecycle is Probe -> InspectHeader -> Plan -> BindWorkspace -> Execute, then
  sticky Done, Cancelled, or Error. Reset restores neither source position,
  destination contents, nor cumulative budget; indefinite streams refine plans
  only inside original limits.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Capability planning plus sticky Done/Cancelled/Error and non-restoring Reset tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.12.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.12.2 - General encoder sizing and commit planning

Status: Planned.

Context:

This is the exclusive foundations handoff for general encoder sizing and commit planning. Its API
and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release
cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete general encoder sizing and commit planning with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: General encoder sizing and commit planning.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Define independent LengthKnowledge, EmissionStrategy, EncodeInputCapabilities, SinkCapabilities, ScratchRequirements, and CommitPolicy fields.
- Require explicit maximum memory for every buffered region; a forward-only source cannot satisfy CountThenEmit without replayability.
- Bind counting/emission to identical configuration, metadata policy, source pixels, and numeric backend while charging both passes to input/work budgets.
- Report unavailable input or sink capabilities before output mutation and define commit boundaries plus recoverability for forward-only sinks.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Compositional length/strategy/input/sink/scratch/commit fields, replayability, two-pass identity, and pre-mutation failure.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.12.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.12.3 - Source-bound decode session planning

Status: Planned.

Context:

This is the exclusive foundations handoff for source-bound decode session planning. Its API
and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release
cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete source-bound decode session planning with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Source-bound decode session planning.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Distinguish reusable format-neutral LayoutPlan from source-bound DecodeSessionPlan.
- Bind with retained typestate, exact critical bytes, an unforgeable session-generation token, or an intentionally admitted collision-resistant digest; ordinary hashes are insufficient.
- Cover codec/profile declarations, source epoch/prefix, limits, compatibility, native/rendered choice, output layout/color, numeric backend, metadata policy, and commit mode.
- Require a stable-snapshot promise for consistent mutable-source rereads, retain critical declarations, and revalidate before output while continuing all parser checks.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Exact-session/header/token/digest identity, stable-snapshot policy, complete configuration binding, and pre-output revalidation.
- Audit arithmetic, source/session binding, offsets, terminal transitions,
  capability negotiation, committed units, cumulative/live/peak budgets,
  scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, binding, commit mode,
  limitations, numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.12.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.13.0 - MSB/LSB bit readers and writers

Status: Planned.

Context:

This is the exclusive foundations handoff for
msb/lsb bit readers and writers. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete msb/lsb bit readers and writers with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: MSB/LSB bit readers and writers.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Every-bit truncation, width, refill, and shift proofs.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.13.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.14.0 - Incremental decoder/encoder progress contracts

Status: Planned.

Context:

This is the exclusive foundations handoff for
incremental decoder/encoder progress contracts. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete incremental decoder/encoder progress contracts with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Incremental decoder/encoder progress contracts.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Fix every incremental decoder and encoder trait to one concrete
  `DecodeStepReport<'a>` or `EncodeStepReport<'a>` alias over
  `StepReport<StepState, CommitSet<'a>>`; every call reports exact state,
  input_consumed, output_produced, work_consumed, and committed units.
- Declare `StepState` and `CommitSet<'a>` non-exhaustive and include at v0.14.0
  all eventual Progress, NeedInput, NeedOutput, Yielded, Done, Cancelled,
  LimitExceeded, Error, no-commit, byte, row, frame, and disjoint-region
  categories. Later milestones enable behavior without adding categories or
  changing the trait return type.
- Give every borrowed commit payload an explicit lifetime. Region-token storage
  is caller-provided and plan-sized; reserve a slot before region mutation, and
  report exact `CommitTokens` capacity exhaustion without allocating, hidden
  growth, losing existing tokens, or committing unreportable pixels.
- Done, Cancelled, and Error are sticky; post-terminal step is deterministic, Reset clears local state without refunding cumulative budget, and warnings cannot repair Error.
- NeedInput/NeedOutput report exact counts and only committed output; no borrow outlives a call unless represented in the type; Done requires terminator/trailing policy.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Fixed allocation-free trait return aliases, all
  eventual non-exhaustive states and commit categories, explicit borrow
  lifetimes, caller-planned token-capacity exhaustion, chunk-boundary
  equivalence, and zero-progress rejection.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Compile-time API tests prove each trait has one fixed report alias, downstream
  matches remain extension-safe, and commit payload borrows cannot outlive
  caller-provided token storage.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.14.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.14.1 - Cooperative execution quantum and cancellation latency

Status: Planned.

Context:

This is the exclusive foundations handoff for cooperative execution quantum and cancellation latency. Its API
and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release
cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete cooperative execution quantum and cancellation latency with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Cooperative execution quantum and cancellation latency.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Require a caller-supplied work grant or maximum quantum for every incremental call.
- Place deterministic checkpoints in entropy loops, ICC CLUT execution, resampling taps, animation composition, and encoder searches.
- Enable the `Yielded`, `LimitExceeded`, and `Cancelled` state categories already
  declared by v0.14.0 without changing either fixed trait return alias.
- Quantum exhaustion returns Yielded { work_consumed }; a zero grant yields without input/output changes and resume never repeats or skips semantic work.
- Distinguish resumable Yielded, sticky terminal LimitExceeded, and terminal caller Cancelled requiring explicit Reset without budget refund.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Bounded work, Yielded/LimitExceeded/Cancelled semantics, deterministic resume, committed prefixes, and latency tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.14.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.14.2 - Physically enforceable incremental decode commit modes

Status: Planned.

Context:

This is the exclusive foundations handoff for physically enforceable incremental decode commit modes. Its API
and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release
cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete physically enforceable decode commit modes with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Physically enforceable decode commit modes.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Define Atomic, CommittedRows, CommittedFrames, CommittedRegions, and explicitly named relaxed behavior.
- Plan scratch and destination requirements before mutation; unsupported capabilities or insufficient staging fail early and never downgrade Atomic or another requested mode.
- Enable the byte, row, frame, and disjoint-region `CommitSet<'a>` categories
  declared by v0.14.0 without changing either fixed trait return alias.
- Return one StepReport with exact committed units for Progress, NeedInput,
  NeedOutput, Yielded, Done, Cancelled, LimitExceeded, and Error.
- Charge region-token count/storage and bind tokens to the session, destination, and generation so reset or reuse rejects them.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Planned atomic/unit/region staging, no downgrade, generation-bound tokens, and StepReport commits on every state.
- Audit arithmetic, source/session binding, offsets, terminal transitions,
  capability negotiation, committed units, cumulative/live/peak budgets,
  scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, binding, commit mode,
  limitations, numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.14.2 implementation stop reached. Follow the release cadence and publication rules.`


### v0.15.0 - Metadata envelopes and bounded Exif/ICC/XMP header transport

Status: Planned.

Context:

This is the exclusive foundations handoff for
metadata envelopes and bounded exif/icc/xmp header transport. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete metadata envelopes and bounded exif/icc/xmp header transport with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Metadata envelopes and bounded Exif/ICC/XMP header transport.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Offset/count validation without full metadata interpretation.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.15.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.15.1 - Bounded ICC v2/v4 structural parser

Status: Planned.

Context:

This is the exclusive foundations handoff for
bounded icc v2/v4 structural parser. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bounded icc v2/v4 structural parser with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Bounded ICC v2/v4 structural parser.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Tag counts/sizes/offsets/overlap, curves, LUT dimensions, recursion, opaque preservation, and fuzzing.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.15.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.15.2 - ICC matrix/TRC and chromatic-adaptation engine

Status: Planned.

Context:

This is the exclusive foundations handoff for
icc matrix/trc and chromatic-adaptation engine. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete icc matrix/trc and chromatic-adaptation engine with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: ICC matrix/TRC and chromatic-adaptation engine.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Parametric curves, PCS conversion, adaptation, rendering intent, deterministic scalar vectors, and limits.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.15.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.15.3 - ICC v2 LUT pipelines and deterministic interpolation

Status: Planned.

Context:

This is the exclusive foundations handoff for
icc v2 lut pipelines and deterministic interpolation. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete icc v2 lut pipelines and deterministic interpolation with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: ICC v2 LUT pipelines and deterministic interpolation.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: LUT dimensions/elements, interpolation, PCS bounds, intent, numeric tolerance, and v2 profiles.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.15.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.15.4 - ICC v4 mAB/mBA and processing-element pipelines

Status: Planned.

Context:

This is the exclusive foundations handoff for
icc v4 mab/mba and processing-element pipelines. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete icc v4 mab/mba and processing-element pipelines with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: ICC v4 mAB/mBA and processing-element pipelines.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Element counts/types/order, curves, matrices, CLUTs, recursion, interpolation, and v4 profiles.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.15.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.15.5 - ICC PCS Lab/XYZ, intent selection, and core execution audit

Status: Planned.

Context:

This is the exclusive foundations handoff for
icc pcs lab/xyz, intent selection, and core execution audit. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete icc pcs lab/xyz, intent selection, and core execution audit with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: ICC PCS Lab/XYZ, intent selection, and core execution audit.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Publish an ICC execution-profile matrix covering profile classes,
  input/output channel topologies, transform directions, intent fallback,
  matrix/TRC versus LUT/mAB/mBA paths, named-color/device-link policy, and every
  unsupported class. Structurally valid non-executable profiles return
  Unsupported while remaining preservable.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Core execution-profile matrix, PCS/intents/adaptation, preservable Unsupported profiles, and differential tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.15.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.15.6 - ICC adaptive-gain tag and type structural parsing

Status: Planned.

Context:

This handoff admits only the structure added by the ICC 17 April 2025
Adaptive Gain Curve amendment. Execution remains unavailable.

Goal:

Parse and preserve ADGC/adaptiveGainCurveType without arithmetic ambiguity,
out-of-range reads, unbounded tables, or accidental execution.

Deliverables:

- Pin the ICC amendment and its normative ISO 21496-1:2025 dependency; record
  redistribution, purchase, patent, and implementation-authorization status.
- Validate profile class/data colour space, embedded flags, signatures,
  reserved fields, GUID, CICP values, float encodings, 64-bit positions,
  shared curve ranges, triplet counts, exact lengths, and non-overlap policy.
- Preserve valid unsupported data opaquely and reject malformed structures
  without interpreting the curve.
- Add every-field truncation, offset/count overflow, overlap, non-finite float,
  duplicate-position, huge-table, mutation, fuzz, and 32-bit tests.

Verification:

- Required release evidence: clause mapping, primary-source and patent review,
  exact bounds, independent structural vectors, and fail-closed unsupported
  execution.
- Run repository, supported-Rust, no_std, target, package, SBOM, fuzz, Miri,
  sanitizer, and denial-of-service gates.

Exit criteria:

- Structural support is the only new capability and cannot execute an adaptive
  curve through any public API.
- The legal/IP decision and unsupported behavior are explicit; all
  critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green and at a publication checkpoint the permanent report records PASS.
- `v0.15.6 implementation stop reached. Follow the release cadence and publication rules.`

### v0.15.7 - ICC adaptive-gain execution admission

Status: Planned.

Context:

This handoff either admits the independently reviewed adaptive-gain algorithm
or freezes it as preservable Unsupported. Structural parsing is unchanged.

Goal:

Make one explicit, legally reviewed execution decision with complete numeric,
resource, and interoperability evidence.

Deliverables:

- Complete patent/licensing review before implementation; absence of authority
  produces a documented unclaimed decision, not an inferred permission.
- If admitted, implement the input, piecewise-cubic gain, and output evaluators
  with bounded node search, target-headroom policy, CICP handling, finite-domain
  checks, coefficient stability, declared precision/FMA behavior, and work cost.
- Define degenerate x coordinates, node ordering, extrapolation, clipping,
  negative/non-finite values, shared curves, LUT approximation, and output-tier
  semantics.
- Add normative, high-precision, boundary, cross-platform, differential,
  property, fuzz, and hostile-complexity vectors.

Verification:

- Required release evidence: legal admission record plus bit/tolerance contract,
  high-precision oracle comparisons, deterministic resume, and bounded work; or
  a tested Unsupported result for every structurally valid ADGC profile.
- Pentest the exact admitted or explicitly unclaimed surface.

Exit criteria:

- The support matrix states exactly whether ADGC execution is supported and
  never conflates preservation, parsing, and execution.
- All critical/high findings are fixed and cleanly retested; CI and CodeQL
  default setup are green and at a publication checkpoint the permanent report records PASS.
- `v0.15.7 implementation stop reached. Follow the release cadence and publication rules.`

### v0.15.8 - Complete declared ICC profile audit

Status: Planned.

Context:

This evidence handoff freezes the complete declared ICC v2/v4.4 profile,
including the 2025 amendment decision, before codecs consume it.

Goal:

Demonstrate that every declared ICC profile class, tag path, execution route,
limit, preservation rule, and unsupported case is mapped and tested.

Deliverables:

- Publish the amendment-inclusive structural/execution cross-product and close
  every SPEC_MAPPING disposition.
- Run conformance and differential profiles across matrix/TRC, v2 LUT,
  mAB/mBA, PCS, intents, adaptation, malformed profiles, and ADGC policy.
- Freeze numeric tolerances, corpus provenance, public API, limits, package
  contents, release notes, SBOM, and cadence-assigned pentest scope.

Verification:

- Required release evidence: complete profile matrix, long fuzzing, 32-bit and
  cross-target vectors, independent color-engine comparisons, and zero
  unexplained mismatches.

Exit criteria:

- No wildcard ICC claim remains and valid unimplemented profiles are
  preservable Unsupported.
- All critical/high findings are fixed and cleanly retested; CI and CodeQL
  default setup are green and at a publication checkpoint the permanent report records PASS.
- `v0.15.8 implementation stop reached. Follow the release cadence and publication rules.`

### v0.15.9 - Bounded BCP 47 language-tag profile

Status: Planned.

Context:

PNG iTXt, XML `xml:lang`, and XMP language alternatives share one language-tag
grammar and must not grow independent, inconsistent parsers.

Goal:

Provide a no_std, allocation-free BCP 47 syntax/canonicalization and matching
foundation with bounded input and explicit registry-independent semantics.

Deliverables:

- Pin RFC 5646 and RFC 4647 and distinguish well-formed syntax from living
  IANA-registry validity.
- Parse language, extlang, script, region, variant, extension, private-use, and
  grandfathered shapes with duplicate-singleton/variant checks and ASCII case
  rules.
- Define length/subtag/work limits, canonical comparison, basic filtering/
  lookup scope, private-use handling, and XMP's `x-default` policy.
- Add every-boundary, malformed, case, duplicate, private-use, long-tag,
  matching, PNG/XML/XMP integration, property, fuzz, and differential tests.

Verification:

- Required release evidence: RFC production mapping, registry-independent
  claim language, cross-container vectors, allocation-free bounds, and no
  locale/environment dependence.

Exit criteria:

- One shared parser serves every admitted language-tag field and does not claim
  that a syntactically valid tag is currently registered.
- Pentest covers the exact parser; all critical/high findings are fixed and
  cleanly retested, and CI/CodeQL are green.
- `v0.15.9 implementation stop reached. Follow the release cadence and publication rules.`

### v0.16.0 - Format IDs, media types, bounded probing, static registry

Status: Planned.

Context:

This is the exclusive foundations handoff for
format ids, media types, bounded probing, static registry. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete format ids, media types, bounded probing, static registry with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Format IDs, media types, bounded probing, static registry.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Collision, ambiguity, polyglot, and disabled-feature tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.16.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.16.1 - Non-destructive forward-only and seekable probing

Status: Planned.

Context:

This is the exclusive foundations handoff for non-destructive forward-only and seekable probing. Its API
and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release
cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete non-destructive forward-only and seekable probing with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Non-destructive forward-only and seekable probing.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Use a caller-owned bounded prefix so every candidate sees identical bytes and the selected decoder inherits consumed bytes without seeking.
- NeedInput reports minimum additional bytes and the absolute cap; physical bytes are charged once while repeated probe work remains charged.
- Ambiguous/rejected probing returns the intact prefix, and seekable probing restores the original logical position or reports failure.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Shared caller prefix, decoder inheritance, NeedInput minima/cap, one-time byte charging, and seek restoration.
- Audit arithmetic, source/session binding, offsets, terminal transitions,
  capability negotiation, committed units, cumulative/live/peak budgets,
  scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, binding, commit mode,
  limitations, numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.16.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.17.0 - Fallible owned storage and std::io adapters

Status: Planned.

Context:

This is the exclusive foundations handoff for
fallible owned storage and std::io adapters. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete fallible owned storage and std::io adapters with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Fallible owned storage and std::io adapters.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Allocation-failure, interrupted-I/O, and feature-matrix tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.17.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.17.1 - Reentrancy, concurrency, and auto-trait contract

Status: Planned.

Context:

This is the exclusive foundations handoff for reentrancy, concurrency, and auto-trait contract. Its API
and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release
cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete reentrancy, concurrency, and auto-trait contract with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Reentrancy, concurrency, and auto-trait contract.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- State intended Send/Sync behavior for decoders, encoders, plans, workspaces, budgets, views, registries, and color transforms with compile-time assertions.
- Permit one plan to execute concurrently only with independent typed workspaces, parent-backed child grants, disjoint destinations, and exclusive scratch.
- Document which states may move between threads and require reusable registries/transforms to be immutable and reentrant where claimed.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Send/Sync assertions, independent-workspace concurrency, disjoint output, immutable registry, and scratch ownership tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.17.1 implementation stop reached. Follow the release cadence and publication rules.`


### v0.18.0 - Foundation candidate review and representative-codec readiness

Status: Planned.

Context:

This is the exclusive foundations handoff for
foundation candidate review and representative-codec readiness. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete foundation candidate review and representative-codec readiness with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Foundation candidate review and representative-codec readiness.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- This is a candidate review, not a permanent public API freeze. Representative real codecs may still drive compatible redesign before v0.94.8.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: External design review, dummy lifecycle exercise, no-default/32-bit/WASM matrix, and documented evolvability.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.18.0 implementation stop reached. Follow the release cadence and publication rules.`


## Phase: Simple and lossless codecs

Prove contracts on bounded formats while keeping the public API evolvable and splitting encoders from audits.

### v0.19.0 - Common codec crate template and decode-plan contract

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
common codec crate template and decode-plan contract. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete common codec crate template and decode-plan contract with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Common codec crate template and decode-plan contract.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Codec contracts expose decode_native and decode_rendered separately and negotiate source/sink capabilities during planning.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: A dummy codec proves limit, scratch, progress, and rollback invariants.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.19.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.20.0 - BMP source ledger, file envelope, and dialect-matrix freeze

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp source provenance, file-envelope validation, and the dialect-matrix freeze.
No dialect parser or pixel decoder may begin until this release defines exact
entry points and combinations. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete the BMP source ledger, file envelope, and dialect-matrix freeze with
bounded behavior, explicit claims, and evidence sufficient for an iterative
pentest and release decision.

Deliverables:

- Complete only the release-scoped capability: BMP source ledger, file envelope, and dialect-matrix freeze.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Pin exact Microsoft GDI/Open Specification revisions, original IBM OS/2
  sources, and primary provenance for every compatibility header before its
  admission; source uncertainty is Unsupported, not a guessed layout.
- Separate standalone `.bmp` files from bare packed DIBs. Validate the 14-byte
  `BITMAPFILEHEADER`, reserved fields, declared file size, pixel offset, DIB
  start, intervening masks/palette/profile regions, and actual source bounds
  without trusting redundant size fields.
- Freeze a machine-readable cross-product keyed by envelope, exact DIB header
  size/name, dimensions, planes, depth, palette width, mask placement,
  compression, orientation, profile behavior, and decode/encode tier. Every
  cell is Supported or Unsupported and links to planned evidence.
- Recognize only allow-listed header sizes. Unknown, truncated, oversized, or
  inconsistent headers fail before allocation/output and never fall back to a
  shorter known structure.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: `.bmp`/bare-DIB separation, exact-size dispatch
  policy, primary-source ledger, complete dialect cross-product, header-size
  confusion and offset corpus, and fail-closed unknown-size tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- No BMP parser milestone starts until the source ledger and dialect matrix
  contain no implicit, inherited, or “best effort” support cell.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.20.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.20.1 - BMP 12-byte core-family headers and RGBTRIPLE palettes

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for the 12-byte
`BITMAPCOREHEADER` family shared by early Windows/OS/2-compatible DIBs. It does
not reuse INFO-family signed-dimension, palette, mask, compression, or profile
semantics. Later BMP families remain unavailable or explicitly fail closed.

Goal:

Complete bounded structural parsing for the 12-byte core family with an exact
layout, explicit capabilities, and evidence sufficient for an iterative
pentest and release decision.

Deliverables:

- Complete only the release-scoped capability: BMP 12-byte core-family headers and RGBTRIPLE palettes.
- Parse exact unsigned 16-bit width/height, require planes=1, and accept only
  the source-admitted 1/4/8/24-bpp core depths; zero or unsupported
  combinations fail before output planning.
- Locate and bound three-byte `RGBTRIPLE` palette entries independently from
  four-byte INFO-family `RGBQUAD` entries. Derive default palette counts with
  checked shifts and reconcile them with the pixel offset and available bytes.
- Reject top-down, masks, RLE, color-profile, and extension fields that the core
  layout cannot represent; trailing bytes never enlarge the header implicitly.
- Keep Windows/OS/2 provenance ambiguity visible in metadata without inventing
  distinct semantics where the bytes cannot distinguish them.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, security documentation, changelog, notes, packages, SBOM,
  and the cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Exact 12-byte dispatch, unsigned-dimension and
  planes/depth matrices, RGBTRIPLE palette bounds, pixel-offset reconciliation,
  every-field truncation, and cross-family confusion tests.
- Add positive and negative fixtures for minimum/maximum dimensions, every
  admitted depth/palette count, short/overlong palettes, offsets inside headers
  or palettes, fake extension bytes, and core-versus-INFO reinterpretation.
- Audit checked arithmetic, offsets, source/session binding, limits, scratch,
  output planning, terminal transitions, and zero-progress behavior.
- Run applicable unit, property, truncation, mutation, differential, fuzz,
  Kani, Miri, sanitizer, stack, code-size, performance, and denial-of-service
  checks plus repository, dependency, toolchain, feature, platform, and SBOM gates.

Exit criteria:

- The 12-byte core family is structurally complete and no later-header
  semantics leak into it.
- Every admitted and rejected combination links to primary-source mapping and
  passing evidence; unsupported behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- Pentest covers header confusion, palette sizing, offsets, truncation, and
  inherited invariants; all critical/high findings are fixed and retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.20.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.20.2 - BMP Windows INFO/V4/V5 and V2/V3 compatibility headers

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for Windows-family
DIB headers. The official 40-byte `BITMAPINFOHEADER`, 108-byte
`BITMAPV4HEADER`, and 124-byte `BITMAPV5HEADER` are distinct layouts. De-facto
52/56-byte V2/V3 headers are a separate compatibility claim and are admitted
only after primary provenance is pinned. Pixel and color execution remain in
later releases.

Goal:

Complete exact structural dispatch for Windows INFO/V4/V5 and explicitly
admitted V2/V3 compatibility headers without size fallback or semantic bleed.

Deliverables:

- Complete only the release-scoped capability: BMP Windows INFO/V4/V5 and V2/V3 compatibility headers.
- Parse the exact 40/108/124-byte layouts with signed dimensions, planes,
  depths, compression, image size, resolution, palette counts, masks, color
  space, intent, and profile range fields appropriate to each revision.
- Treat 52/56-byte headers as named compatibility dialects only if their exact
  field layouts and provenance are recorded; otherwise return Unsupported.
- Distinguish masks appended after a 40-byte header from masks stored inside
  later headers. Never infer alpha from an unused BI_RGB high byte or from
  bytes belonging to a palette/pixel region.
- Validate extension-field availability, reserved values, offsets, ranges, and
  combination legality before allocation/output; parsing V4/V5 fields does not
  claim their color semantics before v0.24.0.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, security documentation, changelog, notes, packages, SBOM,
  and the cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Exact 40/108/124-byte dispatch, explicit
  52/56-byte provenance decision, every-field boundary/truncation corpus,
  external-versus-inline mask placement, and no-shorter-header fallback.
- Add fixtures for header sizes one byte below/above every known size, claimed
  sizes larger than input, extension/pixel overlap, palette/mask ambiguity,
  signed-height extrema, invalid planes/depth/compression tuples, and profile
  ranges outside the file.
- Audit checked arithmetic, offsets, source/session binding, limits, scratch,
  output planning, terminal transitions, and zero-progress behavior.
- Run applicable unit, property, truncation, mutation, differential, fuzz,
  Kani, Miri, sanitizer, stack, code-size, performance, and denial-of-service
  checks plus repository, dependency, toolchain, feature, platform, and SBOM gates.

Exit criteria:

- Every Windows/compatibility header is either exact-layout Supported or
  explicitly Unsupported; unknown sizes cannot enter a pixel decoder.
- Each admitted combination links to primary-source mapping and passing
  evidence; V4/V5 color/profile execution remains gated on v0.24.0.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- Pentest covers size confusion, mask placement, signed extrema, overlaps, and
  inherited invariants; all critical/high findings are fixed and retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.20.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.20.3 - BMP OS/2 2.x extended headers and container decision

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for IBM OS/2 2.x
extended bitmap headers and their surrounding container namespace. Numeric
fields that resemble Windows values do not inherit Windows semantics. Work is
blocked until exact original IBM editions and hashes are in the source ledger.

Goal:

Complete source-bound structural parsing and explicit admission decisions for
each documented OS/2 2.x header revision without guessing undocumented sizes,
compression meanings, palette layouts, or container behavior.

Deliverables:

- Complete only the release-scoped capability: BMP OS/2 2.x extended headers and container decision.
- Enumerate each IBM-documented `BITMAPINFOHEADER2` revision and shortened form
  by exact size and field layout; record Supported or Unsupported independently.
- Validate OS/2 dimensions, planes, depths, recording/rendering fields,
  palette-entry width, color encoding, and declared image size using only the
  pinned revision's semantics.
- Give the OS/2 compression namespace its own typed dispatch. RLE24 and Huffman
  1D remain unavailable until their later admission/implementation decision;
  colliding Windows numeric values cannot select a Windows decoder.
- Decide standalone `BM` support separately from bitmap-array and icon/pointer
  signatures `BA`, `IC`, `CI`, `PT`, and `CP`. Unadmitted containers return
  Unsupported before following offsets or allocating frame tables.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, security documentation, changelog, notes, packages, SBOM,
  and the cadence-appropriate release-evidence record.

Verification:

- Required release evidence: IBM-source revision matrix, exact-size dispatch,
  palette/layout differences, typed compression namespace, and explicit
  BA/IC/CI/PT/CP admission results.
- Add fixtures for every admitted revision and signature, cyclic/overlapping
  array offsets, cross-family numeric collisions, short headers, unknown sizes,
  palette/pixel overlap, dimensions/planes/depth extrema, and unsupported
  compression values.
- Audit checked arithmetic, offsets, source/session binding, limits, scratch,
  output planning, terminal transitions, and zero-progress behavior.
- Run applicable unit, property, truncation, mutation, differential, fuzz,
  Kani, Miri, sanitizer, stack, code-size, performance, and denial-of-service
  checks plus repository, dependency, toolchain, feature, platform, and SBOM gates.

Exit criteria:

- Every sourced OS/2 2.x header/container form is Supported or Unsupported by
  exact identity; no numeric or structural behavior is borrowed from Windows.
- Missing primary documentation, an unresolved signature, or an implicit
  compression mapping blocks release and all later BMP milestones.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- Pentest covers family confusion, offset graphs, compression dispatch,
  truncation, and inherited invariants; all critical/high findings are fixed
  and retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.20.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.21.0 - BMP BI_RGB depths, palettes, padding, row orientation

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp bi_rgb depths, palettes, padding, row orientation. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp bi_rgb depths, palettes, padding, row orientation with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: BMP BI_RGB depths, palettes, padding, row orientation.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Implement BI_RGB only through the exact dialect table: core-family depths and
  RGBTRIPLE palettes remain separate from INFO/V4/V5 depths and RGBQUAD
  palettes; unsupported depth/header pairs fail before output mutation.
- Compute row bits, DWORD-aligned stride, absolute-height storage, pixel extent,
  and source/output requirements with checked arithmetic. Reconcile zero or
  redundant image-size fields without allowing them to shrink required bytes.
- Permit negative-height top-down storage only for a header/compression pair
  whose source admits it; reject signed-height minimum and compressed top-down
  forms without applying absolute value unsafely.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Per-header depth/palette/stride/orientation matrix,
  1/4/8/16/24/32-bit goldens, signed-height extrema, redundant-size mismatch,
  row-padding, palette-bound, and every-row truncation tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.21.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.22.0 - BMP bitfields, alpha masks, top-down rules

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp bitfields, alpha masks, top-down rules. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp bitfields, alpha masks, top-down rules with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: BMP bitfields, alpha masks, top-down rules.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Distinguish three masks appended after a 40-byte INFO header from inline
  RGB/alpha masks in admitted V2/V3/V4/V5 headers. Mask bytes cannot overlap a
  palette, profile, or pixel payload and the pixel offset cannot hide a missing
  mask.
- Require masks to fit the declared depth, use contiguous bits where mandated,
  and obey overlap and required-channel rules. An absent alpha mask is not
  inferred from unused BI_RGB bits.
- Record an explicit, source-backed `BI_ALPHABITFIELDS`/Windows CE compatibility
  decision rather than aliasing its numeric value to an OS/2 compression mode.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: External/inline mask-placement matrix,
  BI_ALPHABITFIELDS decision, absent-alpha behavior, overlap/gap/full-width,
  pixel-offset confusion, and signed-height tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.22.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.23.0 - BMP RLE4/RLE8

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp rle4/rle8. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp rle4/rle8 with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: BMP RLE4/RLE8.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Admit RLE8 only for sourced 8-bpp header combinations and RLE4 only for
  sourced 4-bpp combinations. Reject core-family, V4/V5, OS/2, or other pairings
  unless their exact dialect cell explicitly permits the mode.
- Require positive bottom-up height, bounded encoded and absolute runs,
  word-aligned absolute payloads, in-range delta moves, explicit end-of-line and
  end-of-bitmap handling, exact committed output, and deterministic trailing
  data policy.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Header/depth admission matrix, compressed top-down
  rejection, escape/delta/absolute/padding boundaries, early/late terminators,
  exact-output accounting, and no-progress fuzzing.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.23.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.24.0 - BMP V4/V5 color declarations and embedded-profile transport

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp v4/v5 color declarations and embedded-profile transport. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp v4/v5 color declarations and embedded-profile transport with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: BMP V4/V5 color declarations and embedded-profile transport.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Separate calibrated RGB endpoints/gamma, sRGB, system/default space,
  rendering intent, embedded profile, and linked profile cases by exact V4/V5
  revision. Reserved and incompatible combinations fail or warn only according
  to the pinned source policy.
- Validate profile offsets relative to the correct header origin, checked
  profile end, pixel/palette/mask overlap, placement rules, and declared size.
  Linked profile bytes remain inert data and can never trigger file or network I/O.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Calibrated/sRGB/profile/intent revision matrix,
  fixed-point endpoint/gamma validation, profile-origin/range/overlap cases,
  reserved fields, and linked-profile no-I/O tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.24.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.25.0 - BMP deterministic uncompressed encoders

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp deterministic uncompressed encoders. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp deterministic uncompressed encoders with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: BMP deterministic uncompressed encoders.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Publish an explicit encoder dialect policy. The caller selects an admitted
  named header revision or a documented canonical default; the encoder never
  chooses a header from input-controlled padding or preserved unknown bytes.
- Plan exact file/header/mask/palette/profile/pixel sizes and offsets before
  emission. Emit only depth, orientation, alpha, color, and metadata
  combinations supported by that selected revision, with reserved bytes zeroed.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Per-dialect encoder capability matrix, canonical
  default rationale, exact headers/masks/palettes/padding/offsets, reserved-byte
  normalization, determinism, and same/cross-dialect round trips.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.25.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.25.1 - BMP deterministic RLE4/RLE8 encoders

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bmp deterministic rle4/rle8 encoders. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bmp deterministic rle4/rle8 encoders with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: BMP deterministic RLE4/RLE8 encoders.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Emit RLE4/RLE8 only for the exact header/depth pairs admitted by v0.23.0;
  reject top-down orientation and dialects whose compression namespace differs.
- Define canonical encoded-versus-absolute choice, row termination, final
  bitmap termination, absolute-run padding, delta policy, and size fields before
  emission; encoder search spends the caller's work budget.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Header/depth encoder matrix, canonical
  escape/padding/delta/terminator policy, exact size fields, deterministic
  packets, bounded work, and decode/encode round trips.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.25.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.25.2 - Complete declared BMP dialect audit

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
complete declared bmp dialect audit. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete complete declared bmp dialect audit with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Complete declared BMP dialect audit.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Freeze the exhaustive support table across file/bare-DIB envelope, exact
  header version/size, depth, palette entry width, mask location, compression,
  orientation, color/profile mode, decode tier, and encode tier. No aggregate
  BMP claim may exceed the table's cells.
- Resolve OS/2 Huffman 1D, RLE24, bitmap arrays, icon/pointer forms,
  BI_ALPHABITFIELDS, CMYK/RLE variants, and embedded BI_JPEG/BI_PNG individually.
  Any supported choice requires a prior implementation handoff and full
  evidence; otherwise it is explicitly Unsupported for 1.0.
- Differential-test each admitted dialect against independent implementations,
  classify every disagreement, and retain cross-family/polyglot cases that try
  to reinterpret one header or compression namespace as another.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Exhaustive dialect cross-product, BI_RGB/bitfield/RLE
  decode and encoder claims, OS/2 legacy and container decisions,
  BI_ALPHABITFIELDS/CMYK/embedded-payload policy, differential results, corpus
  provenance, fuzz coverage, and independent security review.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Every known header and compression family is Supported with evidence or
  explicitly Unsupported; there are no wildcard, nearest-version, or fallback claims.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.25.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.26.0 - QOI structural parse and bounded decoder

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
qoi structural parse and bounded decoder. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete qoi structural parse and bounded decoder with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: QOI structural parse and bounded decoder.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Magic, dimensions, channels, colorspace hint,
  pixel count, wraparound, end-marker, and trailing-data tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.26.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.27.0 - QOI deterministic encoder

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
qoi deterministic encoder. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete qoi deterministic encoder with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: QOI deterministic encoder.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Reference-vector and encode/decode conformance.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.27.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.28.0 - Bounded Netpbm tokenizer

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
bounded netpbm tokenizer. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bounded netpbm tokenizer with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Bounded Netpbm tokenizer.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Comment, whitespace, decimal overflow, token-length fuzzing.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.28.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.29.0 - PBM P1/P4 decode/encode

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
pbm p1/p4 decode/encode. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete pbm p1/p4 decode/encode with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PBM P1/P4 decode/encode.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Bit order, row padding, multi-image policy.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.29.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.30.0 - PGM P2/P5 decode/encode

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
pgm p2/p5 decode/encode. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete pgm p2/p5 decode/encode with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PGM P2/P5 decode/encode.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: MAXVAL scaling, 8/16-bit, source-defined BT.709
  transfer/range declaration, explicit linear/sRGB variant policy, and
  truncation.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.30.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.31.0 - PPM P3/P6 decode/encode

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
ppm p3/p6 decode/encode. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete ppm p3/p6 decode/encode with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PPM P3/P6 decode/encode.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Sample scaling, BT.709 primaries/transfer/range
  declaration, explicit variant policy, token bombs, and binary boundaries.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.31.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.32.0 - PAM P7, if the public claim is “Netpbm”

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
pam p7, if the public claim is “netpbm”. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete pam p7, if the public claim is “netpbm” with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PAM P7, if the public claim is “Netpbm”.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Tuple types, depth, linear opacity, color/alpha
  declarations, header termination, and unknown fields.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.32.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.33.0 - Combined PNM/PAM stream and conformance audit

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
combined pnm/pam stream and conformance audit. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete combined pnm/pam stream and conformance audit with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Combined PNM/PAM stream and conformance audit.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Freeze “Netpbm” to the official PNM plus PAM family. PFM has incompatible
  floating-point/endian/row-order semantics, is not an official Netpbm format,
  and remains explicitly unsupported before 1.0.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Concatenated images, official-tool differential
  tests, and a support matrix that explicitly excludes PFM.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.33.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.34.0 - farbfeld decode and encode

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
farbfeld decode and encode. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete farbfeld decode and encode with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: farbfeld decode and encode.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Exact-size arithmetic, RGBA16-BE, alpha semantics.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.34.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.35.0 - Simple-codec contract and security freeze

Status: Planned.

Context:

This is the exclusive simple and lossless codecs handoff for
simple-codec contract and security freeze. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete simple-codec contract and security freeze with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Simple-codec contract and security freeze.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Cross-codec probe fuzzing, 32-bit memory tests, simple-codec contract freeze, and external delta review.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.35.0 implementation stop reached. Follow the release cadence and publication rules.`


## Phase: Complex formats

Implement PNG, GIF, JPEG, WebP, and TIFF with native/rendered separation, shared animation/Deflate/IFD foundations, and smaller entropy/color/encoder review units.

### v0.36.0 - PNG signature and bounded probing

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG signature and bounded probing. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete PNG signature and bounded probing with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: PNG signature and bounded probing.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: signature, ambiguity, prefix ownership, and every-byte truncation.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.36.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.36.1 - PNG chunk framing and CRC

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG chunk framing and CRC. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete PNG chunk framing and CRC with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: PNG chunk framing and CRC.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: length/type/data/CRC boundaries, overflow, and mutation fuzzing.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.36.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.36.2 - PNG chunk-order state machine

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG chunk-order state machine. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete PNG chunk-order state machine with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: PNG chunk-order state machine.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: critical/ancillary transition matrix and unknown-critical rejection.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.36.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.37.0 - PNG IHDR and color-type/bit-depth validation

Status: Planned.

Context:

This is the exclusive complex formats handoff for
png ihdr and color-type/bit-depth validation. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete png ihdr and color-type/bit-depth validation with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG IHDR and color-type/bit-depth validation.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Full normative combination matrix.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.37.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.38.0 - Bounded mynd-zlib wrapper

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for Bounded mynd-zlib wrapper. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Bounded mynd-zlib wrapper with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Bounded mynd-zlib wrapper.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: RFC 1950 header/dictionary/trailer rules, Adler-32, and truncation.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.38.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.38.1 - Deflate stored blocks

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for Deflate stored blocks. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Deflate stored blocks with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Deflate stored blocks.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: RFC 1951 stored-block alignment/complement/output bounds and truncation.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.38.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.38.2 - Deflate fixed-Huffman blocks

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for Deflate fixed-Huffman blocks. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Deflate fixed-Huffman blocks with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Deflate fixed-Huffman blocks.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: fixed tables, distance/overlap bounds, transactional bits, and fuzzing.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.38.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.39.0 - Shared dynamic-Huffman and complete bounded mynd-deflate

Status: Planned.

Context:

This is the exclusive complex formats handoff for
shared dynamic-huffman and complete bounded mynd-deflate. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete shared dynamic-huffman and complete bounded mynd-deflate with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Shared dynamic-Huffman and complete bounded mynd-deflate.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- GIF LZW and TIFF LZW remain distinct engines and policies.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Tree proofs, 32 KiB window, distance/overlap fuzzing, output bombs, and reusable crate audit.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.39.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.40.0 - PNG row-filter reconstruction

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG row-filter reconstruction. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG row-filter reconstruction with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG row-filter reconstruction.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: all five filters at every admitted byte width with prior-row boundaries.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.40.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.40.1 - PNG noninterlaced 8-bit core color decoding

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG noninterlaced 8-bit core color decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG noninterlaced 8-bit core color decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG noninterlaced 8-bit core color decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: grayscale, truecolor, grayscale-alpha, and truecolor-alpha golden vectors.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.40.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.41.0 - Packed 1/2/4-bit and 16-bit PNG samples

Status: Planned.

Context:

This is the exclusive complex formats handoff for
packed 1/2/4-bit and 16-bit png samples. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete packed 1/2/4-bit and 16-bit png samples with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Packed 1/2/4-bit and 16-bit PNG samples.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Scaling, endian, tail-bit tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.41.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.42.0 - Adam7 decode and progressive row events

Status: Planned.

Context:

This is the exclusive complex formats handoff for
adam7 decode and progressive row events. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete adam7 decode and progressive row events with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Adam7 decode and progressive row events.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Pass geometry proofs and tiny-image corpus.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.42.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.43.0 - PNG PLTE and tRNS semantics

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG PLTE and tRNS semantics. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG PLTE and tRNS semantics with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG PLTE and tRNS semantics.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: palette cardinality/index/transparency matrices and invalid combinations.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.43.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.43.1 - PNG bKGD, hIST, sBIT, and sPLT chunks

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG bKGD, hIST, sBIT, and sPLT chunks. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG bKGD, hIST, sBIT, and sPLT chunks with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG bKGD, hIST, sBIT, and sPLT chunks.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: per-chunk length/value/order matrices and bounded suggested palettes.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.43.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.44.0 - PNG cHRM, gAMA, and sRGB declarations

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG cHRM, gAMA, and sRGB declarations. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG cHRM, gAMA, and sRGB declarations with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG cHRM, gAMA, and sRGB declarations.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: PNG Third Edition value rules and declaration-precedence matrix.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.44.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.44.1 - PNG iCCP transport and ICC precedence

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG iCCP transport and ICC precedence. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG iCCP transport and ICC precedence with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG iCCP transport and ICC precedence.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: profile-name/Deflate limits, ICC bombs, and conflict precedence.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.44.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.44.2 - PNG cICP and HDR/WCG metadata

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG cICP and HDR/WCG metadata. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG cICP and HDR/WCG metadata with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG cICP and HDR/WCG metadata.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: cICP, mDCV, and cLLI order/dependency/value rules with HDR vectors.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.44.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.45.0 - PNG bounded text chunks

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG bounded text chunks. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG bounded text chunks with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG bounded text chunks.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: tEXt, zTXt, and iTXt keyword/language/compression/UTF-8 limits.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.45.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.45.1 - PNG eXIf, pHYs, and tIME metadata

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG eXIf, pHYs, and tIME metadata. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG eXIf, pHYs, and tIME metadata with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG eXIf, pHYs, and tIME metadata.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: chunk-specific order, length, value, and metadata-transport tests.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.45.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.45.2 - PNG unknown and private chunk policy

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG unknown and private chunk policy. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG unknown and private chunk policy with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG unknown and private chunk policy.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: critical rejection, ancillary preservation, and safe-to-copy editor matrix.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.45.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.46.0 - APNG control and frame-chunk sequencing

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for APNG control and frame-chunk sequencing. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete APNG control and frame-chunk sequencing with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: APNG control and frame-chunk sequencing.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: acTL/fcTL/fdAT order, sequence numbers, rectangles, timing, and limits.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.46.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.46.1 - APNG frame decoding and composition

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for APNG frame decoding and composition. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete APNG frame decoding and composition with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: APNG frame decoding and composition.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: default-image cases, source/source-over, disposal, streaming, and bombs.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.46.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.46.2 - PNG deterministic encoding

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for PNG deterministic encoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete PNG deterministic encoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: PNG deterministic encoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Third Edition emission, row filters, Deflate, metadata, and determinism.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.46.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.46.3 - APNG deterministic encoding

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for APNG deterministic encoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete APNG deterministic encoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: APNG deterministic encoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: frame sequencing, rectangles, timing, disposal/blend, and round trips.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.46.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.46.4 - Complete PNG/APNG conformance and security audit

Status: Planned.

Context:

This is the exclusive PNG/APNG handoff for complete PNG/APNG conformance and security audit. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete complete PNG/APNG conformance and security audit with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Complete PNG/APNG conformance and security audit.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Third Edition mapping, conformance/differential corpus, and long fuzzing.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.46.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.47.0 - GIF87a/89a structure, palettes, sub-blocks, descriptors

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif87a/89a structure, palettes, sub-blocks, descriptors. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif87a/89a structure, palettes, sub-blocks, descriptors with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: GIF87a/89a structure, palettes, sub-blocks, descriptors.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Logical-screen fields, color resolution/sort/
  background/aspect policy, block termination, and palette bounds.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.47.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.48.0 - GIF LZW

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif lzw. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif lzw with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: GIF LZW.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Dictionary/code-width proofs and fuzzing.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.48.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.49.0 - GIF single-frame decode and deinterlace

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif single-frame decode and deinterlace. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif single-frame decode and deinterlace with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: GIF single-frame decode and deinterlace.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Exact pixels and four-pass geometry tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.49.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.50.0 - GIF GCE, transparency, frame composition, all disposal modes

Status: Planned.

Context:

This is the exclusive complex formats handoff for
gif gce, transparency, frame composition, all disposal modes. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gif gce, transparency, frame composition, all disposal modes with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: GIF GCE, transparency, frame composition, all disposal modes.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Snapshot caps and animation bomb corpus.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.50.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.51.0 - GIF named-extension parsing

Status: Planned.

Context:

This is the exclusive GIF handoff for GIF named-extension parsing. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete GIF named-extension parsing with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: GIF named-extension parsing.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Comment, Plain Text, Application, and unknown extension boundaries.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.51.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.51.1 - GIF compatibility and termination policy

Status: Planned.

Context:

This is the exclusive GIF handoff for GIF compatibility and termination policy. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete GIF compatibility and termination policy with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: GIF compatibility and termination policy.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Netscape loop, EOI/trailer, extra pixels, delay, and disposal decisions.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.51.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.51.2 - GIF raw-frame and composited-frame APIs

Status: Planned.

Context:

This is the exclusive GIF handoff for GIF raw-frame and composited-frame APIs. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete GIF raw-frame and composited-frame APIs with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: GIF raw-frame and composited-frame APIs.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: coordinates, disposal sequencing, valid prefixes, and frame ranges.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.51.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.51.3 - GIF exact palettes and bounded histogram

Status: Planned.

Context:

This is the exclusive GIF handoff for GIF exact palettes and bounded histogram. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete GIF exact palettes and bounded histogram with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: GIF exact palettes and bounded histogram.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: unique-color limits, entry layout, ordering, overflow, and caller palettes.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.51.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.51.4 - GIF deterministic palette generation and remapping

Status: Planned.

Context:

This is the exclusive GIF handoff for GIF deterministic palette generation and remapping. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete GIF deterministic palette generation and remapping with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: GIF deterministic palette generation and remapping.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: median-cut policy, remap, bounded dithering, budgets, and goldens.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.51.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.51.5 - GIF LZW encoder

Status: Planned.

Context:

This is the exclusive GIF handoff for GIF LZW encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete GIF LZW encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: GIF LZW encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: dictionary growth/reset/saturation, widths, end code, proofs, and round trips.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.51.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.51.6 - Single-frame GIF encoder

Status: Planned.

Context:

This is the exclusive GIF handoff for Single-frame GIF encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Single-frame GIF encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Single-frame GIF encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: palette/table/transparency/sub-block integration and deterministic output.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.51.6 implementation stop reached. Follow the release cadence and publication rules.`

### v0.51.7 - Animated GIF encoder

Status: Planned.

Context:

This is the exclusive GIF handoff for Animated GIF encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Animated GIF encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Animated GIF encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: canvas/frame limits, timing, loop, disposal, ranges, and round trips.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.51.7 implementation stop reached. Follow the release cadence and publication rules.`

### v0.51.8 - Complete GIF conformance and security audit

Status: Planned.

Context:

This is the exclusive GIF handoff for Complete GIF conformance and security audit. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Complete GIF conformance and security audit with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Complete GIF conformance and security audit.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: normative/de-facto matrix, differential corpus, fuzzing, and animation bombs.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.51.8 implementation stop reached. Follow the release cadence and publication rules.`

### v0.52.0 - JPEG marker and segment framing

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG marker and segment framing. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete JPEG marker and segment framing with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: JPEG marker and segment framing.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: standalone/length-bearing marker boundaries and size mutation fuzzing.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.52.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.52.1 - JPEG quantization and entropy-table declarations

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG quantization and entropy-table declarations. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete JPEG quantization and entropy-table declarations with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: JPEG quantization and entropy-table declarations.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: DQT/DHT/DAC/DRI types, precision, counts, redefinition, and dependencies.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.52.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.52.2 - JPEG frame declarations

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG frame declarations. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete JPEG frame declarations with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: JPEG frame declarations.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: SOF process/precision/components/sampling/dimensions and DNL policy.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.52.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.52.3 - JPEG scan declarations and ordering

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG scan declarations and ordering. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete JPEG scan declarations and ordering with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: JPEG scan declarations and ordering.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: SOS selectors/ranges, multiscan/abbreviated-table state, and invalid transitions.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.52.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.53.0 - JPEG Huffman entropy and byte stuffing

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG Huffman entropy and byte stuffing. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete JPEG Huffman entropy and byte stuffing with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG Huffman entropy and byte stuffing.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: canonical table proofs, marker boundaries, and every-bit truncation.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.53.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.53.1 - JPEG restart and bounded MCU coefficient accounting

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG restart and bounded MCU coefficient accounting. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete JPEG restart and bounded MCU coefficient accounting with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG restart and bounded MCU coefficient accounting.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: restart reset/sequence, MCU geometry, coefficient bounds, and work budgets.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.53.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.54.0 - JPEG scalar IDCT and grayscale reconstruction

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jpeg scalar idct and grayscale reconstruction. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jpeg scalar idct and grayscale reconstruction with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG scalar IDCT and grayscale reconstruction.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: IDCT coefficient bounds, normative tolerance, grayscale blocks, restart corpus, and deterministic scalar vectors.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.54.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.54.1 - JPEG component sampling and bounded upsampling

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jpeg component sampling and bounded upsampling. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jpeg component sampling and bounded upsampling with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG component sampling and bounded upsampling.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Sampling factors, MCU geometry, edge extension, upsampling policy, limits, and reference vectors.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.54.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.54.2 - JPEG native YCbCr and rendered RGB output tiers

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jpeg native ycbcr and rendered rgb output tiers. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jpeg native ycbcr and rendered rgb output tiers with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG native YCbCr and rendered RGB output tiers.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Entropy/native-sample failures and shared color-rendering failures remain distinct.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Native plane/coefficients versus shared-color rendering, JFIF declarations, tolerances, and error separation.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.54.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.55.0 - Extended sequential and 12-bit DCT processes

Status: Planned.

Context:

This is the exclusive complex formats handoff for
extended sequential and 12-bit dct processes. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete extended sequential and 12-bit dct processes with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Extended sequential and 12-bit DCT processes.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Precision and coefficient-range evidence.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.55.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.56.0 - JPEG progressive DC scans

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG progressive DC scans. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete JPEG progressive DC scans with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG progressive DC scans.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: first/refinement DC scan state, predictors, restart, and malformed order.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.56.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.56.1 - JPEG progressive AC scans

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG progressive AC scans. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete JPEG progressive AC scans with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG progressive AC scans.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: spectral selection, EOB runs, coefficient bounds, and restart behavior.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.56.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.56.2 - JPEG successive-approximation integration audit

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG successive-approximation integration audit. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete JPEG successive-approximation integration audit with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG successive-approximation integration audit.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: complete scan-script matrix, native coefficients, work limits, and fuzzing.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.56.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.57.0 - Lossless predictive JPEG process

Status: Planned.

Context:

This is the exclusive complex formats handoff for
lossless predictive jpeg process. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete lossless predictive jpeg process with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Lossless predictive JPEG process.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Predictor, point transform, precision tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.57.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.58.0 - JPEG arithmetic coding

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jpeg arithmetic coding. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jpeg arithmetic coding with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG arithmetic coding.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Conditioning-table and arithmetic-state proofs.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.58.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.59.0 - JPEG differential processes

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jpeg differential processes. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jpeg differential processes with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG differential processes.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Reference-frame dependencies, differential scan state, reconstruction bounds, and malformed graphs.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.59.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.59.1 - JPEG hierarchical processes

Status: Planned.

Context:

This is the exclusive complex formats handoff for
jpeg hierarchical processes. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete jpeg hierarchical processes with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG hierarchical processes.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Frame hierarchy, expansion, dependencies, reconstruction limits, native output, and differential evidence.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.59.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.60.0 - JPEG JFIF and Adobe color declarations

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG JFIF and Adobe color declarations. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete JPEG JFIF and Adobe color declarations with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG JFIF and Adobe color declarations.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: T.871 APP0 plus Adobe RGB/CMYK/YCCK interpretation and precedence.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.60.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.60.1 - JPEG Exif APP1 transport

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG Exif APP1 transport. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete JPEG Exif APP1 transport with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG Exif APP1 transport.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: identifier, length, nested offset, duplicate, and preservation policies.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.60.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.60.2 - JPEG ICC APP2 assembly and color precedence

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for JPEG ICC APP2 assembly and color precedence. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete JPEG ICC APP2 assembly and color precedence with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: JPEG ICC APP2 assembly and color precedence.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: chunk numbering/completeness/duplicates, profile limits, and color vectors.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.60.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.60.3 - JPEG COM and registered/unknown APPn policy

Status: Planned.

Context:

T.86 registrations, COM, SPIFF identifiers, and unknown APPn segments are a
metadata policy surface distinct from JFIF, Exif, and ICC interpretation.

Goal:

Bound, classify, preserve, discard, or reject non-image JPEG segments without
executing metadata or making an aggregate SPIFF claim.

Deliverables:

- Pin T.86 and map registered APPn identifiers, COM, duplicates, ordering, and
  conflicts; decide SPIFF structural support explicitly.
- Bound segment count, individual/aggregate payload, preserved unknown bytes,
  text exposure, and warnings; metadata remains inert.
- Define exact round-trip and safe editor policy for recognized, unknown,
  duplicate, malformed, and truncated segments.
- Add identifier-collision, polyglot, log-injection, NUL/text, segment-bomb,
  truncation, mutation, preservation, and fuzz fixtures.

Verification:

- Required release evidence: T.86 registry mapping, SPIFF decision,
  byte-preservation matrix, metadata budgets, and no hidden I/O or execution.

Exit criteria:

- Every APPn/COM class has a documented disposition and no “unknown APP”
  wildcard claim remains.
- Pentest covers the exact metadata surface; all critical/high findings are
  fixed and cleanly retested, and CI/CodeQL are green.
- `v0.60.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.61.0 - Baseline JPEG encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
baseline jpeg encoder. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete baseline jpeg encoder with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Baseline JPEG encoder.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Deterministic valid baseline emission, quality controls, coefficient limits, and round trips.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.61.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.61.1 - Progressive JPEG encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
progressive jpeg encoder. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete progressive jpeg encoder with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Progressive JPEG encoder.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Scan scripts, successive approximation, deterministic tables, restart policy, and round trips.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.61.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.61.2 - Extended-sequential JPEG encoder

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for extended-sequential JPEG encoder. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete extended-sequential JPEG encoder with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Extended-sequential JPEG encoder.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: precision/process-valid emission, tables, restart, and differential tests.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.61.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.61.3 - Lossless JPEG encoder

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for lossless JPEG encoder. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete lossless JPEG encoder with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Lossless JPEG encoder.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: predictor/point-transform emission, precision, restart, and round trips.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.61.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.61.4 - Arithmetic JPEG encoder admission

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for arithmetic JPEG encoder admission. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete arithmetic JPEG encoder admission with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Arithmetic JPEG encoder admission.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: independent arithmetic evidence or an explicit unclaimed decision.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.61.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.61.5 - Differential JPEG encoder admission

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for differential JPEG encoder admission. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete differential JPEG encoder admission with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Differential JPEG encoder admission.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: independent differential-process evidence or an explicit unclaimed decision.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.61.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.61.6 - Hierarchical JPEG encoder admission

Status: Planned.

Context:

This is the exclusive classic-JPEG handoff for hierarchical JPEG encoder admission. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete hierarchical JPEG encoder admission with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Hierarchical JPEG encoder admission.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: independent hierarchy evidence or an explicit unclaimed decision.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.61.6 implementation stop reached. Follow the release cadence and publication rules.`

### v0.62.0 - Complete declared T.81 conformance and security audit

Status: Planned.

Context:

This is the exclusive complex formats handoff for
complete declared t.81 conformance and security audit. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete complete declared t.81 conformance and security audit with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Complete declared T.81 conformance and security audit.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Reference software, official material, long fuzz campaign.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.62.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.63.0 - WebP RIFF framing and simple-file dispatch

Status: Planned.

Context:

This is the exclusive WebP handoff for WebP RIFF framing and simple-file dispatch. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete WebP RIFF framing and simple-file dispatch with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: WebP RIFF framing and simple-file dispatch.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: RIFF/WEBP size, padding, VP8/VP8L simple payload, and trailing policy.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.63.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.63.1 - WebP VP8X feature and chunk-order state machine

Status: Planned.

Context:

This is the exclusive WebP handoff for WebP VP8X feature and chunk-order state machine. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete WebP VP8X feature and chunk-order state machine with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: WebP VP8X feature and chunk-order state machine.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: feature bits, canvas, chunk multiplicity/order, and invalid combinations.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.63.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.63.2 - WebP ICCP, EXIF, XMP, and unknown chunks

Status: Planned.

Context:

This is the exclusive WebP handoff for WebP ICCP, EXIF, XMP, and unknown chunks. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete WebP ICCP, EXIF, XMP, and unknown chunks with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: WebP ICCP, EXIF, XMP, and unknown chunks.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: bounded metadata, feature consistency, preservation, and unknown policy.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.63.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.64.0 - VP8 Boolean-decoder primitive

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8 Boolean-decoder primitive. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8 Boolean-decoder primitive with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8 Boolean-decoder primitive.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: range/value normalization, refill, termination, and arithmetic-state fuzzing.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.64.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.64.1 - VP8 partition and frame-header parsing

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8 partition and frame-header parsing. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8 partition and frame-header parsing with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8 partition and frame-header parsing.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: partition sizes/counts, key/inter headers, segmentation, and bounds.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.64.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.64.2 - VP8 probability-update and token state

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8 probability-update and token state. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8 probability-update and token state with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8 probability-update and token state.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: coefficient/mode probability updates, defaults, reset, and work accounting.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.64.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.65.0 - VP8 prediction and coefficient reconstruction

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8 prediction and coefficient reconstruction. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8 prediction and coefficient reconstruction with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: VP8 prediction and coefficient reconstruction.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Macroblock/reference bounds, prediction modes, token reconstruction, partition limits, and scalar differential tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.65.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.65.1 - VP8 inverse transforms and reconstructed macroblocks

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8 inverse transforms and reconstructed macroblocks. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8 inverse transforms and reconstructed macroblocks with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: VP8 inverse transforms and reconstructed macroblocks.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Transform arithmetic, coefficient ranges, clipping, prediction integration, and scalar reference vectors.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.65.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.65.2 - VP8 loop filtering and complete still reconstruction

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8 loop filtering and complete still reconstruction. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8 loop filtering and complete still reconstruction with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: VP8 loop filtering and complete still reconstruction.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Expose native YCbCr independently from rendered RGB.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Filter levels/edges, macroblock bounds, native YCbCr output, rendered output, and differential corpus.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.65.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.66.0 - WebP ALPH decoding

Status: Planned.

Context:

This is the exclusive WebP handoff for WebP ALPH decoding. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete WebP ALPH decoding with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: WebP ALPH decoding.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: ALPH filter/compression/preprocessing modes, dimensions, and bombs.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.66.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.66.1 - VP8 native YCbCr and rendered RGB integration

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8 native YCbCr and rendered RGB integration. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8 native YCbCr and rendered RGB integration with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8 native YCbCr and rendered RGB integration.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: matrix/range assumptions, alpha association, output tiers, and color vectors.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.66.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.67.0 - VP8L prefix-code parsing and decoding

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8L prefix-code parsing and decoding. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8L prefix-code parsing and decoding with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8L prefix-code parsing and decoding.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: simple/normal code validation, tables, symbols, and every-bit truncation.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.67.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.67.1 - VP8L LZ77 distance and copy engine

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8L LZ77 distance and copy engine. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8L LZ77 distance and copy engine with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8L LZ77 distance and copy engine.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: distance mapping, overlap, history/output bounds, progress, and bombs.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.67.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.67.2 - VP8L color-cache engine

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8L color-cache engine. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8L color-cache engine with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8L color-cache engine.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: cache-bit bounds, hashing, initialization, access, and deterministic vectors.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.67.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.68.0 - VP8L transform declarations and meta-prefix images

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8L transform declarations and meta-prefix images. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8L transform declarations and meta-prefix images with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8L transform declarations and meta-prefix images.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: transform order/count, dimensions, recursion, prefix images, and limits.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.68.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.68.1 - VP8L predictor transform

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8L predictor transform. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8L predictor transform with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8L predictor transform.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: all predictor modes, edge rules, modular arithmetic, and goldens.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.68.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.68.2 - VP8L color transform

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8L color transform. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8L color transform with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8L color transform.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Map the normative VP8L color transform explicitly; it is distinct
  from the entropy color cache.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: transform-image geometry, delta arithmetic, bounds, and golden vectors.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.68.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.68.3 - VP8L subtract-green transform

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8L subtract-green transform. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8L subtract-green transform with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8L subtract-green transform.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: modular channel arithmetic, order, and exact pixel vectors.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.68.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.68.4 - VP8L color-indexing transform

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8L color-indexing transform. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8L color-indexing transform with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8L color-indexing transform.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: palette image, packing widths, dimension reduction, indexes, and bounds.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.68.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.68.5 - VP8L complete lossless reconstruction audit

Status: Planned.

Context:

This is the exclusive WebP handoff for VP8L complete lossless reconstruction audit. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete VP8L complete lossless reconstruction audit with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: VP8L complete lossless reconstruction audit.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: all transforms plus prefix/LZ/cache integration, exact pixels, bombs, and fuzzing.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.68.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.69.0 - WebP animation decoding

Status: Planned.

Context:

This is the exclusive complex formats handoff for
webp animation decoding. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete webp animation decoding with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: WebP animation decoding.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- WebP blend/disposal mapping reuses the shared v0.5.4 animation kernel.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: ANIM background/loop plus ANMF rectangles,
  duration, blend/dispose, frame limits, and animation fuzzing.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.69.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.69.1 - VP8L deterministic encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8l deterministic encoder. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8l deterministic encoder with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: VP8L deterministic encoder.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Prefix/LZ/cache/transform validity, quality-effort controls, bounded search, determinism, and round trips.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.69.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.69.2 - VP8 deterministic encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
vp8 deterministic encoder. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete vp8 deterministic encoder with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: VP8 deterministic encoder.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Prediction/partition/token validity, quality-effort controls, bounded heuristics, backend determinism, and differential tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.69.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.69.3 - Animated WebP encoder

Status: Planned.

Context:

This is the exclusive complex formats handoff for
animated webp encoder. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete animated webp encoder with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Animated WebP encoder.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: ANMF ordering/rectangles, mixed frame modes, blend/dispose, metadata, and round trips.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.69.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.69.4 - Complete WebP conformance and security audit

Status: Planned.

Context:

This is the exclusive complex formats handoff for
complete webp conformance and security audit. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete complete webp conformance and security audit with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Complete WebP conformance and security audit.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: RFC/VP8/VP8L mappings, still/animated split, ALPH/metadata, encoder modes, long fuzzing, and external review.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.69.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.70.0 - Shared bounded mynd-ifd graph and typed-value engine

Status: Planned.

Context:

This is the exclusive TIFF handoff for shared bounded mynd-ifd graph and typed-value engine. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete shared bounded mynd-ifd graph and typed-value engine with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Shared bounded mynd-ifd graph and typed-value engine.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: count/offset arithmetic, cycles, overlaps, typed values, and fuzzing.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.70.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.70.1 - TIFF 6.0 tag schema and dependency validation

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF 6.0 tag schema and dependency validation. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF 6.0 tag schema and dependency validation with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF 6.0 tag schema and dependency validation.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: required/defaulted tags, types/counts, duplicate policy, and dependencies.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.70.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.71.0 - TIFF baseline uncompressed strips

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF baseline uncompressed strips. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF baseline uncompressed strips with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF baseline uncompressed strips.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: bilevel/Gray/palette/RGB strip geometry, FillOrder, and truncation.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.71.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.71.1 - TIFF PackBits strip decoding

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF PackBits strip decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF PackBits strip decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF PackBits strip decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: packet boundaries, no-op bytes, row/strip output bounds, and bombs.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.71.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.72.0 - TIFF LZW decoding

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF LZW decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF LZW decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF LZW decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: TIFF code-width dialect, clear/end codes, dictionary limits, and bombs.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.72.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.72.1 - TIFF Deflate decoding

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF Deflate decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF Deflate decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF Deflate decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: old/new compression-tag policy, zlib integration, and output limits.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.72.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.72.2 - TIFF horizontal Predictor 2

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF horizontal Predictor 2. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF horizontal Predictor 2 with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF horizontal Predictor 2.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: integer sample widths, planar/contiguous rows, endian, and overflow.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.72.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.72.3 - TIFF floating-point Predictor 3 profile

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF floating-point Predictor 3 profile. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF floating-point Predictor 3 profile with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF floating-point Predictor 3 profile.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Treat Predictor 3 as a separately sourced Adobe extension, not an
  implicit TIFF 6.0 baseline claim; unsupported sample widths fail closed.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Adobe Technical Note 3 byte reordering/differencing vectors and profile matrix.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.72.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.73.0 - TIFF CCITT modified-Huffman RLE

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF CCITT modified-Huffman RLE. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF CCITT modified-Huffman RLE with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF CCITT modified-Huffman RLE.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: run tables, FillOrder, EOL policy, row bounds, and differential corpus.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.73.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.73.1 - TIFF CCITT Group 3 fax decoding

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF CCITT Group 3 fax decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF CCITT Group 3 fax decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF CCITT Group 3 fax decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: T4 options, 1D/2D transitions, EOL/RTC, damaged rows, and limits.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.73.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.73.2 - TIFF CCITT Group 4 fax decoding

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF CCITT Group 4 fax decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF CCITT Group 4 fax decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF CCITT Group 4 fax decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: T6 transitions, EOFB, reference-line bounds, and malformed streams.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.73.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.74.0 - TIFF tiled image layout

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF tiled image layout. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF tiled image layout with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF tiled image layout.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: tile geometry, edge tiles, sparse/overlap policy, offsets, and byte counts.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.74.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.74.1 - TIFF planar image layout

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF planar image layout. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF planar image layout with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF planar image layout.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: plane ordering, per-plane strips/tiles, sample dependencies, and limits.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.74.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.74.2 - TIFF multipage and SubIFD traversal

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF multipage and SubIFD traversal. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF multipage and SubIFD traversal with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF multipage and SubIFD traversal.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: next-IFD/SubIFD graphs, cycles, aggregate page limits, and valid prefixes.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.74.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.75.0 - TIFF YCbCr samples and tag dependencies

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff ycbcr samples and tag dependencies. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff ycbcr samples and tag dependencies with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF YCbCr samples and tag dependencies.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Coefficients, reference black/white, subsampling, positioning, strip/tile geometry, and color vectors.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.75.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.75.1 - TIFF CMYK and CIELab native samples

Status: Planned.

Context:

This is the exclusive complex formats handoff for
tiff cmyk and cielab native samples. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete tiff cmyk and cielab native samples with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF CMYK and CIELab native samples.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Photometric dependencies, signed/sample domains, planar layouts, declarations, and native golden vectors.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.75.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.75.2 - TIFF alpha, ICC, and rendered-color integration

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF alpha, ICC, and rendered-color integration. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF alpha, ICC, and rendered-color integration with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF alpha, ICC, and rendered-color integration.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: ExtraSamples association, ICC precedence, shared rendering, and tolerances.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.75.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.75.3 - TIFF signed-integer and IEEE floating sample domains

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF signed-integer and IEEE floating sample domains. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF signed-integer and IEEE floating sample domains with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF signed-integer and IEEE floating sample domains.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: SampleFormat/depth combinations, endian, NaN/infinity policy, and native output.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.75.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.75.4 - TIFF Orientation presentation policy

Status: Planned.

Context:

TIFF Orientation changes the relationship between stored samples and presented
coordinates and must be settled before TIFF profile and encoder audits.

Goal:

Preserve native storage coordinates while offering explicit, bounded,
caller-selected orientation normalization for all eight defined values.

Deliverables:

- Define stored, native, oriented, region, tile, row-event, and output
  coordinates for every orientation, including width/height swaps.
- Keep decode_native unapplied; require explicit normalization and report its
  allocation, scratch, work, metadata, and loss effects.
- Reuse one checked coordinate kernel later for Exif without duplicating
  format policy.
- Add corners, 1xN/Nx1, odd dimensions, tiles, planar data, regions,
  incremental output, overflow, round-trip, and property tests.

Verification:

- Required release evidence: all-eight-value matrix, inverse mappings,
  selective-decode equivalence, metadata effects, and checked-coordinate proofs.

Exit criteria:

- No TIFF path silently applies Orientation and every output tier states its
  coordinate space.
- Pentest covers the exact presentation surface; all critical/high findings are
  fixed and cleanly retested, and CI/CodeQL are green.
- `v0.75.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.75.5 - TIFF calibrated color declarations

Status: Planned.

Context:

Baseline TIFF colorimetry includes calibrated declarations that are neither ICC
execution nor raw photometric decoding.

Goal:

Validate and interpret the declared TIFF calibrated-color tag profile with
explicit precedence and unsupported behavior.

Deliverables:

- Map WhitePoint, PrimaryChromaticities, TransferFunction,
  ReferenceBlackWhite, BitsPerSample, PhotometricInterpretation, ICC, and
  default dependencies by sample/color profile.
- Bound rational/table counts and numeric domains; reject contradictions and
  preserve valid unsupported combinations without silently assuming sRGB.
- Define ICC-versus-tag precedence, native declarations, rendering plans,
  transfer-table interpolation, rounding, and tolerance.
- Add malformed count/type/rational/table, duplicate/conflict, grayscale/RGB/
  YCbCr, cross-endian, reference-color, and differential fixtures.

Verification:

- Required release evidence: calibrated-tag dependency and precedence matrix,
  independent color vectors, limits, and explicit unsupported cells.

Exit criteria:

- Every admitted calibrated-color combination has a source-mapped disposition;
  no absent or contradictory declaration silently becomes sRGB.
- Pentest covers the exact color surface; all critical/high findings are fixed
  and cleanly retested, and CI/CodeQL are green.
- `v0.75.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.76.0 - Corrected JPEG-in-TIFF decoding

Status: Planned.

Context:

This is the exclusive TIFF handoff for corrected JPEG-in-TIFF decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete corrected JPEG-in-TIFF decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Corrected JPEG-in-TIFF decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: old/new JPEG distinction, table ownership, strip/tile boundaries, and corpus.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.76.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.76.1 - TIFF Exif IFD integration

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF Exif IFD integration. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF Exif IFD integration with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF Exif IFD integration.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Exif/GPS/Interop graph namespaces, nested offsets, cycles, and limits.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.76.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.76.2 - TIFF admitted-extension profile freeze

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF admitted-extension profile freeze. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete TIFF admitted-extension profile freeze with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: TIFF admitted-extension profile freeze.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: each extension has pinned provenance and an explicit support disposition.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.76.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.0 - TIFF baseline uncompressed-strip encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF baseline uncompressed-strip encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF baseline uncompressed-strip encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF baseline uncompressed-strip encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: endian tags/strips/exact sizes/determinism and round trips.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.1 - TIFF PackBits encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF PackBits encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF PackBits encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF PackBits encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: packet/run/row boundaries, exact lengths, determinism, and round trips.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.2 - TIFF LZW encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF LZW encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF LZW encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF LZW encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: dialect widths, clear/end behavior, proofs, determinism, and round trips.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.3 - TIFF Deflate encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF Deflate encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF Deflate encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF Deflate encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: mynd-deflate/zlib integration, output limits, validity, and round trips.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.4 - TIFF horizontal Predictor 2 encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF horizontal Predictor 2 encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF horizontal Predictor 2 encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF horizontal Predictor 2 encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: integer widths, endian, planar/contiguous rows, and round trips.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.5 - TIFF floating-point Predictor 3 encoder admission

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF floating-point Predictor 3 encoder admission. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF floating-point Predictor 3 encoder admission with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF floating-point Predictor 3 encoder admission.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Adobe-profile widths and vectors or an explicit unclaimed decision.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.6 - TIFF CCITT RLE encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF CCITT RLE encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF CCITT RLE encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF CCITT RLE encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: run-code validity, FillOrder, row termination, and differential tests.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.6 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.7 - TIFF CCITT Group 3 encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF CCITT Group 3 encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF CCITT Group 3 encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF CCITT Group 3 encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: T4 options, 1D/2D transitions, EOL/RTC, and differential tests.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.7 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.8 - TIFF CCITT Group 4 encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF CCITT Group 4 encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF CCITT Group 4 encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF CCITT Group 4 encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: T6 transitions, EOFB, reference rows, and differential tests.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.8 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.9 - TIFF tiled-image encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF tiled-image encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF tiled-image encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF tiled-image encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: tile geometry, edge tiles, offsets/byte counts, and deterministic output.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.9 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.10 - TIFF planar-image encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF planar-image encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF planar-image encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF planar-image encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: plane ordering, per-plane storage, dependencies, and round trips.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.10 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.11 - TIFF multipage and SubIFD encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF multipage and SubIFD encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF multipage and SubIFD encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF multipage and SubIFD encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: IFD graph, next/SubIFD links, aggregate limits, and deterministic output.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.11 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.12 - TIFF extended sample/color encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for TIFF extended sample/color encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete TIFF extended sample/color encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: TIFF extended sample/color encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: photometric/SampleFormat/ExtraSamples/YCbCr/ICC dependencies and claims.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.12 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.13 - Corrected JPEG-in-TIFF encoder

Status: Planned.

Context:

This is the exclusive TIFF handoff for Corrected JPEG-in-TIFF encoder. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Corrected JPEG-in-TIFF encoder with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Corrected JPEG-in-TIFF encoder.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: new-style table ownership, strip/tile boundaries, validity, and round trips.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.13 implementation stop reached. Follow the release cadence and publication rules.`

### v0.77.14 - Complete declared TIFF profile audit

Status: Planned.

Context:

This is the exclusive TIFF handoff for Complete declared TIFF profile audit. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Complete declared TIFF profile audit with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Complete declared TIFF profile audit.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: compression/layout/sample/color/extension matrix, conformance, fuzzing, and review.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.77.14 implementation stop reached. Follow the release cadence and publication rules.`

## Phase: Color, metadata, processing, and facade

Complete shared conversions before rendered-color audit, track metadata effects,
implement selective processing, then establish an audited unified-facade
candidate for adapter-driven stabilization.

### v0.78.0 - Cross-format native-sample and color-declaration integration

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
cross-format native-sample and color-declaration integration. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete cross-format native-sample and color-declaration integration with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Cross-format native-sample and color-declaration integration.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- This is not rendered-color conformance; it validates native samples and declarations before the remaining shared conversions.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: decode_native consistency, declaration precedence, preserved profiles, and no premature rendered-color claim.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.78.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.79.0 - Shared bounded TIFF/Exif IFD inspection

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
shared bounded tiff/exif ifd inspection. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete shared bounded tiff/exif ifd inspection with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Shared bounded TIFF/Exif IFD inspection.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Offset graphs, entry counts, cycles, value bounds, MakerNote opacity, and fuzzing.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.79.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.80.0 - Selected bounded Exif field interpretation

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for selected bounded Exif field interpretation. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete selected bounded Exif field interpretation with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Selected bounded Exif field interpretation.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: dimensions, timestamps, strings, types/counts, encoding, and conflicts.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.80.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.80.1 - Bounded Exif thumbnail extraction

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for bounded Exif thumbnail extraction. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete bounded Exif thumbnail extraction with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Bounded Exif thumbnail extraction.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: thumbnail offsets/lengths, nested format limits, overlap, and bomb resistance.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.80.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.80.2 - Explicit Exif orientation policy

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for explicit Exif orientation policy. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete explicit Exif orientation policy with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Explicit Exif orientation policy.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: all eight orientations, coordinate mapping, opt-in transform, and metadata effect.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.80.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.80.3 - Exif 3.1 and Exif-for-XMP profile freeze

Status: Planned.

Context:

The selected Exif API needs an edition-specific audit against CIPA
DC-008-Translation-2026 and DC-010-2026 before XMP reconciliation begins.

Goal:

Freeze an honest Exif 3.1 field and container profile plus the exact
Exif-to-XMP mapping boundary.

Deliverables:

- Map every Exif 3.1 IFD, tag, type/count, value domain, encoding, thumbnail,
  orientation, GPS/Interop relationship, MakerNote, and container rule to
  selected, opaque-preserved, unsupported, or rejected.
- Map every DC-010-2026 property conversion, multiplicity, precedence,
  information-loss, namespace, and round-trip limitation.
- Differentially test JPEG, TIFF, and raw Exif envelopes and publish the exact
  metadata profile used by later XMP policy.
- Freeze limits, warnings, support tables, sources, release notes, SBOM, and
  cadence-assigned pentest scope.

Verification:

- Required release evidence: complete DC-008/DC-010 dispositions, cross-
  container consistency, differential corpus, and zero unexplained mappings.

Exit criteria:

- “Selected Exif” is an enumerable edition-specific claim, not an open-ended
  promise, and every unimplemented valid field remains safely opaque or
  Unsupported.
- Pentest covers the exact profile; all critical/high findings are fixed and
  cleanly retested, and CI/CodeQL are green.
- `v0.80.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.81.0 - XMP packet framing and bounded raw transport

Status: Planned.

Context:

This handoff treats XMP as bounded inert bytes. XML and RDF interpretation
remain unavailable.

Goal:

Frame, validate, expose, and preserve raw XMP packets without parsing their
data model or accessing external resources.

Deliverables:

- Complete only the release-scoped capability: XMP packet framing and bounded raw transport.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Validate xpacket begin/id/end processing instructions, UTF encoding
  declarations, BOM, padding, read-only/update flags, exact packet boundaries,
  aggregate limits, and byte-preservation policy.
- No XML parser, resolver, filesystem, network, schema, or namespace behavior
  is reachable in this release.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: packet boundary/encoding/padding/read-only matrix,
  every-byte truncation, exact preservation, and no external I/O.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.81.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.81.1 - Bounded XML 1.0 and Namespaces profile for XMP

Status: Planned.

Context:

XML tokenization, encoding, namespaces, and entity handling are an independent
hostile-input state machine beneath RDF/XMP interpretation.

Goal:

Implement only the XML 1.0 Fifth Edition and Namespaces 1.0 Third Edition
subset required for XMP, with no resource resolution.

Deliverables:

- Pin the dated W3C XML and Namespaces Recommendations and applicable errata.
- Bound bytes, decoded scalar values, depth, names, attributes, namespace
  bindings, text, comments, CDATA, processing instructions, and total events.
- Support required UTF-8/UTF-16 detection and XML character/end-of-line rules
  with explicit malformed, unsupported-encoding, and limit outcomes.
- Reject DTDs and all general, parameter, internal, and external entity
  declarations except direct recognition of XML's five predefined references;
  disable XInclude, catalogs, schemas, and every resolver.
- Add encoding, namespace, duplicate-attribute, billion-laughs, quadratic-text,
  deep-nesting, truncation, mutation, property, differential, and fuzz tests.

Verification:

- Required release evidence: W3C production mapping, malicious XML corpus,
  deterministic event stream, hard memory/work limits, and proof of no I/O.

Exit criteria:

- Only the documented XMP XML profile is accepted; a generic XML conformance
  claim is forbidden.
- Pentest covers the exact parser state machine; all critical/high findings are
  fixed and cleanly retested, and CI/CodeQL are green.
- `v0.81.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.81.2 - Bounded RDF/XML and XMP data-model inspection

Status: Planned.

Context:

RDF/XML graph construction and XMP arrays, structures, qualifiers, aliases,
and namespaces are separate from safe XML tokenization.

Goal:

Inspect the admitted RDF/XML/XMP data model with finite storage and work while
preserving valid unsupported constructs.

Deliverables:

- Pin RDF 1.1 XML Syntax and map the XMP Part 1 restrictions and deviations.
- Bound graph nodes, properties, values, arrays, structures, qualifiers,
  aliases, namespace bindings, identifiers, strings, language alternatives,
  recursion, and duplicate work.
- Define rdf:about/ID/nodeID/resource/parseType, URI/base behavior, arrays,
  typed nodes, unknown namespaces, aliases, duplicates, and unsupported
  productions without dereferencing any IRI.
- Route `xml:lang` and XMP language alternatives through the shared v0.15.9
  BCP 47 profile, including the XMP-specific `x-default` policy.
- Add grammar-production, graph-bomb, alias-cycle, duplicate, namespace,
  malformed, truncation, round-trip, differential, property, and fuzz tests.

Verification:

- Required release evidence: RDF/XMP production and data-model mapping,
  bounded graph accounting, malicious RDF corpus, and no URI dereference.

Exit criteria:

- The public claim is the exact bounded XMP inspection profile, not a general
  RDF store or validating XML processor.
- Pentest covers the exact graph surface; all critical/high findings are fixed
  and cleanly retested, and CI/CodeQL are green.
- `v0.81.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.81.3 - XMP and legacy-metadata conflict/rewrite policy

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for XMP and legacy-metadata conflict/rewrite policy. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete XMP and legacy-metadata conflict/rewrite policy with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: XMP and legacy-metadata conflict/rewrite policy.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: XMP Part 3 and CIPA DC-010 reconciliation, preserve/discard/rewrite, duplicates, and round trips.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.81.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.81.4 - Transformation-aware metadata effect planning

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for transformation-aware metadata effect planning. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete transformation-aware metadata effect planning with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Transformation-aware metadata effect planning.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: preserved/rewritten/invalidated/decision results for every operation.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.81.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.82.0 - YCbCr matrices, ranges, subsampling, and chroma siting

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
ycbcr matrices, ranges, subsampling, and chroma siting. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete ycbcr matrices, ranges, subsampling, and chroma siting with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: YCbCr matrices, ranges, subsampling, and chroma siting.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: JPEG/WebP/TIFF reference vectors.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.82.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.83.0 - Gray, CMYK, and YCCK conversion

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for gray, CMYK, and YCCK conversion. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete gray, CMYK, and YCCK conversion with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Gray, CMYK, and YCCK conversion.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: black-generation/Adobe policy, native-to-rendered vectors, and gamut limits.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.83.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.83.1 - CIELab and declared wide-gamut conversion

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for CIELab and declared wide-gamut conversion. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete CIELab and declared wide-gamut conversion with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: CIELab and declared wide-gamut conversion.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: white-point/adaptation/range rules, out-of-gamut policy, and reference vectors.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.83.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.84.0 - Straight/premultiplied alpha conversion

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
straight/premultiplied alpha conversion. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete straight/premultiplied alpha conversion with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Straight/premultiplied alpha conversion.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Zero-alpha, rounding, and invariant tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.84.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.85.0 - Explicit color-conversion planning

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for explicit color-conversion planning. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete explicit color-conversion planning with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Explicit color-conversion planning.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: information-loss declaration, numeric tier, stages, scratch, and work plan.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.85.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.85.1 - Sample-depth conversion

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for sample-depth conversion. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete sample-depth conversion with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Sample-depth conversion.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: all admitted integer/float depths, scaling, rounding, saturation, and alpha.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.85.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.85.2 - Deterministic advanced dithering

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for deterministic advanced dithering. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete deterministic advanced dithering with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Deterministic advanced dithering.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: ordered/error-diffusion kernels, edge/error bounds, budgets, and goldens.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.85.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.85.3 - Final cross-format rendered-color conformance audit

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for final cross-format rendered-color conformance audit. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete final cross-format rendered-color conformance audit with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Final cross-format rendered-color conformance audit.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: PNG/JPEG/WebP/TIFF profiles, precedence, tolerances, and differential results.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.85.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.86.0 - Crop, flip, rotate, transpose

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
crop, flip, rotate, transpose. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete crop, flip, rotate, transpose with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Crop, flip, rotate, transpose.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: In-place overlap and rectangle proofs.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.86.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.87.0 - Checked affine geometry and border modes

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
checked affine geometry and border modes. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete checked affine geometry and border modes with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Checked affine geometry and border modes.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Finite-matrix and coordinate-overflow proofs.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.87.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.88.0 - Nearest and bilinear resampling

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
nearest and bilinear resampling. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete nearest and bilinear resampling with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Nearest and bilinear resampling.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Pixel-center and edge-policy golden tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.88.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.89.0 - Bicubic resampling

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
bicubic resampling. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete bicubic resampling with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Bicubic resampling.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Coefficient normalization and overshoot policy.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.89.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.90.0 - Lanczos3 resampling

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
lanczos3 resampling. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete lanczos3 resampling with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Lanczos3 resampling.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Tap planning, ring-buffer limits, reference vectors.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.90.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.91.0 - Remaining Porter-Duff compositing operators

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
remaining porter-duff compositing operators. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete remaining porter-duff compositing operators with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Remaining Porter-Duff compositing operators.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Source and source-over remain the audited shared animation primitive; this release adds the other Porter-Duff operators.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Shared source/source-over compatibility, remaining operators, linear domain, alpha, overlap, and invariants.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.91.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.92.0 - Declared artistic blend modes

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for Declared artistic blend modes. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Declared artistic blend modes with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Declared artistic blend modes.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: formula/domain/clamping/NaN/alpha matrices and interoperability vectors.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.92.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.92.1 - Clipped pixel, span, and fill primitives

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for Clipped pixel, span, and fill primitives. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Clipped pixel, span, and fill primitives with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Clipped pixel, span, and fill primitives.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: signed/overflowing coordinates, clipping, layout, alpha, and work budgets.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.92.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.92.2 - Rectangle and overlap-safe blit primitives

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for Rectangle and overlap-safe blit primitives. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Rectangle and overlap-safe blit primitives with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Rectangle and overlap-safe blit primitives.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: empty/degenerate rectangles, clipping, overlap directions, and alias policy.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.92.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.92.3 - Deterministic integer line primitives

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for Deterministic integer line primitives. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Deterministic integer line primitives with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Deterministic integer line primitives.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: octants, endpoints, degenerates, clipping symmetry, and golden rasters.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.92.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.92.4 - Deterministic integer circle and ellipse primitives

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for Deterministic integer circle and ellipse primitives. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Deterministic integer circle and ellipse primitives with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Deterministic integer circle and ellipse primitives.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: quadrants, degenerates, clipping symmetry, overflow, and golden rasters.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.92.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.92.5 - Bounded raster-drawing contract and security audit

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for Bounded raster-drawing contract and security audit. Its independently reviewable delta must be implemented, tested, and reviewed before later capability is admitted; pentesting follows the release cadence table.

Goal:

Complete Bounded raster-drawing contract and security audit with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Bounded raster-drawing contract and security audit.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Freeze 1.0 drawing to the named raster primitives; fonts, text
  shaping, vector paths, strokes, and antialiasing remain unclaimed.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: layout/color/alpha/overlap/work tests and explicit support matrix.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.92.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.93.0 - Optional safe SIMD or audited external backends

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
optional safe simd or audited external backends. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete optional safe simd or audited external backends with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Optional safe SIMD or audited external backends.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Keep #![forbid(unsafe_code)] in every first-party crate. Acceleration is safe
  Rust or an explicitly audited optional external backend with no transitive
  activation; disabling it preserves the scalar implementation.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Scalar differential, tail/alignment, dependency/unsafe-boundary, Miri, and sanitizer evidence.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.93.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.94.0 - Streaming and tiled processing graph

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for
streaming and tiled processing graph. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete streaming and tiled processing graph with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Streaming and tiled processing graph.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Define per-plane halo requirements and prove seam equivalence for Lanczos,
  bicubic, affine borders, chroma planes, and premultiplied-alpha filtering
  across whole-image, scanline-band, and independently scheduled tile runs.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Scratch bounds, fusion equivalence, cancellation, and honest random-access disclosure.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.94.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.94.1 - Metadata- and header-only decoding

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for metadata- and header-only decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete metadata- and header-only decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Metadata- and header-only decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: no-pixel paths, source-position policy, metadata budgets, and format matrix.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.94.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.94.2 - Region-selective decoding

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for region-selective decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete region-selective decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Region-selective decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: coordinate spaces, chroma/halo planning, committed regions, and fallbacks.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.94.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.94.3 - Frame-range selective decoding

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for frame-range selective decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete frame-range selective decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Frame-range selective decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: raw/composited selection, dependency closure, disposal state, and bombs.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.94.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.94.4 - Reduced-resolution and progressive-preview decoding

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for reduced-resolution and progressive-preview decoding. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete reduced-resolution and progressive-preview decoding with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Reduced-resolution and progressive-preview decoding.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: JPEG reduced IDCT, TIFF selection, progressive events, and numeric evidence.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.94.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.94.5 - Processing and selective-decoding contract freeze

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for processing and selective-decoding contract freeze. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete processing and selective-decoding contract freeze with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Processing and selective-decoding contract freeze.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: fusion equivalence, peak limits, cancellation, DoS, and support matrix.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.94.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.94.6 - Unified borrowed inspection and decode_into facade

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for unified borrowed inspection and decode_into facade. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete unified borrowed inspection and decode_into facade with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Unified borrowed inspection and decode_into facade.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: hints/mismatch, static dispatch, output tiers, limits, and disabled features.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.94.6 implementation stop reached. Follow the release cadence and publication rules.`

### v0.94.7 - Unified encoder and transcoding facade

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for unified encoder and transcoding facade. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete unified encoder and transcoding facade with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Unified encoder and transcoding facade.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: capability plans, conversion orchestration, metadata effects, and transactions.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.94.7 implementation stop reached. Follow the release cadence and publication rules.`

### v0.94.8 - Fallible owned APIs and facade-candidate integration audit

Status: Planned.

Context:

This is the exclusive color, metadata, processing, and facade handoff for fallible owned APIs and facade-candidate integration audit. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete fallible owned APIs and facade-candidate integration audit with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Fallible owned APIs and facade-candidate integration audit.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Establish the audited candidate baseline exercised by synchronous codecs;
  adapter-driven corrections remain open only through v0.98.8.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: allocation failure, representative codecs, feature matrix, and candidate baseline.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.94.8 implementation stop reached. Follow the release cadence and publication rules.`

## Phase: Integration and assurance

Exercise the audited facade candidate through async, WASM, parallel, GPU,
service, and CLI adapters; review and pentest any resulting facade corrections;
close all implementation and public-API issues at v0.98.8; then run fuzzing,
proofs, and audits against that exact reconciled implementation before the
verification-only final API freeze.

### v0.95.0 - Runtime-neutral async source/sink adapters

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
runtime-neutral async source/sink adapters. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete runtime-neutral async source/sink adapters with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Runtime-neutral async source/sink adapters.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Backpressure, cancellation, partial-I/O tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.95.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.95.1 - WASM/browser streaming adapters

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
wasm/browser streaming adapters. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete wasm/browser streaming adapters with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: WASM/browser streaming adapters.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: wasm32-unknown-unknown, JS-size, memory-growth tests.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.95.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.96.0 - Caller-provided parallel scheduling interface

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
caller-provided parallel scheduling interface. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete caller-provided parallel scheduling interface with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Caller-provided parallel scheduling interface.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Determinism, budget partition, cancellation.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.96.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.96.1 - Optional Rayon/service adapter

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
optional rayon/service adapter. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete optional rayon/service adapter with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Optional Rayon/service adapter.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: No core dependency or automatic global pool.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.96.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.97.0 - GPU-compatible descriptors and upload-layout hooks

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
gpu-compatible descriptors and upload-layout hooks. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete gpu-compatible descriptors and upload-layout hooks with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: GPU-compatible descriptors and upload-layout hooks.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Stable layout contract; no device ownership in core.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.97.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.97.1 - Optional backend adapters

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for
optional backend adapters. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete optional backend adapters with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Optional backend adapters.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: CPU/GPU differential results and synchronization policy.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v0.97.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.98.0 - mynd-cli inspect command

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for mynd-cli inspect command. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete mynd-cli inspect command with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: mynd-cli inspect command.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: escaped bounded metadata, stable schema, hostile terminals, and exit codes.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.98.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.98.1 - mynd-cli validate command

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for mynd-cli validate command. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete mynd-cli validate command with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: mynd-cli validate command.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: strict/compatibility modes, bounded diagnostics, exit codes, and no output mutation.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.98.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.98.2 - mynd-cli decode command

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for mynd-cli decode command. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete mynd-cli decode command with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: mynd-cli decode command.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: transactional files, output-tier/color disclosure, limits, and cancellation.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.98.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.98.3 - mynd-cli encode command

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for mynd-cli encode command. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete mynd-cli encode command with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: mynd-cli encode command.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: format capability validation, metadata policy, transactions, and determinism.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.98.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.98.4 - mynd-cli convert command

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for mynd-cli convert command. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete mynd-cli convert command with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: mynd-cli convert command.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: explicit conversion plan, information-loss confirmation, and atomic replacement.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.98.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.98.5 - mynd-cli frame command

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for mynd-cli frame command. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete mynd-cli frame command with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: mynd-cli frame command.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: raw/composited selection, frame/range limits, filenames, and animation bombs.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.98.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.98.6 - mynd-cli bounded batch profile

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for mynd-cli bounded batch profile. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete mynd-cli bounded batch profile with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: mynd-cli bounded batch profile.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: aggregate budgets, hostile filenames, collision policy, and cancellation.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.98.6 implementation stop reached. Follow the release cadence and publication rules.`

### v0.98.7 - mynd-cli bounded service profile

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for mynd-cli bounded service profile. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete mynd-cli bounded service profile with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: mynd-cli bounded service profile.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: request isolation, aggregate/live budgets, cancellation, and no ambient authority.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.98.7 implementation stop reached. Follow the release cadence and publication rules.`

### v0.98.8 - Cross-adapter facade reconciliation

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for cross-adapter facade reconciliation. Its API and attack-surface delta must be implemented, tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain unavailable or explicitly fail closed.

Goal:

Complete cross-adapter facade reconciliation with bounded behavior, explicit claims, and evidence
sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Cross-adapter facade reconciliation.
- Define contracts, invariants, limits, terminal states, errors, output
  commits, compatibility behavior, and unsupported cases before code.
- Update SPEC_MAPPING, support/source/architecture records, crate
  boundaries, corpus provenance, security documentation, and claims.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Resolve the complete async/WASM/parallel/GPU/CLI issue register on
  one facade; prohibit adapter-specific forks, bypasses, and shadow APIs.
- Record the exact source identity, dependencies, packages, claims, and
  test manifests that every v0.99.x campaign must use.
- Update changelog, notes, crate versions, packages, SBOM, and the
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Zero unresolved implementation or public-API issues and one exact assurance input.
- Audit arithmetic, offsets, terminal transitions, capability
  negotiation, cumulative/live/peak budgets, scratch, output,
  metadata, and work accounting.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack,
  code-size, numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, SBOM,
  supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; supported, unsupported, and
  limit-exceeded behavior is explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the
  exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the new surface and
  inherited invariants; all critical/high findings are fixed and cleanly retested.
- CI and CodeQL default setup are green, the permanent report records
  PASS, and the version release gate accepts the final release candidate.
- `v0.98.8 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.0 - cargo-fuzz harness and corpus integration

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for cargo-fuzz harness and corpus integration. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete cargo-fuzz harness and corpus integration with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: cargo-fuzz harness and corpus integration.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: every parser/entropy/metadata/dispatcher target builds with provenance.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.0 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.1 - Long-running fuzz and truncation campaign

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Long-running fuzz and truncation campaign. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Long-running fuzz and truncation campaign with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Long-running fuzz and truncation campaign.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: coverage report, minimized persistent corpus, and no stalls/panics.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.1 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.2 - Kani checked-arithmetic and geometry proofs

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Kani checked-arithmetic and geometry proofs. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Kani checked-arithmetic and geometry proofs with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Kani checked-arithmetic and geometry proofs.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: conversion/size/stride/rectangle assumptions and unwind bounds.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.2 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.3 - Kani view and buffer-state proofs

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Kani view and buffer-state proofs. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Kani view and buffer-state proofs with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Kani view and buffer-state proofs.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: bounds, alias policy, commit visibility, and state invariants.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.3 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.4 - Kani byte- and bit-I/O proofs

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Kani byte- and bit-I/O proofs. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Kani byte- and bit-I/O proofs with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Kani byte- and bit-I/O proofs.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: cursor/refill/shift/rollback/progress invariants.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.4 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.5 - Kani Deflate and zlib state proofs

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Kani Deflate and zlib state proofs. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Kani Deflate and zlib state proofs with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Kani Deflate and zlib state proofs.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: table/distance/window/checksum/output/progress invariants.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.5 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.6 - Kani GIF and TIFF LZW state proofs

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Kani GIF and TIFF LZW state proofs. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Kani GIF and TIFF LZW state proofs with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Kani GIF and TIFF LZW state proofs.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: dialect dictionary/width/reset/end/output/progress invariants.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.6 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.7 - Kani JPEG entropy state proofs

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Kani JPEG entropy state proofs. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Kani JPEG entropy state proofs with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Kani JPEG entropy state proofs.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Huffman/arithmetic/stuffing/restart/coefficient/progress invariants.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.7 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.8 - Kani WebP entropy state proofs

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Kani WebP entropy state proofs. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Kani WebP entropy state proofs with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Kani WebP entropy state proofs.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: VP8 Boolean and VP8L prefix/LZ/cache/output/progress invariants.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.8 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.9 - Kani TIFF fax and IFD state proofs

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Kani TIFF fax and IFD state proofs. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Kani TIFF fax and IFD state proofs with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Kani TIFF fax and IFD state proofs.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: run/transition/reference-row and graph/count/progress invariants.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.9 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.10 - Miri audit

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Miri audit. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Miri audit with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Miri audit.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: all supported feature sets and mutation/view/adapter paths pass Miri.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.10 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.11 - Sanitizer audit

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Sanitizer audit. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Sanitizer audit with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Sanitizer audit.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: address, leak, memory, and undefined-behavior sanitizer matrix.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.11 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.12 - Supported-Rust, target, and feature audit

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Supported-Rust, target, and feature audit. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Supported-Rust, target, and feature audit with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Supported-Rust, target, and feature audit.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Rust 1.90.0-1.97.1, targets, no-default, alloc/std, WASM, and combinations.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.12 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.13 - Stack and code-size audit

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Stack and code-size audit. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Stack and code-size audit with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Stack and code-size audit.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: per-target stack ceilings, recursion absence, and binary-size budgets.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.13 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.14 - Official conformance and differential freeze

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Official conformance and differential freeze. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Official conformance and differential freeze with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Official conformance and differential freeze.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: every format claim mapped with no unexplained reference disagreement.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.14 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.15 - Cross-format color conformance freeze

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Cross-format color conformance freeze. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Cross-format color conformance freeze with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Cross-format color conformance freeze.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: native/rendered profiles, precedence, tolerances, and reference vectors.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.15 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.16 - Performance and denial-of-service freeze

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Performance and denial-of-service freeze. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Performance and denial-of-service freeze with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Performance and denial-of-service freeze.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: valid throughput plus hostile rejection-time/work/memory regression limits.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.16 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.17 - Reproducible package, SBOM, and provenance freeze

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for Reproducible package, SBOM, and provenance freeze. It runs against the exact v0.98.8 input; product implementation and public API changes are prohibited. A defect returns to v0.98.8.

Goal:

Complete Reproducible package, SBOM, and provenance freeze with bounded behavior, explicit claims, and cadence-appropriate release evidence.

Deliverables:

- Complete only the release-scoped capability: Reproducible package, SBOM, and provenance freeze.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- Update release notes, crate versions, packages, SBOM, and cadence-appropriate release-evidence record.

Verification:

- Required release evidence: byte-reproducible archives, dependency identity, signatures, and attestations.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- At a pentest checkpoint, the cumulative review covers the delta and inherited invariants; critical/high findings are fixed and cleanly retested.
- CI and CodeQL default are green; at a publication checkpoint the permanent report records PASS.
- `v0.99.17 implementation stop reached. Follow the release cadence and publication rules.`

### v0.99.18 - Cumulative pentest handoff and final public API freeze

Status: Planned.

Context:

This is the exclusive integration and assurance handoff for the cumulative
pentest handoff and final public API freeze. It runs against the exact v0.98.8
input; product implementation and public API changes are prohibited. A defect
returns to v0.98.8.

Goal:

Freeze the public API and assemble the complete evidence and scope required for
the cumulative v1.0.0-rc.1 pentest without beginning that pentest here.

Deliverables:

- Complete only the release-scoped capability: Cumulative pentest handoff and
  final public API freeze.
- Define invariants, limits, states, errors, commits, compatibility, and unsupported cases.
- Update SPEC_MAPPING, source/support/architecture records, crate boundaries, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression, determinism, lifecycle, and resource tests.
- This is the verification-only final public API freeze; any product
  or API defect returns to v0.98.8 and invalidates affected evidence.
- Update release notes, crate versions, packages, SBOM, and the cumulative
  pentest handoff bundle.

Verification:

- Required release evidence: unchanged v0.98.8 input, complete evidence bundle,
  frozen scope, and no unresolved implementation defect.
- Audit arithmetic, offsets, transitions, budgets, scratch, output, metadata, and work.
- Run applicable unit, property, truncation, round-trip, differential, conformance, fuzz, proof, Miri, sanitizer, stack, size, performance, DoS, Rust, feature, and target gates.
- Run repository, supply-chain, latest-crate/tool, package, and SBOM gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Supported, unsupported, and limit-exceeded claims link to passing evidence.
- Packages, SBOM, mappings, fixtures, and notes match the exact candidate commit.
- The v1.0.0-rc.1 pentest scope covers every delta after v0.95.0 and references
  the complete evidence bundle; this handoff makes no pentest PASS claim.
- CI and CodeQL default are green; v1.0.0-rc.1 owns the permanent report.
- `v0.99.18 implementation stop reached. Follow the release cadence and publication rules.`

## Phase: Production admission

Pentest and reproduce exact candidate archives, then promote without changing bytes or claims.

### v1.0.0-rc.1 - Exact versioned production candidate

Status: Planned.

Context:

This is the exclusive production admission handoff for
exact versioned production candidate. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete exact versioned production candidate with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Exact versioned production candidate.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Freeze the production image-format matrix with farbfeld decode and encode
  admitted only from the v0.34.0 implementation and v0.35.0 cross-codec audit,
  including its official source record, exact-length tests, conformance
  evidence, limitations, and security review.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Pentest and reproduce the exact .crate archives;
  the candidate support matrix explicitly retains audited farbfeld decode and
  encode.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v1.0.0-rc.1 implementation stop reached. Follow the release cadence and publication rules.`

### v1.0.0 - Byte-for-byte promotion of the approved candidate

Status: Planned.

Context:

This is the exclusive production admission handoff for
byte-for-byte promotion of the approved candidate. Its API and attack-surface delta must be implemented,
tested, and reviewed independently; pentesting follows the release cadence table. Later capabilities remain
unavailable or explicitly fail closed.

Goal:

Complete byte-for-byte promotion of the approved candidate with bounded behavior, explicit claims, and
evidence sufficient for a release security decision.

Deliverables:

- Complete only the release-scoped capability: Byte-for-byte promotion of the approved candidate.
- Define contracts, invariants, limits, capabilities, terminal states, errors,
  output commits, compatibility, native/rendered behavior, and unsupported cases.
- Update SPEC_MAPPING, support/source/architecture records, crate boundaries,
  corpus provenance, numeric tolerances, and security documentation.
- Add positive, boundary, malformed, truncation, mutation, regression,
  determinism, lifecycle, and resource-accounting fixtures.
- Verify the byte-for-byte promoted support matrix still includes the exact
  audited farbfeld decode/encode capability accepted by v1.0.0-rc.1; promotion
  cannot add, remove, or alter its claims.
- Update changelog, notes, crate versions, packages, SBOM, and exact-version
  cadence-appropriate release-evidence record.

Verification:

- Required release evidence: Signed checksums, SBOM, provenance, and stable support matrix.
- Audit arithmetic, offsets, terminal transitions, capability negotiation,
  cumulative/live/peak budgets, typed scratch, output, metadata, and work.
- Run applicable unit, property, every-byte/bit truncation, round-trip,
  differential, conformance, fuzz, Kani, Miri, sanitizer, stack, code-size,
  numeric-tolerance, performance, and denial-of-service checks.
- Run repository, cargo-deny, cargo-audit, latest-crate/tool, and SBOM gates.
- Run supported-Rust, feature, no-default, 32-bit, WASM, and platform gates.

Exit criteria:

- The capability is complete, documented, and the only new capability.
- Claims link to passing evidence; capabilities, output tier, limitations,
  numeric tolerance, metadata effects, and compatibility are explicit.
- Packages, dependencies, SBOM, mappings, fixtures, and notes match the exact
  candidate commit.
- At a pentest checkpoint, the cumulative review covers every delta since
  the prior published checkpoint; all critical/high findings are fixed and
  cleanly retested.
- CI and CodeQL default setup are green for every GitHub tag; at a publication
  checkpoint the permanent report records PASS and the strict publication gate
  accepts the candidate.
- `v1.0.0 implementation stop reached. Follow the release cadence and publication rules.`

## Post-1.0 admission

Post-1.0 formats require their own threat-boundary review, official source
ledger, granular versions, conformance profile, corpus provenance, resource
model, and iterative pentest. They never enter a 1.0 patch merely because
they share an image category.

The first later planning pass may separately evaluate TGA, AVIF/HEIF, JPEG XL,
JPEG 2000, JPEG-LS, JPEG XR, JPEG XS, OpenEXR, Radiance HDR, ICO/CUR, PSD/PSB,
DDS, KTX, SVG/rasterization, and emerging formats without treating one family
as an extension of another.
