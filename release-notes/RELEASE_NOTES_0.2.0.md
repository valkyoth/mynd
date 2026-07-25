# mynd 0.2.0 Release Notes

Status: release candidate; pentest PASS; awaiting GitHub CI and CodeQL.

This governance-only release reconciles the exact pre-1.0 image scope, public
claim vocabulary, standards and errata ledger, corpus provenance schema, crate
ownership, and independent crate-version policy. It adds no parser, decoder,
encoder, processing behavior, feature, unsafe code, or runtime dependency.

The pre-1.0 format set is BMP/DIB versioned dialects, QOI, official Netpbm
PNM/PAM, farbfeld, PNG Third Edition/APNG, GIF87a/89a, declared classic-JPEG
profiles, WebP, and declared TIFF 6.0 profiles. TGA, PFM, BigTIFF, AVIF/HEIF,
JPEG XL, and other unlisted formats remain outside the 1.0 claim.

The new claim contract separates planned work, capability tiers, unsupported
behavior, outside-scope behavior, unresolved blockers, and resource-limit
rejection. Format-wide or unconditional “fully compliant” claims are
prohibited. BMP support is explicitly a matrix of file envelopes, exact header
revisions, depth, palette, masks, compression namespace, orientation, and color
or profile behavior; unknown versions never fall back to a nearby structure.

The standards ledger covers every ID in `specs/SOURCES.json` and distinguishes
pinned bytes from manual blockers, living errata that require review at the
implementation handoff, license evidence, and future-only sources. A checked
JSON Schema now fixes the source manifest fields and rejects unknown provenance
metadata or mismatched automatic/manual acquisition fields.

The 2026-07-25 publisher review reconfirmed PNG Third Edition and its current
errata, CIPA’s 2026 Exif 3.1 and Exif-for-XMP editions, ICC.1:2022 v4.4 plus
its April 2025 adaptive-gain amendment, CSS Color 4’s 2 May 2026 draft, and
BT.2100-3. Living status and errata endpoints still require review again at
their implementation handoffs.

This is also the first release to exercise independent workspace crate
versions. The facade crate advances from `mynd` 0.1.0 to 0.2.0 because its
packaged documentation changes. The published inputs for `mynd-core` remain
unchanged at 0.1.0, so it is not republished.

Security evidence required before release:

- governance consistency validation proves the active documents, capability
  vocabulary, source coverage, schema, and release graph agree;
- the specification source, package exclusion, documentation, supported Rust,
  platform, latest-tool, dependency, audit, and SBOM gates pass;
- package inspection proves only `mynd` 0.2.0 is selected for publication;
- `security/pentest/v0.2.0.md` records the review and reaches `Status: PASS`;
- the final committed candidate passes GitHub CI and CodeQL before tagging.
