# mynd Release Runbook

1. Select one bounded milestone from `docs/VERSION_PLAN.md`.
2. Verify current official sources, tool versions, and dependency versions.
3. Implement with focused tests and update support/specification mappings.
4. Run `scripts/checks.sh`, the Rust matrix, target builds, and applicable
   conformance, truncation, fuzz, Kani, Miri, sanitizer, and regression suites.
5. Update `CHANGELOG.md`, `release-notes/`, `release-crates.toml`, crate versions,
   and the crate version matrix.
6. Inspect every `cargo package` archive and dependency tree.
7. Commit the candidate and run the pentest. Keep the existing
   `security/pentest/vX.Y.Z.md` report updated with the result.
8. If the pentest finds anything, record it, fix it, retest, and repeat in the
   same report until the result is `Status: PASS`.
9. Commit the complete candidate and PASS report, then wait for GitHub CI and
   CodeQL. If either fails, fix the problem, record the correction and relevant
   retest in the report, commit again, and wait for green results.
10. Run the strict release gate against the final green commit, tag that commit,
    then use `scripts/release_crates.py --require-tag` to publish in dependency
    order.

Patch releases contain only bug/security/documentation/test corrections. Minor
pre-1.0 releases add one bounded capability. Never republish unchanged support
crates merely to match the facade version.
