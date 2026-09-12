# rs — a throwaway Cargo project whose lifetime is the shell you are typing in.
#
#   rs                 # std only
#   rs anyhow rand     # with crates
#
# Opens a nested bash (or zsh) inside a fresh project under $TMPDIR. `exit` (or Ctrl-D)
# returns you to where you were and deletes the project. Source this file from
# ~/.bashrc or ~/.zshrc.

rs() {
    local tmp="${TMPDIR:-/tmp}"; tmp="${tmp%/}"
    local dir
    dir=$(mktemp -d "$tmp/rs.XXXXXX") || return 1
    cargo init --quiet --name scratch --vcs none --edition 2024 "$dir" || return 1

    # the two learner defaults: unused-binding warnings off, absolute paths in diagnostics
    printf '\n[lints.rust]\nunused_variables = "allow"\nunused_imports = "allow"\nunused_mut = "allow"\ndead_code = "allow"\n' >> "$dir/Cargo.toml"
    mkdir -p "$dir/.cargo"
    printf '[build]\nrustflags = ["--remap-path-prefix==%s/"]\n' "$dir" > "$dir/.cargo/config.toml"

    if [ $# -gt 0 ]; then
        cargo add --quiet --manifest-path "$dir/Cargo.toml" "$@" || { rm -rf "$dir"; return 1; }
    fi

    echo "scratch: $dir  (exit deletes it)"
    ( cd "$dir" && SCRATCH_DIR="$dir" "${SHELL:-bash}" -i )
    rm -rf "$dir"
    echo "scratch: deleted"
}
