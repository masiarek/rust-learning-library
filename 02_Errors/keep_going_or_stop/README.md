# Keep going, or stop

**Level:** 201 · working knowledge

**One line:** One bad row in a thousand is a design decision, not an error-handling one — stop at the first, or process the rest and report at the end — and in Rust the *return type* is where you say which you chose.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `collect::<Result<Vec<T>, E>>()` — an iterator of `Result`s collected into a `Result` of a `Vec`, short-circuiting at the first `Err`; it reads like magic and is an ordinary `FromIterator` impl
- The other direction: keeping both halves, so the caller gets the good rows *and* a list of what was rejected
- An error that names the row it came from — line numbers are the difference between a usable message and a shrug
- `filter_map(Result::ok)` and friends: fine when "skip the junk" is genuinely the specification, silent data loss when it is not
- What exit status a partly-successful run should report

## The trap it exists for

The two policies look identical on a file with no errors, which is every file during development. Choosing by accident means the choice surfaces on the day it matters — a 40,000-line import that stopped on line 3 and told nobody, or one that skipped 200 malformed rows and reported success.

## See also

- [Readers are fallible](../readers_are_fallible/README.md) — where the iterator of `Result`s comes from
- [Returning `None` on error](../../17_Option_and_Result/none_on_error/README.md) — the same loss one level down: four distinct causes arriving as one `None`
- [The long way round](../../ROADMAP.md) — rung 7 is this idea with ballots in it

## Po polsku

Wybór „przerwać na pierwszym błędnym wierszu czy przerobić resztę i zdać raport na końcu” jest decyzją **projektową**, a nie kwestią obsługi błędów — i w Ruscie zapisuje się ją w typie zwracanym, nie w komentarzu. Najbardziej zaskakuje `collect::<Result<Vec<T>, E>>()`: ten sam `collect`, tylko z innym typem docelowym, zamienia iterator wartości `Result` w jeden `Result` i zatrzymuje się na pierwszym `Err` — wygląda to jak magia, a jest zwyczajną implementacją cechy `FromIterator`. Adnotacja typu bywa tu więc regułą biznesową, co brzmi dziwacznie dopiero do momentu, gdy zobaczysz wariant przeciwny: rozdzielenie obu połówek, żeby wywołujący dostał i dobre wiersze, i listę odrzuconych — z numerami linii, bo bez nich komunikat jest tylko wzruszeniem ramion. Najniebezpieczniejszy jest `filter_map(Result::ok)`, uczciwy wyłącznie wtedy, gdy „pomiń śmieci” naprawdę stoi w specyfikacji; w każdym innym przypadku to cicha utrata danych, która wychodzi na jaw przy imporcie czterdziestu tysięcy wierszy, a nie na trzech poprawnych linijkach z czasu pisania kodu.

**Szukaj po polsku:** walidacja wsadowa · cicha utrata danych · `rust collect Result Vec` · `rust iterator of results partition` · `rust filter_map Result::ok`
