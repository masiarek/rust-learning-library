# Rust without the standard library

**One line:** `std` is `core` plus `alloc` plus an operating system, and `#![no_std]` removes the operating system and the heap — along with everything the runtime was quietly doing before your `main` ran and after your program panicked.

This section is for code with nothing underneath it: a microcontroller, a bootloader, a kernel, a module that must not assume a host. It follows chapter 12 of *Rust for Rustaceans*, "Rust Without the Standard Library" ([Books](../10_Resources/books/README.md) says where that book sits): what disappears, what you now write yourself, and how the type system keeps a peripheral from being misused. It is not a tour of boards, probes and HALs — that is [The Embedded Rust Book ↗](https://docs.rust-embedded.org/book/) and, one layer lower, [the Embedonomicon ↗](https://docs.rust-embedded.org/embedonomicon/) — and it is not about `unsafe` in general, which [What `unsafe` turns off](../09_Advanced/what_unsafe_turns_off/README.md) covers. The allocator itself is [The global allocator](../09_Advanced/the_global_allocator/README.md); what a target triple decides is [Compilers](../20_Compilers/README.md).

| Lesson | Level | What it covers |
|---|---|---|
| [`core`, `alloc` and `std`](core_alloc_and_std/README.md) | 201 | The three layers, what `#![no_std]` removes, and why `HashMap` is on the far side of the line — Stub |
| [The panic handler](the_panic_handler/README.md) | 301 | The one `#[panic_handler]` a `no_std` binary must supply, and what std's handler did that you now choose — Stub |
| [Allocating without std](allocating_without_std/README.md) | 301 | `extern crate alloc` and the `#[global_allocator]` a device has to provide from a region of RAM you picked — Stub |
| [What runs before `main`](before_main_runs/README.md) | 301 | std's start-up work (`SIGPIPE`, descriptors, arguments, exit code 101), `#![no_main]`, and the reset handler — Stub |
| [Out of memory](out_of_memory/README.md) | 301 | Why a failed allocation aborts instead of returning an error, and `try_reserve` as the one stable way to ask first — Stub |
| [Memory-mapped registers](memory_mapped_registers/README.md) | 301 | A peripheral is an address: volatile reads and writes, `&raw const`, and register crates generated from SVD — Stub |
| [Hardware you cannot misuse](hardware_you_cannot_misuse/README.md) | 301 | Typestate pins, zero-sized mode tags, `take()`-once singletons and the `embedded-hal` traits — Stub |
| [Cross-compiling](cross_compiling/README.md) | 301 | `rustup target add`, the linker it does not install, and `cross` or a container for the rest — Stub |

**Every page here is a stub** — an outline with its boundaries and its trap written down, and no runnable example yet. [CONTRIBUTING.md](../CONTRIBUTING.md) says what graduating one takes.

## How these pages graduate

Bare `rustc` can already build a `#![no_std]` library for the host with `-C panic=abort` — that is how every compiler message quoted in these outlines was checked on 1.98.0 — so a page graduates with such a crate as its checked example, and where the program has to run on a chip, with `rustc --target thumbv7em-none-eabihf` inside a Docker demo under QEMU, its transcript kept as a dated "Real runs" fence rather than an answer key.

## Where it goes next

[WebAssembly](../42_WebAssembly/README.md) is the other place Rust runs with no operating system — `wasm32v1-none` is a `core`-and-`alloc` target, and `wasm32-unknown-unknown` keeps a `std` whose operating-system half returns errors. [Targets and triples](../20_Compilers/targets_and_triples/README.md) and [The linker](../20_Compilers/the_linker/README.md) are the compiler's side of the same move. And [C and C++](../31_C_and_Cpp/README.md) is the list of bugs that this section's raw pointers can still reintroduce, and that its type-level tricks exist to keep out.

## Po polsku

Po polsku mówi się zwykle o **programowaniu bez biblioteki standardowej** albo po prostu o `no_std`, i warto od razu rozdzielić dwie rzeczy, które ta nazwa zlewa. `std` to w Ruscie trzy warstwy: `core` (typy i cechy, które nie potrzebują niczego — `Option`, `Result`, iteratory, wycinki), `alloc` (wszystko, co potrzebuje sterty — `Vec`, `String`, `Box`) i dopiero na wierzchu system operacyjny — wątki, pliki, zegar, `println!`. Atrybut `#![no_std]` zdejmuje warstwę systemową i stertę, a `core` zostaje w całości.

Druga rzecz jest mniej oczywista i to na niej ten rozdział się skupia: biblioteka standardowa to nie tylko funkcje, które wywołujesz, ale też kod, który **wykonuje się bez pytania** — przed `main` (ignorowanie `SIGPIPE`, argumenty programu) i po panice (komunikat, odwijanie stosu, kod wyjścia 101). Bez `std` każdą z tych decyzji podejmujesz sam: piszesz *panic handler*, wybierasz alokator, a na mikrokontrolerze także procedurę obsługi resetu (*reset handler*), która przygotowuje pamięć RAM, zanim wykona się jakakolwiek linijka Rusta. Wszystkie strony są na razie szkicami, a przykłady będą budowane dla hosta albo w kontenerze z emulatorem QEMU.

**Szukaj po polsku:** Rust bez biblioteki standardowej · programowanie systemów wbudowanych · mikrokontroler Rust · `rust no_std embedded` · `rust embedded book` · `embedonomicon`
