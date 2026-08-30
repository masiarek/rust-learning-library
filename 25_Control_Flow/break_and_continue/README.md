# `break` and `continue`

**Level:** 101 → 201 · for newcomers

**One line:** `continue` starts the next pass and `break` leaves the loop — and both take an optional **label** (`'outer:`), which is how you leave a nested loop without the flag variable every other language makes you invent.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Both work in all three loops; `break value` only means something in a `loop`
- Labels: `'outer: for … { for … { break 'outer; } }`, and the lifetime-looking syntax that is not a lifetime
- `continue 'outer` too, which is rarer and reads worse — say when it is worth it
- The alternative it replaces: a `mut found = false;` flag checked by the outer loop, and why that version is easier to get wrong
- Labelled `break` out of a plain block (stabilised in 1.65), which is the same feature without a loop
- When the right answer is neither — `.find()`, `.any()`, `.position()` express "stop at the first match" as a value rather than as control flow

## The trap it exists for

An unlabelled `break` inside a nested loop leaves the *inner* loop only, and the code often still looks correct — the outer loop simply carries on with the next item. There is no warning, because it is a legal thing to want.

## See also

- [`for` loops](../for_loops/README.md) · [`while` loops](../while_loops/README.md) · [`loop`](../the_loop_keyword/README.md) — the three loops these leave
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — the chains that stop early without a `break`, and exactly when they stop
- [Comprehensive Rust: `break` and `continue` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/break-continue.html)
