# Step 3 — A stub that panics

[A first test, step by step](../README.md) › **Step 3** · previous: [Step 2](../what_the_test_body_says/README.md) · next: [Step 4 — Making it pass](../making_it_pass/README.md)

**Level:** 101 · for newcomers

**One line:** A function with the right signature and `todo!()` as its body is enough to make the test compile, and the test then fails with *not yet implemented*, which is progress: the build is green, and the harness names the one thing left to write.

```rust
fn parse_rows(_csv: String) -> Vec<Vec<f32>> {
    todo!()
}
```

Added above the test module from [step 1](../the_test_comes_first/README.md), this makes `stub_first.rs`.

## Why `todo!()` compiles in any function

`todo!()` is a macro that panics with the message *not yet implemented*. A panic never returns, so the expression has [the never type `!`](../../../15_First_Programs/the_never_type/README.md), and `!` coerces to any type. The same one line can stand in for a `Vec<Vec<f32>>`, a `usize` or a `String` (verified output below). It also takes a message, like `println!`: `todo!("count the rows")` panics with *not yet implemented: count the rows*.

`unimplemented!()` does the same thing with the message *not implemented*. The two differ only in what they tell a reader: `todo!` means *not written yet*, `unimplemented!` means *not going to be here*.

## Why `_csv`

The stub never reads its parameter, and rustc warns about an unused variable. A leading underscore tells rustc the name is unused on purpose:

```text title="Abridged — real rustc 1.98.0 output for the same stub with csv instead of _csv"
warning: unused variable: `csv`
 --> stub_named.rs:3:15
  |
3 | fn parse_rows(csv: String) -> Vec<Vec<f32>> {
  |               ^^^ help: if this is intentional, prefix it with an underscore: `_csv`
```

The underscore changes nothing for callers: the parameter is still a `String`, and `parse_rows(csv)` still moves one in. In [step 4](../making_it_pass/README.md) it gets used and loses the underscore. [What a warning is asking](../../../15_First_Programs/what_a_warning_is_asking/README.md) explains why `_csv` and `_` are different answers.

## The second run: it compiles, and the test fails

```text title="One run of rustc --edition 2024 --test stub_first.rs -o t && ./t (exit status 101)"
running 1 test
test tests::loads_the_or_table ... FAILED

failures:

---- tests::loads_the_or_table stdout ----

thread 'tests::loads_the_or_table' (29220598) panicked at stub_first.rs:4:5:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::loads_the_or_table

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Read it from the top:

| Line | Says |
|---|---|
| `running 1 test` | the harness found one `#[test]` function |
| `test tests::loads_the_or_table ... FAILED` | the test's path: module `tests`, function `loads_the_or_table` |
| `---- tests::loads_the_or_table stdout ----` | what that test printed, including its panic, collected and shown after the run |
| `thread '…' (29220598) panicked at stub_first.rs:4:5:` | each test runs on its own thread, named after the test; the number is a thread id and changes every run; `4:5` is the `todo!()` |
| `not yet implemented` | the panic message |
| `test result: FAILED. 0 passed; 1 failed; …` | the totals, and the harness exits with status 101 |

The unused-import warning from step 1 is gone: the test now calls `parse_rows`, which `use super::*` brought in.

## In a normal build, the stub is dead code

The test is the only caller of `parse_rows`, and `#[cfg(test)]` removes the test from a normal build. So `rustc stub_first.rs`, with no `--test`, warns:

```text title="Abridged — real rustc 1.98.0 output for rustc --edition 2024 stub_first.rs"
warning: function `parse_rows` is never used
 --> stub_first.rs:3:4
  |
3 | fn parse_rows(_csv: String) -> Vec<Vec<f32>> {
  |    ^^^^^^^^^^
```

