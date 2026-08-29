# The three pillars

**Level:** 101 → 201 · working knowledge

**One line:** Traces, logs and metrics are three views of one event — which path it took, what happened along the way, and how often it happens at all.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- One request, three records. A **trace**: a span per hop, with durations, showing the call to the pricing service took 8.6 of the 9 seconds. A **log**: a line inside that hop saying which cache it missed. A **metric**: a counter saying this is the four-hundredth time today.
- What each is bad at, stated as plainly as what it is good at:

  | Signal | Answers | Cannot answer |
  |---|---|---|
  | Trace | Where this request spent its time | Whether it is getting worse |
  | Log | What happened inside one step | Anything about the step it never reached |
  | Metric | How often, how bad, what trend | Which request, or why |

- Cost and shape: metrics are pre-aggregated numbers with fixed cardinality; logs are one record per event; traces are structured events with parent links, and the most expensive of the three per request.
- OpenTelemetry calls them [**signals** ↗](https://opentelemetry.io/docs/concepts/signals/), not pillars, and the list is open — logs, metrics, traces, baggage, with more in progress. Worth naming so the reader recognises the word in the docs.
- The join column. They are three views of *the same event* only if they carry the same trace id, which is [context propagation](../context_propagation/README.md) and is usually the missing piece.

## The trap it exists for

The diagram with three boxes and arrows between them, which implies correlation is a property of having all three. It is not. Three signals with three unrelated ids are three data sources, and joining them by timestamp during an incident is guesswork with extra steps.

## See also

- [Why observability matters](../why_observability_matters/README.md) — why any of them are worth the storage
- [A span is not a log line](../spans_not_lines/README.md) — the first two pillars, and what actually separates them
- [Metrics, and the label that multiplies](../metrics_and_cardinality/README.md) — the third, and the one with a bill attached
