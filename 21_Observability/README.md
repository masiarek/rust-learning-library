# Observability

Your program is a service now. Something you did not write calls it, it calls three more, and the only account of what happened to one request is whatever the code chose to write down. This section is about choosing well: which signals a Rust program emits about its own work, what each one can answer, and what each one costs to keep.

Monitoring is an alarm on a question somebody asked in advance — error rate, CPU, queue depth. Observability is the other direction: enough recorded detail to answer a question first asked *during* the incident, about one request out of a million. The gap between them is the dashboard that is green while a user is angry.

**These pages are stubs** — outlines waiting for a runnable example, written so the arc has a shape and a permanent URL before the prose does. [Errors](../02_Errors/README.md) says what that means and how a page graduates.

| Lesson | Level | What it will teach |
|---|---|---|
| [Why observability matters](why_observability_matters/README.md) | 101 → 201 | Not *is it broken* but *why was this one request slow* — and why only one of those can be prepared in advance |
| [The three pillars](the_three_pillars/README.md) | 101 → 201 | Traces, logs and metrics as three views of one event, and what each alone gets wrong |
| [A log line a machine can read](structured_logging/README.md) | 201 | `println!` writes a sentence; a field per value writes a record you can still query in six months |
| [A span is not a log line](spans_not_lines/README.md) | 201 | A log line is a point, a span is an interval with a parent — and the parent is what makes a request out of a pile of events |
| [Instrumenting async code](instrumenting_async/README.md) | 301 | The guard held across an `.await` that files another task's work under your span |
| [Carrying the trace across a boundary](context_propagation/README.md) | 301 | 55 characters of header, and every place a trace goes quietly missing |
| [Metrics, and the label that multiplies](metrics_and_cardinality/README.md) | 201 → 301 | Counters, gauges, histograms — and why one `user_id` label is a bill rather than a bug |
| [What OpenTelemetry is](what_opentelemetry_is/README.md) | 201 | A spec, a wire format and a set of SDKs; which of them you actually depend on |
| [Where the data goes](where_the_data_goes/README.md) | 201 → 301 | Exporter, collector, backend — what runs inside your process, and what must not |
| [Sampling](sampling/README.md) | 301 | Keeping 1% and still having the trace you needed: head, tail, and the errors you threw away |
| [What to instrument first](what_to_instrument/README.md) | 201 | The three places that pay for themselves: the request boundary, the outbound call, the error path |

## Why these are the hardest pages in the library to finish

Every checked example here is a single `.rs` file compiled by `rustc` with no dependencies — that is what [the tool](../CONTRIBUTING.md) runs, and it is why a page can promise that its output is a test result. Observability in Rust is a crate story: [`tracing` ↗](https://docs.rs/tracing), [`opentelemetry` ↗](https://docs.rs/opentelemetry), an exporter, usually an async runtime under all of it. None of that compiles under `rustc` alone.

So a page here graduates by splitting the two halves rather than by pretending the problem away. **The mechanism** can be modelled in std and checked like anything else: a span is a struct with a parent id, a `traceparent` header is 55 bytes to parse and reject, cardinality is a product you can count, sampling is a decision function you can feed a thousand ids. **The API** — the actual `tracing` call you will type — goes in a `text` fence, clearly marked as not compiled here, with a link to the crate's own docs, which are versioned and will outlive this page's memory of them.

That split is worth stating rather than hiding: a modelled span tree teaches the tree, and it is not evidence about `tracing`.

## Where this arc goes

The first two pages are the case for doing any of it. The next three are what a Rust program emits — a line, a span, and the async trap that makes spans harder here than in a thread-per-request language. Then two pages on what leaves the process and what it costs, two on the ecosystem the data lands in, and one on where to start when all of it is available and the budget is not.

It follows [Clients](../07_Clients/README.md) deliberately. That section is about calling a service you do not own; this one is about being one.
