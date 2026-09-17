# Tasks

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** [`tokio::spawn` ↗](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html) hands a future to the runtime as a task — many tasks share a few worker threads and take turns only at `.await` — so a spawned future must be `Send + 'static`, and work that never reaches an `.await` belongs on `spawn_blocking` instead.

## What it has to cover

- A task against an OS thread: what each costs to create, how many you can have, and who decides when each one runs
- `JoinHandle`: awaiting it for the result, and the `JoinError` you get when the task panicked
- **Why `Send + 'static`.** A task may move between worker threads at any `.await`, and may outlive the function that spawned it — so it cannot borrow a local, and everything it holds across an `.await` must be `Send`. The fix is usually `move` plus an `Arc` clone
- [`join!` ↗](https://docs.rs/tokio/latest/tokio/macro.join.html) against `spawn`: both run futures concurrently, only `spawn` lets them run in parallel on a multi-thread runtime
- [`JoinSet` ↗](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html) for a number of tasks known only at run time, with results in completion order
- [`spawn_blocking` ↗](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html): the separate thread pool, its limit, and why a blocking task cannot be aborted once it has started
- Cooperative scheduling: a task gives up its worker only at an `.await`, and Tokio's per-task budget makes some awaits yield even when they could continue

## The trap it exists for

A CPU-heavy loop or a blocking call inside a task. It has no `.await`, so it never yields; with four workers, four such requests stop the whole server from answering, and nothing reports an error.

## What minidb gains

Store operations run as spawned tasks, and the one slow operation — loading a large snapshot — moves to `spawn_blocking`.

## See also

- [Spawning a thread](../../../09_Advanced/spawning_a_thread/README.md) — `std::thread::spawn`, which has the same `'static` bound for the same reason
- [`Send` and `Sync`](../../../09_Advanced/send_and_sync/README.md) — what the `Send` half of the bound means
- [Async task ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/units_of_execution/async_task/index.html), [cooperative scheduling ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/scheduling/cooperative_scheduling/index.html) and [work stealing ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/scheduling/work_stealing/index.html) — in the Concurrency library

## Po polsku

`tokio::spawn` zamienia future w **zadanie** (*task*) — jednostkę, którą planista wykonuje na jednym z niewielu wątków roboczych. Zadanie to nie wątek: oddaje wątek innym tylko w miejscu `.await`, więc pętla licząca coś długo albo blokujące wywołanie zatrzymuje wszystkich. Stąd wymaganie `Send + 'static` (zadanie może przejść na inny wątek i przeżyć funkcję, która je utworzyła) oraz `spawn_blocking` dla pracy, która nie ma gdzie oddać sterowania.

**Szukaj po polsku:** zadania w tokio · wątek a zadanie asynchroniczne · `tokio spawn send static` · `tokio spawn_blocking vs block_in_place`
