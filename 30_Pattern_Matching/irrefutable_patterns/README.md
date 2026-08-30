# Irrefutable patterns

**Level:** 101 → 201 · for newcomers

**One line:** The left of every `let` is a **pattern**, not a name — `let x = 5;` is the simplest one there is — and the rule that decides where a pattern is allowed is whether it can *fail*: `let (a, b) = pair;` always fits, `let Some(n) = opt;` might not, and only the first is legal in a plain `let`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- **Irrefutable** = cannot fail to match. A bare name, a tuple, a struct, `_`. **Refutable** = might not fit: any enum variant, any literal, any range
- The four places that demand an irrefutable pattern — `let`, `for` loop bindings, function parameters, closure parameters — and why: there is no second branch for the failure to go to
- The four that accept a refutable one — `match` arms, `if let`, `while let`, `let else` — because each supplies that branch
- `fn print_pair((a, b): (i32, i32))` and `for (i, name) in names.iter().enumerate()` are pattern matching, and reading them that way is the point of the page
- `E0005` and what its "refutable pattern in local binding" message is actually telling you to do
- `_` versus `_name`, which are different, and only one of them is a pattern that binds

## The trap it exists for

`let Some(n) = opt;` is the natural thing to write and it does not compile. Every beginner writes it, reads `E0005`, and reaches for `.unwrap()` — which works and throws away the branch that made the pattern refutable in the first place. The right fixes are `if let`, `let else`, or a `match`, and which of the three depends on what should happen when it does not fit.

## See also

- [`let else`](../let_else/README.md) — the shape that makes a refutable pattern usable in a binding
- [`if let`](../../17_Option_and_Result/if_let/README.md) — the same, when there is something to do in the other branch
- [Variables](../../15_First_Programs/variables/README.md) — the `let` this page is re-reading
- [Destructuring structs](../destructuring_structs/README.md) — the irrefutable pattern you will use most
- [Patterns ↗](https://doc.rust-lang.org/reference/patterns.html) · [`E0005` ↗](https://doc.rust-lang.org/error_codes/E0005.html) · [Comprehensive Rust: Irrefutable Patterns ↗](https://google.github.io/comprehensive-rust/pattern-matching/infallible.html)
