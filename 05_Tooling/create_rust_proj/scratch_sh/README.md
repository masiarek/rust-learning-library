# scratch.sh: mktemp, cargo init, five crates, and the IDE

**Level:** 101 · for newcomers

**One line:** One shell script makes a Cargo project in the system temp folder, writes the two lines that make a scratch project pleasant, adds the five crates a learner reaches for, proves it with `cargo run`, and opens it in RustRover or VS Code — eleven seconds from nothing to a window you can type into, and nothing to clean up because `$TMPDIR` is not a place you look.

## Run it

[`scratch.sh` ↗](https://github.com/masiarek/rust-learning-library/blob/master/05_Tooling/create_rust_proj/scratch_sh/scratch.sh) — put it somewhere on `PATH`, or run it by path.

```sh
scratch.sh                       # anyhow rand itertools serde serde_json, then print the path
scratch.sh --rustrover           # ...and open it in RustRover
scratch.sh --code                # ...or in VS Code
scratch.sh --crates "regex clap" # a different crate list (space-separated)
scratch.sh --no-crates           # std only: no network, ~2 s
scratch.sh --prune               # delete every earlier scratch project first
```

```text title="Real output — scratch.sh, cargo 1.98.0, registry already warm"
$ time scratch.sh
  cargo init  /var/folders/bc/gzwjtz8d6mx3l2f9yyf_p8yw0000gn/T/rust-scratch.tMdSfW
  wrote       [lints.rust] + .cargo/config.toml
  cargo add   anyhow rand itertools serde serde_json
[/var/folders/bc/gzwjtz8d6mx3l2f9yyf_p8yw0000gn/T/rust-scratch.tMdSfW/src/main.rs:14:5] n = 184
2, 4, 6, 8, 10
{"x":3,"y":4}
  cargo run   ok

cd /var/folders/bc/gzwjtz8d6mx3l2f9yyf_p8yw0000gn/T/rust-scratch.tMdSfW
scratch.sh  11.000 total
```

The `dbg!` line is the first thing worth looking at: it names the file by its **absolute** path. That is the remap in step 2 at work, and it is the difference between a diagnostic you can click from any terminal and `--> src/main.rs:14:5`, which is useless once two scratch projects are open.

## What it does, step by step

**0. `--prune`, if asked.** `rm -rf` on every `$TMPDIR/rust-scratch.*` from earlier runs. The script never deletes anything otherwise: a scratch project you made this morning is still there after lunch, which is the point of a temp *folder* over a temp *shell*.

**1. `mktemp -d "$TMPDIR/rust-scratch.XXXXXX"`, then `cargo init --name scratch --vcs none --edition 2024`.** A unique folder under the per-user temp directory (`/var/folders/…/T/` on macOS), a manifest, and a hello-world `main.rs`. `--vcs none` because there is nothing to commit in a folder that exists to be forgotten; `--name scratch` because the folder's name has six random characters in it and Cargo would otherwise refuse or mangle it. One macOS detail: `$TMPDIR` ends in a slash, so the script strips it (`${tmp%/}`) or every path prints with a `//`.

**2. The two learner defaults.** Appended to `Cargo.toml`:

```toml
[lints.rust]
unused_variables = "allow"
unused_imports = "allow"
unused_mut = "allow"
dead_code = "allow"
```

and written to `.cargo/config.toml`:

```toml
[build]
rustflags = ["--remap-path-prefix==/var/folders/…/T/rust-scratch.tMdSfW/"]
```

The first turns off the four warnings a scratch file exists to trigger — five ways to make a `String`, each bound to a name nobody reads, is five `unused_variables`. It is four named lints and **not** `unused = "allow"`, because that group also holds `unused_must_use`, and an ignored `Result` is a dropped error even in a throwaway. The second is the absolute-path trick: an *empty* old prefix (the `==`) matches every relative path component and no absolute one, so your files get spelled out and `std`'s and `~/.cargo/registry`'s do not. Both are explained at length on [Scaffolding a practice tree](../../scaffolding/README.md); here they are just two heredocs.

**3. `cargo add anyhow rand itertools serde serde_json --features serde/derive`.** The crates, at whatever version crates.io currently serves — `rand = "0.10.2"`, `serde = { version = "1.0.229", features = ["derive"] }` on the day of the transcript. `cargo add` writes real versions, never a `*`. The `serde/derive` feature is added only when `serde` is in the list, because `serde` without its derive macros is a crate you cannot use.

Then `src/main.rs` is overwritten with a program that touches every one of the five, so the build in step 4 proves them all:

```rust
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
```

With `--crates` naming a different list, the seed is a plain `println!` instead — the script cannot know what your crates export.

**4. `cargo run --quiet`.** The proof. A scratch project that has already compiled its crates and printed something is one you can start typing into; one that has not is one you first have to check. `set -e` means a failure here stops the script before it opens an editor on a broken project.

**5. `open -a RustRover "$dir"` or `code --new-window "$dir"`.** Only with the flag. [Opening it in the IDE](../open_in_the_ide/README.md) has what was measured about each — including why the script says `open -a RustRover` and not `rustrover`.

Last line: `cd <path>`, printed so it can be copied.

## What the crates cost

| `--crates` | First `cargo run` | `target/` |
|---|---|---|
| `--no-crates` | 2.0 s | 1.1 MB |
| `"anyhow rand itertools"` | 4.5 s | 23 MB |
| default: `+ serde serde_json` | 11.0 s | 71 MB |
| `"… regex clap chrono thiserror"` | 14.0 s | 184 MB |

Registry already warm in every row: the crates were downloaded once, by the first project on this machine to use them, into `~/.cargo/registry`, which is shared. What each row pays for is compilation, and `target/` is where it goes — which is why `--prune` exists: a dozen of the default row is close to a gigabyte in a folder nobody visits.

## Cleaning up

There is nothing to do. The project is under `$TMPDIR`, which the system owns; it is not in a folder you browse, it is not in git, and the next `--prune` removes it. What you want to keep, `cp -r` somewhere first — or start it with [`scratch.py --keep`](../scratch_py/README.md), which is the same script with a lifetime tied to the editor window instead.

## See also

- [The hub — a disposable Rust workspace, six ways](../README.md) — the workflow this script is one route through, and the review of the ten-line version it grew from
- [`scratch.py`](../scratch_py/README.md) — the same project, deleted when the VS Code window closes
- [Scaffolding a practice tree](../../scaffolding/README.md) — the two learner defaults, explained, and `doctor`, which checks them
- [A throwaway that needs a crate](../../scratch_with_a_crate/README.md) — the three commands this script wraps, done by hand once so the script is not magic
