# rs — a throwaway Cargo project whose lifetime is the shell you are typing in.
#
#   rs                 # std only
#   rs anyhow rand     # with crates
#
# Opens a nested fish inside a fresh project under $TMPDIR. `exit` (or Ctrl-D) returns
# you to where you were and deletes the project. Drop this file in
# ~/.config/fish/functions/ and it autoloads; no reload needed.

function rs --description 'throwaway cargo project for this shell; deleted on exit'
    set -l dir (mktemp -d (string trim --right --chars=/ $TMPDIR)/rs.XXXXXX)
    cargo init --quiet --name scratch --vcs none --edition 2024 $dir
    or return

    # the two learner defaults: unused-binding warnings off, absolute paths in diagnostics
    printf '\n[lints.rust]\nunused_variables = "allow"\nunused_imports = "allow"\nunused_mut = "allow"\ndead_code = "allow"\n' >> $dir/Cargo.toml
    mkdir -p $dir/.cargo
    printf '[build]\nrustflags = ["--remap-path-prefix==%s/"]\n' $dir > $dir/.cargo/config.toml

    if test (count $argv) -gt 0
        cargo add --quiet --manifest-path $dir/Cargo.toml $argv
        or begin
            rm -rf $dir
            return 1
        end
    end

    echo "scratch: $dir  (exit deletes it)"
    fish --init-command "cd $dir; function fish_right_prompt; set_color yellow; echo scratch; set_color normal; end"
    rm -rf $dir
    echo "scratch: deleted"
end
