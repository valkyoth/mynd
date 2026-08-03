# Security Policy

`mynd` parses hostile binary input. Treat parsers, byte and bit readers,
decompression, frame composition, metadata, color conversion, allocation,
resource limits, dependencies, CI, and releases as security-sensitive.

## Current status

Version 0.5.0 provides checked arithmetic, validated geometry and plane
layouts, and explicit sample-storage and pixel-layout domains. It contains no
image decoder or encoder and is not production-ready. Support claims in
`FORMAT_SUPPORT.md` must remain evidence-based and fail closed.

## Reporting

Do not open a public issue containing exploitable details. Use a private GitHub
security advisory once repository security channels are enabled, or contact
the maintainers privately. Include affected versions, target, feature set,
resource limits, a minimal reproducer, and expected impact when safe to do so.

## Baseline rules

- No panics, hangs, unbounded loops, unchecked arithmetic, or out-of-bounds
  writes from malformed input.
- Every input-derived allocation and expensive operation is limit-accounted.
- First-party runtime crates use `#![forbid(unsafe_code)]`.
- Published runtime crates use only `core`, optional `alloc`/`std`, and audited
  first-party `mynd-*` crates unless an explicit exception is approved.
- Codecs do not access the network, filesystem, environment, clock, or threads.
- Compatibility mode never disables bounds, arithmetic, state, or work checks.
- Errors and logs never include unbounded input-derived text.

## Consumer build profiles

The workspace enables overflow checks in its own development, test, and
release profiles and uses abort-on-panic for its own release builds. Cargo does
not propagate those profile settings when `mynd` is embedded as a dependency;
the downstream application controls its panic and overflow behavior.

Mynd's library guarantee therefore comes from checked arithmetic, panic-free
source, forbidden unsafe code, and the lint, test, review, and release gates.
It does not depend on a consumer selecting `panic = "abort"`.

## Routine checks

```sh
scripts/checks.sh
scripts/check_latest_tools.sh
cargo deny check
cargo audit
```

GitHub CodeQL default setup must be enabled in repository settings. Do not add
an advanced CodeQL workflow while default setup is active. See
[GitHub security settings](docs/github-security-settings.md).

## Release gate

Every planned version is committed, passes the complete applicable local gate,
waits for green GitHub CI and CodeQL, and receives a GitHub tag. From v0.5.0
onward, permanent pentest reports and crates.io publication occur only at the
cumulative checkpoints v0.5.0, v0.10.0, v0.15.0, and each following fifth
minor through v0.95.0. Interim tags carry their complete delta forward and
must not contain placeholder pentest reports or publish crates.

A publication checkpoint keeps one matching report in
`security/pentest/vX.Y.Z.md`. It covers the complete delta since the preceding
published checkpoint, accumulates findings, fixes, CI corrections, and
retests, and must end at `Status: PASS`. There is no separate report-only
commit requirement.

After a checkpoint report reaches PASS, commit the complete candidate and wait
for GitHub CI and CodeQL. A failure returns to the same fix, document, commit,
and verify loop. Every release tag points at the final commit only after its
local release gate, GitHub CI, and CodeQL are green. A critical/high defect in
an already published artifact may use a documented, fully pentested emergency
publication outside the normal cadence.

Required evidence grows with capability: formatting, lints, MSRV/current Rust,
feature combinations, target builds, unit and documentation tests, package
inspection, dependency policy, truncation/malformed corpora, fuzzing, proofs,
Miri, differential/conformance results, and hostile-input regression tests.

## Supported versions

Before 1.0, security fixes normally target the latest release line. Once 1.0
is published, this section will list maintained stable and backport lines.
