# Absolute paths and hygiene

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Generated code runs in somebody else's module, where `Result` may be their own type and `fmt` may not be imported — so it spells `::core::fmt::Result` in full, and it names its own helper identifiers so they cannot collide with the user's.

## What it has to cover

- The break, recorded: a derive that emits `fmt::Result`, used in a crate that defines its own `Result` or never imported `fmt`
- `::core::` and `::std::` with the leading `::`, and the user who has a module named `core`
- `Span::call_site()` versus `Span::mixed_site()`: which identifiers a user's code can see
- Local variables in generated code that shadow or are shadowed by the user's names, and how to name them safely

## See also

- [Expanding `thiserror`](../expanding_thiserror/README.md) — a real derive that does this
- [The re-export pattern](../the_reexport_pattern/README.md) — naming your own crate from generated code
- [Procedural macros](../README.md) — the chapter, in reading order
