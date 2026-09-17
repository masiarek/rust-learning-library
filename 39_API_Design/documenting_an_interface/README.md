# Documenting an interface

**Level:** 201 · working knowledge

**One line:** A public item's doc comment is the part of its contract the types cannot state — `# Errors`, `# Panics` and `# Safety` sections, examples that `cargo test` compiles and runs — and `#[must_use]` and `#[doc(hidden)]` move two more promises to where the compiler and rustdoc act on them.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The three headings the API Guidelines ask for (C-FAILURE): `# Errors` on every function returning `Result`, `# Panics` on every reachable panic, `# Safety` on every `unsafe fn` — and the rustdoc book's own `# Panics` example
- Which lint checks which heading, on clippy 0.1.98: `clippy::missing_errors_doc` and `clippy::missing_panics_doc` are pedantic, so off by default; `clippy::missing_safety_doc` is style, so on
- `#![warn(missing_docs)]`: the rustc lint, allow-by-default, that reports *missing documentation for a function* on each undocumented public item — and `rustdoc::missing_crate_level_docs` for the crate root
- Examples as tests: a Rust fence in a `///` comment is compiled and run by `cargo test` unless its info string says otherwise ([The example that is a test](../../28_Testing/doc_tests/README.md)); C-QUESTION-MARK asks that examples use `?` rather than `unwrap`, and the rustdoc book shows the two ways to make `?` compile there — a hidden `# fn main() -> io::Result<()> {` wrapper, or, since 1.34.0, a final `Ok` with its error type written out
- `#[must_use]` with a message: ignoring the result of `Request::with_retries(3)` warns *unused return value of `Request::with_retries` that must be used* and prints your message as a note; `clippy::must_use_candidate` and `clippy::return_self_not_must_use` (both pedantic) suggest where it belongs
- `#[doc(hidden)]` (C-HIDDEN) for items that must be `pub` — for a macro's expansion or a sibling crate — but are not part of the API: the item is still nameable by anyone, so is it still covered by semver?
- Links in docs (C-LINK): intra-doc links such as ``[`Vec`]`` resolved by rustdoc, and `rustdoc::broken_intra_doc_links`, warn by default, for the ones that no longer resolve
- Crate-level docs with an example (C-CRATE-DOC) and the `Cargo.toml` metadata docs.rs shows (C-METADATA)

## The trap it exists for

Leaving out `# Panics` because the panic "cannot happen" — an `.unwrap()` on a lookup whose key comes from the caller. The caller's input makes it happen, nothing in the signature warned them, and `clippy::missing_panics_doc` would have flagged it but is off unless you turn on `clippy::pedantic`.

## Where this sits

[Comments that compile](../../15_First_Programs/comments_that_compile/README.md) covers the six comment forms and what each attaches to; [The example that is a test](../../28_Testing/doc_tests/README.md) covers doc tests as tests; [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) covers attribute syntax. This page is what a public item's documentation must say, and which lints hold you to it.

## See also

- [The example that is a test](../../28_Testing/doc_tests/README.md) — the fences `cargo test` runs, and `#` lines
- [Comments that compile](../../15_First_Programs/comments_that_compile/README.md) — `///`, `//!` and where each one lands
- [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) — `#[must_use]`, `#[doc]` and the lint levels
- [`unwrap` is a TODO you forgot to remove](../../02_Errors/unwrap_is_a_todo/README.md) — the panic a `# Panics` section has to admit
- [Strict clippy](../../05_Tooling/strict_lints/README.md) — turning lint groups on for a whole crate
- [Making misuse a compile error](../making_misuse_a_compile_error/README.md) — the rules that are better as types than as docs
- [Rust API Guidelines — Documentation ↗](https://rust-lang.github.io/api-guidelines/documentation.html) — C-CRATE-DOC through C-HIDDEN
- [The rustdoc book: how to write documentation ↗](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html) — sections, examples and Markdown

## If you are coming from another language

- **Java.** Javadoc's `@throws` is `# Errors`; `@param` and `@return` have no Rust convention because the types carry most of that. Checked exceptions put part of `# Errors` into the signature, which Rust does with `Result<T, E>` and its `E`.
- **Python.** Sections such as `Raises:` in a docstring are convention only; `doctest` runs `>>>` examples the way `cargo test` runs doc fences, but only when you invoke it.
- **Go.** A doc comment is plain text above the declaration, and `Example` functions in `_test.go` files are compiled, shown in the documentation, and run by `go test` when they end in an `// Output:` comment — the same idea as a doc test, kept in a separate file.
- **C.** Preconditions live in a comment or a man page with nothing to enforce them — [The functions that do not check ↗](https://masiarek.github.io/c-learning-library/03_Strings/the_functions_that_do_not_check/). `# Safety` on an `unsafe fn` is the same comment, and the `unsafe` keyword makes every caller acknowledge it.

## Po polsku

Komentarz dokumentacyjny publicznej funkcji to ta część kontraktu, której nie wyrażą typy: sekcje `# Errors` (kiedy zwraca błąd), `# Panics` (kiedy panikuje) i `# Safety` (co musi zagwarantować wywołujący `unsafe fn`), a przykłady w nim są kompilowane i uruchamiane przez `cargo test`. Uwaga na lintery: `clippy::missing_panics_doc` i `missing_errors_doc` należą do grupy *pedantic*, więc domyślnie milczą, a `missing_docs` z `rustc` trzeba włączyć samemu.

**Szukaj po polsku:** dokumentacja API w Ruście · komentarze dokumentacyjne · `rust doc panics errors safety sections` · `rust missing_docs lint` · `rust must_use attribute`
