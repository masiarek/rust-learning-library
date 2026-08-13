# `expect`: writing down the proof

**Level:** 201 · working knowledge

**One line:** Mechanically it is `unwrap` with a sentence attached — but the sentence is not an error message, it is a claim about why this could not fail, and discovering that you cannot write it is the *finding*, not an inconvenience.

This page is about the message. What the panic itself does — where it points, what unwinding restores and what it leaves half-done, why the exit code is 101 — is [its own page](../what_a_panic_costs/README.md).

---

## Four panics, two of them useful

```text
None.unwrap()            -> called `Option::unwrap()` on a `None` value
None.expect("…")         -> the config should list a quorum
Err(e).unwrap()          -> called `Result::unwrap()` on an `Err` value: "invalid digit found in string"
Err(e).expect("…")       -> the quorum line should be a number: "invalid digit found in string"
```

`unwrap`'s message is the standard library's, and it can only ever describe the *shape* of what happened, because that is all it knows. Yours describes the situation.

Note what the `Result` form does: **`{your sentence}: {the error, Debug-printed}`**. The claim and the cause both survive, so `expect` never costs you the error — unlike every fallback in the [`unwrap_or` family](../unwrap_or/README.md), which drops it. (`expect_err` is the mirror, for when the surprise is that it *worked*: it panics with your sentence and the `Ok` value.)

That printing is also why `expect` on a `Result` needs `E: Debug`, a bound `unwrap_or` does not have:

```text
error[E0277]: `Opaque` doesn't implement `Debug`
  |
4 |     let _ = r.expect("the config should parse");
  |               ^^^^^^ the trait `Debug` is not implemented for `Opaque`
  = note: add `#[derive(Debug)]` to `Opaque` or manually `impl Debug for Opaque`
note: required by a bound in `Result::<T, E>::expect`
```

The bound is a fact about what the message is *made of*, not an arbitrary requirement — which is the argument for deriving `Debug` on error types by reflex.

## Say what should be true, not what went wrong

Three wordings, the same bug:

```text
expect("failed to get quorum")   -> PANIC: failed to get quorum
expect("unwrap failed")          -> PANIC: unwrap failed
expect("the [election] section should set a quorum; the loader fills it in from defaults")
```

Only the third helps, and the standard library's own guidance says why: an `expect` message should describe **the reason you expected a value to be there**. Written that way, the panic line reads as a claim — and since the program stopped, the reader immediately knows *which* claim was false and *who* was supposed to make it true. "Failed to get X" only restates that the program stopped, which the word `panicked` already said.

A useful test when writing one: **name the guarantor.** "The loader fills it in", "we returned early if the slice was empty", "this is a literal in the source above". If no such phrase exists, keep reading.

## If you cannot write the sentence, you do not have the proof

```rust
config.iter().find(|(k, _)| *k == "quorum")
    .expect("the config should have a quorum")   // should — according to whom?
