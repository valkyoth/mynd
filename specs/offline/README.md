# Offline Specification Copies

`files/` is deliberately ignored by Git. It contains copyright-restricted,
acceptance-gated, or redistribution-unclear reference copies and is not part
of Mynd's public source tree or software licenses.

Run `scripts/fetch-specs.sh offline` to recreate every checksum-locked source
that has a direct official download. Run `scripts/spec-sources.py status` for
the remaining manual acquisition list. Put lawfully acquired manual files at
the exact names shown and record their SHA-256 values in
`specs/MANUAL_SHA256SUMS`.

Never commit anything below `files/`.
