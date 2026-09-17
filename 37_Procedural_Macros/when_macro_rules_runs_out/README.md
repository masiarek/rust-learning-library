# When `macro_rules!` runs out

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `macro_rules!` matches patterns and substitutes; it cannot build a new identifier, inspect a string literal, count without recursion, or report an error of its own shape — and each of those is where a function-like procedural macro starts to pay for its extra crate.

## What it has to cover

- One task tried with `macro_rules!` first, and the exact place it stops, recorded
- The same task as a function-like proc macro
- The cases where `macro_rules!` is still the better tool (no extra crate, faster builds, works in the same crate)
- A short decision table specific to function-like macros

## See also

- [A function, `macro_rules!`, or a procedural macro](../function_macro_rules_or_proc_macro/README.md) — the general version of this decision
- [A `routes!` macro](../a_routes_macro/README.md) — a DSL that needs the procedural version
- [Procedural macros](../README.md) — the chapter, in reading order
