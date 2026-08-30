# `for` loops

**Level:** 101 · for newcomers

**One line:** `for x in 1..5` walks a range and `for elem in [2, 4, 8]` walks a collection — and both are the same mechanism, because `for` does not know about ranges or arrays at all, only about **iterators**.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `1..5` is **exclusive**: four iterations, ending at 4. `1..=5` is the inclusive form, and the `=` is the whole difference
- A range is a value — `let r = 1..5;` — not syntax belonging to `for`
- Iterating an array, a `Vec`, a slice; and the three ways to ask (`iter`, `iter_mut`, `into_iter`), which decide whether the loop borrows or consumes
- `for x in v` **moves** `v`. The commonest beginner error in this section is using the collection afterwards, and the fix is `&v`
- `.enumerate()` when the index is wanted, rather than a hand-managed counter
- What the loop desugars into, briefly — enough to make "`for` is iterator sugar" concrete rather than a slogan
- Why the idiomatic Rust for many `for` loops is not a `for` loop at all: `map`, `filter`, `sum`

## The trap it exists for

`for x in v` consumes `v`, and the error arrives on a *later* line — `E0382`, borrow of moved value — pointing at the innocent use rather than at the loop that took it. One `&` fixes it, and knowing which `&` is the whole of [iter, iter_mut, into_iter](../../24_Iterators/iter_iter_mut_into_iter/README.md).

## See also

- [`while` loops](../while_loops/README.md) — the loop with a condition instead of a sequence
- [`loop`](../the_loop_keyword/README.md) — the one with neither
- [`break` and `continue`](../break_and_continue/README.md) — leaving early, including out of a nested loop
- [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) — the three questions `for` is really asking
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — why the chain you write instead of a loop does no work until you ask
- [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md) — the macro these examples print with, and the four things it does that `println!` does not
- [`for` loops ↗](https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops) · [Comprehensive Rust: `for` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/loops/for.html)