The warning stays after step 4, until `main` calls the function. It is [step 1](../the_test_comes_first/README.md#cfgtest-is-decided-when-you-compile)'s point seen from the other side: the two builds contain different code.

## Words on this page

| Word | Explained on |
|---|---|
| `todo!()`, `unimplemented!()` | this page · [Macros](../../../25_Control_Flow/macros/README.md) |
| `!`, the never type | [The never type `!`](../../../15_First_Programs/the_never_type/README.md) |
| `_csv`, unused variables | [What a warning is asking](../../../15_First_Programs/what_a_warning_is_asking/README.md) |
| panic, exit status 101 | [What a panic costs](../../../17_Option_and_Result/what_a_panic_costs/README.md) |
| `RUST_BACKTRACE=1` | [Reading a backtrace](../../../17_Option_and_Result/reading_a_backtrace/README.md) |
| dead code | [What a warning is asking](../../../15_First_Programs/what_a_warning_is_asking/README.md) |
| `catch_unwind`, used by the example | [What a panic costs](../../../17_Option_and_Result/what_a_panic_costs/README.md) |

## The verified output

The example catches each stub's panic with `catch_unwind` and prints its message.

<!-- output:a_stub_that_panics -->
*Verified output of [`a_stub_that_panics.rs`](examples/a_stub_that_panics.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Each stub compiles, and each one panics when it is called:
  parse_rows(..)  -> "not yet implemented"
  a_count()       -> "not yet implemented: count the rows"
  a_label()       -> "not implemented"
```
<!-- /output -->

## If you are coming from another language

**Python.** `raise NotImplementedError` is the equivalent of `todo!()`, and a test runner reports it as an error in the test. Python needs nothing like `!`, because a function body is not type-checked against its return annotation: any body is accepted. The unused-parameter warning has no counterpart in the interpreter; linters such as pylint report it.

**ABAP.** A method implementation left empty compiles, and a test that calls it fails on its assertion, not on the call. The nearest thing to `todo!()` is raising an exception on purpose, and ABAP Unit reports an unexpected exception in a test method as an error.

## See also

- [The never type `!`](../../../15_First_Programs/the_never_type/README.md) — why a panic fits where a `Vec` was expected
- [What a warning is asking](../../../15_First_Programs/what_a_warning_is_asking/README.md) — `_csv` against `_`
- [`unwrap` is a `todo`](../../../02_Errors/unwrap_is_a_todo/README.md) — the other placeholder that panics
- [What a panic costs](../../../17_Option_and_Result/what_a_panic_costs/README.md) — what the harness catches when a test fails
- [Step 4 — Making it pass](../making_it_pass/README.md)

## Po polsku

Żeby test się skompilował, wystarczy funkcja z właściwą sygnaturą i `todo!()` zamiast ciała. `todo!()` to makro, które panikuje z komunikatem *not yet implemented*; panika nigdy nie wraca, więc wyrażenie ma typ `!` (*never type*), a `!` pasuje w miejsce dowolnego typu — ta sama linijka zastępuje `Vec<Vec<f32>>`, `usize` czy `String`. `todo!("…")` dopisuje własny komunikat. `unimplemented!()` działa tak samo z komunikatem *not implemented* i mówi czytelnikowi „tego tu nie będzie”, a nie „jeszcze nie napisane”.

Parametr, którego stub nie czyta, dostaje podkreślenie: `_csv` wyłącza ostrzeżenie *unused variable*, nie zmieniając niczego dla wywołującego. Druga kompilacja testowa już przechodzi, a test pada: raport mechanizmu testów (*test harness*) pokazuje `FAILED`, komunikat paniki wraz z miejscem `todo!()` i podsumowanie `0 passed; 1 failed`, a program kończy się kodem 101. Każdy test działa we własnym wątku nazwanym jak test; liczba w nawiasie to identyfikator wątku i zmienia się przy każdym uruchomieniu. W zwykłej kompilacji `parse_rows` jest martwym kodem (*dead code*), bo jedyne wywołanie siedzi w usuniętym module testów.

**Szukaj po polsku:** zaślepka funkcji · `rust todo! vs unimplemented!` · `rust not yet implemented` · `rust unused variable underscore` · `rust test output FAILED` · `rust function is never used cfg test`
