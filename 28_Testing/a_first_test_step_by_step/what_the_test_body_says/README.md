# Step 2 — What the test body says

[A first test, step by step](../README.md) › **Step 2** · previous: [Step 1](../the_test_comes_first/README.md) · next: [Step 3 — A stub that panics](../a_stub_that_panics/README.md)

**Level:** 101 · for newcomers

**One line:** Five lines of test body hold four ideas the chapter uses without stopping on: a `\` at the end of a line inside a string, a `Vec` of `Vec`s as a table, `assert_eq!` comparing a `Vec` with an array, and a `1.0` whose type comes from what it is compared with.

```rust
let csv = String::from(
    "1,1,1\n\
     1,-1,1\n",
);
let rows: Vec<Vec<f32>> = vec![vec![1.0, 1.0, 1.0], vec![-1.0, -1.0, -1.0]];
assert_eq!(csv, "1,1,1\n1,-1,1\n"); // the indent is not in the string
assert_eq!(rows[1], [-1.0, -1.0, -1.0]); // a Vec<f32> equals an array
```

Each block of [the verified output](#the-verified-output) runs one of these.

## A `\` at the end of the line

Inside a string literal, a backslash as the last character of a line removes the line break **and** the whitespace at the start of the next line. That lets the test indent its CSV with the code and still produce `"1,1,1\n1,-1,1\n…"`, with no spaces. Block 1 prints both versions: with the `\`, and without, where the newline and nine spaces of indent stay in the string.

Two consequences. The `\n` before each `\` is what puts the line breaks back; the `\` alone would join the lines. And a line that is meant to *start* with spaces cannot be written this way, because they are removed too. [Raw strings and escapes](../../../14_Strings/raw_strings_and_escapes/README.md) lists every escape.

`String::from(…)` turns the literal, a `&str`, into an owned `String`, because the function under test takes a `String`. Passing it moves it: `csv` cannot be used after `parse_rows(csv)`. [Making a `String`](../../../14_Strings/making_a_string/README.md) covers the other ways to build one.

## `Vec<Vec<f32>>` is a table of rows

Read the type inside out: `f32` is one number, `Vec<f32>` is one row of them, and `Vec<Vec<f32>>` is a list of rows. The chapter calls it a matrix, with rows first and columns second, so `rows[1][1]` is row 1, column 1. Indexing starts at 0, and an index past the end panics. `rows.get(9)` asks the same question and returns `None` instead (block 2).

Nothing makes the rows the same length. A `Vec<Vec<f32>>` can hold a row of three and a row of two, and the chapter says it will introduce a `Matrix` type later. [Grids and nested `Vec`s](../../../26_Collections/vec_of_vecs/README.md) weighs this shape against a flat `Vec`.

## `assert_eq!` compares a `Vec` with an array

`assert_eq!(rows[3], [-1.0, -1.0, -1.0])` compares a `Vec<f32>` on the left with an array `[f32; 3]` on the right. They are different types, and the comparison compiles because std implements `PartialEq<[U; N]> for Vec<T>`: a `Vec` equals an array when they have the same length and equal elements (block 3). The test can write the expected row as a plain array instead of `vec![…]`.

`assert_eq!` panics when the two differ, and a panic is how a test fails. [What a test asserts](../../what_a_test_asserts/README.md) shows the message it prints.

## The type of `1.0`

The chapter says Rust defaults to `f64`. That is the rule for a float literal with nothing else to decide its type. On its own, `1.0` is an `f64`. In `assert_eq!(rows[0][2], 1.0)`, the left side is an `f32`, so the `1.0` becomes an `f32` (block 4). Checking a literal's type from the context it is used in takes a small generic function, `shared_type`, in the example.

Comparing floats with `==` is exact, and it works here because `1.0` and `-1.0` have exact binary representations. `0.1` does not, and its `f32` and `f64` approximations differ: the last line of block 4 is `false`. [What a float stores](../../../19_Numbers/what_a_float_stores/README.md) explains why.

## Words on this page

| Word | Explained on |
|---|---|
| `let rows: Vec<Vec<f32>>` | [Variables](../../../15_First_Programs/variables/README.md) · [What a type annotation does](../../../15_First_Programs/what_an_annotation_does/README.md) |
| `String::from`, `&str` | [Making a `String`](../../../14_Strings/making_a_string/README.md) · [`String` vs `&str`](../../../14_Strings/string_vs_str/README.md) |
| `\` and `\n` in a string | [Raw strings and escapes](../../../14_Strings/raw_strings_and_escapes/README.md) |
| `Vec<T>`, `vec![…]`, `rows[i]`, `get` | [`Vec`](../../../26_Collections/the_vec/README.md) · [Grids and nested `Vec`s](../../../26_Collections/vec_of_vecs/README.md) |
| `[f32; 3]`, an array | [Arrays and slices](../../../26_Collections/arrays_and_slices/README.md) |
| `f32`, `f64` | [Values](../../../15_First_Programs/values/README.md) · [What a float stores](../../../19_Numbers/what_a_float_stores/README.md) |
| `assert_eq!` | [What a test asserts](../../what_a_test_asserts/README.md) |
| `PartialEq` | [Comparison traits](../../../12_Traits/comparison_traits/README.md) |
| `{:?}` | [Debug and Display](../../../15_First_Programs/debug_vs_display/README.md) |
| moving a `String` into a call | [Ownership and moves](../../../18_Ownership/ownership_and_moves/README.md) |

## The verified output

<!-- output:what_the_test_body_says -->
*Verified output of [`what_the_test_body_says.rs`](examples/what_the_test_body_says.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A \ at the end of a line removes the newline and the next line's indent
   with \:    "1,1,1\n1,-1,1\n"
   without:  "1,1,1\n         1,-1,1"

2. Vec<Vec<f32>>: a Vec of rows, each row a Vec of columns
   rows.len()  = 4, rows[1].len() = 3
   rows[1]     = [1.0, -1.0, 1.0]   <- one row, a Vec<f32>
   rows[1][1]  = -1.0               <- row 1, column 1, an f32
   rows.get(9) = None               <- rows[9] would panic instead

3. assert_eq! compares a Vec with an array: Vec<T> implements PartialEq<[U; N]>
   rows[3] == [-1.0, -1.0, -1.0]  is true
   rows[3] == [-1.0, -1.0]        is false   <- lengths differ
   assert_eq!(rows[3], [-1.0, -1.0, -1.0]) passed

4. The literal 1.0 takes its type from what it is compared with
   1.0 on its own:          f64
   1.0 beside rows[0][2]:   f32
   rows[0][2] == 1.0 is true: 1.0 and -1.0 are exact in binary
   0.1_f32 == 0.1_f64 as f32 is true, 0.1_f32 as f64 == 0.1 is false
```
<!-- /output -->

