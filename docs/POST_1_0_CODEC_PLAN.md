# Post-1.0 Codec Plan

This roadmap preserves future format decisions without expanding the Mynd 1.0
attack surface. Every codec begins as an independently versioned experimental
crate and joins facade defaults only after its own conformance and security
admission.

Recommended first review order: TGA, BigTIFF, JPEG-LS, JPEG XL, AVIF/HEIF,
JPEG 2000, OpenEXR, Radiance HDR, ICO/CUR, PSD/PSB, DDS/KTX, then specialist
families. This is not an implementation promise; each family needs a fresh
source, threat-boundary, maintenance-cost, and ecosystem-value decision.

## Naming and boundaries

- `mynd-jpeg`: classic JPEG under ITU-T T.81 / ISO/IEC 10918.
- `mynd-jxl`: JPEG XL under ISO/IEC 18181.
- `mynd-j2k`, `mynd-jls`, `mynd-jxr`, `mynd-jxs`: separate format families.
- Never publish `mynd-jpeg-xl`.
- JPEG XL remains one public crate; its entropy, modular, VarDCT, container,
  color, reconstruction, frame, and feature areas are internal modules.

Experimental codecs cannot force stable shared crates back to 0.x or require
users of unrelated formats to compile their implementation.

## Already-admitted 1.0 families

BMP, QOI, PNM/PAM, farbfeld, PNG/APNG, GIF, classic JPEG, TIFF/Exif, and WebP
are admitted to the 1.0 train and are not post-1.0 candidates. Their normative
facade handoffs, codec boundaries, and evidence gates live only in
`docs/VERSION_PLAN.md`; this file must not assign a second or conflicting codec
train.

## mynd-jxl internal structure

One public `mynd-jxl` crate contains focused modules for:

- container signatures, boxes, sizes, metadata, and JPEG reconstruction;
- bit readers/writers, JXL integers, and prefixes;
- entropy tables, distributions, prefix coding, and ANS;
- image, animation, preview, extra-channel, and color headers;
- frame groups, passes, progressive state, and composition;
- modular channels, trees, predictors, palettes, transforms, and decode;
- VarDCT low/high frequency, strategies, coefficients, dequantization, and
  scalar inverse transforms;
- patches, splines, noise, filters, color, and legacy JPEG reconstruction;
- decode and encode orchestration.

These are modules, not public crates, until independent reuse is proven.

## mynd-jxl train

| Codec version | Capability |
| --- | --- |
| 0.1.0 | Standards ledger, clause map, threat model, feature matrix |
| 0.2.0 | Signature and bounded container probe |
| 0.3.0 | Generic 32/64-bit box sizes |
| 0.4.0 | Unknown-box skip and bounded preservation |
| 0.5.0 | Raw/containerized codestream dispatch |
| 0.6.0 | Basic image header |
| 0.7.0 | Extra-channel declarations and limits |
| 0.8.0 | Animation, preview, frame declarations |
| 0.9.0 | Color-encoding declarations |
| 0.10.0 | JXL bitstream integer primitives |
| 0.11.0 | Prefix-code parsing and validation |
| 0.12.0 | Entropy-table validation |
| 0.13.0 | Bounded ANS state machinery |
| 0.14.0 | Entropy-distribution decoding |
| 0.15.0 | Frame header and group layout |
| 0.16.0 | Modular channel representation |
| 0.17.0 | Modular decision trees and predictors |
| 0.18.0 | Modular transforms and palette handling |
| 0.19.0 | Basic lossless modular frame decode |
| 0.20.0 | Multi-group modular decode with work budgets |
| 0.21.0 | VarDCT low-frequency parsing |
| 0.22.0 | Transform-strategy validation |
| 0.23.0 | High-frequency coefficient decode |
| 0.24.0 | Dequantization |
| 0.25.0 | Scalar inverse transforms |
| 0.26.0 | Upsampling and reconstruction filters |
| 0.27.0 | Extra channels and alpha output |
| 0.28.0 | Patches, splines, noise, optional-feature limits |
| 0.29.0 | Frame composition and animation |
| 0.30.0 | Progressive and partial-decode API |
| 0.31.0 | Metadata boxes and preservation policy |
| 0.32.0 | Legacy JPEG reconstruction-data parsing |
| 0.33.0 | Exact legacy JPEG reconstruction |
| 0.34.0 | Lossless modular encoder |
| 0.35.0 | Initial VarDCT encoder |
| 0.36.0 | Encoder effort/quality controls |
| 0.37.0 | Streaming groups and caller-scheduled work units |
| 0.38.0 | Official Part 3 conformance material |
| 0.39.0 | Reference-software cross-check |
| 0.40.0 | Extended fuzz, proof, memory, CPU audit |
| 0.41.0 | Decoder stable candidate |
| 0.42.0+ | Encoder-quality work without decoder destabilization |

Stable JPEG XL support is impossible without ISO/IEC 18181 Part 3 conformance
material. Purchased ISO documents remain private; the repository records only
hashes, original clause notes, tests, and redistribution-safe evidence.
