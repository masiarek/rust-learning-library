# Migrating C to Rust

**Level:** 201 → 301 · for C and C++ programmers

**One line:** Moving a C codebase to Rust is not one rewrite but many small crossings of a boundary — the C ABI — so the work is learning what that boundary promises, designing types and APIs that keep C's mistakes out of safe Rust, and then replacing one module at a time behind a header that never changes.

The rest of this section shows the bugs Rust refuses to build. This folder is the other practical question: you have a working C codebase, and you want Rust inside it without a big-bang rewrite.

**Every page below is a stub** — an outline and the questions the finished page has to answer. See [Adding a lesson](../../CONTRIBUTING.md#stubs). [Calling C](../../09_Advanced/calling_c/README.md) is the one verified page on the subject so far, and the natural first read.

## The pages, in order

| # | Page | Level | What it will teach |
|---|---|---|---|
| 1 | [The C ABI](the_c_abi/README.md) | 201 | `extern "C"`, symbols, the linker, `#[unsafe(no_mangle)]` — and what a declaration promises that nothing checks |
| 2 | [Building and linking](building_and_linking/README.md) | 201 → 301 | The `-sys` crate, `build.rs`, and fitting Cargo into a build that already has Make or CMake |
| 3 | [Generating bindings](generating_bindings/README.md) | 201 | `bindgen` from C headers to Rust, and `cheadergen` or `cbindgen` from Rust back to headers |
| 4 | [Data across the boundary](data_across_the_boundary/README.md) | 301 | Primitives, strings, structs and collections — and who frees each one |
| 5 | [FFI-safe types](ffi_safe_types/README.md) | 301 | `NonNull`, `Option`, `#[repr(transparent)]` newtypes, and enums a C integer must not become |
| 6 | [Validating at the boundary](validating_at_the_boundary/README.md) | 301 | Every exported function is a trust boundary: nulls, ranges, UTF-8, and panics |
| 7 | [Errors across FFI](errors_across_ffi/README.md) | 301 | `Result` stops at `extern "C"`: error codes, out-parameters, and a small, coarse API |
| 8 | [C idioms in Rust](c_idioms_in_rust/README.md) | 201 → 301 | Loops to iterators, out-parameters to return values, vtables to traits, flags to `bitflags` — and when to keep the C shape |
| 9 | [Safe wrappers](safe_wrappers/README.md) | 301 | A safe API over unsafe bindings: RAII handles, lifetimes, and the `Send`/`Sync` decision |
| 10 | [Documenting unsafe contracts](documenting_unsafe_contracts/README.md) | 301 | `# Safety` sections, numbered invariants, lints, and an FFI surface maintained as an API |
| 11 | [Testing FFI code](testing_ffi_code/README.md) | 301 | Tests, benchmarks, sanitizers, Valgrind and Miri — what each catches and where each stops |
| 12 | [Rewriting a module](rewriting_a_module/README.md) | 301 | The playbook: pick a module, keep its header, compare old and new, switch, repeat |

Pages 1–3 get Rust and C into one binary. Pages 4–7 are the boundary itself, from the data that crosses it to the errors that cannot. Pages 8–10 are what makes the Rust side worth having. Pages 11–12 are how you know it worked and how you do it again.

## Sources

This outline merges two course plans whose chapters overlap and between them cover the whole arc:

- [*The C to Rust Migration Book* ↗](https://mainmatter.com/c-to-rust-migration-book/) by Jonas Kruckenberg (Mainmatter) — six chapters from the C ABI to production-quality FFI, with [exercises on GitHub ↗](https://github.com/mainmatter/migrating-c-to-rust); new chapters were still being added when checked on 2026-09-16. Listed in [Going deeper](../../10_Resources/going_deeper/README.md#unsafe-and-ffi)
- A four-part *Migrating C to Rust* workshop outline: FFI-safe Rust, building and linking, idiomatic FFI, and testing with sanitizers, Valgrind and Miri — plus the module-rewrite process, performance, debugging and maintenance its introduction promises

## See also

- [Calling C](../../09_Advanced/calling_c/README.md) — a whole round trip with no build script, verified
- [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) — everything on these pages lives inside it
- [The linker](../../20_Compilers/the_linker/README.md) — what resolves the symbols page 1 declares
- [C learning library ↗](https://masiarek.github.io/c-learning-library/) — the C side: [a record on the wire ↗](https://masiarek.github.io/c-learning-library/05_Bytes_on_the_Wire/a_record_on_the_wire/index.html) is the struct-layout problem page 4 meets at the boundary, and [what the symbol table lists ↗](https://masiarek.github.io/c-learning-library/02_Decompiling/what_the_symbol_table_lists/index.html) is what page 1's symbols look like from outside
- [Refactoring to Rust ↗](https://www.manning.com/books/refactoring-to-rust) — the book-length treatment, listed on [Books](../../10_Resources/books/README.md)

## Po polsku

Przenoszenie kodu z C do Rusta rzadko jest jednym przepisaniem całości. To seria przejść przez granicę **interfejsu binarnego C** (*C ABI*): trzeba wiedzieć, co ta granica obiecuje, zaprojektować typy i API tak, żeby błędy przychodzące z C nie przedostały się do bezpiecznego Rusta, a potem zastępować po jednym module, zostawiając plik nagłówkowy bez zmian. **FFI** (*foreign function interface*) to po polsku zwykle „interfejs funkcji obcych”, ale w praktyce mówi się po prostu FFI.

**Szukaj po polsku:** migracja z C do Rusta · interfejs funkcji obcych · `rust ffi c migration` · `rust incremental rewrite c library`
