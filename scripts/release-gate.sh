#!/usr/bin/env sh
set -eu

version="${1:-$(python3 scripts/release_policy.py plan-field version)}"
pinned_rust="$(sed -n 's/^channel = "\([0-9][0-9.]*\)"$/\1/p' rust-toolchain.toml)"

if [ -z "$pinned_rust" ]; then
    echo "rust-toolchain.toml does not pin a numeric stable Rust" >&2
    exit 1
fi

rustc --version | grep -q "^rustc ${pinned_rust} "
scripts/checks.sh
scripts/check-kani.sh
python3 scripts/check-spec-recreation.py
scripts/check-rust-version-matrix.sh
scripts/check-platform-targets.sh
scripts/check_latest_tools.sh
cargo deny check
cargo audit
scripts/generate-sbom.sh --check
MYND_RELEASE_FINAL=1 scripts/validate-release-metadata.sh "$version"
