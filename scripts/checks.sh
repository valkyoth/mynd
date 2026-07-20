#!/usr/bin/env sh
set -eu

cargo fmt --all --check
scripts/check_shell_syntax.sh
scripts/check_doc_links.sh
scripts/validate-modularity-policy.sh check
scripts/validate-security-policy.sh
python3 scripts/check-runtime-dependencies.py
python3 scripts/test_release_crates.py
scripts/validate-release-metadata.sh
cargo +1.90.0 check --workspace --no-default-features
cargo +1.90.0 test --workspace --all-features
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
cargo package -p mynd-core --allow-dirty
cargo package -p mynd --allow-dirty \
    --config 'patch.crates-io.mynd-core.path="crates/mynd-core"'
