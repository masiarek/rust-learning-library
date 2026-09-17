# Procedural macros

**Level:** 201 → 301 · working knowledge to deep dive

**One line:** A procedural macro is Rust code the compiler runs on your code — tokens in, tokens out — and this chapter goes from reading the ones you use, through `syn`, `quote` and `darling`, to writing derive, function-like and attribute macros whose errors point at the right token.

The six parts follow a one-day course outline, in its order. Pages marked *outline* are stubs: their questions are set, and nothing on them has been checked yet.

## 1. Macros you already use

| Lesson | What it answers |
|---|---|
| [Three kinds of procedural macro](three_kinds_of_procedural_macro/README.md) | What a derive, a function-like macro and an attribute each receive and return — and why `#[derive(Debug)]` is not one of them |
| [Expanding `thiserror`](expanding_thiserror/README.md) | Every line `#[derive(Error)]` writes, read from the expansion |
| [A function, `macro_rules!`, or a procedural macro](function_macro_rules_or_proc_macro/README.md) | When a procedural macro is the right call, and what it costs when it is not |

## 2. The toolkit

| Lesson | What it answers |
|---|---|
| [A proc-macro crate](a_proc_macro_crate/README.md) | Why the macros need a crate of their own, and what that crate may export |
| [Tokens and token streams](tokens_and_token_streams/README.md) | The four kinds of token tree a macro reads and writes, with their spacing and spans |
| [`proc-macro2` makes it testable](proc_macro2_makes_it_testable/README.md) *(outline)* | Why `proc_macro` panics outside the compiler, and the wrapper that does not |
| [Parsing with `syn`](parsing_with_syn/README.md) *(outline)* | Tokens into a syntax tree: `DeriveInput`, `Data`, `Fields` |
| [Generating with `quote`](generating_with_quote/README.md) *(outline)* | Rust with holes in it: interpolation, repetition, new identifiers |
| [The re-export pattern](the_reexport_pattern/README.md) *(outline)* | Why `serde` and `thiserror` are two crates each, and how the user sees one |
| [Testing with `trybuild`](testing_with_trybuild/README.md) *(outline)* | Putting the compiler errors your users see under test |

## 3. Derive macros

| Lesson | What it answers |
|---|---|
| [Every struct and enum shape](every_struct_and_enum_shape/README.md) | Named, tuple and unit fields, and enums with all three |
| [Absolute paths and hygiene](absolute_paths_and_hygiene/README.md) | Generated code that compiles in a crate with its own `Result`, and what `mixed_site` does and does not protect |
| [Errors: from `panic!` to `syn::Error`](errors_from_panic_to_syn_error/README.md) | Three ways to fail, and the one that underlines the right token |
| [Helper attributes by hand](helper_attributes_by_hand/README.md) | Container and field attributes, parsed with `parse_nested_meta` |
| [Helper attributes with `darling`](helper_attributes_with_darling/README.md) | The same attributes, declared as a struct |
| [Generics, lifetimes and `where`](generics_lifetimes_and_where/README.md) | An `impl` that repeats every parameter and bound the type has, and which bound to add |

## 4. Function-like macros

| Lesson | What it answers |
|---|---|
| [Why `println!` is a macro](why_println_is_a_macro/README.md) | What no function signature can take, and what `format_args!` checks while compiling |
| [Parsing arbitrary tokens](parsing_arbitrary_tokens/README.md) | Your own syntax between the delimiters, with `syn::parse::Parse` |
| [When `macro_rules!` runs out](when_macro_rules_runs_out/README.md) | The exact point a declarative macro stops, and a procedural one starts to pay |
| [A `routes!` macro](a_routes_macro/README.md) | A small DSL, checked at compile time |

## 5. Attribute macros

| Lesson | What it answers |
|---|---|
| [Parse, tweak, re-emit](parse_tweak_reemit/README.md) | Changing a function and handing all of it back |
| [Re-emit the item on error](re_emit_on_error/README.md) | One error for the user instead of one per method call |
| [A `#[retry]` attribute](a_retry_attribute/README.md) | `#[retry(times = 3, delay_ms = 100)]`, with arguments parsed by `darling` |

## 6. Putting it all together

| Lesson | What it answers |
|---|---|
| [A `StateMachine` derive](a_state_machine_derive/README.md) | Variants, helper attributes, transitions rejected at compile time, identifiers that never clash |

## How these pages are checked

A procedural macro cannot be a single file: it needs a crate of its own, and usually `syn` and `quote` from crates.io. So a lesson here keeps a small Cargo workspace in its `demo/` folder, with a committed `Cargo.lock`, and declares the commands it runs in `demo/cargo_runs.toml`. [`tools/run_cargo_demos.py`](../tools/run_cargo_demos.py) runs each one with `--locked`, holds its output to an answer key in `demo/keys/`, and pastes that output into the page. Compiler errors are recorded the same way, from runs that are declared to fail. CI runs it as its own job, beside the one that checks every single-file example.

To run a demo yourself, `cd` into its `demo/` folder and use the command the page shows. The toolchain is the repo's pin, 1.98.0.

## What this chapter does not cover

- **`macro_rules!` itself.** That is [Declarative macros](../38_Declarative_Macros/README.md), after [Macros](../25_Control_Flow/macros/README.md) introduces the `!`. This chapter compares declarative macros with procedural ones, and teaches only the second.
- **Build scripts.** A `build.rs` also generates code before your crate compiles, but it writes files rather than transforming tokens, and it sees none of your source.

## See also

- [Macros](../25_Control_Flow/macros/README.md) — what the `!` means at a call site
- [Declarative macros](../38_Declarative_Macros/README.md) — `macro_rules!`, the other half of the topic
- [Traits](../12_Traits/README.md) — nearly every derive generates a trait `impl`
- [Generics](../22_Generics/README.md) — what a derive on a generic type has to repeat
- [The Rust Reference: Procedural macros ↗](https://doc.rust-lang.org/reference/procedural-macros.html)
- [The `proc_macro` crate ↗](https://doc.rust-lang.org/proc_macro/)

## Po polsku

**Makro proceduralne** (*procedural macro*) to kod w Ruscie, który kompilator uruchamia na twoim kodzie: dostaje **tokeny** (*tokens*) i zwraca tokeny. Rozdział idzie w kolejności jednodniowego kursu: najpierw makra, których już używasz (`#[derive(Error)]` z `thiserror` rozwinięte linijka po linijce), potem narzędzia — osobny crate typu `proc-macro`, `proc-macro2`, `syn` do **parsowania** i `quote` do **generowania** kodu, `trybuild` do testowania komunikatów błędów — a dalej trzy rodzaje makr po kolei: **derive**, **funkcyjne** i **atrybutowe**. Na końcu jest projekt łączący wszystko: `#[derive(StateMachine)]`, który odrzuca niedozwolone przejście między stanami już podczas kompilacji, z błędem wskazującym dokładnie ten token, który jest zły.

Każda strona opiera się na małym workspace Cargo w folderze `demo/`; to, co program wypisał — także komunikaty kompilatora — wkleja narzędzie, nie człowiek.

**Szukaj po polsku:** makra proceduralne w Ruscie · `syn` i `quote` · `rust derive macro tutorial` · `rust proc macro workshop` · `darling FromDeriveInput`
