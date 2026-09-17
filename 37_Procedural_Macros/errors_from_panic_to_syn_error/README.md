# Errors: from `panic!` to `syn::Error`

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A macro that panics gives the user *"proc-macro derive panicked"* at the derive; `compile_error!` does better; a `syn::Error` spanned on the offending token puts the red line exactly where the mistake is, and several can be combined into one report.

## What it has to cover

- The same mistake reported three ways, each compiler message recorded
- `syn::Error::new_spanned`, `into_compile_error`, and `Error::combine` for more than one problem
- The `Result`-returning `expand` shape that makes `?` work inside a macro
- What the user sees in their editor for each

## See also

- [Re-emit the item on error](../re_emit_on_error/README.md) — the attribute-macro version of this problem
- [Testing with `trybuild`](../testing_with_trybuild/README.md) — keeping these messages under test
- [Procedural macros](../README.md) — the chapter, in reading order
