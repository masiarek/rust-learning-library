# Testing

**One line:** Three kinds of test, one command, and no assertion library — `cargo test` runs the unit tests inside your modules, the integration tests in `tests/`, and every example in your documentation.

The test framework is part of the language rather than a dependency, which has one consequence worth noticing on day one: there is no vocabulary to learn. A test is a function that panics when it is unhappy, and `assert_eq!` is the whole API.

| Lesson | Level | What it covers |
|---|---|---|
| [What a test asserts](what_a_test_asserts/README.md) | 201 | `assert!` vs `assert_eq!`, the message, floats, and the assertion that cannot fail |
| [Where a test goes](where_a_test_goes/README.md) | 201 | Inside the module or outside the crate, and the `#[should_panic]` that passes on any panic |
| [The example that is a test](doc_tests/README.md) | 201 | Doc tests: an integration test that is also the documentation |

## The three, in one table

| | Lives in | Sees | Good for |
|---|---|---|---|
| unit | `#[cfg(test)] mod tests` in `src/` | private items | the awkward internals |
| integration | a file directly in `tests/` | the public API | behaviour that survives a refactor |
| doc | a ` ``` ` block in `///` | the public API | the one example a reader needs |

## Where the rest of it is

The harness itself, and running tests faster: [cargo-nextest](../05_Tooling/nextest/README.md), one process per test. Testing a program rather than a library — arguments, exit status, stdout: [Testing a command](../03_Command_Line/testing_a_command/README.md). And the attributes all of this is built out of — `#[cfg]`, `#[test]`, `#[should_panic]` — are in [what an attribute is](../27_Modules/what_an_attribute_is/README.md).

## Where it goes next

A test that fails is a panic, and what a panic actually does is [its own page](../17_Option_and_Result/what_a_panic_costs/README.md). And the reason most of this library's examples need no tests at all is that every one of them has a recorded answer key CI re-checks — the same idea as a doc test, applied to a whole program.

## Po polsku

W Ruscie nie dobiera się biblioteki do testów: mechanizm uruchamiający je (*test harness*) jest częścią języka, więc nie ma tu odpowiednika JUnita, pytesta ani osobnej biblioteki asercji do nauczenia się. Test to zwyczajna funkcja, która **panikuje**, kiedy coś się nie zgadza, a `assert_eq!` to w zasadzie całe API. Jedno polecenie `cargo test` uruchamia wszystkie trzy rodzaje naraz: testy jednostkowe w module `#[cfg(test)]` wewnątrz `src/` (widzą elementy prywatne), testy integracyjne w katalogu `tests/` (widzą wyłącznie publiczne API) oraz testy dokumentacyjne, czyli przykłady z komentarzy `///`, które są przy okazji dokumentacją.

**Szukaj po polsku:** testy jednostkowe w Ruscie · testy integracyjne · `rust cargo test` · `rust doc tests` · `rust cfg test mod tests`
