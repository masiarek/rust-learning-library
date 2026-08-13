# What a panic costs

**Level:** 201 · working knowledge

**One line:** Unwinding is tidy about **memory** and careless about **work** — every destructor between the panic and the top runs, and everything the job had not finished simply never happens.

Choosing `.unwrap()` over a `None` arm is usually discussed as a style question. It is not: it is a decision about what your program *does*, and this page is about the thing it decides. A panic is not an error you handled and it is not a return value. It is the middle of a job, kept — with the memory cleaned up around it.

### If you are coming from another language

- **Python.** A panic is closest to an exception nobody catches: the traceback prints, `finally` blocks and context managers still run, the interpreter exits. The difference is what you *cannot* do with it. There is no `except Panic:` around your business logic — `catch_unwind` exists for FFI boundaries and test harnesses, not for control flow — so "I'll wrap it in a try later" is not a plan you can carry over.
- **ABAP.** It is a short dump, not a catchable exception: the work process stops, the `ROLLBACK` boundary decides what survives, and you read about it in ST22 afterwards. The parallel worth keeping is that in both languages the interesting question is not the message but **what had already been committed** when it fired.

---

## Where the panic points, and where it does not

```text
thread 'main' panicked at src/main.rs:16:24:
called `Option::unwrap()` on a `None` value
```

