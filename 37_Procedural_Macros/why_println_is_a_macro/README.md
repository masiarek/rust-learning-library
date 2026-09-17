# Why `println!` is a macro

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A function cannot take a variable number of arguments of different types, and it cannot read its format string before the program runs; `println!` does both, because `format_args!` checks the string against the arguments at compile time.

## What it has to cover

- What a function version would have to look like, and what it would lose
- The compile-time errors `format_args!` gives for a missing argument, an extra one, a bad `{name}` — recorded
- `println!` is `macro_rules!` over the built-in `format_args_nl!` (std source at 1.98.0), so the checking is in the compiler, not in a proc macro
- What a function-like procedural macro can do in the same spirit: validate a literal while compiling

## See also

- [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) — built-in macros versus procedural ones
- [The braces take a name](../../15_First_Programs/braces_take_a_name/README.md) — the format string from the user's side
- [Procedural macros](../README.md) — the chapter, in reading order
