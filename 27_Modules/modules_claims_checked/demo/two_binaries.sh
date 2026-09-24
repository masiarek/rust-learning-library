#!/bin/sh
# A package with src/main.rs and src/bin/my_binary.rs, and `cargo run` with
# nothing to say which. Both calls inherit the shared target directory, and
# nothing below prints a path.

echo '1. cargo run -p pg_store_two_binaries'
cargo run -q --locked -p pg_store_two_binaries 2>&1
echo "exit status $?"

echo
echo '2. cargo run -p pg_store_two_binaries --bin pg_store_two_binaries'
cargo run -q --locked -p pg_store_two_binaries --bin pg_store_two_binaries 2>&1
echo "exit status $?"
