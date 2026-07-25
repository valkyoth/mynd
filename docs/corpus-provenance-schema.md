# Corpus Provenance Schema

Status: normative schema contract

[`specs/SOURCES.schema.json`](../specs/SOURCES.schema.json) is the
machine-readable schema for [`specs/SOURCES.json`](../specs/SOURCES.json).
Schema version 1 describes source identity, edition, role, legal disposition,
safe acquisition metadata, exact destination name, license/terms evidence, and
the maximum accepted byte count.

## Required identity

Every source record contains:

- a stable lowercase `id`;
- non-empty `title`, `publisher`, `edition`, and project `role`;
- one `disposition`: `public`, `offline`, or `manual`;
- one safe filename without directories;
- a non-empty license/terms description and allow-listed HTTPS `terms_url`;
- a positive `max_bytes` no greater than 100 MB.

Public and offline records contain `download_url` and cannot contain
`acquisition_url`. Manual records contain `acquisition_url` and cannot contain
`download_url`. Unknown properties fail validation so provenance cannot be
hidden in an unreviewed field.

## Validation beyond JSON Schema

The repository validator deliberately enforces security properties that generic
JSON Schema does not:

- source IDs and disposition/filename destinations are unique;
- URLs use HTTPS, port 443, no credentials or fragment, and an explicit
  publisher-host allowlist;
- checksum manifests contain exactly the public/offline file set;
- manual locks cannot name an unknown record;
- tracked public files are regular, non-symlink, read-only, exact-hash files;
- ignored offline/manual files are verified when present or explicitly
  required;
- crate allowlists exclude every specification document.

The schema and validator are both release-controlled. Raising
`schema_version`, adding a field, widening a host or size limit, changing a
disposition, or changing source bytes requires provenance, legal, security, and
release review.

## Relationship to claims

Schema validity means only that the provenance record is structurally complete.
It does not establish redistribution permission, resolve an ambiguous edition,
accept errata, admit a format, or prove conformance. Those decisions live in
[`STANDARDS_LEDGER.md`](STANDARDS_LEDGER.md),
[`SCOPE_AND_CLAIMS.md`](SCOPE_AND_CLAIMS.md), and exact-version release
evidence.
