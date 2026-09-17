# What runs before `main`

**Level:** 301 · deep dive

**One line:** `fn main` is not where a Rust program starts — std's runtime runs first and makes decisions you never wrote down — and `#![no_main]` hands that job back to you, down to the reset handler that has to prepare RAM before a single line of Rust can rely on it.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What std does on Unix before your `main`, read from 1.98.0's `library/std/src/sys/pal/unix/mod.rs`: reopens any closed standard descriptor on `/dev/null`, sets `SIGPIPE` to ignored, installs the stack-overflow handler, stores `argc`/`argv` for `env::args`, and names the main thread on Apple targets. Then `rt.rs` runs `main` inside `catch_unwind`, turns a panic into exit code 101, and flushes stdout on the way out.
- Why the `SIGPIPE` line is the one people meet: [Broken pipe](../../02_Errors/broken_pipe/README.md) is that single start-up decision, seen from `tool | head`.
- The refusals that prove the runtime is there. `#![no_std]` with an ordinary `fn main` stops at ``error: using `fn main` requires the standard library``, whose help line points at `#![no_main]`. Add `#![no_main]` on macOS and the next failure comes from [the linker](../../20_Compilers/the_linker/README.md), which lists `"_main"` under `Undefined symbols for architecture x86_64` — the platform still expects a symbol by that name (both on 1.98.0, x86_64 macOS).
- On a microcontroller there is no loader. The Embedonomicon's account: the first two vector-table entries are the initial stack pointer and the reset vector, and the reset handler is `extern "C" fn Reset() -> !` because there is no frame to return to. What must it do to the `.bss` and `.data` sections before Rust code can trust a `static`?
- What a runtime crate does for you: [`cortex-m-rt` ↗](https://docs.rs/cortex-m-rt)'s `#[entry]` function must have type `fn() -> !` and is called by the reset handler *after RAM has been initialized* (its docs). Which linker-script regions does it expect the application to name?
- Rust gives your own code no hook before `main`: a `static` needs a `const` initializer. What do `OnceLock` and `LazyLock` do instead, and where does a crate that runs code before `main` get its hook?
- A checkable example on the host: a `#![no_std]` `#![no_main]` library exporting `#[unsafe(no_mangle)] extern "C"` symbols, linked by a C program whose own `main` calls it — the ["no C runtime" build ↗](https://masiarek.github.io/c-learning-library/02_Decompiling/reading_the_memory_map/) in the C library is the same idea from the other side.

## The trap it exists for

Porting a working program to `no_std` and expecting it to behave like the original minus printing. What goes missing is everything the runtime did silently — the ignored `SIGPIPE`, the reopened descriptors, the exit code 101 — and on a chip, the initialisation of RAM itself, which nothing in the program's source appears to depend on.

## Where this sits

[The panic handler](../the_panic_handler/README.md) is the runtime's exit path; this page is its entry path. [The linker](../../20_Compilers/the_linker/README.md) explains the symbol-resolution step that the macOS failure above comes from, and [Broken pipe](../../02_Errors/broken_pipe/README.md) owns the `SIGPIPE` consequence in full.

## See also

- [Broken pipe](../../02_Errors/broken_pipe/README.md) — the start-up decision you are most likely to have already met
- [The linker](../../20_Compilers/the_linker/README.md) — where `_main` is looked for, and not found
- [The call stack](../../18_Ownership/the_call_stack/README.md) — the stack pointer the reset vector sets up
- [Catching a signal](../../09_Advanced/catching_a_signal/README.md) — what std does not install for you
- [Cross-compiling](../cross_compiling/README.md) — building the binary this page's reset handler lives in
- [The Embedonomicon: the smallest `#![no_std]` program ↗](https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html) and [memory layout ↗](https://docs.rust-embedded.org/embedonomicon/memory-layout.html)
- [head closes the pipe early ↗](https://masiarek.github.io/linux-learning-library/01_Pipelines/head_closes_the_pipe_early/) — the shell side of the `SIGPIPE` story

## If you are coming from another language

- **C.** The counterpart is the C start-up code that runs before `main` and calls it — on bare metal a start-up file that has the same `.data` and `.bss` work to do as the Embedonomicon's reset handler. Rust sits on the same layers and adds its own: on a hosted target, the `SIGPIPE` decision is written in `std` (its source comment explains why), not inherited from libc — and on a device the reset handler is a Rust function whose `-> !` the compiler checks.
- **C++.** Static constructors run before `main` in an order that crosses translation units unpredictably — the *static initialization order fiasco*. Rust refuses the whole category by requiring `const` initializers for `static`, and moves lazy set-up to `OnceLock`/`LazyLock`, where the first use decides the order.
- **Go.** Program execution begins by initialising every package — its `init()` functions included — and only then calls `main.main` (the Go spec, *Program execution*). Rust has no `init()`: nothing of yours runs before `main` unless a runtime crate or the reset handler calls it. See [`main` does not wait ↗](https://masiarek.github.io/go-learning-library/01_Goroutines/main_does_not_wait/) for what happens at the other end.
- **Java.** A class is initialised immediately before its first active use (JLS §12.4.1), so "before `main`" is spread across the program's lifetime. Rust's `LazyLock` is the explicit version of that behaviour, one value at a time.

## Po polsku

Funkcja `main` nie jest pierwszą rzeczą, która się wykonuje. Na Uniksie `std` najpierw otwiera ponownie zamknięte deskryptory standardowe na `/dev/null`, ustawia ignorowanie sygnału `SIGPIPE`, instaluje obsługę przepełnienia stosu i zapamiętuje argumenty, a po `main` zamienia panikę na kod wyjścia 101. Atrybut `#![no_main]` oddaje tę pracę programiście; na mikrokontrolerze oznacza to procedurę obsługi resetu (*reset handler*), która musi przygotować pamięć — sekcje `.data` i `.bss` — zanim jakakolwiek zmienna statyczna będzie miała sensowną wartość.

**Szukaj po polsku:** co się dzieje przed main · procedura obsługi resetu · tablica wektorów przerwań · `rust no_main reset handler` · `rust lang_start sigpipe` · `cortex-m-rt entry`
