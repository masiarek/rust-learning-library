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

- [A score is not a number](../../01_Foundations/newtype_score/README.md) — the same pattern asking *is this value legal?*; this page asks *what does this value mean?*
- [What a float actually stores](../../01_Foundations/what_a_float_stores/README.md) — why a temperature round trip may not land exactly where it started
- [Scaled integers](../../09_Advanced/scaled_integers/README.md) — the other way out of floating point, when the domain has a fixed precision
