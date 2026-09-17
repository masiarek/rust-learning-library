# `core`, `alloc` and `std`

**Level:** 201 · working knowledge

**One line:** `std` is `core` plus `alloc` plus an operating system — `#![no_std]` links `core` alone, `alloc` comes back only when you ask for it and supply a heap, and everything that needs a kernel stays behind.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The three layers as crates you can name: [`core` ↗](https://doc.rust-lang.org/core/) needs nothing, [`alloc` ↗](https://doc.rust-lang.org/alloc/) needs a global allocator, `std` needs an operating system. `std::option` is `core::option` re-exported (`pub use core::option;` in 1.98.0's `std/src/lib.rs`), so the `Option` in a `no_std` crate is the same type, not a copy.
- What `#![no_std]` links: `core` and nothing else. `alloc` returns with `extern crate alloc;`, and a crate that then allocates without an allocator stops at *"no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait"* (checked on 1.98.0).
- What disappears, each with the thing it needed: threads and `Mutex` (a scheduler), files and `println!` (file descriptors), `Instant` (a clock), `env::args` (a loader that passed arguments), and `HashMap` — in `std` rather than `alloc` because `RandomState::new` asks the operating system for its keys.
- What `alloc` gives back: `Box`, `Vec`, `String`, `Rc`, `Arc`, and `alloc::collections` — `BTreeMap`, `VecDeque`, `BinaryHeap`, `LinkedList`, and no hash map.
- A `no_std` library still works from an ordinary program. How does a crate offer both, with `#![cfg_attr(not(feature = "std"), no_std)]` and a default `std` feature — and what does a user's `default-features = false` turn off?
- Arrays and fixed-capacity containers such as [`heapless` ↗](https://docs.rs/heapless) as the answer when there is no allocator at all.
- How to prove a crate really is `no_std`: build it for a target with no `std` in it. Why is that the only build that catches a dependency that links `std` back in?

## The trap it exists for

Adding `#![no_std]`, watching the crate compile on a laptop, and calling it done. On a host target `std` is always there to be found, so one dependency that was never `no_std` goes unnoticed until the first build for a board — where it arrives as a crate that cannot be found, not as a line in your own code.

## Where this sits

[The panic handler](../the_panic_handler/README.md) and [What runs before `main`](../before_main_runs/README.md) are what a `no_std` *binary* has to add; [Allocating without std](../allocating_without_std/README.md) is how `alloc` gets its heap. This page is only the map of what lives in which layer.

## See also

- [Array or `Vec`?](../../26_Collections/array_or_vec/README.md) — "needs an allocator: no", the row this whole section leans on
- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — the static `alloc` is waiting for
- [Stack and heap](../../18_Ownership/stack_and_heap/README.md) — what is left when the heap is gone
- [Targets and triples](../../20_Compilers/targets_and_triples/README.md) — targets that ship without a `std`
- [Compiling to `wasm32`](../../42_WebAssembly/compiling_to_wasm32/README.md) — `wasm32v1-none`, a target with `core` and `alloc` only
- [Books](../../10_Resources/books/README.md) — *Rust for Rustaceans*, whose chapter 12 this section follows
- [The Embedded Rust Book: a `no_std` Rust environment ↗](https://docs.rust-embedded.org/book/intro/no-std.html)

## If you are coming from another language

- **C.** A *freestanding* C implementation is the same idea: the standard guarantees only a short list of headers such as `<stddef.h>` and `<stdint.h>`, and `<stdio.h>` is not among them ([cppreference ↗](https://en.cppreference.com/w/c/language/conformance.html)). Two things change. Rust's `core` is far larger than those headers — iterators, formatting, slices, `Option`, `Result` all survive. And the boundary is enforced: `-ffreestanding` on its own still compiles and links a call to `printf` and `malloc` against the host libc (checked with Apple clang 21), where a `no_std` crate that calls `println!` does not compile.
- **Python.** CPython always brings its runtime; the nearest counterpart is [MicroPython ↗](https://micropython.org/), which is a separate, smaller implementation with a subset of the standard library rather than the same language with a layer removed. In Rust it is the same compiler and the same `core` on both sides of the line.
- **Go.** Go's runtime — the scheduler, the garbage collector — is part of every program, so running without an operating system means a different compiler such as [TinyGo ↗](https://tinygo.org/). Rust has no runtime of that size to remove, which is why `no_std` is an attribute rather than a separate compiler.

## Po polsku

`std` w Ruscie to trzy warstwy, a nie jedna biblioteka: `core` nie potrzebuje niczego, `alloc` potrzebuje sterty (alokatora), a dopiero `std` potrzebuje systemu operacyjnego. Atrybut `#![no_std]` zostawia samo `core`; `Vec` i `String` wracają po dopisaniu `extern crate alloc;`, ale tylko jeśli ktoś dostarczy alokator. `HashMap` zostaje po stronie `std`, bo jego domyślny `RandomState` prosi system o losowe klucze — to najlepszy przykład, że granica przebiega według tego, **czego typ potrzebuje od świata**, a nie według tego, jak zaawansowany jest.

**Szukaj po polsku:** biblioteka standardowa Rusta · biblioteka core · alokacja bez systemu operacyjnego · `rust core vs alloc vs std` · `rust no_std hashmap` · `cfg_attr no_std feature std`
