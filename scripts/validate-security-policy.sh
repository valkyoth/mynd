#!/usr/bin/env sh
set -eu

grep -R '#!\[forbid(unsafe_code)\]' crates >/dev/null
grep -q 'unknown-git = "deny"' deny.toml
grep -q 'unknown-registry = "deny"' deny.toml
grep -q 'multiple-versions = "deny"' deny.toml
grep -q 'panic = "abort"' Cargo.toml
grep -q 'overflow-checks = true' Cargo.toml
grep -q 'CodeQL default setup' SECURITY.md
grep -q 'CodeQL analysis default setup is active' docs/github-security-settings.md
grep -q 'no third-party crates' docs/supply-chain-security.md
test -f docs/threat-model.md
test -f docs/unsafe-policy.md
test -f SPEC_SOURCES.md
test -f FORMAT_SUPPORT.md
