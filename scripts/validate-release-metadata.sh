#!/usr/bin/env sh
set -eu

version="${1:-0.5.0}"
tag="v${version}"
report="security/pentest/${tag}.md"

test -f "release-notes/RELEASE_NOTES_${version}.md"
test -f "$report"
grep -q "version = \"${version}\"" release-crates.toml
grep -q "## \[${version}\]" CHANGELOG.md
scripts/release_crates.py --version "$version" --check
grep -Eq '^Status: (NOT RUN|IN PROGRESS|PASS)$' "$report"

if [ "${MYND_RELEASE_REQUIRE_PASS:-0}" = "1" ]; then
    scripts/validate-release-readiness.sh "$tag"
fi
