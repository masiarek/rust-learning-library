# Step 1 — The test comes first

[A first test, step by step](../README.md) › **Step 1** · next: [Step 2 — What the test body says](../what_the_test_body_says/README.md)

**Level:** 101 · for newcomers

**One line:** Four lines turn a function into a unit test — `#[cfg(test)]`, `mod tests`, `use super::*` and `#[test]` — and written before the function it calls, the test's first build fails with `E0425`, which is the expected first result.

```rust title="test_first.rs"
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_or_table() {
        let csv = String::from(
            "1,1,1\n\
             1,-1,1\n\
             -1,1,1\n\
             -1,-1,-1\n",
        );
        let rows: Vec<Vec<f32>> = parse_rows(csv);

        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0][2], 1.0);
        assert_eq!(rows[3], [-1.0, -1.0, -1.0]);
    }
}
```

The body of the test is [step 2](../what_the_test_body_says/README.md). This page is the wrapping around it.

## The four lines

| Line | What it does |
|---|---|
| `#[cfg(test)]` | an [attribute](../../../27_Modules/what_an_attribute_is/README.md): compile the item below only when the `test` cfg is set, which `rustc --test` and `cargo test` do |
| `mod tests { … }` | a module named `tests`, a child of the module the file defines. The name is a convention, not a keyword |
| `use super::*;` | bring every name from the parent module into this one, so the test can call `parse_rows` without a path |
| `#[test]` | mark the function below as a test: the harness calls it, and it passes unless it panics |

Unit tests in a `tests` module at the bottom of the same file is the convention the chapter describes, and it is the one [The Book ↗](https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests) teaches. [Where a test goes](../../where_a_test_goes/README.md) compares it with integration tests in `tests/` and doc tests.

`fn main() {}` is there for the normal build. `rustc --test` and `cargo test` generate their own `main` to run the tests, so a file of nothing but tests builds for testing, but `cargo build` on it stops with `E0601`, *`main` function not found*.

## The first run does not compile

```text title="Abridged — real rustc 1.98.0 output for rustc --edition 2024 --test test_first.rs"
warning: unused import: `super::*`
 --> test_first.rs:5:9
  |
5 |     use super::*;
  |         ^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

error[E0425]: cannot find function `parse_rows` in this scope
  --> test_first.rs:15:35
   |
15 |         let rows: Vec<Vec<f32>> = parse_rows(csv);
   |                                   ^^^^^^^^^^ not found in this scope
```

The error is the point of writing the test first: it names the function the test needs. The warning comes from the glob. `use super::*` brought in every name the parent module has, which here is only `main`, and the test uses none of them. The file is not empty. The warning goes away in [step 3](../a_stub_that_panics/README.md), once `parse_rows` exists and the test calls it through the import.

## `#[cfg(test)]` is decided when you compile

`cfg` stands for *configuration*. `#[cfg(test)]` does not skip a module at run time. When the `test` cfg is not set, the module is removed before the rest of the program is compiled, as if it had never been written. In a test build it is compiled in.

So the question is *which build*, not *are tests being run*. `cargo test --no-run` compiles the test module into a test binary and runs nothing. A normal `cargo build` never compiles it at all.

The example below builds the same file both ways. Exactly one of two constants exists in each build:

```rust
#[cfg(test)]
const BUILD: &str = "a test build";
#[cfg(not(test))]
const BUILD: &str = "a normal build";
```

The normal build prints `This is a normal build` (in the verified output below). Built with `rustc --test`, the same file runs its test instead, and the test asserts the other constant:

```text title="One run of rustc --edition 2024 --test the_test_comes_first.rs"
running 1 test
test tests::this_is_a_test_build ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## `super` is the parent module

`super` names the module one level up. In `test_first.rs` the parent of `tests` is the file's own module, which is why "imports the code from the file" sounds right. But `use` imports **names**, not code, and in a nested module `super` is the module around it. The example's `data::checks` module uses `super::*` and reaches `row_count`, a private function of `data`: the last two lines of [the verified output](#the-verified-output) print the module it ran in and the value it got.

A child module may use its parent's private items. That is what lets a unit test call a private function, and why `use super::*` is the one glob import nobody argues about.

## An attribute can change the program

The chapter describes attributes as metadata that influence compilation, behaviour or tooling without changing the program's logic. `#[cfg(test)]` is a counterexample on the same page: it decides whether code exists. `#[derive]` is another, since it writes code:

```rust
#[derive(PartialEq)] // delete this line and `==` below is error E0369
struct Row(i32);

fn main() {
    println!("{}", Row(1) == Row(1)); // true
}
```

Without the attribute, rustc 1.98.0 says *binary operation `==` cannot be applied to type `Row`*. [What an attribute is](../../../27_Modules/what_an_attribute_is/README.md) sorts attributes into five families. Lint levels and hints such as `#[inline]` leave what the program does alone; `cfg`, `derive` and `test` do not.

