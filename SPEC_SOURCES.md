# Specification Sources

`mynd` does not implement formats from memory. Each codec milestone must verify
the current official/original source, record exact edition or revision,
document clauses used, and add conformance evidence. This public ledger stores
metadata and links, not copyrighted standards text.

## Initial formats

| Family | Primary source | Edition/status | Repository status |
| --- | --- | --- | --- |
| BMP/DIB | [Microsoft Bitmap Storage](https://learn.microsoft.com/en-us/windows/win32/gdi/bitmap-storage), structure pages, and applicable Microsoft Open Specifications | Living Microsoft documentation; exact page revisions captured per milestone | Approved public source; clause map pending |
| TGA | Truevision TGA File Format Specification 2.0, Technical Manual 2.2, January 1991 | Original publisher manual; surviving copies are mirrors | Provenance-sensitive; implementation blocked until a local copy is hashed and reviewed |
| GIF87a | CompuServe Graphics Interchange Format 87a | Original 1987 specification | Source copy and hash required before GIF work |
| GIF89a | [CompuServe GIF89a specification hosted by W3C](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) | 31 July 1990; extends GIF87a | Approved public anchor; local evidence hash required before GIF work |

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
