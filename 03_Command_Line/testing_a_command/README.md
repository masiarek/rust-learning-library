# Testing a command

**Level:** 201 → 301 · deep dive

**One line:** A unit test proves a function; only running the actual binary proves the *program* — the argument parsing, the exit status, and which stream each line came out of.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Where integration tests live: `tests/`, one binary per file, seeing only your crate's public API — a different world from `#[cfg(test)]` beside the code
- Running your own program from a test: Cargo hands it to you as `env!("CARGO_BIN_EXE_<name>")`, and [`assert_cmd` ↗](https://docs.rs/assert_cmd) wraps that up
- Asserting the two things a caller can see: exit status, and a *substring* of the right stream
- Fragile assertions — matching a full transcript, exact whitespace, or a path that differs per machine — and [`predicates` ↗](https://docs.rs/predicates) as the way to say "contains" instead of "equals"
- Testing the test: break the program on purpose and confirm the test goes red, because a test that cannot fail is documentation with a green tick
- Fixing a bug by writing the failing test first, which is the habit the whole section is really about

## The trap it exists for

A test suite that only calls functions passes on a program whose `main` reads `args[2]`. Everything between the argument and the function — the parsing, the ordering, the exit code, the stream — is exactly the part users touch and the part unit tests structurally cannot reach.

## See also

- [Standard error, and exit status](../../02_Errors/stderr_and_exit_status/README.md) — the two things an integration test is asserting on
- [Deriving a parser with `clap`](../clap_derive/README.md) — `try_parse_from` tests the interface without starting a process
- [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md) — the moment a command test needs a filesystem of its own

## Po polsku

Test jednostkowy dowodzi funkcji; dopiero uruchomienie samego binarium dowodzi **programu** — parsowania argumentów, kodu wyjścia i tego, którym strumieniem wyszła dana linia. W Ruście podział na testy jednostkowe i integracyjne nie jest konwencją nazewniczą, jak w JUnicie czy w pytest, tylko granicą kompilacji: moduł `#[cfg(test)]` obok kodu widzi także elementy prywatne, a każdy plik w katalogu `tests/` jest osobnym crate'em i widzi wyłącznie publiczne API — czyli dokładnie tyle, ile widzi obcy użytkownik; ścieżkę do własnego binarium poda ci Cargo w `env!("CARGO_BIN_EXE_<nazwa>")`, a `assert_cmd` opakuje to w asercje na kod wyjścia i na **fragment** właściwego strumienia. Jedna pułapka dotyczy polskich zespołów mocniej niż innych: komunikat pochodzący z systemu (`std::io::Error`) na polskim Windowsie przychodzi po polsku — „Nie można odnaleźć określonego pliku” zamiast *No such file or directory* — więc test porównujący pełny tekst zaświeci się na czerwono tylko u części zespołu, i właśnie po to jest `predicates` z „zawiera” zamiast „równa się”. Na koniec nawyk, o który w tej lekcji naprawdę chodzi: zepsuj program celowo i sprawdź, czy test się czerwieni, bo test, który nie potrafi upaść, jest dokumentacją z zielonym znaczkiem.

**Szukaj po polsku:** testy integracyjne w Ruście · kod wyjścia programu · strumień błędów · `rust CARGO_BIN_EXE integration test` · `rust assert_cmd predicates`
