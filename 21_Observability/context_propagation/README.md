# Carrying the trace across a boundary

**Level:** 301 · deep dive

**One line:** A trace crosses a process boundary only because something copied the ids into the outgoing request and something else read them out — 55 characters of header, and a dozen places to drop them.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The header itself, which is small enough to read in full. [W3C Trace Context ↗](https://www.w3.org/TR/trace-context/) defines `traceparent` as four hyphen-separated fields — version, a 32-hex-digit trace id, a 16-hex-digit parent span id, and two hex digits of flags — for exactly 55 characters. `tracestate` carries vendor data alongside it.
- Parsing it is a real, checkable exercise with no crate involved: fixed length, fixed alphabet, an all-zero id that is invalid, an unknown version that must be tolerated rather than rejected. That is the shape of the example this page will graduate with.
- Inject on the way out, extract on the way in. Where the propagator plugs into an HTTP client and a server, and what the same job looks like for a queue message, where the "headers" are whatever the producer agreed to put in the envelope.
- The boundaries that lose context by default, which is the practical half of the page: a message queue, a `tokio::spawn`, a thread pool, a retry that builds a fresh request, a cron job that starts its own root, and any hop through a proxy that strips unknown headers.
- What a severed trace looks like from the UI: two complete-looking traces of one request, or an orphan span naming a parent nobody has. Neither is reported as an error anywhere.
- The sampling flag rides in that last byte, so a downstream service inherits the upstream decision — which is why [sampling](../sampling/README.md) has to be set once, not per service.

## If you are coming from another language

**ABAP** has been doing this for longer than the word existed: SAP Passport travels as an HTTP header, carries an id through every system it touches, and lets the statistics records collected on each of them be tied back to one user action. Same idea, same failure mode when a hop drops the header. **Python**'s `contextvars` is the async-safe carrier of exactly this kind of ambient value, and is what its OpenTelemetry SDK uses; Rust has no runtime-level equivalent, which is why context is threaded by the subscriber and the propagator rather than by the language. Both bridges are worth checking against current documentation before the finished page states them.

## The trap it exists for

The header that arrives and is thrown away. Each service's own traces look complete — its spans are all there, its root is where it expects — so nothing local is broken. The end-to-end picture is severed at exactly that hop, and the only symptom is a trace that starts in the middle.

## See also

- [Instrumenting async code](../instrumenting_async/README.md) — the same loss of context inside one process
- [Sampling](../sampling/README.md) — the decision this header carries
- [What OpenTelemetry is](../what_opentelemetry_is/README.md) — where the propagator API comes from

## Po polsku

Nagłówek jest na tyle mały, że opłaca się go znać na pamięć: `traceparent` to cztery pola rozdzielone myślnikami — wersja, `trace id` o 32 znakach szesnastkowych, `parent span id` o 16 znakach i dwa znaki flag — razem **dokładnie 55 znaków**. Obok jedzie `tracestate` z danymi konkretnego dostawcy. Przy pisaniu parsera dwie reguły łatwo przeoczyć, a obie są w specyfikacji: identyfikator złożony z samych zer jest **nieprawidłowy**, a nieznaną wersję trzeba **tolerować**, nie odrzucać. To rzadki przypadek, w którym poprawny parser ma być pobłażliwy — i akurat ten fragment da się w tej bibliotece sprawdzić bez żadnego crate'a, samą biblioteką standardową.

Po polsku najlepiej opisuje to obraz sztafety: pałeczkę wkłada się do żądania wychodzącego (*inject*) i wyjmuje z przychodzącego (*extract*). Miejsca, w których wypada domyślnie, warto znać z nazwy, bo to praktyczna połowa tej strony — kolejka komunikatów, `tokio::spawn`, pula wątków, ponowienie budujące świeże żądanie od zera, zadanie z crona zaczynające własny korzeń oraz każdy przeskok przez proxy lub WAF, który wycina nieznane nagłówki. Nigdzie nie pojawi się przy tym błąd: dostajesz albo dwa kompletnie wyglądające ślady jednego żądania, albo sierotę wskazującą rodzica, którego nikt nie zna. Lokalnie wszystko wygląda w porządku, bo każda usługa ma komplet własnych spanów — rozjazd widać wyłącznie z góry.

Jeszcze jedno, jeśli pracujesz w firmie, która ma własny `X-Request-Id` albo `X-Correlation-Id` — a ma je w Polsce niemal każdy zespół, który dorobił się mikrousług przed 2020 rokiem. Własny nagłówek rozumieją tylko twoje usługi; `traceparent` jest standardem W3C, więc rozumie go cudza biblioteka, cudze proxy i cudzy dostawca. W tym ostatnim bajcie flag jedzie też **decyzja o próbkowaniu**, dlatego usługa niżej w łańcuchu dziedziczy wybór usługi wyżej — i dlatego próbkowanie ustawia się raz, a nie osobno w każdym serwisie. Ten sam pomysł, tylko starszy niż samo słowo, znajdziesz w SAP-owym Passporcie: jeden identyfikator wędrujący przez wszystkie systemy, i ta sama awaria, gdy któryś przeskok zgubi nagłówek.

**Szukaj po polsku:** propagacja kontekstu · nagłówek traceparent · identyfikator korelacji · `w3c trace context` · `opentelemetry propagator inject extract`
