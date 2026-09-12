# A disposable Rust workspace, six ways

**Level:** 101 → 201 · for newcomers

**One line:** A place to type Rust *right now*, with the crates you usually want and the editor already open on it, that costs nothing to abandon — that is one workflow with six implementations, from a ten-line shell script to a REPL with no project at all, and the choice is mostly about how long the thing should live and what deletes it.

## The workflow

Every route on this page is the same six steps. Four of them are the same three commands as [A throwaway that needs a crate](../scratch_with_a_crate/README.md); the other two are what makes it disposable.

| Step | What happens | The command underneath |
|---|---|---|
| **1. make** | a fresh folder somewhere you will not browse, and a manifest in it | `mktemp -d`, `cargo init --vcs none` |
| **2. settle** | the two lines a learner's project wants and `cargo init` does not write: unused-binding warnings off, absolute paths in diagnostics | `[lints.rust]` in `Cargo.toml`, `.cargo/config.toml` |
| **3. stock** | the crates you reach for, at real current versions | `cargo add anyhow rand itertools serde serde_json` |
| **4. prove** | it compiles and prints before you type a line | `cargo run` |
| **5. open** | the editor on the folder | `open -a RustRover "$dir"` or `code --wait "$dir"` |
| **6. discard** | the folder goes away without you doing anything | `$TMPDIR`, `exit`, a closed window, or the next run |

Step 6 is the whole design question. A folder under `$TMPDIR` is discarded by *neglect*; a shell function discards on *exit*; `code --wait` discards on *window close*; `cargo temp` discards on either. Pick the lifetime first and the script follows.

## Six routes

| Route | One command | Ready in | Learner defaults | IDE | Dies when |
|---|---|---|---|---|---|
| [`scratch.sh`](scratch_sh/README.md) | `scratch.sh --rustrover` | 11 s (2 s with `--no-crates`) | yes | `--rustrover` / `--code` | you `--prune`, or the OS cleans `$TMPDIR` |
| [`scratch.py`](scratch_py/README.md) | `scratch.py --code` | 11 s | yes | `--code` waits, `--rustrover` cannot | the VS Code window closes; or the next run |
| [a shell function](shell_function/README.md) | `rs anyhow rand` | 2 s + crates | yes | you open it, from inside | you `exit` the nested shell |
| [`cargo-temp`](cargo_temp/README.md) | `cargo temp anyhow rand` | ~2 s + crates | **no** | config file: `editor = "code"` | you `exit`, or the editor exits |
| [a single-file script](single_file_scripts/README.md) | `cargo +nightly -Zscript f.rs`, `rust-script f.rs` | 3–4 s, then cached | n/a | none — no manifest on disk | you delete one file |
| [`evcxr`](evcxr_repl/README.md) | `evcxr` | 9 s for the first line | n/a | none | you close the REPL |

"Ready in" was measured on this machine with the crate registry already warm; the first project ever to use `serde` also pays the download. Every number on the sub-pages is from a real run and says which one.

## Which one

- **"What does this method return?"** — [`rs`](shell_function/README.md) if it needs a crate, `rustc` on a loose file if it does not ([running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md)), [`evcxr`](evcxr_repl/README.md) if it is one expression and the REPL is already open.
- **An afternoon with a crate, in the IDE** — [`scratch.sh --rustrover`](scratch_sh/README.md). It is still there after lunch, and gone when you next `--prune`.
- **The same, but tidy** — [`scratch.py --code`](scratch_py/README.md): the folder disappears when the window does.
- **You would rather install a tool than keep a script** — [`cargo temp`](cargo_temp/README.md), and add the two learner lines by hand when you miss them.
- **A program you will run more than once as a program** — [a single-file script](single_file_scripts/README.md): it carries its manifest, it has a shebang, and it lives in a gist.
- **The fortieth exercise this month** — none of these: [a practice tree](../practice_workspace/README.md), and `cargo new` inside it.

## The ten-line version, reviewed

The script that started this page:

