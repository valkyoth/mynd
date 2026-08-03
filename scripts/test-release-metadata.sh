#!/usr/bin/env sh
set -eu

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/scripts" "$tmp/release-notes" "$tmp/security/pentest"
cp scripts/validate-release-metadata.sh "$tmp/scripts/"
cp scripts/release_policy.py "$tmp/scripts/"
cat >"$tmp/scripts/release_crates.py" <<'EOF'
#!/usr/bin/env sh
set -eu
test "$1" = "--version"
test "$3" = "--check"
EOF
chmod +x "$tmp/scripts/release_crates.py"

write_engineering() {
    cat >"$tmp/release-crates.toml" <<'EOF'
[release]
version = "0.5.1"
kind = "engineering"
checkpoint = "0.10.0"
previous_publication = "0.5.0"

[crates.mynd]
previous_version = "0.5.0"
published_version = "0.5.0"
version = "0.5.1"
change = "code"
publish = false
reason = "fixture"
EOF
    cat >"$tmp/release-notes/RELEASE_NOTES_0.5.1.md" <<'EOF'
# mynd 0.5.1

Release kind: GitHub engineering checkpoint
External pentest: No; cumulative review at v0.10.0
Crates.io: Not published; next publication is mynd 0.10.0
EOF
    printf '## [0.5.1]\n' >"$tmp/CHANGELOG.md"
}

write_publication() {
    cat >"$tmp/release-crates.toml" <<'EOF'
[release]
version = "0.10.0"
kind = "publication"
checkpoint = "0.10.0"
previous_publication = "0.5.0"

[crates.mynd]
previous_version = "0.9.0"
published_version = "0.5.0"
version = "0.10.0"
change = "code"
publish = true
reason = "fixture"
EOF
    cat >"$tmp/release-notes/RELEASE_NOTES_0.10.0.md" <<'EOF'
# mynd 0.10.0

Release kind: Cumulative publication checkpoint
EOF
    printf '## [0.10.0]\n' >"$tmp/CHANGELOG.md"
    printf 'Status: IN PROGRESS\n' >"$tmp/security/pentest/v0.10.0.md"
}

assert_fails() {
    if (cd "$tmp" && scripts/validate-release-metadata.sh "$@") \
        >"$tmp/stdout" 2>"$tmp/stderr"; then
        echo "expected metadata validation to fail" >&2
        exit 1
    fi
}

write_engineering
(cd "$tmp" && scripts/validate-release-metadata.sh 0.5.1)
printf 'Status: PASS\n' >"$tmp/security/pentest/v0.5.1.md"
assert_fails 0.5.1
rm "$tmp/security/pentest/v0.5.1.md"
sed -i '/External pentest:/d' "$tmp/release-notes/RELEASE_NOTES_0.5.1.md"
assert_fails 0.5.1

write_publication
(cd "$tmp" && scripts/validate-release-metadata.sh 0.10.0)
rm "$tmp/security/pentest/v0.10.0.md"
assert_fails 0.10.0

printf 'release metadata tests passed\n'
