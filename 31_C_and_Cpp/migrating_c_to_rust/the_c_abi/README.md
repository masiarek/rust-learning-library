# The C ABI

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** C and Rust meet at the binary level, not the source level — a symbol name in an object file, and a calling convention both compilers follow — so an `extern "C"` declaration is a promise about a function the Rust compiler never sees, and a wrong signature is undefined behaviour rather than an error.

## What it has to cover

- What an ABI fixes that an API does not: argument passing, return values, which registers survive a call, struct layout
- **Calling C from Rust:** an `unsafe extern "C"` block written by hand, and why edition 2024 put the `unsafe` on the block — the promise is made at the declaration
- **Calling Rust from C:** `extern "C" fn` plus `#[unsafe(no_mangle)]`, and the mangled name you get without it, read with `nm`
- **`#[repr(C)]`:** when a struct needs it (whenever its layout crosses), and when it does not (a pointer to it is all C ever sees)
- The linker's part: an undefined symbol at link time against a mismatched symbol that links and then misbehaves
- **A wrong signature, on purpose:** declare a C function with one argument too few or an `i32` where C returns a `double`, and record what happens — no compiler error, and output that depends on the machine
- The `improper_ctypes` lints, and which mistakes they do and do not catch

## The trap it exists for

Treating the `extern` block as checked. Rust takes the declaration on trust; the header it was copied from can change, and nothing between the two will notice.

## See also

- [Calling C](../../../09_Advanced/calling_c/README.md) — the round trip, verified, including edition 2024's `unsafe extern`
- [The linker](../../../20_Compilers/the_linker/README.md) and [Targets and triples](../../../20_Compilers/targets_and_triples/README.md) — who resolves the symbol, and which ABI a target means
- [Building and linking](../building_and_linking/README.md) — the next page
- [What the symbol table lists ↗](https://masiarek.github.io/c-learning-library/02_Decompiling/what_the_symbol_table_lists/index.html) — symbols from C's side, in the C learning library
- [External blocks ↗](https://doc.rust-lang.org/reference/items/external-blocks.html) and [ABI ↗](https://doc.rust-lang.org/reference/abi.html) in the Reference

## Po polsku

C i Rust spotykają się na poziomie binarnym, nie źródłowym: łączy je nazwa symbolu w pliku obiektowym i **konwencja wywołań** (*calling convention*), której przestrzegają oba kompilatory. Blok `extern "C"` to obietnica złożona kompilatorowi w sprawie funkcji, której on nigdy nie widzi — dlatego błędna sygnatura nie daje błędu kompilacji, tylko **zachowanie niezdefiniowane**.

**Szukaj po polsku:** interfejs binarny C · konwencja wywołań · `rust extern c no_mangle` · `rust repr c struct ffi`
