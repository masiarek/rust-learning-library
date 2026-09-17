# Testing: courses and links

**Level:** reference · the reading list

**One line:** The sources behind [this section](../README.md), starting with the one course that goes past `#[test]`: what it teaches section by section, which pages here cover the same ground with programs that ran, and the places where the course has fallen behind the tools.

All links checked 2026-09-16. Crate versions are from crates.io on the same day.

## Advanced Rust testing

**[Advanced Rust testing ↗](https://rust-exercises.com/advanced-testing/)** by Luca Palmieri ([Mainmatter ↗](https://mainmatter.com/rust-consulting/)), the author of *100 Exercises to Learn Rust* and *Zero To Production*. It is a one-day classroom workshop that also works alone. The book is short, and most of the teaching is in the exercises: each section is a Cargo package in the [companion repository ↗](https://github.com/mainmatter/rust-advanced-testing-workshop), and answers are on its [`solutions` branch ↗](https://github.com/mainmatter/rust-advanced-testing-workshop/tree/solutions).

**Before you start:** you need stable *and* nightly toolchains, a clone of the repository, and [`wr`, the workshop runner ↗](https://mainmatter.github.io/rust-workshop-runner/), which checks each exercise before letting you move on. Since the subject is tests, an exercise is checked by *how your tests fail*: a small tool runs them and compares the outcome with an `expectations.yml` beside the exercise, which you are told never to edit. The database section also needs Docker.

**Who it is for:** someone who already writes `#[test]` and `assert_eq!` without thinking, and now has code that talks to a filesystem, a database or an HTTP API. If that is not you yet, start with [What a test asserts](../what_a_test_asserts/README.md) and [Where a test goes](../where_a_test_goes/README.md).

The repository has no licence file, so this library links to the course rather than copying from it. The pages in the right-hand column below are written from scratch, and each is backed by its own program.

### Section by section

| Course section | What it teaches | Crate | Here |
|---|---|---|---|
| [Better assertions ↗](https://rust-exercises.com/advanced-testing/01_better_assertions/00_intro.html) | why a shared assertion library beats home-made helpers; `googletest` matchers for `Option`, `Result`, enums and collections; writing a `Matcher`; `expect_that!`, which records a failure and carries on so one test reports every broken property | [`googletest` ↗](https://docs.rs/googletest) 0.14.3 | [What a test asserts](../what_a_test_asserts/README.md) |
| [Snapshot testing ↗](https://rust-exercises.com/advanced-testing/02_snapshots/00_intro.html) | `insta`: save an output, review changes with `cargo insta review`, inline or file snapshots, redactions for timestamps and random ids | [`insta` ↗](https://insta.rs/docs/) 1.48.0 | every lesson here is one: see below |
| [Mocks ↗](https://rust-exercises.com/advanced-testing/03_mocks/00_intro.html) | refactor to an interface first; `#[automock]`, `times`, `Sequence`, `checkpoint`; `mock!` for traits from other crates; why mock-heavy suites get expensive to maintain | [`mockall` ↗](https://docs.rs/mockall) 0.15.0 | [A test double is a second `impl`](../a_test_double_by_hand/README.md) |
| [The testing system ↗](https://rust-exercises.com/advanced-testing/04_interlude/00_testing_infrastructure.html) | which tests are compiled into which binary, and what tests in one process share | — | [How `cargo test` runs your tests](../how_cargo_test_runs/README.md) |
| [Filesystem isolation ↗](https://rust-exercises.com/advanced-testing/05_filesystem_isolation/00_intro.html) | take paths as arguments; `NamedTempFile`, `tempfile()`, `TempDir`; paths relative to a root rather than the working directory | [`tempfile` ↗](https://docs.rs/tempfile) 3.27.0 | [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md) |
| [Database isolation ↗](https://rust-exercises.com/advanced-testing/06_database_isolation/00_intro.html) | in-memory stand-ins, rolled-back transactions, or a logical database per test, and why the third wins; `#[sqlx::test]` | [`sqlx` ↗](https://docs.rs/sqlx/latest/sqlx/attr.test.html) 0.9.0 | not covered yet |
| [HTTP mocking ↗](https://rust-exercises.com/advanced-testing/07_http_mocking/00_intro.html) | take base URLs as arguments; `MockServer` on a random port, matchers, expectations, scoped mocks | [`wiremock` ↗](https://docs.rs/wiremock) 0.6.5 | [Injecting the base URL](../../07_Clients/injecting_the_base_url/README.md) and [Mocking a server](../../07_Clients/mocking_a_server/README.md), both outlines so far |
| [Macros ↗](https://rust-exercises.com/advanced-testing/08_macros/00_intro.html) | what `#[tokio::test]` expands to; an attribute macro that adds `#[test]` and before/after hooks, with `syn` and `quote` | `syn`, `quote` | [What an attribute is](../../27_Modules/what_an_attribute_is/README.md); the expansion of `#[test]` in [A harness of your own](../a_harness_of_your_own/README.md) |
| [Test harnesses ↗](https://rust-exercises.com/advanced-testing/09_test_harness/00_intro.html) | `[[test]]` with `harness = false`; `libtest-mimic` | [`libtest-mimic` ↗](https://docs.rs/libtest-mimic) 0.8.2 | [A harness of your own](../a_harness_of_your_own/README.md) |
| [Capstone ↗](https://rust-exercises.com/advanced-testing/10_capstone/00_capstone.html) | a harness that starts one Postgres container, gives every test its own database, and runs them in parallel | all of the above | — |

**Snapshot testing is how this whole library is checked.** Every `examples/*.rs` has a recorded `.out` beside it, CI fails when the program's output drifts from it, and `run_examples.py --update --only <stem>` accepts a change after you have read it. That is `insta`'s review loop, applied to programs instead of values, and [CONTRIBUTING](../../CONTRIBUTING.md) is its manual. The course's advice about redactions is this library's determinism rule seen from the other side: where `insta` removes the timestamp from a snapshot, an example here never prints one.

### Read with care

Each item was checked on 2026-09-16 against Rust 1.98.0 or against the crate's current documentation. The course is right about the ideas in every case; these are the details that have moved.

1. **The `assert_eq!` failure message** in *The built-in toolkit* is the older format, with backticks round `(left == right)` and each value. Rust 1.98.0 prints ``assertion `left == right` failed`` and then `left: 1` and `right: 2` on their own lines, unquoted. The course's claim about custom messages is still right: `assert!`'s message replaces the default, while `assert_eq!`'s is appended after `failed:` and the values still print. [What a test asserts](../what_a_test_asserts/README.md) shows the current output.
2. **Unit tests get one binary per target, not per package.** *The testing system* says a package's unit tests share one binary. A package with `src/lib.rs`, `src/main.rs` and `src/bin/report.rs` produced three unit-test binaries.
3. **Doc tests are merged on edition 2024.** *The testing system* says each doc test is compiled into a binary of its own, which is true only before edition 2024. On 2024, rustdoc compiles a crate's doc tests into one merged binary (`merged doctests compilation took 0.38s`). Each doc test still runs in a process of its own; two that wrote their process id wrote different ids.
4. **A directory under `tests/` is a test target only when it holds `main.rs`.** *The testing system* counts a directory such as `tests/foo/` as a target by itself. `tests/common/mod.rs` alone produces no binary, which is why shared helpers go there.
5. **A custom harness may fail with any non-zero exit status**, where *Quaking like `cargo test`* says it must be exactly 1. Any non-zero status is a failure, and cargo exits with the harness's status: a `main` that exited 3 made `cargo test` exit 3. libtest itself uses 101.
6. **`assert_display_snapshot!`**, listed among `insta`'s macros, is deprecated in 1.48.0, and its docs say to use `assert_snapshot!` instead.
7. **`#[sqlx::test]` deletes the test database only when the test passes.** *Testing with `sqlx`* lists deletion as the last step, unconditionally. The 0.9.0 docs say failed tests leave their databases in place for debugging, and the next run of a test binary deletes them.
8. **`MockServer::register_scoped`**, named in the HTTP section's *Checkpoints*, is `register_as_scoped` in `wiremock` 0.6.5. The same page's bullet list has the right name.
9. **Documentation links are pinned to old versions**: `googletest` 0.11.0 (now 0.14.3), `mockall` 0.12.1 (now 0.15.0), `sqlx` 0.7.3 (now 0.9.0). Read the current docs before copying a trait signature.
10. **Nothing promises to empty the temporary directory.** *Temporary files* says the OS deletes files there eventually; [`std::env::temp_dir` ↗](https://doc.rust-lang.org/std/env/fn.temp_dir.html) makes no such promise. It warns instead that the directory may be shared with other users and that predictable file names are a security problem. `tempfile`'s own cleanup is what to rely on.

## Official documentation

- [The Book, ch. 11 — Writing automated tests ↗](https://doc.rust-lang.org/book/ch11-00-testing.html): `#[test]`, `assert!`, `should_panic`, and [test organisation ↗](https://doc.rust-lang.org/book/ch11-03-test-organization.html), which answers "unit or integration, and which file".
- [The rustc book — Tests ↗](https://doc.rust-lang.org/rustc/tests/index.html): libtest's command line in full: `--exact`, `--list`, `--ignored`, `--test-threads`, `--nocapture`, and the unstable ones.
- [The Cargo book — `cargo test` ↗](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [target settings ↗](https://doc.rust-lang.org/cargo/reference/cargo-targets.html): `--no-fail-fast`, test target discovery, and the `harness` field.
- [The rustdoc book — Documentation tests ↗](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html): fence attributes, hidden lines, and how doc tests are merged in edition 2024.

## Tools, by the question they answer

| If the question is… | Reach for | Here |
|---|---|---|
| can one test process crash the rest? | [cargo-nextest ↗](https://nexte.st/): one process per test | [cargo-nextest](../../05_Tooling/nextest/README.md) |
| did the output change, and was that intended? | [`insta` ↗](https://insta.rs/docs/) | the snapshot note above |
| does this hold for inputs I did not think of? | [proptest ↗](https://proptest-rs.github.io/proptest/intro.html): generated inputs, shrunk to the smallest failing case | — |
| does anything crash on hostile input? | [cargo-fuzz ↗](https://rust-fuzz.github.io/book/introduction.html) | — |
| the same test, many rows, each reported separately | [`rstest` ↗](https://docs.rs/rstest) cases and fixtures | [A harness of your own](../a_harness_of_your_own/README.md), practice |
| does this code *fail* to compile, with the right error? | [`trybuild` ↗](https://docs.rs/trybuild) | — |
| which part of a long `assert_eq!` differs? | [`pretty_assertions` ↗](https://docs.rs/pretty_assertions): a coloured diff | — |

The property, snapshot, compile-fail and fuzz rows are what [Other kinds of test](../other_kinds_of_test/README.md) is being written to cover, one example each.

## Books

- **[Zero To Production In Rust ↗](https://www.zero2prod.com/)** (paid), by the course's author. Its testing chapters build the same approach inside a real service: a database per test, `wiremock` for the email provider, black-box tests through HTTP. The [books shelf](../../10_Resources/books/README.md) has the full verdict.
- **[100 Exercises to Learn Rust ↗](https://rust-exercises.com/100-exercises/)**, the same author's free course, is test-driven from the first exercise. It is the one to do before this one; see [Exercises](../../10_Resources/exercises/README.md).

## Po polsku

Kurs *Advanced Rust testing* nie ma polskiej wersji, ale jest krótki i w większości składa się z ćwiczeń, więc angielskiego do przeczytania jest niewiele. Znacznie więcej jest do napisania. Warto zacząć od sekcji *The testing system* (czyli od strony [Co buduje `cargo test`](../how_cargo_test_runs/README.md)), bo tłumaczy, dlaczego testy w jednym pliku wykonywalnym dzielą katalog roboczy i zmienne `static`, a wszystkie późniejsze sekcje o izolacji plików, baz danych i HTTP są odpowiedzią właśnie na to. Lista „Read with care” powyżej zbiera miejsca, w których kurs rozminął się z Rustem 1.98 albo z aktualnymi wersjami bibliotek: format komunikatu `assert_eq!`, scalone w edycji 2024 testy dokumentacyjne, przestarzałe `assert_display_snapshot!` i `#[sqlx::test]`, które po nieudanym teście zostawia bazę do obejrzenia. Idee są wszędzie trafne; zmieniły się szczegóły.

**Szukaj po polsku:** testowanie w Ruście · testy migawkowe (*snapshot testing*) · atrapy i mocki · izolacja testów · `rust advanced testing workshop` · `rust googletest insta mockall wiremock`
