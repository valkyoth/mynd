#!/usr/bin/env sh
set -eu

for target in \
    x86_64-unknown-linux-gnu \
    i686-unknown-linux-gnu \
    wasm32-unknown-unknown \
    x86_64-unknown-freebsd \
    aarch64-linux-android \
    x86_64-pc-windows-msvc \
    aarch64-apple-ios \
    x86_64-apple-darwin; do
    cargo +1.97.1 check --workspace --lib --no-default-features --target "$target"
done

# Compile target-width-specific tests without linking a foreign executable.
cargo +1.97.1 check --workspace --tests --all-features --target i686-unknown-linux-gnu
