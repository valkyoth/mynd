# mynd Supply-Chain Security

Published runtime crates should use no third-party crates, build dependencies,
procedural macros, build scripts, native code, dynamic loading, or downloaded
build-time code. Development tooling remains allowed and encouraged.

Required controls:

- `cargo deny check` for licenses, sources, advisories, wildcards, and versions;
- `cargo audit` for RustSec advisories;
- Dependabot for Cargo and GitHub Actions;
- GitHub Actions pinned to full commit SHAs;
- latest-version checks before every dependency/tool edit;
- package-content and runtime-dependency review before releases.

GitHub Actions and Rust command-line tools use different pinning mechanisms.
Actions accept arbitrary Git refs, so workflows pin their full commit SHA.
Release tools are installed from crates.io at an immutable version with
`cargo install --locked --version`; the published crate's lockfile fixes its
dependency resolution, and `scripts/check_latest_tools.sh` rejects stale pins.
The repository does not claim a locally pinned artifact checksum for these
tools. Adding a second binary installer or binary mirror would introduce
another bootstrap trust path and is not required for v0.1.0.

If a third-party runtime integration becomes necessary, isolate it in a small
adapter crate so `mynd` core and codec crates remain `no_std`. Admission needs
license, maintenance, ownership, feature, MSRV, unsafe, build-script,
native-code, network, filesystem, and transitive-graph review.

Current published runtime dependency inventory: none outside `mynd-*`.
