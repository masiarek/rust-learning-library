# A span is not a log line

**Level:** 201 · working knowledge

**One line:** A log line records a point in time; a span records an interval with a parent — and the parent link is what turns a pile of events into a request.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The two shapes: an **event** is a timestamp plus fields; a **span** is a name, a start, an end, fields, and a parent. Everything a trace UI draws — the waterfall, the flame graph, the "8.6 s of 9 s was here" — is that parent link plus arithmetic.
- Building the tree: a root span per inbound request, a child per outbound call, events attached to whichever span is current when they fire.
- `#[tracing::instrument]` on a function, and the two things about it that surprise people: it records the arguments by default, and `skip` / `skip_all` is therefore not an optimisation but a requirement the moment an argument is a password, a token, or a 4 MB body.
- Fields known at creation versus fields recorded later — the status code does not exist when the span opens, so a span has to be able to learn.
- What "the current span" means, why it is ambient rather than passed, and why that convenience is the whole subject of [the next page](../instrumenting_async/README.md).
- Modelling it here rather than depending on a crate: a `Span { id, parent, name, start, end }` and a `Vec` of them is enough to print a real waterfall from fixed data, and enough to show that a missing parent produces two trees instead of one.

## The trap it exists for

Instrumenting every function. A span per three-line helper produces a trace with four hundred rows, which nobody reads, and a bill nobody approved. Spans belong on boundaries — the request, the query, the outbound call — because a boundary is where time is actually spent and where blame is actually transferred.

## See also

- [Instrumenting async code](../instrumenting_async/README.md) — where the ambient current span stops being free
- [A log line a machine can read](../structured_logging/README.md) — the point-in-time half of the same crate
- [What to instrument first](../what_to_instrument/README.md) — which boundaries earn a span

## Po polsku

Najkrócej po polsku: log to **punkt** na osi czasu, a span to **odcinek** — ma nazwę, początek, koniec, pola i, co najważniejsze, **rodzica**. Cały wodospad, który rysuje przeglądarka śladów, i całe zdanie „8,6 z 9 sekund poszło tutaj” to nic więcej niż ten link do rodzica plus arytmetyka na znacznikach czasu. Słowo `span` zostaje po polsku nieprzetłumaczone (odmieniamy je normalnie: *spanu*, *spanem*) i warto od razu odnotować, że przychodzi już zajęte: ze spanem `<span>` z HTML-a nie ma absolutnie nic wspólnego. Drzewo buduje się prosto — jeden span korzeniowy na przychodzące żądanie, dziecko na każde wywołanie na zewnątrz, a zdarzenia doczepiają się do tego spanu, który akurat jest bieżący.

Dwie rzeczy zaskakują tu praktycznie każdego, więc lepiej je usłyszeć wcześniej niż na produkcji. Po pierwsze `#[tracing::instrument]` **domyślnie zapisuje argumenty funkcji** — więc `skip` i `skip_all` nie są optymalizacją, tylko wymogiem w chwili, gdy argumentem jest hasło, token albo czteromegabajtowe ciało żądania. Po drugie span musi umieć **douczyć się** pól po otwarciu: kodu odpowiedzi jeszcze nie ma, kiedy span powstaje. Na koniec pokusa, która wygląda na sumienność, a jest jej przeciwieństwem — oznaczanie spanem każdej funkcji. Trzylinijkowy pomocnik nie zasługuje na span; ślad z czterystoma wierszami to ślad, którego nikt nie czyta, i rachunek, którego nikt nie zatwierdzał. Spany stawia się na **granicach**: żądanie, zapytanie do bazy, wywołanie na zewnątrz — bo granica jest tym miejscem, gdzie naprawdę schodzi czas i gdzie naprawdę przechodzi wina.

**Szukaj po polsku:** span a zdarzenie · ślad rozproszony · dane wrażliwe w spanach · `tracing instrument skip_all` · `rust span vs event`
