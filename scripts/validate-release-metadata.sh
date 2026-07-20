#!/usr/bin/env sh
set -eu

version="${1:-0.1.0}"
tag="v${version}"

test -f "release-notes/RELEASE_NOTES_${version}.md"
test -f "security/pentest/${tag}.md"
grep -q "version = \"${version}\"" release-crates.toml
grep -q "## \[${version}\]" CHANGELOG.md
scripts/release_crates.py --version "$version" --check

if [ "${MYND_RELEASE_REQUIRE_PASS:-0}" = "1" ]; then
    grep -q '^Status: PASS$' "security/pentest/${tag}.md"
fi
