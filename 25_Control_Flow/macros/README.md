# Macros

**Level:** 101 → 201 · for newcomers

**One line:** The `!` means the call is expanded into source code before compilation, which is how `println!` can check your format string at compile time and how `vec![]` can take a list of anything — neither is possible for an ordinary function.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What the `!` actually tells you at a call site, and what it does not (it is not "unsafe", not "special", not a negation)
- The handful worth knowing on day one: `println!` / `format!` / `vec!` / `assert!` / `assert_eq!` / `panic!` / `dbg!` / `todo!`
- Why a macro can do things a function cannot: a variable number of arguments, arguments of different types, and **inspecting the source text** — which is how `dbg!` prints the expression you wrote
- Why the format string must be a literal, and the error you get when you pass a `String`
- The two kinds — declarative (`macro_rules!`) and procedural (`#[derive(Debug)]` is one) — named, with writing one deferred
- `cargo expand` as the way to stop guessing what a macro became

## The trap it exists for

A macro is not a function, so the usual intuitions about evaluation do not hold: an argument may be evaluated twice, never, or in a different order. This is why `assert!` can print the expression that failed, and why a macro that looks like it takes a value may actually be taking a *place*.

## See also

- [The braces take a name](../../15_First_Programs/braces_take_a_name/README.md) — `println!`'s format string in detail, and the three ways `{}` refuses
- [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md) — the macro that reads its own source text, and the five things it does that `println!` does not
- [`Debug` and `Display`](../../15_First_Programs/debug_vs_display/README.md) — the traits the printing macros call, and why only one can be derived
- [`unwrap` is a TODO](../../02_Errors/unwrap_is_a_todo/README.md) — `todo!` and `unimplemented!` as the honest placeholders
- [Comprehensive Rust: Macros ↗](https://google.github.io/comprehensive-rust/control-flow-basics/macros.html)
