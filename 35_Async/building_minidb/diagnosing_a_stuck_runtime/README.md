# Diagnosing a stuck runtime

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A worker thread stuck in blocking code produces no error and no log line — only requests that stop being answered — so finding one takes tools that watch the tasks from outside: [`tokio-console` ↗](https://github.com/tokio-rs/console), the runtime's own metrics, and a map of the sync–async bridges that fail quietly.

## What it has to cover

- Planting the bug: a `std::thread::sleep`, a synchronous file read, or a long computation inside one request handler — and what the other clients see
- **`tokio-console`.** Wiring it in (the subscriber crate and the `tokio_unstable` cfg flag), then reading its task list: busy, idle and scheduled time, poll durations, and the warnings it raises for a long poll or a task that lost its waker
- [Runtime metrics ↗](https://docs.rs/tokio/latest/tokio/runtime/struct.RuntimeMetrics.html): which are stable, which need `tokio_unstable`, and the two or three worth exporting from a real server
- **The bridges.** [`block_in_place` ↗](https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html) (multi-thread runtime only), [`Handle::block_on` ↗](https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html) (not from inside an async context), `spawn_blocking`, and a synchronous channel between the two worlds — what each is for, and the condition under which each one panics or deadlocks
- A blocking pool that is quietly full: `spawn_blocking` calls queueing behind each other with no error
- The spans added in chapter 8, read as evidence of where a request stopped

## The trap it exists for

`block_in_place` that works in production and panics in a test. The server runs on the multi-thread runtime; `#[tokio::test]` builds a current-thread one by default, where the call is not allowed.

## What minidb gains

A console hook behind a feature flag, a handful of exported runtime metrics, and no blocking call left on a worker thread.

## See also

- [Tasks](../tasks/README.md) — `spawn_blocking`, first met there
- [Debugging](../../../32_Debugging/README.md) — the library's map of what to reach for when something surprises you
- [Metrics and cardinality](../../../21_Observability/metrics_and_cardinality/README.md) — before exporting per-task numbers
- [Blocking the event loop ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/blocking_the_event_loop/index.html) — in the Concurrency library

## Po polsku

Wątek roboczy zablokowany przez synchroniczny kod nie zgłasza błędu i nie zostawia wpisu w logu — widać tylko żądania, na które nikt nie odpowiada. Żeby go znaleźć, trzeba obserwować zadania z zewnątrz: `tokio-console` pokazuje czas zajętości i czas trwania każdego `poll`, a metryki środowiska uruchomieniowego — kolejki i wątki. Druga część tematu to **mosty między światem synchronicznym i asynchronicznym** (`block_in_place`, `Handle::block_on`), które w złym kontekście panikują albo po cichu się zakleszczają.

**Szukaj po polsku:** zablokowany wątek roboczy · `tokio console tutorial` · `tokio block_in_place panic current_thread` · `tokio runtime metrics`
