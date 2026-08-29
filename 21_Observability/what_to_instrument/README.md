# What to instrument first

**Level:** 201 · working knowledge

**One line:** Three places pay for themselves before anything else — the request boundary, every call that leaves the process, and the error path.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The minimum, as a checklist a reader can apply to a service this afternoon: one root span per inbound request (route, method, status, duration); one child span per outbound call (which dependency, which target, what it returned); one event on every error path carrying the reason rather than the fact.
- Fields, not sentences. Ids, sizes and decisions go on the span where a query can reach them — the argument [structured logging](../structured_logging/README.md) makes about a line, applied to an interval.
- The never list, and why it needs saying twice: passwords, tokens, keys, full request bodies, anything personal. `#[instrument]` records arguments **by default**, so the omission is the thing you have to write down.
- Cost, honestly: the health check that fires every five seconds and produces most of your spans, and how it gets dropped at the collector rather than by an `if` in the handler.
- Timing. Instrumentation added during an incident is too late; instrumentation added everywhere in advance is a bill. A per-boundary checklist is the middle, and it is the only part of this section that is a recommendation rather than a mechanism.

## The trap it exists for

Instrumenting what is easy to instrument. The pure function you just wrote takes a span in ten seconds and will never be the reason a request took nine seconds. The network call, the deserialize, the lock and the retry are harder to reach and are where the time and the failures actually are.

## See also

- [A span is not a log line](../spans_not_lines/README.md) — what you are putting on those boundaries
- [Why observability matters](../why_observability_matters/README.md) — the same argument, before it became a checklist
- [Sampling](../sampling/README.md) — what happens to all of it on the way out
