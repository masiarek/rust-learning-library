# Callbacks across FFI

**Level:** 301 · deep dive

**One line:** C takes a callback as a bare function pointer plus a `void *` for your data, so a Rust closure crosses in two halves — an `extern "C" fn` trampoline and a pointer to the closure — and a panic inside it aborts the process unless the function is declared `extern "C-unwind"`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The C shape: `void each(int (*cb)(void *user, int value), void *user)`. `qsort` has no user pointer and `qsort_r` does — and the macOS man page puts `thunk` before the comparator and as the comparator's first argument, where glibc puts it last in both. What does a Rust declaration for each look like?
- Why a closure cannot be the callback: only a closure that captures nothing coerces to a `fn` pointer ([Function pointers](../../23_Closures/function_pointers/README.md) shows the `E0308`), and a capturing closure is a struct with nowhere to go in a bare pointer.
- The trampoline: a generic `extern "C" fn trampoline<F: FnMut(c_int)>(user: *mut c_void, value: c_int)` that casts `user` back to `&mut F` and calls it, monomorphised once per closure type. What must its `// SAFETY:` comment argue — that the pointer is live, of type `F`, and not aliased for the duration of the call?
- How long the user data lives: C may keep the pointer after `register` returns. `Box::into_raw` on the closure, and an `unregister` that takes it back with `Box::from_raw` — the ownership rules from [Data across the boundary](../../31_C_and_Cpp/migrating_c_to_rust/data_across_the_boundary/README.md).
- Panics: a panic that reaches the end of an `extern "C"` function aborts — "panic in a function that cannot unwind", then "thread caused non-unwinding panic. aborting." — the behaviour Rust 1.81.0 made standard. `extern "C-unwind"`, stable since 1.71.0, lets the panic unwind through the boundary instead; when is unwinding through C frames acceptable, and when is `catch_unwind` inside the trampoline the only sound answer?
- rustc's allow-by-default `ffi_unwind_calls` lint flags calls to foreign functions or function pointers with an unwinding ABI — which crates would want it on?
- Threads: C may invoke the callback from a thread it created. Which bounds — `Send`, `Sync`, `'static` — does that put on `F`, and what does `thread::spawn`'s signature already say about the same question?

## The trap it exists for

`&mut closure as *mut _ as *mut c_void` handed to a C library that stores it, with the closure living on the stack of a function that returns. The callback fires later, through a pointer to a stack frame that no longer exists — and no borrow checker saw it, because the reference became a raw pointer at the boundary.

## Where this sits

- [Function pointers](../../23_Closures/function_pointers/README.md) covers `fn` against closures; [Calling C](../calling_c/README.md) covers calling out; [Catching a signal](../catching_a_signal/README.md) covers the one callback restricted to async-signal-safe work; [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) covers `catch_unwind`. This page covers only the callback-plus-user-data pattern and unwinding across it.

## See also

- [Function pointers](../../23_Closures/function_pointers/README.md) — "nothing can ride along" in a bare `fn`
- [What a closure is](../../23_Closures/what_a_closure_is/README.md) — the struct the trampoline points at
- [Calling C](../calling_c/README.md) — the other direction, and `#[unsafe(no_mangle)]` for exporting
- [Catching a signal](../catching_a_signal/README.md) — an `extern "C" fn` handler, with stricter rules still
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — `catch_unwind`, which "exists so a panic does not cross an FFI boundary into C"
- [Data across the boundary](../../31_C_and_Cpp/migrating_c_to_rust/data_across_the_boundary/README.md) — the boxed closure C holds between calls
- [FFI-safe types](../../31_C_and_Cpp/migrating_c_to_rust/ffi_safe_types/README.md) — `Option<extern "C" fn(…)>` as a nullable callback
- [Panics in unsafe code](../panics_in_unsafe_code/README.md) — what a half-finished callback leaves behind

## If you are coming from another language

- **C.** The `void *user` argument is C's closure: `pthread_create` takes one for the thread's start function, `qsort_r` for the comparator. Rust's trampoline is the function C programmers write by hand, with the cast back to the right type generated per closure rather than written once per call site.
- **C++.** A lambda with no captures converts to a function pointer and a capturing one does not — the same rule as Rust's — and `std::function` cannot cross a C boundary for the same reason a `Box<dyn FnMut>` cannot. An exception escaping into C frames is the same hazard as a panic, which is why a C++ callback registered with a C library catches everything before it returns.
- **Python.** `ctypes.CFUNCTYPE` wraps a Python function as a C function pointer, and the [`ctypes` docs ↗](https://docs.python.org/3/library/ctypes.html#callback-functions) warn to keep a reference to the callback object for as long as C uses it — the garbage collector will otherwise free it while C still holds the pointer, which is this page's trap in Python.
- **Go.** cgo cannot pass a Go func to C; the pattern is an `//export`ed Go function as the trampoline and a `runtime/cgo.Handle` (Go 1.17) as the `void *` — an integer that stands for the Go value, deleted with `h.Delete()` when C is done, which is `Box::from_raw` by another name.

## Po polsku

C przyjmuje wywołanie zwrotne (*callback*) jako goły wskaźnik do funkcji plus `void *` na dane użytkownika, więc domknięcie Rusta przechodzi przez granicę w dwóch częściach: **trampolina** `extern "C" fn`, która rzutuje wskaźnik z powrotem na domknięcie, i sam wskaźnik do domknięcia. Trzeba pilnować, jak długo te dane żyją — C może trzymać wskaźnik dłużej niż trwa wywołanie. Panika wewnątrz `extern "C" fn` od Rusta 1.81 przerywa proces (*abort*); `extern "C-unwind"` pozwala odwinąć stos przez granicę, a `catch_unwind` w trampolinie w ogóle do tego nie dopuszcza.

**Szukaj po polsku:** wywołania zwrotne przez FFI · trampolina · odwijanie stosu przez granicę języków · `rust ffi callback closure void pointer trampoline` · `rust extern C-unwind panic abort`
