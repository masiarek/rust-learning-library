# An enum as a state machine

**Level:** 201 · working knowledge

**One line:** Match on a *tuple* of two enums and the compiler enumerates the whole transition table for you — every state crossed with every event, and it names the cells you left out.

```rust
fn step(state: State, event: Event) -> State {
    match (state, event) {
        (Blank, Mark) => Marked,
        (Marked, Submit) => Cast,
        // ...
    }
}
```

Two enums — one for what the thing *is*, one for what just *happened* — and a function from the pair to the next state. That is the whole pattern, and the reason to write it this way rather than with `if` is in the next section.

---

## The compiler checks the table, not just the states

A `match` on `(State, Event)` is exhaustive over the **cross product**. Three states and three events means nine cells, and leaving one out is a build error that says which:

```text
error[E0004]: non-exhaustive patterns: `(State::Idle, Event::Stop)` and
              `(State::Running, Event::Start)` not covered
 --> sm.rs:6:11
  |
6 |     match (state, event) {
  |           ^^^^^^^^^^^^^^ patterns `(State::Idle, Event::Stop)` and
  |                          `(State::Running, Event::Start)` not covered
```

That is a design review you get for free. *What happens if a submit arrives before anything is marked?* is a question the compiler will not let you leave unanswered — and "nothing happens" is a perfectly good answer, it just has to be written down.

## The one line that throws it away

```rust
match (state, event) {
    (Blank, Mark) => Marked,
    (Marked, Submit) => Cast,
    _ => state,               // <- the audit ends here
}
```

Every table you find in the wild ends this way, and it converts the compiler's nine-cell checklist into two cells and a shrug. Add a fourth event and no build breaks; the machine simply ignores it. This is the same trade as [`if let`](../../17_Option_and_Result/if_let/README.md) and the same accident as [a typo becoming a binding](../a_typo_becomes_a_binding/README.md) — exhaustiveness is not something an enum gives you, it is something a `match` can decline.

The middle position, when nine arms really are too many, is a **partial** wildcard:

```rust
(Cast, _) => Cast,     // a cast ballot is final, whatever arrives
```

That still forces an arm for every new *state* while absorbing every new *event*. Which half you want checked is a real decision — write the one you mean rather than the one that is shortest.

## Grouping without losing the check

`|` collapses several cells into one arm and keeps them named, so the table stays complete and stays readable:

```rust
(Cast, Mark) | (Cast, Submit) | (Cast, Void) => Cast,
```

Three cells, one answer, and a fourth event still breaks the build. [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) is the page on that, including the two lints that catch a collapse done wrong.

## Why the states are a type and not a `bool`

Two booleans — `is_marked`, `is_cast` — describe four states, and one of them (`cast` but not `marked`) is nonsense that every function reading them has to defend against. Three named variants describe three. That is the [product-versus-sum](../variants_that_carry_data/README.md) argument again, and a state machine is where it bites hardest, because the impossible states are exactly the ones that produce the bug reports nobody can reproduce.

## What this pattern does not give you

`step` returns a `State`, so *"the event was refused"* and *"the event was accepted and changed nothing"* arrive as the same answer. If a caller needs to tell those apart, the return type has to say so — `Option<State>`, or a small `Transition` enum. The compiler checked the table's *completeness*; it has no opinion about whether the answers are right.

Nor does anything stop a caller constructing `State::Cast` directly and skipping the machine. Keeping the variants private to a module and exposing only `step` is what makes the transition table the only way through.

## If you are coming from another language

**Python.** The nearest idiom is a `dict` keyed by `(state, event)`, and it is genuinely close — the same table, written as data. What is missing is the completeness check: a missing key is a `KeyError` at runtime, in production, on the transition nobody tested, and `.get(key, state)` is the `_ => state` arm with the same consequence. `match` statements plus `enum.Enum` get you the shape; only a strict type checker gets you the audit.

**ABAP.** A nested `CASE` on a status field, or a Z-table of transitions read at runtime. Both work, and neither can tell you that a status was added last sprint and one of the nine branches was never written — that is a code inspection, done by a person, every time. This is the single clearest place where Rust's `CASE` equivalent does something ABAP's cannot.

**C.** A `switch` inside a `switch`, or a two-dimensional array of function pointers indexed by state and event. The array is the better version and is the direct ancestor of this pattern: it forces you to fill in every cell, because the array has a fixed size. Rust's `match` is that array with names instead of indices — and with the compiler, rather than a segfault, telling you when the dimensions change.

## The verified output

[`examples/an_enum_as_a_state_machine.rs`](examples/an_enum_as_a_state_machine.rs) compiled and run — a ballot driven through six events, including two that are refused:

<!-- output:an_enum_as_a_state_machine -->
*Verified output of [`an_enum_as_a_state_machine.rs`](examples/an_enum_as_a_state_machine.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
from     event    to
----------------------------
Blank    Submit   Blank    no change
Blank    Mark     Marked
Marked   Void     Blank
Blank    Mark     Marked
Marked   Submit   Cast
Cast     Void     Cast     no change

final state: Cast
```
<!-- /output -->

---

## See also

- [What an enum is](../what_an_enum_is/README.md) — the declaration and exhaustiveness, from the start
- [Variants that carry data](../variants_that_carry_data/README.md) — why three variants beat two booleans
- [A typo becomes a binding](../a_typo_becomes_a_binding/README.md) — the other way a table stops being audited
- [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) — `|` groups, ranges, and their lints
