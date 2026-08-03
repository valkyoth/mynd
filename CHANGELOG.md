# Changelog

All notable changes are documented here. The project follows semantic
versioning, while pre-1.0 releases intentionally add one bounded capability at
a time.

## [Unreleased]

## [0.5.0] - Unreleased

### Added

- Added validated integer and floating sample-storage domains with explicit
  significant width, physical unit, byte order, and packed-stream bit order.
- Added typed gray, RGB, and YCbCr layouts with explicit channel ordering,
  alpha association, six chroma-subsampling domains, and interleaved, planar,
  or semi-planar organization.
- Added exact logical-plane dimensions, checked tightly packed row-byte
  derivation, and concrete plane relationship validation.
- Added focused boundary and relationship tests, five Kani storage/layout
  harnesses, facade coverage, and the normative pixel-storage contract.

### Changed

- Advanced `mynd` to 0.5.0 and `mynd-core` to 0.3.0; unchanged `mynd-math`
  remains at 0.1.0 and is not republished.
- Re-exported the pixel-storage foundation through `mynd::core` without adding
  allocation, buffer access, parsing, decoding, encoding, or format support.

### Security

- Invalid sample width/unit/class states and contradictory
  plane/chroma/alpha organizations cannot become validated public values.
- Concrete planes fail closed on count, sampled-height, exact used-row-byte,
  ordering, or overlap mismatch before an output extent is committed.
- Added the cumulative v0.5.0 pentest scope covering the complete delta since
  the prior published v0.4.0 checkpoint.

## [0.4.0] - 2026-08-03

### Added

- Added `mynd-core` 0.2.0 validated nonzero dimensions with exact fixed-width
  pixel counts and explicit target-width conversion.
- Added retained-bounds nonempty rectangles with checked exclusive ends and
  exact areas.
- Added aligned nonempty plane layouts with exact last-row output extents,
  ordered nonoverlapping plane-set validation, and nonzero output lengths.
- Added boundary, error-order, overlap, 32-bit compilation, and facade tests,
  plus Kani proofs with explicitly documented full and reduced domains.
- Added the normative geometry/layout contract and expanded the platform gate
  with 32-bit Linux and WASM targets.

### Changed

- Advanced the `mynd` facade to 0.4.0 and `mynd-core` to 0.2.0; unchanged
  `mynd-math` remains at 0.1.0 and is not republished.
- Re-exported the validated geometry API through `mynd::core` while retaining
  the checked arithmetic API through `mynd::math`.

### Security

- Geometry constructors reject zero extents, containment overflow, invalid
  strides, numeric misalignment, plane overlap, arithmetic overflow, and
  target-width truncation before returning usable values.
- Added exact-version v0.4.0 pentest scope for validated geometry, target-width
  output arithmetic, facade exposure, and crate-publication boundaries.
- Remediated the pentest's Low misleading-error finding by eliminating the
  generic output-length fallback and constructing the final nonzero extent
  directly from validated, checked target-width values.
- Completed the v0.4.0 pentest remediation loop with a green external retest
  and no unresolved finding.

## [0.3.0] - 2026-08-03

### Added

- Added the dependency-free `mynd-math` 0.1.0 support crate with explicit
  checked integer conversions, `u64`/`usize` addition and multiplication,
  upward alignment, bounded half-open ranges, and structured errors.
- Added extrema cross-products, exhaustive reduced-domain alignment/range
  tests, facade re-export coverage, and 12 Kani arithmetic proof harnesses.
- Added a requirement-to-implementation/test/proof contract and a dedicated
  Kani CI job using the current pinned verifier.

### Changed

- Advanced the `mynd` facade to 0.3.0 and re-exported `mynd-math` as
  `mynd::math`; `mynd-core` remains unchanged at 0.1.0.
- Defined the v0.5.0-and-later release cadence: every planned version remains a
  green GitHub tag, while cumulative pentests and crates.io publication occur
  at each fifth minor checkpoint (`v0.5.0`, `v0.10.0`, `v0.15.0`, and so on).
- Defined interim tags as unpentested, unpublished engineering checkpoints,
  added cumulative pentest scope, and retained an emergency security-release
  exception for affected published artifacts.

### Security

- Checked operations never wrap, saturate, truncate, allocate, or include
  attacker-controlled operands in errors; alignment zero and range overflow
  are distinct failures.
- Added exact-version v0.3.0 pentest scope for the new public arithmetic and
  crate-publication surface.
- Completed the v0.3.0 pentest and remediation loop with a green retest, no
  exploitable finding, and both informational observations documented.

## [0.2.1] - 2026-07-27

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
- Remediated the follow-up Medium stale-legal-approval finding by binding
  approval to a canonical digest of every public provenance field and locked
  source hash, with a coupled provenance/content mutation regression test.
- Completed the v0.2.1 pentest and remediation loop with a green final retest
  and no unresolved Critical, High, Medium, or Low finding.

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

[Unreleased]: https://github.com/valkyoth/mynd/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/valkyoth/mynd/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/valkyoth/mynd/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/valkyoth/mynd/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/valkyoth/mynd/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/valkyoth/mynd/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/valkyoth/mynd/releases/tag/v0.1.0
