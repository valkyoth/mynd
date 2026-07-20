#!/usr/bin/env sh
set -eu

for toolchain in \
    1.90.0 1.91.0 1.92.0 1.93.0 1.94.0 \
    1.95.0 1.96.0 1.96.1 1.97.0 1.97.1; do
    rustup run "$toolchain" cargo check --workspace --no-default-features
    rustup run "$toolchain" cargo test --workspace --all-features
done
