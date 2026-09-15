# Training: relaxed, and pedantic on purpose

**Level:** reference · a template to copy

**One line:** A whole project to copy — `Cargo.toml`, `clippy.toml`, the toolchain and an example — in which nothing stops `cargo run`, clippy's `pedantic` and `nursery` groups plus 31 hand-picked lints *explain* every line that has a better form, and `cargo strict` turns those explanations into errors when you want the lesson enforced.

## Copy it

```bash
cp -R 34_Templates/training/template ~/RustroverProjects/my_project
```

Then change `name` in `Cargo.toml`. Or let the script copy, rename, and make diagnostics name files in full:

```bash
python3 34_Templates/automation/lint_profiles.py new ~/RustroverProjects/my_project
```

For a project that already exists, paste the blocks below into its files, or run `lint_profiles.py apply` — it adds the lint and profile tables and leaves `[package]` and `[dependencies]` alone. [Automation](../automation/README.md) has the details, and [Production](../production/README.md) is the strict twin of this page.

## The files

Every block below is pasted from the template folder by tools/run_examples.py, and CI fails if a block and its file differ — so what you copy here is exactly what the script writes.

### Cargo.toml

<!-- file:template/Cargo.toml -->
```toml title="template/Cargo.toml"
# Training template: relaxed, and pedantic on purpose.
#
# Everything educational is at `warn`, so `cargo run` never stops for style while
# `cargo clippy` -- or RustRover, with Clippy as its linter -- names the idiomatic
# version of each line. The four lints about unused names are off: a practice file
# binds names only to show a form. `cargo strict` (.cargo/config.toml) runs the
# same lints with every warning an error.
#
# Change `name` to your project's folder name; everything else can stay.

[package]
name = "practice"
version = "0.1.0"
edition = "2024"
rust-version = "1.98"
publish = false

[dependencies]
anyhow = "1"                                     # `?` on any error in main, with context
rand = "0.10"                                    # random numbers
regex = "1"                                      # regular expressions
serde = { version = "1", features = ["derive"] } # #[derive(Serialize, Deserialize)]
serde_json = "1"                                 # ...to and from JSON

# Your own code stays unoptimised and debuggable; dependencies are built
# optimised, once, so a regex or a random number costs what it will in release.
[profile.dev.package."*"]
opt-level = 2

[lints.rust]
# Practice-file noise: a name bound only to show a form is not a mistake.
unused_variables = "allow"
unused_imports = "allow"
unused_mut = "allow"
dead_code = "allow"
# Idioms rustc knows and does not warn about by default.
rust_2018_idioms = { level = "warn", priority = -1 }
unsafe_code = "warn"
trivial_casts = "warn"
trivial_numeric_casts = "warn"
unused_qualifications = "warn"
let_underscore_drop = "warn"
unit_bindings = "warn"
redundant_lifetimes = "warn"
unused_lifetimes = "warn"

[lints.clippy]
# The two teaching groups: the idiom std expects, and the lints still in development.
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
# Panics. Each one points at the API that returns an Option or a Result instead.
unwrap_used = "warn"
expect_used = "warn"
indexing_slicing = "warn"
string_slice = "warn"
panic = "warn"
panic_in_result_fn = "warn"
todo = "warn"
unimplemented = "warn"
unreachable = "warn"
exit = "warn"
# Conversions and arithmetic that say what they do.
as_conversions = "warn"
integer_division = "warn"
lossy_float_literal = "warn"
# Names, patterns and attributes.
shadow_unrelated = "warn"
wildcard_enum_match_arm = "warn"
rest_pat_in_fully_bound_structs = "warn"
ref_patterns = "warn"
allow_attributes_without_reason = "warn"
# Smaller idioms, each with a one-line fix.
if_then_some_else_none = "warn"
iter_over_hash_type = "warn"
verbose_file_reads = "warn"
try_err = "warn"
empty_structs_with_brackets = "warn"
unnecessary_self_imports = "warn"
string_lit_chars_any = "warn"
undocumented_unsafe_blocks = "warn"
map_err_ignore = "warn"
clone_on_ref_ptr = "warn"
rc_buffer = "warn"
mem_forget = "warn"
dbg_macro = "warn"
# Measured and left out:
#   min_ident_chars          `i`, `x` and `n` are fine names
#   arithmetic_side_effects  rejects `n + 1`, and changes how everything reads
#   shadow_reuse             rejects `let line = line.trim();`, which is idiomatic
#   let_underscore_must_use  rejects `let _ =`, the fix rustc's own message suggests
```
<!-- /file -->

