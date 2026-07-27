# Crate Version Matrix

The facade is the integration train. Support crates publish only when their
code or published dependency requirements change.

The root workspace package version remains the `0.1.0` support-crate baseline.
Beginning with this release, `mynd` declares its integration-train version
explicitly; an unchanged support crate can continue inheriting its existing
workspace version without changing its package inputs.

| Crate | Previous | Planned | Change | Publish | Reason |
| --- | --- | --- | --- | --- | --- |
| `mynd-core` | `0.1.0` | `0.1.0` | unchanged | No | No published code, dependency, or package metadata changed. |
| `mynd` | `0.2.0` | `0.2.1` | metadata | Yes | Publish reproducible corpus, legal-disposition, and verification documentation. |

Change kinds follow the adapted `eth` release tool: facade code uses the
milestone version; support-crate code gets its next independent minor; bugfixes
increment patch; dependency-only changes increment patch; unchanged crates are
not published.
