# Spawning a thread

**Level:** 201 → 301 · working knowledge

**One line:** `thread::spawn` starts a thread and hands back a `JoinHandle<T>`; `join` is the only way to get the `T` back, and it returns a `Result` because the thread may have panicked instead of finishing.

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| (1..=100).sum::<u32>());
    println!("{}", handle.join().unwrap());   // 5050
}
```

## `move` is usually not optional

```rust
use std::thread;

fn main() {
    let threshold = 4u32;
    let h = thread::spawn(move || threshold * 2);
    println!("{}", h.join().unwrap());   // 8
}
```

Without `move`, the closure borrows `threshold`, and the compiler refuses:

```text
error[E0373]: closure may outlive the current function, but it borrows `threshold`, which is owned by the current function
 --> e373.rs:4:27
  |
4 |     let h = thread::spawn(|| threshold * 2);
  |                           ^^ --------- `threshold` is borrowed here
  |                           |
  |                           may outlive borrowed value `threshold`
  |
note: function requires argument type to outlive `'static`
```

That `'static` bound is the whole story: `spawn` cannot know when the thread finishes, so it will not accept a closure holding a borrow of anything that might end sooner. `move` transfers ownership instead — and then a second thread cannot have the same value, which is the moment most people reach for `Arc`.

## `scope` is the other answer, and usually the better one

```rust
use std::thread;

fn main() {
    let rows = vec![1u32, 2, 3, 4];
    let (sum, max) = thread::scope(|s| {
        let a = s.spawn(|| rows.iter().sum::<u32>());
        let b = s.spawn(|| rows.iter().copied().max().unwrap_or(0));
        (a.join().unwrap(), b.join().unwrap())
    });
    println!("{sum} {max}");   // 10 4
}
```

Both closures **borrow** `rows` — no `Arc`, no clone. `scope` cannot return until every thread it started has finished, so the borrow provably ends first and the lifetime works out. Stable since 1.63, and the right default for a fan-out over data you already have.

