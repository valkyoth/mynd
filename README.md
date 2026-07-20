# mynd

Security-first, resource-bounded, `no_std` image codecs and processing for Rust.

`mynd` is being built in small, independently auditable releases. The current
`0.1.0` repository foundation contains the facade and core crate skeletons; it
does **not** yet decode or encode image files. Support claims remain fail-closed
until code, conformance evidence, fuzzing, and release security review exist.

## Project goals

- no panics or unchecked arithmetic from malformed input;
- caller-controlled memory and work budgets;
- `core`-only operation by default, with explicit `alloc` and `std` layers;
- no unsafe Rust in default first-party implementations;
- no third-party runtime dependencies in published codec crates;
- separate crates for stable responsibilities and codec security surfaces;
- deterministic decoding and encoding for identical inputs and options;
- Linux, Windows, BSD, macOS, Android, and iOS support from the first code
  release, without architecture choices that block future Aesynx support.

## Rust compatibility

The minimum supported Rust version (MSRV) is 1.90.0. Development and release
gates use the latest stable patch, currently 1.97.1. Every stable toolchain in
the supported interval is checked before release.

| Rust | Role | Expected status |
| --- | --- | --- |
| 1.90.0 | MSRV | Supported |
| 1.91.0 | Compatibility | Supported |
| 1.92.0 | Compatibility | Supported |
| 1.93.0 | Compatibility | Supported |
| 1.94.0 | Compatibility | Supported |
| 1.95.0 | Compatibility | Supported |
| 1.96.0 | Compatibility | Supported |
| 1.96.1 | Compatibility patch | Supported |
| 1.97.0 | Compatibility | Supported |
| 1.97.1 | Pinned development and release toolchain | Supported |

See [MSRV.md](MSRV.md) and the [toolchain policy](docs/toolchain-policy.md).

## Workspace

The root is a virtual Cargo workspace. Only crates needed by the current
milestone are created; future crates are added when their first auditable
release begins.

| Crate | Current role | Status |
| --- | --- | --- |
| `mynd` | User-facing facade | Skeleton |
| `mynd-core` | Format-neutral image types | Skeleton |
| `mynd-math` | Checked image arithmetic | Planned for 0.2.0 |
| `mynd-io` | Minimal byte and bit I/O | Planned |
| `mynd-codec` | Limits, errors, policies, codec contracts | Planned |
| `mynd-color` | Deterministic color conversion | Planned |
| `mynd-metadata` | Bounded metadata transport | Planned |
| `mynd-processing` | Budgeted image operations | Planned |
| `mynd-quantize` | Deterministic palette generation | Planned |
| `mynd-bmp`, `mynd-tga`, `mynd-gif` | Initial codec security surfaces | Planned |

The split is governed by stable responsibility and independent audit surface,
not file count alone. No non-generated code file may exceed 500 lines.

## Build and verify

```sh
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-features
scripts/checks.sh
```

Core-only verification is explicit:

```sh
cargo check --workspace --no-default-features
```

## Security and specifications

Image bytes are hostile input. Security policy, disclosure instructions, and
release gates are in [SECURITY.md](SECURITY.md). The threat model is in
[docs/threat-model.md](docs/threat-model.md).

Implementation must begin from current official or original format material,
not memory or secondary tutorials. Active and future sources are recorded in
[SPEC_SOURCES.md](SPEC_SOURCES.md); claimed support is recorded separately in
[FORMAT_SUPPORT.md](FORMAT_SUPPORT.md).

## Plans

- [Implementation plan](docs/IMPLEMENTATION_PLAN.md)
- [Version plan](docs/VERSION_PLAN.md)
- [Post-1.0 codec plan](docs/POST_1_0_CODEC_PLAN.md)
- [Architecture and modularity](docs/modularity-policy.md)
- [Platform support](docs/platform-support.md)
- [Release runbook](docs/release-runbook.md)

## License

Licensed under either the Apache License, Version 2.0 or the MIT License, at
your option. See [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT).
