# Security Policy

`mynd` parses hostile binary input. Treat parsers, byte and bit readers,
decompression, frame composition, metadata, color conversion, allocation,
resource limits, dependencies, CI, and releases as security-sensitive.

## Current status

Version 0.1.0 is a repository foundation and contains no image decoder or
encoder. It is not production-ready. Support claims in `FORMAT_SUPPORT.md`
must remain evidence-based and fail closed.

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

Every release requires a matching release note and pentest report. A release
tag must point at the final report commit, and the report must say `Status:
PASS`. The implementation commit and report commit remain a linear pair; do
not rewrite or squash between review and tagging.

Required evidence grows with capability: formatting, lints, MSRV/current Rust,
feature combinations, target builds, unit and documentation tests, package
inspection, dependency policy, truncation/malformed corpora, fuzzing, proofs,
Miri, differential/conformance results, and hostile-input regression tests.

## Supported versions

Before 1.0, security fixes normally target the latest release line. Once 1.0
is published, this section will list maintained stable and backport lines.
