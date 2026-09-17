# `crossbeam` channels

**Level:** 301 · deep dive

**One line:** Since Rust 1.67 std's `mpsc` has been built on the design of [`crossbeam-channel` ↗](https://docs.rs/crossbeam-channel) — same machinery, smaller API — so the reasons left to add the crate are a `Receiver` you can clone and a `select!` that waits on several channels at once.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What 1.67 changed: the [release announcement ↗](https://blog.rust-lang.org/2023/01/26/Rust-1.67.0/) says the `mpsc` implementation was "switched out to be based on crossbeam-channel", with no API changes. So is there any performance reason left to add the crate, and what did std deliberately not take?
- Multi-consumer: `crossbeam_channel::Receiver` is `Clone`, so N workers read one channel without the `Arc<Mutex<Receiver>>` that [Worker pools](../worker_pools/README.md) needs. std has [`std::sync::mpmc` ↗](https://doc.rust-lang.org/std/sync/mpmc/index.html), but in Rust 1.98 it is nightly-only (`mpmc_channel`, issue #126840).
- `select!` with `recv`, `send` and `default(timeout)` arms. The docs say that when several operations are ready at once, "a random one among them is selected" — what does a program that relies on arm order print over a thousand runs?
- Zero capacity: `bounded(0)` is a rendezvous where a send waits for a receive — and so is std's [`sync_channel(0)` ↗](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html), whose docs call it a "rendezvous channel". The difference people cite is not a difference.
- The extra constructors `after`, `at`, `tick` and `never`, each returning only a `Receiver` — how they turn a timeout or a heartbeat into one more `select!` arm.
- Disconnection with many receivers: when does `send` start returning `Err`, and when does `recv` on one clone see the channel as closed?
- `crossbeam` is a family: the umbrella [`crossbeam` ↗](https://docs.rs/crossbeam) crate lists channels, work-stealing deques, epoch-based memory management and `CachePadded` among its tools. Which single crate does this page need, and what does depending on the umbrella add to a build?

## The trap it exists for

Adding `crossbeam-channel` "because it is faster than std" in code written after 2023, when std has used the same design since 1.67 — or hand-building a shared `Arc<Mutex<Receiver>>` when the two features that were missing, a cloneable receiver and `select!`, are exactly what the crate provides.

## Where this sits

- [Channels](../channels/README.md) covers std's semantics — `send` moves, empty against disconnected, a bound as backpressure, and the kept `Sender`. This page covers only what `crossbeam-channel` adds on top.

## See also

- [Channels](../channels/README.md) — the std API this page is measured against
- [Worker pools](../worker_pools/README.md) — the `Arc<Mutex<Receiver>>` that a cloneable receiver removes
- [Actors](../actors/README.md) — one owner with several mailboxes, the case `select!` is for
- [Spawning a thread](../spawning_a_thread/README.md) — the threads on either end
- [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) — what `cargo add crossbeam-channel` writes
- [Books](../../10_Resources/books/README.md) — *Rust in Action*'s chapter 10 teaches threads with `crossbeam` channels, and that page pairs it with the two std lessons

## If you are coming from another language

- **Go.** This is the crate that makes Rust channels feel like Go's. A Go channel has any number of senders and receivers, which is `crossbeam`'s `Clone` on both ends; `select` picks at random among ready cases, as `select!` does — [`select` chooses at random ↗](https://masiarek.github.io/go-learning-library/03_Select/select_chooses_at_random/) and [`select` waits on many channels ↗](https://masiarek.github.io/go-learning-library/03_Select/select_waits_on_many/) are the Go pages. An unbuffered `make(chan T)` is `bounded(0)`, per [An unbuffered send waits for a receiver ↗](https://masiarek.github.io/go-learning-library/02_Channels/an_unbuffered_send_waits_for_a_receiver/), and `time.After` is `crossbeam_channel::after`. What stays different is closing: Go closes a channel with `close`, `crossbeam` when the last sender drops.
- **Java.** `BlockingQueue` is multi-consumer already, and `SynchronousQueue` is the zero-capacity rendezvous. Java has no `select` over queues; the usual substitute is one queue carrying a sum type, which in Rust is an `enum` on one channel — often simpler than `select!`.
- **Python.** `queue.Queue` is multi-consumer, and there is no select across several of them for threads. `asyncio.wait(…, return_when=FIRST_COMPLETED)` is the nearest thing, and it lives in the async world.
- Concepts: [select ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/select/) · [unbuffered channel ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/unbuffered_channel/) · [bounded channel ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/bounded_channel/) · [channel ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/channel/)

## Po polsku

Od Rusta 1.67 standardowe `mpsc` działa w oparciu o projekt `crossbeam-channel`, więc „szybciej niż std” przestało być argumentem. Zostają dwie rzeczy, których std na stabilnym kanale (*stable*) nie ma: **klonowalny odbiornik** (`Receiver: Clone`, czyli wielu konsumentów — *multi-consumer*) i makro `select!`, które czeka na kilka kanałów naraz. Kanał o pojemności zero (*rendezvous*) nie jest różnicą — `sync_channel(0)` z biblioteki standardowej działa tak samo.

**Szukaj po polsku:** kanał wielu producentów i wielu konsumentów · kanał spotkaniowy (rendezvous) · `crossbeam channel vs std mpsc` · `rust select multiple channels` · `rust mpmc channel stable`
