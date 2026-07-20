# mynd Toolchain Policy

`mynd` pins stable Rust 1.97.1 and supports 1.90.0 through 1.97.1.

Pins last verified against upstream on 2026-07-20:

| Tool | Pin |
| --- | --- |
| Rust stable | 1.97.1 |
| `cargo-deny` | 0.20.2 |
| `cargo-audit` | 0.22.2 |
| `cargo-sbom` | 0.10.0 |
| `actions/checkout` | v7.0.1 at full tag SHA |

Rules:

- `scripts/check_latest_tools.sh` compares the pin with the official stable
  distribution manifest and fails when stale.
- Keep `workspace.package.rust-version` at the supported MSRV.
- Test every stable release and patch listed in the README before release.
- Run the full release gate on the pinned stable toolchain.
- Never require nightly for normal builds; nightly tools are evidence only.
- Review current tool and crate versions whenever manifests or CI change.

```sh
scripts/check-rust-version-matrix.sh
```