```bash
#!/usr/bin/env bash
set -euo pipefail

dir=$(mktemp -d "${TMPDIR:-/tmp}/rust-scratch.XXXXXX")
cd "$dir"
cargo init --name scratch --vcs none
cargo add anyhow rand

cat > src/main.rs <<'EOF'
fn main() -> anyhow::Result<()> {
    let n: u8 = rand::random();
    dbg!(n);
    Ok(())
}
EOF

cargo run
open -a "RustRover" "$dir"   # macOS
# or: rustrover "$dir" &      # Linux
echo "Scratch project at: $dir"
```

It makes sense, and it is steps 1, 3, 4 and 5 of the table in twelve lines; [`scratch.sh`](scratch_sh/README.md) is this script with the gaps filled. What was checked, and what changed:

| Line | Verdict |
|---|---|
| `mktemp -d "${TMPDIR:-/tmp}/…"` | right, with one cosmetic catch: macOS sets `TMPDIR` with a trailing `/`, so the path prints as `T//rust-scratch.XXXXXX`. `${TMPDIR%/}` strips it |
| `cargo init --name scratch --vcs none` | right; `--edition 2024` is already the default on 1.98, and the scripts say it anyway |
| `cargo add anyhow rand` | right, and `rand::random()` into a `u8` compiles on the `rand 0.10.2` it resolves today. Missing: the crates a learner meets next (`itertools`, `serde` + `serde_json`), and the `serde/derive` feature without which `serde` is unusable |
| the heredoc `main.rs` | right, and the reason `cargo run` proves something |
| `open -a "RustRover" "$dir"` | **the correct command on this Mac**, and the comment under it is the trap: `rustrover "$dir"` prints an `NSCocoaErrorDomain` error and opens nothing, because the launcher on `PATH` names `/Volumes/T5/Applications/RustRover.app`, a drive the app left. [Opening it in the IDE](open_in_the_ide/README.md) has the transcript |
| `echo "Scratch project at: $dir"` | prints *after* the IDE call, which is fine since `open -a` returns at once |
| what is not there | step 2 — the four `[lints.rust]` lines and the `.cargo/config.toml` remap. Without them the first scratch file prints five `unused_variable` warnings and every diagnostic says `--> src/main.rs`. Two heredocs, and the difference between a scratch project you use and one you tolerate |

`set -euo pipefail` is doing real work: a `cargo add` of a misspelled crate, or a `cargo run` that fails, stops the script before an editor opens on a broken folder.

## The crates a learner keeps reaching for

The default list in both scripts, with what each one is for and what it costs on top of the empty project:

| Crate | What it is for | First build | `target/` |
|---|---|---|---|
| *(none)* | `std` is a lot | 2.0 s | 1.1 MB |
| `anyhow` | `fn main() -> anyhow::Result<()>` and `?` on any error, so a scratch program never needs an error type | | |
| `rand` | `rand::random()`, `random_range(1..=6)` — the first crate every tutorial asks for | | |
| `itertools` | `.join(", ")`, `.tuple_windows()`, `.sorted()` — the iterator adaptors `std` does not have | 4.5 s | 23 MB |
| `serde` + `serde_json` | `#[derive(Serialize, Deserialize)]` and a JSON round trip | 11.0 s | 71 MB |
| `regex`, `clap`, `chrono`, `thiserror` | patterns, a command line, dates, a real error enum — the next four, when you need them | 14.0 s | 184 MB |

Rows accumulate: the 11.0 s is all five, the 14.0 s is all nine. Adding a crate is `cargo add <name>` in the folder, so the default list is a starting point rather than a decision — and `--crates "regex clap"` in either script makes a different one.

## See also

- [A throwaway that needs a crate](../scratch_with_a_crate/README.md) — the three commands, done by hand, and what each file they write is for
- [Scaffolding a practice tree](../scaffolding/README.md) — the two learner defaults explained, `rust_scaffold.py`, and the practice tree for the projects that are not throwaways
- [A tree of practice projects](../practice_workspace/README.md) — when there are forty of these rather than one
- [RustRover setup](../rustrover_setup/README.md) — the once-per-project toolchain click every route here still needs
