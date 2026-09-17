# The panic handler

**Level:** 301 · deep dive

**One line:** In `std` a panic prints a message, unwinds and exits with 101 because somebody wrote that code; in `#![no_std]` nobody did, so the program must supply exactly one `#[panic_handler]` — a function that takes `&PanicInfo` and never returns.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The refusal: a `#![no_std]` binary with no handler stops at ``error: `#[panic_handler]` function required, but not found`` (1.98.0). The shape it wants is `fn(&core::panic::PanicInfo) -> !`, and [the never type](../../15_First_Programs/the_never_type/README.md) is what forbids returning.
- The second refusal, the one met first on a laptop: a `no_std` staticlib built for the host with the default strategy fails with *"unwinding panics are not supported without std"*. `-C panic=abort` — `panic = "abort"` in a [Cargo profile ↗](https://doc.rust-lang.org/cargo/reference/profiles.html#panic) — makes it build. Bare-metal targets already default to it: `rustc --print cfg --target thumbv7em-none-eabihf` prints `panic="abort"`.
- What std's handler did that you now choose: the `thread 'main' panicked at` line, the backtrace note, unwinding through destructors, exit code 101. Which of those can a chip with no stderr and no parent process do at all?
- The usual handlers and what each costs during bring-up: loop forever, reset the chip, write the message to a debug probe, halt for a debugger. The [`panic-halt` ↗](https://docs.rs/panic-halt) crate is the one-line version of the first.
- What [`PanicInfo` ↗](https://doc.rust-lang.org/core/panic/struct.PanicInfo.html) carries — `location()` (stable 1.10.0) and `message()` (stable 1.81.0) — and whether formatting that message pulls in `core::fmt` code a small flash chip cannot spare.
- Exactly one per program: a crate that defines a handler next to a dependency that also defines one fails with `` error[E0152]: found duplicate lang item `panic_impl` `` (1.98.0). So which crate should own it — why does a `no_std` *library* leave it to the binary, the same program-wide rule as `#[global_allocator]`?
- Out-of-memory in `no_std` routes here too, since `alloc`'s default `handle_alloc_error` calls `panic!` when nothing links `std` — see [Out of memory](../out_of_memory/README.md).

## The trap it exists for

Writing `loop {}` because it is the shortest function that returns `!`. It is a correct handler, and it turns every `unwrap` during bring-up into a board that silently stops — no message, no line number, nothing on any wire to say which call failed.

## Where this sits

[What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) covers the unwinding panic, `catch_unwind` and exit code 101 under `std`; this page covers only what replaces all of that when `std` is gone.

## See also

- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — the `panic = "abort"` bullet this page starts from
- [Reading a backtrace](../../17_Option_and_Result/reading_a_backtrace/README.md) — the output a device handler gives up
- [The never type `!`](../../15_First_Programs/the_never_type/README.md) — why the handler's return type is not `()`
- [`core`, `alloc` and `std`](../core_alloc_and_std/README.md) — why the handler is missing in the first place
- [What runs before `main`](../before_main_runs/README.md) — the other half of the runtime you now write
- [GLOSSARY.md](../../GLOSSARY.md) — *panic* and *unwinding*, as the rest of the library uses them
- [The Embedded Rust Book: Panicking ↗](https://docs.rust-embedded.org/book/start/panicking.html)
- [The Rust Reference: the `panic_handler` attribute ↗](https://doc.rust-lang.org/reference/runtime.html#the-panic_handler-attribute)

## If you are coming from another language

- **C.** C has no panic, so the closest thing is a failed `assert` calling `abort()` — and on bare metal, what `abort` does is whatever the C library you linked decided. Rust turns that decision into a required function with a checked signature, and refuses to build a binary that has not made it.
- **C++.** An uncaught exception ends in `std::terminate`, and `-fno-exceptions` removes exceptions from a build altogether. `panic = "abort"` is the Rust version of that trade: no unwinding, so no destructors run on the way out.
- **Go.** A panic nobody recovers ends the whole program with a stack trace, and the runtime owns how that looks — [A panic ends the whole program ↗](https://masiarek.github.io/go-learning-library/01_Goroutines/a_panic_ends_the_whole_program/). Without `std`, Rust hands you that last step as your own function.

## Po polsku

W zwykłym programie panika wypisuje komunikat, odwija stos i kończy proces kodem 101 — ale tylko dlatego, że ktoś to napisał w `std`. Bez biblioteki standardowej trzeba samemu dostarczyć dokładnie jedną funkcję z atrybutem `#[panic_handler]`, która przyjmuje `&PanicInfo` i nigdy nie wraca (typ `!`). Na mikrokontrolerze nie ma gdzie wypisać komunikatu ani procesu, który można zakończyć, więc wybór jest prawdziwą decyzją projektową: pętla bez końca, reset układu albo wysłanie komunikatu przez sondę debugującą.

**Szukaj po polsku:** obsługa paniki bez std · panika na mikrokontrolerze · `rust panic_handler no_std` · `rust unwinding panics are not supported without std` · `rust duplicate lang item panic_impl`
