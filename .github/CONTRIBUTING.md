# Contributing to mynd

`mynd` handles hostile binary input. Contributions must keep the workspace
small, explicit, testable, resource-bounded, and honest about support status.

## License

By contributing, you agree that your contribution is licensed under either
MIT or Apache-2.0, at the user's option.

## Development setup

Use the pinned Rust toolchain from `rust-toolchain.toml`.

```sh
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-features
scripts/checks.sh
```

## Security-sensitive changes

Treat these areas as high risk:

- input-derived lengths, offsets, strides, dimensions, and allocations;
- byte/bit readers, parser states, decompression, RLE, and LZW;
- frame composition, metadata preservation, and color conversion;
- resource limits and work accounting;
- unsafe code, dependencies, CI, packaging, and release scripts.

Every behavior change needs focused unit tests plus the applicable truncation,
mutation, round-trip, differential, fuzz, or proof evidence. Never post
exploitable private details in a public issue; follow
[SECURITY.md](../SECURITY.md).

## Dependencies

Published runtime crates should remain first-party and dependency-free. Any
exception requires latest-version, license, maintenance, feature, `no_std`,
native-code, build-script, and attack-surface review. Git dependencies require
an exact `rev` and a documented exception.
