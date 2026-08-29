# Sampling

**Level:** 301 · deep dive

**One line:** You cannot keep every trace, so the real question is who decides and when — before the request runs, or after you know it was interesting.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- **Head sampling**: decide at the root, write the decision into the [`traceparent` flags](../context_propagation/README.md), and let it propagate. Cheap, predictable, and it throws the slow request away before anyone knows it was slow.
- **Tail sampling**: buffer the whole trace at the collector and keep it if it errored or exceeded a latency threshold. It answers the question head sampling cannot, and it costs memory plus a constraint most people meet the hard way — every span of a trace has to reach the same collector instance.
- **Parent-based** sampling, and why a mixed decision produces half a trace: a service that samples independently of its caller drops the middle of somebody else's kept request.
- Rates that lie. 1% of traces is not 1% of errors when errors are rare; the useful policy is almost always *keep everything that failed, sample what succeeded*, which is a different sentence from "sample at 1%".
- Metrics are not sampled — which is exactly why the trend survives at 1% while the individual evidence does not. That division of labour is the argument for having both.
- A checkable example without a crate: a sampling decision is a function from a trace id to a bool, so a modelled run over ten thousand ids can show the kept fraction, the bias of a bad hash, and what a per-service rate does to whole traces.

## The trap it exists for

Sampling configured per service, by whoever deployed each one. Every service is individually reasonable; the traces have holes in them, and nothing in the UI distinguishes a hole from a hop that never happened.

## See also

- [Carrying the trace across a boundary](../context_propagation/README.md) — the flag the decision travels in
- [Where the data goes](../where_the_data_goes/README.md) — where tail sampling actually runs
- [Metrics, and the label that multiplies](../metrics_and_cardinality/README.md) — the signal that survives the cut
