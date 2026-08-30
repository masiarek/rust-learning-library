# Channels

**Level:** 201 → 301 · working knowledge

**One line:** A channel moves a value from one thread to another, and ownership goes with it — which is why nothing needs a lock: at every moment the value has exactly one owner.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(String::from("counted")).unwrap();
    });
    println!("{}", rx.recv().unwrap());   // counted
}
```

`send` takes the value **by value**. The `String` was built on the spawned thread and is owned by `main` now — one move, no copy, and the sending thread cannot touch it again because it no longer has it. That is the channel's entire safety argument, and it is the borrow checker's rule rather than a runtime one.

## Multi-producer, single-consumer

`mpsc` means the `Sender` is `Clone` and the `Receiver` is not.

```rust
for name in ["Ada", "Ben", "Cara"] {
    let tx = tx.clone();
    thread::spawn(move || { tx.send(name).unwrap(); });
}
drop(tx);                                  // <- the original
let mut names: Vec<&str> = rx.iter().collect();
names.sort();
```

Note the sort: arrival **order** is whatever the scheduler decided, so anything that compares output has to impose one.

## The trap: the `Sender` you kept

`for x in rx` ends when **every** `Sender` has been dropped. The clones inside the threads drop when those threads finish. The original `tx` in `main` does not — unless you drop it.

Forget that one line and the loop waits forever, on a program that has finished all its work. There is no error message, no deadlock detector, and nothing in the output: just a process that never exits. It is the most common `mpsc` bug there is.

Either drop it explicitly, or never bind it — hand out clones and let the original go out of scope with the block that made it.

## `recv` distinguishes empty from finished

| | |
|---|---|
| `rx.recv()` | blocks until a value arrives, or `Err(RecvError)` when every sender is gone |
| `rx.try_recv()` | returns now: `Empty` or `Disconnected` |
| `rx.recv_timeout(d)` | blocks up to `d`, then `Timeout` |

*Empty* and *Disconnected* are two different answers and only the second is a reason to stop. The practice below uses `recv_timeout` to show the hang without hanging — the `Timeout` it prints is exactly what `for x in rx` would have waited on forever.

`send` can fail too, when the receiver has been dropped, and the value comes back inside the `SendError` — the only sensible design for a function that took ownership and could not deliver.

## Bounded channels are backpressure

`mpsc::sync_channel(2)` holds two values; the third `send` **blocks** until the consumer takes one. An unbounded channel with a fast producer and a slow consumer is a memory leak with good manners; a bound turns it into the producer running at the consumer's speed.

## Channel or `Arc<Mutex<T>>`?

| | |
|---|---|
| **channel** | hand a value **over**: a pipeline, a work queue, results collected from a fan-out |
| **`Arc<Mutex<T>>`** | share **one** value: a counter, a cache, a registry |

The question is whether the data has an owner at each moment or is genuinely shared. Reaching for a `Mutex` where a channel would do is how a program acquires a lock it then has to reason about — and a lock is the thing that can poison, deadlock, and serialise your readers.

## If you are coming from another language

- **Python.** `queue.Queue` is the same object and a weaker guarantee: a Python queue passes a *reference*, so the producer can keep using the object it just handed over, and the resulting bug is invisible until it is not. Rust's `send` moves, and the compiler stops the second use. `q.join()`/`task_done()` has no counterpart — Rust's shutdown signal is the last `Sender` dropping, which is the same idea expressed through ownership rather than through a counter. A `multiprocessing.Queue` is closer in spirit, since it genuinely copies, and it is worth noticing that Rust gives you that isolation without leaving the process.
- **ABAP.** There is no in-memory channel; the shape you already know is the qRFC/tRFC queue, or a shared-memory area with `ENQUEUE`. Both correspondences are useful. A qRFC unit is exactly this page's model — the payload is serialised, handed over, and the sender no longer owns it — which is why that pattern is reliable and why a `SHARED MEMORY` area guarded by locks is the one that goes wrong. The channel-versus-mutex table above is the same choice ABAP makes between a queue and a shared-memory-enabled class, with the same answer: prefer handing the data over.
- **Go.** `chan T` is the direct ancestor of this API, and *"do not communicate by sharing memory; share memory by communicating"* is the sentence this whole page is arguing. Two differences: Rust's channel is `mpsc` rather than any-to-any, and Rust's closes when the senders drop rather than on an explicit `close()`, so the shutdown is tied to ownership instead of remembered.
- **Java.** `BlockingQueue` and `ExecutorService`, with `put`/`take` matching `send`/`recv` and `ArrayBlockingQueue`'s capacity matching `sync_channel`. Java passes references, so the same "who still holds it?" question applies as in Python.

---

## The verified output

<!-- output:channels -->
*Verified output of [`channels.rs`](examples/channels.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One sender, one receiver, one value
   rx.recv() = "counted"
   `send` takes the value BY VALUE. The String was built on the
   spawned thread and is owned by main now — one move, no copy, and
   the sending thread cannot touch it again because it no longer has
   it. That is the channel's whole safety argument.

2. Many senders: clone the Sender, not the Receiver
   [("Ada", 3), ("Ben", 3), ("Cara", 4)]
   mpsc is Multi-Producer, Single-Consumer: Sender is Clone and
   Receiver is not. Note the sort — the arrival ORDER is whatever the
   scheduler decides, so anything comparing output has to impose one.

3. The hang, and the one line that prevents it
   `for x in rx` ends when every Sender has been dropped. The clones
   inside the threads drop when those threads finish; the ORIGINAL tx
   in main does not, unless you drop it. Forget that `drop(tx)` and
   the loop above waits forever, on a program that looks finished.
   It is the single most common mpsc bug, and it has no error message
   at all — just a process that never exits.

4. `recv` tells you the difference between empty and finished
   first  recv() = Ok(5)
   second recv() = Err(RecvError)   <- RecvError: every sender is gone
   `recv` BLOCKS until a value arrives or the channel closes;
   `try_recv` returns immediately with Empty or Disconnected. Those
   are two different questions and the types keep them apart.

5. And the send can fail too
   send() with no receiver = true
   The value comes back inside the SendError, so nothing is lost —
   which is the only sensible design for a function that took
   ownership and could not deliver.

6. Which tool for which problem
   channel        hand a value OVER: a pipeline, a work queue, a
                  result collected from a fan-out
   Arc<Mutex<T>>  share ONE value: a counter, a cache, a registry
   The question is whether the data has an owner at each moment or
   is genuinely shared. Reaching for a Mutex when a channel would do
   is how a program acquires a lock it then has to reason about.
```
<!-- /output -->