```

Nobody proved that. A *user* typed the file, and a typo'd key is a thing users do — so the sentence is a hope wearing the grammar of a proof, and the program dies on valid-looking input with a message that blames nothing.

The signal is that specific: you can write words, but you cannot name a guarantor. That is the moment to change the return type instead.

```rust
fn quorum(config: &[(&str, &str)]) -> Result<u32, String> {
    let (_, raw) = config.iter().find(|(k, _)| *k == "quorum")
        .ok_or_else(|| "no `quorum` key in [election]".to_string())?;
    raw.parse().map_err(|e| format!("quorum = {raw:?} is not a number ({e})"))
}
```

Same information, delivered to the same reader, plus the offending value — and the caller decides whether it is fatal. The rule of thumb: **`expect` is for what you have proved; `Result` is for what you have merely hoped.**

This is also the review heuristic that makes the method worth using at all. `.unwrap()` in a diff means *no proof offered*; `.expect("…")` means *a proof is offered, now check it*. The second is reviewable and the first is not, which is why `expect` beats `unwrap` everywhere the crash is genuinely defensible.

## Where it is exactly right

```rust
let max: u8 = "5".parse().expect("the literal \"5\" parses as a u8");
let mid = median(&scores).expect("median returns Some for a non-empty slice, and this one has 3");
let guard = shared.lock().expect("no thread panics while holding this lock");
```

Each sentence is *checkable by a reader*: one points at a literal in the source, one at an early return a few lines up, one at a claim about the whole program a reviewer can go and test. That is the bar — not "this feels safe" but "here is the argument, go and check it". ([Lock poisoning](../../09_Advanced/mutex_poisoning/README.md) is the case where that third claim is worth thinking about hardest.)

Tests are the fourth case, and the easiest: there the panic *is* the failure report, and the message is what the runner prints.

## The message is an argument, so it is built even when nothing fails

```text
expect(&proof(name))                    total 8, proof() ran 3 times
unwrap_or_else(|| panic!(proof(name)))  total 8, proof() ran 0 times
```

`expect` takes a `&str` — an ordinary, eagerly-evaluated argument, exactly like [`unwrap_or`'s default](../unwrap_or/README.md#the-eager-trap). So a formatted message is built, allocated and dropped on **every successful call**, in the hot loop, forever, to be read never.

A plain string literal costs nothing, so prefer one. When the message genuinely needs a runtime value in it, the standard library's own answer is to move the whole thing into the sad path:

```rust
score.unwrap_or_else(|| panic!("every ballot should score {name}; the loader pads missing columns"))
```

## If you are coming from another language

- **Python** — `assert cond, "the loader should have filled this in"` is the same idiom, including the message style. One difference matters: `python -O` **strips asserts entirely**, which is why seasoned Python code avoids relying on them for anything load-bearing. `expect` is never stripped — no build mode removes it — so the reflex to distrust the assertion does not transfer, and the sentence you write really will be the one someone reads.
- **ABAP** — the pair is `ASSERT` and a short dump. What lands in ST22 is the text you chose, read months later by someone who was not you, under time pressure — which is exactly the audience an `expect` message is written for, and the reason "should" wording beats "failed to".
- **JavaScript / Java** — `throw new Error("…")` carries a message the same way, but it is *catchable* by anything up the stack, so the sentence tends to drift toward the user. An `expect` message is addressed to a programmer, and can say things like "the loader fills this in from defaults" that you would never show an end user.

---

## The verified output

<!-- output:expect -->
*Verified output of [`expect.rs`](examples/expect.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: unwrap's message is the standard library's; expect's is yours
  None.unwrap()                      -> PANIC: called `Option::unwrap()` on a `None` value
  None.expect("…")                   -> PANIC: the config should list a quorum
  Err(e).unwrap()                    -> PANIC: called `Result::unwrap()` on an `Err` value: "invalid digit found in string"
  Err(e).expect("…")                 -> PANIC: the quorum line should be a number: "invalid digit found in string"
  Ok(5).expect_err("…")              -> PANIC: no quorum should parse from an empty file: 5
      Four panics, and only two of them tell you anything. Note what the
      Result form does: `{your sentence}: {the error, Debug-printed}`, so
      the claim AND the cause survive — expect never costs you the error.
      expect_err is the mirror, for when the surprise is that it worked.

──── Step 2: The panic names your line, not the standard library's
  panicked at expect.rs:89
  the ballot file should have been validated by now
      That location is the `expect` call in THIS file — not a line inside
      core/src/option.rs, which is where the panic is physically raised.
      So the two halves of a good panic report come from two different
      places: the address from the attribute on the method, the meaning
      from the sentence you wrote.

──── Step 3: Say what SHOULD be true, not what went wrong
  expect("failed to get quorum")     -> PANIC: failed to get quorum
  expect("unwrap failed")            -> PANIC: unwrap failed
  expect("[election] should set…")   -> PANIC: the [election] section should set a quorum; the loader fills it in from defaults
      Same bug, three panics, one of them useful. The standard library's
      own guidance is to describe the reason you expected a value — so the
      line reads as a CLAIM, and a reader who sees it knows both what was
      supposed to hold and who was supposed to make it hold. 'Failed to
      get X' only restates that the program stopped, which the word PANIC
      already said.

──── Step 4: If you cannot write the sentence, you do not have the proof
  well-formed config:
    by expect -> 50
    by Result -> Ok(50)
  config with a typo'd key (a thing users do):
    by expect -> PANIC: the config should have a quorum
    by Result -> Err("no `quorum` key in [election]")
      Read the first message again: 'the config SHOULD have a quorum' —
      should according to whom? Nobody proved that; a user typed the file.
      The sentence is a hope, and the tell is that you cannot name who
      guaranteed it. That is the signal to return Result: the second form
      says the same thing to the same reader, names the offending value,
      and lets the caller decide whether it is fatal.

──── Step 5: Where expect is exactly right: a proof the compiler cannot check
  literal in the source          -> 5
  invariant established locally  -> 3
  a lock that is never poisoned  -> 3 scores
      All three sentences are checkable by a reader: one points at a
      literal, one at an early return four lines up, one at a claim about
      the whole program that a reviewer can go and test. That is the bar —
      not 'this feels safe' but 'here is the argument, go and check it'.
      A test is the fourth case, where the panic IS the failure report.

──── Step 6: The message is an argument, so it is built even when nothing fails
  expect(&proof(name))                    total 8, proof() ran 3 times
  unwrap_or_else(|| panic!(proof(name)))  total 8, proof() ran 0 times
      `expect` takes a &str, which is an ordinary eagerly-evaluated
      argument — exactly like unwrap_or's default. A formatted message is
      therefore built, allocated, and dropped on every SUCCESSFUL call, in
      the hot loop, forever. A plain literal costs nothing, so write one;
      when the message genuinely needs the value in it, the standard
      library's own answer is unwrap_or_else with a panic! inside.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/expect/examples/expect.rs -o /tmp/ex && /tmp/ex
```

## See also

- [What a panic costs](../what_a_panic_costs/README.md) — what the crash this method chooses actually does
- [`unwrap_or`](../unwrap_or/README.md) — the answer when the missing value is not a bug, and the eager-argument rule this page inherits
- [`unwrap_or_else`](../unwrap_or_else/README.md) — where `panic!` goes when the message needs a runtime value
- [`Option` vs `Result`](../option_vs_result/README.md) — the decision the "cannot write the sentence" tell sends you back to
- [`Option::expect`](https://doc.rust-lang.org/std/option/enum.Option.html#method.expect) · [`Result::expect`](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect) · [`expect_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect_err)
