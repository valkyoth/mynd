# Specification Sources

`mynd` does not implement formats from memory. Each codec milestone must verify
the current official/original source, record exact edition or revision,
document clauses used, and add conformance evidence. This public ledger stores
metadata and links, not copyrighted standards text.

## Initial formats

| Family | Primary source | Edition/status | Repository status |
| --- | --- | --- | --- |
| BMP/DIB | Microsoft [Bitmap Storage](https://learn.microsoft.com/en-us/windows/win32/gdi/bitmap-storage), [Bitmap Header Types](https://learn.microsoft.com/en-us/windows/win32/gdi/bitmap-header-types), `BITMAPCOREHEADER`, `BITMAPINFOHEADER`, `BITMAPV4HEADER`, `BITMAPV5HEADER`, compression pages, applicable Microsoft Open Specifications, and original IBM OS/2 bitmap documentation for every admitted OS/2 dialect | Living Microsoft GDI/Open Specifications plus exact IBM editions; URLs, revisions, and document hashes captured per dialect milestone | Microsoft sources are approved public anchors; OS/2 variants and de-facto 52/56-byte headers remain blocked until primary-source provenance and an explicit admission decision are recorded |
| TGA | Truevision TGA File Format Specification 2.0, Technical Manual 2.2, January 1991 | Original publisher manual; surviving copies are mirrors | Provenance-sensitive; implementation blocked until a local copy is hashed and reviewed |
| GIF87a | CompuServe Graphics Interchange Format 87a | Original 1987 specification | Source copy and hash required before GIF work |
| GIF89a | [CompuServe GIF89a specification hosted by W3C](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) | 31 July 1990; extends GIF87a | Approved public anchor; local evidence hash required before GIF work |
| farbfeld | [suckless farbfeld format definition](https://tools.suckless.org/farbfeld/) | Original public format definition; exact snapshot captured at the implementation milestone | Approved public source; requirement map and source snapshot required before `v0.34.0` |

## Future formats

| Family | Primary source family | Admission rule |
| --- | --- | --- |
| PNG/APNG | [W3C PNG Third Edition](https://www.w3.org/TR/png-3/), Recommendation 24 June 2025 | Pin the dated Recommendation and errata when `mynd-png` begins |
| Classic JPEG | [ITU-T T.81](https://www.itu.int/rec/T-REC-T.81/en), ISO/IEC 10918, and applicable JFIF/Exif/ICC sources | Record purchased/private materials by hash only; never commit protected text |
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
