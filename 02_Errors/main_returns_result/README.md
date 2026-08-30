# `main` can return a `Result`

**Level:** 201 · working knowledge

**One line:** Writing `fn main() -> Result<(), E>` lets you use `?` at the top level — and hands the runtime an error it prints with **`Debug`**, not with the `Display` sentence you wrote.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The [`Termination` ↗](https://doc.rust-lang.org/std/process/trait.Termination.html) trait: what `main` is allowed to return, and why `E` only needs `Debug`
- What the runtime actually prints for an `Err`, and where it prints it
- Which exit code comes out, and how that differs from a panic's
- [`ExitCode` ↗](https://doc.rust-lang.org/std/process/struct.ExitCode.html) when a specific number matters — an exit status a caller distinguishes
- The shape worth copying: a two-line `main` that calls `run() -> Result<…>` and does the presenting, so the error can be formatted for a person before it escapes

## The trap it exists for

`?` in `main` is the easiest error handling in Rust, which is why applications ship with it — and the message a user gets is the `Debug` form of an error whose `Display` impl somebody wrote carefully. `Error: Io(Os { code: 2, kind: NotFound, message: "No such file or directory" })` is the default, and it names everything except the file.

## See also

- [Debug and Display](../../15_First_Programs/debug_vs_display/README.md) — the two traits, two audiences, and the default paths that print the wrong one
- [Standard error, and exit status](../stderr_and_exit_status/README.md) — where that message goes, and the number beside it
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — the other way a program stops, and its own exit code

## Po polsku

`fn main() -> Result<(), E>` to najtańsza obsługa błędów w Ruscie — `?` działa na najwyższym poziomie i nie trzeba pisać ani jednego `match`a — i dokładnie dlatego tyle programów wychodzi z nią na produkcję. Haczyk polega na tym, że środowisko uruchomieniowe wypisuje przy `Err` postać **`Debug`**, a nie starannie ułożone zdanie z `Display`: użytkownik dostaje `Error: Io(Os { code: 2, kind: NotFound, message: "No such file or directory" })`, czyli zrzut struktury, który wymienia wszystko oprócz nazwy pliku, o który chodziło. Zauważ przy okazji, że nawet w całkowicie polskojęzycznym programie ten wiersz zaczyna się angielskim `Error:` — wypisuje go biblioteka standardowa przez cechę (*trait*) `Termination` i nie da się tego ani przetłumaczyć, ani przeformatować. Stąd wzorzec wart skopiowania: dwuwierszowy `main`, który woła `run() -> Result<…>` i sam decyduje, jak pokazać błąd człowiekowi, zanim ten wymknie się na zewnątrz.

**Szukaj po polsku:** kod wyjścia programu · Debug kontra Display · `rust main returns Result` · `rust Termination trait` · `rust ExitCode`
