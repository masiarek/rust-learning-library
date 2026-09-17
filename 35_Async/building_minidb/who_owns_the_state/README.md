# Who owns the state

**Level:** 201 → 301 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Every connection task needs the same store, and there are two honest designs — share it behind `Arc<Mutex<Store>>`, or give it to one actor task that owns it outright and takes requests over a channel — and a benchmark under contention should decide between them, not taste.

## What it has to cover

- **Design one: `Arc<Mutex<Store>>`.** Which mutex: `std::sync::Mutex` for short critical sections that never span an `.await`, and [`tokio::sync::Mutex` ↗](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html) only when the guard really must be held across one — Tokio's own docs make that recommendation, and the page should show why
- Where the guard may not be held: a `std::sync::MutexGuard` is not `Send`, so holding it across `.await` makes `tokio::spawn` reject the future — and the Clippy lint that names the problem earlier
- **Design two: an actor.** One task owns the `Store`; connections send a command plus a `oneshot` reply channel over `mpsc`. No lock, requests served one at a time, and the channel's bound is a queue you can see (chapter 6)
- What each design makes easy: reads in parallel, multi-key operations, adding a write-ahead log later (chapter 9)
- **The benchmark.** A [criterion ↗](https://docs.rs/criterion/latest/criterion/) run of both designs with 1, 8 and 64 concurrent clients — throughput and latency — and what the numbers cannot tell you about production

## The trap it exists for

Choosing `tokio::sync::Mutex` everywhere because the code is async. It is slower for short sections, and it hides the real question — whether any guard needs to live across an `.await` at all.

## What minidb gains

One store shared by every connection, in both designs, and a benchmark that says which one the rest of the course keeps.

## See also

- [Sharing across threads: `Arc`](../../../18_Ownership/sharing_across_threads/README.md) — `Arc<Mutex<T>>` with threads, before tasks
- [Mutex poisoning](../../../09_Advanced/mutex_poisoning/README.md) and [`RwLock` and atomics](../../../09_Advanced/rwlock_and_atomics/README.md) — the std locks' other behaviours
- [Channels](../../../09_Advanced/channels/README.md) — `mpsc` with threads
- [`black_box` is a hint](../../../33_Time_and_Benchmarking/black_box_is_a_hint/README.md) — keeping the benchmark honest
- [Actor model ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/actor_model/index.html) and [mutex ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/synchronization/mutex/index.html) — in the Concurrency library

## Po polsku

Wszystkie połączenia potrzebują tego samego magazynu i są na to dwa uczciwe sposoby: **współdzielenie pod blokadą** (`Arc<Mutex<Store>>`) albo **aktor** — jedno zadanie, które jest jedynym właścicielem danych i obsługuje żądania przychodzące kanałem. Przy blokadzie liczy się, *którą* wybrać: `std::sync::Mutex` dla krótkich sekcji krytycznych, a `tokio::sync::Mutex` tylko wtedy, gdy strażnik (*guard*) musi przetrwać `.await`. Wybór między projektami ma rozstrzygnąć pomiar pod obciążeniem, a nie gust.

**Szukaj po polsku:** współdzielony stan w tokio · wzorzec aktora · `tokio mutex vs std mutex` · `rust actor pattern tokio`
