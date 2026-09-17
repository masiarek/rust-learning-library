# Testing with `trybuild`

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `trybuild` compiles small programs against your macro and compares each compiler error with a recorded `.stderr` file, so the messages your users see are under test too.

## What it has to cover

- `t.pass(...)` and `t.compile_fail(...)` in a `tests/` file
- The `.stderr` file: how it is produced (`TRYBUILD=overwrite`), what it holds, and how paths in it are normalized
- A failing UI test, recorded, to show what a regression looks like
- How this pairs with unit tests of the `proc_macro2` logic

## See also

- [`proc-macro2` makes it testable](../proc_macro2_makes_it_testable/README.md) — the unit-test half
- [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) — the messages worth testing
- [Procedural macros](../README.md) — the chapter, in reading order
