# Helper attributes by hand

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A derive declares the attributes it reads — `#[proc_macro_derive(Builder, attributes(builder))]` — and parses container attributes (`#[builder(...)]` on the struct) and field attributes (`#[builder(default)]` on a field) from `attrs` itself, with `parse_nested_meta`.

## What it has to cover

- Declaring the helper attribute, and the error a user gets for an undeclared one
- Container versus field attributes, read from `DeriveInput::attrs` and `Field::attrs`
- `Attribute::parse_nested_meta`: flags, `key = value`, and rejecting an unknown key with a spanned error
- What this costs in code, which is the argument for `darling`

## See also

- [Helper attributes with `darling`](../helper_attributes_with_darling/README.md) — the same attributes, declared as a struct
- [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) — the errors an unknown key should produce
- [Procedural macros](../README.md) — the chapter, in reading order
