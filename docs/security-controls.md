# mynd Security Controls

Initial enforced controls:

- `no_std` and empty default features;
- `#![forbid(unsafe_code)]`;
- workspace development, test, and release profiles enable overflow checks,
  and the workspace release profile aborts on panic;
- panic, unwrap, expect, indexing, and arithmetic-side-effect Clippy policy;
- denied unknown registries, unknown git sources, wildcard dependencies, and
  multiple dependency versions;
- 500-line source ceiling and inward-only crate boundaries;
- MSRV/current-stable and platform compilation matrices;
- GitHub CodeQL default setup policy.

Cargo profiles belong to the top-level build. Downstream consumers control
their own overflow-check and panic strategies; Mynd does not claim that its
workspace profiles propagate into applications. Library safety instead relies
on checked arithmetic in source, panic-free APIs, unsafe-code prohibition, and
the lint, test, review, and release gates applied to every published version.

Controls activated with parser work:

- explicit memory, output, metadata, frame, nesting, seek, and work budgets;
- every-byte truncation and mutation suites;
- format corpora with provenance;
- fuzz targets for probes, headers, streaming, chunk boundaries, policies,
  encoders, and round trips;
- Kani proofs for bounded arithmetic/state invariants;
- Miri and sanitizer evidence;
- independent differential and official conformance tests.
