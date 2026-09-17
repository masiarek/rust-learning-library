# Other kinds of test

**Level:** 201 → 301 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Unit, integration and doc tests all check examples you chose; the other kinds — property, snapshot, compile-fail, fuzz and benchmark — each check something an example cannot, and each is a crate or tool rather than part of the language.

## What it has to cover

For each kind: the question it answers, the tool, and a minimal example beside the unit test it complements.

| Kind | Checks | Tool |
|---|---|---|
| Property | a rule over generated inputs, shrunk to the smallest failure | [`proptest` ↗](https://docs.rs/proptest/latest/proptest/) |
| Snapshot | output against a reviewed, stored copy | [`insta` ↗](https://insta.rs/) |
| Compile-fail | that code is *rejected*, with the expected error | [`trybuild` ↗](https://docs.rs/trybuild/latest/trybuild/), or a doc test marked `compile_fail` |
| Fuzz | inputs nobody thought of, until something panics | `cargo-fuzz` |
| Benchmark | how long, compared with last time | [`criterion` ↗](https://docs.rs/criterion/latest/criterion/) |

- When a property test finds what fifty handwritten cases did not, and what a good property looks like
- Snapshot review as a workflow: the diff is the assertion, so accepting a snapshot without reading it removes the test
- Compile-fail tests for an API whose safety depends on code *not* compiling
- Which of these run under plain `cargo test` and which need their own command
- This library's own answer key, compared: every example's `.out` file is a snapshot test that CI re-checks

## The trap it exists for

Treating more example tests as more coverage. A hundred hand-picked inputs share the blind spots of the person who picked them; a property or a fuzzer does not.

## See also

- [What a test asserts](../what_a_test_asserts/README.md) — the example test all of these extend
- [The example that is a test](../doc_tests/README.md) — `compile_fail` in a doc test
- [How `cargo test` runs your tests](../how_cargo_test_runs/README.md) — where each kind fits in the run
- [`black_box` is a hint](../../33_Time_and_Benchmarking/black_box_is_a_hint/README.md) — keeping a benchmark honest
- [Going deeper — Testing](../../10_Resources/going_deeper/README.md#testing) — the advanced testing workshop and the fuzz book

## Po polsku

Testy jednostkowe, integracyjne i dokumentacyjne sprawdzają przykłady wybrane przez autora. Pozostałe rodzaje sprawdzają to, czego przykład sprawdzić nie może: **testy własnościowe** (*property-based*) — regułę dla losowo generowanych danych, **testy migawkowe** (*snapshot*) — wynik względem zatwierdzonej kopii, testy „nie kompiluje się” — że błędny kod zostanie odrzucony, **fuzzing** — dane, o których nikt nie pomyślał, a **benchmarki** — czas wykonania.

**Szukaj po polsku:** testy własnościowe · testy migawkowe · `rust proptest tutorial` · `rust insta snapshot` · `rust trybuild compile fail`