### clippy.toml

<!-- file:template/clippy.toml -->
```toml title="template/clippy.toml"
# Clippy's own settings. Lint LEVELS live in Cargo.toml; this file holds carve-outs
# and thresholds. The MSRV comes from `rust-version` in Cargo.toml.

# Inside #[test] and #[cfg(test)], unwrap, expect, panic, indexing and dbg! are how a
# test says "this must hold", so the lints about them stay quiet there.
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
# `cargo strict`: the lints in Cargo.toml with every warning an error, and the four
# unused-name lints back on. Relaxed while you write, strict before you call it done.
# The flags are inline because a file path here is resolved from the directory
# cargo runs in, so `--config some-file.toml` would break in src/.
strict = ["clippy", "--all-targets", "--config", "build.rustflags = ['-W', 'unused', '-D', 'warnings']"]
```
<!-- /file -->

### rust-toolchain.toml

<!-- file:template/rust-toolchain.toml -->
```toml title="template/rust-toolchain.toml"
# The compiler this project builds with. rustup reads this file and installs the
# version if it is missing. Bump it on purpose, in a commit of its own.

[toolchain]
channel = "1.98.0"
components = ["clippy", "rustfmt", "rust-analyzer"]
```
<!-- /file -->

### src/main.rs

<!-- file:template/src/main.rs -->
```rust title="template/src/main.rs"
//! A practice program the training template has nothing to say about: a die roll
//! from `rand`, words from `regex`, JSON from `serde_json`, and every error passed
//! up with `?` through `anyhow`.

use anyhow::{Context, Result};
use rand::RngExt;
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Roll {
    sides: u8,
    value: u8,
}

fn main() -> Result<()> {
    let value = rand::rng().random_range(1..=6);
    let roll = Roll { sides: 6, value };
    println!("{}", serde_json::to_string(&roll)?); // {"sides":6,"value":4} -- the value varies

    let words = Regex::new(r"\b[a-z]+ing\b").context("the pattern should compile")?;
    let text = "reading and writing, then testing the thing";
    let found: Vec<&str> = words.find_iter(text).map(|word| word.as_str()).collect();
    println!("{found:?}"); // ["reading", "writing", "testing", "thing"]

    let total: u32 = "42".parse().context("not a whole number")?;
    println!("{}", total + 1); // 43

    Ok(())
}
```
<!-- /file -->

```text title="cargo run, in template/ — real output; the roll varies"
{"sides":6,"value":3}
["reading", "writing", "testing", "thing"]
43
```

`cargo clippy --all-targets` and `cargo strict` both finish with nothing to report.

## What it does to a first attempt

A first attempt that compiles and runs:

```rust title="src/main.rs — a first attempt"
#[derive(Debug, Clone, Copy)]
enum Status {
    Draft,
    Published,
    Archived,
}

fn main() {
    let scores = [90, 72, 85];
    let i = 1;
    println!("{}", scores[i]); // 72

    let parsed: i32 = "42".parse().unwrap();
    println!("{parsed}"); // 42

    let small: u8 = 200;
    let wide = small as i64;
    println!("{wide}"); // 200

    println!("{}", 7 / 2); // 3

    let word = "héllo";
    println!("{}", &word[0..3]); // hé

    for status in [Status::Draft, Status::Published, Status::Archived] {
        let shown = match status {
            Status::Published => "visible",
            _ => "hidden",
        };
        println!("{status:?}: {shown}"); // Draft: hidden, Published: visible, Archived: hidden
    }
}
```

