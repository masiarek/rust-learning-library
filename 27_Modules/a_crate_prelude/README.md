# A crate prelude

**Level:** 201 · working knowledge

**One line:** `use bevy::prelude::*;` is not a language feature — it is an ordinary module a crate author filled with `pub use` lines so one glob brings in the names you almost always need — and std's own prelude is the same idea, applied by the compiler to every module.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Writing one: `pub mod prelude { pub use crate::{Thing, OtherThing}; pub use crate::ext::MyExt as _; }` — including the `as _` trick that brings a trait's methods into scope without its name
- How glob imports resolve against explicit ones: an explicit `use` or a local item shadows a glob name, and two globs with the same name are an error only if you use that name
- std's prelude by edition: what `rust_2021` added (`TryFrom`, `TryInto`, `FromIterator`) and what the 2024 prelude added — verify against the [edition guide ↗](https://doc.rust-lang.org/edition-guide/rust-2024/prelude.html)
- Why preludes mostly hold *traits*: a method needs its trait in scope — see [a trait must be in scope](../../12_Traits/trait_in_scope/README.md)
- The cost: a reader cannot tell where a name came from, and a crate adding a name to its prelude can clash with yours
- `io::prelude` in std as a small, readable example

## The trap it exists for

Glob-importing two preludes that both export a name — two different `Result`s, or two `Error`s — and getting an ambiguity error at the first use, far from either `use` line. Import the second crate's items by name, or alias one.

## Where this sits

[Bringing names in with `use`](../the_use_declaration/README.md) covers globs and renames in general. [A trait must be in scope](../../12_Traits/trait_in_scope/README.md) is the reason preludes exist. This page is the pattern a crate author builds from both.

## See also

- [Bringing names in with `use`](../../27_Modules/the_use_declaration/README.md) — globs, renames and the collision a glob causes
- [A trait must be in scope](../../12_Traits/trait_in_scope/README.md) — why preludes are mostly traits
- [Extension traits](../../12_Traits/extension_traits/README.md) — the traits a prelude most often re-exports
- [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) — `pub use` as re-export
- [Resources and plugins](../../43_Games/resources_and_plugins/README.md) — Bevy's prelude in use

## If you are coming from another language

- **Python.** `from package import *` guided by `__all__` is the same mechanism with the same readability cost; Python's builtins are its std prelude.
- **Java.** `java.lang.*` is imported implicitly — the std prelude — and a wildcard `import com.x.*` is the crate prelude, minus the trait-scope reason for it.
- **C++.** `using namespace std;` is the cautionary version: a glob into a namespace large enough to collide with anything.

## Po polsku

Preludium skrzyni (`use bevy::prelude::*;`) to nie cecha języka, tylko zwykły moduł wypełniony liniami `pub use`, żeby jeden import z gwiazdką przyniósł najczęściej potrzebne nazwy. Preludium biblioteki standardowej działa tak samo, tylko dokłada je kompilator do każdego modułu — i jego zawartość zależy od edycji.

**Szukaj po polsku:** preludium skrzyni · import z gwiazdką · `rust prelude module pattern` · `rust 2024 prelude changes`
