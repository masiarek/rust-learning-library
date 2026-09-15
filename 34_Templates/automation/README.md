# Automation: a training and a production lint profile

**Level:** 201 · working knowledge

**One line:** **Training** lives in `Cargo.toml`, so every `cargo run`, `cargo clippy` and RustRover Run uses it — very strict, except that unused bindings are allowed. **Production** is `cargo prod`, the same project with the unused lints back on and every warning an error. [`lint_profiles.py`](lint_profiles.py) writes both into a project, checks them, and keeps the path in `.cargo/config.toml` true.

## Use it

```bash
python3 34_Templates/automation/lint_profiles.py new   ~/RustroverProjects/untitled2   # cargo new, then both profiles
python3 34_Templates/automation/lint_profiles.py apply ~/RustroverProjects/untitled1   # an existing project
python3 34_Templates/automation/lint_profiles.py check ~/RustroverProjects/*           # exit 1 on any drift
```

```text title="lint_profiles.py new — real output, the scratch directory shortened to <e2e>"
<e2e>/practice
  added   [lints.rust] with 14 keys  (Cargo.toml)
  added   [lints.clippy] with 25 keys  (Cargo.toml)
  wrote   test carve-outs  (clippy.toml)
  added   [alias] with 2 keys  (.cargo/config.toml)
  added   build.rustflags remap — diagnostics name the file in full  (.cargo/config.toml)
  added   /.cargo/config.toml to .gitignore — the remap names this machine's path
```

| File | Goes into | What it holds |
|---|---|---|
| [training.toml](training.toml) | `Cargo.toml` | `[lints.rust]` and `[lints.clippy]`: the training profile |
| [production.toml](production.toml) | `.cargo/config.toml` | `[alias] prod` and `prod-clippy`: the production profile |
| [clippy.toml](clippy.toml) | `clippy.toml` | the carve-outs that let a test `unwrap`, index and panic |
| [lint_profiles.py](lint_profiles.py) | — | `new`, `apply`, `check`; stdlib only, Python 3.11+ |

The script names no lint. It reads the three TOML files at run time, so editing a template changes what the next `apply` writes — and `check` lists every project that has not got the edit yet.

## Training: strict, except for unused bindings

Every line in [training.toml](training.toml) is one of three levels:

| Level | Lints | Why |
|---|---|---|
| `allow` | `unused_variables`, `unused_imports`, `unused_mut`, `dead_code` | a name bound only to show a form is not a mistake — [the four a practice tree turns off](../../05_Tooling/scaffolding/README.md) |
| `warn` | `missing_const_for_fn` | a suggestion whose fix changes nothing a program does |
| `deny` | everything else — clippy's `pedantic` and `nursery` groups, the panic set (`unwrap_used`, `expect_used`, `indexing_slicing`, `string_slice`, `panic`…), `as_conversions`, `integer_division`, [`shadow_unrelated`](../../18_Ownership/nothing_checks_a_shadow/README.md), `wildcard_enum_match_arm`, `allow_attributes_without_reason`, and rustc's `rust_2018_idioms`, `unsafe_code`, `unused_must_use`, `trivial_casts` | each fires on a line with a better way to write it, and the message names that way |

`unused_must_use` is `deny` rather than allowed with the other four: an ignored `Result` is a dropped error, not an unread name.

The two halves of the file reach you through different commands, which matters in an IDE:

| | `cargo run`, RustRover Run | `cargo clippy`, RustRover's linter set to Clippy |
|---|---|---|
| `[lints.rust]` | yes — a `deny` stops the build | yes |
| `[lints.clippy]` | **no** | yes — a `deny` fails clippy, never the program |

A practice file that runs, with one unused import and one unused binding:

```rust title="src/main.rs"
use std::collections::HashMap;

fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let v: Vec<i32> = vec![1, 2, '3' as i32];
    let first = v[0];
    let parsed: i32 = "42".parse().unwrap();
    let unused = 5;
    println!("{}", double(first) + parsed); // 44
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles() {
        let v = vec![2];
        assert_eq!(double(v[0]), "4".parse::<i32>().unwrap());
    }
}
```

`cargo run` prints `44` and not one warning. `cargo clippy --all-targets` refuses three lines and suggests a fourth:

