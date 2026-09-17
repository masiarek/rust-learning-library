# A `#[retry]` attribute

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** `#[retry(times = 3, delay_ms = 100)]` on a function returning `Result` re-runs the body until it succeeds or the attempts run out, with its arguments parsed by `darling`'s `FromMeta` — defaults, type checking and unknown-key errors included.

## What it has to cover

- `#[derive(FromMeta)]` on the argument struct, and `NestedMeta::parse_meta_list`
- Generating the loop around the original body, and keeping the signature
- A deterministic demo: a function that fails a known number of times (no real sleeping in the answer key)
- Bad arguments, recorded: an unknown key, a wrong type, `times = 0`
- What to do with `async fn`

## See also

- [Helper attributes with `darling`](../helper_attributes_with_darling/README.md) — darling on a derive
- [Re-emit the item on error](../re_emit_on_error/README.md) — failing gracefully
- [Procedural macros](../README.md) — the chapter, in reading order
