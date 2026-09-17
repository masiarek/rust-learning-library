# The re-export pattern

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A proc-macro crate can export only macros, so a library ships two crates — `serde` and `serde_derive`, `thiserror` and `thiserror-impl` — and the one users depend on re-exports the derive beside the trait it implements.

## What it has to cover

- A facade crate with the trait, and `pub use my_derive::MyTrait;` — the trait and the derive share a name, in two namespaces
- serde's way (an optional `derive` feature) and thiserror's way (always on), read from their published manifests
- Why the generated code must name the trait through the facade (`::my_facade::MyTrait`), and what breaks when a user depends only on the derive crate or renames the dependency
- Hidden re-exports for generated code (`#[doc(hidden)] pub mod __private`)

## See also

- [A proc-macro crate](../a_proc_macro_crate/README.md) — why the split is forced
- [Absolute paths and hygiene](../absolute_paths_and_hygiene/README.md) — naming the facade from generated code
- [Procedural macros](../README.md) — the chapter, in reading order
