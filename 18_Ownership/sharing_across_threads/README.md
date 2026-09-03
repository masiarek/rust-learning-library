# Sharing across threads: `Arc`

**Level:** 201 · working knowledge

**One line:** `Arc<T>` is `Rc<T>` with an atomic counter, and the difference is not a performance note — it is the reason one of them compiles across a thread boundary and the other does not.

```rust
use std::sync::Arc;
use std::thread;

let roster = Arc::new(vec!["Ada".to_string(), "Ben".to_string()]);
let mine = Arc::clone(&roster);                       // one owner per thread
let names = thread::spawn(move || mine.len()).join().unwrap();
println!("{names}");                                  // 2
```

Everything [`Rc`](../reference_counting/README.md) does, this does: a count beside the value, `+1` on clone, `−1` on drop, freed at zero. One line of that description changes, and it changes what the compiler will let you write.

## The refusal, and what it names

Swap the `Arc` above for an `Rc` and the program stops building:

```text title="Abridged — real rustc output, without the closure-span diagram and the toolchain path"
error[E0277]: `Rc<Vec<String>>` cannot be sent between threads safely
   --> arc.rs:8:19
    |
    = help: within `{closure@arc.rs:8:19: 8:26}`, the trait `Send` is not implemented for `Rc<Vec<String>>`
note: required by a bound in `spawn`
    |
125 | pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    |        ----- required by a bound in this function
...
128 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`
```

Read the bottom half first. `thread::spawn` requires `F: Send + 'static`, the closure captured an `Rc`, and `Rc` is not [`Send`](../../12_Traits/marker_traits/README.md). The compiler names the *marker*, not the mistake — it will not say "use `Arc`" — so the error is only legible once you know that `Send` means "safe to move to another thread".

## What `atomic` is paying for

`count += 1` is three steps: read, add, write. A scheduler may run another thread between any two of them.

```text
A reads 0, B reads 0, A writes 1, B writes 1   ->  two increments, counter says 1
```

On an ordinary counter that is a wrong number. On a *reference* count it is a freed value that somebody still holds, which is a use-after-free reached without writing a line of `unsafe`. `Arc` closes it by making the three steps indivisible — one instruction the hardware will not split.

That is the whole difference between the two types, and it is why `Rc` across threads is *refused* rather than merely discouraged: the bug it would cause is not one the borrow checker could catch later.

## `Arc` grants the ownership; `Mutex` grants the write

