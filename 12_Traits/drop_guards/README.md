# Drop guards

**Level:** 201 · working knowledge

**One line:** A drop guard is a value whose only job is its `Drop` impl — cleanup that runs on every way out of a scope, including `?`, early `return` and a panic — and it stops working the moment you bind it to `_`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The shape: a struct holding what needs undoing, an `impl Drop` that undoes it, created at the top of the scope
- std's own guards — `MutexGuard`, `RefMut`, `vec::Drain` — read as examples of the pattern rather than as special cases
- `let _guard = …` versus `let _ = …`: the second drops immediately, so the cleanup runs before the work (already shown on [`Drop`, and what RAII buys](../drop_and_raii/README.md))
- Disarming a guard on the success path: a `bool` field, or `mem::forget` — and why a guard must stay *correct* if it is forgotten, since forgetting is safe
- The `scopeguard` crate's `defer!` and `guard()`, and when a hand-written struct reads better
- Guards in unsafe code: restoring an invariant if a panic unwinds mid-update — see [panics in unsafe code](../../09_Advanced/panics_in_unsafe_code/README.md)

## The trap it exists for

`let _ = lock.lock().unwrap();` — the `_` pattern binds nothing, so the guard is dropped at the semicolon and the code below runs unlocked. `let _guard` is a real binding that lives to the end of the scope. rustc refuses the `Mutex` case outright: `let_underscore_lock` is a deny-by-default lint on 1.98.0.

## Where this sits

[`Drop`, and what RAII buys](../drop_and_raii/README.md) is the trait and the drop order. This page is the pattern built on it. [Forgotten unlock](../../31_C_and_Cpp/forgotten_unlock/README.md) is the C bug the pattern removes.

## See also

- [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) — the trait, the order, and `let _ =`
- [Forgotten unlock](../../31_C_and_Cpp/forgotten_unlock/README.md) — the C bug a guard makes impossible
- [The wildcard `_`](../../30_Pattern_Matching/the_wildcard/README.md) — why `_` binds nothing and so drops at once
- [Lock poisoning](../../09_Advanced/mutex_poisoning/README.md) — the guard that also records a panic
- [Panics in unsafe code](../../09_Advanced/panics_in_unsafe_code/README.md) — guards that restore an invariant
- [Destructors that can fail](../../39_API_Design/destructors_that_can_fail/README.md) — what a guard cannot report

## If you are coming from another language

- **Go.** `defer mu.Unlock()` is the same guarantee written as a statement; the Go library's [a mutex guards a counter ↗](https://masiarek.github.io/go-learning-library/04_Sync/a_mutex_guards_a_counter/) shows it. Rust ties the cleanup to a value instead, so it moves and returns with that value.
- **Python.** A `with` block's `__exit__` is a drop guard with visible edges; Rust's version has no indentation, and ends where the binding's scope ends.
- **C++.** RAII is C++'s idea — `std::lock_guard`, `std::unique_ptr`. What changes in Rust is that the guard cannot be used after it is moved away, which C++ only warns about.
- **C.** No destructors, so the pattern is `goto cleanup` — see [forgotten unlock](../../31_C_and_Cpp/forgotten_unlock/README.md).

## Po polsku

Strażnik (*drop guard*) to wartość, której jedynym zadaniem jest implementacja `Drop`: sprzątanie wykonuje się przy każdym wyjściu z zakresu, także przez `?`, wczesny `return` i `panic!`. Najczęstszy błąd to `let _ = …` — podkreślnik niczego nie wiąże, więc strażnik ginie od razu i reszta kodu działa bez blokady; poprawnie jest `let _guard = …`.

**Szukaj po polsku:** strażnik Drop · RAII w Ruście · `rust drop guard pattern` · `rust let underscore drops immediately`
