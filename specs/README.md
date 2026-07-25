# Specification Reference Corpus

Mynd keeps exact, checksum-locked implementation references locally. The
corpus covers the active pre-1.0 formats and shared color/metadata work, plus
manual acquisition records for the explicitly named post-1.0 TGA and JPEG XL
sources.

Mynd and its maintainers claim no copyright or ownership in any third-party
document here. Each document retains its original notices, authorship,
trademarks, status, and legal terms. Third-party references are **not** covered
by Mynd's MIT or Apache-2.0 software licenses.

## Deliberate legal split

`SOURCES.json` assigns every reference one of three dispositions:

| Disposition | Location | Git status | Meaning |
| --- | --- | --- | --- |
| `public` | `specs/public/` | tracked | Unmodified redistribution is supported by explicit publisher terms |
| `offline` | `specs/offline/files/` | ignored | Direct official download exists, but redistribution permission is absent, unclear, or too narrow |
| `manual` | `specs/offline/files/` | ignored | Purchase, login, disclaimer acceptance, or unresolved provenance prevents unattended acquisition |

Availability without a paywall is not treated as redistribution permission.
Patent promises, royalty-free implementation rights, and copyright permission
are also treated as separate questions.

Tracked material currently relies on these explicit terms:

| Publisher/source | Governing terms |
| --- | --- |
| RFC Editor publications | [IETF Trust Legal Provisions 5.0](https://trustee.ietf.org/documents/trust-legal-provisions/tlp-5/) |
| W3C technical reports | [W3C Document License 2023](https://www.w3.org/copyright/document-license-2023/) |
| MicrosoftDocs snapshots | CC BY 4.0; the exact repository license copies are included |
| QOI reference source/specification | MIT; the exact license copy is included |
| farbfeld format source | ISC; the exact upstream license page is included |
| libwebp VP8L specification source | BSD-3-Clause; the exact `COPYING` is included |

The files are kept byte-for-byte unmodified. Project annotations, clause maps,
errata decisions, and implementation notes belong outside `specs/public/`.

## Recreate and verify

From the repository root:

```sh
# Fetch or verify every public and automatically downloadable offline source.
scripts/fetch-specs.sh

# Fetch only ignored local material.
scripts/fetch-specs.sh offline

# Validate manifests, exact public file set, hashes, symlink safety, and modes.
scripts/verify-specs.sh

# Also require the complete automatically downloadable offline set.
scripts/verify-specs.sh --require-offline

# Show the remaining purchase/login/manual work.
python3 scripts/spec-sources.py status
```

Downloads use an HTTPS host allowlist, validate every redirect, enforce a
per-source byte limit, hash a temporary file, and atomically install only the
expected bytes. Existing files are never silently replaced. A changed upstream
response fails closed and requires a reviewed source update.

Manual sources must be acquired from their `acquisition_url`, placed at the
exact declared filename, and added to `MANUAL_SHA256SUMS`. The fetcher never
accepts click-through terms, supplies credentials, or automates purchases.

## Integrity and review

- `SOURCES.json` records identity, edition, role, acquisition, terms, and legal
  disposition.
- `SHA256SUMS` pins every tracked public byte sequence.
- `OFFLINE_SHA256SUMS` makes ignored direct downloads reproducible.
- `MANUAL_SHA256SUMS` locks lawfully acquired manual copies without publishing
  them.
- `scripts/lock-specs.sh` applies a local read-only guard.
- `.gitattributes` prevents text and line-ending normalization.
- `CODEOWNERS` protects source and checksum changes when branch rules enforce
  review.
- CI reconstructs the read-only bit before verification because Git does not
  preserve it portably.

The manifests and public copies are excluded from crates.io packages by each
crate's strict package allowlist. Builds and library tests never need network
access.

## Updating a source

1. Confirm the official publisher, current edition, errata, and actual
   redistribution terms. Do not infer permission from public availability.
2. Update `SOURCES.json` with an immutable/datable URL where available.
3. Run `python3 scripts/spec-sources.py candidate-locks public` or `offline`
   into a temporary review record.
4. Inspect the fetched identity, legal notices, edition, and content before
   changing the appropriate checksum manifest.
5. Remove only the specific obsolete local copy, run `scripts/fetch-specs.sh`,
   and review the replacement.
6. Update `SPEC_SOURCES.md`, requirement mappings, tests, security evidence,
   and release notes together.

Never “fix” an upstream document locally. Record publisher errata separately
or adopt a new reviewed edition.
