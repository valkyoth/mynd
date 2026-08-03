# mynd Implementation Plan

Status: checked-arithmetic and validated-geometry foundation. No codec
capability is implemented or claimed.

This plan turns the final direction from the project design discussion into
small, testable, security-reviewable increments. `docs/VERSION_PLAN.md` is the
normative release sequence; this document describes how the work is organized.

## 1. Product boundary

`mynd` is a high-assurance, resource-bounded, `no_std`-first image decoding,
encoding, processing, and conversion ecosystem for Rust.

Priority order:

1. correctness;
2. bounded memory, stack, output, and CPU work;
3. auditability;
4. interoperability and conformance;
5. performance;
6. advanced encoding quality.

The 1.0 scope includes BMP, QOI, official Netpbm PNM/PAM, farbfeld, PNG/APNG,
GIF87a/89a, the declared classic-JPEG profiles, WebP, and the declared TIFF
6.0 plus admitted-extension profiles. TGA, PFM, BigTIFF, JPEG XL, and every
format not expressly admitted by `docs/VERSION_PLAN.md` remain outside 1.0.

## 2. Architecture

```text
mynd-cli -> mynd facade -> one mynd-* crate per admitted format family
                         -> mynd-processing
format/processing crates -> focused shared crates:
  mynd-codec | mynd-core | mynd-math | mynd-budget | mynd-io
  mynd-zlib -> mynd-deflate
  mynd-exif -> mynd-ifd -> mynd-metadata
  mynd-color -> mynd-icc
```

Crates are created only when their first release begins. This keeps package
claims and audit scope aligned with delivered capability.

| Crate | Responsibility | Forbidden responsibility |
| --- | --- | --- |
| `mynd` | feature-gated facade and static dispatch | format parsing and large algorithms |
| `mynd-math` | checked image-specific arithmetic | generic math or codec algorithms |
| `mynd-core` | validated dimensions, layouts, views, frames | format parsing and I/O |
| `mynd-io` | minimal byte/bit traits and bounded adapters | filesystem policy in core APIs |
| `mynd-codec` | limits, budgets, errors, probe/codec contracts | format implementations |
| `mynd-metadata` | bounded transport-neutral metadata | full Exif/ICC/XMP parsing initially |
| `mynd-ifd` | bounded typed IFD graph/value mechanics | TIFF or Exif schema policy |
| `mynd-exif` | selected Exif schema and inspection | general TIFF image decoding |
| `mynd-icc` | bounded ICC parsing and color pipelines | container parsing |
| `mynd-deflate` / `mynd-zlib` | RFC 1951 engine and RFC 1950 wrapper | GIF/TIFF LZW or container policy |
| `mynd-color` | explicit deterministic conversions | silent lossy conversion |
| `mynd-processing` | budgeted image operations | automatic threads/runtime coupling |
| `mynd-quantize` | deterministic bounded palettes | hidden randomness or float requirement |
| codec crates | one format family per audit surface | unrelated format families |

Future complex metadata uses `mynd-exif`, `mynd-icc`, `mynd-xmp`, and
`mynd-jumbf`. Classic JPEG is `mynd-jpeg`; JPEG XL is `mynd-jxl`; JPEG 2000,
JPEG-LS, JPEG XR, and JPEG XS remain separate crates.

## 3. Portability and features

Every library begins core-only:

```rust
#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;
```

Feature hierarchy:

```toml
[features]
default = []
alloc = []
std = ["alloc"]
```

- Core-only: caller-owned buffers; no heap, filesystem, or OS services.
- `alloc`: fallible owned images and bounded collections.
- `std`: I/O/error convenience adapters only; parser validation is unchanged.

Codecs eventually default to `decode` without allocation. The facade's 1.0
defaults are chosen only after the v0.99.x assurance campaign and v0.99.18
final freeze.

The target matrix is Linux, Windows, FreeBSD/BSD, macOS, Android, and iOS.
Aesynx is an architecture constraint, not a current build claim: no core API may
assume conventional processes, filesystem, threads, or global allocation.

## 4. Security invariants

All parser and algorithm APIs preserve these properties:

- malformed input cannot panic, hang, recurse without a fixed bound, or write
  outside caller output;
- input-derived addition, multiplication, alignment, conversion, offset,
  stride, rectangle, and buffer calculations use explicit checked operations;