| Reach for | When |
|---|---|
| [`thread::scope` ↗](https://doc.rust-lang.org/std/thread/fn.scope.html) | the data is on this stack and the work finishes here |
| [`Arc<T>` ↗](https://doc.rust-lang.org/std/sync/struct.Arc.html) | the thread outlives this function, or the value has no single owner — read-only sharing needs nothing more |
| `Arc<Mutex<T>>` | …and they also write. Every reader now pays for the writers, so ask whether a channel would do |

## The trap: printing from inside the thread

Threads finish in whatever order the scheduler decides, so `println!` from four threads interleaves differently on every run — and on a test's output, that is a flake nobody can reproduce. **Do the work in the thread, return the value, and print on the joining side.** Joining in a fixed order is what makes a parallel program's report deterministic; the parallelism is unaffected, because all four were already running.

## A panicking thread does not take the process with it

```rust
match handle.join() {
    Ok(v) => println!("returned {v}"),
    Err(_) => println!("the thread panicked"),
}
```

`join` returns `Err(Box<dyn Any + Send>)` — the panic payload, usually a `String` you can downcast if you care. The message still goes to stderr, and `main` carries on. A thread you never join is a panic you never hear about, which is one more argument for joining everything.

And if `main` returns while a spawned thread is still running, the process exits and takes the thread with it, mid-work. There is no "wait for stragglers" at the end; `scope` exists partly to make that impossible.

## If you are coming from another language

- **Python.** `threading.Thread(target=f).start()` / `.join()` is the same shape, and then everything diverges. Python's threads share every object freely and the GIL makes most data races invisible-but-real; Rust's compiler will not let a thread touch anything it has not been given, and `Send`/`Sync` are checked rather than hoped for. The practical consequence: the Python habit of "just use a global and a `threading.Lock`" does not translate, and the Rust version of `concurrent.futures.ThreadPoolExecutor.map` is `thread::scope` plus a `spawn` per chunk — with real parallelism, because there is no GIL. `join()` returning the value is the other difference: Python's returns nothing and you reach for a `Future` or a queue.
- **ABAP.** The nearest thing is `CALL FUNCTION … STARTING NEW TASK`, with `PERFORM … ON END OF TASK` as the callback and `RECEIVE RESULTS` as `join`. The correspondence is closer than it looks: aRFC work processes share nothing, so results come back through the RECEIVE rather than through memory — which is exactly the discipline Rust enforces, and exactly why an ABAP developer's instinct here is already right. Two differences worth naming: Rust threads share an address space, so `Arc<Mutex<T>>` is possible where an ABAP task's memory is genuinely separate; and there is no equivalent of the resource check (`sy-subrc = 1` when no work process is free), because a thread is cheap.
- **Java / C#.** `Thread`/`Task` and `join`/`await`, with `ExecutorService.submit` returning a `Future` the way `spawn` returns a `JoinHandle`. What is new is that the compiler checks what may cross the boundary: Java lets you close over a mutable field and find out at run time.
- **Go.** `go f()` is `spawn` with no handle at all — the value comes back through a channel, always. Rust gives you both, and `scope` is `sync.WaitGroup` with the waiting made mandatory.

---

## The verified output

<!-- output:spawning_a_thread -->
*Verified output of [`spawning_a_thread.rs`](examples/spawning_a_thread.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. spawn takes a closure and hands back a handle
   candidate A total, computed on another thread: 24
   `spawn` returns a JoinHandle<T>, and `join` is the ONLY way to
   get the T back. It also returns a Result, because the thread may
   have panicked instead of finishing.

2. Four threads, and why the output is still in order
   A: 24
   B: 22
   C: 20
   Nothing was printed FROM a thread. Interleaved println! output is
   the classic non-deterministic test failure — do the work in the
   thread, return the value, and print on the joining side.

3. `move` is usually not optional
   ballots with A >= 4: 4
   Without `move`, the closure borrows `threshold` — and the compiler
   refuses, because it cannot prove the borrow outlives the thread:
   E0373, "closure may outlive the current function, but it borrows
   `threshold`, which is owned by the current function". `spawn`
   requires a `'static` closure for exactly that reason.

4. A panicking thread does not take the process with it
   the thread panicked; join() gave Err and main is fine
   The panic message would normally go to stderr — suppressed here
   so this example's transcript stays clean. `join` returns
   Err(Box<dyn Any + Send>), which is the panic payload — usually a
   String you can downcast if you care.

5. `scope` borrows what `spawn` cannot
   C's scores [0, 1, 2, 3, 5, 4, 5, 0] -> sum 20, max 5
   Both closures BORROW `local`, with no Arc and no clone. `scope`
   guarantees every thread it started has finished before it returns,
   so the borrow cannot outlive the data. Stable since 1.63, and the
   right default for fan-out over data you already have.
```
<!-- /output -->

## Practice

**The same fan-out three ways.** Total three candidates' scores over eight ballots, in parallel: sequentially first for an answer to check against, then with `spawn` + `Arc`, then with `thread::scope`. All three should agree.

Then write the version with neither `move` nor `Arc` and read `E0373` — including the `note:` line, which says what the real constraint is. Say what `move` alone would have done to a second thread wanting the same data, and why the `scope` version needs `move` on its closures anyway even though nothing is moved.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:spawning_a_thread_kata -->
*[`spawning_a_thread_kata.rs`](examples/spawning_a_thread_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the same fan-out three ways, and the one that needs no Arc.
//!
//!   rustc --edition 2024 spawning_a_thread_kata.rs -o /tmp/spk && /tmp/spk

use std::sync::Arc;
use std::thread;

/// Eight ballots, three candidates. The job: a per-candidate total, in parallel.
fn ballots() -> Vec<[u32; 3]> {
    vec![
        [5, 3, 0], [4, 4, 1], [0, 5, 2], [3, 3, 3],
        [5, 0, 5], [2, 4, 4], [1, 1, 5], [4, 2, 0],
    ]
}

fn main() {
    let rows = ballots();

    println!("1. Sequential, for the answer to check against");
    let expected: Vec<u32> = (0..3).map(|i| rows.iter().map(|b| b[i]).sum()).collect();
    println!("   totals = {expected:?}");

    println!();
    println!("2. spawn + Arc: the version that compiles first for most people");
    let shared = Arc::new(rows.clone());
    let mut handles = Vec::new();
    for i in 0..3 {
        let shared = Arc::clone(&shared);
        handles.push(thread::spawn(move || shared.iter().map(|b| b[i]).sum::<u32>()));
    }
    let with_arc: Vec<u32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("   totals = {with_arc:?}   matches: {}", with_arc == expected);
    println!("   Three Arc clones, one refcount, and a `move` per closure. The Arc");
    println!("   is here for exactly one reason: `spawn` needs a 'static closure,");
    println!("   and a borrow of a local is not 'static.");

    println!();
    println!("3. scope: the same thing with no Arc and no clone");
    let rows_ref = &rows;   // a shared reference is Copy, so `move` copies IT
    let with_scope: Vec<u32> = thread::scope(|s| {
        let handles: Vec<_> = (0..3)
            .map(|i| s.spawn(move || rows_ref.iter().map(|b| b[i]).sum::<u32>()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    println!("   totals = {with_scope:?}   matches: {}", with_scope == expected);
    println!("   The closures BORROW `rows`. `scope` cannot return until every");
    println!("   thread it started has finished, so the borrow provably ends first");
    println!("   — which is the whole reason the lifetime works out.");
    println!("   The `move` on each closure is about `i`, not about the data: a");
    println!("   loop variable is owned by the loop, so it has to be copied in, and");
    println!("   `move` then copies the shared REFERENCE rather than the Vec.");

    println!();
    println!("4. The version that does not compile, and what rustc says");
    println!("   let h = thread::spawn(|| rows.len());   // no `move`");
    println!("   E0373: \"closure may outlive the current function, but it borrows");
    println!("   `rows`, which is owned by the current function\", with a note that");
    println!("   \"function requires argument type to outlive `'static`\" and the");
    println!("   help offering `move`. Adding `move` compiles — and MOVES rows into");
    println!("   the first thread, so a second one cannot have it. That is the");
    println!("   moment most people reach for Arc; scope is the other answer.");

    println!();
    println!("5. Which to reach for");
    println!("   scope        the data is on this stack and the work finishes here.");
    println!("                Cheapest, and the borrow checker still helps.");
    println!("   Arc          the thread outlives this function, or the value has");
    println!("                no single owner. Read-only sharing needs nothing more.");
    println!("   Arc<Mutex>   ...and they also WRITE. Every reader now pays for the");
    println!("                writers, so ask whether a channel would do instead.");
    println!("   The order matters: each rung costs something the one above does");
    println!("   not, and a fan-out over data you already have needs only the first.");
}
```
<!-- /source -->

<!-- output:spawning_a_thread_kata -->
*Verified output of [`spawning_a_thread_kata.rs`](examples/spawning_a_thread_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Sequential, for the answer to check against
   totals = [24, 22, 20]

2. spawn + Arc: the version that compiles first for most people
   totals = [24, 22, 20]   matches: true
   Three Arc clones, one refcount, and a `move` per closure. The Arc
   is here for exactly one reason: `spawn` needs a 'static closure,
   and a borrow of a local is not 'static.

3. scope: the same thing with no Arc and no clone
   totals = [24, 22, 20]   matches: true
   The closures BORROW `rows`. `scope` cannot return until every
   thread it started has finished, so the borrow provably ends first
   — which is the whole reason the lifetime works out.
   The `move` on each closure is about `i`, not about the data: a
   loop variable is owned by the loop, so it has to be copied in, and
   `move` then copies the shared REFERENCE rather than the Vec.

4. The version that does not compile, and what rustc says
   let h = thread::spawn(|| rows.len());   // no `move`
   E0373: "closure may outlive the current function, but it borrows
   `rows`, which is owned by the current function", with a note that
   "function requires argument type to outlive `'static`" and the
   help offering `move`. Adding `move` compiles — and MOVES rows into
   the first thread, so a second one cannot have it. That is the
   moment most people reach for Arc; scope is the other answer.

5. Which to reach for
   scope        the data is on this stack and the work finishes here.
                Cheapest, and the borrow checker still helps.
   Arc          the thread outlives this function, or the value has
                no single owner. Read-only sharing needs nothing more.
   Arc<Mutex>   ...and they also WRITE. Every reader now pays for the
                writers, so ask whether a channel would do instead.
   The order matters: each rung costs something the one above does
   not, and a fan-out over data you already have needs only the first.
```
<!-- /output -->

</details>

---

## See also

- [Channels](../channels/README.md) — handing a value over instead of sharing one
- [Sharing across threads: `Arc`](../../18_Ownership/sharing_across_threads/README.md) — the rung above `scope`, and the refusals on the way
- [Lock poisoning](../mutex_poisoning/README.md) — what happens to a `Mutex` when one of these threads panics
- [The `move` keyword](../../23_Closures/the_move_keyword/README.md) — what it moves, and what it does not
- [Marker traits](../../12_Traits/marker_traits/README.md) — `Send` and `Sync`, the two auto traits deciding what may cross

## Sources

[Threads ↗](https://doc.rust-lang.org/rust-by-example/std_misc/threads.html) in Rust by Example; [`std::thread`  ↗](https://doc.rust-lang.org/std/thread/index.html) and [`std::thread::scope` ↗](https://doc.rust-lang.org/std/thread/fn.scope.html). The `E0373` transcript is a real compile of the five-line program above it.

## Po polsku

`thread::spawn` przyjmuje domknięcie (*closure*) i oddaje `JoinHandle<T>`; `join()` to jedyna droga, żeby odebrać `T`, i zwraca `Result`, bo wątek mógł zamiast wyniku spanikować. Prawie zawsze trzeba dopisać `move`, i warto wiedzieć dlaczego, bo komunikat `E0373` mówi to wprost: „closure may outlive the current function”. `spawn` nie wie, kiedy wątek się skończy, więc żąda domknięcia o statycznym czasie życia (`'static`), a pożyczenie zmiennej lokalnej takiego czasu życia nie ma. `move` przenosi własność do wątku — i od razu rodzi następny problem: skoro wartość jest już w pierwszym wątku, drugi jej nie dostanie. To ten moment, w którym większość ludzi sięga po `Arc`.

I tu uwaga, która dla polskiego czytelnika jest na tej stronie chyba najważniejsza: `Arc` **nie jest pierwszą odpowiedzią**. `thread::scope` jest stabilne dopiero od wersji 1.63 (2022), a większość polskich tutoriali i tłumaczeń jest starsza, więc uczy odruchu `Arc::new` plus `Arc::clone` w pętli jako jedynego wyjścia. Tymczasem wewnątrz `scope` domknięcia po prostu **pożyczają** dane — bez `Arc`, bez `clone` — bo `scope` nie może wrócić, zanim nie skończy się każdy uruchomiony w nim wątek, więc pożyczenie musi się zamknąć przed danymi i kompilator to widzi. Drabinka ma trzy szczeble i każdy kosztuje coś, czego poprzedni nie kosztował: `scope`, gdy dane leżą na tym stosie, `Arc`, gdy wątek przeżyje funkcję, `Arc<Mutex<T>>` dopiero wtedy, gdy wątki także piszą. W wersji ze `scope` `move` na domknięciu bywa nadal potrzebne, ale dotyczy licznika pętli `i`, a nie danych — kopiuje referencję współdzieloną, nie wektor.

Na koniec dwie rzeczy o kończeniu wątku, które w polskich materiałach rzadko stoją obok siebie. Po pierwsze: nie wypisuj niczego **z wnętrza** wątku. Kolejność ustala planista systemu, więc `println!` z czterech wątków przeplata się inaczej przy każdym uruchomieniu, a w teście daje to niedeterministyczną porażkę, której nikt nie odtworzy — policz w wątku, zwróć wartość, wypisz po stronie `join`. Równoległość na tym nic nie traci, bo wszystkie wątki i tak już biegły. Po drugie: panika w wątku **nie zabija procesu** (`join()` zwraca wtedy `Err(Box<dyn Any + Send>)` z ładunkiem paniki), ale wątek, którego nikt nie odbierze przez `join`, to panika, o której się nigdy nie dowiesz. I odwrotnie niż w Javie, gdzie wątek niebędący demonem trzyma JVM przy życiu: kiedy `main` wraca, proces kończy się razem z wciąż pracującymi wątkami, w połowie roboty. `scope` istnieje między innymi po to, żeby to było niemożliwe.

**Szukaj po polsku:** wątki w Ruscie · domknięcie z `move` · statyczny czas życia · `rust thread::scope vs Arc` · `rust E0373 closure may outlive the current function`
