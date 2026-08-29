# Where the data goes

**Level:** 201 → 301 · deep dive

**One line:** Your process should hand telemetry to something local as fast as it can and forget about it; retries, buffering, redaction and routing belong outside the program.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The path, once, concretely: instrumentation → a batch processor in the SDK → an OTLP exporter → a [collector ↗](https://opentelemetry.io/docs/collector/) → one or more backends (traces, metrics and logs usually land in different stores) → whatever UI the team actually opens.
- Why a collector rather than exporting straight to the vendor: your program stops knowing endpoints and API keys, sampling and redaction change without a redeploy, and one agent per host replaces one exporter per process.
- The failure that matters more than any of it — telemetry that can take the service down. Bounded queues, batching, and dropping on overflow. **Dropping spans is correct behaviour**, and the finished page has to say where the drop count shows up, because a silent drop is indistinguishable from an instrumentation bug.
- Shutdown, which is where the most interesting spans are lost: the process ends, the queue still holds the batch describing the crash, and nothing flushed it. What a flush-on-exit looks like, and why the last trace before a restart is the one you wanted.
- What the Rust page must show concretely: where the batch processor is configured, what its queue bound is, and what the program does when the collector is simply not there.

## The trap it exists for

The exporter in the request path. A synchronous send, an unbounded queue, or a blocking flush turns your observability stack into a dependency of every request — and the first thing you learn from it is that it went down with you.

## See also

- [What OpenTelemetry is](../what_opentelemetry_is/README.md) — what OTLP and the collector are, in the wider layout
- [Sampling](../sampling/README.md) — the other lever on volume, and the one that has to be decided globally
- [Metrics, and the label that multiplies](../metrics_and_cardinality/README.md) — what is actually filling the store
