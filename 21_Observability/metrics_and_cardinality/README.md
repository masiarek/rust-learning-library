# Metrics, and the label that multiplies

**Level:** 201 → 301 · deep dive

**One line:** A metric is a number aggregated over time; its labels decide how many numbers you are storing, and one unbounded label turns a single series into millions.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The three instruments and the question each answers: a **counter** only goes up (requests, errors), a **gauge** goes both ways (queue depth, connections), a **histogram** keeps a distribution. Latency has to be a histogram, because an average latency is the single number most capable of hiding an incident.
- Cardinality is a product, and it is arithmetic anyone can do before shipping: `method` (5) × `status` (10) × `route` (40) is 2,000 series. Add `user_id` and it is 2,000 × every user who has ever called you, kept forever, whether or not anyone looks.
- What actually breaks: not your service. The metrics backend falls over, or does not, and sends an invoice instead — which is why this failure is usually discovered by somebody outside the team.
- The division of labour that resolves it: high-cardinality detail belongs on **spans**, which are per-event and sampled. Metrics are per-series and permanent. A `customer_id` on a span costs one span; on a metric it costs a series forever.
- Rust's two routes — the [`metrics` ↗](https://docs.rs/metrics) facade with an exporter, or OpenTelemetry meters — and the pull-versus-push difference between a Prometheus scrape and an OTLP push.
- Naming and units in one paragraph: the `_seconds` / `_total` conventions, and the fact that renaming a metric breaks every dashboard and alert built on it, silently, with no compiler anywhere in the loop.

## The trap it exists for

The label that seemed harmless. `customer_id`, a URL with an id in the path, an `error_message` string taken straight from the error — each one is a debugging convenience at the call site and an unbounded dimension in the store. The commit adding it is one line and reviews clean.

## See also

- [The three pillars](../the_three_pillars/README.md) — where metrics sit next to the other two signals
- [Sampling](../sampling/README.md) — the same budget question, asked of traces
- [Where the data goes](../where_the_data_goes/README.md) — who is holding all these series

## Po polsku

Trzy przyrządy i po jednym pytaniu na każdy: **licznik** (*counter*) tylko rośnie — żądania, błędy; **gauge** (po polsku bywa „miernikiem”) chodzi w obie strony — długość kolejki, liczba połączeń; **histogram** trzyma rozkład. Opóźnienie musi być histogramem, bo średni czas odpowiedzi to pojedyncza liczba najzdolniejsza ze wszystkich do ukrycia awarii. Kardynalność (*cardinality*) brzmi groźnie, a jest zwyczajnym iloczynem — najkrócej po polsku: **iloczyn kartezjański** zbiorów wartości etykiet, do policzenia długopisem przed wdrożeniem. `method` (5) × `status` (10) × `route` (40) to 2000 serii. Dorzuć `user_id` i mnożysz te 2000 przez liczbę wszystkich użytkowników, jacy kiedykolwiek do ciebie zapukali — a raz założone serie zostają na zawsze, niezależnie od tego, czy ktokolwiek na nie patrzy.

Psuje się przy tym nie twoja usługa, tylko magazyn metryk — który albo się przewraca, albo się nie przewraca i przysyła fakturę. Dlatego akurat ten błąd wykrywa zwykle ktoś spoza zespołu. Podział pracy, który go rozwiązuje, warto zapamiętać jako jedno zdanie: szczegóły o wysokiej kardynalności należą do **spanów** (jeden na zdarzenie, i jeszcze próbkowane), a metryki są per seria i wieczne — `customer_id` na spanie kosztuje jeden span, na metryce kosztuje serię na zawsze. I ostrzeżenie, które w tej bibliotece brzmi szczególnie dotkliwie: **tu nie ma kompilatora.** Nazwa metryki to łańcuch znaków, konwencje `_total` i `_seconds` to umowa między ludźmi, a zmiana nazwy rozwala po cichu każdy pulpit i każdy alert na niej zbudowany — `rustc` nie powie ani słowa, a commit dodający jedną etykietę ma jedną linię i przechodzi przegląd bez uwag.

**Szukaj po polsku:** kardynalność metryk · licznik, gauge, histogram · `prometheus cardinality explosion` · `rust metrics crate` · `histogram vs average latency`
