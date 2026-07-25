# mynd 0.1.0 Release Notes

Release date: 2026-07-25.

This release establishes the virtual workspace, facade/core package boundaries,
dual license, `no_std` and unsafe-forbidden baseline, toolchain/platform CI,
security policies, source ledger, and staged release plans. It contains no
image decoder or encoder and makes no format-support claim.

The source ledger is backed by a checksum-locked local reference corpus.
Explicitly redistributable documents are tracked without changing their legal
notices; restricted or unclear documents remain ignored and reproducible from
official sources, while purchase/login/acceptance-gated standards use manual
acquisition records.

The initial corpus also pins the transitive XML, Namespaces, RDF/XML, BCP 47,
and TIFF fax references found during the release-plan coverage audit. Protected
ISO 21496-1:2025 remains a manual private acquisition record.

Release security uses one evolving pentest report. Findings, fixes, retests,
and any CI corrections are recorded until the report reaches PASS. The final
green commit containing that report is the tag target; a separate report-only
commit is not required.

Workspace builds enable overflow checks and workspace release builds abort on
panic. Applications consuming Mynd choose their own Cargo profiles, so library
claims rely on checked arithmetic, panic-free source, forbidden unsafe code,
and release verification rather than profile propagation. CI security tools
use immutable crates.io versions with their published lockfiles; GitHub Actions
remain pinned by full commit SHA.

Security evidence required before release:

- all repository checks pass on Rust 1.90.0 and 1.97.1;
- the full supported Rust matrix compiles and tests;
- every initial target build passes;
- package contents, bundled MIT/Apache-2.0 texts, and zero third-party runtime
  dependencies are verified;
- `security/pentest/v0.1.0.md` records a clean final retest and `Status: PASS`.
