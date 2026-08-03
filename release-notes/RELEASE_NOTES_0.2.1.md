# mynd 0.2.1 Release Notes

Status: released 2026-07-27.

This governance-only release completes the reproducible specification-corpus
and legal-disposition handoff. It adds no image parser, decoder, encoder,
processing behavior, feature, unsafe code, or runtime dependency.

Every source in `specs/SOURCES.json` remains classified as `public`, `offline`,
or `manual`. Public sources are tracked only as exact, checksum-locked,
unmodified copies. Automatically downloadable sources without sufficiently
clear redistribution permission remain ignored, and purchase-, login-,
acceptance-, blocked-, or provenance-dependent sources remain manual.

`specs/LEGAL_REVIEW.json` records the maintainer disposition decision for every
tracked source ID exactly once. Its checker binds each approval to the source
manifest's exact license and terms URL, requires explicit attribution
instructions, confirms every offline/manual destination is ignored, and makes
no ownership claim or broad legal conclusion.

Corpus verification now rejects unknown filesystem entries in both the
tracked public directory and the shared ignored offline/manual directory.
Focused tests mutate source bytes and manifest structure, inject an unknown
private entry, prove `fetch all` never selects manual sources, and prove a
changed upstream response is not installed or allowed to rewrite a reviewed
lock.

CI recreates all 58 automatically downloadable public/offline sources from
isolated empty directories, verifies their immutable hashes, regular-file
type, read-only mode, and exact sets, and confirms no manual source was
fetched. Crate package allowlists continue to exclude all specification
documents.

This release exercises the independent crate-version policy again. The facade
crate advances from `mynd` 0.2.0 to 0.2.1 because its packaged documentation
changes. `mynd-core` remains byte-for-byte unchanged at 0.1.0 and is not
republished.

Release requirements:

- the isolated public/offline corpus recreation and all mutation tests pass;
- the exact legal-review coverage and private-file exclusion checks pass;
- repository, supported-Rust, platform, latest-tool, dependency, RustSec,
  package, documentation, and SBOM gates pass;
- `security/pentest/v0.2.1.md` reaches `Status: PASS`;
- the final committed candidate passes GitHub CI and CodeQL before tagging.

The initial pentest found one Low defense-in-depth inconsistency: the legal
checker reconstructed ignored private paths from raw manifest JSON after the
canonical validator had separately accepted the file. The checker now consumes
only validated `Source` objects, and a traversal-shaped filename regression
test proves failure before any Git path check. The final retest confirmed the
remediation.

A follow-up review found that legal approval was not yet cryptographically
bound to the approved source bytes and complete provenance record. The legal
ledger now contains a canonical corpus SHA-256 covering every validated public
identity, publisher, edition, role, disposition, filename, acquisition field,
terms field, license, byte ceiling, and locked source hash. Changing both
source provenance and its checksum now fails legal validation until a human
deliberately renews that approval digest. The final retest confirmed this
remediation, and the permanent pentest report records `Status: PASS`.
