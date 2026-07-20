#!/usr/bin/env sh
set -eu

facade_readme="crates/mynd/README.md"
logo_url="https://raw.githubusercontent.com/valkyoth/mynd/main/.github/images/mynd.webp"

if ! cmp -s README.md "$facade_readme"; then
    echo "README.md and $facade_readme must remain identical" >&2
    diff -u README.md "$facade_readme" >&2 || true
    exit 1
fi

for readme in README.md "$facade_readme" crates/mynd-core/README.md; do
    if ! grep -Fq "<img src=\"$logo_url\"" "$readme"; then
        echo "$readme must use the repository-hosted Mynd logo URL" >&2
        exit 1
    fi

    if grep -F ".github/images/mynd.webp" "$readme" | grep -Fv "$logo_url"; then
        echo "$readme contains a local Mynd logo reference" >&2
        exit 1
    fi
done

for package in mynd mynd-core; do
    package_files="$(cargo package -p "$package" --allow-dirty --list)"
    if printf '%s\n' "$package_files" \
        | grep -Eq '(^|/)\.github/images/|\.(gif|jpe?g|png|svg|webp)$'; then
        echo "$package would publish a repository image:" >&2
        printf '%s\n' "$package_files" \
            | grep -E '(^|/)\.github/images/|\.(gif|jpe?g|png|svg|webp)$' >&2
        exit 1
    fi
done
