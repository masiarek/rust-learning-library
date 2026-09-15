# Clippy: the groups, the settings, the commands

**Level:** reference · the map

**One line:** Clippy 1.98 has 822 lints in nine groups — five on by default, four you opt into — and every setting lives in one of two files: **levels** in `Cargo.toml`, **thresholds and carve-outs** in `clippy.toml`. The [training](../training/README.md) and [production](../production/README.md) templates are two ways of filling that in.

## The commands

```bash
cargo clippy                  # this package's own code
cargo clippy --all-targets    # ...and its tests, examples and benches
cargo clippy --fix            # apply the suggestions clippy can apply by itself
cargo clippy -- -D warnings   # any warning fails the run: the CI form
cargo strict                  # training template: its lints, with every warning an error
cargo ci                      # production template: cargo clippy --all-targets -- -D warnings
```

`--all-targets` is the one to remember. Without it clippy never reads your tests, so the carve-outs in `clippy.toml` have nothing to carve out.

## The nine groups

Sizes and default levels counted from `clippy-driver -W help` on 1.98.0; the descriptions are clippy's own, from its README.

| Group | Lints | Default | Clippy's description | Training | Production |
|---|---|---|---|---|---|
| `correctness` | 67 | `deny` | code that is outright wrong or useless | `deny` | `deny` |
| `suspicious` | 83 | `warn` | code that is most likely wrong or useless | `warn` | `deny` |
| `style` | 158 | `warn` | code that should be written in a more idiomatic way | `warn` | `deny` |
| `complexity` | 143 | `warn` | code that does something simple but in a complex way | `warn` | `deny` |
| `perf` | 38 | `warn` | code that can be written to run faster | `warn` | `deny` |
| `pedantic` | 143 | `allow` | lints which are rather strict or have occasional false positives | `warn` | `deny` |
| `nursery` | 53 | `allow` | new lints that are still under development | `warn` | off |
| `restriction` | 132 | `allow` | lints which prevent the use of language and library features | 31 picked, `warn` | 22 picked, `deny` |
| `cargo` | 5 | `allow` | lints for the cargo manifest | off | off |

`all` is the first five groups, 489 lints, and it is what production denies in a single line. `restriction` is the one never to switch on whole: its lints forbid ordinary, legal code, and turning on the entire group is itself reported by one of the default lints, `blanket_clippy_restriction_lints`. Take from it one lint at a time, as both templates do.

## Where each setting lives

| Where | What it sets | Example |
|---|---|---|
| `Cargo.toml`, `[lints.clippy]` | a **level** — `allow`, `warn`, `deny` or `forbid` — for a group or one lint | `pedantic = { level = "warn", priority = -1 }` |
| `clippy.toml`, beside `Cargo.toml` | a **threshold** or a **carve-out** | `allow-unwrap-in-tests = true` |
| the command line, after `--` | a level for this run only | `cargo clippy -- -D warnings` |
| an attribute on an item | a level for that item only | `#[allow(clippy::indexing_slicing, reason = "the index is checked above")]` |

`priority = -1` on a group is load-bearing: a group and one of its own members conflict by construction, and the lower priority applies the group first, so a single-lint line below it can override one member. The minimum Rust version clippy respects comes from `rust-version` in `Cargo.toml`, which is why neither template repeats it in `clippy.toml`.

## Which setting fired

Every clippy message ends with a note saying where its level came from:

```text title="Abridged — real clippy notes, source excerpts dropped"
warning: used `unwrap()` on a `Result` value
   = note: requested on the command line with `-W clippy::unwrap-used`

warning: casts from `u8` to `i64` can be expressed infallibly using `From`
   = note: `-W clippy::cast-lossless` implied by `-W clippy::pedantic`

warning: useless use of `vec!`
   = note: `#[warn(clippy::useless_vec)]` on by default
```

| The note says | The level comes from | Change it in |
|---|---|---|
| `requested on the command line with …` | a line naming that lint — Cargo hands `[lints]` to clippy as command-line flags, hence the wording | `Cargo.toml` |
| `… implied by -W clippy::pedantic` | a group line | `Cargo.toml`: add a line for the one lint, below the group |
| `#[warn(…)]` on by default | no setting at all: the lint's default level | `Cargo.toml`, if you want it different |

The first two come from the [training page's first attempt](../training/README.md#what-it-does-to-a-first-attempt); the third from a scratch project with a `vec![2]` in a test.

## The two clippy.toml templates

Pasted from the template folders by tools/run_examples.py, so they cannot drift from the files.

<!-- file:../training/template/clippy.toml -->
```toml title="../training/template/clippy.toml"
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

<!-- file:../production/template/clippy.toml -->
```toml title="../production/template/clippy.toml"
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

## In the editor

- **RustRover** shows the `[lints.clippy]` levels only once *Settings → Rust → External Linters* is set to Clippy — [section 1 of the RustRover page](../../05_Tooling/rustrover_setup/README.md). Put `--all-targets` in that dialog's *Additional arguments*, and leave `-D warnings` out of it.
- **In a terminal**, [bacon](../../05_Tooling/bacon/README.md) runs clippy again on every save.

## See also

- [Training](../training/README.md) and [Production](../production/README.md) — every setting above, filled in two ways, in projects to copy
- [Strict clippy lints](../../05_Tooling/strict_lints/README.md) — the panic set, and what each part of it costs
- [RustRover setup](../../05_Tooling/rustrover_setup/README.md) — Clippy as the IDE's linter
- [Automation](../automation/README.md) — `lint_profiles.py check`, which compares a project's settings with a template

## Po polsku

Clippy w wersji 1.98 ma 822 linty w dziewięciu grupach. Pięć jest włączonych domyślnie — `correctness` na `deny`, a `suspicious`, `style`, `complexity` i `perf` na `warn` — i razem tworzą grupę `all` (489 lintów). Cztery pozostałe trzeba włączyć samemu: `pedantic` (surowe, czasem fałszywie alarmujące), `nursery` (linty w budowie), `restriction` (zakazy rzeczy zupełnie legalnych) i `cargo` (metadane manifestu). Grupy `restriction` nigdy nie włącza się w całości — sam clippy to zgłasza lintem `blanket_clippy_restriction_lints` — tylko wybiera się z niej pojedyncze linty.

Ustawienia mieszkają w dwóch plikach. **Poziomy** (`allow`, `warn`, `deny`, `forbid`) dla grup i pojedynczych lintów są w `Cargo.toml` w tabeli `[lints.clippy]`; **progi i wyjątki** (np. `allow-unwrap-in-tests = true`) są w `clippy.toml` obok. Każdy komunikat clippy kończy się notatką, skąd wziął się poziom: `requested on the command line` oznacza linijkę w `Cargo.toml` (Cargo przekazuje tabelę `[lints]` jako flagi wiersza poleceń), `implied by … pedantic` — linijkę grupy, a `on by default` — brak jakiegokolwiek ustawienia.

**Szukaj po polsku:** grupy lintów clippy · `clippy.toml` konfiguracja · `cargo clippy --all-targets` · `clippy pedantic nursery restriction`
