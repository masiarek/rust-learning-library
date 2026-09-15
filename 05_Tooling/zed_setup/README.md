# Zed for Rust: one key to save and run, and three defaults in its way

**Level:** 101 → 201 · working knowledge

**One line:** Zed runs a Cargo project with nothing to configure, but a quick experiment meets three defaults — Zed wants a folder rather than a file, `rust-analyzer` wants a `Cargo.toml`, and a task runs what is on disk because tasks do not save first — and [a small kit](zed/install.sh) sets up **Ctrl+R: save, then run the Rust file in front of you**, on this machine and the next one.

[Choosing an editor](../editors/README.md) compares Zed with the others. This page is the setup, measured with Zed 1.19.2, the `rust-analyzer` Zed downloads (0.3.2896, built 2026-05-10) and rustc 1.98.0, on an x86-64 Mac in September 2026.

## Set up a machine

You need Zed and a rustup toolchain. Download the installer, read it, then run it:

```sh
curl -fsSLO https://raw.githubusercontent.com/masiarek/rust-learning-library/master/05_Tooling/zed_setup/zed/install.sh
bash install.sh
```

Both lines are the same in bash, zsh and fish. The installer fetches the other files it needs from [the same folder](zed/install.sh) of this repo and **never overwrites a file** — where one already exists it leaves it alone and prints what to add. Run a second time, it wrote nothing: three *exists, left alone* notices, and *autosave is already set*.

| File | Written to | What it does |
|---|---|---|
| [`run-rust-file.sh`](zed/run-rust-file.sh) | `~/.config/zed/` | picks the command for the open file — `cargo run --bin <name>` for `src/bin/<name>.rs`, `cargo run` elsewhere in a Cargo package, `rustc` into `/tmp` for a loose file |
| [`tasks.json`](zed/tasks.json) | `~/.config/zed/` | one task, *save and run this Rust file*, with `"save": "all"` |
| [`keymap.json`](zed/keymap.json) | `~/.config/zed/` | Ctrl+R runs that task, in the code editor only |
| `settings.json` | `~/.config/zed/` | `"autosave": "on_focus_change"` — written if the file does not exist yet, otherwise printed for you to add |

`~/.config/zed` is where Zed reads its settings on macOS and Linux. Two more modes set up one folder rather than the machine; the traps below say when you want each:

```sh
bash install.sh --cargo path/to/a/cargo/project
bash install.sh --loose path/to/a/folder/of/rs/files
```

## Ctrl+R

The binding, from [`keymap.json`](zed/keymap.json):

```json
{
  "context": "Editor && mode == full",
  "bindings": {
    "ctrl-r": ["task::Spawn", { "task_name": "save and run this Rust file" }]
  }
}
```

