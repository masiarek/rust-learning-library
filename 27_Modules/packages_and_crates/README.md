# Packages and crates

**Level:** 101 · for newcomers

**One line:** A **package** is a folder with a `Cargo.toml`; a **crate** is one unit rustc compiles — a library or a binary — and a package holds at most one library crate and any number of binary crates, each with its own module tree rooted at one file.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The three words and the file each one lives in: package (`Cargo.toml`), crate root (`src/lib.rs`, `src/main.rs`, `src/bin/*.rs`), module (`mod name;` inside a crate)
- A binary crate using the library crate of the same package: `use my_package::thing;`, and why the name uses `_` where the package name used `-`
- `crate::`, `self::` and `super::` as paths from inside one crate, and the package name as the path from outside it
- What `rustc file.rs` alone makes: one crate, no package — how this library's examples work (see [running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md))
- How crates.io uses "crate" to mean a package — and why that loose usage is harmless until you have a package with a library and two binaries
- Workspaces as the level above packages — see [workspaces](../../05_Tooling/workspaces/README.md)

## The trap it exists for

Putting shared code in `src/main.rs` and trying to use it from `src/bin/other.rs`. Each file under `src/bin/` is its own crate, so it cannot see `main.rs`'s modules. The shared code belongs in `src/lib.rs`, which every binary in the package can `use`.

## Where this sits

[Modules and visibility](../modules_and_visibility/README.md) is the tree inside one crate. [One module per file](../one_module_per_file/README.md) is how that tree maps onto files. This page is the two levels above.

## See also

- [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) — the tree inside a crate
- [One module per file](../../27_Modules/one_module_per_file/README.md) — `mod name;` and the file next door
- [Bringing names in with `use`](../../27_Modules/the_use_declaration/README.md) — `crate::` and friends
- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — one crate, no package, and `src/bin/`
- [Workspaces](../../05_Tooling/workspaces/README.md) — the level above packages
- [Publishing a crate](../../05_Tooling/publishing_a_crate/README.md) — what "crate" means on crates.io

## If you are coming from another language

- **Python.** A distribution (what `pip` installs, named in `pyproject.toml`) contains import packages and modules; Rust's package → crate → module is the same three levels, with the crate as a compilation unit Python does not have.
- **Go.** A Go module (`go.mod`) holds packages, and a package is one directory; Rust's module tree is inside the crate rather than one per directory.
- **Java.** A Maven artifact is the package, a JAR is roughly the crate, and Java packages are the module tree.
- **C and C++.** No standard unit above the translation unit: a static library, a shared library or an executable is what a crate compiles into.

## Po polsku

Pakiet (*package*) to katalog z `Cargo.toml`; skrzynia (*crate*) to jedna jednostka kompilacji — biblioteka albo program wykonywalny. Pakiet ma najwyżej jedną skrzynię biblioteczną i dowolnie wiele binarnych; każda ma własne drzewo modułów, dlatego plik w `src/bin/` nie widzi modułów z `main.rs`.

**Szukaj po polsku:** pakiet a skrzynia · moduły w Ruście · `rust package vs crate vs module` · `cargo src/bin shared code lib.rs`
