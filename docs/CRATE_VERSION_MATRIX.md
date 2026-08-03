# Crate Version Matrix

The facade is the integration train. Every GitHub tag records current source
versions, but crates.io advances only at cumulative publication checkpoints.
Support crates publish only when their code or published dependency
requirements changed since their latest crates.io version.

The root workspace package version remains the `0.1.0` support-crate baseline.
Beginning with this release, `mynd` declares its integration-train version
explicitly; an unchanged support crate can continue inheriting its existing
workspace version without changing its package inputs.

| Crate | Previous GitHub tag | Published before release | Planned | Change | Publish now | Reason |
| --- | --- | --- | --- | --- | --- | --- |
| `mynd-math` | `0.1.0` | `0.1.0` | `0.1.0` | unchanged | No | No published code, dependency, or package metadata changed. |
| `mynd-core` | `0.2.0` | `0.2.0` | `0.3.0` | code | Yes | Add explicit sample storage, pixel layouts, chroma and alpha domains, plane relationships, tests, and Kani proofs. |
| `mynd` | `0.4.0` | `0.4.0` | `0.5.0` | code | Yes | Re-export the validated `mynd-core` 0.3.0 pixel-storage API through the facade. |

Facade code uses the milestone version; support-crate code gets its next
independent minor; compatible fixes, dependency changes, and package metadata
changes increment a support crate's patch. At engineering checkpoints every
`Publish now` value is No. At a cumulative checkpoint, a support crate may be
unchanged from the previous GitHub tag and still publish when its planned
version is newer than `Published before release` and a published crate needs
it. Already-published versions are never republished.
