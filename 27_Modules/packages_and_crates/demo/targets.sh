#!/bin/sh
# What Cargo counts as a target, asked through `cargo metadata`: first for the
# committed package, then for a fresh `cargo new` in a temporary folder that
# gets the book's `bin/` folder (Listing 6.2) and then Cargo's `src/bin/`.
# The repo's rust-toolchain.toml is copied in so the temporary package is built
# by the same pinned compiler. Nothing below prints a path outside a package.

here=$(pwd)
repo=$(cd "$here/../../.." && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

targets() {
    cargo metadata --no-deps --format-version 1 --manifest-path "$1" | python3 "$here/targets.py" "$2"
}

echo '1. The committed package pg-pkg-two-crates: kind, target name, file'
targets two-crates/Cargo.toml pg-pkg-two-crates

cp "$repo/rust-toolchain.toml" "$tmp/"
cd "$tmp" || exit 1
cargo new -q --vcs none my_package

echo
echo '2. cargo new my_package, on this toolchain'
grep '^edition' my_package/Cargo.toml
targets my_package/Cargo.toml

echo
echo '3. Listing 6.2: bin/my_binary.rs at the package root, beside src/'
mkdir my_package/bin
printf 'fn main() {\n    println!("Hello from my_binary");\n}\n' > my_package/bin/my_binary.rs
targets my_package/Cargo.toml
(cd my_package && cargo run -q 2>&1)

echo
echo '4. The same file moved to src/bin/'
mkdir my_package/src/bin
mv my_package/bin/my_binary.rs my_package/src/bin/
targets my_package/Cargo.toml

echo
echo '5. rustc alone: one file, one crate, named after the file'
(cd my_package && rustc --print crate-name src/main.rs && rustc --print crate-name src/bin/my_binary.rs)
