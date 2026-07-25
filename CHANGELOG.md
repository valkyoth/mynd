# Changelog

All notable changes are documented here. The project follows semantic
versioning, while pre-1.0 releases intentionally add one bounded capability at
a time.

## [Unreleased]

### Added

- Checksum-locked specification corpus with a legally reviewed tracked/offline
  split, secure reproducible fetcher, manual acquisition ledger, and CI
  integrity tests.
- Release-plan source-coverage audit adding independent gates for corpus
  reproducibility, ICC adaptive gain, BCP 47, JPEG APPn/COM policy, TIFF
  orientation/calibrated color, Exif 3.1, and separate XMP packet/XML/RDF
  layers.
- Pinned XML, Namespaces, RDF/XML, BCP 47, and TIFF T.4/T.6 dependencies plus
  a manual ISO 21496-1:2025 acquisition record.

## [0.1.0] - Unreleased

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

### Security

- Release overflow checks and aborting panic strategy.
- Denied unknown registries, unknown git sources, wildcards, and duplicate
  dependency versions.
- Source line limit and first-party dependency-boundary checks.

[Unreleased]: https://github.com/valkyoth/mynd/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/valkyoth/mynd/releases/tag/v0.1.0
