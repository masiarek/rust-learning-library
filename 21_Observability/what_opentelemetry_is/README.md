# What OpenTelemetry is

**Level:** 201 · working knowledge

**One line:** OpenTelemetry is three separable things — a data model, a wire protocol, and per-language SDKs — and knowing which of them you depend on is what decides the cost of changing vendor.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The five pieces, named separately because they fail and change separately: the **spec** (what a span or a metric *is*), the [**semantic conventions** ↗](https://opentelemetry.io/docs/specs/semconv/) (agreed attribute names — `http.request.method`, `server.address`), **OTLP** (the wire format, over gRPC or HTTP), the **SDK** (batching, sampling, export), and the [**Collector** ↗](https://opentelemetry.io/docs/collector/), which is a separate process and the subject of [its own page](../where_the_data_goes/README.md).
- Why it exists: instrumenting for a vendor used to mean that vendor's agent compiled into your program. OTLP moves the choice into a config file, and the migration stops being a code change.
- The Rust crates, and how they divide up — [`opentelemetry` ↗](https://docs.rs/opentelemetry) as the API, an SDK crate, an OTLP exporter crate, and [`tracing-opentelemetry` ↗](https://docs.rs/tracing-opentelemetry) as the bridge from the `tracing` spans a Rust program is probably already emitting. Check the split and the crate names against [the project's own repository ↗](https://github.com/open-telemetry/opentelemetry-rust) when the page is written; this layout has been reorganised before.
- Which signals are stable in Rust, checked against [the language page ↗](https://opentelemetry.io/docs/languages/rust/) rather than stated as a number here, because a stability claim ages faster than anything else on the page.
- The API/SDK split as a design decision worth understanding rather than a packaging quirk: a **library** can be instrumented against the API and ship no exporter, so it does not choose a backend on behalf of the application that depends on it.
- Semantic conventions as the underrated half. Two services naming the same thing differently do not correlate, and every dashboard is written against names rather than meanings.

## The trap it exists for

Saying "we use OpenTelemetry" as though it named one dependency. In Rust the usual arrangement is that you emit with `tracing` and bridge to OTel — so the instrumentation in your code, the SDK that batches it, and the protocol it leaves on are three choices, and only the last one is the vendor-neutral part.

## See also

- [Where the data goes](../where_the_data_goes/README.md) — the collector, and what belongs outside your process
- [A span is not a log line](../spans_not_lines/README.md) — the model these specs are describing
- [Carrying the trace across a boundary](../context_propagation/README.md) — the propagator half of the API
