# `async fn` and `.await`

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `async fn` returns an anonymous type that implements `Future`, the compiler turns its body into a state machine, and each `.await` is a point where that machine may stop and hand control back — with every local still alive stored inside the future.

## What it has to cover

- What `async fn f() -> u32` really returns: `impl Future<Output = u32>`, a type with no name you can write
- The body as a state machine: one state per `.await`, and the locals that live across an `.await` become fields of the future
- **The size of a future is measurable in std.** `size_of_val` on an `async fn`'s future grows by the size of what is held across an `.await` — a 1 KiB array kept across one makes the future over 1 KiB
- `.await` is a suspension point, not a call: the current function may stop there and resume later, possibly on another thread
- Where `.await` is allowed (inside `async` only), and why `main` cannot simply be `async` without a runtime
- `async` blocks and `async move` blocks, and what `move` means for a future that outlives the scope that built it
- Why a blocking call between two `.await`s blocks every other task on that thread — the preview of [common pitfalls](../common_async_pitfalls/README.md)

## The trap it exists for

Treating `.await` as an ordinary function call. It usually behaves like one, which is exactly why the three places it does not — a lock guard held across it, a reference that must now be `Send`, and a future dropped while suspended there — arrive as surprises.

## If you are coming from another language

- **JavaScript and Python:** the syntax transfers directly, and so does the idea of a suspension point. What changes: there is no built-in event loop, the state machine's size is visible, and the compiler checks what is held across the suspension.
- **C#:** `async`/`await` also compiles to a state machine; Rust's is a plain value that lives wherever you put it — on the stack, inside another future — until something such as a spawn or `Box::pin` moves it to the heap.
- **Go:** there is no `await` because every goroutine may block; Rust makes the suspension points explicit and costs nothing where there are none.

## See also

- [What a future is](../what_a_future_is/README.md) — the trait this syntax implements
- [Tasks](../building_minidb/tasks/README.md) — why the future you spawn has to be `Send + 'static`
- [Async functions in traits](../building_minidb/async_functions_in_traits/README.md) — the same syntax where the return type has to be named
- [Async state machine ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/async_state_machine/index.html) and [function coloring ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/function_coloring/index.html) — in the Concurrency library

## Po polsku

`async fn` nie wykonuje niczego od razu — zwraca future, a kompilator zamienia ciało funkcji w **maszynę stanów**, w której każdy `.await` jest **punktem zawieszenia**: tu funkcja może się zatrzymać i wrócić później, być może na innym wątku. Zmienne lokalne żyjące przez `.await` stają się polami tej maszyny, dlatego rozmiar future’a da się zmierzyć i rośnie razem z nimi.

**Szukaj po polsku:** async await w Ruscie · maszyna stanów · `rust async fn desugar` · `rust await suspension point`
