# `anyhow` and context

**Level:** 201 · working knowledge

**One line:** [`anyhow`](https://docs.rs/anyhow) gives an application one error type that anything converts into, and `.context("reading ballots.txt")` is what turns *"No such file or directory"* into a sentence naming the file.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `anyhow::Result<T>` is `Result<T, anyhow::Error>` — an alias, expandable like any other
- Why `io::Error` does not contain the path it failed on, which is the gap `.context()` fills
- `.context(…)` against `.with_context(|| …)`: the second builds the string only on failure, which matters in a loop
- What a chained error prints: the outermost message, then each cause
- `bail!` and `ensure!` for the failure you raise yourself
- The cost, stated honestly: an `anyhow::Error` is type-erased, so recovering a specific cause means `downcast_ref`

## The trap it exists for

Context is added where the error is *created* — the deepest, least informed frame — or not at all. The useful message is usually assembled on the way out: *"loading the config"* → *"reading ballots.txt"* → *"No such file or directory"*, each layer adding what only it knew.

## See also

- [Not every error is an `io::Error`](../not_every_error_is_io_error/README.md) — why an application wants one type in the first place
- [`thiserror` vs `anyhow`](../thiserror_vs_anyhow/README.md) — the line this crate belongs on the far side of
- [Debug and Display](../../01_Foundations/debug_vs_display/README.md) — which of the two the chain prints, and where
