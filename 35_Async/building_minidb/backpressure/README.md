# Backpressure

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** An unbounded queue does not remove overload, it moves it into memory, where it ends the process — so every queue in the server needs a bound and a decision for when it is full: make the sender wait, refuse the work, or drop it.

## What it has to cover

- An unbounded channel under a producer faster than its consumer, with memory measured as it grows
- Bounded [`mpsc` ↗](https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html) and the three policies: `send().await` **waits** (the slowness travels back to the producer), `try_send` **refuses** (the client hears "busy" now), and dropping is **lossy** (acceptable for metrics, never for writes)
- Choosing a policy per queue in minidb — the store actor's queue, a logging queue — and choosing the bound from a measurement rather than a round number
- A [`Semaphore` ↗](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html) to limit concurrent connections, with `acquire_owned` so the permit moves into the connection task and is released when the task ends
- **Where the permit is acquired decides what the limit protects.** Before `accept`, excess clients wait in the kernel's listen backlog and hold nothing of yours; after `accept` but inside the spawned task, every waiting client already holds a socket and a file descriptor

## The trap it exists for

Acquiring the permit inside the spawned task. The limit then caps work, not connections — accepted sockets pile up without bound while they wait for a permit, and the file-descriptor limit is hit anyway.

## What minidb gains

A bounded queue in front of the store and a cap on concurrent connections, each with a stated policy for when it is full.

## See also

- [Who owns the state](../who_owns_the_state/README.md) — the actor whose queue this bounds
- [Channels](../../../09_Advanced/channels/README.md) — `sync_channel`, std's bounded channel
- [Backpressure ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/backpressure/index.html), [bounded channel ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/bounded_channel/index.html) and [semaphore ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/synchronization/semaphore/index.html) — in the Concurrency library

## Po polsku

Nieograniczona kolejka nie usuwa przeciążenia, tylko przenosi je do pamięci, gdzie kończy się ono śmiercią procesu. **Ciśnienie zwrotne** (*backpressure*) to decyzja, co zrobić, gdy ograniczona kolejka jest pełna: kazać nadawcy czekać, odmówić albo porzucić pracę. Przy ograniczaniu liczby połączeń semaforem liczy się miejsce pobrania zezwolenia — przed `accept` nadmiarowi klienci czekają w kolejce jądra, po `accept` każdy z nich już zajmuje gniazdo i deskryptor pliku.

**Szukaj po polsku:** ciśnienie zwrotne · ograniczona kolejka · `tokio bounded channel backpressure` · `tokio semaphore limit connections`
