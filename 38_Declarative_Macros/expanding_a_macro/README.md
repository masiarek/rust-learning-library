# Expanding a macro

**Level:** 301 · deep dive

**One line:** `cargo expand`, `rustc -Zunpretty=expanded` and `trace_macros!` show the code a macro wrote — all three rest on unstable compiler features — and the printout is a tree printed back as text, so it can read differently from the program that was compiled.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `RUSTC_BOOTSTRAP=1 rustc --edition 2024 -Zunpretty=expanded main.rs`: the command [Printing the HIR](../../20_Compilers/printing_the_hir/README.md) takes apart word by word, one stage earlier — macros are gone, `for` and `?` are still there, and `println!` stops at `format_args!`, which is built into the compiler
- The parentheses you did not write: `square!(1 + 2)` with the rule `$e * $e` prints as `(1 + 2) * (1 + 2)`, because a matched `$e:expr` is one parsed node
- The printout that lies about names: a macro `{ let tmp = 2; $e * tmp }` called as `times_two!(tmp)` with the caller's `tmp = 3` prints 6 on 1.98.0, and `-Zunpretty=expanded` shows `{ let tmp = 2; tmp * tmp }` — which, pasted back, prints 4
- `-Zunpretty=expanded,hygiene`: every identifier annotated with a syntax context, so the two `tmp`s print with different numbers after the `#` — the way to see [Hygiene](../hygiene/README.md) instead of guessing it
- [`cargo expand` ↗](https://github.com/dtolnay/cargo-expand): its README describes it as a wrapper around `cargo rustc --profile=check -- -Zunpretty=expanded` that runs rustfmt over the result — how does it get a `-Z` flag through a stable toolchain, and does it show `#[derive]` output too?
- `trace_macros!(true)`: on stable 1.98.0 the call is `E0658` (issue #29598), yet the `note: trace_macro` lines showing each expansion step are still printed; under `RUSTC_BOOTSTRAP=1` with `#![feature(trace_macros)]` it compiles. When is a step-by-step trace the only readable view — a recursive `tt` muncher?
- `-Zmacro-backtrace`, the flag every `this error originates in the macro` note names: the `in this expansion of …` frames it adds, and when they help
- Why none of these outputs compiles on stable as-is: the expanded file opens with `#![feature(prelude_import)]`

## The trap it exists for

Treating the expanded printout as the program. It is the macro's output with hygiene erased, so two variables that rustc kept apart print under one name, and the code you paste back to "test the expansion" computes something else. Use `expanded,hygiene` whenever the question is about which name refers to what.

## Where this sits

[Printing the HIR](../../20_Compilers/printing_the_hir/README.md) owns the command itself: `RUSTC_BOOTSTRAP=1`, the `--edition` trap, the table of `-Zunpretty` values and the promise-free output. This page is only the reading of a macro's expansion while you are writing the macro.

## See also

- [Printing the HIR](../../20_Compilers/printing_the_hir/README.md) — the same flag family, one stage later, and every trap of the command line
- [`rustup default nightly`](../../05_Tooling/nightly/README.md) — the other way to get `-Z` flags, and why it is the wrong default
- [Hygiene](../hygiene/README.md) — what the `#` numbers in `expanded,hygiene` are telling you
- [Repetition](../repetition/README.md) — the depth errors that expansion helps you read
- [Macros](../../25_Control_Flow/macros/README.md) — where `cargo expand` is first mentioned
- [`trace_macros` in the unstable book ↗](https://doc.rust-lang.org/nightly/unstable-book/library-features/trace-macros.html) — the feature's entry

## If you are coming from another language

- **C and C++.** `cc -E` prints the source after the preprocessor, and [Printing the HIR](../../20_Compilers/printing_the_hir/README.md) shows `TWICE(21)` becoming `((21) * 2)`. The difference is what you get back: `-E` output is plain text that compiles as it stands, because the preprocessor had no idea of scope to erase; Rust's printout is a syntax tree turned back into text, missing exactly the hygiene information that made it correct.
- **Go.** Generated code from `go generate` is an ordinary file in the repository, read and reviewed like any other source. A Rust macro's expansion is never written anywhere unless you ask the compiler for it.

## Po polsku

Rozwinięcie makra (*macro expansion*) da się zobaczyć na trzy sposoby — `cargo expand`, `rustc -Zunpretty=expanded` i `trace_macros!` — i wszystkie trzy opierają się na niestabilnych funkcjach kompilatora. Pułapka: wydruk to drzewo składni zamienione z powrotem w tekst, **bez informacji o higienie**, więc dwie różne zmienne `tmp` wyglądają na wydruku jak jedna, a kod skopiowany z wydruku liczy co innego niż oryginał. Kiedy pytanie brzmi „która nazwa do czego się odnosi”, użyj `-Zunpretty=expanded,hygiene`.

**Szukaj po polsku:** rozwinięcie makra · podgląd kodu po makrach · `cargo expand` · `rustc unpretty expanded hygiene` · `rust trace_macros`
