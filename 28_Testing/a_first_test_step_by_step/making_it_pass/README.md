# Step 4 — Making it pass

[A first test, step by step](../README.md) › **Step 4** · previous: [Step 3](../a_stub_that_panics/README.md)

**Level:** 101 → 201 · for newcomers

**One line:** Two iterator chains, one inside the other, turn the CSV text into `Vec<Vec<f32>>`: `lines` makes rows, `split(',')` makes fields, `parse` makes numbers, and each `collect` builds the `Vec` the return type asks for. Written with `split('\n')` instead of `lines()`, the same loader fails the test.

```rust
fn parse_rows(csv: String) -> Vec<Vec<f32>> {
    csv.lines()
        .map(|line| {
            line.split(',')
                .map(|field| field.trim().parse().unwrap())
                .collect()
        })
        .collect()
}
```

This replaces the stub from [step 3](../a_stub_that_panics/README.md) and makes `pass_first.rs`.

## The third run passes

```text title="One run of rustc --edition 2024 --test pass_first.rs -o t && ./t"
running 1 test
test tests::loads_the_or_table ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stage by stage

| Stage | Takes | Gives |
|---|---|---|
| `csv.lines()` | the `String`, borrowed | a `Lines`: each line, without its `\n` |
| <code>.map(&#124;line&#124; …)</code> | one line, a `&str` | whatever the closure returns: here a `Vec<f32>` |
| `line.split(',')` | the line | a `Split`: the text between commas |
| <code>.map(&#124;field&#124; field.trim().parse().unwrap())</code> | one field | an `f32` |
| inner `.collect()` | the `f32`s of one line | a `Vec<f32>` |
| outer `.collect()` | the rows | a `Vec<Vec<f32>>` |

Block 2 of [the verified output](#the-verified-output) prints what `lines()` and `split(',')` return: `core::str::iter::Lines<'_>` and `core::str::iter::Split<'_, char>`. The chapter calls the result of `lines()` an Iterator and describes `Iterator` as a type. It is a **trait**. `Lines` and `Split` are structs that implement it, and every method in the chain after them (`map`, `collect`) is a method of the `Iterator` trait. `map` returns another iterator, and `collect` consumes one, as the chapter says. Nothing runs until `collect` asks for items: [iterators are lazy](../../../24_Iterators/iterators_are_lazy/README.md).

Neither `parse` nor `collect` names a type, and neither needs to. The return type `Vec<Vec<f32>>` tells the outer `collect` to build a `Vec` of rows. That makes each row a `Vec<f32>`, which tells the inner `collect` what to build, and that makes each field an `f32`, which tells `parse` what to parse. [Type inference](../../../15_First_Programs/type_inference/README.md) solves the whole chain from its end.

`"-1".parse::<f32>()` is `Ok(-1.0)`: text with no decimal point parses as a float.

## Why `trim`, and why `lines` rather than `split('\n')`

`parse` does not skip spaces: `" 1".parse::<f32>()` is an error (block 3). `trim()` removes whitespace at both ends of each field. The chapter's data has no spaces, so here it guards against a CSV written as `1, -1, 1`.

The CSV ends with `\n`. `lines()` treats that as the end of the last line. `split('\n')` treats it as a separator with an empty piece after it, so the table gets a fifth line, `""`. `"".parse::<f32>()` fails with `ParseFloatError { kind: Empty }`, and `unwrap` turns that into a panic:

```text title="Abridged — one run of the test with csv.split('\n') in place of csv.lines()"
running 1 test
test tests::loads_the_or_table ... FAILED

failures:

---- tests::loads_the_or_table stdout ----

