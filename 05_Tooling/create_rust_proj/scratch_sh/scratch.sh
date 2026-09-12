#!/usr/bin/env bash
# scratch.sh — a disposable Cargo project in the system temp folder, ready to type into.
#
#   scratch.sh                       # anyhow rand itertools serde serde_json, then print the path
#   scratch.sh --rustrover           # ...and open it in RustRover
#   scratch.sh --code                # ...or in VS Code
#   scratch.sh --crates "regex clap" # a different crate list (space-separated)
#   scratch.sh --no-crates           # std only: no network, ~0.3 s
#   scratch.sh --prune               # delete every earlier scratch project first
#
# The project lives under $TMPDIR (macOS: /var/folders/.../T/rust-scratch.XXXXXX), so it
# is gone on the next reboot and never clutters a folder you look at. Nothing is git-
# tracked, so nothing needs cleaning up; --prune exists for the target/ folders, which
# reach a few hundred MB each once serde and friends are compiled.

set -euo pipefail

crates="anyhow rand itertools serde serde_json"
ide=""
prune=0

while [ $# -gt 0 ]; do
    case "$1" in
        --crates)     crates="$2"; shift ;;
        --no-crates)  crates="" ;;
        --rustrover)  ide="rustrover" ;;
        --code)       ide="code" ;;
        --prune)      prune=1 ;;
        -h|--help)    sed -n '2,15p' "$0"; exit 0 ;;
        *)            echo "unknown option: $1" >&2; exit 2 ;;
    esac
    shift
done

tmp="${TMPDIR:-/tmp}"; tmp="${tmp%/}"          # macOS sets TMPDIR with a trailing slash

# ---- 0. throw the old ones away, if asked ----------------------------------------------
if [ "$prune" -eq 1 ]; then
    for old in "$tmp"/rust-scratch.*; do
        [ -d "$old" ] || continue
        rm -rf "$old" && echo "  pruned $old"
    done
fi

# ---- 1. make the disposable project ----------------------------------------------------
dir=$(mktemp -d "$tmp/rust-scratch.XXXXXX")
cd "$dir"
cargo init --quiet --name scratch --vcs none --edition 2024
echo "  cargo init  $dir"

# ---- 2. learner settings: the two lines that make the first minute pleasant -------------
# (a) The scratch-noise half of rustc's `unused` group. NOT `unused = "allow"`:
#     unused_must_use is in that group, and an ignored Result is a dropped error.
cat >> Cargo.toml <<'TOML'

[lints.rust]
unused_variables = "allow"
unused_imports = "allow"
unused_mut = "allow"
dead_code = "allow"
TOML
# (b) Diagnostics name the absolute file, so `--> src/main.rs:5:9` becomes a path you can
#     click from any terminal when three scratch projects are open. The EMPTY old prefix
#     (the `==`) matches every relative path component and no absolute one, so std and
#     ~/.cargo/registry keep their own paths.
mkdir -p .cargo
printf '[build]\nrustflags = ["--remap-path-prefix==%s/"]\n' "$dir" > .cargo/config.toml
echo "  wrote       [lints.rust] + .cargo/config.toml"

# ---- 3. the crates a learner reaches for, and a main.rs that already uses them -----------
if [ -n "$crates" ]; then
    # `serde` alone is useless without its derive macros; add the feature when it is listed.
    # (A string, not an array: an empty array is an "unbound variable" under set -u on the
    # bash 3.2 that macOS ships as /bin/bash.)
    features=""
    case " $crates " in *" serde "*) features="--features serde/derive" ;; esac
    # shellcheck disable=SC2086   # $crates and $features are deliberate word lists
    cargo add --quiet $crates $features
    echo "  cargo add   $crates"
fi

if [ "$crates" = "anyhow rand itertools serde serde_json" ]; then
    cat > src/main.rs <<'RS'
//! Scratch — nothing here survives the reboot.

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> anyhow::Result<()> {
    let n: u8 = rand::random();
    dbg!(n);

    let evens = (1..=10).filter(|n| n % 2 == 0).join(", ");
    println!("{evens}"); // 2, 4, 6, 8, 10

    let p = Point { x: 3, y: 4 };
    let json = serde_json::to_string(&p)?;
    println!("{json}"); // {"x":3,"y":4}
    let back: Point = serde_json::from_str(&json)?;
    assert_eq!(p, back);
    Ok(())
}
RS
else
    cat > src/main.rs <<'RS'
//! Scratch — nothing here survives the reboot.

fn main() {
    println!("{}", 6 * 7); // 42
}
RS
fi

# ---- 4. prove it builds before you start typing ---------------------------------------
cargo run --quiet
echo "  cargo run   ok"

# ---- 5. open it, if asked --------------------------------------------------------------
case "$ide" in
    rustrover)
        # `open -a` resolves the app through LaunchServices, which knows an app on an
        # external volume; the `rustrover` launcher on PATH is a Toolbox script that may
        # name a path that no longer exists.
        open -a RustRover "$dir" ;;
    code)
        code --new-window "$dir" ;;
esac

echo
echo "cd $dir"
