# Modules

**One line:** How a program larger than one file is organised — the namespace, the wall, the shortcut, and the two item kinds that live at the top of a module rather than inside a function.

A module does two jobs at once, and confusing them is where most of the difficulty comes from. It is a **namespace**, so two types can both be called `Result`. And it is a **privacy boundary**, so an invariant can be enforced by making the field private — with the catch that the boundary is the module, not the type, so everything in the same `mod` is on the inside.

| Lesson | Level | What it covers |
|---|---|---|
| [Packages and crates](packages_and_crates/README.md) | 101 | The two levels above a module — a package is a `Cargo.toml`, a crate is one compiled unit, and why `src/bin/` cannot see `main.rs`. Stub |
| [Modules and visibility](modules_and_visibility/README.md) | 201 | Private by default, the four `pub` forms, and the door a helper in the same module leaves open |
| [Bringing names in with `use`](the_use_declaration/README.md) | 101 → 201 | A shortcut, not an import — plus the rename that fixes a collision and the glob that causes one |
| [A crate prelude](a_crate_prelude/README.md) | 201 | `use x::prelude::*` is an ordinary module of `pub use` lines — mostly traits, and the glob collision it invites. Stub |
| [One module per file](one_module_per_file/README.md) | 201 | `mod name;` is a declaration; the tree is the same either way; and the file nobody declared |
| [`const` and `static`](const_and_static/README.md) | 201 | Substituted at every use, versus one address for the program — and `const fn` |
| [Items inside a function](items_inside_a_function/README.md) | 201 | A nested `fn` is an item, not a closure — it sees no locals (`E0434`), and its position in the block does not matter. Stub |
| [What an attribute is](what_an_attribute_is/README.md) | 201 | `derive`, the four lint levels, `cfg`, and the field order a derived `Ord` reads |
| [Conditional compilation](conditional_compilation/README.md) | 201 | `#[cfg]` deletes code before type checking, `cfg!` is a `bool` in code that always compiles — and the misspelled condition that compiles nothing. Stub |

## The order to read them

The first three are one story told from three sides: what a module *is*, how to refer into one, and where the files go. The last two are items that live at module level and have nowhere else to be taught — `const` and `static` because they are the first things you put next to a `mod`, and attributes because `#[derive]` is on almost every type in this library and nothing else defines it.

## What is next door

Testing is the other thing `#[cfg(test)]` is for, and it has [a section of its own](../28_Testing/README.md). The invariant that private fields protect is [the newtype](../16_Structs/newtype_score/README.md). And the `use` line whose absence produces the strangest error in Rust — a method that exists but cannot be found — is [a trait must be in scope](../12_Traits/trait_in_scope/README.md).

Cargo, crates and dependencies are toolchain rather than language, and live in [Tooling](../05_Tooling/README.md).

## Po polsku

Moduł (*module*) robi w Ruscie dwie rzeczy naraz i stąd bierze się większość trudności: jest przestrzenią nazw — dzięki czemu dwa różne typy mogą nazywać się `Result` — a przy okazji granicą prywatności. Ta druga rola zaskakuje osoby przychodzące z Javy czy C#, gdzie „prywatne” znaczy prywatne dla klasy: w Ruscie ścianę stawia **moduł**, a nie typ, więc każda funkcja leżąca w tym samym `mod` siedzi po wewnętrznej stronie i widzi prywatne pola cudzych struktur. Przy szukaniu materiałów warto pamiętać, że polski przekład Tour of Rust urywa się na rozdziale 5, a moduły są w rozdziale 9 — po polsku zostają pojedyncze wpisy blogowe, więc na konkretne pytanie i tak szybciej odpowie fraza angielska.

**Szukaj po polsku:** moduły w Ruscie · widoczność i prywatność w Ruscie · `rust module system` · `rust pub(crate) visibility`
