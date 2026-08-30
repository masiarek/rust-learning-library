# The format mini-language

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `{:>8.3}` is not Rust syntax — it is a small language of its own, parsed by `std::fmt`, and it is the one part of printing that nobody guesses correctly from first principles.

---

## What this page has to answer

- The full grammar in one place: fill, align (`<^>`), sign, `#`, `0`, width, precision, and the type character.
- Named and positional arguments — `{name}`, `{0}`, and the captured-identifier form that made most `format!("{}", x)` calls obsolete.
- `$` for dynamic width and precision: `{val:>width$}` and `{val:.prec$}`, which is how you right-align a column whose width you computed.
- `{:?}` vs `{}` vs `{:#?}` — the [`Debug`/`Display`](../../15_First_Programs/debug_vs_display/README.md) split, and the pretty-printed form worth knowing.
- Why `{s:<8?}` pads nothing: the spec is state on the `Formatter`, and an impl has to *ask* for it — [`Debug for str` never does](../../15_First_Programs/debug_vs_display/README.md), while `Debug` for a number does, and a `Vec` passes your width down to its elements.
- The numeric ones: `{:#x}`, `{:#b}`, `{:e}`, and what `{:.2}` does to a float that is exactly halfway.
- `write!` into a `String` versus `format!`, and why the first one belongs in a loop.

## See also

- [Building a `String`](../building_a_string/README.md)
- [`Debug` vs `Display`](../../15_First_Programs/debug_vs_display/README.md)
- [Making a `String`](../making_a_string/README.md)
- [STRINGS.md](../../STRINGS.md) — the map this page is a gap in
- [Strings: links, books and videos](../resources/README.md) — where to read about it in the meantime
