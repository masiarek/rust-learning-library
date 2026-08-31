# `break` and `continue`

**Level:** 101 → 201 · for newcomers

**One line:** `continue` starts the next pass and `break` leaves the loop — and both take an optional **label** (`'outer:`), which is how you leave a nested loop without the flag variable every other language makes you invent.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Both work in all three loops; `break value` only means something in a `loop`
- Labels: `'outer: for … { for … { break 'outer; } }`, and the lifetime-looking syntax that is not a lifetime
- `continue 'outer` too, which is rarer and reads worse — say when it is worth it
- The alternative it replaces: a `mut found = false;` flag checked by the outer loop, and why that version is easier to get wrong
- Labelled `break` out of a plain block (stabilised in 1.65), which is the same feature without a loop
- When the right answer is neither — `.find()`, `.any()`, `.position()` express "stop at the first match" as a value rather than as control flow

## The trap it exists for

An unlabelled `break` inside a nested loop leaves the *inner* loop only, and the code often still looks correct — the outer loop simply carries on with the next item. There is no warning, because it is a legal thing to want.

## See also

- [`for` loops](../for_loops/README.md) · [`while` loops](../while_loops/README.md) · [`loop`](../the_loop_keyword/README.md) — the three loops these leave
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — the chains that stop early without a `break`, and exactly when they stop
- [Comprehensive Rust: `break` and `continue` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/break-continue.html)

## Po polsku

`continue` zaczyna następny obieg, `break` wychodzi z pętli — i na tym kończy się część znana z każdego innego języka. Pułapka siedzi w zagnieżdżeniu: `break` bez etykiety opuszcza **tylko** pętlę wewnętrzną, zewnętrzna leci dalej z kolejnym elementem, a ostrzeżenia nie będzie, bo bywa to całkiem sensowne życzenie. Stąd etykieta — `'outer: for … { for … { break 'outer; } }` — której apostrof wygląda jak czas życia (*lifetime*), ale nim nie jest; zastępuje ona zmienną-flagę w rodzaju `let mut found = false;`, którą w innych językach trzeba wymyślać i którą łatwo sprawdzić o jeden poziom za późno. Dwie rzeczy na koniec: `break v` z wartością znaczy coś wyłącznie w `loop`, a najlepszą odpowiedzią często nie jest ani `break`, ani flaga, tylko `.find()`, `.any()` czy `.position()`, które wyrażają „zatrzymaj się na pierwszym trafieniu” jako wartość, a nie jako sterowanie.

**Szukaj po polsku:** etykieta pętli · przerwanie pętli zagnieżdżonej · `rust labeled break outer loop` · `rust break with value`
