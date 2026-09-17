# Arrays in and out of functions

[Arrays: the map](../README.md) › **Lesson 2** · previous: [Writing an array down](../writing_an_array_down/README.md) · next: [Where an array lives](../where_an_array_lives/README.md)

**Level:** 101 → 201 · for newcomers

**One line:** An array crosses a function boundary like any other value: returned, all its elements come back; taken by value, it must be exactly the right length. A slice is a borrow, so returning one needs something that outlives the call — the caller's array, or a constant the compiler has promoted to `'static`.

```rust
fn create_array() -> [i32; 3] {
    [1, 2, 3]
}

fn create_slice() -> &'static [i32] {
    &[1, 2, 3]
}

fn create_mut_slice(arr: &mut [i32]) -> &mut [i32] {
    &mut arr[1..3]
}

fn main() {
    println!("{:?} {:?}", create_array(), create_slice()); // [1, 2, 3] [1, 2, 3]
    let mut arr = [1, 2, 3, 4];
    let slice = create_mut_slice(&mut arr);
    slice[0] = 10;
    println!("{arr:?}"); // [1, 10, 3, 4]
}
```

A word used below: the **parameter** is the name in the signature (`arr: [i32; 3]`), and the **argument** is the value at the call (`[1, 2, 3]`).

## Returning an array returns its elements

`create_array` hands back a `[i32; 3]`: twelve bytes of `i32`, with no pointer and nothing on the heap. There is nothing to free and nothing that could dangle. The size of what is returned grows with the length, so a big array is returned the way any big value is: the caller provides the room.

## Returning `&'static [i32]`: a promoted constant

`create_slice` returns a reference, which is only allowed if what it points at outlives the function. `[1, 2, 3]` is a constant expression, so the compiler **promotes** it: it stores the array once in the program's static data and gives out a reference to that. An explanation that says `'static` "means the data pointed to by this reference will not be deallocated for the entire runtime of the program" is right about this case. The data is part of the binary and nothing ever frees it.

Promotion only works for what could have been written as a constant. Put one run-time value in the array and it becomes a temporary in the function's own frame:

```rust
fn create_slice(x: i32) -> &'static [i32] {
    // &[x, 1, 2] — E0515: cannot return reference to temporary value
    let _ = x;
    &[0, 1, 2]
}
```

The refusal, with rustc's full text, is [error 27](../array_errors/README.md#27-returning-x-1-2-as-static-i32). The fix is to return the array itself, `[i32; 3]`, or a `Vec` when the length varies. [`&'static str`](../../../14_Strings/static_str/README.md) is the same idea for text, where every string literal is already static.

## Returning part of the caller's array

`create_mut_slice(&mut arr)` returns `&mut arr[1..3]`. Nobody wrote a lifetime, and none is needed: with one reference in and one out, elision ties the result to `arr`. The slice starts where the range starts, so **index 0 of the slice is index 1 of the array**, and `slice[0] = 10` turned `arr[1]` from 2 into 10.

While `slice` is in use, `arr` is borrowed mutably and cannot be read. That is the borrow checker, and [Borrowing](../../../18_Ownership/borrowing/README.md) covers it.

## A getter returns a copy, or refuses

```rust
struct Readings {
    data: [i32; 3],
}

impl Readings {
    fn get_array(&self) -> [i32; 3] {
        self.data
    }
}

fn main() {
    let r = Readings { data: [7, 8, 9] };
    let mut copy = r.get_array();
    copy[0] = 0;
    println!("{copy:?} {:?}", r.data); // [0, 8, 9] [7, 8, 9]
}
```

