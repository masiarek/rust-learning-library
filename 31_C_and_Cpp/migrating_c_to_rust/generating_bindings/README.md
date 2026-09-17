# Generating bindings

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Hand-written declarations drift from the header they copied, so both directions are generated — [`bindgen` ↗](https://rust-lang.github.io/rust-bindgen/) reads C headers and writes Rust declarations, and [`cheadergen` ↗](https://cheadergen.com/) or [`cbindgen` ↗](https://github.com/mozilla/cbindgen) read Rust and write C headers — which moves the question from "is this signature right" to "is this generator configured right".

## What it has to cover

- **C to Rust with `bindgen`:** running it from `build.rs` against running it once and committing the output, and what each choice does to a build machine without libclang
- Configuring it for a real header: allow-lists so the bindings are not the whole of `<stdio.h>`, opaque types, and how it renders enums, bitfields and `#define` constants
- **Rust to C:** a header generated from `extern "C"` items. `cheadergen` reads the compiler's own view through rustdoc JSON (and so needs a pinned nightly); `cbindgen` parses the source. What each sees that the other does not, such as items produced by macros
- Configuring the header for non-trivial types: opaque structs, `#[repr(C)]` enums, and one header per crate in a workspace
- Keeping generated files honest: a CI check that regenerates and fails on any difference

## The trap it exists for

Committing generated bindings once and never regenerating them. They were right on the day they were made; the header moved on, and the declarations now describe a library that no longer exists.

## See also

- [The C ABI](../the_c_abi/README.md) — what a generated declaration promises, exactly as a hand-written one does
- [Building and linking](../building_and_linking/README.md) — where `bindgen` usually runs
- [Documenting unsafe contracts](../documenting_unsafe_contracts/README.md) — Rust doc comments that end up in the generated C header
- [Going deeper — `unsafe` and FFI](../../../10_Resources/going_deeper/README.md#unsafe-and-ffi) — the tools' own books

## Po polsku

Deklaracje pisane ręcznie rozjeżdżają się z nagłówkiem, z którego je przepisano, więc oba kierunki się generuje: `bindgen` czyta nagłówki C i tworzy deklaracje w Ruscie, a `cheadergen` lub `cbindgen` czytają kod Rusta i tworzą **pliki nagłówkowe** C. Pytanie „czy ta sygnatura jest poprawna” zamienia się wtedy w pytanie „czy generator jest dobrze skonfigurowany” — i czy CI sprawdza, że wygenerowane pliki są aktualne.

**Szukaj po polsku:** generowanie wiązań · pliki nagłówkowe C z Rusta · `rust bindgen tutorial` · `cheadergen vs cbindgen`
