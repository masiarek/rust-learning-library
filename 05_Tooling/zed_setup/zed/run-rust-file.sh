#!/usr/bin/env bash
# Run the Rust file open in Zed (bound to ctrl-r in keymap.json).
#   - inside a Cargo package: cargo run, with --bin for a file in src/bin/
#   - a loose .rs file: compile it with rustc into /tmp and run it
set -euo pipefail
file=$1
case "$file" in
*.rs) ;;
*) echo "not a Rust file: $file" >&2; exit 1 ;;
esac

# The nearest Cargo.toml at or above the file's folder.
root=$(dirname "$file")
while [ "$root" != / ] && [ ! -f "$root/Cargo.toml" ]; do
    root=$(dirname "$root")
done

if [ -f "$root/Cargo.toml" ]; then
    cd "$root"
    case "$file" in
    "$root"/src/bin/*.rs) exec cargo run --bin "$(basename "$file" .rs)" ;;
    *) exec cargo run ;;
    esac
else
    out="/tmp/zed-rustc-$(basename "$file" .rs)"
    rustc --edition 2024 "$file" -o "$out"
    exec "$out"
fi
