# A function, `macro_rules!`, or a procedural macro

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Reach for a function first, `macro_rules!` when you need syntax a function cannot take, and a procedural macro only when you have to *read* the code you were given — a type's fields, a function's signature — or compute something no pattern can match.

## What it has to cover

- One task solved three ways where it can be, and the point at which each tool stops being able to do it
- What only a macro can do: variable arguments, taking a type or an identifier, generating items
- What only a procedural macro can do: iterate over a struct's fields, inspect a signature, build a new identifier, validate a string literal at compile time
- The costs, measured: a separate crate, `syn`'s compile time in a clean build, and debugging through an expansion
- The decision as a short table

## See also

- [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) — the three kinds
- [When `macro_rules!` runs out](../when_macro_rules_runs_out/README.md) — the same question for a function-like macro specifically
- [Procedural macros](../README.md) — the chapter, in reading order
