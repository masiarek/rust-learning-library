# cargo-temp: the crate that already does it

**Level:** 101 · for newcomers

**One line:** `cargo temp anyhow rand` makes a project, adds the crates, and drops you into a shell inside it; typing `exit` deletes the project — the whole disposable workflow as one installed subcommand, with an editor hook, at the cost of the two learner defaults it has no way to write.

## Install and run

```sh
cargo install --locked cargo-temp     # 27.6 s here, cargo-temp 0.4.1
cargo temp anyhow rand
```

```text title="Real output — cargo temp anyhow, first run, exit typed at the prompt"
[INFO cargo-temp] Config file created at: /Users/amasa/.config/cargo-temp/config.toml
    Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[INFO cargo-temp] Temporary project created at: /Users/amasa/.cache/cargo-temp/tmp-vwV0sS

To preserve the project when exiting the shell, don't forget to delete the `TO_DELETE` file.
To exit the project, you can type "exit" or use `Ctrl+D`
```

Inside that shell, `pwd`, the manifest, and the folder:

```text
/Users/amasa/.cache/cargo-temp/tmp-vwV0sS
[package]
name = "tmp-vwv0ss"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "*"
.  ..  .git  .gitignore  Cargo.toml  TO_DELETE  src
```

After `exit`, `ls -a ~/.cache/cargo-temp/` shows only `.` and `..` — the project is gone. That is the entire contract: the project lives as long as the shell it opened, and `TO_DELETE` is the switch. `rm TO_DELETE` before exiting and the folder survives.

## What it does that a script does not

| Feature | Command |
|---|---|
| a version or a feature on the way in | `cargo temp tokio=1.48+full`, `clap+derive` |
| a crate straight from git or a local path | `cargo temp https://github.com/rust-random/rand#0.9.0`, `cargo temp ../my_lib` |
| a library crate | `cargo temp --lib` |
| a Criterion benchmark wired up | `cargo temp --bench` |
| a throwaway **git worktree** of the repo you are standing in | `cargo temp --worktree` — pruned on exit |
| a throwaway clone of any repo | `cargo temp --git <url>` — history truncated to one commit |

The worktree row is the one a learner meets later and is glad to have: "try this refactor on a copy of my project, and throw the copy away" is exactly `cargo temp --worktree`.

## The editor hook

The config file it wrote on first run, `~/.config/cargo-temp/config.toml`, takes an editor. When one is set, `cargo temp` opens the editor **instead of** a shell, and deletes the project when the editor process exits:

```toml
editor = "code"
editor_args = ["--wait", "--new-window"]
```

`code --wait` returns when that window is closed, so the project disappears the moment you close it — the tightest lifetime of any route on [the hub page](../README.md).

RustRover is the awkward one. `open -W -a RustRover <dir>` blocks *"until the used applications are closed (even if they were already running)"* — measured here: the call had not returned after the IDE had been open for a minute with the project loaded, and it returns only when RustRover quits. So this configuration:

```toml
editor = "open"
editor_args = ["-W", "-a", "RustRover"]
```

means *quitting RustRover deletes the scratch project*, which is defensible for a throwaway but is not "close the tab". If that is not what you want, leave the editor unset, and from the shell it opens run `open -a RustRover .` yourself; `exit` after you are done in the IDE.

## What it cannot write

Two things a learner wants in a throwaway are Cargo manifest lines, and `cargo temp` writes the manifest that `cargo new` writes plus the `[dependencies]` you named — nothing else. The [four-line unused block](../../scaffolding/README.md) and the [absolute-path remap](../../rustrover_setup/README.md) have to be added by hand every time, which in a project that lives for twenty minutes means never. [`scratch.sh`](../scratch_sh/README.md) and [`scratch.py`](../scratch_py/README.md) exist because of those two lines.

Three smaller things, all visible in the transcript above:

- **`anyhow = "*"`** — a wildcard requirement rather than the real version `cargo add` would have written. `Cargo.lock` pins what actually resolved, so the build is reproducible for as long as the folder exists, and the folder is designed not to exist. Fine here; not a habit to carry into a real manifest.
- **A `.git` is initialised** by default. `vcs = "none"` in the config turns it off; there is nothing to commit in a folder that deletes itself.
- **The project is under `~/.cache/cargo-temp/`**, not `$TMPDIR` — so a project kept with `rm TO_DELETE` is not swept by the OS, and `preserved_project_dir` in the config moves kept projects somewhere you will find them.

## See also

- [The hub — a disposable Rust workspace, six ways](../README.md) — the comparison table this page is one row of
- [`scratch.sh`](../scratch_sh/README.md) — the same lifetime under `$TMPDIR`, with the two learner defaults written in
- [A throwaway that needs a crate](../../scratch_with_a_crate/README.md) — the three commands `cargo temp` is a wrapper for
- [cargo-temp on crates.io ↗](https://crates.io/crates/cargo-temp) · [the configuration template ↗](https://github.com/yozhgoor/cargo-temp/blob/main/config_template.toml) — every key, including `[[subprocess]]` for a `bacon` pane started beside the shell
