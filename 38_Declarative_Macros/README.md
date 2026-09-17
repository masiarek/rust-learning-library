# Declarative macros

**One line:** `macro_rules!` is pattern matching on tokens instead of values — a list of rules, tried top to bottom, each rewriting the tokens it matched into new code before type checking runs — and this section is about writing one, not calling one.

[Macros](../25_Control_Flow/macros/README.md) is the page on what the `!` means at a call site and stops where writing one begins; this section starts there and covers `macro_rules!` only — derive, attribute and function-like procedural macros belong to [Procedural macros](../37_Procedural_Macros/README.md). Patterns over *values* are [Pattern matching](../30_Pattern_Matching/README.md), and what rustc does with the expanded code is [Printing the HIR](../20_Compilers/printing_the_hir/README.md). The outline follows *Rust for Rustaceans* ch. 7 "Macros" → "Declarative Macros" and the Reference's [Macros by example ↗](https://doc.rust-lang.org/reference/macros-by-example.html). **Every page here is a stub** — [CONTRIBUTING.md](../CONTRIBUTING.md) says what graduating one takes.

| Lesson | Level | What it covers |
|---|---|---|
| [Expanding a macro](expanding_a_macro/README.md) | 301 | `cargo expand`, `-Zunpretty=expanded` and `trace_macros!`: seeing the code a macro wrote, and the printout that reads `tmp * tmp` for a program that multiplies two different variables. Stub |
| [Matchers and fragment specifiers](macro_rules_patterns/README.md) | 301 | `$x:expr`, `$x:ident`, `$x:tt` and twelve more: what each fragment accepts, why the first matching rule wins, what edition 2024 changed about `expr`, and the `tt` muncher. Stub |
| [Repetition](repetition/README.md) | 301 | `$( … ),*`, `+` and `?`: matching a list, writing one out, and the "still repeating at this depth" error when the two sides disagree. Stub |
| [Hygiene](hygiene/README.md) | 301 | A `let` inside a macro cannot see or clobber the caller's variables, but a function name resolves at the call site — mixed-site hygiene, and `$crate` for paths that must work from another crate. Stub |
| [Exporting a macro](exporting_a_macro/README.md) | 301 | A macro exists only below its definition until `pub(crate) use` gives it a path inside the crate or `#[macro_export]` publishes it at the crate root. Stub |
| [The std macros you have not met](std_macros_you_have_not_met/README.md) | 201 | `matches!`, `concat!`, `stringify!`, `include_str!`, `env!`, `compile_error!`, `thread_local!` and the rest of std's list past `println!` and `vec!`. Stub |

## Where it goes next

Procedural macros are the other half of the topic: a `#[derive]` or an attribute is a program that receives tokens and returns tokens, which is how the things `macro_rules!` cannot do — glue two names into a new identifier, run as `#[derive(…)]` on somebody's struct — get done. That is [Procedural macros](../37_Procedural_Macros/README.md); start at [a function, `macro_rules!`, or a procedural macro](../37_Procedural_Macros/function_macro_rules_or_proc_macro/README.md) for the decision between the three, and [when `macro_rules!` runs out](../37_Procedural_Macros/when_macro_rules_runs_out/README.md) for the exact places this section stops. A macro other people call is an interface like any other, so [API design](../39_API_Design/README.md) applies, and the [Rust API Guidelines ↗](https://rust-lang.github.io/api-guidelines/macros.html) have five rules just for macros.

## Po polsku

Po polsku *macro* to po prostu **makro**, a `macro_rules!` bywa nazywane **makrem deklaratywnym** albo „makrem przez przykład” (*macros by example*, nazwa z Reference). Najważniejsza zmiana nastawienia: makro nie operuje na wartościach, tylko na **tokenach** — reguła to wzorzec dopasowywany do fragmentu kodu źródłowego, a jej prawa strona to kod, który powstanie w miejscu wywołania, zanim kompilator zacznie sprawdzać typy. Dlatego obowiązują tu inne pułapki niż przy `match`: nie ma sprawdzania wyczerpania przypadków, kolejność reguł decyduje po cichu, a nazwy zmiennych w środku makra podlegają **higienie** (*hygiene*).

Sekcja zaczyna się tam, gdzie kończy się strona o znaczeniu wykrzyknika, i kończy się na `macro_rules!` — makra proceduralne (`#[derive]`, atrybuty) to osobny rozdział. Szukając materiałów, wpisuj angielskie nazwy: polskich opracowań o `macro_rules!` jest niewiele, a komunikaty kompilatora i tak mówią o *fragment specifier* i *repetition*.

**Szukaj po polsku:** makra deklaratywne w Ruście · higiena makr · `macro_rules tutorial` · `little book of rust macros` · `rust macro fragment specifiers`
