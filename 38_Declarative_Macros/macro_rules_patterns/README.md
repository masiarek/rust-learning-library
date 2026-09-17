# Matchers and fragment specifiers

**Level:** 301 · deep dive

**One line:** A rule's left side is a pattern over tokens — `$name:expr`, `$name:ident`, `$name:tt` and twelve other fragment specifiers — and rules are tried top to bottom, with the first one that matches doing the expansion.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The fifteen fragment specifiers the Reference lists for 1.98.0 — `block`, `expr`, `expr_2021`, `ident`, `item`, `lifetime`, `literal`, `meta`, `pat`, `pat_param`, `path`, `stmt`, `tt`, `ty`, `vis` — each with one call it accepts and one it refuses
- `tt` as the escape hatch: one token or one bracketed group. Once a fragment is forwarded to a second macro it is opaque, and only `ident`, `lifetime` and `tt` can still be matched there by literal tokens
- Order decides: with `($i:ident)` above `($e:expr)`, `kind!(x)` takes the `ident` rule; swap the two rules and `expr` claims `x`. Nothing warns about the rule that can no longer fire — `unused_macro_rules` is allow-by-default
- A rule that starts parsing a fragment commits to it: against `($e:expr)` then `($a:tt +)`, the call `m!(1 +)` fails with `expected expression, found end of macro arguments` and the second rule is never tried
- Follow-set restrictions: `expr` and `stmt` may be followed only by `=>`, `,` or `;`, `ty` and `path` by a longer list — what does rustc say about a matcher like `$a:expr $b:expr`, and why is the rule there at all?
- Edition 2024 widened `expr` to match `_` and `const { … }`: the same macro prints `const block` / `underscore` under `--edition 2021` and `expr` / `expr` under 2024. `expr_2021` keeps the old meaning, and the allow-by-default `edition_2024_expr_fragment_specifier` lint (in the `rust-2024-compatibility` group) flags the matchers that change
- The `tt` muncher: a rule that takes `$head:tt $($tail:tt)*` and recurses on the tail. Counting 150 tokens that way hits `recursion limit reached while expanding` with a hint to add `#![recursion_limit = "256"]` — what does the limit cost to raise?

## The trap it exists for

Putting the general rule first. `($e:expr)` above `($i:ident)` swallows every identifier, the `ident` rule is dead, and the default lints say nothing. A `match` over values would have warned about an unreachable arm; a list of macro rules does not.

## Where this sits

[Pattern matching](../../30_Pattern_Matching/README.md) is patterns over values, where exhaustiveness is checked and `unreachable_patterns` warns by default. This page is patterns over tokens, where neither check runs. [Repetition](../repetition/README.md) owns `$( … )*`; this page covers a single fragment and the choice between rules.

## See also

- [`match` expressions](../../25_Control_Flow/match_expressions/README.md) — first match wins there too, with checks this page does not get
- [Pattern matching](../../30_Pattern_Matching/README.md) — what a pattern is when the input is a value
- [Repetition](../repetition/README.md) — matching a list of fragments
- [Expanding a macro](../expanding_a_macro/README.md) — `trace_macros!` for watching which rule fired
- [Macros by example: metavariables ↗](https://doc.rust-lang.org/reference/macros-by-example.html#r-macro.decl.meta.specifier) — the Reference's specifier list and the follow-set rules below it
- [Edition guide: macro fragment specifiers ↗](https://doc.rust-lang.org/edition-guide/rust-2024/macro-fragment-specifiers.html) — the 2024 change to `expr`
- [The Little Book of Rust Macros: incremental TT munchers ↗](https://lukaswirth.dev/tlborm/decl-macros/patterns/tt-muncher.html) — the recursion pattern in full

## If you are coming from another language

- **C.** A function-like `#define` has exactly one pattern: a fixed list of parameter names, or `...`, each argument a comma-separated run of tokens. Rust adds typed fragments and several rules per macro, so a malformed argument is refused at the call — `no rules expected` the offending token — instead of expanding into broken code somewhere downstream.
- **Python.** A `match` statement's `case` clauses are tried top to bottom and the first match wins, as macro rules are. The difference is timing and input: Python matches values at run time, a macro matches source tokens at compile time.

## Po polsku

Lewa strona reguły `macro_rules!` to wzorzec dopasowywany do **tokenów**, a nie wartości: `$x:expr` oznacza „tu ma stać wyrażenie”, `$x:ident` — identyfikator, `$x:tt` — dowolne drzewo tokenów (*token tree*). Reguły sprawdzane są od góry, wygrywa pierwsza pasująca, a kompilator — inaczej niż przy `match` — nie ostrzeże, że ogólna reguła postawiona wyżej zasłania szczegółową. Edycja 2024 poszerzyła `expr` o `_` i `const { … }`; stare zachowanie daje `expr_2021`.

**Szukaj po polsku:** wzorce w makrach · specyfikatory fragmentów · `rust macro fragment specifier list` · `rust tt muncher` · `expr_2021 edition 2024`
