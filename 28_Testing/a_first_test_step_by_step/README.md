# A first test, step by step

**Level:** 101 → 201 · a path through one test

**One line:** The first test in *Rust for Machine Learning* checks that four lines of CSV load into a `Vec<Vec<f32>>`. It is written before the function it tests, so the test build goes through three states: it does not compile, it compiles and fails, and it passes. Four pages, one per step, and every new idea on the way is explained where it first appears.

```rust
fn parse_rows(csv: String) -> Vec<Vec<f32>> {
    csv.lines()
        .map(|line| line.split(',').map(|field| field.trim().parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_or_table() {
        let rows = parse_rows(String::from("1,1,1\n-1,-1,-1\n"));
        assert_eq!(rows[1], [-1.0, -1.0, -1.0]);
    }
}
```

Run it with `rustc --edition 2024 --test first_test.rs -o /tmp/t && /tmp/t`, which prints `test tests::loads_the_or_table ... ok`.

This follows chapter 1 of the book from "A Dataset for AND and OR Functions" to "Making the Test Pass". The chapter's opening, the `and` and `or` functions, is checked on [its own page](../../15_First_Programs/and_or_claims_checked/README.md). The code on these pages is written for this library, so names differ from the book's (`parse_rows` here, a longer name there), but each step does the same thing the chapter does.

## The data, before the Rust

The test loads the truth table for OR, and the chapter introduces machine-learning vocabulary for it:

| Term | Also called | In the OR table |
|---|---|---|
| **dataset** | | the whole table, four rows |
| **feature** | attribute, input variable | the columns `A` and `B` |
| **label** | outcome, output variable | the column `A or B` |
| **example** | instance | one row, such as `True, False, True` |
| **class** | | a value the label can take: `True` or `False` |

The program cannot read `True` and `False` as numbers, so the chapter writes `True` as `1` and `False` as `-1`, one example per line, with the label last:

```text
1,1,1
1,-1,1
-1,1,1
-1,-1,-1
```

`1` and `-1` rather than `1` and `0` is a choice. It suits the perceptron the chapter builds next, whose output is either −1 or 1. Loaded, each line becomes a `Vec<f32>` of three numbers, and the table becomes a `Vec<Vec<f32>>`.

## The steps

