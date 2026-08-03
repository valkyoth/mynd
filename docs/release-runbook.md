# mynd Release Runbook

Every named version completes steps 1-6:

1. Select one bounded milestone from `docs/VERSION_PLAN.md`.
2. Verify current official sources, tool versions, and dependency versions.
3. Implement with focused tests and update support/specification mappings.
4. Run `scripts/checks.sh`, the Rust matrix, target builds, and applicable
   conformance, truncation, fuzz, Kani, Miri, sanitizer, and regression suites.
5. Update `CHANGELOG.md`, `release-notes/`, `release-crates.toml`, crate versions,
   and the crate version matrix.
6. Inspect every `cargo package` archive and dependency tree.

Run `scripts/release-gate.sh X.Y.Z` for the complete local gate. Release
metadata derives whether that version is an engineering or publication
checkpoint and fails if its pentest or publication state does not match the
authoritative cadence. `release-crates.toml` separately records the previous
GitHub-tag version and latest crates.io-published version for every crate.

Then follow the milestone's Pentest and Crates.io columns.

## GitHub engineering checkpoint

For a row marked `Pentest: No` and `Crates.io: Not published`:

1. State in its release notes that the tag is not externally pentested or
   published, and name the next cumulative checkpoint. Use these exact
   machine-checked fields, replacing the checkpoint version as needed:

   ```text
   Release kind: GitHub engineering checkpoint
   External pentest: No; cumulative review at v0.10.0
   Crates.io: Not published; next publication is mynd 0.10.0
   ```
2. Commit the complete candidate and wait for GitHub CI and CodeQL.
3. Fix any failure, rerun the affected local checks, commit, and repeat.
4. Tag the final green commit. Do not create a permanent pentest report and do
   not run `scripts/release_crates.py`.

## Cumulative publication checkpoint

For a row marked `Pentest: Yes` and `Crates.io: Publish`:

1. Pentest the complete delta since the previous published checkpoint. Keep
   `security/pentest/vX.Y.Z.md` updated with the result. Its release notes use
   `Release kind: Cumulative publication checkpoint`.
2. Record every finding, fix it, retest, and repeat in the same report until it
   reaches `Status: PASS`.
3. Commit the complete candidate and PASS report, then wait for GitHub CI and
   CodeQL. If either fails, fix the problem, record the correction and relevant
   retest in the report, commit again, and wait for green results.
4. Run the strict publication gate against the final green commit, tag that
   commit, then use `scripts/release_crates.py --require-tag` to publish changed
   or carried-forward crates in dependency order. The script refuses an
   engineering checkpoint and verifies that every selected crate's exact
   first-party dependencies are already published or selected earlier.

Critical/high security fixes for an already published artifact may use an
out-of-cadence pentest and patch publication. Document the exception in the
release notes and pentest report, set the release metadata kind to `emergency`
with a non-empty reason, and use
`Release kind: Emergency publication checkpoint` in the release notes.

Patch releases contain only bug/security/documentation/test corrections. Minor
pre-1.0 releases add one bounded capability. Every named version is committed
and tagged on GitHub. Never republish unchanged support crates merely to match
the facade version.
