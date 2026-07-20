# mynd Release Runbook

1. Select one bounded milestone from `docs/VERSION_PLAN.md`.
2. Verify current official sources, tool versions, and dependency versions.
3. Implement with focused tests and update support/specification mappings.
4. Run `scripts/checks.sh`, the Rust matrix, target builds, and applicable
   conformance, truncation, fuzz, Kani, Miri, sanitizer, and regression suites.
5. Update `CHANGELOG.md`, `release-notes/`, `release-crates.toml`, crate versions,
   and the crate version matrix.
6. Inspect every `cargo package` archive and dependency tree.
7. Freeze the reviewed implementation commit.
8. Write `security/pentest/vX.Y.Z.md` as its direct linear child; unresolved
   critical/high issues prevent `PASS`.
9. Run `scripts/validate-release-metadata.sh vX.Y.Z` and the release gate.
10. Tag the report commit, then use `scripts/release_crates.py --require-tag` to
    publish in dependency order.

Patch releases contain only bug/security/documentation/test corrections. Minor
pre-1.0 releases add one bounded capability. Never republish unchanged support
crates merely to match the facade version.
