# Deriving a parser with `clap`

**Level:** 201 · working knowledge

**One line:** With [`clap` ↗](https://docs.rs/clap)'s `derive` feature, a struct **is** the command-line interface: `#[derive(Parser)]` generates the parsing, the `--help` text, the `--version`, and the error message for a flag nobody defined.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Cargo features: `clap` does nothing until `features = ["derive"]` is on, and what a feature actually is
- The mapping — a `bool` field is a switch, an `Option<T>` is optional, a `Vec<T>` collects repeats, a plain `T` is required
- `#[command(...)]` metadata pulled from `Cargo.toml`, so the version in `--version` cannot drift from the version you shipped
- **Doc comments become help text** — the same `///` that [is really an attribute](../../15_First_Programs/comments_that_compile/README.md), doing a third job
- `Parser::parse()` in `main` against `try_parse_from(["prog", "--words"])` in a test, which is what makes the interface unit-testable at all
- The counterweight: one flag and no dependencies does not need a dependency

## The trap it exists for

A derived parser is so cheap that it becomes the *specification* of the program without anyone deciding it should be. Rename a field and you have renamed a flag; make a field required and you have broken every caller's script. The struct is a public interface — the page should say so where a reader will meet it.

## See also

- [Flags by hand](../flags_by_hand/README.md) — the work this is replacing, and the conventions it gets right for free
- [Testing a command](../testing_a_command/README.md) — testing the parser and testing the program are two different tests
- [Comments that compile](../../15_First_Programs/comments_that_compile/README.md) — why a `///` above a field can turn into help text at all

## Po polsku

Atrybut `#[derive(Parser)]` z crate'a `clap` sprawia, że struktura (*struct*) **jest** interfejsem programu: z jednego typu powstaje parsowanie argumentów, tekst `--help`, `--version` i komunikat o fladze, której nikt nie zdefiniował. Jeśli znasz adnotacje z Javy albo atrybuty z C#, to najbliższa analogia — z tą różnicą, że `derive` jest makrem proceduralnym działającym **w czasie kompilacji**, więc pomoc programu istnieje, zanim program w ogóle wystartuje; nic się jednak nie wydarzy, dopóki w `Cargo.toml` nie włączysz `features = ["derive"]`, bo to jest *cargo feature*, a nie osobna biblioteka. Odwzorowanie typów warto zapamiętać w całości — `bool` to przełącznik, `Option<T>` to argument opcjonalny, `Vec<T>` zbiera powtórzenia, zwykłe `T` jest wymagane — a komentarze dokumentacyjne `///` stają się tekstem pomocy, co dla polskiego autora oznacza realną decyzję, bo `clap` własnych komunikatów nie tłumaczy i `error: unexpected argument` zostanie po angielsku niezależnie od tego, w jakim języku opiszesz pola. Największa pułapka jest przy tym nietechniczna: ta struktura to publiczny interfejs — zmiana nazwy pola zmienia nazwę flagi, a dodanie pola wymaganego psuje wszystkie cudze skrypty.

**Szukaj po polsku:** parsowanie argumentów w Ruście · makra proceduralne · atrybuty i `derive` · `rust clap derive Parser` · `rust cargo features derive`
