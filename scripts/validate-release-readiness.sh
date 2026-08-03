#!/usr/bin/env sh
set -eu

tag="${1:-}"
case "$tag" in
    v[0-9]*.[0-9]*.[0-9]*) ;;
    *)
        echo "usage: scripts/validate-release-readiness.sh vX.Y.Z" >&2
        exit 2
        ;;
esac

version="${tag#v}"
release_notes="release-notes/RELEASE_NOTES_${version}.md"
pentest_report="security/pentest/${tag}.md"
publish_tag="${MYND_RELEASE_PUBLISH_TAG:-}"
planned_version="$(python3 scripts/release_policy.py plan-field version)"
kind="$(python3 scripts/release_policy.py plan-field kind)"

if [ "$version" != "$planned_version" ]; then
    echo "release tag ${tag} does not match planned version v${planned_version}" >&2
    exit 1
fi

if git rev-parse -q --verify "refs/tags/${tag}" >/dev/null; then
    if [ "$publish_tag" != "$tag" ]; then
        echo "tag already exists locally: ${tag}" >&2
        exit 1
    fi

    tag_commit="$(git rev-list -n 1 "$tag")"
    head_commit="$(git rev-parse HEAD)"
    if [ "$tag_commit" != "$head_commit" ]; then
        echo "publish tag ${tag} does not point at HEAD" >&2
        exit 1
    fi
elif [ -n "$publish_tag" ]; then
    echo "publish tag context requires existing tag: ${tag}" >&2
    exit 1
fi

if [ -n "$(git status --porcelain)" ]; then
    echo "release candidate worktree must be clean" >&2
    exit 1
fi

if [ -f PENTEST.md ]; then
    echo "root PENTEST.md is temporary input and must be removed" >&2
    exit 1
fi

if [ ! -s "$release_notes" ]; then
    echo "missing or empty release notes: ${release_notes}" >&2
    exit 1
fi

if [ ! -s sbom/mynd.spdx.json ]; then
    echo "missing or empty SBOM: sbom/mynd.spdx.json" >&2
    exit 1
fi

if [ "$kind" = "engineering" ]; then
    if [ -e "$pentest_report" ]; then
        echo "engineering checkpoint must not keep a pentest report" >&2
        exit 1
    fi
    exit 0
fi

if [ ! -s "$pentest_report" ]; then
    echo "missing or empty pentest report: ${pentest_report}" >&2
    exit 1
fi
if ! git cat-file -e "HEAD:${pentest_report}" 2>/dev/null; then
    echo "pentest report must be committed in the release candidate" >&2
    exit 1
fi
if ! grep -q '^Status: PASS$' "$pentest_report"; then
    echo "pentest report must say Status: PASS" >&2
    exit 1
fi
if ! grep -Eq '^Tester: .+' "$pentest_report"; then
    echo "pentest report is missing Tester" >&2
    exit 1
fi
if ! grep -Eq '^Scope: .+' "$pentest_report"; then
    echo "pentest report is missing Scope" >&2
    exit 1
fi
if ! grep -Eq '^Date: [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pentest_report"; then
    echo "pentest report is missing a valid Date" >&2
    exit 1
fi
if grep -Eq '^(Tester|Scope|Date): pending$' "$pentest_report"; then
    echo "pentest report still contains pending release fields" >&2
    exit 1
fi

for heading in "## Summary" "## Scope" "## Evidence" \
    "## Findings And Remediation" "## Residual Risk" "## Release Decision"; do
    if ! grep -q "^${heading}$" "$pentest_report"; then
        echo "pentest report is missing ${heading}" >&2
        exit 1
    fi
done
