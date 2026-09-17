# Rewriting a module

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A migration that works replaces one C module at a time behind its existing header — the header is the contract, the old C implementation is the test oracle, and each step is small enough to switch back — until the Rust side is large enough that the boundary moves.

## What it has to cover

- **Choosing the first module:** a leaf with few callers, a clear header, tests or a way to write them, and a real reason to move it (a parser handling untrusted input, not the stable `main`)
- **The header stays:** the Rust module exports exactly the functions the C header declares, generated and diffed against the original
- **Differential testing:** the C and Rust implementations built side by side and fed the same inputs — including fuzzed ones — until their outputs agree
- **Switching:** a build flag that links one implementation or the other, so a regression is a rebuild rather than a revert
- **Performance:** measured before and after, with the boundary crossings counted, since a chatty interface costs more than the code behind it
- **Debugging a mixed binary:** one debugger session stepping from C into Rust and back, and the symbols each side needs for that to work
- **When the boundary moves:** once most callers are Rust, the C remainder becomes the thing wrapped, and the direction of the bindings flips
- Maintenance: who owns the FFI layer, and the point at which the C original is deleted

## The trap it exists for

Rewriting the module and its interface at once. Every C caller then has to change in the same step, the old implementation can no longer serve as the oracle, and a migration that was meant to be incremental becomes the big-bang rewrite it set out to avoid.

## See also

- [The C ABI](../the_c_abi/README.md) — the contract the header stands for
- [Testing FFI code](../testing_ffi_code/README.md) — the tools the differential tests run under
- [Debugging](../../../32_Debugging/README.md) — the library's map of debugging tools
- [What the debugger records ↗](https://masiarek.github.io/c-learning-library/04_Debugging/what_the_debugger_records/index.html) — lldb and gdb on a C program, in the C learning library
- [Refactoring to Rust ↗](https://www.manning.com/books/refactoring-to-rust) — a book-length version of this playbook

## Po polsku

Migracja, która się udaje, zastępuje po jednym module C naraz, zostawiając jego plik nagłówkowy bez zmian. **Nagłówek jest kontraktem**, stara implementacja w C służy za **wyrocznię testową** (*test oracle*) w testach porównawczych, a każdy krok jest na tyle mały, że da się go wycofać przełącznikiem w budowaniu. Zmiana modułu i jego interfejsu jednocześnie zamienia migrację przyrostową z powrotem w przepisywanie wszystkiego naraz.

**Szukaj po polsku:** stopniowa migracja z C do Rusta · testy porównawcze · `incremental c to rust migration` · `differential testing c rust`
