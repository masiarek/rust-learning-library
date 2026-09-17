# A proc-macro crate

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `proc-macro = true` makes a crate the compiler loads and runs during compilation; it may export macros and nothing else, and the crate that uses them just depends on it.

## What it has to cover

- The manifest: `[lib] proc-macro = true`, and the workspace layout (macro crate + the crate that uses it)
- Why it has to be a separate crate: it is compiled for the host and run by the compiler before your crate exists
- The refusal when a proc-macro crate exports a function, a trait or a type — the real error, recorded
- `proc_macro` is in the sysroot: Cargo passes it for a proc-macro crate, and under bare `rustc --crate-type proc-macro` you need `extern crate proc_macro;` (probed on 1.98.0: without it, E0432)
- The smallest useful derive: one `impl` for the named type, built without `syn` or `quote`

## See also

- [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) — what the crate's macros can be
- [The re-export pattern](../the_reexport_pattern/README.md) — how a library ships a trait and its derive together
- [Procedural macros](../README.md) — the chapter, in reading order
