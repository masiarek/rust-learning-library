# `thiserror` vs `anyhow`

**Level:** 301 · deep dive

**One line:** A library **names** its errors so callers can decide; an application **erases** them because the only remaining decision is what to print — and the whole choice is which side of that line your code is on.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- [`thiserror` ↗](https://docs.rs/thiserror) is a boilerplate remover, not a new error model: it derives `Display`, `Error` and `From` on an enum you would otherwise write by hand
- [`anyhow` ↗](https://docs.rs/anyhow) is a type-erased error for the top of a program, where the caller is a person
- Why `anyhow::Result` in a public API is a dead end: the caller can print it and nothing else
- Error types are semver-visible — adding a variant is a breaking change unless the enum is `#[non_exhaustive]`
- The common shape: a binary with a library half, `thiserror` inside, `anyhow` in `main`
- What you give up either way, so the page ends on a trade rather than a rule

## The trap it exists for

The crates are not rivals, so "which is better" has no answer — and picking one by habit is how a library ends up returning `anyhow::Error` to callers who needed to distinguish *file missing* from *file malformed*. The question is never which crate; it is whether anyone downstream has a decision to make.

## See also

- [`anyhow` and context](../anyhow_and_context/README.md) — the application side, in detail
- [Not every error is an `io::Error`](../not_every_error_is_io_error/README.md) — the problem both crates are answering
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — Steps 8 and 9: designing the `E`, and deciding not to
