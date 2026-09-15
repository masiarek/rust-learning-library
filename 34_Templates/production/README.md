# Production: strict

**Level:** reference · a template to copy

**One line:** A whole project to copy for code that ships — clippy's default groups and `pedantic` denied, 22 hand-picked lints denied, `unsafe` forbidden, and a release profile that keeps overflow checks. `cargo run` still runs anything that compiles; `cargo ci` is the one command a pull request has to pass.

## Copy it

```bash
cp -R 34_Templates/production/template ~/code/my_service
```

Then change `name` in `Cargo.toml` — [it matters](#why-each-part-is-there). Or let the script copy, rename, and make diagnostics name files in full:

```bash
python3 34_Templates/automation/lint_profiles.py new ~/code/my_service --profile production
```

For a project that already exists, paste the blocks below, or run `lint_profiles.py apply --profile production`. Moving a project from [Training](../training/README.md) needs `--force` as well, which also *removes* training's four `allow`s — without it they survive the switch. [Automation](../automation/README.md) has the details.

## The files

Every block below is pasted from the template folder by tools/run_examples.py, and CI fails if a block and its file differ — so what you copy here is exactly what the script writes.

### Cargo.toml

<!-- file:template/Cargo.toml -->
```toml title="template/Cargo.toml"
# Production template: strict.
#
# Denies what has one right answer, using stable lint groups only. Every `deny` fails
# `cargo clippy`, so `cargo ci` (.cargo/config.toml) cannot pass with a panic path, a
# silent `as` cast or an unexplained `#[allow]` in it. No `nursery`: those lints are
# still in development, and at `deny` a toolchain bump could fail code that did not change.
#
# Change `name` to your project's folder name; everything else can stay.

[package]
name = "app"
version = "0.1.0"
edition = "2024"
rust-version = "1.98"
publish = false                                   # a published crate also needs license, description, repository

[dependencies]
anyhow = "1"                                      # errors in a binary: `?` with context
thiserror = "2"                                   # errors in a library: a typed enum, derived
clap = { version = "4", features = ["derive"] }   # command-line arguments
rand = "0.10"                                     # random numbers
regex = "1"                                       # regular expressions
serde = { version = "1", features = ["derive"] }  # #[derive(Serialize, Deserialize)]
serde_json = "1"                                  # ...to and from JSON

[profile.release]
lto = "thin"                                      # optimise across crate boundaries at link time
codegen-units = 1                                 # one unit per crate: slower to build, more room to optimise
overflow-checks = true                            # integer overflow panics in release too, instead of wrapping

[lints.rust]
unsafe_code = "forbid"                            # `forbid`: no #[allow] can switch it back on
unused_must_use = "deny"
rust_2018_idioms = { level = "deny", priority = -1 }
trivial_casts = "deny"
trivial_numeric_casts = "deny"
unused_qualifications = "deny"
let_underscore_drop = "deny"
redundant_lifetimes = "deny"
unused_lifetimes = "deny"
missing_debug_implementations = "deny"

[lints.clippy]
# Clippy's default groups and the idiom group, as errors.
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
# No panics outside tests (clippy.toml carves tests out).
unwrap_used = "deny"
expect_used = "deny"
indexing_slicing = "deny"
string_slice = "deny"
panic = "deny"
panic_in_result_fn = "deny"
todo = "deny"
unimplemented = "deny"
unreachable = "deny"
exit = "deny"
dbg_macro = "deny"
# Conversions that say what they do.
as_conversions = "deny"
lossy_float_literal = "deny"
# Names, patterns and attributes.
shadow_unrelated = "deny"
wildcard_enum_match_arm = "deny"
rest_pat_in_fully_bound_structs = "deny"
allow_attributes_without_reason = "deny"
# Errors, pointers and memory.
map_err_ignore = "deny"
unused_result_ok = "deny"
clone_on_ref_ptr = "deny"
rc_buffer = "deny"
mem_forget = "deny"
# For a library crate, add:
#   missing_docs = "deny"        (under [lints.rust]) every public item documented
#   clippy::missing_errors_doc   already denied by pedantic: say when a function returns Err
```
<!-- /file -->

### clippy.toml

<!-- file:template/clippy.toml -->
```toml title="template/clippy.toml"
# Clippy's own settings. Lint LEVELS live in Cargo.toml; this file holds carve-outs
# and thresholds. The MSRV comes from `rust-version` in Cargo.toml.

# Inside #[test] and #[cfg(test)], unwrap, expect, panic, indexing and dbg! are how a
# test says "this must hold", so the lints about them stay quiet there. Nowhere else.
allow-unwrap-in-tests = true
allow-expect-in-tests = true
allow-panic-in-tests = true
allow-indexing-slicing-in-tests = true
allow-dbg-in-tests = true
```
<!-- /file -->

### .cargo/config.toml

<!-- file:template/.cargo/config.toml -->
```toml title="template/.cargo/config.toml"
# Cargo settings for this project only.

[alias]
# `cargo ci`: what a pull request has to pass. Cargo.toml already denies the policy;
# this adds every remaining warning, tests and examples included.
ci = ["clippy", "--all-targets", "--", "-D", "warnings"]
```
<!-- /file -->

### rust-toolchain.toml

<!-- file:template/rust-toolchain.toml -->
```toml title="template/rust-toolchain.toml"
# The compiler this project builds with. rustup reads this file and installs the
# version if it is missing. Bump it on purpose, in a commit of its own, and let
# `cargo ci` tell you what the new clippy thinks before anything else merges.

[toolchain]
channel = "1.98.0"
components = ["clippy", "rustfmt", "rust-analyzer"]
```
<!-- /file -->

### src/main.rs

<!-- file:template/src/main.rs -->
````rust title="template/src/main.rs"
//! `app`: which of the given words match a pattern, and one of them picked at random.
//!
//! ```text
//! cargo run -- --pattern 'ing$' reading writing arithmetic
//! ```

use anyhow::{Context, Result};
use clap::Parser;
use rand::seq::IndexedRandom;
use regex::Regex;
use serde::Serialize;

/// Report which words match a regular expression.
#[derive(Debug, Parser)]
struct Args {
    /// The regular expression each word is matched against.
    #[arg(long, default_value = "ing$")]
    pattern: String,

    /// The words to test.
    #[arg(default_values = ["reading", "writing", "arithmetic"])]
    words: Vec<String>,
}

/// What `app` prints, as JSON.
#[derive(Debug, Serialize)]
struct Report<'a> {
    pattern: &'a str,
    matched: Vec<&'a str>,
    picked: &'a str,
}

