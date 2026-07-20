# mynd Modularity Policy

The facade must not become the implementation home.

- `mynd` provides ergonomic feature-gated dispatch and re-exports.
- Format-neutral types, arithmetic, I/O, metadata, color, processing, and
  quantization live in focused crates.
- BMP, TGA, GIF, farbfeld, PNG, classic JPEG, and JPEG XL are separate security surfaces.
- Complex metadata standards become separate crates when implemented.
- Codecs depend inward on shared crates; shared crates never depend on codecs.
- All internal dependencies set `default-features = false`.
- A module stays local until at least two real users justify shared ownership.
- Non-generated code files may never exceed 500 lines; review splitting near
  300 lines.
- `lib.rs` wires modules and public API; it does not host large implementations.
- No runtime plugin loader, async-runtime dependency, or automatic thread pool.

Future Aesynx support requires core-only APIs, caller-owned buffers, explicit
platform adapters, and no assumptions about filesystem, environment, threads,
virtual memory, or conventional process services.
