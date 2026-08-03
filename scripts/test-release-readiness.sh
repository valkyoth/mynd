#!/usr/bin/env sh
set -eu

unset MYND_RELEASE_PUBLISH_TAG

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
validator="$(pwd)/scripts/validate-release-readiness.sh"
policy="$(pwd)/scripts/release_policy.py"

make_fixture() {
    name="$1"
    repo="$tmp/$name"
    mkdir -p "$repo/scripts" "$repo/release-notes" \
        "$repo/security/pentest" "$repo/sbom"
    cp "$validator" "$repo/scripts/validate-release-readiness.sh"
    cp "$policy" "$repo/scripts/release_policy.py"
    (
        cd "$repo"
        git init -q
        git config user.email "release-readiness@example.invalid"
        git config user.name "Release Readiness Test"
        printf 'fixture\n' >README.md
        git add README.md scripts
        git commit -q -m "fixture"
    )
    printf '%s\n' "$repo"
}

write_plan() {
    version="$1"
    kind="$2"
    checkpoint="$3"
    previous_publication="$4"
    previous="$5"
    published="$6"
    publish="$7"
    cat >release-crates.toml <<EOF
[release]
version = "${version}"
kind = "${kind}"
checkpoint = "${checkpoint}"
previous_publication = "${previous_publication}"

[crates.mynd]
previous_version = "${previous}"
published_version = "${published}"
version = "${version}"
change = "code"
publish = ${publish}
reason = "fixture"
EOF
}

write_candidate_files() {
    version="$1"
    status="$2"
    printf '# Release %s\n' "$version" >"release-notes/RELEASE_NOTES_${version}.md"
    printf '{"spdxVersion":"SPDX-2.3"}\n' >sbom/mynd.spdx.json
    cat >"security/pentest/v${version}.md" <<EOF
# Pentest

Status: ${status}

Tester: Release Readiness Test
Scope: Fixture release.
Date: 2026-08-03

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
    assert_fails_with "usage:" scripts/validate-release-readiness.sh 0.5.0
)

repo="$(make_fixture missing-report)"
(
    cd "$repo"
    write_plan 0.5.0 publication 0.5.0 0.4.0 0.4.0 0.4.0 true
    printf '# Release 0.5.0\n' >release-notes/RELEASE_NOTES_0.5.0.md
    printf '{"spdxVersion":"SPDX-2.3"}\n' >sbom/mynd.spdx.json
    git add .
    git commit -q -m "metadata without report"
    assert_fails_with "missing or empty pentest report" \
        scripts/validate-release-readiness.sh v0.5.0
)

repo="$(make_fixture not-pass)"
(
    cd "$repo"
    write_plan 0.5.0 publication 0.5.0 0.4.0 0.4.0 0.4.0 true
    write_candidate_files 0.5.0 "NOT RUN"
    git add .
    git commit -q -m "candidate"
    assert_fails_with "must say Status: PASS" \
        scripts/validate-release-readiness.sh v0.5.0
)

repo="$(make_fixture publication-ready)"
(
    cd "$repo"
    write_plan 0.5.0 publication 0.5.0 0.4.0 0.4.0 0.4.0 true
    write_candidate_files 0.5.0 "PASS"
    git add .
    git commit -q -m "passing publication candidate"
    scripts/validate-release-readiness.sh v0.5.0
)

repo="$(make_fixture engineering-ready)"
(
    cd "$repo"
    write_plan 0.5.1 engineering 0.10.0 0.5.0 0.5.0 0.5.0 false
    printf '# Release 0.5.1\n' >release-notes/RELEASE_NOTES_0.5.1.md
    printf '{"spdxVersion":"SPDX-2.3"}\n' >sbom/mynd.spdx.json
    git add .
    git commit -q -m "engineering candidate without pentest"
    scripts/validate-release-readiness.sh v0.5.1
)

repo="$(make_fixture engineering-report)"
(
    cd "$repo"
    write_plan 0.5.1 engineering 0.10.0 0.5.0 0.5.0 0.5.0 false
    write_candidate_files 0.5.1 "PASS"
    git add .
    git commit -q -m "engineering candidate with forbidden report"
    assert_fails_with "must not keep a pentest report" \
        scripts/validate-release-readiness.sh v0.5.1
)

repo="$(make_fixture existing-tag)"
(
    cd "$repo"
    write_plan 0.5.0 publication 0.5.0 0.4.0 0.4.0 0.4.0 true
    write_candidate_files 0.5.0 "PASS"
    git add .
    git commit -q -m "candidate"
    git tag v0.5.0
    assert_fails_with "tag already exists locally" \
        scripts/validate-release-readiness.sh v0.5.0
    MYND_RELEASE_PUBLISH_TAG=v0.5.0 \
        scripts/validate-release-readiness.sh v0.5.0
)

printf 'release readiness tests passed\n'
