# `loop`

**Level:** 101 → 201 · for newcomers

**One line:** `loop` repeats forever and is left with `break` — and because `break` can carry a value, `loop` is the one loop in Rust that is routinely used as an **expression**: `let x = loop { … break v; };`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `loop { }` with no condition, and why a dedicated keyword beats `while true`: the compiler knows a `loop` without `break` never finishes, so it type-checks the code after it differently
- `break value` — the reason this page is separate from `while`. `while` and `for` always evaluate to `()`; only `loop` can produce a value
- The idiom it exists for: retry-until-success, and a read loop that ends on a sentinel
- Its type is `!` (never) when nothing breaks, which is why `let x: i32 = loop {};` compiles
- Where it turns into `while let` or an iterator, and why that is usually the better ending

## The trap it exists for

`while true` and `loop` are not interchangeable to the compiler even though they run the same: flow analysis treats `loop` as definitely-not-returning and `while true` as a condition that might one day be false, so the two differ on whether a following `let` is considered initialised. Clippy's `while_immutable_condition` and the `never_loop` lint both live in this neighbourhood.

## See also

- [`break` and `continue`](../break_and_continue/README.md) — the keyword this loop depends on, and labels for the nested case
- [`while` loops](../while_loops/README.md) — the condition-at-the-top version
- [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) — the rule that lets a loop have a value at all
- [Comprehensive Rust: `loop` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/loops/loop.html)
