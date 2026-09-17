# Panics in unsafe code

**Level:** 301 · deep dive

**One line:** A panic can leave any function halfway through, and in safe code that costs a wrong answer; halfway through unsafe code it can leave an invariant broken that a destructor then relies on — `set_len(n)` before the `n` elements exist becomes undefined behaviour the moment the code filling them panics.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The window: every call that can panic between the line that breaks an invariant and the line that restores it — a `clone()`, a user closure, an `Ord::cmp`, an index out of bounds, an arithmetic overflow in a debug build. How do you list them for one `unsafe` block?
- The worked failure: `Vec::with_capacity(n)`, then `set_len(n)`, then elements written through `as_mut_ptr()` by a closure that panics on the second one. Unwinding drops the `Vec`, which drops `n` elements of which one exists. Under Miri this is "reading memory … but memory is uninitialized"; what does the native build do?
- The first fix is ordering: write the elements, then `set_len`. What does [`spare_capacity_mut`](../../26_Collections/vec_methods/vec_spare_capacity_mut/README.md) do to make that order the natural one?
- The second fix is a guard: a local struct whose `Drop` restores the invariant if unwinding reaches it. std's own `alloc` code has `SetLenOnDrop` in `Vec` and `Hole` in `BinaryHeap` — what does each restore, and why does the `Drop` run on the panic path and the normal path alike? `std::mem::DropGuard` would name the pattern, and in 1.98 it is nightly-only (`drop_guard`, issue #144426).
- [`UnwindSafe` ↗](https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html) is an auto trait but not an `unsafe` one; its docs describe it as a speed bump rather than a contract, and `AssertUnwindSafe` removes it with no `unsafe` at all. What does it stop, and what does it leave entirely to you?
- `panic = "abort"` removes unwinding, and with it this whole page — for a binary. Why can a library never assume its caller chose it?
- A panic that reaches an `extern "C"` boundary aborts; the unsafe-code consequences of that belong to [Callbacks across FFI](../callbacks_across_ffi/README.md).

## The trap it exists for

The fast path written as `set_len` plus raw writes, and tested only with closures that never panic. Every test passes, and the first panicking input — a `Clone` impl that indexes past an end, a caller's closure that unwraps — turns a recoverable panic into undefined behaviour inside `Drop`.

## Where this sits

- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) covers unwinding in safe code, where memory stays tidy and work stays half-done; [Lock poisoning](../mutex_poisoning/README.md) covers how std records a broken invariant for a lock; [`Vec::set_len`](../../26_Collections/vec_methods/vec_set_len/README.md) covers that method's two conditions. This page covers only exception safety inside unsafe code.

## See also

- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — unwinding, `catch_unwind` and `panic = "abort"`
- [`Vec::set_len`](../../26_Collections/vec_methods/vec_set_len/README.md) — "initialises nothing, drops nothing, checks nothing"
- [Lock poisoning](../mutex_poisoning/README.md) — the safe-code version: record that the invariant may be broken
- [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) — the mechanism a guard is built from
- [What an invariant is](../what_an_invariant_is/README.md) — the property the window leaves broken
- [Miri](../miri/README.md) — how the failure above was observed
- [Callbacks across FFI](../callbacks_across_ffi/README.md) — where a panic meets a C frame

## If you are coming from another language

- **C++.** This page is C++'s exception-safety guarantees — basic, strong, no-throw — applied to `unsafe` blocks. `std::vector::push_back` offers the strong guarantee and relies on `noexcept` move constructors to do it; the guard struct here is the RAII "rollback on unwind" idiom C++ programmers already write. What Rust changes is the default: safe code needs none of this, so the discipline only applies where `unsafe` is written.
- **C.** No unwinding, so no panic mid-block — but `longjmp` out of a half-updated structure is the same hazard, and skipping a Rust frame's destructors with `longjmp` breaks an assumption the Reference lists under undefined behaviour.
- **Java and Python.** An exception thrown between two field writes leaves an object in a state its methods do not expect, and `finally` is the repair. The consequence stops at wrong answers, because neither runtime lets an object's memory be read as a type it does not hold — which is the part `unsafe` Rust gives up.

## Po polsku

Panika (*panic*) może przerwać każdą funkcję w połowie. W bezpiecznym kodzie kosztuje to złą odpowiedź, ale w środku bloku `unsafe` może zostawić złamany niezmiennik, na którym polega potem destruktor (`Drop`) — `set_len(n)` wywołane, zanim `n` elementów istnieje, staje się niezdefiniowanym zachowaniem, gdy tylko kod wypełniający bufor spanikuje. Są dwa lekarstwa: kolejność (najpierw zapis, potem `set_len`) i **strażnik** (*guard*) — lokalna struktura, której `Drop` przywraca niezmiennik podczas odwijania stosu. `UnwindSafe` to tylko „próg zwalniający”, nie kontrakt.

**Szukaj po polsku:** bezpieczeństwo wyjątków · odwijanie stosu · strażnik RAII · `rust exception safety unsafe` · `rust set_len panic safety` · `nomicon exception safety`
