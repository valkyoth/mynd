# Mynd Standards And Errata Ledger

Status: v0.2.0 scope ledger

Last publisher review: 2026-07-25

This ledger connects every record in [`specs/SOURCES.json`](../specs/SOURCES.json)
to a project scope and makes unresolved editions, provenance, corrigenda, and
errata visible before implementation. A listed source is not an implementation
or support claim. Legal disposition and byte identity remain authoritative in
the source manifest and checksum files.

## State vocabulary

| State | Meaning |
| --- | --- |
| Pinned | Exact reviewed bytes and identity are checksum-locked. |
| Manual-blocked | Acquisition, provenance, edition, or legal review requires a human; dependent implementation is blocked. |
| Review-at-handoff | A living errata/status endpoint must be rechecked and recorded by the named implementation release. |
| Future | Recorded for post-1.0 investigation and not admitted to the 1.0 claim. |
| License evidence | Legal evidence for a tracked source, not a normative format requirement. |

## Baselines and blockers

| Scope | Baseline decision | Errata/status decision | Earliest gate |
| --- | --- | --- | --- |
| Normative language | RFC 2119 and RFC 8174 are pinned. | RFC Editor status and errata are reviewed when requirement language is mapped. | v0.2.1 |
| BMP/DIB | Microsoft Win32/SDK snapshots and MS-WMF 18.0 are pinned. IBM OS/2 editions and primary 52/56-byte provenance are manual-blocked. | Every admitted header/compression cell remains blocked until those primary records and publisher status are resolved. | v0.20.0 |
| QOI | The author’s reference/specification is commit-pinned with MIT evidence. | Upstream revision history is rechecked before the first parser. | v0.26.0 |
| Netpbm | Living PBM/PGM/PPM/PAM documentation snapshots are pinned offline. | The living pages are rechecked and relocked before tokenizer work; PFM remains outside scope. | v0.28.0 |
| farbfeld | The original format definition and ISC license page are pinned. | Upstream identity is rechecked before the combined decoder/encoder handoff. | v0.34.0 |
| PNG/APNG | W3C PNG Third Edition, dated 24 June 2025, and its errata snapshot are pinned; ISO/IEC 15948:2004 is manual-blocked. | The W3C errata page currently records a proposed 10 September 2025 mDCV example correction and must be rechecked before implementation. | v0.36.0 |
| Deflate/zlib | RFC 1950 and RFC 1951 are pinned. | Official RFC Editor errata for both RFCs are review-at-handoff. | v0.38.0 |
| GIF | GIF87a and GIF89a are pinned offline without inferring redistribution permission. | Publisher provenance and admitted de-facto extensions are frozen before parsing. | v0.47.0 |
| Classic JPEG | T.871 is pinned offline. T.81, its 2004 corrigendum, T.86, and Adobe APP14 conventions are manual-blocked. | Exact editions, corrigenda, and application-marker conventions must all be admitted before structure work. | v0.52.0 |
| WebP | RFC 9649, RFC 6386, and a commit-pinned VP8L source specification are pinned. | Official RFC Editor errata and the VP8L upstream revision are review-at-handoff. | v0.63.0 |
| TIFF | TIFF 6.0 and its referenced 1988 T.4/T.6 editions are pinned offline. Adobe extension notes and Predictor 3 provenance are manual-blocked. | Baseline errata, extension editions, corrected JPEG-in-TIFF rules, and each admitted compression dialect are frozen separately. | v0.70.0 |
| Shared color | ICC v2, ICC v4.4, the 2025 adaptive-gain amendment, BT.601/709/2020/2100, H.273, CSS Color 4, and W3C compositing are pinned. IEC sRGB, ISO 21496-1:2025, CIE editions, and Porter-Duff are manual-blocked. | Edition, amendment, patent, corrigenda, numeric-domain, and tolerance decisions remain separate gates. | v0.5.2 |
| Exif/XMP/XML/RDF | XML, Namespaces, RDF/XML, BCP 47, and XMP Parts 1-3 are pinned. Exif 3.1, Exif-for-XMP 2026, and the living RDF errata snapshot are manual-blocked. | XML and Namespaces errata are pinned; RDF errata and CIPA editions must be acquired and reviewed before their respective parsers. | v0.15.9 / v0.80.0 / v0.81.0 |
| Post-1.0 formats | TGA 2.0/2.2 and JPEG XL Parts 1-3 have manual acquisition records only. | All are Future and cannot widen the 1.0 claim. | Separate admission |

## Official living errata/status endpoints

