# mynd Supply-Chain Security

Published runtime crates should use no third-party crates, build dependencies,
procedural macros, build scripts, native code, dynamic loading, or downloaded
build-time code. Development tooling remains allowed and encouraged.

Required controls:

- `cargo deny check` for licenses, sources, advisories, wildcards, and versions;
- `cargo audit` for RustSec advisories;
- Dependabot for Cargo and GitHub Actions;
- SHA-pinned GitHub Actions;
- latest-version checks before every dependency/tool edit;
- package-content and runtime-dependency review before releases.

If a third-party runtime integration becomes necessary, isolate it in a small
adapter crate so `mynd` core and codec crates remain `no_std`. Admission needs
license, maintenance, ownership, feature, MSRV, unsafe, build-script,
native-code, network, filesystem, and transitive-graph review.

Current published runtime dependency inventory: none outside `mynd-*`.
