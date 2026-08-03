# Crate Version Matrix

The facade is the integration train. Support crates publish only when their
code or published dependency requirements change.

The root workspace package version remains the `0.1.0` support-crate baseline.
Beginning with this release, `mynd` declares its integration-train version
explicitly; an unchanged support crate can continue inheriting its existing
workspace version without changing its package inputs.

| Crate | Previous | Planned | Change | Publish | Reason |
| --- | --- | --- | --- | --- | --- |
| `mynd-math` | `0.0.0` | `0.1.0` | code | Yes | Introduce checked conversion, arithmetic, alignment, range, tests, and Kani proofs. |
| `mynd-core` | `0.1.0` | `0.1.0` | unchanged | No | No published code, dependency, or package metadata changed. |
| `mynd` | `0.2.1` | `0.3.0` | code | Yes | Re-export the checked `mynd-math` 0.1.0 API through the facade. |

Change kinds follow the adapted `eth` release tool: facade code uses the
milestone version; support-crate code gets its next independent minor; bugfixes
increment patch; dependency-only changes increment patch; unchanged crates are
not published.