/// The one failure this program names itself; `anyhow` carries the rest.
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("no word matches {0:?}")]
    NoMatch(String),
}

fn main() -> Result<()> {
    let args = Args::parse();
    let pattern = Regex::new(&args.pattern)
        .with_context(|| format!("{:?} is not a valid regular expression", args.pattern))?;
    let matched: Vec<&str> = args
        .words
        .iter()
        .map(String::as_str)
        .filter(|word| pattern.is_match(word))
        .collect();
    let Some(&picked) = matched.choose(&mut rand::rng()) else {
        return Err(AppError::NoMatch(args.pattern).into());
    };
    let report = Report {
        pattern: &args.pattern,
        matched,
        picked,
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
````
<!-- /file -->

`cargo ci` passes it with nothing to report, and `cargo build --release` builds it with the profile above. Run with its default arguments:

```text title="cargo run, in template/ — real output; the pick varies"
{
  "pattern": "ing$",
  "matched": [
    "reading",
    "writing"
  ],
  "picked": "reading"
}
```

And the two ways it refuses, each exiting non-zero rather than panicking:

```text title="cargo run -- --pattern zzz — real output"
Error: no word matches "zzz"
```

```text title="cargo run -- --pattern '(' — real output"
Error: "(" is not a valid regular expression

Caused by:
    regex parse error:
        (
        ^
    error: unclosed group
```

The first is the `thiserror` enum; the second is `anyhow`'s `with_context` wrapped around regex's own error, and `Caused by:` is `anyhow` printing the chain.

## What it does to a first attempt

Take [the first attempt from the training page](../training/README.md#what-it-does-to-a-first-attempt) — it indexes, unwraps, casts with `as`, slices a string and matches with `_`. `cargo run` still runs it: no clippy level stops a program running. `cargo ci` refuses it with six errors:

| Line | Lint | Where the `deny` comes from |
|---|---|---|
| 11 | `indexing_slicing` | `[lints.clippy]` |
| 13 | `unwrap_used` | `[lints.clippy]` |
| 17 | `cast_lossless` | `pedantic = { level = "deny", … }` |
| 17 | `as_conversions` | `[lints.clippy]` |
| 23 | `string_slice` | `[lints.clippy]` |
| 28 | `wildcard_enum_match_arm` | `[lints.clippy]` |

Training reported two more on the same file, and production drops both on purpose: `integer_division`, because in code that ships `7 / 2` usually means what it says, and `needless_type_cast`, because it lives in `nursery`. The version with every fix applied — on the training page — passes `cargo ci`.

## The release profile

| Setting | What it does |
|---|---|
| `lto = "thin"` | optimises across crate boundaries at link time, so a small function in a dependency can be inlined into yours |
| `codegen-units = 1` | compiles each crate as one unit: a slower build, more room for the optimiser |
| `overflow-checks = true` | an integer overflow panics in release as it does in a debug build, instead of wrapping to a wrong number |

The third one is the policy decision. Release builds wrap on overflow by default; this template trades the check's cost for a crash that says where, over a value that is silently wrong.

## For a library crate

- Add `missing_docs = "deny"` under `[lints.rust]`. `pedantic` already denies `missing_errors_doc` and `missing_panics_doc`, so every public function that can fail says how.
- Use `thiserror` for the error type and leave `anyhow` to binaries: a library's caller should be able to `match` on what went wrong.
- Remove `publish = false`, and add `license`, `description` and `repository`.

## Why each part is there

- **`forbid` for `unsafe_code`**, where everything else is `deny`: no `#[allow]` further down can switch a `forbid` back on.
- **`all` and `pedantic` at `deny`.** `all` is clippy's five default groups — 489 lints — so one line turns every default warning into an error. See [Clippy](../clippy/README.md) for what each group holds.
- **No `nursery`.** Its lints are still in development; at `deny`, a toolchain bump could fail code that did not change.
- **The toolchain is pinned** so that bump happens in a commit of its own, where `cargo ci` shows what the new clippy thinks before anything else merges.
- **`cargo ci` passes `-D warnings` after `--`**, straight to clippy, so rustc's own warn-by-default lints fail the build too — everything `Cargo.toml` does not already deny.
- **Change `name`.** Two projects with the same package name, sharing a `CARGO_TARGET_DIR`, can pass a stale check. In testing this template, `cargo ci` in a second project named `app` printed `Finished` and exited 0 on the six-error first attempt above, because the target directory already held a passing `app`; touching `src/main.rs` brought the six errors back.

## See also

- [Training](../training/README.md) — the relaxed twin, where the same lints warn
- [Clippy](../clippy/README.md) — the nine groups, where each setting lives, and the commands
- [Automation](../automation/README.md) — `lint_profiles.py new`, `apply` and `check`
- [Strict clippy lints](../../05_Tooling/strict_lints/README.md) — the panic set, and `arithmetic_side_effects`, which neither template enables

---

*The outputs, clippy results and the release build above were captured on 2026-09-14 by running cargo 1.98.0 and clippy in the template and in scratch copies of it; they are verified once, since tools/run_examples.py compiles bare `rustc` examples and runs neither Cargo nor clippy. The file blocks are checked on every commit.*

## Po polsku

To jest szablon **produkcyjny**: surowy. Grupy domyślne clippy (`all`, czyli 489 lintów) i `pedantic` są na `deny`, do tego 22 wybrane linty z grupy `restriction`, a `unsafe_code` na `forbid`, którego żadne `#[allow]` niżej nie odwoła. Mimo to `cargo run` uruchomi wszystko, co się kompiluje — poziom lintu clippy nigdy nie blokuje uruchomienia programu. Bramką jest `cargo ci`: `cargo clippy --all-targets -- -D warnings`, więc odpadają też zwykłe ostrzeżenia rustc.

Profil `release` ma trzy ustawienia: `lto = "thin"` (optymalizacja między crate'ami przy linkowaniu), `codegen-units = 1` (wolniejsza kompilacja, więcej miejsca dla optymalizatora) i `overflow-checks = true` — przepełnienie liczby całkowitej w wersji release kończy się paniką ze wskazaniem miejsca, zamiast cichym zawinięciem do złej wartości. Nie ma tu `nursery`: linty w budowie przy `deny` potrafią po aktualizacji kompilatora wywalić kod, który się nie zmienił. I jedna praktyczna uwaga, zmierzona przy testach: zmień `name` w `Cargo.toml`. Dwa projekty o tej samej nazwie pakietu ze wspólnym `CARGO_TARGET_DIR` potrafią zgłosić nieaktualny wynik — `cargo ci` wypisało `Finished` na kodzie, który ma sześć błędów.

**Szukaj po polsku:** szablon projektu Rust produkcja · `clippy deny warnings` · `profile.release lto` · `overflow-checks` · `unsafe_code forbid`
