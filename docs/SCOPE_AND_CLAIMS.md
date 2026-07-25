# Mynd Scope And Claim Contract

Status: normative for project scope and public capability claims

This document is the single vocabulary for what Mynd plans, implements, and
advertises. It does not grant support by itself. Current released behavior is
recorded in [`FORMAT_SUPPORT.md`](../FORMAT_SUPPORT.md); release ordering and
exit criteria are defined by [`VERSION_PLAN.md`](VERSION_PLAN.md).

## Authority and conflict handling

The documents have distinct responsibilities:

1. `FORMAT_SUPPORT.md` states current implemented capability.
2. This document defines admitted scope, crate ownership, and claim words.
3. `VERSION_PLAN.md` defines the only pre-1.0 delivery sequence.
4. `IMPLEMENTATION_PLAN.md` defines architecture and implementation method.
5. `SPEC_SOURCES.md`, `STANDARDS_LEDGER.md`, and `specs/SOURCES.json` define
   source identity, edition, errata state, provenance, and legal disposition.
6. `POST_1_0_CODEC_PLAN.md` records candidates that are not in the 1.0 claim.

A contradiction fails the repository gate. It is not resolved by choosing the
broader claim. Until corrected and released, the narrower implemented claim in
`FORMAT_SUPPORT.md` governs.

## Pre-1.0 format scope

Each format family has a dedicated implementation crate. Crates are created
only when their first implementation release begins.

| Family | Owning crate | Delivery train | 1.0 scope |
| --- | --- | --- | --- |
| BMP/DIB versioned dialects | `mynd-bmp` | `v0.20.0-v0.25.2` | Explicitly admitted header/compression cells only |
| QOI | `mynd-qoi` | `v0.26.0-v0.27.0` | Original pinned QOI profile |
| Netpbm PNM/PAM | `mynd-netpbm` | `v0.28.0-v0.33.0` | PBM, PGM, PPM, and PAM; PFM excluded |
| farbfeld | `mynd-farbfeld` | `v0.34.0` | Decode and encode the original pinned format |
| Simple-codec integration audit | No codec crate | `v0.35.0` | Evidence-only audit over the four preceding families |
| PNG Third Edition and APNG | `mynd-png` | `v0.36.0-v0.46.4` | Declared PNG/APNG profiles |
| GIF87a/89a | `mynd-gif` | `v0.47.0-v0.51.8` | Standards plus separately named admitted extensions |
| Classic JPEG | `mynd-jpeg` | `v0.52.0-v0.62.0` | Declared ITU-T T.81 processes and named conventions |
| WebP | `mynd-webp` | `v0.63.0-v0.69.4` | RFC 9649 container, VP8, VP8L, alpha, and animation |
| TIFF 6.0 profiles | `mynd-tiff` | `v0.70.0-v0.77.14` | Declared baseline and admitted extensions; BigTIFF excluded |

TGA, PFM, BigTIFF, AVIF/HEIF, JPEG XL, JPEG 2000, JPEG-LS, JPEG XR,
JPEG XS, and every unlisted format are outside the 1.0 claim. A source record
or post-1.0 plan is not an admission.

## Capability claim vocabulary

Claims are cumulative only inside the exact named edition and profile. A later
tier never silently widens formats, metadata, color behavior, resource limits,
or compatibility modes.

| Claim | Required meaning |
| --- | --- |
| `not-implemented` | No public behavior is available. Roadmap text is not support. |
| `probe-only` | A bounded prefix can identify or reject the exact profile; no complete structural validation or pixels are promised. |
| `structural-parse` | The declared structure is validated and reported without claiming pixel reconstruction. |
| `defensive-decode` | Declared inputs decode within explicit limits, but complete profile conformance is not yet claimed. |
| `conformant-decode` | Every conforming input in the exact declared profile decodes unless an explicit resource limit is exceeded. |
| `defensive-encode` | Emission is bounded and deterministic for named options, but complete encoder-profile conformance is not yet claimed. |
| `conformant-encode` | Every emitted stream conforms to the exact named encoder profile. |
| `stable` | The admitted API and behavior have completed the production assurance and compatibility gates. |

The following terms are states, not capability tiers:

| State | Meaning |
| --- | --- |
| `unsupported` | Recognized behavior is intentionally rejected by the current exact profile. |
| `outside-scope` | The format or feature has not been admitted to this release train. |
| `blocked` | Admission is planned, but required source, legal, design, or security evidence is unresolved. |
| `limit-exceeded` | Otherwise admissible input exceeds a documented caller/project resource limit; it is not malformed. |

## Dimensions of every public claim

A support statement is incomplete unless it names:

- format family, exact specification edition, profile, and compatibility mode;
- operation and capability tier from the vocabulary above;
- input envelope and recognized dialects or processes;
- caller-visible limits and the `limit-exceeded` behavior;
- native samples, rendered output, color declarations, and alpha association;
- metadata parsed, preserved, discarded, transformed, or left opaque;
- incremental commit behavior, terminal states, and partial-output policy;
- crate version, test/conformance evidence, and unresolved exclusions.

“Supports FORMAT,” “fully compliant,” and “100% compliant” are prohibited
without those qualifiers.

## BMP dialect rule

BMP is never one Boolean capability. Claims are keyed by file envelope, exact
DIB header size/revision, dimensions, planes, bit depth, palette layout, mask
placement, compression namespace, orientation, color/profile fields, and
decode/encode tier. Unknown header sizes are `unsupported`; implementations
must not fall back to the nearest known header.

Windows core/INFO/V4/V5, de-facto 52/56-byte compatibility headers, IBM OS/2
revisions, RLE4/RLE8, OS/2 Huffman 1D/RLE24, BI_JPEG/BI_PNG, bitmap-array and
icon/pointer envelopes are separate decisions. The detailed admission matrix
is frozen by `v0.20.0` before BMP parsing begins.

## Versioning and publication

`mynd` is the integration train and uses the milestone version. A support crate
changes version only when its own published code, dependencies, or metadata
change. An unchanged support crate remains at its previous version and is not
republished merely to match `mynd`.

For `v0.2.0`, only the `mynd` facade advances to `0.2.0`; `mynd-core` remains
at `0.1.0`. This release changes governance and packaged facade documentation
only and adds no image parser, decoder, encoder, processing operation, or new
runtime dependency.

## Gate

The repository consistency check must prove:

- the family set, crate names, and version trains above match the README,
  support matrix, implementation plan, version plan, source ledger, modularity
  policy, and post-1.0 exclusions;
- every support state uses the vocabulary above;
- every source in `specs/SOURCES.json` is covered by the standards ledger;
- the source manifest conforms to `specs/SOURCES.schema.json`;
- release metadata preserves independent support-crate versions.

Any failure blocks the next implementation milestone.
