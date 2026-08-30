# Enums

**One line:** An enum names a closed set of alternatives and makes that set a type — and because a `match` on it has to account for every alternative, the compiler can tell you which code a new alternative breaks.

Rust's enum is not C's. A C `enum` is an integer with names attached; Rust's is a **sum type** — each variant may carry its own data, of its own shape — which is why `Option`, `Result`, and most well-designed Rust data models are enums and nothing about them is built into the language.

Two things follow, and the section is mostly about the second. An enum lets you model *"exactly one of these"* so that the impossible combinations cannot be written down. And `match` turns adding a variant into a build error everywhere the new case matters, which converts a discipline you have to remember into a list the compiler computes.

| Lesson | Level | What it teaches |
|---|---|---|
| [What an enum is](what_an_enum_is/README.md) | 101 → 201 | The declaration, the four shapes of variant, behaviour in an `impl` block, and the `E0004` you get for free when a variant is added — plus why a fieldless enum casts to an integer and a payload-carrying one does not |
| [Variants that carry data](variants_that_carry_data/README.md) | 201 | Product versus sum, counted rather than asserted; what the payload costs, measured; and the niche that makes `Option<Box<T>>` the same eight bytes as the pointer |
| [A typo becomes a binding](a_typo_becomes_a_binding/README.md) | 201 | Under `use Enum::*`, a mistyped arm is a catch-all variable, not an error — zero warnings, and the two lints that look like they should catch it do not |
| [An enum as a state machine](an_enum_as_a_state_machine/README.md) | 201 | `match (state, event)`: the compiler enumerates the whole transition table, names the cells you left out, and stops the moment you write `_` |

## Enum pages that live elsewhere

`Option` and `Result` are ordinary enums, and they are met on the first day rather than in a section about enums — so their lessons stay where a beginner will find them. This section links them rather than moving them:

- [`Some` and `None`](../17_Option_and_Result/some_and_none/README.md) — the two-variant enum in the prelude
- [`Option` vs `Result`](../17_Option_and_Result/option_vs_result/README.md) — absence versus failure
- [Six kinds of zero](../17_Option_and_Result/six_kinds_of_zero/README.md) — writing your own when two variants are not enough
- [`if let`](../17_Option_and_Result/if_let/README.md) and [`while let`](../17_Option_and_Result/while_let/README.md) — matching one variant, and giving up exhaustiveness to do it
- [One arm, many values](../17_Option_and_Result/one_arm_many_values/README.md) — `A | B` and ranges
- [`Option` is a one-item collection](../17_Option_and_Result/option_as_collection/README.md) — the discriminant, and when the tag is free
- [Nullable pointers](../17_Option_and_Result/nullable_pointers/README.md) — `Option<Box<T>>`, and recursive types
- [What a union is](../09_Advanced/what_a_union_is/README.md) — the untagged version, and the desync only it can have
- [Generic enums](../22_Generics/generic_enums/README.md) — `<T>` on an enum, and the two parameters `Result<T, E>` is made of
- [A generic recursive type](../22_Generics/a_generic_recursive_type/README.md) — the `Box` a type that contains itself needs, and why a hand-rolled `Next`/`End` enum is `Option` with the API removed

[OPTION.md](../OPTION.md) is the full reading order for `Option` itself.

## Not yet written

The rest of the map, listed here rather than as empty pages so the gaps are visible: **enums with a lifetime parameter** (`enum Token<'a>`, which is the other thing those brackets hold), **`#[non_exhaustive]`** and what it does to a downstream crate's `match`, **explicit discriminants and `repr`** (`enum Code { Ok = 200 }`, `repr(u8)`, and converting an integer *back* with `TryFrom`), **custom error enums** with `Display`, `Error` and `From` (the `thiserror` shape, by hand first), **implementing `Display` for an enum** — the `match` inside `fmt`, which is the usual first hand-written trait impl — and **match guards and binding `@`**.
