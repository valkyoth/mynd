<p align="center">
  <b>Checked <code>no_std</code> arithmetic for Mynd's hostile image-data boundaries.</b><br>
  Explicit conversion, overflow, alignment, and bounded-range failures without allocation or unsafe code.
</p>

<div align="center">
  <a href="https://crates.io/crates/mynd">mynd crate</a>
  |
  <a href="https://docs.rs/mynd-math">Docs.rs</a>
  |
  <a href="https://github.com/valkyoth/mynd/blob/main/docs/VERSION_PLAN.md">Release Plan</a>
  |
  <a href="https://github.com/valkyoth/mynd/blob/main/docs/math-primitives.md">Contract</a>
  |
  <a href="https://github.com/valkyoth/mynd/blob/main/SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <a href="https://github.com/valkyoth/mynd">
    <img src="https://raw.githubusercontent.com/valkyoth/mynd/main/.github/images/mynd.webp" alt="mynd-math Rust crate overview">
  </a>
</p>

# mynd-math

Support crate for [`mynd`](https://crates.io/crates/mynd): small, audited,
format-neutral arithmetic primitives used before untrusted image values become
offsets, lengths, ranges, or platform-sized indexes.

Most applications should depend on the facade crate:

```toml
[dependencies]
mynd = { version = "0.3.0", default-features = false }
```

Expert users can depend directly on `mynd-math`:

```toml
[dependencies]
mynd-math = { version = "0.1.0", default-features = false }
```

## Capability

- checked signed, fixed-width, and platform-width integer conversions;
- checked `u64` and `usize` addition and multiplication;
- checked upward alignment for any nonzero multiple;
- checked half-open ranges against an exclusive upper bound;
- structured, allocation-free errors identifying conversion, overflow,
  alignment, and range failures.

Operations never wrap, saturate, silently truncate, allocate, inspect external
state, or use unsafe code. Alignment does not imply a power-of-two memory-layout
guarantee. Range functions accept an empty range at the exclusive upper bound.

## Verification

The crate is covered by extrema matrices, exhaustive reduced-domain tests,
cross-operation consistency tests, all supported Rust and platform targets,
and Kani proofs over full-width and explicitly bounded domains. See the
[arithmetic contract](https://github.com/valkyoth/mynd/blob/main/docs/math-primitives.md)
for exact invariants and evidence mapping.

## Compatibility

The supported MSRV is Rust 1.90.0. Rust 1.90.0 through 1.97.1 are covered by
the workspace compatibility matrix, with 1.97.1 used for development and
release verification.

## License

Licensed under either
[Apache-2.0](https://github.com/valkyoth/mynd/blob/main/LICENSE-APACHE) or
[MIT](https://github.com/valkyoth/mynd/blob/main/LICENSE-MIT), at your option.
