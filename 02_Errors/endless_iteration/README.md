# Endless iteration

**Level:** 201 · working knowledge

**One line:** [`read_line` ↗](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line) reports end of input by returning `Ok(0)` — success, zero bytes — so a loop that only watches for `Err` runs forever on a finished file.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The return value: `Ok(n)` is bytes read *including* the newline, and `Ok(0)` is the only end-of-input signal there is
- Why `while let Ok(_) = reader.read_line(&mut buf)` is an infinite loop, and why it looks correct
- The second bug in the same function: `read_line` **appends**, so a buffer you never `clear()` grows for the length of the file
- The persistent-error version — a reader that fails every call, looped over without checking
- Writing the test that would have caught it: a reader you control, feeding end-of-input or an error on demand, with the loop under a bound

## The trap it exists for

Both bugs pass every test you would think to write, because a test with three lines of input and a correct file ends by luck. They show up on an empty file, on a closed pipe, and in production — the classic shape of a bug that only the *absence* of data can trigger. The fix is not defensive code; it is one test whose input is nothing at all.

## If you are coming from another language

- **Python** — `for line in f` ends because the iterator raises `StopIteration` for you. Rust's `lines()` iterator does the same thing; `read_line` is the lower-level call where ending is *your* comparison to make.
- **ABAP** — this is `READ DATASET ... INTO` inside a `DO` loop: the exit is a condition you write, and forgetting it hangs the work process. Rust does not save you here either — but it does hand you a value that distinguishes "nothing left" from "it broke", which `sy-subrc` alone does not.

## See also

- [Readers are fallible](../readers_are_fallible/README.md) — the `Result` this loop is failing to look at
- [`while let`](../../17_Option_and_Result/while_let/README.md) — the loop whose exit condition is a pattern, and the one bug it can have: a body that never makes progress

## Po polsku

Rust rozdziela tu dwie rzeczy, które polskie zdanie zlepia w jedno. „Nie udało się nic przeczytać” znaczy po polsku równie dobrze „plik się skończył”, jak i „odczyt padł” — a `read_line` mówi to dwiema zupełnie różnymi wartościami: `Ok(0)` to **sukces**, w którym przeczytano zero bajtów, czyli koniec wejścia, a awaria przychodzi osobno, jako `Err`. Stąd bierze się pętla `while let Ok(_) = reader.read_line(&mut buf)`, która wygląda poprawnie i kręci się w nieskończoność dokładnie wtedy, gdy plik się skończy: pilnuje wyłącznie tego, czy nie było błędu, a koniec danych błędem nie jest. Warunek wyjścia jest twój i brzmi `n == 0` — nikt go za ciebie nie postawi.

Druga usterka siedzi w tej samej linijce i jest czysto nazewnicza: `read_line` **dopisuje** do bufora, a nie nadpisuje go. Bez `buf.clear()` w każdym obrocie bufor puchnie przez całą długość pliku, a program dalej działa — tyle że coraz wolniej i z coraz dziwniejszą zawartością. Obie przechodzą każdy test, który przyszedłby ci do głowy, bo test na trzech linijkach poprawnego pliku kończy się przypadkiem. Ujawnia je dopiero **brak** danych: pusty plik, zamknięty potok. Lekarstwem nie jest defensywny kod, tylko jeden test, którego wejściem jest nic.

**Szukaj po polsku:** koniec pliku EOF · nieskończona pętla odczytu · `rust read_line returns Ok(0)` · `rust read_line appends to buffer` · `rust BufRead lines`
