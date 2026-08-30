# Temporary directories in tests

**Level:** 201 → 301 · deep dive

**One line:** A test that writes to a fixed path cannot run beside itself — and the usual fix, a self-cleaning temporary directory, deletes itself the instant you stop holding on to the handle.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why `/tmp/test_output.txt` fails: Rust's test harness runs tests in **threads, in parallel**, so two tests sharing a path are a race, and a leftover file from a crashed run is a test that passes for the wrong reason
- [`tempfile::TempDir` ↗](https://docs.rs/tempfile) — a directory whose `Drop` removes it, and the sharp edge that follows: bind it to a variable, because `TempDir::new()?.path().to_owned()` hands you a path to a directory that has already been deleted
- Which is a very concrete instance of a rule this library already has a page for: a value lives until its owner goes out of scope, and a temporary's owner is the end of the statement
- What to assert: the *behaviour* (the file now contains the record) rather than the path, which differs every run
- Keeping an example deterministic when it touches a filesystem at all — the reason this section's pages are harder to finish than the others

## The trap it exists for

The dropped-too-early temp dir produces a `NotFound` from code that just created the directory, which reads as a filesystem or permissions problem and is neither. It is the ownership rules doing exactly what they say, in the one place a newcomer is not thinking about ownership.

## See also

- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) — the schedule a value actually dies on, and the five things that move it
- [Testing a command](../../03_Command_Line/testing_a_command/README.md) — the tests that need a filesystem of their own
- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — a move transfers responsibility, and the temp directory is the responsibility

## Po polsku

`cargo test` uruchamia testy równolegle, w wątkach jednego procesu, więc stała ścieżka w rodzaju `/tmp/wynik_testu.txt` to nie wygoda, tylko wyścig — a plik pozostały po przerwanym przebiegu potrafi sprawić, że test przechodzi z niewłaściwego powodu. Standardowym lekarstwem jest `tempfile::TempDir`, czyli katalog, który kasuje się sam w `Drop`, i właśnie tam czeka ostra krawędź: `TempDir::new()?.path().to_owned()` zwraca ścieżkę do katalogu, którego już nie ma, bo wartość tymczasowa ma właściciela tylko do końca instrukcji — do średnika. Wygląda to jak kłopot z systemem plików albo z uprawnieniami, a nie jest ani jednym, ani drugim: `NotFound` przychodzi z kodu, który przed chwilą ten katalog utworzył, i są to po prostu reguły własności robiące dokładnie to, co obiecują, w jedynym miejscu, w którym początkujący o własności nie myśli. Uchwyt trzeba więc przypisać do zmiennej i trzymać przez cały test — z jednym zastrzeżeniem, na które łatwo się nadziać: `let _ = TempDir::new()?;` nic nie utrzymuje, bo `_` nie jest nazwą, tylko odrzuceniem wartości, podczas gdy `let _dir = …` wiąże ją i pilnuje do końca zasięgu.

**Szukaj po polsku:** testy równoległe w Ruscie · katalog tymczasowy w testach · czas życia wartości tymczasowej · `rust tempfile TempDir` · `rust temporary value dropped while borrowed`
