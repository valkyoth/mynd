# Format Support

Support is not a yes/no claim. Each capability advances only with documented
tests, specification mapping, conformance evidence, and security review.

| Format | Probe | Structural parse | Decode | Encode | Metadata | Status |
| --- | --- | --- | --- | --- | --- | --- |
| BMP | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned |
| TGA | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned |
| GIF87a/89a | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Planned |
| PNM, QOI, PNG, JPEG, TIFF, WebP, AVIF, JXL, J2K, JLS, JXR, JXS | Not implemented | Not implemented | Not implemented | Not implemented | Not implemented | Post-1.0 |

The support vocabulary is: `probe-only`, `structural-parse`,
`defensive-decode`, `conformant-decode`, `defensive-encode`,
`conformant-encode`, and `stable`. A codec README must state unsupported
features and policy differences before its first release.
