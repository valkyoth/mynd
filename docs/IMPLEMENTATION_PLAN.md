# mynd Implementation Plan

Status: repository foundation. No codec capability is implemented or claimed.

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

The initial 1.0 scope is BMP, TGA, and GIF. PNG, classic JPEG, and JPEG XL are
post-1.0 codec projects. Their future complexity must not destabilize the core.

## 2. Architecture

```text
mynd-cli -> mynd facade -> mynd-bmp / mynd-tga / mynd-gif
                         -> mynd-codec
                            |-> mynd-core -> mynd-math
                            |-> mynd-io
                            |-> mynd-metadata
                            |-> mynd-color
                            |-> mynd-processing -> mynd-quantize
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
defaults are chosen only after the feature audit at versions 0.99-0.100.

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
8. Update support claims, release notes, crate versions, and pentest scope.

No second capability enters the release until the first is testable and its
security delta is reviewable.

## 6. Foundation phases

### Phase A: governance and image model (0.1-0.12)

Establish repository policy, checked math, validated dimensions/layouts/views,
frames, timing, and fallible owned storage. `mynd-core` never parses formats.

Acceptance emphasis: integer extrema, zero dimensions, stride/rectangle
overflow, short buffers, failed reservation, and invalid type combinations.

### Phase B: I/O and codec contracts (0.13-0.24)

Build slice/fixed/bounded/subrange readers and writers, endian and bit I/O,
probing, limits, work budget, errors/warnings, and incremental state-machine
contracts. Freeze these contracts before the first format decoder.

Acceptance emphasis: truncation at every byte/bit, cursor overflow, nested
bounds, transactional output, bit-width transitions, probe ambiguity, budget
bypass, and zero-progress states.

## 7. Initial codec phases

### BMP (0.25-0.45)

Start with file/DIB headers and 24-bit uncompressed bottom-up decoding into a
caller-provided RGB/BGR view. Add header families, depths, palettes, bitfields,
top-down images, RLE, bounded profiles, and encoders one release at a time.

Security focus: header-size confusion, pixel offsets, signed height minimum,
row padding, palette underflow, mask overlap, RLE escapes/deltas, embedded
payloads, profile ranges, and huge dimensions with tiny inputs.

### TGA (0.46-0.60)

Add header parsing, true-color/grayscale/indexed samples, origins, bounded RLE,
ID/footer/extension/developer areas, and canonical raw/RLE encoding.

Security focus: weak probing, packet output bounds, cross-scanline policy,
color-map origins, footer offsets, thumbnail sizes, overlapping regions,
orientation, and alpha interpretation.

### GIF decoding (0.61-0.84)

Build signatures, screen/palette/sub-block parsing, LZW as its own audit unit,
deinterlacing, transparency, bounded frame composition/disposal, and extensions.

Security focus: dictionary/code-width transitions, forward references, exact
pixel count, endless sub-blocks, canvas rectangles, restore-to-previous memory,
frame totals, timing, palette indexes, and animation decompression bombs.

## 8. Encoding, facade, and stabilization

Versions 0.85-0.90 add deterministic bounded quantization and GIF encoding.
Versions 0.91-0.98 add the facade, static dispatch, owned convenience APIs,
basic color conversion, processing, and CLI.

Versions 0.99-0.111 are assurance releases: feature matrix, panic/arithmetic
audits, limit calibration, truncation automation, conformance/differential
coverage, extended fuzzing, Kani, Miri/sanitizers, performance/DoS regression,
reproducible packaging, external review, and API freeze.

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
- BMP, TGA, and GIF support matrices are honest and complete for claimed scope;
- every accepted input path is resource-bounded;
- default code has no unsafe Rust and no known critical/high security issue;
- meaningful parser states have fuzz and conformance coverage;
- proof assumptions and unsupported behavior are documented;
- packages rebuild without network-time code generation;
- supported target and Rust matrices pass;
- at least one independent security review is complete;
- maintainers accept normal semantic-versioning obligations.

Existence of version 0.111.0 does not automatically authorize 1.0.0.
