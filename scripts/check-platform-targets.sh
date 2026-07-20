#!/usr/bin/env sh
set -eu

for target in \
    x86_64-unknown-linux-gnu \
    x86_64-unknown-freebsd \
    aarch64-linux-android \
    x86_64-pc-windows-msvc \
    aarch64-apple-ios \
    x86_64-apple-darwin; do
    cargo +1.97.1 check --workspace --lib --no-default-features --target "$target"
done
