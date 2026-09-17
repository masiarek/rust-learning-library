# Async

**One line:** `async` turns a function into a value that does nothing until something polls it — std gives you that value and the syntax, and a runtime such as Tokio supplies everything that actually runs it.

The section has two parts. **The basics** are what the language and std define on their own: the `Future` trait, what `async fn` and `.await` compile to, and the mistakes people make in the first week. **Building minidb** is a fifteen-chapter course that turns a small in-memory store into a TCP server on Tokio, one production problem at a time — sharing state, cancellation, backpressure, shutdown, durability, framing, and testing all of it.

**These pages are stubs** — outlines with the questions each finished page has to answer, and no runnable example behind them yet. See [Adding a lesson](../CONTRIBUTING.md#stubs) for what that means and how a page graduates.

## The basics

| Lesson | Level | What it will teach |
|---|---|---|
| [What a future is](what_a_future_is/README.md) | 201 | One trait, one method, and nothing happens until somebody calls it |
| [`async fn` and `.await`](async_fn_and_await/README.md) | 201 | A function that returns a state machine, and the suspension point where its locals are stored |
| [Common async pitfalls](common_async_pitfalls/README.md) | 201 | The mistakes that compile — each one pointing at the chapter that fixes it |

## Building minidb

| Chapter | Level | What it will teach |
|---|---|---|
| [The course outline](building_minidb/README.md) | — | What minidb is, and the order the fifteen chapters build it in |
| [1. The Tokio runtime](building_minidb/the_tokio_runtime/README.md) | 201 | What Tokio adds to the language, and enough `Future`, `poll` and `Pin` to reason about it |
| [2. Tasks](building_minidb/tasks/README.md) | 201 | `tokio::spawn`, `Send + 'static`, `join!`, `JoinSet` and `spawn_blocking` |
| [3. Building the server](building_minidb/building_the_server/README.md) | 201 | An accept loop, a line protocol, one task per connection |
| [4. Who owns the state](building_minidb/who_owns_the_state/README.md) | 201 → 301 | `Arc<Mutex<Store>>` against an actor task, measured under contention |
| [5. Cancellation](building_minidb/cancellation/README.md) | 301 | Dropping a future, `timeout`, `select!`, `CancellationToken`, and cancel safety |
| [6. Backpressure](building_minidb/backpressure/README.md) | 301 | Bounded channels, wait-refuse-or-drop, and where a `Semaphore` permit is taken |
| [7. Shutdown and supervision](building_minidb/shutdown_and_supervision/README.md) | 301 | Draining connections before the store, and what to do when a task dies on its own |
| [8. Testing async code](building_minidb/testing_async_code/README.md) | 301 | A paused clock, an in-memory socket, and spans asserted as data |
| [9. Durability across restarts](building_minidb/durability_across_restarts/README.md) | 301 | A write-ahead log, group commit, and replay on startup |
| [10. `AsyncRead` and `AsyncWrite`](building_minidb/async_read_and_async_write/README.md) | 301 | The poll-level IO contract, `ReadBuf`, and a byte-counting wrapper |
| [11. Codecs and framing](building_minidb/codecs_and_framing/README.md) | 301 | A length-prefixed frame with `Decoder`, `Encoder` and `Framed` |
| [12. Streams, sinks, and pipelining](building_minidb/streams_sinks_and_pipelining/README.md) | 301 | Split halves, several requests in flight, responses kept in order |
| [13. Async functions in traits](building_minidb/async_functions_in_traits/README.md) | 301 | Pluggable durability, the missing `Send` bound, and what `dyn` allows |
| [14. Diagnosing a stuck runtime](building_minidb/diagnosing_a_stuck_runtime/README.md) | 301 | `tokio-console`, runtime metrics, and the sync–async bridges that fail quietly |
| [15. Testing against a hostile network](building_minidb/testing_against_a_hostile_network/README.md) | 301 | `turmoil`: latency, dropped connections and partitions inside a unit test |

## The gap these pages have to close

Every checked example in this library is a single `.rs` file compiled by `rustc` with no dependencies. The basics fit that rule: `Future`, `Pin`, `Context` and a do-nothing `Waker::noop()` are all in std, so a future that prints nothing until its first poll can be an ordinary answer-keyed example. Tokio does not fit it — it is a crate, so the course chapters cannot graduate the same way. [Observability](../21_Observability/README.md) has the same problem and splits each page into a mechanism modelled in std plus an API fence marked as not compiled here; that split is the default plan here too, until the library can check a Cargo project.

## Where the rest of it is

- [Spawning a thread](../09_Advanced/spawning_a_thread/README.md), [Channels](../09_Advanced/channels/README.md) and [`Send` and `Sync`](../09_Advanced/send_and_sync/README.md) — the thread-based versions of chapters 2, 4 and 6
- [Instrumenting async code](../21_Observability/instrumenting_async/README.md) — why a span has to be attached to a future rather than entered
- [An iterator vs a stream](../24_Iterators/iterator_vs_stream/README.md) — the async `Iterator`, which chapter 12 builds on
- [Concurrency learning library ↗](https://masiarek.github.io/concurrency-learning-library/) — the language-neutral concept pages: [async runtime ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/scheduling/async_runtime/index.html), [cancellation ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/cancellation/index.html), [backpressure ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/backpressure/index.html)
- [Going deeper — Async](../10_Resources/going_deeper/README.md#async) — the books

## Po polsku

Programowanie asynchroniczne w Ruscie ma dwie warstwy i warto je rozdzielić od pierwszego dnia. Język i biblioteka standardowa dają tylko **future** (w polskich tekstach zwykle bez tłumaczenia; „przyszłość” się nie przyjęła) oraz składnię `async`/`.await` — wartość, która sama z siebie nic nie robi. Wszystko, co ją faktycznie wykonuje, czyli planista zadań, obsługa gniazd i zegary, pochodzi ze **środowiska uruchomieniowego** (*runtime*), a najpopularniejszym jest crate `tokio`. Pierwsza część działu dotyczy tej pierwszej warstwy; druga to kurs, w którym mały magazyn danych `minidb` staje się serwerem TCP na Tokio.

**Szukaj po polsku:** programowanie asynchroniczne w Ruscie · `async await rust` · `rust tokio tutorial` · `rust future poll`