These URLs are review inputs, not checksum locks:

- PNG Third Edition:
  <https://www.w3.org/2025/06/REC-PNG-20250624-errata>
- RFC 1950: <https://www.rfc-editor.org/errata/rfc1950>
- RFC 1951: <https://www.rfc-editor.org/errata/rfc1951>
- RFC 2119: <https://www.rfc-editor.org/errata/rfc2119>
- RFC 4647: <https://www.rfc-editor.org/errata/rfc4647>
- RFC 5646: <https://www.rfc-editor.org/errata/rfc5646>
- RFC 6386: <https://www.rfc-editor.org/errata/rfc6386>
- RFC 8174: <https://www.rfc-editor.org/errata/rfc8174>
- RFC 9649: <https://www.rfc-editor.org/errata/rfc9649>
- XML 1.0 Fifth Edition: <https://www.w3.org/XML/xml-V10-5e-errata>
- Namespaces in XML 1.0 Third Edition:
  <https://www.w3.org/XML/2009/xml-names-errata>
- RDF 1.1: <https://www.w3.org/2014/rdf1.1-errata>
- CIPA standards: <https://www.cipa.jp/e/std/std-sec.html>

An implementation handoff records the review date, applicable accepted or held
errata, interpretation, tests, and any source-lock update. Silence is never
interpreted as “no errata.”

## Manifest coverage

The repository check extracts the `source:` tokens below and requires exact
coverage of every ID in `specs/SOURCES.json`. An ID may have a normative,
supplemental, future, or license-evidence role; all still require provenance.

| Scope | Manifest source IDs |
| --- | --- |
| Requirement language | `source:rfc2119`, `source:rfc8174` |
| BCP 47 | `source:rfc4647`, `source:rfc5646` |
| Deflate/zlib | `source:rfc1950`, `source:rfc1951` |
| PNG/APNG | `source:png3`, `source:png3-errata`, `source:iso-png` |
| W3C color/compositing | `source:compositing1`, `source:css-color4` |
| XML/RDF | `source:xml10-fifth`, `source:xml10-fifth-errata`, `source:xml-names10-third`, `source:xml-names10-third-errata`, `source:rdf11-xml`, `source:rdf11-errata` |
| WebP | `source:rfc6386`, `source:rfc9649`, `source:vp8l`, `source:libwebp-license` |
| BMP/DIB | `source:ms-win32-license`, `source:bmp-storage`, `source:bmp-header-types`, `source:bmp-compression`, `source:ms-sdk-api-license`, `source:bmp-file-header`, `source:bmp-core-header`, `source:bmp-info-header`, `source:bmp-v4-header`, `source:bmp-v5-header`, `source:bmp-rgbquad`, `source:bmp-rgbtriple`, `source:ms-wmf`, `source:bmp-os2`, `source:bmp-compat-headers` |
| QOI | `source:qoi-spec`, `source:qoi-license` |
| farbfeld | `source:farbfeld-format`, `source:farbfeld-license` |
| Netpbm | `source:netpbm-pnm`, `source:netpbm-pbm`, `source:netpbm-pgm`, `source:netpbm-ppm`, `source:netpbm-pam` |
| GIF | `source:gif87a`, `source:gif89a` |
| TIFF | `source:tiff6`, `source:itu-t4-1988`, `source:itu-t6-1988`, `source:tiff-extension-notes`, `source:tiff-predictor3` |
| ICC and video color | `source:icc-v2`, `source:icc-v4`, `source:icc-v4-adgc`, `source:bt601`, `source:bt709`, `source:bt2020`, `source:bt2100`, `source:h273`, `source:iec-srgb`, `source:iso-gain-map`, `source:cie-colorimetry` |
| XMP/Exif/compositing | `source:xmp-part1`, `source:xmp-part2`, `source:xmp-part3`, `source:exif31`, `source:exif-xmp-2026`, `source:porter-duff` |
| Classic JPEG | `source:t871`, `source:jpeg-t81`, `source:jpeg-t81-corrigendum`, `source:jpeg-t86`, `source:adobe-jpeg-app14` |
| Future formats | `source:tga20`, `source:jxl-part1`, `source:jxl-part2`, `source:jxl-part3` |

## Release rule

Before a parser milestone starts, every baseline and transitive normative
dependency in its row must be Pinned or explicitly rejected from that profile.
Manual-blocked material cannot be replaced with a blog, secondary summary,
memory, or majority implementation behavior. Source or errata changes require
a new reviewed lock, mapping update, tests, release note, and security review.
