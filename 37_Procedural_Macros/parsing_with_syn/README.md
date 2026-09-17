# Parsing with `syn`

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `syn` turns a `TokenStream` into a syntax tree — `DeriveInput` for a derive, `ItemFn` for an attribute on a function — so a macro asks for `input.ident` and `data.fields` instead of counting tokens.

## What it has to cover

- `parse_macro_input!` and `syn::parse2` / `syn::parse_str`
- `DeriveInput`: `attrs`, `vis`, `ident`, `generics`, `data` — printed from a real struct with `extra-traits`' `Debug`
- `Data::Struct` / `Data::Enum` / `Data::Union`, `Fields::Named` / `Unnamed` / `Unit`
- Features: what `full` and `extra-traits` add, and what a derive-only macro needs
- This library pins `syn` 3; if a course or blog uses `syn` 2, note on the page any API the demo shows that differs (check the syn 3 release notes; do not assume)

## See also

- [Tokens and token streams](../tokens_and_token_streams/README.md) — what `syn` saves you from
- [Generating with `quote`](../generating_with_quote/README.md) — the other direction
- [Every struct and enum shape](../every_struct_and_enum_shape/README.md) — handling every `Fields` variant in a real derive
- [Procedural macros](../README.md) — the chapter, in reading order
