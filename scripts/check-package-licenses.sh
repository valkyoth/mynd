#!/usr/bin/env sh
set -eu

for package in mynd mynd-core; do
    for license in LICENSE-MIT LICENSE-APACHE; do
        packaged="crates/${package}/${license}"
        if ! cmp -s "$license" "$packaged"; then
            echo "${packaged} must match the repository ${license}" >&2
            exit 1
        fi
    done

    package_files="$(cargo package -p "$package" --allow-dirty --list)"
    for license in LICENSE-MIT LICENSE-APACHE; do
        if ! printf '%s\n' "$package_files" | grep -qx "$license"; then
            echo "${package} package is missing ${license}" >&2
            exit 1
        fi
    done
done