```text title="Abridged — real cargo clippy output under training; source excerpts and help lines dropped"
warning: this could be a `const fn`
 --> <e2e>/practice/src/main.rs:3:1
  = note: requested on the command line with `-W clippy::missing-const-for-fn`

error: using a potentially dangerous silent `as` conversion
 --> <e2e>/practice/src/main.rs:8:34
  = note: requested on the command line with `-D clippy::as-conversions`

error: indexing may panic
 --> <e2e>/practice/src/main.rs:9:17
  = help: consider using `.get(n)` or `.get_mut(n)` instead
  = note: requested on the command line with `-D clippy::indexing-slicing`

error: used `unwrap()` on a `Result` value
  --> <e2e>/practice/src/main.rs:10:23
   = note: if this value is an `Err`, it will panic
   = note: requested on the command line with `-D clippy::unwrap-used`
```

The test repeats both `v[0]` and `.unwrap()`, and neither is reported — that is [clippy.toml](clippy.toml) at work.

### What it costs, measured

Run over this library's own 627 lesson programs, with every level capped at `warn` so nothing stops early, **60 raise nothing under training**. The five lints that fire most:

| Lint | Programs | What the fix teaches |
|---|---|---|
| `clippy::doc_markdown` | 347 | an identifier in a doc comment goes in backticks |
| `clippy::missing_const_for_fn` | 123 | what `const fn` is — at `warn`, for this reason |
| `clippy::unwrap_used` | 121 | [`?`, `match`, `unwrap_or`](../../02_Errors/unwrap_is_a_todo/README.md) |
| `clippy::shadow_unrelated` | 113 | one meaning per name per scope |
| `clippy::indexing_slicing` | 107 | `.get(i)`, which returns an `Option` |

Lesson programs call `unwrap` and index on purpose, to show what those do, so read this as the order you will meet the lints in rather than as a verdict on the lessons.

Five lints were measured and left out:

| Lint | Programs | Why it is not here |
|---|---|---|
| `clippy::min_ident_chars` | 512 | `i`, `x` and `n` are fine names |
| `clippy::arithmetic_side_effects` | 257 | rejects `n + 1`, which [changes how everything reads](../../05_Tooling/strict_lints/README.md) |
| `clippy::missing_assert_message` | 40 | a message on every `assert_eq!` in a practice test is noise |
| `clippy::shadow_reuse` | 32 | forbids `let line = line.trim();`, which is idiomatic Rust |
| `clippy::let_underscore_must_use` | 19 | forbids `let _ =`, the fix rustc's own message suggests |

## Production: `cargo prod`

[production.toml](production.toml) adds two aliases to `.cargo/config.toml`:

```toml
[alias]
prod = ["check", "--all-targets", "--config", "build.rustflags = ['-W', 'unused', '-D', 'warnings']"]
prod-clippy = ["clippy", "--all-targets", "--config", "build.rustflags = ['-W', 'unused', '-D', 'warnings']"]
```

- `-W unused` turns the four allowed lints back on. Cargo passes these flags **after** the `[lints]` table on rustc's command line, so they win.
- `-D warnings` makes every warning an error — including `missing_const_for_fn`, which training only warns about.

That is the answer to *what goes in production*: nothing new, and nothing left as a warning. An unused binding in code you ship is a real defect — a value computed and dropped, or an import left from a refactor — so the lints training allows are exactly the ones production needs back.

The same file, under `cargo prod`:

```text title="Abridged — real cargo prod output; source excerpts dropped"
error: unused import: `std::collections::HashMap`
 --> <e2e>/practice/src/main.rs:1:5
  = note: `-D unused-imports` implied by `-D warnings`
error: unused variable: `unused`
  --> <e2e>/practice/src/main.rs:11:9
   = note: `-D unused-variables` implied by `-D warnings`
```

`cargo prod-clippy` reports six errors on `main.rs`: those two, the three clippy refused under training, and the `const fn` suggestion, now an error. It is the one to put in CI — with the toolchain [pinned](../../05_Tooling/pinning_the_toolchain/README.md), because `nursery` at `deny` means a compiler upgrade can add an error to code that has not changed.

## Five traps, each measured

**A Cargo `[profile]` cannot carry lints.** The obvious shape — `rustflags` under `[profile.dev]` — is refused on stable 1.98:

