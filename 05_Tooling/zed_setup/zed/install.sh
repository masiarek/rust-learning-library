#!/usr/bin/env bash
# The Zed setup from the rust-learning-library page 05_Tooling/zed_setup.
#
#   bash install.sh                Ctrl+R saves, then runs the Rust file you are
#                                  looking at; autosave when focus leaves a file
#                                  (writes to ~/.config/zed)
#   bash install.sh --cargo DIR    a Cargo project: the run button saves first,
#                                  and F4 debugs the open src/bin file (DIR/.zed)
#   bash install.sh --loose DIR    a folder of loose .rs files with no Cargo.toml:
#                                  rust-analyzer support (DIR/rust-project.json)
#
# It never overwrites a file. Where one already exists, it prints what to add.
# Run it from a clone of the library, or on its own: the files it needs are
# fetched from GitHub when they are not next to it.
set -euo pipefail

BASE=https://raw.githubusercontent.com/masiarek/rust-learning-library/master/05_Tooling/zed_setup/zed
here=$(cd "$(dirname "$0")" && pwd)
config="$HOME/.config/zed"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

say() { printf '%s\n' "$*"; }

# kit NAME: the path of one of this kit's files, downloading it if need be.
kit() {
    if [ -f "$here/$1" ]; then
        printf '%s\n' "$here/$1"
    else
        mkdir -p "$(dirname "$tmp/$1")"
        curl -fsSL "$BASE/$1" -o "$tmp/$1"
        printf '%s\n' "$tmp/$1"
    fi
}

# place SOURCE DEST: copy, unless DEST exists; then show what to merge by hand.
place() {
    if [ -e "$2" ]; then
        say "exists, left alone: $2"
        say "  add this to it yourself:"
        sed 's/^/    /' "$1"
    else
        mkdir -p "$(dirname "$2")"
        cp "$1" "$2"
        say "wrote: $2"
    fi
}

global() {
    place "$(kit run-rust-file.sh)" "$config/run-rust-file.sh"
    place "$(kit tasks.json)" "$config/tasks.json"
    place "$(kit keymap.json)" "$config/keymap.json"
    if [ ! -e "$config/settings.json" ]; then
        printf '{\n  "autosave": "on_focus_change"\n}\n' >"$config/settings.json"
        say "wrote: $config/settings.json"
    elif grep -q '"autosave"' "$config/settings.json"; then
        say "autosave is already set in $config/settings.json; left alone"
    else
        say "exists, left alone: $config/settings.json"
        say '  add this line inside its outer { }:'
        say '    "autosave": "on_focus_change",'
    fi
}

cargo_project() {
    [ -f "$1/Cargo.toml" ] || { say "no Cargo.toml in $1" >&2; exit 1; }
    place "$(kit cargo-project/tasks.json)" "$1/.zed/tasks.json"
    place "$(kit cargo-project/debug.json)" "$1/.zed/debug.json"
}

loose_folder() {
    [ -d "$1" ] || { say "not a folder: $1" >&2; exit 1; }
    sysroot=$(rustc --print sysroot)
    library="$sysroot/lib/rustlib/src/rust/library"
    if [ ! -d "$library" ]; then
        say "the standard library's source is missing; run: rustup component add rust-src" >&2
        exit 1
    fi
    if [ -e "$1/rust-project.json" ]; then
        say "exists, left alone: $1/rust-project.json (delete it and run again to list new files)"
    else
        {
            printf '{\n  "sysroot": "%s",\n  "sysroot_src": "%s",\n  "crates": [' "$sysroot" "$library"
            sep=""
            for rs in "$1"/*.rs; do
                [ -e "$rs" ] || continue
                printf '%s\n    { "root_module": "%s", "edition": "2024", "deps": [] }' \
                    "$sep" "$(basename "$rs")"
                sep=","
            done
            printf '\n  ]\n}\n'
        } >"$1/rust-project.json"
        say "wrote: $1/rust-project.json"
    fi
    place "$(kit loose-folder/settings.json)" "$1/.zed/settings.json"
}

case "${1:-}" in
"") global ;;
--cargo) cargo_project "${2:?usage: install.sh --cargo DIR}" ;;
--loose) loose_folder "${2:?usage: install.sh --loose DIR}" ;;
*) say "usage: install.sh [--cargo DIR | --loose DIR]" >&2; exit 2 ;;
esac
