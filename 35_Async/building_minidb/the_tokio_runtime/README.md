# The Tokio runtime

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Tokio supplies what the language leaves out — a scheduler that polls futures, a driver that wakes them when a socket or timer is ready, and the async versions of IO and locks — and `#[tokio::main]` is a macro that builds that runtime and blocks on your `async main`.

## What it has to cover

- What std gives (the `Future` trait, `async`/`.await`, `Waker`) against what Tokio adds: an executor, an IO driver, timers, async sync primitives, and task spawning
- What `#[tokio::main]` expands to, shown with `cargo expand` rather than described
- The two runtime flavours — multi-thread and current-thread — and which one `#[tokio::test]` picks by default
- Why a future does nothing until polled, and why `.await` is a suspension point rather than a call — [the std pages](../../what_a_future_is/README.md) say it without a runtime; this page shows who does the polling
- Just enough `Future`, `poll` and `Pin` to reason about a runtime: `Pending` means "I have arranged to be woken", and a future must not move once polled. [Chapter 10](../async_read_and_async_write/README.md) returns to both at the poll level
- Sequential `.await`s are sequential: two awaited calls in a row do not run at the same time

## The trap it exists for

Believing `async` makes code concurrent. It makes code *suspendable*. Nothing runs at the same time as anything else until it is spawned or joined — a function that awaits three requests one after another takes as long as the three added together.

## What minidb gains

An `async main` on a Tokio runtime, calling the store that is still entirely synchronous. Nothing is concurrent yet.

## See also

- [Tasks](../tasks/README.md) — the next chapter: how work actually starts running concurrently
- [What a future is](../../what_a_future_is/README.md) — the trait, without a runtime
- [Async runtime ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/scheduling/async_runtime/index.html) and [polling ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/scheduling/polling/index.html) — in the Concurrency library
- [Tokio docs ↗](https://docs.rs/tokio/latest/tokio/) — the crate root lists every feature flag, which is also the list of what the runtime provides

## Po polsku

Tokio to **środowisko uruchomieniowe** (*runtime*) dla kodu asynchronicznego: dokłada to, czego nie ma w języku — planistę, który wywołuje `poll` na future’ach, sterownik budzący je, gdy gniazdo lub zegar są gotowe, oraz asynchroniczne wersje wejścia-wyjścia i blokad. `#[tokio::main]` to makro, które takie środowisko tworzy i uruchamia w nim `async main`. Samo słowo `async` nie sprawia, że kod działa równolegle — sprawia tylko, że może zostać zawieszony.

**Szukaj po polsku:** środowisko uruchomieniowe tokio · `tokio main macro expand` · `tokio runtime flavor current_thread`