- **`task_name` is the task's `label`, character for character.** Rename one and the key does nothing.
- **`Editor && mode == full` is the code editor** — not the terminal, where Ctrl+R stays the shell's history search, and not one-line inputs such as the search bar. Zed's own task examples bind on `Workspace`, which also contains the terminal panel. (Not machine-checked here.)
- **Your keymap wins over the base keymap.** Zed loads user bindings after its built-in ones ([Key bindings ↗](https://zed.dev/docs/key-bindings)), so a JetBrains or VS Code base keymap's Ctrl+R gives way.

What the task's command ran, for four kinds of open file — the same command line the task uses, run through fish:

| The open file | What runs | What it printed |
|---|---|---|
| `src/bin/hello.rs` in a Cargo package | `cargo run --bin hello` | `from src/bin/hello.rs` |
| `src/main.rs` in a Cargo package | `cargo run` | `main.rs of a Cargo package` |
| `b.rs` in a folder with no `Cargo.toml` | `rustc --edition 2024 b.rs -o /tmp/zed-rustc-b`, then that program | `loose b 6` |
| `x.py` | nothing | `not a Rust file: /tmp/x.py` |

## Trap 1: a file is not a project

Open a file on its own — from Finder, or `zed r.rs` — and Zed makes a project whose "folders" are that file. The terminal panel then says *"This project has no folders open."*, and a terminal opened anyway starts in your home directory, where `cargo run` finds no `Cargo.toml`.

Open the folder instead: `zed .` from inside it, or File → Open… on the folder. The first `zed <path>` asks whether to *Add to existing Zed window* (`zed --existing`) or *Open a new window* (`zed --classic`). A new window per folder keeps each project's terminal and `rust-analyzer` apart; adding to an existing window is how one project ended up holding a Rust file and an unrelated `README.md` side by side.

## Trap 2: no `Cargo.toml`, no `rust-analyzer`

A folder of loose `.rs` files — Downloads, a folder of exercises — gets *"Failed to discover workspace"* from `rust-analyzer`, and every save runs `cargo check --workspace`, which fails with *could not find `Cargo.toml` in … or any parent directory*. Ctrl+R still works, since a loose file goes to `rustc`, but there are no types, no inline errors and no go-to-definition. There are two ways out.

**Make the folder a Cargo package, with one program per file in `src/bin/`.** Cargo builds every `src/bin/<name>.rs` as a binary of its own, so the next experiment is a new file and nothing to register:

```text
scratch/
  Cargo.toml        [package] with a name and edition = "2024", and no [[bin]] tables
  src/bin/r.rs      cargo run --bin r
  src/bin/next.rs   cargo run --bin next
```

Then `bash install.sh --cargo scratch` for the run button and the debugger. Two things that do not work on the way there. A dated folder name is not a package name:

```text title="cargo new 2026_09_14, cargo 1.98.0"
error: invalid character `2` in package name: `2026_09_14`, the name cannot start with a digit
note: the directory name is used as the package name
help: to override the package name, pass `--name <pkgname>`
```

And a single-file Cargo script, which would need no folder at all, is still nightly-only:

```text title="cargo r.rs, cargo 1.98.0"
error: running the file `r.rs` requires `-Zscript`
```

[A throwaway that needs a crate](../scratch_with_a_crate/README.md) is the same move with a dependency added, and [A disposable Rust workspace, six ways](../create_rust_proj/README.md) makes the folder disposable.

**Or describe the files in a `rust-project.json`.** This is `rust-analyzer`'s manifest for projects Cargo does not build, and `bash install.sh --loose <folder>` writes one with a crate per `.rs` file, plus a `.zed/settings.json` that turns check-on-save off for that folder. The field that matters is `sysroot_src`, the standard library's source. Measured with `rust-analyzer analysis-stats` on one 100-line threaded program:

| `rust-project.json` names | Expressions inferred | With an unknown type | `rust-analyzer diagnostics` |
|---|---|---|---|
| `sysroot` | 170 | 63 (37%) | reports errors |
| `sysroot` and `is_workspace_member` | 170 | 63 (37%) | reports errors |
| `sysroot` and `sysroot_src` | 479 | 0 | clean |

`sysroot_src` points into rustup's `rust-src` component; the installer stops and says `rustup component add rust-src` if it is missing. The file lists the `.rs` files that existed when it was written, so after adding one, delete `rust-project.json` and run `--loose` again. Whether Zed passes a folder's own `.zed/settings.json` to `rust-analyzer` was not confirmed here; the setting is `lsp.rust-analyzer.initialization_options.checkOnSave` from [Zed's Rust page ↗](https://zed.dev/docs/languages/rust). (Not machine-checked here.)

## Trap 3: a task runs what is on disk, not what you see

On this machine, a run printed a greeting the editor no longer showed. The file's timestamps give the order:

| Time | What happened |
|---|---|
| 23:15:56 | `target/debug/r` is built from `src/bin/r.rs`, which says `Hello, sdf!` |
| later | the greeting is edited in Zed, and not saved |
| 23:28 | the run button next to `fn main` starts Zed's *run r* task: `Finished … in 0.00s`, then `Hello, sdf!` |
| 23:28:22 | autosave writes `src/bin/r.rs`, as focus moves to the terminal the task opened |

Cargo was right: nothing on disk had changed since 23:15:56. A Zed task's `save` field defaults to `"none"` ([Tasks ↗](https://zed.dev/docs/tasks)), so the kit's task sets `"save": "all"`. Autosave does not cover it either: `"on_focus_change"` ([Configuring Zed ↗](https://zed.dev/docs/configuring-zed)) saves when focus *leaves* the file, and clicking the run button in the gutter did not move focus until the task was already running.

The run button's task is Zed's own, so `"save"` cannot be added to it — but it can be replaced. `rust-main` is the tag Zed's Rust grammar puts on `fn main` ([`runnables.scm` ↗](https://github.com/zed-industries/zed/blob/main/crates/grammars/src/rust/runnables.scm)), and a project task carrying that tag takes over the button ([Tasks ↗](https://zed.dev/docs/tasks)). `bash install.sh --cargo <project>` writes that task into [`.zed/tasks.json`](zed/cargo-project/tasks.json): `cargo run --bin "$ZED_STEM"` with `"save": "all"`. It belongs in the project, not in `~/.config/zed`: `$ZED_STEM`, the open file's name without `.rs`, is the binary's name only in the `src/bin/` layout. (Not machine-checked here: the button's take-over, as opposed to the task's command, which is.)

## F4: debugging the open file

`--cargo` also writes [`.zed/debug.json`](zed/cargo-project/debug.json):

```json
{
  "label": "Build & debug this src/bin file",
  "adapter": "CodeLLDB",
  "request": "launch",
  "build": { "command": "cargo", "args": ["build", "--bin", "$ZED_STEM"] },
  "program": "$ZED_WORKTREE_ROOT/target/debug/$ZED_STEM",
  "cwd": "$ZED_WORKTREE_ROOT",
  "sourceLanguages": ["rust"]
}
```

Click in the gutter to set a breakpoint, press F4, and pick it from the *Debug* tab. The `build` step runs before every launch, so the debugger never starts an old binary, and Zed's Rust page says CodeLLDB needs `sourceLanguages` for Rust. The first launch downloads CodeLLDB. (Not machine-checked here: a debugging session; the build command is.)

## This library in Zed

The library has no `Cargo.toml` either — every example is compiled by bare `rustc` — so opening it in Zed is trap 2 at scale. `python3 tools/write_cargo_toml.py` writes a gitignored manifest with one binary per example, the same one [the RustRover note in CONTRIBUTING](../../CONTRIBUTING.md) describes, and `rust-analyzer` in Zed reads it too.

## See also

- [Choosing an editor](../editors/README.md) — Zed against RustRover, VS Code, Neovim, Helix and Emacs
- [Neovim with LazyVim](../neovim_setup/README.md) and [RustRover setup](../rustrover_setup/README.md) — the same job in the other windows
- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — what `rustc` on a loose file does, which is the path Ctrl+R takes outside a Cargo package

## Po polsku

Zed uruchamia projekt Cargo bez żadnej konfiguracji, ale szybki eksperyment trafia na trzy domyślne ustawienia. Zed chce **folderu**, a nie pojedynczego pliku (`zed .` w katalogu projektu). `rust-analyzer` chce **`Cargo.toml`** — najprościej zrobić z katalogu pakiet Cargo z plikami w `src/bin/`, gdzie każdy plik jest osobnym programem; nazwa pakietu nie może zaczynać się od cyfry. A zadanie (*task*) uruchamia to, co jest **zapisane na dysku**, bo domyślnie niczego nie zapisuje (`"save": "none"`), a autozapis przy zmianie fokusu przychodzi za późno dla przycisku ▷. `bash install.sh` ustawia Ctrl+R: zapisz wszystko i uruchom bieżący plik Rusta.

**Szukaj po polsku:** Zed skrót klawiszowy uruchom plik · `This project has no folders open` · `Failed to discover workspace` · `rust-project.json sysroot_src` · autozapis Zed
