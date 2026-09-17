# Parsing arbitrary tokens

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Between a function-like macro's delimiters the syntax is yours: implement `syn::parse::Parse` for a struct, read tokens with `input.parse()`, look ahead with `lookahead1`, and define keywords with `syn::custom_keyword!`.

## What it has to cover

- A small grammar that is not Rust, parsed into a struct
- `ParseStream`: `parse`, `peek`, `lookahead1`, and the error `lookahead` writes for you
- `Punctuated` for comma-separated lists, and `braced!` / `bracketed!` / `parenthesized!`
- `custom_keyword!` and `custom_punctuation!`
- The errors a user gets for bad input, recorded

## See also

- [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) — input that only has to be tokens
- [A `routes!` macro](../a_routes_macro/README.md) — this, used for a whole DSL
- [Procedural macros](../README.md) — the chapter, in reading order
