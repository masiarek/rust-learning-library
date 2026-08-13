# Lock poisoning

**Level:** 301 · deep dive

**One line:** `Mutex::lock()` returns a `Result` for exactly one reason — a thread panicked while holding the guard — and `.lock().unwrap()` is you deciding that this thread should die because that one did.

It is the most-typed `.unwrap()` in Rust, and the one most often defended with *"it can only fail if a thread panicked."* True, and the sentence stops one clause too early: **it can only fail if a thread panicked while the data was under its exclusive control.**

---

## A mutex protects an invariant, not bytes

The lock is not there to stop two threads touching the same memory at the same instant — that is the mechanism, not the purpose. It is there so that other threads only ever see the data in a state you consider *finished*.

Take a list that holds `(candidate, score)` as adjacent pairs. Its invariant is "the length is even". A writer holds the lock, pushes the candidate, and panics before pushing the score:

```rust
let mut guard = shared.lock().unwrap();
guard.push(3);                              // the candidate...
panic!("failed before writing the score");  // ...and the score never comes
```

The `Vec` is perfectly valid as a `Vec`. Every byte is where it should be. It is also *wrong*, in the only sense the program cares about — and the next thread to lock has no way to notice, because nothing about `[1, 5, 2, 4, 3]` looks broken.

So std records it. When a guard is dropped **during unwinding**, the lock sets a poison flag, and every later `lock()` returns `Err(PoisonError)`. That is the whole mechanism: not an error the mutex encountered, but a message from a thread that died mid-sentence.

## The three honest answers

`PoisonError` carries the guard, so nothing is lost — the choice is entirely about what you do next:

```rust
shared.lock().unwrap()                          // (a) die with it
shared.lock().unwrap_or_else(|e| e.into_inner())// (b) carry on, deliberately
shared.lock().map_err(|_| MyError::Corrupt)?    // (c) hand the choice to the caller
```

- **(a) is a real decision, not a formality.** It says: *the state behind this lock may be nonsense, and continuing is worse than stopping.* In an application that is often right and always defensible.
- **(b) says the opposite** — *I know a writer died in here and I am proceeding anyway.* That is a claim about your data. It is right when the invariant cannot actually have been broken (see below), or when you are about to repair it.
- **(c) is the library answer.** A panic inside your crate is a failure your caller cannot catch; deciding on their behalf that the process should end is the one version of this that is simply rude.

The tell for a misplaced (a) is a library whose docs never mention that it can panic, in a codebase where nothing else panics. Your caller's process now ends because of a thread they did not write.

## Poisoning is sticky, and clearing it is two steps

The flag stays set forever, in every thread. Since Rust 1.77 you can clear it — and the order matters more than the call:

```rust
{
    let mut data = shared.lock().unwrap_or_else(|e| e.into_inner());
    data.push(0);              // repair the invariant FIRST
}
shared.clear_poison();         // then retract the warning
```

`clear_poison()` clears the *flag*, not the data. Calling it first, or calling it because the `Err` was inconvenient, converts a loud problem into a silent one — which is the trade nobody ever means to make.

## What poisons, exactly

Mechanical rules, all three verified by the program below:

| Event | Poisons? |
|---|---|
| A thread panics while holding a `Mutex` guard | **yes** |
| A thread panics with no guard held | no |
| A thread panics holding an `RwLock` **read** guard | no |
| A thread panics holding an `RwLock` **write** guard | **yes** |

The `RwLock` split is the rule in miniature: only an exclusive guard can leave a half-written invariant, so only an exclusive guard poisons. Nothing is inspecting your data or judging whether it is *really* broken — the flag follows the guard, not the damage.

## When poisoning is noise

```rust
let mut n = hits.lock().unwrap();
*n += 1;
panic!("died after a complete increment");
```

A single `+= 1` on a `u64` is complete or it never ran; there is no half-state for a panic to leave behind. The lock poisons anyway, because std cannot know that. This is the case the `parking_lot` crate has in mind when it drops poisoning altogether, and it is worth being honest that a large share of real `Mutex`es are exactly this shape — a counter, a flag, a cache where a stale entry is survivable.

Which gives a better question than *"should I unwrap the lock?"*:

**Could a panic between two of my writes leave this data saying something untrue?** If yes, `.unwrap()` is the right instinct and you should say so in an `.expect("…")`. If no, the poison flag is telling you about a thread that died, not about your data, and `unwrap_or_else(|e| e.into_inner())` is the honest response — with a comment saying which of the two you decided.

### If you are coming from another language

