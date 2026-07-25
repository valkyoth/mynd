# Specification Sources

`mynd` does not implement formats from memory. Each codec milestone must verify
the current official/original source, record exact edition or revision,
document clauses used, and add conformance evidence. This public ledger stores
metadata and links, not copyrighted standards text.

## Initial formats

| Family | Primary source | Edition/status | Repository status |
| --- | --- | --- | --- |
| BMP/DIB | Microsoft [Bitmap Storage](https://learn.microsoft.com/en-us/windows/win32/gdi/bitmap-storage), [Bitmap Header Types](https://learn.microsoft.com/en-us/windows/win32/gdi/bitmap-header-types), `BITMAPCOREHEADER`, `BITMAPINFOHEADER`, `BITMAPV4HEADER`, `BITMAPV5HEADER`, compression pages, applicable Microsoft Open Specifications, and original IBM OS/2 bitmap documentation for every admitted OS/2 dialect | Living Microsoft GDI/Open Specifications plus exact IBM editions; URLs, revisions, and document hashes captured per dialect milestone | Microsoft sources are approved public anchors; OS/2 variants and de-facto 52/56-byte headers remain blocked until primary-source provenance and an explicit admission decision are recorded |
| QOI | Author's [QOI specification](https://phoboslab.org/log/2021/12/qoi-specification) | Original public specification; exact snapshot pinned at implementation | Approved public anchor; requirement map and snapshot required before `v0.26.0` |
| Netpbm PNM/PAM | Official [PNM](https://netpbm.sourceforge.net/doc/pnm.html) and [PAM](https://netpbm.sourceforge.net/doc/pam.html) documentation | Living upstream documentation; exact snapshots pinned at implementation | Approved public anchors; PFM is explicitly not part of the official Netpbm claim |
| GIF87a | CompuServe Graphics Interchange Format 87a | Original 1987 specification | Source copy and hash required before GIF work |
| GIF89a | [CompuServe GIF89a specification hosted by W3C](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) | 31 July 1990; extends GIF87a | Approved public anchor; local evidence hash required before GIF work |
| farbfeld | [suckless farbfeld format definition](https://tools.suckless.org/farbfeld/) | Original public format definition; exact snapshot captured at the implementation milestone | Approved public source; requirement map and source snapshot required before `v0.34.0` |
| PNG/APNG | [W3C PNG Third Edition](https://www.w3.org/TR/png-3/), dated Recommendation and errata; RFC 1950 and RFC 1951 | Recommendation 24 June 2025 plus current published errata | Approved public anchors; exact snapshots and clause maps required before `v0.36.0` |
| Classic JPEG | [ITU-T T.81](https://www.itu.int/rec/T-REC-T.81/en), corrigenda, T.871/JFIF, T.86 APPn markers, Exif, ICC, and admitted Adobe conventions | Each source is independently versioned and scoped | Public anchors recorded; purchased/private materials are hash-only and never committed |
| WebP | [RFC 9649](https://www.rfc-editor.org/rfc/rfc9649.html), [RFC 6386](https://www.rfc-editor.org/rfc/rfc6386.html), and the [VP8L bitstream specification](https://developers.google.com/speed/webp/docs/webp_lossless_bitstream_specification) | RFC 9649 container specification plus separately scoped VP8/VP8L sources | Approved public anchors; exact mapping required before `v0.63.0` |
| TIFF | [TIFF 6.0](https://www.itu.int/itudoc/itu-t/com16/tiff-fx/docs/tiff6.pdf), Adobe PageMaker/Photoshop Technical Notes, corrected JPEG-in-TIFF rules, and Adobe Technical Note 3 for floating-point Predictor 3 | Baseline and every extension are separately identified | Approved TIFF 6.0 anchor; extension provenance/profile decision required before its handoff; BigTIFF remains outside 1.0 |

## Future formats

| Family | Primary source family | Admission rule |
| --- | --- | --- |
| TGA | Truevision TGA File Format Specification 2.0 and Technical Manual 2.2 | Outside 1.0; provenance-sensitive and blocked until a separate admission plan pins a lawful source |
| JPEG XL | ISO/IEC 18181 Parts 1-3 and official conformance material | Verify exact editions and license before `mynd-jxl` begins |
| Other JPEG families | Their separate ITU-T/ISO families | Never treat J2K, JLS, JXR, JXS, or JXL as versions of classic JPEG |

## Required record per active source

- title, edition/version, publisher, and normative/informative status;
- canonical acquisition URL or private acquisition record;
- SHA-256 of the exact local document or source snapshot;
- redistribution status and corpus/fixture licenses;
- implemented and unimplemented clauses;
- ambiguities, errata, compatibility decisions, and conformance material.

ISO and other protected publications remain outside the public repository.
Original implementation notes may cite clauses but must not reproduce large
normative passages.
