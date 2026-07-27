<p align="center">
  <b>Security-first <code>no_std</code> image decoding, encoding, processing, and conversion in Rust.</b><br>
  Built in small, auditable releases with explicit resource limits and capability gates on the path to Mynd 1.0.
</p>

<div align="center">
  <a href="https://crates.io/crates/mynd">Crates.io</a>
  |
  <a href="https://docs.rs/mynd">Docs.rs</a>
  |
  <a href="https://github.com/valkyoth/mynd/blob/main/docs/VERSION_PLAN.md">Release Plan</a>
  |
  <a href="https://github.com/valkyoth/mynd/blob/main/docs/threat-model.md">Threat Model</a>
  |
  <a href="https://github.com/valkyoth/mynd/blob/main/SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <a href="https://github.com/valkyoth/mynd">
    <img src="https://raw.githubusercontent.com/valkyoth/mynd/main/.github/images/mynd.webp" alt="mynd Rust image crate overview">
  </a>
</p>

# mynd

`mynd` is a security-first, cross-platform Rust image crate designed for bounded
resource use, deterministic behavior, and deployment from embedded `no_std`
systems through desktop and server applications.

The project is in its current 0.2.1 specification-corpus phase. The workspace,
security policy, verification gates, crate boundaries, scope contract, and
standards ledger exist; image decoding and encoding APIs do not yet exist. The
[release plan](https://github.com/valkyoth/mynd/blob/main/docs/VERSION_PLAN.md)
defines the incremental path to the first production-ready 1.0.0 release.

## Install

Most applications should depend on the facade crate with only the capabilities
they need:

```toml
[dependencies]
mynd = { version = "0.2.1", default-features = false }
```

The default feature set is intentionally empty.

## Capability status

| Capability | Status | Planned release family |
|---|---|---|
| `no_std` facade, core boundary, and governance contract | Foundation available | 0.1.x-0.2.x |
| Checked image foundations and caller-owned buffers | Planned | 0.3.x-0.12.x |
| Shared I/O, probing, and codec contracts | Planned | 0.13.x-0.19.x |
| BMP, QOI, Netpbm PNM/PAM, and farbfeld | Planned | 0.20.x-0.35.x |
| PNG, GIF, JPEG, WebP, and TIFF | Planned | 0.36.x-0.77.x |
| Color, resize, transform, and bounded raster-drawing primitives | Planned | 0.78.x-0.94.x |
| Optional parallelism, async adapters, GPU hooks, and CLI | Planned | 0.95.x-0.98.x |
| Production stabilization and 1.0 security review | Planned | 0.99.x-1.0.0 |

Each release must satisfy its stated verification and security exit criteria;
future capabilities are not considered available merely because they appear on
the roadmap. Exact scope and public claim words are defined by the
[scope and claim contract](https://github.com/valkyoth/mynd/blob/main/docs/SCOPE_AND_CLAIMS.md).

## Design goals

- `no_std` from the first release, with `alloc` and `std` kept behind explicit
  feature boundaries.
- Bounded allocation, dimensions, frame counts, metadata, nesting, and work
  budgets before parsing untrusted image data.
- Panic-free handling of malformed input and checked arithmetic throughout
  size, stride, offset, and allocation calculations.
- No hidden I/O, threads, global mutable state, platform assumptions, or
  mandatory runtime dependencies in the core path.
- Small, focused workspace crates instead of source files larger than 500
  lines or monolithic implementation crates.
- Stable behavior across Linux, Windows, BSD, macOS, Android, and iOS, while
  preserving a path to future Aesynx support.

## Features

| Feature | Default | Purpose |
|---|---:|---|
| `alloc` | No | Reserved for APIs that require caller-visible allocation support |
| `std` | No | Enables standard-library integration and implies `alloc` |

Format and processing capabilities will be added as narrowly scoped opt-in
features only when their release gates are complete.

## Rust Version Support

The minimum supported Rust version is Rust `1.90.0`. New deployments should use
the pinned stable Rust `1.97.1` until the toolchain policy is updated.

Compatibility evidence for `0.2.1`:

| Rust | Local Evidence |
| --- | --- |
| `1.90.0`-`1.97.0` | `cargo check` and `cargo test` on every supported toolchain |
| `1.97.1` | Full release gate |

The compatibility policy is documented in
[toolchain policy](https://github.com/valkyoth/mynd/blob/main/docs/toolchain-policy.md).

## Workspace

| Crate | Role | Runtime dependencies |
|---|---|---:|
| [`mynd`](https://crates.io/crates/mynd) | Public facade and feature composition | 1 first-party crate |
| [`mynd-core`](https://crates.io/crates/mynd-core) | Format-neutral `no_std` image foundations | 0 |

Application-facing APIs should enter through `mynd`. Support crates are
published separately for workspace architecture and expert use, but are not
the default integration surface.

## Verification

Run the repository gate locally with the pinned development toolchain:

```sh
rustup run 1.97.1 scripts/checks.sh
```

The repository gate covers formatting, lints, tests, documentation, `no_std`
builds, MSRV checks, dependency policy, and package contents. The full release
gate and CI additionally run vulnerability auditing, the complete Rust and
platform matrices, latest-tool checks, and SBOM verification. See
[toolchain policy](https://github.com/valkyoth/mynd/blob/main/docs/toolchain-policy.md) for the
full command matrix.

## Security

Treat all image input as hostile. Mynd's security work starts before format
implementation and is governed by:

- [Security policy](https://github.com/valkyoth/mynd/blob/main/SECURITY.md)
- [Threat model](https://github.com/valkyoth/mynd/blob/main/docs/threat-model.md)
- [Supply-chain security](https://github.com/valkyoth/mynd/blob/main/docs/supply-chain-security.md)
- [Unsafe code policy](https://github.com/valkyoth/mynd/blob/main/docs/unsafe-policy.md)
- [Security controls](https://github.com/valkyoth/mynd/blob/main/docs/security-controls.md)

Please report vulnerabilities privately as described in the security policy.

## Specifications and implementation plans

Mynd implementations must be traceable to authoritative format specifications,
errata, and recorded security limits. The repository maintains:

- [Specification corpus and reproduction workflow](https://github.com/valkyoth/mynd/blob/main/specs/README.md)
- [Tracked-source legal disposition review](https://github.com/valkyoth/mynd/blob/main/specs/LEGAL_REVIEW.json)
- [Specification source ledger](https://github.com/valkyoth/mynd/blob/main/SPEC_SOURCES.md)
- [Standards and errata ledger](https://github.com/valkyoth/mynd/blob/main/docs/STANDARDS_LEDGER.md)
- [Scope and claim contract](https://github.com/valkyoth/mynd/blob/main/docs/SCOPE_AND_CLAIMS.md)
- [Corpus provenance schema](https://github.com/valkyoth/mynd/blob/main/docs/corpus-provenance-schema.md)
- [Specification source policy](https://github.com/valkyoth/mynd/blob/main/docs/spec-source-policy.md)
- [Implementation plan](https://github.com/valkyoth/mynd/blob/main/docs/IMPLEMENTATION_PLAN.md)
- [Version plan](https://github.com/valkyoth/mynd/blob/main/docs/VERSION_PLAN.md)
- [Release process](https://github.com/valkyoth/mynd/blob/main/docs/release-runbook.md)

## License

Licensed under either of:

- [Apache License, Version 2.0](https://github.com/valkyoth/mynd/blob/main/LICENSE-APACHE)
- [MIT License](https://github.com/valkyoth/mynd/blob/main/LICENSE-MIT)

at your option.
