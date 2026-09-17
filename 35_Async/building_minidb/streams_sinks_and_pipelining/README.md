# Streams, sinks, and pipelining

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A [`Stream` ↗](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) is the async `Iterator` and a [`Sink` ↗](https://docs.rs/futures/latest/futures/sink/trait.Sink.html) its mirror image, and once a connection is split into halves with separate owners, one client can have several requests in flight — provided the server bounds how many, and still answers in the order they were asked.

## What it has to cover

- `Stream::poll_next` and `StreamExt::next`, beside `Iterator::next`
- `Sink`'s four methods, and why `poll_ready` comes first: it is backpressure, built into the trait
- Splitting a connection: `TcpStream::into_split` for owned halves, and `split` on a `Framed` for a stream half and a sink half
- **Pipelining.** A reader that keeps taking requests while earlier ones are still being served, and a writer that sends responses as they complete
- **Order, on purpose.** `FuturesOrdered` or `buffered(n)` keeps responses in request order; `buffer_unordered(n)` does not — and a client pairing responses with requests by position needs the first
- Choosing the in-flight bound `n`, and what head-of-line blocking costs when one slow request holds up the responses behind it

## The trap it exists for

Reaching for `buffer_unordered` because it is faster. The server now answers in completion order, and a client that matches responses to requests by position silently pairs them wrong.

## What minidb gains

Pipelined connections with a bounded number of requests in flight and responses in request order.

## See also

- [An iterator vs a stream](../../../24_Iterators/iterator_vs_stream/README.md) — the two traits side by side
- [Iterators are lazy](../../../24_Iterators/iterators_are_lazy/README.md) — adapters that compute nothing until pulled, which a stream shares
- [Backpressure](../backpressure/README.md) — the bound, first chosen there
- [Async stream ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/async_stream/index.html) — in the Concurrency library

## Po polsku

`Stream` to asynchroniczny odpowiednik iteratora, a `Sink` — jego lustrzane odbicie, do którego się zapisuje; metoda `poll_ready` wbudowuje ciśnienie zwrotne w sam trait. Po podzieleniu połączenia na część do czytania i część do pisania jeden klient może mieć kilka żądań w toku naraz (**potokowanie**, *pipelining*), ale serwer musi ograniczyć ich liczbę i odpowiadać w kolejności żądań — `buffer_unordered` tej kolejności nie zachowuje.

**Szukaj po polsku:** strumienie asynchroniczne · potokowanie żądań · `rust futures stream sink` · `buffered vs buffer_unordered`