```text title="Abridged — real cargo output"
error: failed to parse manifest at `…/Cargo.toml`
  feature `profile-rustflags` is required
```

So the two profiles here are a manifest table and an alias, not Cargo profiles, and `--profile dev` / `--release` stay free for what they are for.

**An alias that points at a file works in one directory.** `train = "check --config .cargo/training.toml"` resolves the path from wherever cargo runs, so it worked at the project root and failed in `src/`:

```text title="Real cargo output, run from src/"
error: failed to parse value from --config argument `.cargo/training.toml` as a dotted key expression
```

Writing the flags inline, as above, works from any directory.

**`-D warnings` alone does not bring an allowed lint back.** An allowed lint never becomes a warning, so there is nothing for `-D warnings` to promote — with `unused_variables = "allow"` in the manifest, `-D warnings` on its own left the unused binding silent. Hence `-W unused` first.

**A `RUSTFLAGS` environment variable replaces `build.rustflags`** rather than adding to it. Set, it removed the remap *and* both production flags from rustc's command line, silently. `check` fails when it is set.

**`#[allow(unused_variables)]` covers one item.** Above `fn main` it silenced `main` and left the next function warning; `#![allow(unused_variables)]` — with the `!` — at the top of `main.rs` covers the file. The manifest table covers the package, which is why it is the one to use.

## What `apply` changes, and what it will not

It **adds**: a missing table goes in whole, with its comments; a missing key goes at the end of its table. A key already present with a **different value** is somebody's decision, so it is kept and reported until you pass `--force`. Keys the template does not name are never touched. Every result is re-parsed with `tomllib` and compared with the template, and with the original minus the template's keys, before anything is written.

On a project carrying an older four-line block and `unsafe_code = "allow"`:

```text title="lint_profiles.py apply, then apply --force — real output"
<e2e>/old
  kept    lints.rust.unsafe_code = "allow"  (template: "deny"; --force replaces it)
  added   9 key(s) to [lints.rust]  (Cargo.toml)
  added   [lints.clippy] with 25 keys  (Cargo.toml)
  wrote   test carve-outs  (clippy.toml)
  updated remap — it named /Users/amasa/RustroverProjects/untitled1/, and the project is now here  (.cargo/config.toml)
  added   /.cargo/config.toml to .gitignore — the remap names this machine's path
<e2e>/old
  forced  lints.rust.unsafe_code = "deny"  (was "allow")  (Cargo.toml)
  ok      clippy.toml
  ok      .cargo/config.toml
```

**The remap line** — `--remap-path-prefix` with an empty old prefix, [explained on the RustRover page](../../05_Tooling/rustrover_setup/README.md) — spells out the project's directory, because rustflags interpolate nothing. Move the folder and every diagnostic names the old location, so `check` compares the two:

```text title="lint_profiles.py check, after mv practice moved — real output"
<e2e>/moved
  ok      training: 39/39 keys as training.toml says
  ok      clippy.toml: 5/5 keys as clippy.toml says
  ok      production: 2/2 keys as production.toml says
  FAIL    remap names <e2e>/practice — the project has moved; `apply` rewrites it
```

`--no-remap` leaves it out, and the `.gitignore` line with it.

**A workspace** gets `[workspace.lints.rust]` and `[workspace.lints.clippy]` at the root — the table name depends on the manifest's shape, and a wrong one is silently never read. A member whose manifest says `[lints] workspace = true` is skipped, since it inherits; `cargo new` inside the workspace writes that line by itself. **A project already given [`rust_scaffold.py adopt`](../../05_Tooling/scaffolding/rust_scaffold.py)** merges cleanly: `adopt`'s four `allow` lines and remap stay, and `apply` adds the rest.

## In RustRover

- **Training** needs nothing for the `[lints.rust]` half. For the clippy half, set *Settings → Rust → External Linters* to **Clippy**, [section 1 of the RustRover page](../../05_Tooling/rustrover_setup/README.md); without it, the IDE never shows a clippy lint.
- **Production** is `cargo prod` in the IDE's terminal. A *Cargo* run configuration with the command `prod` should work too, since Cargo resolves the alias, but that was not tried in the IDE.
- Do not put `-D warnings` in the linter's *Additional arguments*: every warning then turns red while you type, which is the production profile applied to a half-written line.

