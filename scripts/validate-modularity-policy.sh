#!/usr/bin/env sh
set -eu

mode="${1:-check}"
if [ "$mode" != "check" ]; then
    echo "usage: scripts/validate-modularity-policy.sh check" >&2
    exit 2
fi

violations="$(find crates scripts -type f \
    \( -name '*.rs' -o -name '*.py' -o -name '*.sh' \) \
    -exec wc -l {} \; | awk '$1 > 500 { print }')"
if [ -n "$violations" ]; then
    echo "code files exceed 500 lines:" >&2
    echo "$violations" >&2
    exit 1
fi

grep -q '"crates/mynd"' Cargo.toml
grep -q '"crates/mynd-core"' Cargo.toml
grep -q 'mynd-core.*default-features = false' Cargo.toml

if find crates -type f \( -name 'lib.rs' -o -name 'main.rs' \) \
    -exec grep -L '#!\[forbid(unsafe_code)\]' {} + |
    grep . >/dev/null; then
    echo "every Rust crate root/source file must explicitly forbid unsafe code" >&2
    exit 1
fi