| Step | Page | The test build | New on that page |
|---|---|---|---|
| 1 | [The test comes first](the_test_comes_first/README.md) | does not compile: `E0425`, plus an unused-import warning | `#[cfg(test)]`, `mod tests`, `use super::*`, `#[test]` |
| 2 | [What the test body says](what_the_test_body_says/README.md) | — | the `\` line continuation, `Vec<Vec<f32>>`, indexing, a `Vec` compared with an array, the type of `1.0` |
| 3 | [A stub that panics](a_stub_that_panics/README.md) | compiles, and the test fails: *not yet implemented* | `todo!()`, the `_` prefix on a parameter, reading the harness report |
| 4 | [Making it pass](making_it_pass/README.md) | passes | `lines`, `split`, `trim`, `parse`, `unwrap`, two `collect`s, and the `split('\n')` that fails |

## What the chapter says, run

Each row links to the section that runs it.

| The chapter says | Verdict | Run on |
|---|---|---|
| Unit tests go in a module in the same file | holds | [step 1](the_test_comes_first/README.md) |
| `#[cfg(test)]` compiles the module only when tests are run | narrower: it is compiled in any **test build**, and `cargo test --no-run` compiles it without running anything | [step 1](the_test_comes_first/README.md#cfgtest-is-decided-when-you-compile) |
| `use super::*` imports the code from the file | narrower: it brings the **names** of the parent module into scope, private ones included, and `super` is the parent module, which is the file only at the top level | [step 1](the_test_comes_first/README.md#super-is-the-parent-module) |
| Attributes influence compilation without changing the logic | does not hold: `#[cfg]` removes code, and `#[derive]` writes it | [step 1](the_test_comes_first/README.md#an-attribute-can-change-the-program) |
| The unused-import warning appears because the file is empty | narrower: the file has `main`; the warning means nothing the glob brought in is used yet | [step 1](the_test_comes_first/README.md#the-first-run-does-not-compile) |
| Rust has four primary scalar types and two primitive compound types | holds: `bool`, integers, floats and `char`; tuples and arrays | [Values](../../15_First_Programs/values/README.md) |
| `String` stores text as a vector of bytes | holds: its one field is a `Vec<u8>`, which is always valid UTF-8 | [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) |
| `assert_eq!` checks that its two arguments have the same value | holds, and they need not be the same type: it compares with `==`, so a `Vec` can equal an array | [step 2](what_the_test_body_says/README.md#assert_eq-compares-a-vec-with-an-array) |
| Rust defaults to `f64` | narrower: a float literal with nothing to decide its type falls back to `f64`, and the test's `1.0` is an `f32` | [step 2](what_the_test_body_says/README.md#the-type-of-10) |
| `todo!()` fails with *not yet implemented* | holds | [step 3](a_stub_that_panics/README.md) |
| `Iterator` is a type | does not hold: it is a trait, and `lines()` returns `Lines`, a struct that implements it | [step 4](making_it_pass/README.md#stage-by-stage) |
| `unwrap()` tells the compiler the parse will work | does not hold: the compiler checks nothing, and a bad string panics at run time | [step 4](making_it_pass/README.md#unwrap-is-a-run-time-check) |
| The explicit `parse` snippet, `Result<f32, ParseFloatError>` | does not compile as printed: it needs `use std::num::ParseFloatError;` | [step 4](making_it_pass/README.md#unwrap-is-a-run-time-check) |

## The steps, in order, with the commands

The chapter runs everything through Cargo. Each command below was run with Cargo 1.98.0 on the files from the step pages; paths and timings are trimmed from the output.

**1. Make a folder and write the test first.** Put the test from [step 1](the_test_comes_first/README.md) in `main.rs`, with an empty `fn main() {}` above it.

**2. Turn the folder into a Cargo package.**

```bash
cargo init
```

Because `main.rs` already exists, `cargo init` does not create `src/`. It writes a `Cargo.toml` whose `[[bin]]` section points at the file, `path = "main.rs"`. Outside an existing Git repository it also runs `git init` and writes a `.gitignore`; inside one it does neither. [From one `.rs` file to a Cargo project](../../05_Tooling/from_rustc_to_cargo/README.md) covers `cargo init` in full, including the file name it silently ignores, and [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) compares it with `cargo new` and with bare `rustc`.

**3. Run the tests: the build fails.**

```bash
cargo test
```

```text title="Abridged — cargo test, first run (exit status 101)"
warning: unused import: `super::*`
 --> main.rs:5:9
error[E0425]: cannot find function `parse_rows` in this scope
  --> main.rs:15:35
warning: `play` (bin "play" test) generated 1 warning
error: could not compile `play` (bin "play" test) due to 1 previous error; 1 warning emitted
```

`(bin "play" test)` is Cargo saying which build failed: the test build of the binary target. [Step 1](the_test_comes_first/README.md#the-first-run-does-not-compile) reads both messages.

**4. Add a stub** with the right signature and [`todo!()`](a_stub_that_panics/README.md) as its body, above the test.

**5. Run the tests again: it builds, and the test fails.**

```text title="Abridged — cargo test, second run (exit status 101)"
     Running unittests main.rs (target/debug/deps/play-74c017eb9950b1a4)

running 1 test
test tests::loads_the_or_table ... FAILED

---- tests::loads_the_or_table stdout ----
thread 'tests::loads_the_or_table' (29429874) panicked at main.rs:4:5:
not yet implemented

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin play`
```

`Running unittests main.rs` names the file whose unit tests are running, and the path in brackets is the test binary Cargo built. The last line is Cargo's advice for running only this target again. [Step 3](a_stub_that_panics/README.md#the-second-run-it-compiles-and-the-test-fails) reads the report line by line.

**6. Replace the stub with the loader** from [step 4](making_it_pass/README.md).

**7. Run the tests a third time: it passes.**

```text title="Abridged — cargo test, third run (exit status 0)"
running 1 test
test tests::loads_the_or_table ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Without Cargo, the same three states come from `rustc --edition 2024 --test main.rs -o /tmp/t && /tmp/t`: `--test` builds the test harness in place of your `main`.

## Every keyword and command, and where it is explained

| Word or command | What it is here | Explained on |
|---|---|---|
| **Commands** | | |
| `cargo init` | makes the current folder a Cargo package | [step list above](#the-steps-in-order-with-the-commands) · [From one `.rs` file to a Cargo project](../../05_Tooling/from_rustc_to_cargo/README.md) |
| `cargo test` | builds the test harness and runs every `#[test]` | [Where a test goes](../where_a_test_goes/README.md) · [cargo-nextest](../../05_Tooling/nextest/README.md) |
| `cargo test --no-run` | builds the test binary and runs nothing | [step 1](the_test_comes_first/README.md#cfgtest-is-decided-when-you-compile) |
| `cargo build` | the normal build, where `#[cfg(test)]` code does not exist | [step 3](a_stub_that_panics/README.md#in-a-normal-build-the-stub-is-dead-code) |
| `rustc --test` | the same test harness, without Cargo | [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) |
| **Attributes and modules** | | |
| `#[cfg(test)]` | keep the item only in a test build | [step 1](the_test_comes_first/README.md) · [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) |
| `#[test]` | mark a function as a test | [step 1](the_test_comes_first/README.md) · [Where a test goes](../where_a_test_goes/README.md) |
| `mod tests` | a child module holding the tests | [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) |
| `use super::*` | import every name from the parent module | [step 1](the_test_comes_first/README.md#super-is-the-parent-module) · [The `use` declaration](../../27_Modules/the_use_declaration/README.md) |
| **Functions and closures** | | |
| `fn`, `->` | a function and its return type | [Functions](../../25_Control_Flow/functions/README.md) |
| `_csv` | a parameter the body does not use yet | [step 3](a_stub_that_panics/README.md#why-_csv) · [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) |
| <code>&#124;line&#124; …</code> | a closure, the function `map` calls on each item | [What a closure is](../../23_Closures/what_a_closure_is/README.md) |
| **Types** | | |
| `let x: Vec<Vec<f32>>` | a variable with a type annotation | [Variables](../../15_First_Programs/variables/README.md) · [What a type annotation does](../../15_First_Programs/what_an_annotation_does/README.md) |
| `bool`, integers, `f32`/`f64`, `char` | the four scalar types | [Values](../../15_First_Programs/values/README.md) · [What a float stores](../../19_Numbers/what_a_float_stores/README.md) |
| tuples, arrays, slices | the compound types the chapter names | [Tuples](../../26_Collections/tuples/README.md) · [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) |
| `String`, `&str`, `String::from` | owned text, borrowed text, and one way to make the first | [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) · [Making a `String`](../../14_Strings/making_a_string/README.md) |
| `Vec<T>`, `<T>` | a growable list, and the generic parameter in its name | [`Vec`](../../26_Collections/the_vec/README.md) · [What a generic is](../../22_Generics/what_a_generic_is/README.md) |
| `Vec<Vec<f32>>` | a table of rows | [step 2](what_the_test_body_says/README.md) · [Grids and nested `Vec`s](../../26_Collections/vec_of_vecs/README.md) |
| `HashMap` | the key-value map the chapter mentions | [`HashMap`](../../26_Collections/the_hashmap/README.md) |
| `Result<T, E>`, `ParseFloatError` | the value or the reason there is none | [`Ok` and `Err`](../../17_Option_and_Result/ok_and_err/README.md) · [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) |
| **Macros** | | |
| `name!(…)` | a macro call, not a function call | [Macros](../../25_Control_Flow/macros/README.md) · [The braces take a name](../../15_First_Programs/braces_take_a_name/README.md) |
| `assert_eq!` | panic unless the two sides are `==` | [step 2](what_the_test_body_says/README.md#assert_eq-compares-a-vec-with-an-array) · [What a test asserts](../what_a_test_asserts/README.md) |
| `todo!()` | a placeholder that panics | [step 3](a_stub_that_panics/README.md) · [The never type `!`](../../15_First_Programs/the_never_type/README.md) |
| `vec![…]` | build a `Vec` from a list | [`Vec`](../../26_Collections/the_vec/README.md) |
| **Strings and iterators** | | |
| `.lines()` | split text into lines, dropping `\n` | [step 4](making_it_pass/README.md) · [`str::lines`](../../14_Strings/str_methods/str_lines/README.md) |
| `.split(',')` | the pieces between commas | [`str::split`](../../14_Strings/str_methods/str_split/README.md) |
| `.trim()` | remove whitespace at both ends | [`str::trim`](../../14_Strings/str_methods/str_trim/README.md) |
| `.parse()` | text to a number, as a `Result` | [Parsing a string](../../14_Strings/parsing_a_string/README.md) · [`str::parse`](../../14_Strings/str_methods/str_parse/README.md) |
| `.map(…)` | an adapter: transform each item | [Adapters by job](../../24_Iterators/adapters_by_job/README.md) · [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) |
| `.collect()` | a consumer: build a collection | [Collect into a `Vec`](../../24_Iterators/collect_into_a_vec/README.md) |
| `Iterator` | the trait `Lines` and `Split` implement | [Iterators](../../24_Iterators/README.md) · [step 4](making_it_pass/README.md#stage-by-stage) |
| **Errors and failures** | | |
| `.unwrap()` | the value, or a panic | [What `unwrap` does](../../17_Option_and_Result/what_unwrap_does/README.md) · [`unwrap` is a TODO](../../02_Errors/unwrap_is_a_todo/README.md) |
| panic, exit status 101 | how a Rust program or a test fails | [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) |
| `E0425` | a name that is not in scope | [step 1](the_test_comes_first/README.md#the-first-run-does-not-compile) · [Error codes](../../ERRORS.md) |

## See also

- [Where a test goes](../where_a_test_goes/README.md) — unit, integration and doc tests, and `#[should_panic]`
- [What a test asserts](../what_a_test_asserts/README.md) — `assert!` against `assert_eq!`, custom messages, and comparing floats
- [`and`, `or` and a first program's explanation, run](../../15_First_Programs/and_or_claims_checked/README.md) — the pages of the chapter before this one
- [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) — the five families `#[cfg(test)]` and `#[test]` belong to
- [Books](../../10_Resources/books/README.md#also-on-the-shelf) — *Rust for Machine Learning* on the shelf

## Po polsku

Pierwszy test w książce *Rust for Machine Learning* sprawdza, czy cztery wiersze CSV z tablicą prawdy dla OR wczytują się do `Vec<Vec<f32>>`. Test powstaje **przed** funkcją, którą sprawdza (to *test-driven development*, TDD), więc kompilacja testowa przechodzi przez trzy stany: nie kompiluje się (`E0425`), kompiluje się i test pada (*not yet implemented* z `todo!()`), wreszcie przechodzi. Każdy krok ma własną stronę, a każde nowe pojęcie jest wyjaśnione tam, gdzie pojawia się pierwszy raz.

Słownictwo uczenia maszynowego z tego rozdziału: **zbiór danych** (*dataset*) to cała tabela; **cechy** (*features*, też *attributes*, *input variables*) to kolumny `A` i `B`; **etykieta** (*label*, *outcome*) to kolumna z wynikiem; **przykład** (*example*, *instance*) to jeden wiersz; **klasa** (*class*) to wartość, jaką może przyjąć etykieta — tu `True` albo `False`. W pliku `True` zapisuje się jako `1`, a `False` jako `-1`, z etykietą w ostatniej kolumnie.

Kilka zdań z rozdziału nie wytrzymuje uruchomienia: atrybuty *zmieniają* program (`#[cfg]` usuwa kod, `#[derive]` go dopisuje), `Iterator` to cecha (*trait*), a nie typ, a `unwrap()` niczego nie obiecuje kompilatorowi — to sprawdzenie w czasie działania, które panikuje. Inne są węższe, niż brzmią: `#[cfg(test)]` decyduje przy kompilacji, a nie przy uruchamianiu testów; `use super::*` importuje **nazwy** z modułu nadrzędnego, a nie „kod z pliku”; literał `1.0` jest typu `f64` tylko wtedy, gdy nic innego nie wskazuje typu — w tym teście jest `f32`.

**Szukaj po polsku:** testy jednostkowe w Ruscie · test-driven development · `rust cfg test mod tests` · `rust use super::*` · `rust todo! not yet implemented` · `rust vec compare array assert_eq`
