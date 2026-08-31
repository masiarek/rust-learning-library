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

## Po polsku

Słowo **próbkowanie** (*sampling*) ciągnie za sobą po polsku dwie intuicje i obie trzeba tu poprawić. Z przetwarzania sygnałów niesie regularność — co n-ta próbka, stała częstotliwość. Ze statystyki niesie **reprezentatywność** — próba ma być nieobciążona. W śladach chodzi dokładnie na odwrót: sensowna polityka jest **celowo obciążona**, brzmi „zachowaj wszystko, co się wywróciło, próbkuj to, co się udało”, i jest zupełnie innym zdaniem niż „próbkujemy na poziomie 1%”. Przy rzadkich błędach 1% śladów to bowiem nie 1% błędów, tylko najczęściej zero.

Reszta sprowadza się do pytania, **kto decyduje i kiedy**. Przy *head sampling* decyzja zapada w korzeniu, ląduje w bajcie flag nagłówka `traceparent` i propaguje się w dół — tanio i przewidywalnie, ale wolne żądanie wylatuje do kosza, zanim ktokolwiek zdąży zauważyć, że było wolne. Przy *tail sampling* collector buforuje cały ślad i zatrzymuje go, jeśli poleciał błąd albo przekroczony został próg czasu; to odpowiedź na pytanie, którego *head sampling* zadać nie umie, kosztem pamięci i jednego twardego warunku: **wszystkie spany jednego śladu muszą trafić do tej samej instancji collectora**. Stąd pułapka, o którą chodzi na tej stronie — próbkowanie konfigurowane osobno w każdej usłudze, przez tego, kto akurat ją wdrażał. Każde ustawienie z osobna jest rozsądne, ślady mają dziury, a w interfejsie nic nie odróżnia dziury od przeskoku, którego nigdy nie było. Metryk się przy tym nie próbkuje i właśnie dlatego przy 1% trend przeżywa, choć pojedynczy dowód ginie.

**Szukaj po polsku:** próbkowanie śladów · próba obciążona · head-based i tail-based · `tail sampling collector` · `otel traceidratio sampler`
