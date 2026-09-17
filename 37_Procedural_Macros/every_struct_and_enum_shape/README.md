# Every struct and enum shape

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A derive that only handles named fields breaks on the first tuple struct; match all of `Fields::Named`, `Fields::Unnamed` and `Fields::Unit`, and every variant of an enum, and the generated code has to address fields as `self.name` or `self.0`.

## What it has to cover

- One derive (for example a `describe()` or field-count method) applied to a named struct, a tuple struct, a unit struct and an enum with all three variant shapes
- Accessing unnamed fields: `syn::Index`, and why `self.#i` with a `usize` generates `self.0usize`
- Enums: a `match` arm per variant, with patterns for each shape
- Unions: refusing them with a spanned error rather than a panic

## See also

- [Parsing with `syn`](../parsing_with_syn/README.md) — the `Data` and `Fields` types
- [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) — refusing a union properly
- [Procedural macros](../README.md) — the chapter, in reading order
