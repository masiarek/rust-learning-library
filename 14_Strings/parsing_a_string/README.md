# Parsing out of a string

**Level:** 101 → 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `"42".parse()` is the `FromStr` trait, and it returns a `Result` because text is input and input lies — so the interesting half of this page is what you do with the `Err`.

---

## What this page has to answer

- What `.parse()` actually is: `FromStr`, and why the turbofish or the annotation is not optional — nothing else in the expression names the target type.
- The four ways to handle the `Result`, and when each is right: `?`, `unwrap_or`, `match`, and the `expect` that is honest in a scratch program and wrong in a tool someone else runs.
- `ParseIntError` and `ParseFloatError` — what the `kind` field distinguishes, and why `""`, `"forty-two"` and `"999999999999"` are three different failures.
- Implementing `FromStr` for your own type, so `"5,2,0".parse::<Ballot>()` works — and why that is the same shape as implementing `Display` for the way out.
- Whitespace and sign handling: what `parse` will forgive (`"+42"`) and what it will not (`" 42 "`), which is the bug in every hand-rolled CSV reader.

## See also

- [Making a `String`](../making_a_string/README.md)
- [Walking a `String`](../walking_a_string/README.md)
- [`unwrap_or`](../../17_Option_and_Result/unwrap_or/README.md)
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md)
- [STRINGS.md](../../STRINGS.md) — the map this page is a gap in
- [Strings: links, books and videos](../resources/README.md) — where to read about it in the meantime
