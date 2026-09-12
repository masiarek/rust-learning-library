# A shell function: the project lives as long as the prompt

**Level:** 101 · for newcomers

**One line:** `rs anyhow rand` opens a nested shell inside a fresh Cargo project under `$TMPDIR`; `exit` brings you back to where you were and deletes it — no install, no config file, and the lifetime is the one thing every other route has to fake.

## The function

Two copies, one per shell family. Fish is the login shell on this machine, so that is the one that autoloads:

- [`rs.fish` ↗](https://github.com/masiarek/rust-learning-library/blob/master/05_Tooling/create_rust_proj/shell_function/rs.fish) — copy to `~/.config/fish/functions/rs.fish`; fish loads it on first use, no reload
- [`rs.bash` ↗](https://github.com/masiarek/rust-learning-library/blob/master/05_Tooling/create_rust_proj/shell_function/rs.bash) — `source` it from `~/.bashrc` or `~/.zshrc`; it is plain POSIX-ish shell and runs under either

```fish
function rs --description 'throwaway cargo project for this shell; deleted on exit'
    set -l dir (mktemp -d (string trim --right --chars=/ $TMPDIR)/rs.XXXXXX)
    cargo init --quiet --name scratch --vcs none --edition 2024 $dir
    or return
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
```

```text title="Real output — rs, with pwd / ls -a / cargo run --quiet / exit typed at the nested prompt"
scratch: /var/folders/bc/gzwjtz8d6mx3l2f9yyf_p8yw0000gn/T/rs.gC7I4m  (exit deletes it)
/var/folders/bc/gzwjtz8d6mx3l2f9yyf_p8yw0000gn/T/rs.gC7I4m
.
..
.cargo
Cargo.toml
src
Hello, world!
scratch: deleted
```

Two seconds from `rs` to a prompt inside a project with `cargo run` working, and nothing on disk afterwards.

## What each line is for

| Line | Why it is there |
|---|---|
| `mktemp -d …/rs.XXXXXX` | a unique folder under `$TMPDIR`; `string trim` strips the trailing `/` macOS puts on that variable, which would otherwise give `T//rs.XXXXXX` |
| `cargo init … --vcs none` | a project with no `.git` — there is nothing to commit in a folder that deletes itself |
| `[lints.rust]` block | the four scratch-noise lints off — `unused_variables`, `unused_imports`, `unused_mut`, `dead_code` — and **not** `unused = "allow"`, which would also silence `unused_must_use`, the lint that catches a dropped `Result` |
| `.cargo/config.toml` remap | `--> /var/folders/…/rs.gC7I4m/src/main.rs:3:5` in every diagnostic instead of `--> src/main.rs:3:5`, so the path is clickable from any terminal when three scratch projects are open |
| `cargo add … $argv` | the crates you named; an unknown crate name fails here, and the folder is removed rather than left half-made |
| `fish --init-command "cd $dir; …"` | the nested shell, started inside the project, with a yellow `scratch` on the right of the prompt so you can tell which shell you are in |
| `rm -rf $dir` | runs when the nested shell exits, however it exits |

## The trap: a shell inside a shell

You are one level deeper than you were. `exit` (or Ctrl-D) leaves the scratch shell and deletes the project; a *second* `exit` closes your real terminal. The right-prompt marker is there so the first `exit` is not a surprise, and `$SHLVL` counts the nesting if you want to check.

The second trap is an editor. `open -a RustRover .` from the nested shell works — the IDE opens the folder — but the folder is deleted when you `exit`, and RustRover will then be showing files that no longer exist. Close the project in the IDE first, then `exit`. If you want the deletion tied to the editor rather than the shell, that is what [`scratch.py --code`](../scratch_py/README.md) does.

## Versus the other routes

| | `rs` | [`scratch.sh`](../scratch_sh/README.md) | [`cargo temp`](../cargo_temp/README.md) |
|---|---|---|---|
| lifetime | this shell session | until reboot, or `--prune` | the shell it opens, or the editor |
| setup | one file in `~/.config/fish/functions/` | one script on `PATH` | `cargo install` |
| learner defaults | yes | yes | no |
| crates | `rs anyhow rand` | `--crates "anyhow rand"`, five by default | `cargo temp anyhow rand` |
| opens the editor | you do, from inside | `--rustrover` / `--code` | config file |
| leaves a project behind | never | by design, so you can come back | only if you `rm TO_DELETE` |

`rs` is the right one for "let me check what this method returns" — a question whose answer you will have in a minute and never need the file for. `scratch.sh` is for an afternoon: you will come back after lunch, and the project should still be there.

## See also

- [The hub — a disposable Rust workspace, six ways](../README.md) — the comparison table this page is one row of
- [Running a scratch program](../../../15_First_Programs/rustc_without_cargo/README.md) — when the question is small enough that no crate and no project are needed
- [fish: autoloading functions ↗](https://fishshell.com/docs/current/language.html#autoloading-functions) — why dropping the file into `functions/` is the whole installation
