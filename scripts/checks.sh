#!/usr/bin/env sh
set -eu

mynd_math_version="$(python3 scripts/release_policy.py crate-field mynd-math version)"
mynd_core_version="$(python3 scripts/release_policy.py crate-field mynd-core version)"
mynd_version="$(python3 scripts/release_policy.py crate-field mynd version)"

cargo fmt --all --check
scripts/check_shell_syntax.sh
scripts/lock-specs.sh
scripts/verify-specs.sh
python3 scripts/test-spec-sources.py
python3 scripts/check-legal-review.py
python3 scripts/check-governance.py
scripts/check-readmes.sh
scripts/check-package-licenses.sh
scripts/check_doc_links.sh
python3 scripts/check-version-plan.py
scripts/validate-modularity-policy.sh check
scripts/validate-security-policy.sh
python3 scripts/check-runtime-dependencies.py
python3 scripts/test_release_crates.py
scripts/test-release-readiness.sh
scripts/test-release-metadata.sh
scripts/validate-release-metadata.sh
cargo +1.90.0 check --workspace --no-default-features
cargo +1.90.0 test --workspace --all-features
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
cargo package -p mynd-math --allow-dirty
cargo package -p mynd-core --allow-dirty
cargo package -p mynd --allow-dirty \
    --config 'patch.crates-io.mynd-core.path="crates/mynd-core"' \
    --config 'patch.crates-io.mynd-math.path="crates/mynd-math"'
cargo test --manifest-path \
    "target/package/mynd-math-${mynd_math_version}/Cargo.toml" --all-features
cargo test --manifest-path \
    "target/package/mynd-core-${mynd_core_version}/Cargo.toml" --all-features \
    --config 'patch.crates-io.mynd-math.path="crates/mynd-math"'
cargo test --manifest-path \
    "target/package/mynd-${mynd_version}/Cargo.toml" --all-features \
    --config 'patch.crates-io.mynd-core.path="crates/mynd-core"' \
    --config 'patch.crates-io.mynd-math.path="crates/mynd-math"'
