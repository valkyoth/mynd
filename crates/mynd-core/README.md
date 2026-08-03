<p align="center">
  <b>Format-neutral <code>no_std</code> image foundations for Mynd.</b><br>
  Validated core types, explicit resource ownership, and security-gated release evidence.
</p>

<div align="center">
  <a href="https://crates.io/crates/mynd">mynd crate</a>
  |
  <a href="https://docs.rs/mynd-core">Docs.rs</a>
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
    <img src="https://raw.githubusercontent.com/valkyoth/mynd/main/.github/images/mynd.webp" alt="mynd-core Rust crate overview">
  </a>
</p>

# mynd-core

Support crate for [`mynd`](https://crates.io/crates/mynd): format-neutral,
`no_std` image foundations shared by Mynd's codecs and processing crates.

Most users should depend on the facade crate:

```toml
[dependencies]
mynd = { version = "0.5.0", default-features = false }
```

`mynd-core` is published separately to maintain small implementation boundaries
and permit expert integrations. It is not the default application-facing API.

Expert users who only need the validated primitives may depend directly on:

```toml
[dependencies]
mynd-core = { version = "0.3.0", default-features = false }
```

## Current status

Version 0.3.0 adds validated sample storage and format-neutral pixel layouts to
the existing geometry foundation. Its types explicitly model integer and
floating storage widths, byte/bit order, channel order, chroma subsampling,
alpha association, and interleaved, planar, or semi-planar relationships.
Concrete plane validation rejects mismatched counts, sampled rows, used row
bytes, ordering, and overlap.

These types describe storage only. Floating-point numeric behavior, color
conversion, slice-backed views, allocation, and codecs remain unavailable until
their staged releases in the
[version plan](https://github.com/valkyoth/mynd/blob/main/docs/VERSION_PLAN.md).

## Features

| Feature | Default | Purpose |
|---|---:|---|
| `alloc` | No | Reserved for allocation-aware core APIs |
| `std` | No | Enables standard-library integration and implies `alloc` |

## Security posture

- `no_std` is the baseline; the default feature set is empty.
- The only runtime dependency is the first-party, dependency-free `mynd-math`.
- Unsafe code is forbidden by workspace policy.
- Dimensions, strides, offsets, row sizes, sample widths, plane relationships,
  and output lengths are validated with checked arithmetic before becoming
  usable values.
- Malformed untrusted input must not panic, over-allocate, or perform unbounded
  work.

Security requirements and disclosure instructions live in the repository's
[threat model](https://github.com/valkyoth/mynd/blob/main/docs/threat-model.md)
and [security policy](https://github.com/valkyoth/mynd/blob/main/SECURITY.md).

## Compatibility

The supported MSRV is Rust 1.90.0. Rust 1.90.0 through 1.97.1 are covered by
the workspace compatibility matrix, with 1.97.1 used for development and
release verification.

## License

Licensed under either
[Apache-2.0](https://github.com/valkyoth/mynd/blob/main/LICENSE-APACHE) or
[MIT](https://github.com/valkyoth/mynd/blob/main/LICENSE-MIT), at your option.
