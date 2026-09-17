# Testing async code

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Async tests become fast and deterministic once three things are taken out of them — real time, real sockets, and assumptions about scheduling order — and `tracing` spans turn "what happened" into data a test can assert on.

## What it has to cover

- `#[tokio::test]` and the runtime flavour it builds by default
- **The paused clock.** [`time::pause` ↗](https://docs.rs/tokio/latest/tokio/time/fn.pause.html) (or `start_paused = true`): when every task is idle the clock jumps to the next timer, so a test of a thirty-second idle timeout finishes in microseconds
- [`tokio::io::duplex` ↗](https://docs.rs/tokio/latest/tokio/io/fn.duplex.html): two connected in-memory streams, so the connection handler can be tested with no port and no listener — once the handler is generic over `AsyncRead + AsyncWrite` instead of taking a `TcpStream`
- **Properties, not interleavings.** Assert that every request gets exactly one response and that the store ends in a state some ordering could produce — not that task A ran before task B
- Instrumenting minidb with [`tracing` ↗](https://docs.rs/tracing/latest/tracing/): a span per connection and per request, with fields
- Capturing those spans in a test as structured values and asserting on their fields, rather than matching log text
- The anatomy of a flaky async test, and how to reproduce one on purpose

## The trap it exists for

`sleep(Duration::from_millis(50))` in a test "to let the task run". It is slow when it works, flaky when the machine is busy, and still says nothing about the order it was meant to wait for.

## What minidb gains

A test suite that runs without the network or the wall clock, and spans that the tests and chapter 14's diagnosis both read.

## See also

- [Testing](../../../28_Testing/README.md) — the synchronous toolkit this builds on
- [Instrumenting async code](../../../21_Observability/instrumenting_async/README.md) and [Spans, not lines](../../../21_Observability/spans_not_lines/README.md) — the observability side of the same spans
- [Testing against a hostile network](../testing_against_a_hostile_network/README.md) — the same determinism, applied to the network
- [Deterministic testing ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/testing_and_tools/deterministic_testing/index.html) — in the Concurrency library

## Po polsku

Testy kodu asynchronicznego stają się szybkie i powtarzalne, gdy usunie się z nich trzy rzeczy: prawdziwy czas, prawdziwe gniazda i założenia o kolejności wykonywania zadań. **Wstrzymany zegar** Tokio przeskakuje do najbliższego timera, gdy wszystkie zadania czekają, więc test trzydziestosekundowego limitu trwa mikrosekundy; `tokio::io::duplex` daje parę połączonych strumieni w pamięci zamiast gniazda. Asercje dotyczą własności (każde żądanie dostaje dokładnie jedną odpowiedź), a nie kolejności.

**Szukaj po polsku:** testowanie kodu asynchronicznego · `tokio test pause time` · `tokio io duplex test` · `tracing test assert spans`