thread 'tests::loads_the_or_table' (29226537) panicked at split_first.rs:7:51:
called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Empty }
```

Nothing else about the loader is wrong, and nothing in the message mentions a trailing newline. The test's `\n` at the end of the last row is what caught it.

## `unwrap` is a run-time check

`parse` returns a `Result<f32, ParseFloatError>`: `Ok` with the number, or `Err` with why it failed. `unwrap()` returns the number from an `Ok` and panics on an `Err`. The chapter describes it as telling the compiler that the result will work. The compiler is not told anything, and it checks nothing:

```rust
let text = "x";
let n: f32 = text.parse().unwrap(); // compiles with no warning, panics when it runs
```

That program builds cleanly and exits with status 101: *called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }*. Using `unwrap` here is a choice the chapter makes for a teaching example with data it controls, and it says so. [`unwrap` is a `todo`](../../../02_Errors/unwrap_is_a_todo/README.md) is about when that choice is fine.

The chapter spells the steps out with an explicit `Result<f32, ParseFloatError>` annotation. As printed, that snippet does not compile, because `ParseFloatError` is not in scope:

```text title="Abridged — real rustc 1.98.0 output for no_import.rs"
error[E0425]: cannot find type `ParseFloatError` in this scope
 --> no_import.rs:3:29
  |
3 |     let result: Result<f32, ParseFloatError> = text.parse();
  |                             ^^^^^^^^^^^^^^^ not found in this scope
  |
help: consider importing this struct
  |
1 + use std::num::ParseFloatError;
  |
```

Add `use std::num::ParseFloatError;` at the top of the file, as the help line says.

## Words on this page

| Word | Explained on |
|---|---|
| `.lines()` | [`str::lines`](../../../14_Strings/str_methods/str_lines/README.md) |
| `.split(',')` | [`str::split`](../../../14_Strings/str_methods/str_split/README.md) |
| `.trim()` | [`str::trim`](../../../14_Strings/str_methods/str_trim/README.md) |
| `.parse()`, `ParseFloatError` | [Parsing a string](../../../14_Strings/parsing_a_string/README.md) · [`str::parse`](../../../14_Strings/str_methods/str_parse/README.md) |
| <code>&#124;line&#124; …</code>, a closure | [What a closure is](../../../23_Closures/what_a_closure_is/README.md) |
| `.map(…)` | [Adapters by job](../../../24_Iterators/adapters_by_job/README.md) |
| `.collect()` | [Collect into a `Vec`](../../../24_Iterators/collect_into_a_vec/README.md) |
| `Iterator`, lazy | [Iterators](../../../24_Iterators/README.md) · [Iterators are lazy](../../../24_Iterators/iterators_are_lazy/README.md) |
| `Result`, `Ok`, `Err` | [`Ok` and `Err`](../../../17_Option_and_Result/ok_and_err/README.md) |
| `.unwrap()` | [What `unwrap` does](../../../17_Option_and_Result/what_unwrap_does/README.md) · [`unwrap` is a TODO](../../../02_Errors/unwrap_is_a_todo/README.md) |
| `use std::num::ParseFloatError` | [The `use` declaration](../../../27_Modules/the_use_declaration/README.md) |
| `type_name_of_val`, used by the example | [Type inference](../../../15_First_Programs/type_inference/README.md) |

## The verified output

The example runs the test's three assertions in `main`, prints each stage's type, and catches the panic from the `split('\n')` version.

<!-- output:making_it_pass -->
*Verified output of [`making_it_pass.rs`](examples/making_it_pass.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The test's three assertions, checked here without the harness
   all three hold: [[1.0, 1.0, 1.0], [1.0, -1.0, 1.0], [-1.0, 1.0, 1.0], [-1.0, -1.0, -1.0]]

2. What each stage hands to the next
   csv.lines()          is a core::str::iter::Lines<'_>
   first line           = "1,1,1"
   line.split(',')      is a core::str::iter::Split<'_, char>
   its items            = ["1", "1", "1"]
   "-1".parse::<f32>()  = Ok(-1.0)   <- integer text parses as a float
   inner collect()      -> Vec<f32>, outer collect() -> Vec<Vec<f32>>, both from the return type

3. Why trim(), and why lines() rather than split('\n')
   " 1".parse::<f32>()   = Err(ParseFloatError { kind: Invalid })
   lines()        gives ["1,1", "-1,1"]
   split('\n')    gives ["1,1", "-1,1", ""]
   "".parse::<f32>()    = Err(ParseFloatError { kind: Empty })
   the loader rewritten with split('\n') panics: called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Empty }
```
<!-- /output -->

