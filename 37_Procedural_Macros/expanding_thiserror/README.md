# Expanding `thiserror`

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `#[derive(Error)]` writes a `Display` impl, an `Error` impl and a `From` impl per `#[from]` field, and the expansion shows every line of it — with nothing a hand-written impl could not have said.

## What it has to cover

- A small `thiserror` 2 enum: a unit variant, a tuple variant with `#[from]`, a struct variant with a field in the message, and a `#[source]`
- The expansion, recorded: `cargo expand` is what readers run; the key is recorded with the command underneath it (`cargo rustc … -Zunpretty=expanded`, allowed on stable by `RUSTC_BOOTSTRAP=1`), and the page says so
- Walk the generated code: the `Display` match, `source()`, the `From` impls, and the paths it spells as `::thiserror::__private…` or `::core::…` — why they are absolute
- What the derive did **not** generate, and why that is a design choice (no `Debug`)
- The same enum written by hand beside it: the derive saves typing, it adds no capability

## See also

- [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) — the kinds, and where `thiserror`'s derive is declared
- [Absolute paths and hygiene](../absolute_paths_and_hygiene/README.md) — why generated code spells `::core::fmt`
- [The re-export pattern](../the_reexport_pattern/README.md) — why you depend on `thiserror` but the macro is in `thiserror-impl`
- [Procedural macros](../README.md) — the chapter, in reading order