`cargo run` runs it and prints every line above. `cargo clippy` explains eight things about it, as warnings:

| Line | Lint | Clippy's message | The fix |
|---|---|---|---|
| 11 | [`indexing_slicing` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#indexing_slicing) | indexing may panic | `scores.get(i)`, which returns an `Option` |
| 13 | [`unwrap_used` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#unwrap_used) | used `unwrap()` on a `Result` value | `?`, with `main` returning `anyhow::Result<()>` |
| 16 | [`needless_type_cast` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#needless_type_cast) — from `nursery` | this binding is defined as `u8` but is always cast to `i64` | convert once, with `From` |
| 17 | [`cast_lossless` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#cast_lossless) — from `pedantic` | casts from `u8` to `i64` can be expressed infallibly using `From` | `i64::from(small)` |
| 17 | [`as_conversions` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#as_conversions) | using a potentially dangerous silent `as` conversion | the same `i64::from(small)` |
| 20 | [`integer_division` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#integer_division) | integer division | `7_i32.div_euclid(2)`, which says you meant it |
| 23 | [`string_slice` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#string_slice) | indexing into a string may panic if the index is within a UTF-8 character | `word.get(0..3)`, which returns `None` instead |
| 28 | [`wildcard_enum_match_arm` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#wildcard_enum_match_arm) | wildcard match will also match any future added variants | name them: `Status::Draft \| Status::Archived` |

`cargo strict` reports the same eight as errors and fails. The version with every fix applied passes `cargo strict` with nothing to report:

```rust title="src/main.rs — every fix applied"
#[derive(Debug, Clone, Copy)]
enum Status {
    Draft,
    Published,
    Archived,
}

fn main() -> anyhow::Result<()> {
    let scores = [90, 72, 85];
    let i = 1;
    println!("{:?}", scores.get(i)); // Some(72)

    let parsed: i32 = "42".parse()?;
    println!("{parsed}"); // 42

    let small: u8 = 200;
    let wide = i64::from(small);
    println!("{wide}"); // 200

    println!("{}", 7_i32.div_euclid(2)); // 3

    let word = "héllo";
    println!("{:?}", word.get(0..3)); // Some("hé")

    for status in [Status::Draft, Status::Published, Status::Archived] {
        let shown = match status {
            Status::Published => "visible",
            Status::Draft | Status::Archived => "hidden",
        };
        println!("{status:?}: {shown}"); // Draft: hidden, Published: visible, Archived: hidden
    }

    Ok(())
}
```

## How loud it is, measured

The lint tables above, run over this library's own 627 lesson programs with every level capped at `warn`: **59 raise nothing**. The five lints that fire most:

| Lint | Programs | What the fix teaches |
|---|---|---|
| `doc_markdown` | 347 | an identifier in a doc comment goes in backticks |
| `missing_const_for_fn` | 123 | what `const fn` is |
| `unwrap_used` | 121 | [`?`, `match` and `unwrap_or`](../../02_Errors/unwrap_is_a_todo/README.md) |
| `shadow_unrelated` | 113 | [one meaning per name per scope](../../18_Ownership/nothing_checks_a_shadow/README.md) |
| `indexing_slicing` | 107 | `.get(i)`, which returns an `Option` |

Lesson programs call `unwrap` and index on purpose, to show what those do, so read this as the order you will meet the lints in rather than as a verdict on the lessons. Five more lints were measured and left out; the last comment in `Cargo.toml` names four:

| Lint | Programs | Why it is not in the template |
|---|---|---|
| `min_ident_chars` | 512 | `i`, `x` and `n` are fine names |
| `arithmetic_side_effects` | 257 | rejects `n + 1`, which [changes how everything reads](../../05_Tooling/strict_lints/README.md) |
| `missing_assert_message` | 40 | a message on every `assert_eq!` in a practice test is noise |
| `shadow_reuse` | 32 | rejects `let line = line.trim();`, which is idiomatic |
| `let_underscore_must_use` | 19 | rejects `let _ =`, the fix rustc's own message suggests |

## Why each part is there

- **Four `allow`s, and nothing else below `warn`.** A name bound only to show a form is not a mistake — [the four a practice tree turns off](../../05_Tooling/scaffolding/README.md). `unused_must_use` is *not* among them: an ignored `Result` is a dropped error, and it keeps rustc's default `warn`.
- **`warn`, not `deny`.** Training shows every lesson and blocks none of them, and a clippy lint never stops `cargo run` whatever its level. `cargo strict` is the switch: `-W unused` puts the four back, `-D warnings` makes everything an error. Its flags are inline because a `--config some-file.toml` path is resolved from wherever cargo runs, so it would work at the project root and fail in `src/`.
- **`nursery` is here and not in production.** Lints still in development are useful suggestions and poor gates: at `deny`, a toolchain bump can fail code that did not change.
- **`[profile.dev.package."*"]`.** Your code stays unoptimised so a debugger can follow it; dependencies are built optimised once, and every `Regex::new` and random number after that runs optimised code.
- **`rust-version` and `rust-toolchain.toml`.** The toolchain file picks the compiler; `rust-version` tells clippy the oldest Rust the code must build on, so it does not suggest an API that version lacks.
- **Five dependencies.** `anyhow`, `rand`, `regex`, `serde` and `serde_json` are what a practice program reaches for first; delete what you do not use. Each is compiled the first time you build, not on every run.
- **RustRover** shows the `[lints.clippy]` half only with *Settings → Rust → External Linters* set to Clippy — [section 1 of the RustRover page](../../05_Tooling/rustrover_setup/README.md). The `[lints.rust]` half needs nothing.

## See also

- [Production](../production/README.md) — the strict template, for code that ships
- [Clippy](../clippy/README.md) — the nine groups, where each setting lives, and the commands
- [Automation](../automation/README.md) — `lint_profiles.py new`, `apply` and `check`
- [Strict clippy lints](../../05_Tooling/strict_lints/README.md) — the panic set, and what each part of it costs

---

*The outputs, clippy messages and pass/fail results above were captured on 2026-09-14 by running cargo 1.98.0 and clippy in the template and in scratch copies of it; they are verified once, since tools/run_examples.py compiles bare `rustc` examples and runs neither Cargo nor clippy. The file blocks are the exception: those are checked on every commit. The counts come from running clippy with this template's lint tables over all 627 `examples/*.rs` on master.*

## Po polsku

To jest szablon **treningowy**: łagodny, ale celowo drobiazgowy. Wszystkie linty uczące są na poziomie `warn`, więc `cargo run` nigdy się nie zatrzymuje, a `cargo clippy` — albo RustRover z Clippy ustawionym jako linter — przy każdej linijce, którą da się napisać lepiej, podaje tę lepszą postać. Wyłączone są tylko cztery linty o nieużywanych nazwach (`unused_variables`, `unused_imports`, `unused_mut`, `dead_code`), bo w pliku ćwiczeniowym nazwa związana tylko po to, żeby pokazać formę, nie jest błędem. Kiedy chcesz, żeby lekcja była egzekwowana, uruchom `cargo strict`: te same linty, a każde ostrzeżenie staje się błędem.

Najprościej skopiować cały katalog `template/` i zmienić `name` w `Cargo.toml` — albo uruchomić `lint_profiles.py new`, które zrobi to samo i dopisze pełne ścieżki w komunikatach. Każdy blok na tej stronie jest wklejany z pliku przez `tools/run_examples.py`, a CI pilnuje, żeby się nie rozjechały, więc to, co kopiujesz, jest dokładnie tym, co zapisuje skrypt. Pierwsza próba z tabeli powyżej działa, ale dostaje osiem uwag: indeksowanie zamiast `get`, `unwrap` zamiast `?`, rzutowanie `as` zamiast `From`, dzielenie całkowite, wycinek napisu w środku znaku UTF-8 i dopasowanie `_`, które połknie przyszły wariant enuma.

**Szukaj po polsku:** szablon projektu Rust · `Cargo.toml` linty · `clippy pedantic` · `clippy.toml` · `rust-toolchain.toml`
