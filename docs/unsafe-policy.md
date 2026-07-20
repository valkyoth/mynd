# mynd Unsafe Policy

Every current first-party crate uses:

```rust
#![forbid(unsafe_code)]
```

Unsafe Rust is not admitted in the current workspace. Byte casting, endian
conversion, unaligned reads, pixel copying, indexing shortcuts, and SIMD are
implemented safely until evidence proves another boundary is necessary.

Any future unsafe or SIMD work requires a dedicated non-default crate or
tightly isolated module, scalar reference implementation, written safety
contract, differential tests, alignment and target tests, Miri/sanitizer
evidence, benchmark justification, and a separate security review.
