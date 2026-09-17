# `proc-macro2` makes it testable

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `proc_macro`'s types only work while the compiler is running a macro — call them anywhere else and they panic — so macro logic is written against `proc_macro2`, which works in a unit test too.

## What it has to cover

- The panic, recorded: `proc_macro::TokenStream` used from an ordinary binary or test
- `proc_macro2::TokenStream` doing the same thing without complaint
- The shape every serious macro has: a thin `#[proc_macro_derive]` wrapper converting with `.into()`, and a `fn expand(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream` that holds the logic
- A unit test of `expand` asserting on the generated tokens (and why comparing `to_string()` is fragile)

## See also

- [Tokens and token streams](../tokens_and_token_streams/README.md) — the types being wrapped
- [Testing with `trybuild`](../testing_with_trybuild/README.md) — the other half: testing what the user sees
- [Procedural macros](../README.md) — the chapter, in reading order
