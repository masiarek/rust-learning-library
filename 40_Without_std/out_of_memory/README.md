# Out of memory

**Level:** 301 · deep dive

**One line:** `Vec::push` has no way to return an error, so when the allocator says no, Rust aborts the process — no unwinding, no destructors, no `catch_unwind` — and the only stable way to get a `Result` instead is to ask first with `try_reserve`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The path: an infallible allocation fails, and the collection calls [`handle_alloc_error` ↗](https://doc.rust-lang.org/std/alloc/fn.handle_alloc_error.html) (stable 1.28.0), which never returns. With `std` linked, its documented default prints a message to stderr and aborts. Measured on 1.98.0, x86_64 macOS, with `Vec::<u8>::with_capacity(isize::MAX as usize)`: `memory allocation of 9223372036854775807 bytes failed`, a `RUST_BACKTRACE` note, and exit status 134 from the shell.
- Without `std` the default is different: 1.98.0's `alloc` source documents that when every crate is `#![no_std]`, `handle_alloc_error` calls `panic!` — so on a device an allocation failure arrives at [the panic handler](../the_panic_handler/README.md) looking like any other panic.
- Asking first: [`Vec::try_reserve` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve) and `try_reserve_exact` (stable 1.57.0) return `Result<(), TryReserveError>`, and the error tells two failures apart — measured on 1.98.0: `isize::MAX` bytes gave *"the memory allocator returned an error"*, `usize::MAX` gave *"the computed capacity exceeded the collection's maximum"*.
- What is still nightly on 1.98.0: [`Vec::try_with_capacity` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_with_capacity) (`try_with_capacity`), `Box::try_new` (`allocator_api`), and [`set_alloc_error_hook` ↗](https://doc.rust-lang.org/std/alloc/fn.set_alloc_error_hook.html) (`alloc_error_hook`). What does a stable program do for a `Box` or a `String` it must not abort on?
- `alloc` has a `no_global_oom_handling` cfg that removes the infallible allocating methods altogether. Who builds `alloc` that way, and what does their code look like without `push`?
- Why this can be rarer than it sounds on a desktop: with overcommit, does a large allocation fail at all, or does the process die later at the hands of the operating system, where no Rust API can see it?
- On a device with a fixed heap, running out is a normal event rather than a catastrophe — fixed-capacity collections, or a `try_reserve` before every growth, and a count of every other allocation on the path.

## The trap it exists for

Writing `v.try_reserve(n)?` once and treating the function as allocation-safe. The reservation covers that one vector up to `n`; the `format!`, the `to_string`, the `collect` and the next `push` past `n` are all still infallible, and any of them still aborts.

## Where this sits

[What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) covers the failure you can catch and unwind from; this page covers the one you cannot. [Allocating without std](../allocating_without_std/README.md) is where the fixed heap that makes this common comes from.

## See also

- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — unwinding, `catch_unwind`, and why an abort skips all of it
- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — the allocator whose null pointer starts this chain
- [`Allocator::shrink`](../../09_Advanced/allocator_shrink/README.md) — another path where `Vec` has no way to report failure, and aborts
- [`Vec`](../../26_Collections/the_vec/README.md) — capacity, and which operations reach the allocator
- [Keep going, or stop](../../02_Errors/keep_going_or_stop/README.md) — the design decision a fallible allocation forces on the caller
- [`TryReserveError` ↗](https://doc.rust-lang.org/std/collections/struct.TryReserveError.html)

## If you are coming from another language

- **C.** `malloc` returns `NULL` and every caller is supposed to check. Rust inverts the default: the common API cannot fail visibly and aborts instead of handing back a null that someone forgot to test, and the fallible API (`try_reserve`) is the one you opt into.
- **C++.** `operator new` throws `std::bad_alloc`, which can in principle be caught and recovered from; `new (std::nothrow)` returns null instead. Rust has no catchable equivalent: under `std` an allocation failure is an abort, not an unwinding panic.
- **Python.** `MemoryError` is an exception you can catch, though by the time it is raised the interpreter may not have the memory to do much about it. Rust's `TryReserveError` is the explicit, checked version, available only where you asked for it.
- **Java.** `OutOfMemoryError` is an `Error`, catchable but not meant to be caught. Rust removes the temptation: the infallible path cannot be caught at all.

## Po polsku

Gdy alokator odmawia pamięci, `Vec::push` nie ma jak zwrócić błędu, więc Rust przerywa proces (*abort*) — bez odwijania stosu i bez uruchamiania destruktorów, czego `catch_unwind` nie złapie. Na Macu z Rustem 1.98.0 wygląda to tak: komunikat `memory allocation of … bytes failed` i kod wyjścia 134. Jedyna stabilna droga do `Result` to zapytać wcześniej przez `try_reserve`; bez `std` domyślna obsługa wywołuje natomiast `panic!`, więc błąd trafia do własnego *panic handlera*.

**Szukaj po polsku:** brak pamięci w Ruscie · obsługa błędu alokacji · `rust handle_alloc_error abort` · `rust try_reserve TryReserveError` · `rust memory allocation of bytes failed`
