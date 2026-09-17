# Exporting a macro

**Level:** 301 · deep dive

**One line:** A `macro_rules!` macro exists only below its definition — textual scope, like a `let` — until `pub(crate) use name;` gives it a path inside the crate or `#[macro_export]` publishes it at the crate root for everyone.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Textual scope: a call above the definition is *cannot find macro `twice` in this scope*, and 1.98.0 adds *consider moving the definition of `twice` before this call* and points at the later definition. Textual scope also flows into child modules declared after the definition, even across files
- Module order: a function in a `mod` declared before `mod helpers` cannot call a macro defined inside `helpers` by name, and the help line on 1.98.0 suggests `#[macro_use]`; `helpers::double!(4)` fails differently, with `E0433`
- `pub(crate) use double;` written after the definition gives the macro path-based scope, so `crate::helpers::double!(4)` works from any module, including one earlier in the file
- The implicit visibility: a macro is `pub(crate)` until `#[macro_export]` makes it `pub`, so `pub use private_m;` is refused while `pub(crate) use private_m;` is fine
- `#[macro_export]` places the macro at the crate root whichever module it is written in: `crate::triple!` inside the crate, `your_crate::triple!` from outside — and where does rustdoc list it?
- The two `#[macro_use]`s: on a module it stretches textual scope past the module's closing brace; on `extern crate` it is the pre-2018 way to import another crate's macros, which clippy's `macro_use_imports` (pedantic) suggests replacing with `use`
- `#[macro_export(local_inner_macros)]`, which the Reference calls a migration aid discouraged in new code, against writing `$crate::helper!` yourself — see [Hygiene](../hygiene/README.md)

## The trap it exists for

Moving a working macro into its own module file, getting *cannot find macro* at every call site, and fixing it with `#[macro_export]`. The errors go away, and the macro is now part of the crate's public API at the root, documented and semver-bound, when `pub(crate) use name;` was all the crate needed.

## Where this sits

[Modules and visibility](../../27_Modules/modules_and_visibility/README.md) and [Bringing names in with `use`](../../27_Modules/the_use_declaration/README.md) cover path-based scope for items; a macro follows those rules only after it has been given a path. [One module per file](../../27_Modules/one_module_per_file/README.md) is where the "moved it into a file" trap starts. This page is only how a macro's own name becomes reachable.

## See also

- [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) — `pub(crate)` and the module wall for ordinary items
- [Bringing names in with `use`](../../27_Modules/the_use_declaration/README.md) — the declaration that also re-exports a macro
- [One module per file](../../27_Modules/one_module_per_file/README.md) — why moving code into a file changes declaration order
- [Hygiene](../hygiene/README.md) — `$crate`, which an exported macro needs
- [Two versions of one crate in one binary](../../05_Tooling/two_versions_of_one_crate/README.md) — what depending on an exported macro commits a caller to
- [Macros by example: scoping, exporting and importing ↗](https://doc.rust-lang.org/reference/macros-by-example.html#r-macro.decl.scope) — textual and path-based scope, `macro_use` and `macro_export`

## If you are coming from another language

- **C.** A `#define` is visible from its line to the end of the translation unit, and it is shared by putting it in a header that others `#include`. Rust's textual scope ends where the enclosing module ends, and sharing is an attribute or a `use`, not a file copied into place.
- **C++.** C++20 named modules do not export macros at all — importing a module brings in declarations only, and a header unit is needed to carry macros. Rust went the other way and gave macros paths.

## Po polsku

Makro `macro_rules!` ma najpierw tylko **zasięg tekstowy** (*textual scope*): istnieje od miejsca definicji w dół, jak zmienna z `let`, więc wywołanie powyżej definicji albo z modułu zadeklarowanego wcześniej kończy się błędem *cannot find macro*. Ścieżkę dostaje dopiero przez `pub(crate) use nazwa;` (w obrębie crate'a) albo `#[macro_export]` (w korzeniu crate'a, publicznie dla wszystkich). Najczęstszy błąd to sięganie po `#[macro_export]` tylko po to, żeby uciszyć kompilator — makro staje się wtedy częścią publicznego API.

**Szukaj po polsku:** eksport makra · zasięg makr · `rust macro_export vs pub use` · `rust cannot find macro in this scope` · `macro textual scope`
