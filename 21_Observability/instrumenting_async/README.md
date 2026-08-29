# Instrumenting async code

**Level:** 301 · deep dive

**One line:** A span guard held across an `.await` files every other task that runs on that thread under your span — it compiles, it produces spans, and the trace is wrong.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- **Lead with the working form**, per [the house rule](../../CONTRIBUTING.md): `future.instrument(span)`, or `#[instrument]` on an `async fn`, which does the same thing for you. The refusal goes further down the page, and the broken line appears as a comment inside otherwise-valid code.
- Why the obvious version is wrong. Entering a span sets a *thread-local* current span. An `.await` hands the thread back to the executor while the guard is still alive, so the next task the runtime picks up runs inside your span and inherits its fields.
- What that looks like in the trace, which is the reason this needs a page at all: no error, no panic, no warning — just sibling work nested under an unrelated request, and durations that include somebody else's time. You cannot find it by reading the code unless you already know.
- `tokio::spawn` starts a task with no parent span. Attaching one is a decision you have to make, and the default is a detached root — the same hole as a message queue, one process earlier.
- Why this is a Rust page and not a general observability page: a thread-per-request server gets the thread-local for free, because the thread *is* the request. An async runtime multiplexes tasks onto threads, so "current" has to follow the task, and following the task is opt-in.
- Verify the mechanism against [`tracing`'s own documentation ↗](https://docs.rs/tracing) at the version being written about, and quote it rather than paraphrasing — this is the crate's own documented footgun, and its wording is more precise than a summary.

## The trap it exists for

The whole page is the trap. Every other failure in this section announces itself: a missing trace is missing, a dropped span is dropped. This one produces a complete, plausible, confidently drawn trace that attributes work to the wrong request.

## See also

- [A span is not a log line](../spans_not_lines/README.md) — where the ambient current span is introduced as a convenience
- [Carrying the trace across a boundary](../context_propagation/README.md) — the same problem one boundary out, between processes
- [Advanced](../../09_Advanced/README.md) — the section for things that assume the foundations
