# Format Support

Support is not a yes/no claim. Each capability advances only with documented
tests, specification mapping, conformance evidence, and security review.

| Format | Probe | Structural parse | Decode | Encode | Metadata | Status |
| --- | --- | --- | --- | --- | --- | --- |
| BMP/DIB versioned dialects | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned pre-1.0 (`v0.20.0-v0.25.2`); each header/compression combination is claimed separately |
| QOI | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned pre-1.0 (`v0.26.0-v0.27.0`) |
| Netpbm PNM/PAM | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned pre-1.0 (`v0.28.0-v0.33.0`); PFM is not included |
| farbfeld | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned pre-1.0 (`v0.34.0`) |
| PNG Third Edition / APNG | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned pre-1.0 (`v0.36.0-v0.46.4`) |
| GIF87a/89a | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned pre-1.0 (`v0.47.0-v0.51.8`) |
| Classic JPEG declared T.81 profiles | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned pre-1.0 (`v0.52.0-v0.62.0`) |
| WebP | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned pre-1.0 (`v0.63.0-v0.69.4`) |
| TIFF 6.0 declared profiles | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned pre-1.0 (`v0.70.0-v0.77.14`); BigTIFF is not included |
| TGA, PFM, BigTIFF, AVIF/HEIF, JXL, J2K, JLS, JXR, JXS, and other unlisted formats | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Outside the 1.0 claim; separate post-1.0 admission required |

The support vocabulary is: `probe-only`, `structural-parse`,
`defensive-decode`, `conformant-decode`, `defensive-encode`,
`conformant-encode`, and `stable`. A codec README must state unsupported
features and policy differences before its first release.
