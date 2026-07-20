# mynd Specification Source Policy

Every format behavior starts with current official or original source material.

1. Verify publisher, title, edition, errata, and redistribution status.
2. Store public metadata in `SPEC_SOURCES.md` and protected documents privately.
3. Hash the exact source snapshot used for implementation.
4. Map requirements to modules, tests, fuzz targets, proofs, and support claims.
5. Test official conformance material where licensing permits.
6. Compare independent implementations without treating majority behavior as
   automatically normative.
7. Stop and document ambiguities before selecting behavior.
8. Record source changes in release notes and security review evidence.

Codec implementation is blocked when source provenance is unresolved. Blog
posts, tutorials, and remembered behavior may help investigation but are never
the normative basis.
