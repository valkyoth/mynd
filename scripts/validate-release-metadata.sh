#!/usr/bin/env sh
set -eu

planned_version="$(python3 scripts/release_policy.py plan-field version)"
version="${1:-$planned_version}"
tag="v${version}"
report="security/pentest/${tag}.md"
kind="$(python3 scripts/release_policy.py plan-field kind)"
checkpoint="$(python3 scripts/release_policy.py plan-field checkpoint)"

test -f "release-notes/RELEASE_NOTES_${version}.md"
grep -q "version = \"${version}\"" release-crates.toml
grep -q "## \[${version}\]" CHANGELOG.md
scripts/release_crates.py --version "$version" --check

case "$kind" in
    engineering)
        test ! -e "$report"
        grep -q '^Release kind: GitHub engineering checkpoint$' \
            "release-notes/RELEASE_NOTES_${version}.md"
        grep -q "^External pentest: No; cumulative review at v${checkpoint}$" \
            "release-notes/RELEASE_NOTES_${version}.md"
        grep -q "^Crates.io: Not published; next publication is mynd ${checkpoint}$" \
            "release-notes/RELEASE_NOTES_${version}.md"
        ;;
    publication)
        test -f "$report"
        grep -Eq '^Status: (NOT RUN|IN PROGRESS|PASS)$' "$report"
        grep -q '^Release kind: Cumulative publication checkpoint$' \
            "release-notes/RELEASE_NOTES_${version}.md"
        ;;
    emergency)
        test -f "$report"
        grep -Eq '^Status: (NOT RUN|IN PROGRESS|PASS)$' "$report"
        grep -q '^Release kind: Emergency publication checkpoint$' \
            "release-notes/RELEASE_NOTES_${version}.md"
        ;;
    *)
        echo "unsupported release kind: ${kind}" >&2
        exit 1
        ;;
esac

if [ "${MYND_RELEASE_FINAL:-${MYND_RELEASE_REQUIRE_PASS:-0}}" = "1" ]; then
    scripts/validate-release-readiness.sh "$tag"
fi
