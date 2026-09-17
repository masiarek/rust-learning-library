# Allocating without std

**Level:** 301 · deep dive

**One line:** `extern crate alloc` gives a `no_std` crate `Vec` and `String` back, and the price is the thing `std` supplied without asking — a `#[global_allocator]`, which on a device means choosing a region of RAM and an algorithm to carve it up.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `extern crate alloc;` and what happens when nothing provides the heap: *"no global memory allocator found but one is required"* (1.98.0). A `no_std` staticlib with a [`GlobalAlloc` ↗](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) impl built from `core::alloc::{GlobalAlloc, Layout}` and a `#[global_allocator]` static builds with bare `rustc -C panic=abort` — so the mechanism is checkable on the host.
- Where the bytes come from on a device: a region the linker script sets aside, or a `static` array. Who initialises the allocator with that region, and what happens to an allocation that runs before it does?
- Off-the-shelf heaps: [`embedded-alloc` ↗](https://docs.rs/embedded-alloc) 0.7.0 ships `LlffHeap` and `TlsfHeap`. What does each algorithm trade — fragmentation, worst-case time, code size?
- A target with no operating system can still arrive with a heap: the 1.98.0 rustc book names `dlmalloc` as the default global allocator of `wasm32-unknown-unknown`. What makes that possible there and not on a microcontroller?
- The counting allocator from [The global allocator](../../09_Advanced/the_global_allocator/README.md), moved onto a device: which of its measurement rules still hold when there is no `println!` to break them?
- The per-container [`Allocator` trait ↗](https://doc.rust-lang.org/std/alloc/trait.Allocator.html) is nightly-only, as [`Allocator::shrink`](../../09_Advanced/allocator_shrink/README.md) shows. What does a stable `no_std` program use instead when it wants an arena and not a global heap?
- The heap is a fixed number you picked, so an allocation beyond it is the normal case rather than the rare one — the subject of [Out of memory](../out_of_memory/README.md).

## The trap it exists for

The bump allocator written in an afternoon because `dealloc` is the hard half. It passes every test that runs once. On a device that formats a `String` on every loop iteration, each one is freed into nothing, and the heap runs out after however many iterations it holds.

## Where this sits

[The global allocator](../../09_Advanced/the_global_allocator/README.md) covers replacing `System` in an ordinary program and measuring with it; [`core`, `alloc` and `std`](../core_alloc_and_std/README.md) covers what `alloc` contains. This page covers only supplying a heap where there is none.

## See also

- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — `GlobalAlloc`, `realloc`, and why one static per program
- [`Allocator::shrink`](../../09_Advanced/allocator_shrink/README.md) — the unstable per-container API and its provided-method traps
- [Array or `Vec`?](../../26_Collections/array_or_vec/README.md) — the choice that needs no allocator at all
- [Stack and heap](../../18_Ownership/stack_and_heap/README.md) — the two regions a device's RAM is split between
- [Out of memory](../out_of_memory/README.md) — what the fixed heap turns into
- [The Embedded Rust Book: Collections ↗](https://docs.rust-embedded.org/book/collections/index.html)

## If you are coming from another language

- **C.** Safety-critical C rules often ban the heap outright — rule 3 of NASA/JPL's [Power of 10 ↗](https://en.wikipedia.org/wiki/The_Power_of_10:_Rules_for_Developing_Safety-Critical_Code) is *"do not use dynamic memory allocation after initialization"* — and in C that ban is a review checklist. Rust keeps both options visible to the compiler: leave out `alloc` and no line in the crate can allocate, or provide a heap once through `#[global_allocator]`.
- **C++.** Overriding global `operator new` is the same move as `#[global_allocator]`, and `std::pmr::memory_resource` is the per-container version — which in Rust is still nightly-only.
- **Go.** Allocation belongs to the runtime and its garbage collector, so there is no hook at this level to replace; the nearest knobs tune the collector rather than swap the heap.

## Po polsku

`extern crate alloc` przywraca w kodzie `no_std` typy `Vec`, `String` i `Box`, ale nie przywraca sterty — tę trzeba dostarczyć samemu jako statyczną wartość z atrybutem `#[global_allocator]`, implementującą `GlobalAlloc`. Na mikrokontrolerze oznacza to wybranie obszaru pamięci RAM (zwykle wskazanego w skrypcie linkera) i algorytmu, który go dzieli; gotowe implementacje daje np. crate `embedded-alloc`. Najczęstszy błąd to alokator „na szybko”, który nigdy nie zwalnia pamięci — przechodzi testy i wyczerpuje stertę dopiero na urządzeniu.

**Szukaj po polsku:** alokator na mikrokontrolerze · sterta bez systemu operacyjnego · `rust no_std global_allocator` · `embedded-alloc heap` · `rust extern crate alloc`
