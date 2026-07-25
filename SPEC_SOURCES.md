# Specification Sources

`mynd` does not implement formats from memory. Each codec milestone verifies
the current official/original source, records its exact edition or revision,
maps the clauses used, and adds conformance evidence.

This is the human-readable scope ledger. The complete machine-readable
acquisition, terms, legal disposition, and filename ledger is
[`specs/SOURCES.json`](specs/SOURCES.json); operating instructions are in
[`specs/README.md`](specs/README.md).

## Initial formats

| Family | Primary source | Edition/status | Repository status |
| --- | --- | --- | --- |
| BMP/DIB | Microsoft [Bitmap Storage](https://learn.microsoft.com/en-us/windows/win32/gdi/bitmap-storage), header/compression and structure documentation, applicable Microsoft Open Specifications, and original IBM OS/2 documentation for every admitted OS/2 dialect | Commit-pinned MicrosoftDocs source under CC BY 4.0 plus exact IBM editions | Microsoft source and license evidence are tracked; OS/2 variants and de-facto 52/56-byte headers are manual and blocked until primary provenance and admission are recorded |
| QOI | Author's [QOI specification/reference](https://github.com/phoboslab/qoi) | Commit-pinned MIT-licensed `qoi.h` specification and license | Tracked public copy; requirement map required before `v0.26.0` |
| Netpbm PNM/PAM | Official [PNM](https://netpbm.sourceforge.net/doc/pnm.html), PBM, PGM, PPM, and [PAM](https://netpbm.sourceforge.net/doc/pam.html) documentation | Living upstream documentation snapshots | Checksum-locked offline copies; PFM is explicitly outside the official Netpbm claim |
| GIF87a | [CompuServe GIF87a specification hosted by W3C](https://www.w3.org/Graphics/GIF/spec-gif87.txt) | 15 June 1987 | Checksum-locked offline copy; underlying CompuServe redistribution permission was not inferred |
| GIF89a | [CompuServe GIF89a specification hosted by W3C](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) | 31 July 1990; extends GIF87a | Checksum-locked offline copy; underlying CompuServe redistribution permission was not inferred |
| farbfeld | [suckless farbfeld definition](https://tools.suckless.org/farbfeld/) | Original ISC-licensed format definition and license snapshot | Tracked public copy; requirement map required before `v0.34.0` |
| PNG/APNG | [W3C PNG Third Edition](https://www.w3.org/TR/png-3/), dated Recommendation and errata, RFC 1950, RFC 1951, and ISO/IEC 15948:2004 | Recommendation 24 June 2025, errata snapshot, and immutable RFC text | W3C/RFC sources are tracked; ISO publication is manual/private; clause maps required before `v0.36.0` |
| Classic JPEG | [ITU-T T.81](https://www.itu.int/rec/T-REC-T.81/en), corrigenda, T.871/JFIF, T.86 APPn markers, Exif, ICC, and admitted Adobe conventions | Each source is independently versioned and scoped; Exif baseline is CIPA Exif 3.1 (2026) | T.871 is reproducible offline; paid, acceptance-gated, and Adobe material is manual/private and hash-only |
| WebP | [RFC 9649](https://www.rfc-editor.org/rfc/rfc9649.html), [RFC 6386](https://www.rfc-editor.org/rfc/rfc6386.html), and the [VP8L source specification](https://github.com/webmproject/libwebp/blob/main/doc/webp-lossless-bitstream-spec.txt) | Immutable RFC text plus a commit-pinned BSD-licensed libwebp specification | Tracked public copies; exact mapping required before `v0.63.0` |
| TIFF | [TIFF 6.0](https://www.itu.int/itudoc/itu-t/com16/tiff-fx/docs/tiff6.pdf), its referenced 1988 T.4/T.6 fax editions, Adobe PageMaker/Photoshop Technical Notes, corrected JPEG-in-TIFF rules, and Adobe Technical Note 3 for floating-point Predictor 3 | Baseline and every extension are separately identified | TIFF 6.0 and T.4/T.6 are checksum-locked offline; Adobe extension notes are manual/private until exact provenance is admitted; BigTIFF remains outside 1.0 |
| Shared color/blending | ICC.1:2001-04, ICC.1:2022/v4.4 plus the 2025 adaptive-gain amendment and ISO 21496-1:2025 dependency, IEC sRGB, BT.601/709/2020, H.273, BT.2100, CIE sources, Porter-Duff, W3C CSS Color 4, and Compositing/Blending 1 | Every edition, patent/admission decision, and claim scope is separate | W3C sources are tracked; ICC/ITU sources are reproducible offline; ISO/IEC/CIE/ACM sources are manual/private |
| Metadata | CIPA Exif 3.1, CIPA Exif-for-XMP 2026, Adobe XMP Parts 1-3, W3C XML 1.0 Fifth Edition, Namespaces in XML 1.0 Third Edition, RDF 1.1 XML Syntax, and RFC 5646/4647 BCP 47 | Current admitted editions as of July 2026 | Dated W3C Recommendations, XML/Namespaces errata, and RFCs are tracked; the publisher-blocked living RDF errata page is manual; Adobe XMP is checksum-locked offline; CIPA sources remain private |

## Future formats

| Family | Primary source family | Admission rule |
| --- | --- | --- |
| TGA | Truevision TGA File Format Specification 2.0 and Technical Manual 2.2 | Outside 1.0; a manual record exists, but provenance remains blocked until a separate admission plan pins a lawful source |
| JPEG XL | ISO/IEC 18181 Parts 1-3 and official conformance material | Manual/private records exist; verify current editions and licenses before `mynd-jxl` begins |
| Other JPEG families | Their separate ITU-T/ISO families | Never treat J2K, JLS, JXR, JXS, or JXL as versions of classic JPEG |

## Required record per active source

- title, edition/version, publisher, and normative/informative status;
- canonical acquisition URL or private acquisition record;
- SHA-256 of the exact local document or source snapshot;
- redistribution status and corpus/fixture licenses;
- every directly or transitively normative dependency and its admission state;
- implemented and unimplemented clauses;
- ambiguities, errata, compatibility decisions, and conformance material.

ISO and other protected publications remain outside the public repository.
Original implementation notes may cite clauses but must not reproduce large
normative passages. Run `scripts/fetch-specs.sh` to recreate public and
direct-download offline evidence; it never automates purchases, credentials,
or click-through acceptance.