- **Python.** `threading.Lock` has no equivalent. A thread that raises mid-update releases the lock in its `finally` and the next thread reads the half-written dict with nothing to warn it; the traceback appears on stderr and the *other* thread carries on computing with data nobody vouched for. Rust's poison flag is not protection — it is the notification Python never sends you.
- **ABAP.** The closest relative is not `ENQUEUE` but the update task: `CALL FUNCTION … IN UPDATE TASK` bundles the writes so that a failing update rolls the whole bundle back, and SM13 keeps the wreckage for you to look at. Both systems refuse to let a half-finished write pass silently — SAP by undoing it, Rust by *recording* it and making the next reader decide. The Rust one is weaker and cheaper: your data is still half-written, and all you are guaranteed is that nobody reads it unknowingly.

---

## The verified output

<!-- output:mutex_poisoning -->
*Verified output of [`mutex_poisoning.rs`](examples/mutex_poisoning.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: lock() returns a Result — and normally it is Ok
  lock().is_ok()        -> true
  is_poisoned()         -> false
  data                  -> [1, 5, 2, 4]
      The Result has exactly one possible cause: a previous holder
      panicked. Nothing else makes lock() fail.

──── Step 2: A holder panics: the invariant breaks and the lock remembers
  is_poisoned()         -> true
  lock()                -> Err(PoisonError)
  the data behind it    -> [1, 5, 2, 4, 3]
  invariant holds?      -> false
      The Vec is not corrupt in a memory sense — it is corrupt in the
      sense that matters: a candidate with no score. That is what the
      Err is warning the next thread about.

──── Step 3: Three honest responses, and the one you write by accident
  (a) .unwrap()             -> panicked: true
  (b) .unwrap_or_else(into_inner) -> [1, 5, 2, 4, 3]  consistent? false
  (c) map_err + ?           -> Err("the score list was left inconsistent by a panicking thread")
      (a) is the default everyone types. It is a real decision: this
      thread dies because a different one did. Fine in an application,
      rude in a library — the caller never got to choose.

──── Step 4: Poisoning is sticky, and clearable
  poisoned now?         -> true
  every later lock()    -> Err, forever, in every thread
  repaired to           -> [1, 5, 2, 4, 3, 0]  consistent? true
  after clear_poison()  -> poisoned? false, lock() ok? true
      clear_poison() only clears the FLAG. Fixing the data is your job,
      and doing it in the other order tells the next thread a lie.

──── Step 5: What actually poisons: the guard's Drop during unwinding
  panic while NOT holding -> poisoned? false
  RwLock, reader panicked -> poisoned? false
  RwLock, writer panicked -> poisoned? true
      Only an exclusive guard can leave a half-written invariant, so only
      an exclusive guard poisons. The rule is mechanical, not a judgement.

──── Step 6: When poisoning is noise
  counter after the panic -> 1, poisoned? true
      Nothing here can be half-done: one += 1 is complete or it never ran.
      The flag is still set, because std cannot know that. This is the case
      the parking_lot crate has in mind when it drops poisoning entirely.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 09_Advanced/mutex_poisoning/examples/mutex_poisoning.rs -o /tmp/mp && /tmp/mp
```

## Traps

- **`.lock().unwrap()` in a library.** You have decided your caller's process should end. Return the error or recover explicitly.
- **`clear_poison()` before repairing the data.** The flag is the only thing telling anyone the invariant may be broken; clearing it first just deletes the warning.
- **Treating `PoisonError` as data loss.** Nothing is lost — `into_inner()` and `get_ref()` both hand you the value. The `Err` is advice, not a wall.
- **Holding a guard across an `.await` or a long call.** Not a poisoning bug, but it is how a panic elsewhere ends up happening *inside* your critical section in the first place.
- **Assuming a panicking reader poisons an `RwLock`.** It does not, and code that "defensively" handles that case is handling something that cannot happen.

## See also

- [`unwrap`: the bet you are making](../../01_Foundations/what_a_panic_costs/README.md) — the general form of the decision this page is one instance of
- [`unwrap_or`](../../01_Foundations/unwrap_or/README.md) — and `unwrap_or_else`, which is what `|e| e.into_inner()` is
- [`Option` vs `Result`](../../01_Foundations/option_vs_result/README.md) — why `lock()` returns a `Result` rather than an `Option`: the caller can absolutely ask *why not?*
- [`std::sync::Mutex::lock`](https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.lock) · [`PoisonError`](https://doc.rust-lang.org/std/sync/struct.PoisonError.html) · [`clear_poison`](https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.clear_poison)
- [The Rust Book, ch. 16.3 — Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
