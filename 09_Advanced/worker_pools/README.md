# Worker pools

**Level:** 301 · deep dive

**One line:** A worker pool is a fixed number of long-lived threads taking jobs from one queue — std has every part (`mpsc`, `Arc<Mutex<Receiver>>`, `JoinHandle`) and no pool, so building one means deciding how jobs arrive, how results come back and how the pool ends.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why not one thread per job: what do 10,000 `thread::spawn` calls cost in time and memory against 8 threads and a queue, measured on this machine?
- The shape: [`mpsc::Receiver` ↗](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html) is not `Clone`, so N workers share one behind `Arc<Mutex<Receiver<Job>>>`, with `type Job = Box<dyn FnOnce() + Send + 'static>` — the design [The Book's final project ↗](https://doc.rust-lang.org/book/ch21-02-multithreaded.html) builds. Which of the three bounds on `Job` does each line of the pool need?
- The lock held too long: `let job = receiver.lock().unwrap().recv().unwrap();` drops the guard at the end of the statement, while `while let Ok(job) = receiver.lock().unwrap().recv() { job(); }` keeps it for the whole body, so one slow job blocks every other worker. The Book shows this as Listing 21-21; the finished page runs both and prints the difference.
- Shutting down by dropping the `Sender`: every worker's `recv` returns `Err(RecvError)`, its loop ends, and the pool's `Drop` joins each handle. What happens to jobs still queued, and what if the `Sender` is never dropped?
- Results back: a second channel, and a job that carries its index so the report comes out in input order however the jobs were scheduled.
- A panicking job: does the worker thread die with it, does the pool know it now has N−1 threads, and does anything poison?
- Sizing and backpressure: [`available_parallelism`](../concurrency_or_parallelism/README.md) for CPU-bound jobs, more for I/O-bound ones, and [`sync_channel(n)` ↗](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html) so a fast producer waits instead of filling memory.

## The trap it exists for

The `while let` loop above. It compiles, it runs, every job completes, and the pool processes one job at a time — the guard from `lock()` is a temporary that lives to the end of the `while let` body. Nothing fails; the pool is serial, and only a timing shows it.

## Where this sits

- [Channels](../channels/README.md) covers one sender and one receiver, and the kept `Sender` that hangs the program; [Spawning a thread](../spawning_a_thread/README.md) covers the `JoinHandle`. This page covers only the pool built from them. [Actors](../actors/README.md) is the one-thread-one-state variant, and [`rayon`](../rayon/README.md) is the pool you do not write.

## See also

- [Channels](../channels/README.md) — the queue, and the disconnect that ends the pool
- [Temporary lifetime extension](../../18_Ownership/temporary_lifetimes/README.md) — why the guard in a `while let` scrutinee outlives the statement
- [Lock poisoning](../mutex_poisoning/README.md) — what a panic under the receiver's lock would do
- [`crossbeam` channels](../crossbeam_channels/README.md) — a cloneable `Receiver`, which removes the `Mutex`
- [Concurrency or parallelism](../concurrency_or_parallelism/README.md) — how many workers, and why the speedup is less than that
- [Actors](../actors/README.md) — one worker that owns its state
- [`rayon`](../rayon/README.md) — a work-stealing pool behind a parallel iterator

## If you are coming from another language

- **Go.** [A worker pool ↗](https://masiarek.github.io/go-learning-library/06_Patterns/a_worker_pool/) is the same design with less code: a Go channel has any number of receivers, so three goroutines ranging over `jobs` need no mutex, and shutdown is `close(jobs)` written by hand. Rust ties the close to the last `Sender` being dropped, and ties the shared receiver to a lock you have to hold for exactly one statement.
- **Python.** `concurrent.futures.ThreadPoolExecutor(max_workers=n)` is the finished product this page builds: `submit` returns a `Future`, `shutdown(wait=True)` is the join. `queue.Queue` is multi-consumer already, which is why no Python pool needs the `Arc<Mutex<Receiver>>` step. With the GIL, a CPU-bound job gains nothing from the pool; in Rust it does, which is the reason to build one.
- **Java.** `Executors.newFixedThreadPool(n)`, `shutdown()` and `awaitTermination` are this page's three decisions made for you. A task handed to `submit` that throws leaves its exception in the `Future`; one handed to `execute` ends its worker thread, and the executor starts a replacement. Ask the same question of the Rust pool: who replaces a worker whose job panicked?
- **ABAP.** `CALL FUNCTION … STARTING NEW TASK … DESTINATION IN GROUP` with `RECEIVE RESULTS` is a worker pool whose workers are dialog work processes and whose size Basis sets in RZ12; `RESOURCE_FAILURE` is the full queue. The `SPTA` framework wraps the same loop. Rust's version has no server group to ask — the pool size is a number in your code.
- Concepts: [worker pool ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/worker_pool/) · [thread pool ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/units_of_execution/thread_pool/) · [producer–consumer ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/producer_consumer/) · [task queue ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/task_queue/)

## Po polsku

**Pula wątków** (*thread pool*, *worker pool*) to stała liczba długo żyjących wątków, które pobierają zadania z jednej kolejki — w bibliotece standardowej Rusta są wszystkie części, ale nie ma gotowej puli. Najważniejsza pułapka siedzi w czasie życia strażnika blokady (*guard*): `while let Ok(job) = receiver.lock().unwrap().recv()` trzyma `Mutex` przez całe ciało pętli, więc pula działa poprawnie i… sekwencyjnie. Zamknięcie puli to upuszczenie (*drop*) ostatniego `Sender`a — wtedy każde `recv` zwraca błąd i wątki kończą pracę.

**Szukaj po polsku:** pula wątków w Ruscie · kolejka zadań · producent–konsument · `rust thread pool mpsc arc mutex receiver` · `rust while let mutex lock held`