## Practice

**A three-stage pipeline, and the drop that ends it.** Read seven ballot lines, two of which are not ballots. Stage one sends the raw lines, stage two parses and forwards the good ones while counting the rejects, and the main thread folds what survives into a total. Each stage should shut down by itself, in order, with nothing co-ordinating it.

Then reproduce the classic hang *without hanging*: keep one extra `Sender` clone alive, and use `recv_timeout` to show what a `for x in rx` would be waiting on. Drop the clone and watch the answer change from `Timeout` to `RecvError`, and say why only one of those two is a reason to stop.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:channels_kata -->
*[`channels_kata.rs`](examples/channels_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a three-stage pipeline, and the drop that ends it.
//!
//!   rustc --edition 2024 channels_kata.rs -o /tmp/chk && /tmp/chk

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Raw ballot lines, two of which are not ballots.
const LINES: [&str; 7] = ["5,3,0", "4,4,1", "x,2,2", "0,5,2", "9,1,1", "3,3,3", "2,4,4"];

fn parse(line: &str) -> Option<[u32; 3]> {
    let mut out = [0u32; 3];
    let mut cells = line.split(',');
    for slot in &mut out {
        let n: u32 = cells.next()?.trim().parse().ok()?;
        if n > 5 {
            return None;
        }
        *slot = n;
    }
    if cells.next().is_some() { None } else { Some(out) }
}

fn main() {
    println!("1. Stage one: parse on a worker, collect in main");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for line in LINES {
            tx.send((line, parse(line))).unwrap();
        }
        // tx is dropped here, when the closure ends. That is what ends the
        // `for` loop below.
    });
    let mut seen: Vec<(&str, Option<[u32; 3]>)> = rx.iter().collect();
    seen.sort();
    for (line, parsed) in &seen {
        println!("   {line:8} -> {parsed:?}");
    }

    println!();
    println!("2. Two stages, wired together");
    let (raw_tx, raw_rx) = mpsc::channel::<&str>();
    let (good_tx, good_rx) = mpsc::channel::<[u32; 3]>();
    let reader = thread::spawn(move || {
        for line in LINES {
            raw_tx.send(line).unwrap();
        }
    });
    let parser = thread::spawn(move || {
        let mut rejected = 0;
        for line in raw_rx {
            match parse(line) {
                Some(b) => good_tx.send(b).unwrap(),
                None => rejected += 1,
            }
        }
        rejected
    });
    let totals = good_rx.iter().fold([0u32; 3], |mut acc, b| {
        for i in 0..3 {
            acc[i] += b[i];
        }
        acc
    });
    reader.join().unwrap();
    let rejected = parser.join().unwrap();
    println!("   totals   = {totals:?}");
    println!("   rejected = {rejected}");
    println!("   Each stage owns its Sender and drops it by finishing, which closes");
    println!("   the next stage's loop in turn. The pipeline shuts down from the");
    println!("   FRONT, one stage at a time, with nothing to co-ordinate.");

    println!();
    println!("3. The hang, demonstrated without hanging");
    let (tx, rx) = mpsc::channel::<u32>();
    let keep = tx.clone();
    thread::spawn(move || {
        tx.send(1).unwrap();
    })
    .join()
    .unwrap();
    println!("   one value sent, the worker's Sender dropped, and a clone kept:");
    println!("   recv()               = {:?}", rx.recv());
    println!("   recv_timeout(50ms)   = {:?}", rx.recv_timeout(Duration::from_millis(50)));
    println!("   That Timeout is what `for x in rx` would have waited on FOREVER,");
    println!("   because one Sender is still alive in this scope. Drop it:");
    drop(keep);
    println!("   after drop(keep), recv() = {:?}", rx.recv());
    println!("   Disconnected is a different answer from Empty, and only the second");
    println!("   one is a reason to stop.");

    println!();
    println!("4. The rule, in one line");
    println!("   A receiver's loop ends when the LAST Sender drops. So a Sender you");
    println!("   keep \"just in case\" is a Sender that never lets the program exit,");
    println!("   and the symptom is a process that finishes its work and hangs.");
    println!("   Either drop it explicitly, or never bind it — pass clones and let");
    println!("   the original go out of scope with the block that made it.");

    println!();
    println!("5. Bounded channels, and what the bound is for");
    let (tx, rx) = mpsc::sync_channel::<u32>(2);
    println!("   sync_channel(2): try_send #1 {:?}", tx.try_send(1).is_ok());
    println!("                    try_send #2 {:?}", tx.try_send(2).is_ok());
    println!("                    try_send #3 {:?}   <- Full", tx.try_send(3).is_ok());
    println!("   drained: {:?} {:?}", rx.recv().unwrap(), rx.recv().unwrap());
    println!("   An unbounded channel with a fast producer and a slow consumer is a");
    println!("   memory leak with good manners. A bound turns that into");
    println!("   backpressure: `send` blocks, and the producer slows to the");
    println!("   consumer's speed instead of buffering the difference.");
}
```
<!-- /source -->

<!-- output:channels_kata -->
*Verified output of [`channels_kata.rs`](examples/channels_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Stage one: parse on a worker, collect in main
   0,5,2    -> Some([0, 5, 2])
   2,4,4    -> Some([2, 4, 4])
   3,3,3    -> Some([3, 3, 3])
   4,4,1    -> Some([4, 4, 1])
   5,3,0    -> Some([5, 3, 0])
   9,1,1    -> None
   x,2,2    -> None

2. Two stages, wired together
   totals   = [14, 19, 10]
   rejected = 2
   Each stage owns its Sender and drops it by finishing, which closes
   the next stage's loop in turn. The pipeline shuts down from the
   FRONT, one stage at a time, with nothing to co-ordinate.

3. The hang, demonstrated without hanging
   one value sent, the worker's Sender dropped, and a clone kept:
   recv()               = Ok(1)
   recv_timeout(50ms)   = Err(Timeout)
   That Timeout is what `for x in rx` would have waited on FOREVER,
   because one Sender is still alive in this scope. Drop it:
   after drop(keep), recv() = Err(RecvError)
   Disconnected is a different answer from Empty, and only the second
   one is a reason to stop.

4. The rule, in one line
   A receiver's loop ends when the LAST Sender drops. So a Sender you
   keep "just in case" is a Sender that never lets the program exit,
   and the symptom is a process that finishes its work and hangs.
   Either drop it explicitly, or never bind it — pass clones and let
   the original go out of scope with the block that made it.

5. Bounded channels, and what the bound is for
   sync_channel(2): try_send #1 true
                    try_send #2 true
                    try_send #3 false   <- Full
   drained: 1 2
   An unbounded channel with a fast producer and a slow consumer is a
   memory leak with good manners. A bound turns that into
   backpressure: `send` blocks, and the producer slows to the
   consumer's speed instead of buffering the difference.
```
<!-- /output -->

</details>

---

## See also

- [Spawning a thread](../spawning_a_thread/README.md) — where the other end of the channel usually is
- [Sharing across threads: `Arc`](../../18_Ownership/sharing_across_threads/README.md) — the alternative this page's last table is choosing against
- [Lock poisoning](../mutex_poisoning/README.md) — the failure mode a channel does not have
- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — what `send` is actually doing
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — `rx.iter()` is one, and it blocks

## Sources

[Channels ↗](https://doc.rust-lang.org/rust-by-example/std_misc/channels.html) in Rust by Example, and [`std::sync::mpsc` ↗](https://doc.rust-lang.org/std/sync/mpsc/index.html), whose module docs are where the disconnect semantics are actually specified.