## If you are coming from another language

**Python.** Adjacent string literals inside parentheses join, so Python writes the CSV as `("1,1,1\n" "1,-1,1\n")`. A `\` at the end of a line inside a normal string also continues it, but it does not remove the next line's indent, which is the part Rust adds. A list of lists plays `Vec<Vec<f32>>`. The comparison behaves differently: `[1.0, -1.0] == (1.0, -1.0)` is `False` in Python, because a list never equals a tuple, while Rust's `Vec` equals an array with the same elements. And Python has one float type, a 64-bit double, so there is no `f32` to be inferred.

## See also

- [Raw strings and escapes](../../../14_Strings/raw_strings_and_escapes/README.md) — `\` at a line end, next to every other escape
- [Grids and nested `Vec`s](../../../26_Collections/vec_of_vecs/README.md) — `Vec<Vec<T>>` against one flat `Vec`
- [What a test asserts](../../what_a_test_asserts/README.md) — `assert_eq!`'s message, and comparing floats
- [Type inference](../../../15_First_Programs/type_inference/README.md) — the `{float}` fallback to `f64`
- [Step 3 — A stub that panics](../a_stub_that_panics/README.md)

## Po polsku

Ciało pierwszego testu ma cztery pomysły, obok których rozdział przechodzi bez zatrzymania. Po pierwsze, **`\` na końcu linii** wewnątrz łańcucha usuwa znak nowej linii **i** wcięcie na początku następnej, dzięki czemu CSV można wciąć razem z kodem, a w łańcuchu nie ma spacji; `\n` przed `\` przywraca podział wiersza. Po drugie, `Vec<Vec<f32>>` czyta się od środka: `f32` to liczba, `Vec<f32>` to wiersz, całość to lista wierszy — macierz (*matrix*), w której `rows[1][1]` to wiersz 1, kolumna 1, a indeks poza zakresem powoduje panikę (`get` zwraca wtedy `None`). Nic nie wymusza równej długości wierszy.

Po trzecie, `assert_eq!(rows[3], [-1.0, -1.0, -1.0])` porównuje `Vec<f32>` z tablicą `[f32; 3]` — to różne typy, ale biblioteka standardowa implementuje `PartialEq<[U; N]> for Vec<T>`, więc wektor jest równy tablicy o tej samej długości i tych samych elementach. W Pythonie lista nigdy nie jest równa krotce. Po czwarte, zdanie „Rust domyślnie używa `f64`” dotyczy literału, któremu nic innego nie wskazuje typu: samo `1.0` to `f64`, ale porównane z `f32` staje się `f32`. Porównanie `==` jest dokładne i działa dla `1.0` i `-1.0`, bo mają dokładną reprezentację binarną — dla `0.1` przybliżenia `f32` i `f64` się różnią.

**Szukaj po polsku:** kontynuacja łańcucha znaków · wektor wektorów · `rust string continuation backslash` · `rust vec partialeq array` · `rust float literal default f64`
