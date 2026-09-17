# Building minidb: a Tokio server, one production problem at a time

**Level:** 201 → 301 · the course outline

**One line:** Fifteen chapters that take `minidb` — a small in-memory key–value store — from a synchronous library to a TCP server that shares its state safely, survives cancellation, overload, shutdown and restarts, and is tested against a network that misbehaves on purpose.

**Every chapter below is a stub** — an outline and the questions the finished page has to answer. See [Adding a lesson](../../CONTRIBUTING.md#stubs), and [how these pages will be checked](../README.md#how-these-pages-will-be-checked): Tokio is a crate, so each chapter will graduate with a Cargo `demo/` workspace; nothing here is compiled yet.

## Before chapter 1

Read [What a future is](../what_a_future_is/README.md) and [`async fn` and `.await`](../async_fn_and_await/README.md) — the std half of async, which every chapter assumes. [Spawning a thread](../../09_Advanced/spawning_a_thread/README.md) and [Sharing across threads: `Arc`](../../18_Ownership/sharing_across_threads/README.md) are the thread-based versions of chapters 2 and 4, and worth knowing first.

## The chapters

| # | Chapter | The problem | What minidb gains |
|---|---|---|---|
| 1 | [The Tokio runtime](the_tokio_runtime/README.md) | A future does nothing on its own | an `async main` on a runtime |
| 2 | [Tasks](tasks/README.md) | Concurrency without a thread per job | spawned work, and blocking work kept off the workers |
| 3 | [Building the server](building_the_server/README.md) | Speaking to clients over TCP | an accept loop, a line protocol, a task per connection |
| 4 | [Who owns the state](who_owns_the_state/README.md) | Every connection needs the same store | a shared store — two ways, benchmarked |
| 5 | [Cancellation](cancellation/README.md) | Work can be dropped at any `.await` | timeouts, and reads that are safe to cancel |
| 6 | [Backpressure](backpressure/README.md) | Unbounded queues end in out-of-memory | bounded queues and a connection limit |
| 7 | [Shutdown and supervision](shutdown_and_supervision/README.md) | A deploy kills requests in flight | a drain in the right order, and a policy for failed tasks |
| 8 | [Testing async code](testing_async_code/README.md) | Time, sockets and scheduling make tests slow and flaky | tests on a paused clock and in-memory streams; `tracing` spans |
| 9 | [Durability across restarts](durability_across_restarts/README.md) | A restart loses everything | a write-ahead log with group commit and replay |
| 10 | [`AsyncRead` and `AsyncWrite`](async_read_and_async_write/README.md) | Adding behaviour to every byte | a byte-counting IO wrapper |
| 11 | [Codecs and framing](codecs_and_framing/README.md) | Lines cannot carry every value | a length-prefixed codec, shared with the log |
| 12 | [Streams, sinks, and pipelining](streams_sinks_and_pipelining/README.md) | One request at a time per connection | several requests in flight, answered in order |
| 13 | [Async functions in traits](async_functions_in_traits/README.md) | Durability is hard-coded | a pluggable backend behind an async trait |
| 14 | [Diagnosing a stuck runtime](diagnosing_a_stuck_runtime/README.md) | A blocked worker says nothing | `tokio-console`, metrics, and safe sync–async bridges |
| 15 | [Testing against a hostile network](testing_against_a_hostile_network/README.md) | Real networks drop, delay and partition | deterministic network-failure tests with `turmoil` |

## Why this order

Chapters 1–4 build a server that works when nothing goes wrong. Chapters 5–7 are the three ways a running server is told to stop doing something — a dropped future, a full queue, a shutdown signal — and chapter 8 makes all of that testable before chapter 9 adds state that must survive a crash. Chapters 10–12 go underneath the IO the earlier chapters used as given, and chapter 11 is where the cancel-safety question from chapter 5 gets its structural answer. Chapter 13 makes durability pluggable, which is what lets chapter 15 swap the real network for a simulated one; chapter 14 sits between them because a simulated network is no help with a worker thread that is stuck.

## One project per chapter

Each chapter's minidb should be a complete Cargo project of its own — the lesson's `demo/` workspace — frozen once written — the same rule as [the long way round](../../ROADMAP.md): the diff between chapter *n* and chapter *n + 1* is the lesson, and a shared crate would mean editing chapter 3 to break chapter 9's page.

The crates the course needs, all by chapter 15: [`tokio` ↗](https://docs.rs/tokio/latest/tokio/), `tokio-util` (codecs, `CancellationToken`, `TaskTracker`), `futures` (`Stream`, `Sink`), [`criterion` ↗](https://docs.rs/criterion/latest/criterion/), [`tracing` ↗](https://docs.rs/tracing/latest/tracing/), [`tokio-console` ↗](https://github.com/tokio-rs/console) and [`turmoil` ↗](https://docs.rs/turmoil/latest/turmoil/).

## See also

- [Common async pitfalls](../common_async_pitfalls/README.md) — each pitfall, and the chapter that fixes it
- [Tokio's own tutorial ↗](https://tokio.rs/tokio/tutorial) — builds a Redis clone, which overlaps chapters 1–4 and makes a good second explanation
- [Concurrency learning library ↗](https://masiarek.github.io/concurrency-learning-library/) — the concepts under chapters 4–7 without Tokio: [actor model ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/actor_model/index.html), [supervision ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/supervision/index.html), [structured concurrency ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/structured_concurrency/index.html)

## Po polsku

`minidb` to mały magazyn klucz–wartość trzymany w pamięci, który w piętnastu rozdziałach staje się serwerem TCP na Tokio. Pierwsze cztery rozdziały budują serwer działający, gdy nic się nie psuje; następne trzy dotyczą sytuacji, w których serwer musi przerwać pracę — **anulowania** (*cancellation*), **przeciążenia** i **ciśnienia zwrotnego** (*backpressure*) oraz **łagodnego zamykania** (*graceful shutdown*); dalej są testy, trwałość danych przez **dziennik zapisu z wyprzedzeniem** (*write-ahead log*), warstwa wejścia-wyjścia pod spodem i na końcu testy z symulowaną, celowo zawodną siecią.

**Szukaj po polsku:** serwer TCP w Ruscie · baza klucz–wartość · `tokio tcp server tutorial` · `rust mini redis`
