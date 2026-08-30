# `while` loops

**Level:** 101 · for newcomers

**One line:** `while` repeats a block for as long as a `bool` condition holds — the loop you reach for when the number of iterations is not known in advance, and the one that needs a `mut` to make progress.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- No parentheses, braces required, condition must be a `bool` — the same three rules as `if`
- The condition is checked *before* each pass, so a `while` may run zero times; Rust has no `do…while`
- A `while` loop nearly always needs a `mut` binding declared outside it, which is worth noticing: it is the shape a `for` over an iterator exists to avoid
- `while let` as the pattern-matching cousin, and the loop it replaces
- When `while` is the honest choice (a condition that is not a sequence — convergence, a queue draining, input arriving) and when it is a `for` in disguise

## The trap it exists for

A `while` whose condition depends on a variable the body forgot to advance is an infinite loop the compiler will not warn about, because nothing is wrong with the program — it is only wrong about what you meant. The `for` version of the same loop cannot make that mistake, which is the argument for preferring it whenever the sequence is known.

## See also

- [`while let`](../../17_Option_and_Result/while_let/README.md) — the same loop driven by a pattern, and the body that peeks where it meant to advance
- [`for` loops](../for_loops/README.md) — when the sequence is known
- [`loop`](../the_loop_keyword/README.md) — when there is no condition at the top
- [Variables](../../15_First_Programs/variables/README.md) — the `mut` a `while` loop almost always needs
- [Comprehensive Rust: `while` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/loops/while.html)
