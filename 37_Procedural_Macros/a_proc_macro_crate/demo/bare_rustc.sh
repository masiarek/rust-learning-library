#!/bin/sh
# hello_macro built by Cargo, printing only the flags that make it a
# proc-macro crate, and then three times by rustc alone. Everything happens in
# a temporary directory, and nothing below prints a path.

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

echo '1. The flags Cargo passes to rustc for hello_macro'
CARGO_TARGET_DIR="$tmp/target" cargo build --locked -v -p hello_macro 2>&1 |
    grep -o -e '--crate-type proc-macro' -e '--extern proc_macro'

cp hello_macro/src/lib.rs "$tmp/lib.rs"
cd "$tmp" || exit 1

echo
echo '2. rustc --edition 2024 --crate-type proc-macro lib.rs'
rustc --edition 2024 --crate-type proc-macro lib.rs 2>&1
echo "exit status $?"

echo
echo '3. rustc --edition 2024 --crate-type proc-macro --extern proc_macro lib.rs'
rustc --edition 2024 --crate-type proc-macro --extern proc_macro lib.rs 2>&1
echo "exit status $?"

echo
echo '4. the same file with extern crate proc_macro; added, and no --extern'
{ cat lib.rs; echo 'extern crate proc_macro;'; } > with_extern.rs
rustc --edition 2024 --crate-type proc-macro --crate-name hello_macro with_extern.rs 2>&1
echo "exit status $?"
ls libhello_macro.*