`Arc<T>` derefs to `&T` and stops, exactly as `Rc` does — the same `E0596`, with the same `DerefMut` explanation, [shown on the `Rc` page](../reference_counting/README.md#shared-means-read-only). Sharing a counter is not the same as being allowed to change it.

```rust
let collected = Arc::new(Mutex::new(Vec::new()));
let mine = Arc::clone(&collected);
thread::spawn(move || mine.lock().unwrap().push(41));
```

Two wrappers, two jobs, and neither substitutes for the other. Drop the `Arc` and the closure has nothing to own; drop the `Mutex` and there is nothing to write through. The cost is that the borrow rule moves to run time: `lock()` blocks instead of failing to compile, so two live locks on one thread deadlock rather than erroring, and a panic while holding one [poisons it](../../09_Advanced/mutex_poisoning/README.md).

## One clone per thread, before the spawn

```rust
for id in 0..8 {
    let mine = Arc::clone(&collected);          // the line people forget
    handles.push(thread::spawn(move || mine.lock().unwrap().push(id)));
}
```

`move` takes the whole binding, so the first closure would consume the only `Arc` and the second iteration gets `E0382`. The clone count ends up equal to the thread count, which is the shape to recognise: an `Arc::clone` immediately above every `spawn`.

## When not to reach for it

**Scoped threads need no owner at all.** `thread::scope` guarantees its threads finish before it returns, so they may *borrow*:

```rust
let total: u32 = thread::scope(|s| {
    let workers: Vec<_> = chunks_in.chunks(2).map(|c| s.spawn(move || c.iter().sum::<u32>())).collect();
    workers.into_iter().map(|w| w.join().unwrap()).sum()
});
```

No `'static`, no `Arc`, no count — and `chunks_in` is still usable afterwards. Reach for this first. `Arc` is for threads that genuinely outlive the borrow.

**Single-threaded code should not pay for it.** `Arc` reads like the safe default and `Rc` like the optimisation, so the habit is to use `Arc` everywhere and stop thinking. Atomic operations are not free, and nothing warns: the profile is the only place it shows.

**It changes nothing else.** `Arc<T>` still hands out `&T`, so `T` supplies its own interior mutability or you get none. Two `Arc`s pointing at each other still leak, exactly as `Rc` does — [`Weak` is the answer here too](../reference_counting/README.md#the-one-leak-safe-rust-still-permits).

## If you are coming from another language

**Python.** You have been getting `Arc` for free and paying for it globally. CPython refcounts every object and the GIL is what stops two threads corrupting a count — the exact problem `Arc` solves, solved once for the whole interpreter. Three things change. The protection is **per type here, not global**: you choose `Rc` or `Arc` per value, and single-threaded code pays nothing. You get **real parallelism** for CPU work, which the GIL is precisely what denies. And Python's *data* was never protected the way its refcounts were — `lst.append(x)` survives by accident of the GIL, `x += 1` does not, and `threading.Lock` is what you already reach for; that is `Mutex`, except Rust's owns the data so there is no way to touch it without locking. Worth knowing that the free-threaded build (PEP 703) has to make those refcount updates safe by other means, which is the same problem stated again.

**ABAP.** The bridge is that there is almost nothing to bridge, and that is the useful part. `CALL FUNCTION … STARTING NEW TASK` runs in a *separate work process* with its own memory: data is copied out, results come back through a callback, and nothing is shared, so no lock is needed and none exists. ABAP's default answer to concurrency is "do not share" — closer to Rust's channels than to `Arc<Mutex<T>>`. Where ABAP does share, it looks much more familiar: a shared memory area (SHMA) is attached `ATTACH_FOR_READ` or `ATTACH_FOR_WRITE`, many readers or one writer, which is the borrow rule enforced at run time by an area lock. So `Arc<Mutex<T>>` is the SHMA case and `spawn` + channels is the aRFC case; the mistake to avoid is assuming a spawned Rust thread starts with a private copy the way a new task does.

**C++.** `shared_ptr<T>` is `Arc<T>`, and there is no `Rc` — the control block is *always* atomic, single-threaded code included, which is the cost Rust splits into two types and lets you decline. The sharper difference is what the count protects. `shared_ptr`'s refcount is thread-safe and its **pointee is not**: two threads writing `*p` is a data race, nothing stops you, and the code compiles. `Arc<T>` hands out `&T` only, so shared-and-mutable has to go through `Mutex` or an atomic. And `std::mutex` guards data by *convention* — it sits beside the thing it protects and nothing enforces the pairing — where `Mutex<T>` **contains** the value, so the only way to reach the data is to take the lock. That is the same move as `Arc` itself: a discipline you used to remember, turned into a type you cannot get wrong.

## Practice

**Three refusals, three fixes, and the one you can delete.** Predict each error before you compile it.

1. Move an `Rc` into `thread::spawn`. Name the trait in the error before you read it, then fix it with `Arc`.
2. Share an `Arc<Vec<u32>>` across four threads and `push` to it. Predict the error code, then fix it with `Arc<Mutex<T>>` and check that all four writes landed.
3. Delete the `Arc::clone` inside a spawn loop. Say which iteration fails and why, then put it back.
4. Count to `8 × 5000` twice: once with `fetch_add`, once with a `load` then a `store`. One total is arithmetic. Say what you can print about the other one that would still be true next run.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:sharing_across_threads_kata -->
*[`sharing_across_threads_kata.rs`](examples/sharing_across_threads_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three refusals, three fixes, and the one you can delete.
//!
//!   rustc --edition 2024 sharing_across_threads_kata.rs -o /tmp/satk && /tmp/satk
//!
//! Nothing prints from inside a spawned thread, and one number here is
//! deliberately NOT printed — see part 2.

use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::sync::{Arc, Mutex};
use std::thread;

const THREADS: usize = 8;
const PER_THREAD: u32 = 5_000;

fn main() {
    println!("Part 1 — three refusals, three fixes.\n");

    println!("  (a) `Rc` into thread::spawn");
    println!("      error[E0277]: `Rc<Vec<String>>` cannot be sent between threads safely");
    println!("      the trait `Send` is not implemented for `Rc<Vec<String>>`");
    println!("      Fix: Arc. Same API, same size, atomic count.");
    let roster = Arc::new(vec!["Ada".to_string(), "Ben".to_string()]);
    let mine = Arc::clone(&roster);
    let names = thread::spawn(move || mine.len()).join().unwrap();
    println!("      the thread saw {names} names\n");

    println!("  (b) pushing through a shared `Arc<Vec<u32>>`");
    println!("      error[E0596]: cannot borrow data in an `Arc` as mutable");
    println!("      Fix: Arc<Mutex<T>>. Arc grants the ownership, Mutex the write.");
    let collected = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for id in 0..4u32 {
        let mine = Arc::clone(&collected);
        handles.push(thread::spawn(move || mine.lock().unwrap().push(id)));
    }
    for h in handles {
        h.join().unwrap();
    }
    let mut rows = collected.lock().unwrap().clone();
    rows.sort(); // the order they arrived in is the scheduler's business
    println!("      four threads wrote {rows:?}\n");

    println!("  (c) forgetting the per-thread clone");
    println!("      error[E0382]: use of moved value: `shared`");
    println!("      The first `move` closure took the only Arc; the second had none.");
    println!("      Fix: one Arc::clone per spawn, made before the loop body ends.");
    let shared = Arc::new(41u32);
    let mut handles = Vec::new();
    for _ in 0..3 {
        let mine = Arc::clone(&shared); // <- the line that was missing
        handles.push(thread::spawn(move || *mine + 1));
    }
    let got: Vec<u32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("      three threads returned {got:?}");
    println!("      strong_count is back to {} now they have all joined\n", Arc::strong_count(&shared));

    println!("Part 2 — predict the two totals.\n");

    println!("  fetch_add is one indivisible step, so the answer is arithmetic:");
    let atomic = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let mine = Arc::clone(&atomic);
        handles.push(thread::spawn(move || {
            for _ in 0..PER_THREAD {
                mine.fetch_add(1, Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let expected = THREADS as u32 * PER_THREAD;
    println!("    {THREADS} x {PER_THREAD} = {expected}, and it counted {}", atomic.load(Relaxed));

    println!("\n  load-then-store is three steps, so a scheduler can split it.");
    println!("  Scripted on one thread, the interleaving looks like this:");
    let racy = AtomicU32::new(0);
    let a = racy.load(Relaxed);
    let b = racy.load(Relaxed);
    racy.store(a + 1, Relaxed);
    racy.store(b + 1, Relaxed);
    println!("    A reads {a}, B reads {b}, both write 1 -> counter says {}", racy.load(Relaxed));

    println!("\n  Run that across {THREADS} real threads and updates go missing:");
    let racy = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let mine = Arc::clone(&racy);
        handles.push(thread::spawn(move || {
            for _ in 0..PER_THREAD {
                let seen = mine.load(Relaxed);
                mine.store(seen + 1, Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let counted = racy.load(Relaxed);
    println!("    counted <= {expected}? {}", counted <= expected);
    println!("    The actual number is NOT printed here, on purpose: it differs");
    println!("    every run, so no answer key could hold it. That is the same");
    println!("    fact as the bug — a lost update is not reproducible, which is");
    println!("    why you cannot test your way to noticing one.");

    println!("\nPart 3 — the Arc you can delete.\n");
    let batches = vec![41u32, 17, 88, 5];
    let total: u32 = thread::scope(|s| {
        let workers: Vec<_> = batches
            .chunks(2)
            .map(|chunk| s.spawn(move || chunk.iter().sum::<u32>()))
            .collect();
        workers.into_iter().map(|w| w.join().unwrap()).sum()
    });
    println!("  thread::scope summed {batches:?} to {total} with no Arc at all,");
    println!("  and `batches` is still usable here: {}", batches.len());
    println!("\n  The rule to carry away: Arc is for threads that OUTLIVE the");
    println!("  borrow. A scoped thread cannot, so it needs no owner and no");
    println!("  count. Reach for the scope first and pay for the Arc second.");
}
```
<!-- /source -->

<!-- output:sharing_across_threads_kata -->
*Verified output of [`sharing_across_threads_kata.rs`](examples/sharing_across_threads_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Part 1 — three refusals, three fixes.

  (a) `Rc` into thread::spawn
      error[E0277]: `Rc<Vec<String>>` cannot be sent between threads safely
      the trait `Send` is not implemented for `Rc<Vec<String>>`
      Fix: Arc. Same API, same size, atomic count.
      the thread saw 2 names

  (b) pushing through a shared `Arc<Vec<u32>>`
      error[E0596]: cannot borrow data in an `Arc` as mutable
      Fix: Arc<Mutex<T>>. Arc grants the ownership, Mutex the write.
      four threads wrote [0, 1, 2, 3]

  (c) forgetting the per-thread clone
      error[E0382]: use of moved value: `shared`
      The first `move` closure took the only Arc; the second had none.
      Fix: one Arc::clone per spawn, made before the loop body ends.
      three threads returned [42, 42, 42]
      strong_count is back to 1 now they have all joined

Part 2 — predict the two totals.

  fetch_add is one indivisible step, so the answer is arithmetic:
    8 x 5000 = 40000, and it counted 40000

  load-then-store is three steps, so a scheduler can split it.
  Scripted on one thread, the interleaving looks like this:
    A reads 0, B reads 0, both write 1 -> counter says 1

  Run that across 8 real threads and updates go missing:
    counted <= 40000? true
    The actual number is NOT printed here, on purpose: it differs
    every run, so no answer key could hold it. That is the same
    fact as the bug — a lost update is not reproducible, which is
    why you cannot test your way to noticing one.

Part 3 — the Arc you can delete.

  thread::scope summed [41, 17, 88, 5] to 151 with no Arc at all,
  and `batches` is still usable here: 4

  The rule to carry away: Arc is for threads that OUTLIVE the
  borrow. A scoped thread cannot, so it needs no owner and no
  count. Reach for the scope first and pay for the Arc second.
```
<!-- /output -->

</details>

## The verified output

<!-- output:sharing_across_threads -->
*Verified output of [`sharing_across_threads.rs`](examples/sharing_across_threads.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The same count as `Rc`, and one owner per thread
   after Arc::new                     1
   four clones, no thread started yet 5
     thread 0 sees 3 names
     thread 1 sees 3 names
     thread 2 sees 3 names
     thread 3 sees 3 names
   after every thread joined          1
   The count is the same mechanism as `Rc`'s. Only the increment differs.

2. What `atomic` is paying for
   A read-modify-write is three steps, and a scheduler may split it.
   Here is that interleaving, executed in order on one thread:
     A reads 0, B reads 0, A writes 1, B writes 1
     two increments, and the counter says 1
   `fetch_add` is the same three steps made indivisible:
     two increments, and the counter says 2
   Lose one of those on a REFERENCE count and you free a live value.
   That is the bug `Arc` exists to make impossible, and the reason
   `Rc` is not merely discouraged across threads but refused.

3. `fetch_add` under real contention
   8 threads x 1000 increments = 8000
   Exact, every run. The load/store version above would not be.

4. `Arc` grants the ownership; `Mutex` grants the write
   8 threads wrote into one Vec: [0, 10, 20, 30, 40, 50, 60, 70]
   `Arc<T>` alone hands out `&T`, so it cannot be pushed to at all.
   Neither type substitutes for the other, and the borrow rule
   moves to run time: two live `lock()`s on one thread deadlock.

5. When no counting is needed at all
   thread::scope borrowed [41, 17, 88, 5] with no Arc: total 151
   A scoped thread cannot outlive the borrow, so the compiler needs
   no `'static` and you need no owner per thread. Reach for this
   first; `Arc` is for the threads that DO outlive the scope.

6. Same size, different promise
   size_of::<Rc<i32>>()  = 8
   size_of::<Arc<i32>>() = 8
   Identical values, identical layout. `Send` is the difference,
   and it is a promise the compiler tracks, not a byte in the value.
   What `Arc` does NOT change: it still hands out `&T`, and two
   `Arc`s pointing at each other still leak, exactly as `Rc` does.
```
<!-- /output -->

## See also

- [`Rc`: the clone that copies a pointer](../reference_counting/README.md) — the single-threaded original, and the counter this page makes atomic
- [Marker traits](../../12_Traits/marker_traits/README.md) — `Send` itself: `assert_send::<Arc<i32>>()` compiles and `assert_send::<Rc<i32>>()` does not, on two values of identical size
- [Mutex poisoning](../../09_Advanced/mutex_poisoning/README.md) — what happens to `Arc<Mutex<T>>` when a thread panics while holding the lock
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — the trait both counters are implementing
- [Ownership and moves](../ownership_and_moves/README.md) — the rule a thread boundary enforces most visibly
- [`Arc` ↗](https://doc.rust-lang.org/std/sync/struct.Arc.html) · [`Mutex` ↗](https://doc.rust-lang.org/std/sync/struct.Mutex.html) · [`thread::scope` ↗](https://doc.rust-lang.org/std/thread/fn.scope.html) · [`Send` ↗](https://doc.rust-lang.org/std/marker/trait.Send.html)

## Po polsku

`Arc<T>` (*atomically reference counted*) to `Rc<T>` z licznikiem atomowym. Różnica nie jest uwagą o wydajności — to powód, dla którego jeden z nich kompiluje się przez granicę wątku, a drugi nie.

Komunikat kompilatora wskazuje cechę `Send` (albo `Sync`) i to jest właściwe miejsce, żeby zacząć czytać. `Rc` nie jest `Send`, bo jego licznik to zwykła liczba: dwa wątki zwiększające ją jednocześnie mogą zgubić inkrementację, a to prowadzi do przedwczesnego zwolnienia pamięci. Atomowość jest dokładnie tym, za co płaci `Arc` — i płaci realnie, więc w kodzie jednowątkowym używa się `Rc`.

Podział ról, który warto zapamiętać jednym zdaniem: **`Arc` daje współwłasność, `Mutex` daje prawo zapisu.** Same `Arc<T>` nie pozwala pisać, bo współdzielone znaczy tylko do odczytu — stąd wszechobecne `Arc<Mutex<T>>`.

Kolejność też jest istotna: klonuje się `Arc` **przed** `spawn`, raz na wątek, i przenosi klon do domknięcia przez `move`. Próba sklonowania w środku domknięcia oznacza pożyczenie oryginału przez granicę wątku, czyli dokładnie ten błąd, którego się unikało.

**Szukaj po polsku:** wątki w Ruscie · `Arc Mutex` · cechy `Send` i `Sync` · `rust Arc vs Rc` · `E0277 cannot be sent between threads safely`
