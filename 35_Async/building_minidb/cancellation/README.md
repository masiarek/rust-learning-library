# Cancellation

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Cancelling async work means dropping its future, which can happen at any `.await` — so the question to ask of every future is what it leaves behind when dropped there, and a future is *cancel safe* only when the answer is "nothing lost".

## What it has to cover

- Drop is cancel: destructors run, and no code after the suspension point ever does
- [`timeout` ↗](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html) drops the inner future when time runs out; [`select!` ↗](https://docs.rs/tokio/latest/tokio/macro.select.html) drops every branch that did not win — on every pass through a loop
- [`CancellationToken` ↗](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html): asking a task to stop, rather than dropping it from outside, and child tokens for a tree of work
- **The state a dropped future leaves.** A response half-written to the socket, a store update applied but never acknowledged, bytes read into a local buffer and lost
- **Cancel safety.** Why [`Lines::next_line` ↗](https://docs.rs/tokio/latest/tokio/io/struct.Lines.html) is documented as cancel safe — its partial line lives in the reader, not in the future — and why a hand-written read loop that keeps its buffer inside the future is not
- Fixing a non-cancel-safe branch in a `select!` loop: create the future once outside the loop and poll it by reference, or move the buffer out of the future
- Dropping a `JoinHandle` does *not* cancel the task — it detaches it; `abort()` is the cancel

## The trap it exists for

A `select!` loop with a timeout branch beside a read that is not cancel safe. Each time the timer wins, the read's future is dropped along with the bytes it had already taken from the socket, and the protocol desynchronises with no error.

## What minidb gains

An idle-connection timeout and a per-request deadline, with every read in the connection loop cancel safe.

## See also

- [Codecs and framing](../codecs_and_framing/README.md) — where cancel safety gets its structural fix
- [Shutdown and supervision](../shutdown_and_supervision/README.md) — `CancellationToken` used for the whole server
- [Cancellation ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/cancellation/index.html), [timeout ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/timeout/index.html) and [select ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/select/index.html) — in the Concurrency library

## Po polsku

**Anulowanie** w asynchronicznym Ruscie to po prostu porzucenie (*drop*) future’a — a to może się stać w każdym punkcie `.await`. Dlatego o każdy future trzeba zapytać, jaki stan zostawi po sobie, gdy zostanie porzucony właśnie tam. Future jest **bezpieczny przy anulowaniu** (*cancel safe*), jeśli nic nie ginie: `next_line` taki jest, bo niedokończona linia siedzi w buforze czytnika, a ręcznie napisana pętla z buforem wewnątrz future’a — nie.

**Szukaj po polsku:** anulowanie zadań w tokio · `tokio cancel safety` · `tokio select cancellation` · `tokio_util CancellationToken`