- all accepted paths charge memory, output, metadata, frame, chunk, seek, and
  work budgets;
- a zero-progress parser state is an error unless it explicitly requests input
  or output;
- probing uses only a small bounded prefix and reports ambiguity;
- compatibility mode documents deviations but never disables safety checks;
- errors are structured, allocation-free, bounded, and distinguish malformed,
  truncated, unsupported, limit-exceeded, and internal-invariant failures;
- codecs never access network, filesystem, environment, clock, or threads;
- default first-party code contains no unsafe Rust;
- identical input and options produce deterministic results.

`DecodeLimits` includes dimensions, pixels, frames, total frame pixels, output,
metadata, preserved unknown bytes, chunks/extensions/palette entries, nesting,
seek distance, and work units. Presets change budgets only, never validation.

## 5. Implementation method

Each minor release adds one bounded capability:

1. Verify current official/original specifications and errata.
2. Update the source ledger and requirement-to-evidence mapping.
3. Define invariants, limits, errors, and negative behavior before code.
4. Implement a clear safe scalar path in a focused module under 500 lines.
5. Add unit, boundary, truncation, mutation, and property tests.
6. Add applicable corpus, round-trip, differential, fuzz, proof, and benchmark
   evidence.
7. Audit panics, arithmetic, allocations, loops, offsets, logging, and package
   contents.
8. Update support claims, release notes, crate versions, and the release's
   pentest/publication-cadence status.

No second capability enters the release until the first is testable and its
security delta is reviewable.

## 6. Foundation phases

### Phase A: governance, model, I/O, and codec contracts (0.1-0.19)

Establish repository policy, checked math, validated dimensions/layouts/views,
frames, timing, budgets, scratch, byte/bit I/O, incremental execution, metadata
transport, a reproducible standards corpus, ICC foundations including an
explicit adaptive-gain admission decision, shared bounded BCP 47 language-tag
syntax, probing, adapters, and the common codec contract. `mynd-core` never
parses formats.

Acceptance emphasis: integer extrema, zero dimensions, stride/rectangle
overflow, short buffers, failed reservation, and invalid type combinations.

### Phase B: simple and lossless codecs (0.20-0.35)

Implement the admitted BMP dialect matrix, QOI, official Netpbm PNM/PAM, and
farbfeld in separate crates, then freeze the common simple-codec contract.

Acceptance emphasis: exact dialect dispatch, truncation at every byte/bit,
checked output size, row/palette/mask boundaries, concatenated-image policy,
Netpbm transfer/linear-opacity semantics and variant policy, PFM exclusion,
deterministic encoders, and cross-codec probe ambiguity.

## 7. Codec phases

Versions 0.20-0.35 implement BMP dialects, QOI, official Netpbm PNM/PAM, and
farbfeld. BMP never uses nearest-header fallback; PFM is not included in the
Netpbm claim. Every codec has checked exact output sizing, bounded probing,
deterministic encoding, and a format-family audit.

Versions 0.36-0.46 implement PNG Third Edition and APNG. Chunk framing/order,
Deflate block types, filters/color types, color metadata, ancillary policy,
APNG sequencing/composition, encoders, and the final audit are separate
handoffs.

Versions 0.47-0.51 implement GIF87a/89a plus explicitly named de-facto
extensions. LZW, frame composition, palette generation, still/animated
encoding, and the compatibility/security audit remain independently testable.

Versions 0.52-0.62 implement the declared ITU-T T.81 classic-JPEG profiles.
Marker/declaration state, entropy/restart accounting, reconstruction processes,
metadata/color conventions, and each encoder-process admission are separate.
Decoder completeness never silently claims every encoder process.

Versions 0.63-0.69 implement RFC 9649 WebP, VP8, VP8L, alpha, animation, and
encoders. Container, entropy, reconstruction, transforms, and animation each
own bounded work and output evidence.

Versions 0.70-0.77 implement the declared TIFF 6.0 and extension profile.
IFD mechanics and schema policy are separate. Compression dialects, predictors,
layout modes, sample domains, orientation, calibrated color, corrected
JPEG-in-TIFF, Exif integration, encoders, and the final profile audit have
explicit support matrices. The TIFF-referenced T.4/T.6 editions are pinned.
BigTIFF remains outside 1.0.

## 8. Encoding, facade, and stabilization