`[i32; 3]` is `Copy` because `i32` is, so `self.data` behind `&self` is copied out, and changing the copy leaves the field alone. Make the field `[String; 3]` and the same body is [`E0507`: cannot move out of `self.names` which is behind a shared reference](../array_errors/README.md#26-a-getter-that-returns-an-array-of-strings). Return `&[String; 3]` instead, or clone when the caller needs its own.

## Taking an array: exactly N, or any length

A parameter typed `[i32; 3]` takes exactly three elements, and the call is checked at compile time:

```text title="Abridged — rustc 1.98.0 on wrong_length_argument.rs"
error[E0308]: mismatched types
 --> wrong_length_argument.rs:6:17
  |
6 |     print_array([1, 2, 3, 4]);
  |     ----------- ^^^^^^^^^^^^ expected an array with a size of 3, found one with a size of 4
  |     |
  |     arguments to this function are incorrect
```

Older compilers said *"expected an array with a fixed size of 3 elements, found one with 4 elements"*. The full 1.98.0 transcript is [error 2](../array_errors/README.md#2-a-four-element-array-where-a-function-takes-three).

Taking `&[i32]` accepts any length, a `Vec`, and part of either. [Arrays and slices](../../arrays_and_slices/README.md) makes that the default for a parameter, and [its kata](../../arrays_and_slices/README.md#practice) counts which callers each signature turns away. Keep `[i32; N]` for when the length is a fact about the problem, like the 32 bytes of a hash.

## Const generics: every length, and the same length back

```rust
fn doubled<const N: usize>(xs: [i32; N]) -> [i32; N] {
    xs.map(|x| x * 2)
}

fn main() {
    println!("{:?}", doubled([1, 2]));          // [2, 4]
    println!("{:?}", doubled([1, 2, 3, 4, 5])); // [2, 4, 6, 8, 10]
}
```

`const N: usize` makes the length a parameter. One function then serves every length, and the signature promises the result is as long as the argument, which `&[i32] -> Vec<i32>` cannot say. `[i32; N]::map` keeps the length. [The kata](#practice) uses the same idea to transpose a matrix.

## If you are coming from another language

- **C.** A C function cannot return an array at all: `int make(void)[3];` is *"function cannot return array type"*, so C wraps it in a struct or fills a buffer the caller passes in. A parameter written `int xs[3]` is really a pointer, and the 3 is ignored, which is why [`sizeof` inside the function gives the pointer's size ↗](https://masiarek.github.io/c-learning-library/03_Strings/a_string_is_bytes_up_to_a_nul/index.html). In Rust, `[i32; 3]` as a parameter really is three `i32`s, and `&[i32]` is the pointer with its length attached.
- **Go.** Go arrays are values like Rust's: `func double(xs [3]int) [3]int` gets a copy, and changing it leaves the caller's array alone. Go code usually passes a slice, `[]int`, the way Rust code passes `&[T]`. Go has no const generics over lengths, so a function taking `[3]int` takes only that type.
- **Java.** `int[]` is a reference, so passing one lets the method change the caller's array, and `int[]` says nothing about the length. `final int[]` only stops you reassigning the variable; the elements can still change. The Rust distinction between `[i32; 3]` (a copy), `&[i32]` (read) and `&mut [i32]` (write) has no Java spelling.
- **Python.** A list passed to a function is the same list, and `return [1, 2, 3]` makes a new heap object each call. The closest thing to a promoted `&'static [i32]` is a module-level tuple.

---

## The verified output

<!-- output:arrays_in_signatures -->
*Verified output of [`arrays_in_signatures.rs`](examples/arrays_in_signatures.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Returning an array returns its elements
   create_array() = [1, 2, 3], a [i32; 3]

2. Returning &'static [i32]: a promoted constant
   create_slice() = [1, 2, 3], a &[i32]
   let promoted: &'static [i32] = &[1, 2, 3];  -> [1, 2, 3]
   With a run-time element, &[x, 1, 2] is a temporary: E0515.

3. Returning part of the caller's array
   slice = [2, 3], len 2
   slice[0] = 10  ->  arr = [1, 10, 3, 4]
   slice index 0 is array index 1: the view starts where the range did

4. A getter on a Copy array returns a copy
   copy = [0, 8, 9], r.data = [7, 8, 9]  (the field did not change)
   With data: [String; 3] the same body is E0507: cannot move out

5. Parameters: exactly N, or any length
   print_array([1, 2, 3])     -> [1, 2, 3]
   print_array([1, 2, 3, 4])  -> E0308, expected an array with a size of 3
   print_slice(&[1, 2, 3, 4]) -> 4 elements: [1, 2, 3, 4]
   print_slice(&arr[..2])     -> 2 elements: [1, 10]

6. Const generics: every length, and the same length back
   doubled([1, 2])          = [2, 4], a [i32; 2]
   doubled([1, 2, 3, 4, 5]) = [2, 4, 6, 8, 10], a [i32; 5]
```
<!-- /output -->

## Practice

**Transpose a matrix.** Turn `[[1, 2, 3], [4, 5, 6], [7, 8, 9]]` into `[[1, 4, 7], [2, 5, 8], [3, 6, 9]]`: row *i* of the input becomes column *i* of the output. Write it first as two index loops into a zeroed `[[0; 3]; 3]`. Run clippy on it and note whether [`needless_range_loop`](../array_lints/README.md#needless_range_loop) fires, and why or why not.

Then make it work for every square size with `const N: usize` and `std::array::from_fn` inside `std::array::from_fn`. Then for any shape, where the type turns `[[T; C]; R]` into `[[T; R]; C]`. Check that transposing twice gives back the original.

A formatting tip for the matrix literal. rustfmt 1.9.0, the one shipped with 1.98.0, collapses a short `[[1, 2, 3], [4, 5, 6], [7, 8, 9]]` written over three lines onto one. A `//` comment after the first row keeps one row per line:

```rust
fn main() {
    let matrix = [
        [1, 2, 3], //
        [4, 5, 6],
        [7, 8, 9],
    ];
    println!("{matrix:?}");
}
```

`#[rustfmt::skip]` on the `let` does the same, for any layout you like.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:transpose_kata -->
*[`transpose_kata.rs`](examples/transpose_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: transpose a 3x3 matrix, then every square matrix, then any.
//!
//!   rustc --edition 2024 transpose_kata.rs -o /tmp/tk && /tmp/tk

/// The first draft: two index loops. Row i of the input becomes column i.
fn transpose_3x3(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}

/// Every square size: N comes from the argument.
fn transpose<const N: usize>(m: [[i32; N]; N]) -> [[i32; N]; N] {
    std::array::from_fn(|i| std::array::from_fn(|j| m[j][i]))
}

/// Any shape: R rows of C become C rows of R. The type says so.
fn transpose_rect<T: Copy, const R: usize, const C: usize>(m: [[T; C]; R]) -> [[T; R]; C] {
    std::array::from_fn(|c| std::array::from_fn(|r| m[r][c]))
}

fn main() {
    let matrix = [
        [1, 2, 3], //
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("1. Two index loops");
    println!("   {:?}", transpose_3x3(matrix));

    println!();
    println!("2. from_fn inside from_fn, for any N");
    println!("   {:?}", transpose(matrix));
    println!("   2x2: {:?}", transpose([[101, 102], [201, 202]]));
    println!("   twice is the identity: {}", transpose(transpose(matrix)) == matrix);

    println!();
    println!("3. Not square: [[T; 3]; 2] in, [[T; 2]; 3] out");
    let wide = [['a', 'b', 'c'], ['d', 'e', 'f']];
    let tall = transpose_rect(wide);
    println!("   {wide:?}");
    println!("   {tall:?}");
    println!("   {} -> {}", std::any::type_name_of_val(&wide), std::any::type_name_of_val(&tall));

    println!();
    println!("4. {{:#?}} prints one number per line, which is why a test's");
    println!("   failure message for a matrix is long:");
    println!("{:#?}", transpose([[101, 102], [201, 202]]));
}
```
<!-- /source -->

<!-- output:transpose_kata -->
*Verified output of [`transpose_kata.rs`](examples/transpose_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Two index loops
   [[1, 4, 7], [2, 5, 8], [3, 6, 9]]

2. from_fn inside from_fn, for any N
   [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
   2x2: [[101, 201], [102, 202]]
   twice is the identity: true

3. Not square: [[T; 3]; 2] in, [[T; 2]; 3] out
   [['a', 'b', 'c'], ['d', 'e', 'f']]
   [['a', 'd'], ['b', 'e'], ['c', 'f']]
   [[char; 3]; 2] -> [[char; 2]; 3]

4. {:#?} prints one number per line, which is why a test's
   failure message for a matrix is long:
[
    [
        101,
        201,
    ],
    [
        102,
        202,
    ],
]
```
<!-- /output -->

On clippy 0.1.98 the two loops are silent. `needless_range_loop` fires when a loop variable only indexes one array, and here `i` and `j` each index two: `transposed[j][i]` and `matrix[i][j]`.

</details>

## See also

- [Arrays: the map](../README.md) — every array page, in reading order
- [Arrays and slices](../../arrays_and_slices/README.md) — why `&[T]` is the default parameter type
- [A `Vec` of arrays](../../vec_of_arrays/README.md) — fixed-width rows in a growable list, and what a `Vec<[T; N]>` returns
- [`&'static str`](../../../14_Strings/static_str/README.md) — promotion's everyday case, the string literal
- [Const promotion, in `const` and `static`](../../../27_Modules/const_and_static/README.md) — what `&CONST` gives you, and which address you may rely on
- [Every array error, and its fix](../array_errors/README.md) — errors 2, 26 and 27 are this page's refusals

## Sources

[The Reference — constant promotion ↗](https://doc.rust-lang.org/reference/destructors.html#constant-promotion); [The Book, ch. 4.3 — *Other Slices* ↗](https://doc.rust-lang.org/book/ch04-03-slices.html#other-slices); [`array::map` ↗](https://doc.rust-lang.org/std/primitive.array.html#method.map) and [`array::from_fn` ↗](https://doc.rust-lang.org/std/array/fn.from_fn.html).

## Po polsku

Tablica przechodzi przez granicę funkcji jak każda inna wartość. `fn create_array() -> [i32; 3]` zwraca same elementy, bez wskaźnika i sterty. Parametr typu `[i32; 3]` przyjmuje dokładnie trzy elementy, a cztery to `E0308` („expected an array with a size of 3, found one with a size of 4”). Parametr `&[i32]` przyjmuje dowolną długość. **Parametr** to nazwa w sygnaturze, **argument** to wartość podana przy wywołaniu.

Zwrócenie referencji wymaga czegoś, co przeżyje wywołanie. `&[1, 2, 3]` jest stałą, więc kompilator ją **promuje** do pamięci statycznej i typ `&'static [i32]` jest prawdziwy. `&[x, 1, 2]` z wartością `x` znaną dopiero w czasie działania to tymczasowa wartość w ramce funkcji, stąd `E0515`. `&mut arr[1..3]` zwraca fragment tablicy wywołującego: indeks 0 wycinka to indeks 1 tablicy. Metoda `get_array(&self) -> [i32; 3]` zwraca kopię, bo `[i32; 3]` jest `Copy`, natomiast dla `[String; 3]` to błąd `E0507`.

**Szukaj po polsku:** tablica jako parametr funkcji · zwracanie tablicy z funkcji · promocja stałych · `rust return array from function` · `rust const generics array length`
