# Common async pitfalls

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Most async bugs compile — a future nobody awaits, a worker thread stuck in a blocking call, a lock held across a suspension point — so each one needs a name and a symptom you can recognise.

## What it has to cover

Each pitfall with its symptom, and the chapter where the fix is built:

| Pitfall | What you see | Fixed in |
|---|---|---|
| A future created and never awaited | nothing happens; one warning | [What a future is](../what_a_future_is/README.md) |
| A blocking call (`std::thread::sleep`, file IO, a long computation) inside a task | every other task on that worker stalls | [Tasks](../building_minidb/tasks/README.md), [Diagnosing a stuck runtime](../building_minidb/diagnosing_a_stuck_runtime/README.md) |
| A `std::sync::MutexGuard` held across `.await` | `tokio::spawn` refuses the future as not `Send`; unspawned, it can deadlock | [Who owns the state](../building_minidb/who_owns_the_state/README.md) |
| Borrowing a local into a spawned task | the `'static` error | [Tasks](../building_minidb/tasks/README.md) |
| A task spawned and never joined | its panic reaches stderr and nothing else — no error, no restart | [Shutdown and supervision](../building_minidb/shutdown_and_supervision/README.md) |
| A future dropped mid-way inside `select!` | data read but never processed | [Cancellation](../building_minidb/cancellation/README.md) |
| An unbounded channel | memory grows until the process dies | [Backpressure](../building_minidb/backpressure/README.md) |
| `block_on` called from inside the runtime | a panic, at run time | [Diagnosing a stuck runtime](../building_minidb/diagnosing_a_stuck_runtime/README.md) |
| A test that depends on scheduling order | passes locally, flakes in CI | [Testing async code](../building_minidb/testing_async_code/README.md) |

- For each row: the smallest program that shows it, and whether the compiler, Clippy, or nothing at all catches it
- Why most of these are silent: the compiler checks types across `.await`, not timing

## See also

- [`async fn` and `.await`](../async_fn_and_await/README.md) — the suspension point most of these are about
- [Blocking the event loop ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/blocking_the_event_loop/index.html) and [task leak ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/hazards/task_leak/index.html) — two of these rows, across languages
- [Forgotten unlock](../../31_C_and_Cpp/forgotten_unlock/README.md) — the C version of the lock row
- [A `#[retry]` attribute](../../37_Procedural_Macros/a_retry_attribute/README.md) — a macro that sleeps between attempts, and the error it gives an `async fn` rather than block inside one

## Po polsku

Najgroźniejsze błędy asynchroniczne w Ruscie **kompilują się bez ostrzeżeń**: blokujące wywołanie w zadaniu zatrzymuje wszystkie inne zadania na tym samym wątku roboczym, blokada ze `std::sync` trzymana przez `.await` kończy się zakleszczeniem albo błędem `Send`, a panika w zadaniu, na które nikt nie czeka, znika bez śladu. Ta strona zbiera je w jednej tabeli i odsyła do rozdziału, który każdy z nich naprawia.

**Szukaj po polsku:** pułapki programowania asynchronicznego · zakleszczenie w async · `rust async pitfalls` · `tokio blocking in async`
