# Testing against a hostile network

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** [`turmoil` ↗](https://docs.rs/turmoil/latest/turmoil/) runs the server and its clients as simulated hosts on a simulated network, so latency, dropped connections and partitions become deterministic inputs to an ordinary unit test — possible here because the server already takes its IO and its durability through abstractions it can swap.

## What it has to cover

- A simulation with one server host and several client hosts, and the simulated `TcpListener` and `TcpStream` that replace Tokio's
- **Swapping the network in:** why the handler generic over `AsyncRead + AsyncWrite` (chapters 8 and 10) and the durability trait (chapter 13) make this a change at the edges rather than a rewrite
- Injecting failure: added latency, dropped messages, a partition between hosts and its repair, and a server host that crashes and restarts — replaying its log from chapter 9
- **Determinism:** a seed that makes a failing run repeat exactly, and how to turn that run into a regression test
- The properties worth asserting: no acknowledged write is lost across a crash; a client that retries after a timeout does not apply a write twice
- What a simulation cannot find: the real kernel, the real disk's sync behaviour, and timing on real hardware

## The trap it exists for

A test suite that only ever runs on a network that never fails. Every chapter before this one handled failures; this is the first place the tests make any of them happen.

## What minidb gains

A test that crashes the server mid-write under a partitioned network, and passes.

## See also

- [Testing async code](../testing_async_code/README.md) — paused time and in-memory streams, which this extends to the whole network
- [Mocking a server](../../../07_Clients/mocking_a_server/README.md) — the client-side version of a fake network, and what it cannot prove
- [Deterministic testing ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/testing_and_tools/deterministic_testing/index.html) and [nondeterminism ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/foundations/nondeterminism/index.html) — in the Concurrency library

## Po polsku

`turmoil` uruchamia serwer i klientów jako symulowane hosty w symulowanej sieci, więc opóźnienia, zerwane połączenia i **podział sieci** (*partition*) stają się powtarzalnymi danymi wejściowymi zwykłego testu jednostkowego. To możliwe tylko dlatego, że serwer korzysta z wejścia-wyjścia i z trwałości danych przez abstrakcje, które da się podmienić. Test ma sprawdzać własności — na przykład że żaden potwierdzony zapis nie ginie po awarii — i dawać się odtworzyć z tego samego ziarna losowości.

**Szukaj po polsku:** testowanie z symulacją sieci · testy deterministyczne · `rust turmoil network simulation` · `deterministic simulation testing`
