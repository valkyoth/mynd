# mynd Security Controls

Initial enforced controls:

- `no_std` and empty default features;
- `#![forbid(unsafe_code)]`;
- checked release overflow and aborting panic strategy;
- panic, unwrap, expect, indexing, and arithmetic-side-effect Clippy policy;
- denied unknown registries, unknown git sources, wildcard dependencies, and
  multiple dependency versions;
- 500-line source ceiling and inward-only crate boundaries;
- MSRV/current-stable and platform compilation matrices;
- GitHub CodeQL default setup policy.

Controls activated with parser work:

- explicit memory, output, metadata, frame, nesting, seek, and work budgets;
- every-byte truncation and mutation suites;
- format corpora with provenance;
- fuzz targets for probes, headers, streaming, chunk boundaries, policies,
  encoders, and round trips;
- Kani proofs for bounded arithmetic/state invariants;
- Miri and sanitizer evidence;
- independent differential and official conformance tests.