## If you are coming from another language

- **Python** — training is close to `ruff` with most rule families selected plus a short ignore list in `pyproject.toml`, and production to the same configuration run in CI where any finding fails the job. What Rust adds is the three levels inside one table — `allow`, `warn`, `deny` per lint — and a second command, `-D warnings`, that promotes one run's warnings to errors without touching the file.
- **ABAP** — the Code Inspector check variant you run while developing is training; the ATC variant that blocks transport release is production. Same split, same politics: the question was never which checks exist but which ones stop a release. What changes is where it lives — here both variants are files in the project, readable in a diff, instead of settings in a system someone else administers.

## See also

- [Strict clippy lints](../../05_Tooling/strict_lints/README.md) — the panic set this training profile denies, and the cost of each part
- [Scaffolding a practice tree](../../05_Tooling/scaffolding/README.md) — the workspace answer, where configuration is shared instead of copied
- [RustRover setup](../../05_Tooling/rustrover_setup/README.md) — Clippy as the linter, and why a diagnostic says `src/main.rs`
- [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) — reading the messages both profiles produce
- [Nothing checks a shadow](../../18_Ownership/nothing_checks_a_shadow/README.md) — what `shadow_unrelated` is for

---

*Every transcript above is real output from `lint_profiles.py` and cargo 1.98.0 on scratch projects, captured 2026-09-14, with the scratch directory shortened to `<e2e>` and source excerpts dropped where a title says so. They are verified once rather than on every commit: [`tools/run_examples.py`](../../tools/run_examples.py) compiles single `.rs` files and runs neither Cargo nor clippy. The counts come from running clippy with [training.toml](training.toml) over all 627 `examples/*.rs` on master at `8f7d18d`.*

## Po polsku

Dwa profile, dwa różne mechanizmy. **Trening** to tabela `[lints]` w `Cargo.toml`, więc czyta go każde `cargo run`, `cargo clippy` i przycisk Run w RustRoverze — jest bardzo surowy, z jednym wyjątkiem: cztery linty o nieużywanych nazwach (`unused_variables`, `unused_imports`, `unused_mut`, `dead_code`) są wyłączone, bo w pliku ćwiczeniowym nazwa związana tylko po to, żeby pokazać formę, nie jest błędem. `unused_must_use` zostaje na `deny` — zignorowany `Result` to zgubiony błąd, a nie szum. **Produkcja** to alias `cargo prod` w `.cargo/config.toml`: ten sam projekt, w którym `-W unused` przywraca tamte cztery linty, a `-D warnings` zamienia każde ostrzeżenie w błąd. Nieużywana zmienna w kodzie, który trafia do ludzi, to prawdziwa wada — dlatego to, co trening wyłącza, produkcja musi włączyć z powrotem.

Pułapki, w które wpadłem po drodze. Sekcja `[profile.dev]` nie przyjmie `rustflags` na stabilnym Ruście (`feature profile-rustflags is required`), więc „profil” nie jest tu profilem Cargo. Alias wskazujący plik (`--config plik.toml`) działa tylko w katalogu głównym projektu, bo ścieżka liczy się od bieżącego katalogu — flagi trzeba wpisać wprost. Samo `-D warnings` nie przywróci lintu ustawionego na `allow`, bo taki lint w ogóle nie staje się ostrzeżeniem. Zmienna środowiskowa `RUSTFLAGS` zastępuje `build.rustflags` w całości. A `#[allow(...)]` bez wykrzyknika dotyczy tylko następnego elementu; na cały plik działa `#![allow(...)]`.

W RustRoverze połowa `[lints.clippy]` pojawia się tylko wtedy, gdy w *Settings → Rust → External Linters* wybrano Clippy — samo `cargo run` lintów clippy nie uruchamia, a clippy na `deny` nigdy nie blokuje uruchomienia programu. Skrypt `lint_profiles.py` czyta trzy pliki TOML w chwili uruchomienia: popraw szablon, a następne `apply` wpisze poprawkę, `check` zaś pokaże każdy projekt, który jej jeszcze nie ma — i ten, który przeniesiono do innego katalogu.

**Szukaj po polsku:** linty w Ruście · `Cargo.toml` `[lints]` · `cargo alias` · `clippy deny warnings` · `RUSTFLAGS` a `build.rustflags`
