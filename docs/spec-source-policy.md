# mynd Specification Source Policy

Status: policy

Every format behavior starts with current official or original source
material. The public source ledger is [`SPEC_SOURCES.md`](../SPEC_SOURCES.md);
the machine-readable acquisition and legal ledger is
[`specs/SOURCES.json`](../specs/SOURCES.json). The exact tracked-source
disposition approval is [`specs/LEGAL_REVIEW.json`](../specs/LEGAL_REVIEW.json).

## Source admission

Before implementation:

1. Verify publisher, exact title, edition/revision, status, corrigenda, errata,
   and normative versus informative role.
2. Check the publisher's actual copyright and redistribution terms. Public
   availability, a patent promise, or royalty-free implementation permission
   is not by itself permission to republish a document.
3. Prefer a dated publication or commit-pinned official source. Record a
   living source only when no immutable official form exists.
4. Hash the exact bytes reviewed for implementation.
5. Map clauses to modules, tests, fuzz targets, proofs, support claims, and
   documented exclusions.
6. Stop and record ambiguities before choosing behavior.

Codec implementation remains blocked when source provenance or the applicable
profile is unresolved. Blogs, tutorials, remembered behavior, and third-party
summaries may aid investigation but are never normative.

## Legal dispositions

Every source has exactly one disposition:

- `public`: an unmodified copy may be tracked because explicit publisher terms
  support redistribution and their notice/attribution conditions are retained;
- `offline`: an official direct download exists, but its copy is ignored
  because redistribution permission is absent, unclear, or narrower than the
  repository use;
- `manual`: purchase, authentication, click-through acceptance, publisher
  blocking of unattended retrieval, or unresolved provenance requires a human
  acquisition step.

The project claims no ownership in third-party materials. They are not
licensed under Mynd's MIT or Apache-2.0 terms. When uncertainty remains, the
source is kept offline; maintainers do not make an optimistic legal inference.
The legal-review ledger documents a maintainer repository disposition, not
legal advice. Its approval applies only to exact unmodified, checksum-locked
bytes with their original notices; any source, bytes, edition, URL, license,
terms, or disposition change invalidates that evidence until reviewed again.
The approval's `corpus_sha256` is computed over canonical JSON containing
every public source's validated identity, publisher, edition, role,
disposition, filename, acquisition and terms URLs, license, byte ceiling, and
locked content hash. Renewing it is an explicit human review action.

## Immutable local evidence

- `specs/SHA256SUMS` locks every tracked public copy.
- `specs/OFFLINE_SHA256SUMS` locks every reproducible ignored copy.
- `specs/MANUAL_SHA256SUMS` locks acquired private copies without publishing
  them.
- Source copies are unmodified and read-only locally. Notes and errata
  decisions live elsewhere.
- `.gitattributes` disables text normalization for tracked copies.
- Builds, tests, and crate consumers never download specifications.
- Strict package allowlists prevent source documents entering crates.io
  archives.

Git does not preserve read-only mode portably. The repository gate reapplies
it before checking mode, file identity, hashes, symlink safety, and the exact
tracked public and ignored private sets.

## Acquisition security

The fetcher:

- accepts only manifest-listed HTTPS URLs on a fixed publisher host allowlist;
- validates every redirect and final host;
- enforces a per-source maximum size;
- hashes a temporary download before atomic installation;
- requests identity-encoded, cache-revalidated responses;
- never overwrites changed local evidence silently;
- never supplies credentials, accepts legal terms, or automates purchases.

A remote hash change fails closed. Isolated CI recreation may reject and retry
a mismatched response up to three times, but succeeds only if a later response
matches the reviewed checksum exactly. Persistent drift is a source-update
event requiring identity, edition, legal, content, test, security, and
release-note review. CI separately recreates every automatic public/offline
source from isolated empty destinations and proves that the operation never
selects a manual source.

## Conformance and interpretation

Official conformance material is tested where licensing permits. Independent
implementations are compared for compatibility evidence, but majority behavior
is never presumed normative. Errata are recorded separately; published source
bytes are never locally “corrected.”

Source editions, clause mappings, ambiguities, compatibility decisions, and
support changes must appear in the affected release evidence and release
notes. See [`specs/README.md`](../specs/README.md) for commands and the update
procedure.
