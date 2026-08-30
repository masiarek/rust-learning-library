# `match` expressions

**Level:** 101 → 201 · for newcomers

**One line:** `match` checks a value against a list of options top to bottom, takes the first that fits, and — unlike a `switch` — **never falls through** and **must cover every case**, which is the property that turns "I forgot one" from a runtime surprise into a compile error.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Arms are tried in order, first match wins, and the body may be one expression or a block — the same thing, since a block is an expression
- No fall-through, and therefore no `break` to write and no way to forget one
- **Exhaustiveness**: every possible value must be covered, or `_` must catch the rest. `E0004`, and the compiler naming the value you missed
- `match` as an expression — every arm's value must have the same type, exactly as with `if`
- Matching on a `bool` first (two arms, and an honest comparison with `if`/`else`), then on an integer with ranges, then on a `char`
- Where `_` earns its keep and where it is a liability — a catch-all silently absorbs a variant added later
- The signpost to pattern matching proper: bindings, tuples, guards, `|` alternatives — named here, taught elsewhere

## The trap it exists for

`_ => …` makes a `match` exhaustive *forever*, so adding a variant to an enum a year later compiles clean and quietly takes the default branch. That is the entire subject of [a typo becomes a binding](../../13_Enums/a_typo_becomes_a_binding/README.md), where a lowercase name in an arm turns into a catch-all binding that matches everything and warns about almost nothing.

## See also

- [`if` expressions](../if_expressions/README.md) — the two-case version, and when it reads better
- [A typo becomes a binding](../../13_Enums/a_typo_becomes_a_binding/README.md) — the catch-all that was not meant to be one
- [One arm, many values](../../17_Option_and_Result/one_arm_many_values/README.md) — ranges and `|` collapsing twenty-six arms into five
- [What an enum is](../../13_Enums/what_an_enum_is/README.md) — the type `match` was built for, and where exhaustiveness pays
- [Six kinds of zero](../../17_Option_and_Result/six_kinds_of_zero/README.md) — an exhaustive match as a domain model, and the arm nobody wrote
- [`match` expressions ↗](https://doc.rust-lang.org/reference/expressions/match-expr.html) · [Comprehensive Rust: `match` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/match.html)
