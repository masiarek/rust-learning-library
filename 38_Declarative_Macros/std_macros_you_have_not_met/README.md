# The std macros you have not met

**Level:** 201 · working knowledge

**One line:** Past `println!`, `vec!` and `assert!`, std's macro list holds `matches!`, `concat!`, `stringify!`, `include_str!`, `env!`, `compile_error!`, `thread_local!` and more — and most of them do their work at compile time, which is the reason they are macros at all.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- A test and three placeholders: `matches!(value, Some(n) if n > 2)` as a `bool`; and `todo!`, `unimplemented!` and `unreachable!`, whose panic messages on 1.98.0 are *not yet implemented*, *not implemented* and *internal error: entered unreachable code* — which one says "later", which "never", and which "this cannot happen"?
- Text made at compile time: `concat!("a", 1, 'b', true)` is the `&'static str` `"a1btrue"`, and `stringify!(1 + 2 * x)` is `"1 + 2 * x"` even when no `x` exists
- Files and variables read at build time: `include_str!` and `include_bytes!` take a path relative to the current source file and put the contents in the binary; `env!("NAME")` stops the build with *environment variable `NAME` not defined at compile time* and a help line pointing to `std::env::var`, while `option_env!` gives `None` — which is how [rustc_without_cargo](../../15_First_Programs/rustc_without_cargo/README.md) tells a Cargo build from bare `rustc`
- Where am I: `file!()`, `line!()`, `column!()` — the three that `dbg!` prints in its `[file:line:column]` prefix — and `module_path!()`
- Decisions at compile time: `cfg!(test)` is a `bool` in code that is always compiled, unlike `#[cfg]` ([What an attribute is](../../27_Modules/what_an_attribute_is/README.md)); `compile_error!` fails the build with your message from a `#[cfg]`-gated item or a catch-all macro rule; `cfg_select!`, stable since 1.95.0, picks the first true branch
- Checks for debug builds: `debug_assert!` and its `_eq`/`_ne` forms expand to the matching `assert` macro inside `if cfg!(debug_assertions) { … }`, so the condition is still type-checked but not run in a default `--release` build — and what that means for a side effect written inside one
- Writing into something: `write!` and `writeln!` against a `String` (`fmt::Write`) or a file (`io::Write`), and `format_args!` as the allocation-free value `format!` and `println!` are built on — `let args = format_args!("{} {}", "x", 5);` followed by `println!("{args}")` compiles on 1.98.0; since which release?
- Per-thread statics: `thread_local!` with a `const { … }` initialiser and `.with(|c| …)` access; and two newer names — `assert_matches!` is stable since 1.96.0 but not in the prelude, so a bare call is *cannot find macro* until `use std::assert_matches;`, while `hash_map!` and `concat_bytes!` are still nightly-only

## The trap it exists for

`env!` where the value was meant to be read when the program runs. `env!("API_URL")` copies the build machine's value into the binary, so setting the variable before starting the program changes nothing — and a CI build without it set fails to compile at all. Run-time configuration is `std::env::var`.

## Where this sits

[Macros](../../25_Control_Flow/macros/README.md) names the day-one macros — `println!`, `format!`, `vec!`, `assert!`, `assert_eq!`, `panic!`, `dbg!`, `todo!` — and [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md) owns `dbg!`. This page is the rest of std's list, one bullet per job.

## See also

- [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md) — the macro built from `file!`, `line!` and `column!`
- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — `option_env!("CARGO_PKG_NAME")` telling Cargo from bare `rustc`
- [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) — `#[cfg]` deletes, `cfg!()` chooses
- [`unwrap` is a TODO you forgot to remove](../../02_Errors/unwrap_is_a_todo/README.md) — `todo!` as the honest placeholder
- [The format mini-language](../../14_Strings/the_format_language/README.md) — what `write!` and `format_args!` accept
- [A function, `macro_rules!`, or a procedural macro](../../37_Procedural_Macros/function_macro_rules_or_proc_macro/README.md) — check this list before writing your own
- [std's macro index ↗](https://doc.rust-lang.org/std/index.html#macros) — every macro, with its stability badge

## If you are coming from another language

- **C.** `__FILE__` and `__LINE__` are `file!()` and `line!()`; `#error` is `compile_error!`; an `assert` compiled out under `NDEBUG` is `debug_assert!` in a release build — except that the Rust condition must still compile. C checks a `printf` format string only through compiler warnings, and [A format string is a program ↗](https://masiarek.github.io/c-learning-library/03_Strings/a_format_string_is_a_program/) shows what an unchecked one can do; `format_args!` refuses a bad one outright.
- **Python.** `__file__` and a frame's line number are looked up at run time; `file!()` and `line!()` are literals fixed at compile time. `assert` statements vanish under `python -O`, the same trade as `debug_assert!` — though Python removes the expression entirely.
- **Go.** `//go:embed` fills a `string` or `[]byte` variable from a file at build time — `include_str!` and `include_bytes!`.

## Po polsku

Biblioteka standardowa ma znacznie więcej makr niż `println!` i `vec!`, a większość z nich pracuje **w czasie kompilacji**: `concat!` i `stringify!` sklejają napisy, `include_str!` wczytuje plik do programu, `env!` odczytuje zmienną środowiskową maszyny, na której trwa budowanie, a `compile_error!` przerywa kompilację własnym komunikatem. Najczęstsza pomyłka to `env!` tam, gdzie chodziło o konfigurację w czasie działania — wartość zostaje „zapieczona” w pliku wykonywalnym, a do odczytu przy uruchomieniu służy `std::env::var`.

**Szukaj po polsku:** makra biblioteki standardowej · makra czasu kompilacji · `rust env vs option_env` · `rust include_str` · `rust todo vs unimplemented`
