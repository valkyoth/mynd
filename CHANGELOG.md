# Changelog

All notable changes are documented here. The project follows semantic
versioning, while pre-1.0 releases intentionally add one bounded capability at
a time.

## [Unreleased]

## [0.2.1] - Unreleased

### Added

- Exact, machine-checked legal-disposition review covering every tracked
  third-party specification and its attribution requirements.
- Isolated clean-destination reconstruction gate for all public and
  automatically downloadable offline sources.
- Mutation tests for source bytes, manifest structure, unknown private files,
  manual-fetch exclusion, and changed upstream responses.

### Changed

- Specification verification now rejects every unknown entry in the shared
  ignored offline/manual directory as well as the tracked public directory.
- Corpus policy and provenance documentation now bind tracked approval to
  exact unmodified bytes and require review again after any relevant change.

### Security

- Downloads remain allow-listed, size-bounded, checksum-locked, and atomically
  installed only after full verification; manual acquisitions remain excluded
  from automation.
- Added a permanent v0.2.1 pentest scaffold for the corpus reconstruction and
  legal-disposition boundary.
- Remediated the pentest's Low inconsistent-filename-validation finding by
  making the legal checker consume only the canonical validated source model
  and adding a traversal-shaped filename regression test.

## [0.2.0] - 2026-07-25

### Added

- Normative scope and claim contract defining exact pre-1.0 codec families,
  dedicated crate ownership, capability vocabulary, exclusions, and claim
  qualifiers.
- Standards and errata ledger with exact coverage of every source-manifest ID,
  implementation blockers, and official living review endpoints.
- JSON Schema for the specification provenance manifest plus dependency-free
  schema drift and source-coverage validation.
- Checksum-locked specification corpus with a legally reviewed tracked/offline
  split, secure reproducible fetcher, manual acquisition ledger, and CI
  integrity tests.
- Release-plan source-coverage audit adding independent gates for corpus
  reproducibility, ICC adaptive gain, BCP 47, JPEG APPn/COM policy, TIFF
  orientation/calibrated color, Exif 3.1, and separate XMP packet/XML/RDF
  layers.
- Pinned XML, Namespaces, RDF/XML, BCP 47, and TIFF T.4/T.6 dependencies plus
  a manual ISO 21496-1:2025 acquisition record.

### Changed

- Reconciled README, format-support, implementation, post-1.0, specification,
  modularity, and version-plan scope around the same BMP-through-TIFF 1.0
  format set, including farbfeld at v0.34.0.
- Corrected the simple-codec release range to begin at v0.20.0.
- Separated `mynd-core` from the facade version train so unchanged support
  crates are not republished; only `mynd` advances to 0.2.0.

### Security

- Completed the v0.2.0 pentest with no exploitable or unresolved
  Critical/High/Medium/Low finding.
- Recorded and accepted one Informational local check-then-open observation
  within the maintainer-controlled specification-tool trust boundary.

## [0.1.0] - 2026-07-25

### Added

- Virtual Cargo workspace with `mynd` and `mynd-core` skeleton crates.
- Dependency-free, `no_std`, unsafe-forbidden baseline.
- Rust 1.90.0 MSRV and Rust 1.97.1 development pin.
- Cross-platform and full supported-toolchain CI matrices.
- Security, specification, modularity, supply-chain, and release policies.
- Local authoritative source workflow with immutable public references and
  ignored reproducible copies of redistribution-restricted documents.
- Staged implementation and version plans through the 1.0 admission gate.
- Adapted independent-crate release planner from the `eth` workflow.
- Iterative pentest-report and green-CI release loop without a report-only
  commit requirement.
- MIT and Apache-2.0 license texts in every published crate archive.

### Security

- Release overflow checks and aborting panic strategy.
- Denied unknown registries, unknown git sources, wildcards, and duplicate
  dependency versions.
- Source line limit and first-party dependency-boundary checks.
- Clean-checkout specification tests that preserve the ignored offline-source
  boundary without requiring private files in CI.
- Correctly scoped workspace panic/overflow profiles and documented downstream
  consumer control of Cargo profiles.
- Explicit crates.io version-and-lockfile trust boundary for CI security-tool
  installation.

[Unreleased]: https://github.com/valkyoth/mynd/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/valkyoth/mynd/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/valkyoth/mynd/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/valkyoth/mynd/releases/tag/v0.1.0
