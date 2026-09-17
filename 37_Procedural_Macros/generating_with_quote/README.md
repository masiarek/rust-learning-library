# Generating with `quote`

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `quote!` writes Rust with holes in it: `#name` interpolates one value, `#(#fields),*` repeats over an iterator, and `format_ident!` builds a new identifier — the result is a `proc_macro2::TokenStream`.

## What it has to cover

- Interpolation of `Ident`, `Type`, expressions, and anything `ToTokens`
- Repetition, with and without separators, and zipping two iterators
- `format_ident!` versus `Ident::new`, and the span a new identifier gets
- Why not `format!` + `.parse()` (the approach the first lesson used): errors without locations, and lost spans
- Printing the generated tokens, and formatting them for reading with `prettyplease` if the page needs it

## See also

- [Parsing with `syn`](../parsing_with_syn/README.md) — where the pieces come from
- [Absolute paths and hygiene](../absolute_paths_and_hygiene/README.md) — what to write inside `quote!`
- [Procedural macros](../README.md) — the chapter, in reading order