## If you are coming from another language

**Python.** The loader is a nested list comprehension: `[[float(f) for f in line.split(",")] for line in csv.splitlines()]`. Two differences show up in the traps above. `float(" 1")` is `1.0`, because Python's `float` strips whitespace itself, so `trim` has no Python counterpart here. And `str.split("\n")` has the same trailing empty string as Rust's `split('\n')`: `"a\nb\n".split("\n")` is `['a', 'b', '']`, while `splitlines()` gives `['a', 'b']`. Where Rust's `unwrap` panics, `float("")` raises `ValueError: could not convert string to float: ''`.

**ABAP.** `SPLIT lv_line AT ',' INTO TABLE lt_fields` is the `split(',')`, and a loop over the lines builds the table row by row. Converting a field to a number is an assignment that raises a conversion exception on bad input, the nearest thing to `unwrap`. Rust's version is one expression because the target type flows back from the return type through both `collect`s.

## See also

- [Iterators are lazy](../../../24_Iterators/iterators_are_lazy/README.md) — nothing runs until `collect`
- [Parsing a string](../../../14_Strings/parsing_a_string/README.md) — `parse`, its errors, and choosing the type
- [`unwrap` is a `todo`](../../../02_Errors/unwrap_is_a_todo/README.md) — when a panic on bad input is acceptable
- [Type inference](../../../15_First_Programs/type_inference/README.md) — how the return type decides both `collect`s
- [A first test, step by step](../README.md) — the path, the data, and the chapter's statements in one table

## Po polsku

Test przechodzi dzięki dwóm łańcuchom iteratorów, jednemu w drugim: `lines()` daje wiersze, `split(',')` pola, `parse()` liczby, a każde `collect()` buduje `Vec`, którego żąda typ zwracany. Ani `parse`, ani `collect` nie podają typu — `Vec<Vec<f32>>` w sygnaturze wyznacza zewnętrzne `collect`, to wyznacza wewnętrzne, a to z kolei `parse`. Rozdział nazywa `Iterator` typem, ale to **cecha** (*trait*): `Lines` i `Split` to struktury, które ją implementują, a `map` i `collect` to metody tej cechy. Iteratory są leniwe — nic się nie dzieje, dopóki `collect` nie zażąda elementów.

Dwie pułapki warto znać. `parse` nie pomija spacji, więc `" 1"` to błąd, a `trim()` go usuwa (Pythonowe `float(" 1")` spacje obcina samo). CSV kończy się znakiem `\n`: `lines()` traktuje go jako koniec ostatniego wiersza, a `split('\n')` jako separator, po którym jest jeszcze pusty kawałek `""` — `"".parse::<f32>()` zwraca `ParseFloatError { kind: Empty }`, `unwrap` zamienia to w panikę i test pada.

`unwrap()` niczego nie obiecuje kompilatorowi: to sprawdzenie w czasie działania, które zwraca wartość z `Ok` albo panikuje na `Err`, a `"x".parse::<f32>().unwrap()` kompiluje się bez ostrzeżenia. Fragment z jawną adnotacją `Result<f32, ParseFloatError>` potrzebuje jeszcze `use std::num::ParseFloatError;` — bez tego rustc 1.98.0 zgłasza `E0425`.

**Szukaj po polsku:** łańcuch iteratorów · parsowanie liczb z tekstu · `rust lines vs split newline` · `rust parse f32 trim` · `rust collect vec of vec` · `rust unwrap panic ParseFloatError`