## Words on this page

| Word | Explained on |
|---|---|
| `fn main() {}` | [Functions](../../../25_Control_Flow/functions/README.md) |
| `#[cfg(test)]`, `#[test]` | [What an attribute is](../../../27_Modules/what_an_attribute_is/README.md) · [Where a test goes](../../where_a_test_goes/README.md) |
| `mod`, `super`, private items | [Modules and visibility](../../../27_Modules/modules_and_visibility/README.md) |
| `use`, the `*` glob | [The `use` declaration](../../../27_Modules/the_use_declaration/README.md) |
| `const` | [`const` and `static`](../../../27_Modules/const_and_static/README.md) |
| `cargo init`, `cargo test`, `rustc --test` | [A first test, step by step](../README.md#the-steps-in-order-with-the-commands) · [From one `.rs` file to a Cargo project](../../../05_Tooling/from_rustc_to_cargo/README.md) · [Running a scratch program](../../../15_First_Programs/rustc_without_cargo/README.md) |
| `E0425`, `E0601` | [Error codes](../../../ERRORS.md) |
| `#[derive]` | [What an attribute is](../../../27_Modules/what_an_attribute_is/README.md) |

## The verified output

<!-- output:the_test_comes_first -->
*Verified output of [`the_test_comes_first.rs`](examples/the_test_comes_first.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
This is a normal build; cfg!(test) = false

A child module reaches its parent's private items through use super::*:
  running in    the_test_comes_first::data::checks
  row_count()   = 4   <- private to data, visible here
```
<!-- /output -->

## If you are coming from another language

**Python.** Tests usually live in separate `test_*.py` files that pytest or `unittest` discovers, and nothing is compiled out: a test file is simply not imported by the program. The closest thing to `#[cfg(test)]` is the `if __name__ == "__main__":` guard, which decides at *run* time whether a block runs. Rust decides at compile time whether the code exists. A test function in pytest is found by its `test_` name; in Rust the name is free and `#[test]` marks it.

**ABAP.** ABAP Unit is the nearest match. Test classes are local classes declared `FOR TESTING` in the same program or class pool as the code they test, test methods are also marked `FOR TESTING`, and a test class can call the private methods of the class under test if it is declared a friend. The `RISK LEVEL` and `DURATION` additions on a test class have no Rust counterpart. Rust's `#[ignore]` is the nearest thing to keeping a slow test out of the default run.

## See also

- [Where a test goes](../../where_a_test_goes/README.md) — unit tests here, integration tests in `tests/`, and doc tests
- [What an attribute is](../../../27_Modules/what_an_attribute_is/README.md) — `#[cfg]` deletes, `cfg!()` chooses
- [Modules and visibility](../../../27_Modules/modules_and_visibility/README.md) — what `super` and a child module's view of private items rest on
- [Step 2 — What the test body says](../what_the_test_body_says/README.md)

## Po polsku

Test jednostkowy w Ruscie to cztery linijki opakowania. `#[cfg(test)]` to atrybut **kompilacji warunkowej** (*conditional compilation*): moduł pod nim istnieje tylko w kompilacji testowej (`rustc --test`, `cargo test`), a w zwykłej jest usuwany, zanim reszta programu się skompiluje. O tym decyduje więc *rodzaj kompilacji*, a nie to, czy testy są uruchamiane — `cargo test --no-run` kompiluje moduł testów i niczego nie uruchamia. `mod tests` to moduł potomny o umownej nazwie `tests`. `use super::*` wciąga do niego wszystkie **nazwy** z modułu nadrzędnego (*parent module*), także prywatne; na najwyższym poziomie modułem nadrzędnym jest sam plik, ale w zagnieżdżonym module `super` to moduł dookoła. `#[test]` oznacza funkcję, którą wywoła mechanizm uruchamiający testy (*test harness*); test przechodzi, jeśli nie spanikuje.

Pisząc test przed funkcją, pierwszą kompilację kończy się błędem `E0425` (*cannot find function*) — i o to chodzi, bo komunikat nazywa brakującą funkcję. Ostrzeżenie *unused import: `super::*`* nie oznacza pustego pliku: plik ma `main`, tylko test niczego z importu jeszcze nie używa. Puste `fn main() {}` jest potrzebne zwykłej kompilacji — kompilacja testowa generuje własne `main`, więc bez niego `cargo test` działa, a `cargo build` zgłasza `E0601`.

Atrybut **może** zmieniać program: `#[cfg]` decyduje, czy kod w ogóle istnieje, a `#[derive(PartialEq)]` dopisuje implementację, bez której `==` to błąd `E0369`.

**Szukaj po polsku:** kompilacja warunkowa · moduł testów · `rust cfg test` · `rust use super::*` · `rust unused import super` · `rust E0425 cannot find function`