That location is a deliberate kindness. `Option::unwrap` is annotated [`#[track_caller]`](https://doc.rust-lang.org/reference/attributes/codegen.html#the-track_caller-attribute), so the line reported is **your** `unwrap` rather than a line inside `core/src/option.rs` — which is where a naive implementation would point, and which would tell you nothing. Step 1 below verifies this rather than asserting it: the program records its own `unwrap`'s line with `line!()` and compares it against the line the panic reported.

It also checks the question you will actually be asking at the time, and the answer is **no**. The location is not the line that handed over the `None`. Put an `unwrap` in a small helper called from thirty places and the panic names the helper; which of the thirty callers was wrong is in the backtrace, which is **off unless `RUST_BACKTRACE=1` is set**. A `None` arm would have needed neither, because the caller's context was still in scope.

## The job stops half-done

Five drink orders, and the third one is missing:

```text
with unwrap: panicked ("called `Option::unwrap()` on a `None` value") after ["water", "coffee"]
with unwrap_or: served ["water", "coffee", "(nothing)", "tea", "cola"]
```

Two glasses poured, three orders never looked at, and **the two poured glasses are still poured**. That is the cost, and it is invisible in a small example precisely because a small example has no side effects: scale the loop up and the rows already inserted are still inserted, the bytes already written to the socket are gone, the email already sent cannot be recalled. An `Option` returned to the caller is a question; a panic is an unfinished job plus a message.

## Unwinding cleans up your memory, not your work

This is the part that surprises people in both directions, so it is worth watching once:

```text
      two glasses in hand, about to unwrap a None
      Drop: the second glass is washed up
      Drop: the first glass is washed up
  caught: the bar always pours something
```

Every destructor between the panic and the catch runs, in reverse order of creation. **RAII holds during a panic** — locks are released, files are closed, buffers are freed, and a `MutexGuard` really does let go. Rust is not leaking anything.

And that is the whole of what it recovers. `Drop` knows how to give memory back; it has no idea that your function was three steps into a five-step operation. So the correct summary is the uncomfortable one: after a panic your *resources* are in a known state and your *data* is in whatever state the half-finished job left it.

Two footnotes that change the picture:

- **`panic = "abort"`.** Set that in a release profile (common for binaries, and the default for some embedded targets) and there is no unwinding at all — the process stops where it stands and **no destructor runs**. Code that quietly relies on `Drop` firing during a panic is relying on a build setting.
- **`Drop` panicking during an unwind** aborts the process. That is why a destructor should not be the place you do fallible work.

## In a thread, only that thread dies

```text
worker panicked, join() -> Err("the worker was promised a quorum")
main is still running, and prints this line
```

A panic unwinds **one** thread. `join()` hands the payload back as an `Err`, so the parent gets to decide — retry, log, give up — which is exactly the decision the `unwrap` inside the worker had taken away from it. Any `Mutex` the worker was holding when it died becomes **poisoned**, which is the mechanism by which one thread's panic reaches the others: they find out not by being killed but by having `lock()` return an `Err` that says *the data behind me is of unknown validity*.

## Exit code 101

Step 5 does not assert this one either — it re-runs this same binary with a flag that makes it panic for real, and reports the child's exit status:

```text
a child process that really panics exits Some(101)
```

**101, not 1.** std reserves it for an unhandled panic, so a supervisor, a shell script, or CI can distinguish *"the program failed and said so"* from *"the program broke"*. Worth knowing before you write `if status != 0` and treat every failure the same.

And note what that step reveals about the rest of this page: every other panic here was survived with [`catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html), which is why the demo can print its own crashes and still exit 0. That is not a pattern to copy. `catch_unwind` exists so a panic does not cross an FFI boundary into C, and so a test harness can report a failing test instead of dying with it. It cannot catch an abort, it says nothing about whether your data is still coherent, and using it as a `try`/`catch` around ordinary logic converts a loud bug into a quiet one.

## So when is the panic the right cost?

When continuing would be worse than stopping — the invariant is broken, and any answer you produced from here would be a wrong answer presented as a right one. That is a real category, and [`Option` vs `Result`](../option_vs_result/README.md) is the page about choosing it deliberately. What this page is for is the sentence to have in mind when you do: *if this fires, the work in flight is left where it fell, and the only thing I am guaranteed is that the destructors ran.*

---

## The verified output

<!-- output:what_a_panic_costs -->
*Verified output of [`what_a_panic_costs.rs`](examples/what_a_panic_costs.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
What a panic costs: unwinding is tidy about memory, not about work

──── Step 1: Where the panic points — and where it does not
  gulp(Some("coffee")) -> I love coffees!
  gulp(None) -> panicked: called `Option::unwrap()` on a `None` value
      reported in what_a_panic_costs.rs (line number withheld here: it moves whenever
      this file is edited, and the recorded output would go stale)
      is it the `unwrap` line?     true
      is it the caller's line?     false
      `Option::unwrap` is #[track_caller], so the location is YOUR
      unwrap rather than a line inside core/src/option.rs. It is still
      not the line that handed over the None — that one is only in the
      backtrace (RUST_BACKTRACE=1), which is off by default.

──── Step 2: A panic is not a return value: the job stops half-done
  with unwrap: panicked ("called `Option::unwrap()` on a `None` value") after ["water", "coffee"]
      Two glasses poured, three orders never looked at — and the two
      poured glasses are still poured. Anything already written to a
      file, a socket, or a database stays written.
  with unwrap_or: served ["water", "coffee", "(nothing)", "tea", "cola"]
      Same five orders, one answer each, the gap visible in the result.

──── Step 3: Unwinding cleans up your memory, not your work
      two glasses in hand, about to unwrap a None
      Drop: the second glass is washed up
      Drop: the first glass is washed up
  caught: the bar always pours something
      Both destructors ran, in reverse order, on the way out — RAII
      holds during a panic, which is why a lock is released and a file
      is closed. What does NOT happen is the rest of the function.
      (With `panic = "abort"` in the release profile, not even this:
      the process stops where it stands and no destructor runs.)

──── Step 4: In a thread, only that thread dies
  worker panicked, join() -> Err("the worker was promised a quorum")
  main is still running, and prints this line
      A panic unwinds one thread. `join` hands you the payload as an
      Err, so the parent decides — which is the choice the unwrap
      inside the worker had taken away. Any Mutex the worker held
      while it died is now poisoned.

──── Step 5: An uncaught panic is exit code 101, not 1
  a child process that really panics exits Some(101)
      101 is std's exit code for an unhandled panic — distinct from 1,
      so a supervisor can tell 'the program failed and said so' from
      'the program broke'. catch_unwind is how THIS demo survived its
      own panics, but it is not a try/catch: it exists for FFI and
      test harnesses, and it cannot catch an abort.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/what_a_panic_costs/examples/what_a_panic_costs.rs -o /tmp/wpc && /tmp/wpc
```

## See also

- [`Option` vs `Result`](../option_vs_result/README.md) — absence versus failure, and the combinators that keep you out of `unwrap`
- [Partial functions](../partial_functions/README.md) — where the `Option` came from in the first place; its step 3 catches a divide-by-zero panic the same way this page does
- [`std::panic::catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) and [`std::panic::set_hook`](https://doc.rust-lang.org/std/panic/fn.set_hook.html) — the two pieces this example is built on
- [The Rust Book, ch. 9.1 — *To `panic!` or Not to `panic!`*](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html) — the same decision from the other side
- [Rust by Example — `Option` & `unwrap`](https://doc.rust-lang.org/rust-by-example/std/option.html) — the source of the drink-serving shape used here (dual-licensed MIT / Apache-2.0)
