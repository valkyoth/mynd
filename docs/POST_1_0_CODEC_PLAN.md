# Post-1.0 Codec Plan

This roadmap preserves future format decisions without expanding the Mynd 1.0
attack surface. Every codec begins as an independently versioned experimental
crate and joins facade defaults only after its own conformance and security
admission.

Recommended order: PNM, QOI, PNG/APNG, classic JPEG, TIFF/Exif, WebP, JPEG-LS,
JPEG XL, AVIF/HEIF, JPEG 2000, then specialist families.

## Naming and boundaries

- `mynd-jpeg`: classic JPEG under ITU-T T.81 / ISO/IEC 10918.
- `mynd-jxl`: JPEG XL under ISO/IEC 18181.
- `mynd-j2k`, `mynd-jls`, `mynd-jxr`, `mynd-jxs`: separate format families.
- Never publish `mynd-jpeg-xl`.
- JPEG XL remains one public crate; its entropy, modular, VarDCT, container,
  color, reconstruction, frame, and feature areas are internal modules.

Experimental codecs cannot force stable shared crates back to 0.x or require
users of unrelated formats to compile their implementation.

## mynd-png train

`mynd-png` is one public codec. `mynd-deflate` and `mynd-zlib` are extracted
only if they become genuinely reusable, independently auditable components.

| Codec version | Capability |
| --- | --- |
| 0.1.0 | Signature and chunk-header parser |
| 0.2.0 | IHDR validation |
| 0.3.0 | Bounded chunk state machine |
| 0.4.0 | CRC implementation and validation |
| 0.5.0 | zlib wrapper header and checksum |
| 0.6.0 | DEFLATE stored blocks |
| 0.7.0 | Fixed Huffman blocks |
| 0.8.0 | Dynamic Huffman table parsing |
| 0.9.0 | Complete bounded DEFLATE decoder |
| 0.10.0 | PNG row-filter reconstruction |
| 0.11.0 | Grayscale color type |
| 0.12.0 | True-color type |
| 0.13.0 | Indexed color |
| 0.14.0 | Grayscale-alpha |
| 0.15.0 | True-color alpha |
| 0.16.0 | Packed bit depths |
| 0.17.0 | 16-bit samples |
| 0.18.0 | Transparency chunks |
| 0.19.0 | Color and gamma chunks |
| 0.20.0 | Text and bounded ancillary metadata |
| 0.21.0 | Unknown critical/ancillary chunk policy |
| 0.22.0 | Adam7 interlace |
| 0.23.0 | Basic encoder and row filters |
| 0.24.0 | DEFLATE encoder |
| 0.25.0 | APNG parsing |
| 0.26.0 | APNG composition |
| 0.27.0 | APNG encoding |
| 0.28.0 | Streaming decode/encode audit |
| 0.29.0 | Conformance and differential audit |
| 0.30.0 | Stable-candidate release |

The active source begins with the dated W3C PNG Third Edition Recommendation
and its errata, then records every additional normative zlib/DEFLATE, color,
metadata, and APNG source before implementation.

## mynd-jpeg train

Classic JPEG remains separate from JPEG-LS, JPEG 2000, and JPEG XL. Planned
modules are marker, segment, tables, huffman, arithmetic, scan, MCU,
coefficients, DCT/IDCT, upsample, color, progressive, lossless, restart,
metadata, decode, and encode.

Each numbered item becomes at least one codec minor release; complex items may
split further after specification review:

1. Marker and segment parser.
2. Standalone and quantization tables.
3. Huffman tables and validation.
4. Frame and scan headers.
5. Entropy byte stuffing and restart markers.
6. MCU layout and coefficient storage limits.
7. Baseline Huffman coefficient decode.
8. Clear scalar integer IDCT reference.
9. Grayscale baseline decode.
10. Three-component baseline decode.
11. Chroma subsampling and upsampling.
12. Explicit YCbCr-to-RGB conversion.
13. APP0/JFIF handling.
14. APP1/Exif transport boundary.
15. APP2/ICC bounded chunk assembly.
16. Adobe marker and color interpretation.
17. Progressive DC scans.
18. Progressive AC scans.
19. Successive approximation.
20. Progressive coefficient and work limits.
21. Lossless process support where admitted.
22. Arithmetic-coded process investigation and scope decision.
23. Baseline encoder and quantization controls.
24. Huffman optimization.
25. Progressive encoder.
26. Metadata preservation.
27. Official conformance, differential testing, fuzzing, and stable-candidate
    security audit.

The scalar coefficient and pixel pipelines stay separately testable. SIMD is
considered only after the scalar result is stable and fully differential-tested.

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
