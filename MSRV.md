# Minimum Supported Rust Version

`mynd` supports stable Rust 1.90.0 through 1.97.1.

- `workspace.package.rust-version` is `1.90` and may not be raised casually.
- `rust-toolchain.toml` pins 1.97.1 for development and release verification.
- Every stable release in the supported interval is checked before a `mynd`
  release.
- Normal builds never require nightly Rust. Nightly-only fuzzing, Miri, or
  model-checking tools are development evidence and do not affect MSRV.

Raising MSRV requires a planned release, compatibility impact analysis,
documentation, and explicit release notes.
