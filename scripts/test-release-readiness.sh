#!/usr/bin/env sh
set -eu

unset MYND_RELEASE_PUBLISH_TAG

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
validator="$(pwd)/scripts/validate-release-readiness.sh"

make_fixture() {
    name="$1"
    repo="$tmp/$name"
    mkdir -p \
        "$repo/scripts" \
        "$repo/release-notes" \
        "$repo/security/pentest" \
        "$repo/sbom"
    cp "$validator" "$repo/scripts/validate-release-readiness.sh"
    (
        cd "$repo"
        git init -q
        git config user.email "release-readiness@example.invalid"
        git config user.name "Release Readiness Test"
        printf 'fixture\n' >README.md
        git add README.md
        git commit -q -m "fixture"
    )
    printf '%s\n' "$repo"
}

write_candidate_files() {
    version="$1"
    status="$2"
    cat >"release-notes/RELEASE_NOTES_${version}.md" <<EOF
# Release ${version}
EOF
    printf '{"spdxVersion":"SPDX-2.3"}\n' >sbom/mynd.spdx.json
    cat >"security/pentest/v${version}.md" <<EOF
# Pentest

Status: ${status}

Tester: Release Readiness Test
Scope: Fixture release.
Date: 2026-07-25

## Summary

Clean.

## Scope

Fixture.

## Evidence

Checks passed.

## Findings And Remediation

Nothing found.

## Residual Risk

None beyond scope.

## Release Decision

PASS.
EOF
}

assert_fails_with() {
    expected="$1"
    shift
    if "$@" >"$tmp/stdout" 2>"$tmp/stderr"; then
        echo "expected command to fail: $*" >&2
        exit 1
    fi
    if ! grep -q "$expected" "$tmp/stderr"; then
        echo "expected stderr to contain: ${expected}" >&2
        cat "$tmp/stderr" >&2
        exit 1
    fi
}

repo="$(make_fixture bad-tag)"
(
    cd "$repo"
    assert_fails_with "usage:" scripts/validate-release-readiness.sh 0.1.0
)

repo="$(make_fixture missing-report)"
(
    cd "$repo"
    printf '# Release 0.1.0\n' >release-notes/RELEASE_NOTES_0.1.0.md
    printf '{"spdxVersion":"SPDX-2.3"}\n' >sbom/mynd.spdx.json
    git add .
    git commit -q -m "metadata without report"
    assert_fails_with "missing or empty pentest report" \
        scripts/validate-release-readiness.sh v0.1.0
)

repo="$(make_fixture not-pass)"
(
    cd "$repo"
    write_candidate_files 0.1.0 "NOT RUN"
    git add .
    git commit -q -m "candidate"
    assert_fails_with "must say Status: PASS" \
        scripts/validate-release-readiness.sh v0.1.0
)

repo="$(make_fixture iterative-ready)"
(
    cd "$repo"
    write_candidate_files 0.1.0 "PASS"
    printf 'remediated implementation\n' >>README.md
    git add .
    git commit -q -m "fix findings and record passing retest"
    scripts/validate-release-readiness.sh v0.1.0
)

repo="$(make_fixture existing-tag)"
(
    cd "$repo"
    write_candidate_files 0.1.0 "PASS"
    git add .
    git commit -q -m "candidate"
    git tag v0.1.0
    assert_fails_with "tag already exists locally" \
        scripts/validate-release-readiness.sh v0.1.0
    MYND_RELEASE_PUBLISH_TAG=v0.1.0 \
        scripts/validate-release-readiness.sh v0.1.0
)

printf 'release readiness tests passed\n'
