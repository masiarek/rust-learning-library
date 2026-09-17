# Advent of Code in Rust

**Level:** 201 · working knowledge

**One line:** Advent of Code is twenty-five puzzles every December, each opening with a text-parsing problem — which makes it a Rust strings, iterators and `Result` workout more than an algorithms one, for the first ten days at least.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Which Rust each early day actually exercises: `lines()`, `split_once`, `parse::<i64>()`, `collect::<Result<Vec<_>, _>>()`, `HashMap` entry counting, grids as a flat `Vec`
- A project layout that survives twenty-five days: one binary per day under `src/bin/`, tests that use the puzzle's *example* input from the text
- Why your personal puzzle input should not be committed to a public repository — check the site's own wording on [its about page ↗](https://adventofcode.com/about) before writing the rule down
- When `unwrap` is fine (a puzzle) and what the same code would need in a program somebody else runs — [`unwrap` is a TODO](../../../02_Errors/unwrap_is_a_todo/README.md)
- Performance: `--release`, and the days where the algorithm matters more than the language
- Which days map onto which library sections — a table to fill in as the pages get written

## The trap it exists for

Writing every day as one `main` full of `unwrap` and index arithmetic, then solving part two by copy-paste. The parsing is the same across both parts; a `parse(input) -> Result<Puzzle, _>` function written on day one is the habit that pays back by day five.

## Where this sits

[Exercises](../README.md) compares the practice tracks by stage. Advent of Code is a seasonal one, harder in its second half than anything else on that shelf. [KATAS.md](../../../KATAS.md) is this library's own sequence.

## See also

- [Exercises](../../../10_Resources/exercises/README.md) — the other practice tracks, by stage
- [KATAS.md](../../../KATAS.md) — this library's katas, in order
- [Parsing out of a string](../../../14_Strings/parsing_a_string/README.md) — `parse` and `FromStr`, day one of every puzzle
- [`collect` and `FromIterator`](../../../24_Iterators/collect_and_fromiterator/README.md) — collecting into `Result<Vec<_>, _>`
- [Grids and nested `Vec`s](../../../26_Collections/vec_of_vecs/README.md) — the grid puzzles
- [Running a scratch program](../../../15_First_Programs/rustc_without_cargo/README.md) — `src/bin/` for one binary per day

## If you are coming from another language

- **Python.** Most published solutions are Python, and they make a good comparison: the same parsing in Rust is longer because every `int()` becomes a `Result` you decide about.
- **Go.** Go solutions look closer to Rust's shape — explicit errors, explicit types — without the iterator chains.

## Po polsku

Advent of Code to 25 łamigłówek publikowanych co grudzień; każda zaczyna się od parsowania tekstu, więc pierwsze dni to raczej trening napisów, iteratorów i `Result` w Ruście niż algorytmiki. Dobry nawyk od pierwszego dnia: osobna funkcja `parse`, testy na przykładzie z treści zadania, a nie na własnych danych wejściowych.

**Szukaj po polsku:** Advent of Code w Ruście · zadania programistyczne · `advent of code rust template` · `rust parsing input lines`
