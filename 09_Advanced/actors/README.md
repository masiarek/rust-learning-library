# Actors

**Level:** 301 · deep dive

**One line:** An actor is a thread that owns its state outright and is reached only through a channel — no lock anywhere, because no other thread can name the data — and a question to it is a message that carries its own reply channel.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The shape, with a boring subject: `enum Msg { Add(u64), Get(mpsc::Sender<u64>) }`, a thread running `for msg in rx`, and a cloneable handle type that wraps the `Sender` so callers never see the enum.
- Request and response: the caller sends a `Sender<u64>` inside the message and blocks on its `Receiver`. What does that `recv` return if the actor has panicked or exited before replying?
- Against `Arc<Mutex<u64>>`: both serialise access to one value. What does each cost per operation, and which one lets a slow operation hold up everyone else — the lock holder, or the message at the head of the mailbox?
- Several fields, one invariant: inside the actor, a check and the act that depends on it happen with nothing in between. Does that close the gap shown in the last section of [Data races](../../31_C_and_Cpp/data_races/README.md), or does a caller that sends `Get` and then `Add` reopen it one level up?
- Deadlock with no lock in the program: two actors each waiting on the other's reply, and an actor that sends a request to itself.
- The mailbox: an unbounded `channel()` grows without limit when callers outpace the actor; [`sync_channel(n)` ↗](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html) makes them wait instead.
- Ending it: the last handle dropped ends `for msg in rx`; an explicit `Shutdown` message; and a panic inside the actor, which every later caller sees as a closed channel.
- Actors on an async runtime, one task per actor instead of one thread, belong to the async section — named here, not taught.

## The trap it exists for

Rebuilding the race in the handle. `if handle.get() < limit { handle.add(1) }` is two messages, and another caller's message can arrive between them. The actor processed each message atomically and the program is still wrong; the check and the act have to be one message, handled inside the actor.

## Where this sits

- [Channels](../channels/README.md) covers the channel itself; [Worker pools](../worker_pools/README.md) spreads many jobs across many threads, where an actor is one thread and one piece of state. [Lock poisoning](../mutex_poisoning/README.md) and [`RwLock` and atomics](../rwlock_and_atomics/README.md) cover the shared-state alternative this page is compared against.

## See also

- [Channels](../channels/README.md) — the queue an actor reads, and "share memory by communicating"
- [Data races](../../31_C_and_Cpp/data_races/README.md) — the check-then-act bug that survives a correct lock
- [Worker pools](../worker_pools/README.md) — many threads, no owned state
- [`crossbeam` channels](../crossbeam_channels/README.md) — `select!`, for an actor with more than one mailbox
- [`Send` and `Sync`](../send_and_sync/README.md) — why the state need only be `Send` when one thread owns it
- [Lock poisoning](../mutex_poisoning/README.md) — what the `Mutex` version does when an operation panics

## If you are coming from another language

- **Go.** A goroutine that owns a map and serves requests on a channel is Go's standard answer to "who owns this state", and a request carrying a `chan int` for the reply is the same pattern as the `Sender<u64>` here. The difference is shutdown: Go's actor ends when someone closes its channel, Rust's when the last handle is dropped. [Closing a channel ends a range ↗](https://masiarek.github.io/go-learning-library/02_Channels/closing_a_channel_ends_a_range/) is the Go half.
- **Java.** Akka is the actor framework most Java and Scala programmers have met: an actor's mailbox, `tell` for fire-and-forget and `ask` for a reply returning a future. The hand-built Rust version has no supervisor, so a panicking actor is not restarted unless you write that; what transfers is the rule that an actor's fields are touched only by its own message handler.
- **Python.** A thread or `multiprocessing.Process` reading a `queue.Queue` and replying on a queue the caller passed in is the same shape. With `multiprocessing` the isolation is enforced by separate address spaces; in Rust it is enforced by ownership inside one process, at no copying cost.
- **ABAP.** An inbound qRFC queue is the closest match: LUWs with the same queue name are processed one at a time, in order, so the queue name plays the part of the actor's identity and serialisation replaces locking. The check-then-act trap is the same, too — two LUWs in the queue cannot overlap, but a program that reads in one and writes in the next can.
- Concepts: [actor model ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/actor_model/) · [message passing ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/message_passing/) · [thread confinement ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/safety_in_languages/thread_confinement/) · [CSP ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/csp/)

## Po polsku

**Aktor** (*actor*) to wątek, który sam posiada swój stan i do którego dociera się wyłącznie wiadomościami — skoro żaden inny wątek nie może tych danych nazwać, żadna blokada nie jest potrzebna. Pytanie do aktora to wiadomość z dołączonym kanałem zwrotnym (*reply channel*). Pułapka jest ta sama co przy blokadach, tylko piętro wyżej: „sprawdź, potem zmień” wysłane jako dwie osobne wiadomości znów tworzy wyścig, bo między nimi może wejść cudza wiadomość.

**Szukaj po polsku:** model aktorów · przekazywanie komunikatów · uwięzienie w wątku · `rust actor pattern mpsc reply channel` · `actors with tokio`