Versions 0.78-0.94 add shared metadata/color interpretation, an Exif 3.1 profile
freeze, separately bounded XMP packet/XML/RDF layers, conversion, geometry,
resampling, compositing, bounded raster-drawing primitives, processing graphs,
selective decoding, and the audited facade candidate.

Versions 0.95-0.98.7 exercise async, WASM, parallel, GPU, CLI, batch, and
service boundaries. Version 0.98.8 resolves every adapter-driven facade issue
and fixes the exact implementation admitted to assurance.

Versions 0.99.0-0.99.18 are evidence-only campaigns over that unchanged input:
fuzzing/truncation, focused Kani proof families, Miri/sanitizers/target/feature
coverage, conformance/differential/color/performance/DoS freeze, reproducible
packaging, preparation for the cumulative `v1.0.0-rc.1` external pentest, and
the final public API freeze.

The CLI is a separate `std` tool and never silently chooses unlimited budgets.
It writes outputs transactionally so failed conversion does not leave a result
that looks complete.

## 9. Test architecture

Every public behavior receives the layers applicable to its risk:

- focused unit and table-driven boundary tests;
- construction tests proving invalid states are rejected;
- every-byte and every-bit truncation for small valid inputs;
- mutations of signatures, lengths, offsets, dimensions, compression, palettes,
  terminators, frame rectangles, and codes;
- golden outputs with dimensions, formats, metadata, frames, warnings, hashes,
  and policy results;
- lossless round trips and encoder determinism;
- chunk-boundary equivalence between slice and incremental APIs;
- independent differential decoding with disagreement classification;
- official conformance suites where licensing allows;
- fuzz targets for probes, headers, bounded/streaming decode, policies, encode,
  round-trip, metadata, and cross-codec dispatch;
- Kani proofs for arithmetic, rectangles, bits, RLE, LZW, palette indexes, and
  row/interlace calculations;
- Miri and sanitizers for views, slicing, mutable output, and future unsafe code;
- realistic and hostile-input performance, peak memory, stack, code-size, and
  rejection-time regression tests.

Corpus files require origin, license, format, expected result, decoded hash,
specification relevance, and redistribution status. Unlicensed corpora and
copyrighted standards do not enter the public repository.

## 10. Dependency strategy

Published runtime crates depend only on `core`, optional `alloc`/`std`, and
audited `mynd-*` crates. Development tools such as cargo-fuzz, Kani, Miri,
sanitizers, coverage, reference decoders, and corpus minimizers remain outside
the runtime graph.

If an external integration is needed, follow the `sanitization` pattern: put it
in a small optional adapter crate, disable default features, and keep the core
crate independent. Every candidate is checked for the latest release and
reviewed under `docs/supply-chain-security.md`.

## 11. Documentation per codec

Each format crate ships `README.md`, `SECURITY.md`, `FORMAT_SUPPORT.md`,
`SPEC_MAPPING.md`, `LIMITS.md`, `COMPATIBILITY.md`, `FUZZING.md`, and
`CONFORMANCE.md`. Requirement entries map original identifiers/short summaries
to implementation modules, tests, fuzz targets, proofs, and current status.

Release notes list new parser states, loops, allocations, offsets, conversions,
metadata behavior, specification clauses, compatibility behavior, corpus,
fuzz/proof additions, known unsupported features, and residual security limits.

## 12. 1.0 admission

Version 1.0.0 is published only when:

- core and no-allocation APIs are stable;
- every admitted BMP, QOI, PNM/PAM, farbfeld, PNG/APNG, GIF, classic-JPEG,
  WebP, and TIFF profile has an honest, complete support matrix;
- every accepted input path is resource-bounded;
- default code has no unsafe Rust and no known critical/high security issue;
- meaningful parser states have fuzz and conformance coverage;
- proof assumptions and unsupported behavior are documented;
- packages rebuild without network-time code generation;
- supported target and Rust matrices pass;
- at least one independent security review is complete;
- maintainers accept normal semantic-versioning obligations.

Existence of v0.99.18 does not automatically authorize 1.0.0. The exact
v1.0.0-rc.1 archives must pass the cumulative pentest and reproducibility
decision covering every delta since v0.95.0. Version v1.0.0 is a byte-for-byte
promotion only; any implementation or artifact change requires another
appropriate review and retest.
