# Units are types

**Level:** 201 · working knowledge

**One line:** `f64` says how big the number is and nothing about what it *means* — a `Temperature` that knows its own scale is the difference between a conversion bug and a compile error.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The failure this prevents, which has crashed real hardware: two numbers in different units, added
- The tuple struct — `struct Celsius(f64)` — and the two things it costs: constructing it, and getting the number back out
- Storing one canonical unit inside and converting at the edges, versus carrying the unit as an enum field
- Conversions as methods, tested in both directions, including that a round trip lands back where it started
- `#[must_use]` on a conversion, because `into_fahrenheit()` on its own line is a no-op the compiler would otherwise accept
- Where the boundary is: a `Display` impl prints `21.5°C`, and the number itself never learns about formatting

## The trap it exists for

The newtype is cheap to add and awkward to keep — every arithmetic operation needs `.0` or an operator impl, and the first frustrating hour tempts you to expose the field. This page is the argument for the discipline, and it should also say when a plain `f64` was fine.

## See also

- [A score is not a number](../../16_Structs/newtype_score/README.md) — the same pattern asking *is this value legal?*; this page asks *what does this value mean?*
- [What a float actually stores](../../19_Numbers/what_a_float_stores/README.md) — why a temperature round trip may not land exactly where it started
- [Scaled integers](../../09_Advanced/scaled_integers/README.md) — the other way out of floating point, when the domain has a fixed precision

## Po polsku

`f64` mówi wyłącznie, jak duża jest liczba, i milczy o tym, co ona **znaczy** — a jednostka schowana w komentarzu nie broni nikogo; najgłośniejszy przykład to sonda Mars Climate Orbiter, stracona w 1999 roku dlatego, że do jednego rachunku trafiły wielkości liczone w dwóch różnych jednostkach. Lekarstwem jest **struktura krotkowa** (*tuple struct*) w rodzaju `struct Celsius(f64)`, czyli wzorzec `newtype` — po polsku bywa nazywany „typem opakowującym”, ale szukać trzeba pod angielską nazwą, bo pod polską prawie nic nie ma. Cena jest realna i lepiej znać ją z góry: każde działanie arytmetyczne wymaga `.0` albo własnego `impl` operatora, i to właśnie ta pierwsza męcząca godzina kusi, żeby pole otworzyć — a wtedy cała gwarancja przepada. Zwróć jeszcze uwagę na granicę wyznaczoną przez `Display`: sama liczba nigdy nie uczy się formatowania i bardzo dobrze, bo polski zapis (`21,5 °C` — przecinek dziesiętny, spacja przed jednostką) różni się od angielskiego, a to sprawa warstwy wyświetlania, nie typu.

**Szukaj po polsku:** struktura krotkowa · typ opakowujący · bezpieczeństwo jednostek · `rust newtype pattern` · `rust units of measure type safety`
