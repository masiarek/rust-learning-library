# Writing an array down: `[1, 2, 3]`, `[u8; 3]` and `[0; 3]`

[Arrays: the map](../README.md) › **Lesson 1** · next: [Arrays in and out of functions](../arrays_in_signatures/README.md)

**Level:** 101 · for newcomers

**One line:** Every array is written one of three ways — its elements listed, a type that fixes element type and length, or a repeat expression `[value; how many]` — and when you leave the element type out, inference takes it from wherever the array is used next, including from the array written beside it.

```rust
fn main() {
    let listed = [1, 2, 3];              // [i32; 3]: i32 when nothing says otherwise
    let annotated: [u8; 3] = [1, 2, 3];  // [u8; 3]
    let repeated = [0u8; 3];             // [0, 0, 0]
    println!("{listed:?} {annotated:?} {repeated:?}"); // [1, 2, 3] [1, 2, 3] [0, 0, 0]
}
```

This is how you *write (initialize) array values*. The page then covers what inference does with them, and the first loop most people write over one. Each claim is printed by [the program at the bottom](#the-verified-output).

## Three spellings

| You write | You get | Its type |
|---|---|---|
| `[1, 2, 3]` | the elements, in order | `[i32; 3]` |
| `let a: [u8; 3] = [1, 2, 3];` | the same elements, typed by the annotation | `[u8; 3]` |
| `[0u8; 3]` | one value, three times | `[u8; 3]` |
| `let a: [i32; _] = [4, 5, 6, 7];` | the compiler counts the length for you | `[i32; 4]` |

Any element type works, one per array: `[f64; 3]`, `[&str; 4]`, `['a', 'b']`, `[true, false, true]`, `['c'; 3]`, `["d"; 3]`. [Tuples](../../tuples/README.md) are the other compound type the language has built in. The Book calls tuples and arrays Rust's "two primitive compound types", and `Vec` is not one of them: it is a library type.

## `[value; how many]`

The repeat expression has two halves:

| `[` | `0` | `;` | `3` | `]` |
|---|---|---|---|---|
| | **value**: any expression | | **how many**: a constant `usize` | |

*How many* has to be known at compile time. A `let n` is refused with [`E0435`](../array_errors/README.md#4-a-run-time-length-in-a-repeat-expression), and a length that comes from input needs `vec![0; n]`.

*Value* is evaluated **once** and copied, so for two or more elements it must be `Copy` or a constant. `["bb".to_string(); 3]` is [`E0277`](../array_errors/README.md#9-a-string-in-a-repeat-expression), and there are three fixes:

| Write | When it fits |
|---|---|
| `[const { String::new() }; 3]` | every element is the same value, and that value can be built at compile time |
| [`std::array::repeat(String::from("bb"))` ↗](https://doc.rust-lang.org/std/array/fn.repeat.html) | you have one run-time value; it is cloned N − 1 times. Stable since 1.91.0 |
| [`std::array::from_fn(\|i\| format!("row {i}"))` ↗](https://doc.rust-lang.org/std/array/fn.from_fn.html) | each element is built separately, usually from its index. Stable since 1.63.0, and it is what rustc's `help:` suggests |

Two edge cases. With a length of 1, nothing is copied, so `[String::from("only"); 1]` compiles. With a length of 0, the value is still evaluated and then dropped straight away; clippy's [`zero_repeat_side_effects`](../array_lints/README.md#zero_repeat_side_effects) catches the case where that matters.

## One suffix types the whole array

`[0u8, 0, 0]` is `[u8; 3]`, and so is `[1u8, 2, 3]`. `[0, 0i32, 0]` is `[i32; 3]`. Every element of an array has one type, so a suffix on any one of them decides it for all.

## Inference reads the neighbours

On its own, `let one = [1, 2, 3];` is `[i32; 3]`. Put it in an array beside a `[u8; 3]`, and it becomes `[u8; 3]`:

```rust
use std::any::type_name_of_val;

fn main() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    let arrays = [one, two, blank1, blank2];
    println!("{}", type_name_of_val(&one));    // [u8; 3]
    println!("{}", type_name_of_val(&arrays)); // [[u8; 3]; 4]
    println!("{}", size_of_val(&arrays));      // 12
}
```

The elements of an array **must be the same type**, and the compiler solves for one type that makes the line true. Nothing had fixed `one` or `blank1` yet, so `u8` fits. `size_of_val(&arrays)` is 12: twelve `u8`s, with no length or header stored anywhere. That is what *Rust in Action* means when it calls arrays fixed-width and lightweight.

The type is only open until something fixes it. Pass `one` to a function taking `[i32; 3]` *first* and it is `[i32; 3]` from then on, so the next line is [`E0308`: expected `[i32; 3]`, found `[u8; 3]`](../array_errors/README.md#8-an-array-already-pinned-to-another-element-type). Two arrays whose annotations disagree are refused the same way: [`[[a1], [a2]]` with a `[u8; 5]` and a `[u16; 5]`](../array_errors/README.md#7-two-arrays-whose-element-types-differ). Write the type once, on either array or as a suffix, and the rest follow.

**Brackets add a level.** `[[a1], [a2]]` is `[[[u8; 5]; 1]; 2]`, two one-element arrays of arrays, while `[a1, a2]` is `[[u8; 5]; 2]`.

## Declare now, assign later

```rust
fn main() {
    let later: [i64; 5];
    later = [100, 101, 102, 103, 104];
    println!("{later:?}"); // [100, 101, 102, 103, 104]
}
```

`later` is not `mut`, since it is assigned exactly once. Reading it before that is [`E0381`](../../../17_Option_and_Result/initial_values/README.md). Clippy's [`needless_late_init`](../array_lints/README.md#needless_late_init) asks you to join the two lines when nothing runs between them.

## Iterating over it by reference

```rust
fn main() {
    let arrays = [[1u8, 2, 3], [0; 3]];
    for a in &arrays {
        // a: &[u8; 3], n: &u8
        for n in a {
            print!("{} ", n + 10);
        }
    }
    println!(); // 11 12 13 10 10 10
}
```

`for a in &arrays` borrows, so each `a` is a `&[u8; 3]`, and `arrays` is still usable afterwards. `n + 10` compiles even though `n` is a `&u8`, because std implements `Add<u8>` for `&u8` as well as for `u8`. [`iter`, `iter_mut` and `into_iter`](../../../24_Iterators/iter_iter_mut_into_iter/README.md) covers the three ways to iterate, and why `for n in array` hands out values since edition 2021.

## The sum nobody typed is a `u8`

The first loop most people write:

```rust
fn index_loop_sum(a: &[u8; 3]) -> u8 {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i];
    }
    sum
}
```

`sum` has no annotation, so `sum += a[i]` makes it a `u8`. `[200, 100, 0]` then overflows. A debug build panics with *attempt to add with overflow*. A release build does not check by default and wraps to 44. Widen before adding: `a.iter().map(|&n| u32::from(n)).sum::<u32>()` is 300.

Clippy flags the loop but not the overflow: [`needless_range_loop`](../array_lints/README.md#needless_range_loop) says *"the loop variable `i` is only used to index `a`"* and suggests `for <item> in &a`. Only the restriction lint `arithmetic_side_effects` notices the `+=`.

## `array::from_fn` is not `iter::from_fn`

Two std functions share the name:

| | [`std::array::from_fn` ↗](https://doc.rust-lang.org/std/array/fn.from_fn.html) | [`std::iter::from_fn` ↗](https://doc.rust-lang.org/std/iter/fn.from_fn.html) |
|---|---|---|
| signature | `fn from_fn<T, const N: usize, F>(f: F) -> [T; N] where F: FnMut(usize) -> T` | `fn from_fn<T, F>(f: F) -> FromFn<F> where F: FnMut() -> Option<T>` |
| the closure gets | the index, `0..N` | nothing |
| the closure returns | the element | `Some(item)`, or `None` to stop |
| you get | an array, length from the type | an iterator, length from the closure |
| since | 1.63.0 | 1.34.0 |

`std::array::from_fn(|i| i * i)` is `[0, 1, 4, 9, 16]` when the type says 5. The iterator kind is the short way to write an `Iterator` without a struct, which [Implementing `Iterator`](../../../24_Iterators/implementing_iterator/README.md) does the long way.

## If you are coming from another language

- **Python.** `[0] * 3` is the repeat expression, and `[1, 2, 3]` looks the same, but a Python list is a growable `Vec`. The repeat trap is different in each language. In Python, `[[0] * 3] * 3` is three references to **one** inner list, so `g[0][0] = 9` changes every row. In Rust, `[[0; 3]; 3]` copies the inner array three times, because `[i32; 3]` is `Copy`, and a non-`Copy` element is refused rather than aliased. Python's `bytes(5)` is five zero bytes, the nearest thing to `[0u8; 5]` — [Making a `bytes` object ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/making_a_bytes_object/index.html) in the Python library.
- **C.** `int a[3] = {1, 2, 3};` is the listed form. `int b[5] = {0};` zero-fills the rest the way `[0; 5]` fills everything. `int c[] = {4, 5, 6, 7};` counts the length the way `[i32; _]` does. What changes: C lets you leave elements out and fills them with zero, while Rust wants all of them or a repeat. C also has no inference from neighbours, since every declaration names its type.
- **Go.** `[3]int{1, 2, 3}` and `[...]int{4, 5, 6, 7}` are Rust's two forms, with the element type written first, and `var z [5]int` is `[0; 5]`. Go arrays are values like Rust's, so `c := a` copies. Go has no repeat syntax for a non-zero value: you fill it with a loop or `from_fn`-style code.
- **Java.** `new int[3]` is `[0; 3]` and `{1, 2, 3}` is the list, but a Java array is a reference to a heap object, so `int[] c = b; c[0] = 99;` changes `b` too. A Rust array assigned to a new name is copied (`Copy` elements) or moved. The Java library's [`split` has sharp edges ↗](https://masiarek.github.io/java-text-learning-library/04_Regex/split_has_sharp_edges/index.html) meets `String[]` from the other side.

---

## The verified output

<!-- output:writing_an_array_down -->
*Verified output of [`writing_an_array_down.rs`](examples/writing_an_array_down.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three spellings
   [1, 2, 3]              [1, 2, 3]  is [i32; 3]
   let _: [u8; 3] = ...   [1, 2, 3]  is [u8; 3]
   [0u8; 3]               [0, 0, 0]  is [u8; 3]
   let _: [i32; _] = [4, 5, 6, 7] is [i32; 4] -- the compiler counts

2. Any element type, one per array
   [0.5, 1.5, 2.5] is [f64; 3]
   ["a", "b", "c", "d"] is [&str; 4]
   ['a', 'b'] is [char; 2]
   [true, false, true] is [bool; 3]
   ['c', 'c', 'c'] is [char; 3]
   ["d", "d", "d"] is [&str; 3]

3. [value; how many]
   [99; 5]     = [99, 99, 99, 99, 99]
   ['a'; 10]   = ['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a']
   [true; 1000] has len 1000 and size_of 1000 bytes
   [const { String::new() }; 3]          = ["", "", ""]
   std::array::repeat(String::from("bb")) = ["bb", "bb", "bb"]
   std::array::from_fn(|i| format!(..))   = ["row 0", "row 1", "row 2"]
   [String::from("only"); 1] compiles too: ["only"]
   (a non-Copy value is fine when there is one copy to make: none)

4. One suffix types the whole array
   [0u8, 0, 0] is [u8; 3]
   [0, 0i32, 0] is [i32; 3]
   [1u8, 2, 3] is [u8; 3]

5. Inference reads the neighbours
   let alone = [1, 2, 3];  alone is [i32; 3]
   the same literal beside a [u8; 3]:
   one is [u8; 3], blank1 is [u8; 3]
   arrays is [[u8; 3]; 4]
   size_of_val(&arrays) = 12 bytes: twelve u8s, no headers

6. Brackets add a level
   [[a1], [a2]] is [[[u8; 5]; 1]; 2]
   [a1, a2]     is [[u8; 5]; 2]

7. Declare now, assign once, later
   let later: [i64; 5]; later = [...];  later = [100, 101, 102, 103, 104]

8. Walking it by reference
   a is &[u8; 3]   [1, 2, 3] -> n + 10 = [11, 12, 13]
   a is &[u8; 3]   [1, 2, 3] -> n + 10 = [11, 12, 13]
   a is &[u8; 3]   [0, 0, 0] -> n + 10 = [10, 10, 10]
   a is &[u8; 3]   [0, 0, 0] -> n + 10 = [10, 10, 10]
   n is &u8, and 1 + 10 = 11: &u8 implements Add<u8>

9. The sum nobody typed is a u8
   index_loop_sum(&[1, 2, 3]) = 6
   index_loop_sum(&[200, 100, 0]) panicked: attempt to add with overflow
   a release build does not check, and wraps: 200u8.wrapping_add(100) = 44
   widen first: row.iter().map(|&n| u32::from(n)).sum::<u32>() = 300

10. Two functions called from_fn
   std::array::from_fn(|i| i * i) -> [0, 1, 4, 9, 16]  (index in, element out, N from the type)
   std::iter::from_fn(|| ..)      -> [3, 2, 1]  (nothing in, Option out, stops at None)
```
<!-- /output -->

## Practice

**The first ten odd numbers, two ways.** Build a `[i32; 10]` holding 1, 3, 5, … 19. First write it as a block: inside `{ }`, fill a zeroed `[0; 10]` from `(0..10).map(|x| 2 * x + 1).enumerate()` and make the array the block's value. Then write it again with `std::array::from_fn`.

Then say what `2 * i as i32 + 1` parses as, and why the closure's `i` is a `usize`. Finally, change both versions to twelve numbers and count the edits each one needs. One of the edits you might miss is a compile error; the other compiles and quietly leaves zeros at the end.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:writing_an_array_down_kata -->
*[`writing_an_array_down_kata.rs`](examples/writing_an_array_down_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the first ten odd numbers, as a block and as `from_fn`.
//!
//!   rustc --edition 2024 writing_an_array_down_kata.rs -o /tmp/wadk && /tmp/wadk

use std::any::type_name_of_val;

fn main() {
    println!("1. A block that fills a zeroed array, then hands it out");
    let odd_numbers: [i32; 10] = {
        let mut arr = [0; 10];
        for (i, num) in (0..10).map(|x| 2 * x + 1).enumerate() {
            arr[i] = num;
        }
        arr // the block's value: no `;`, so the array leaves the block
    };
    println!("   {odd_numbers:?}");
    println!("   `arr` is mutable only inside the block; odd_numbers is not mut at all");

    println!();
    println!("2. from_fn: the closure is asked for element i, N comes from the type");
    let from_fn: [i32; 10] = std::array::from_fn(|i| 2 * i as i32 + 1);
    println!("   {from_fn:?}");
    println!("   same array? {}", odd_numbers == from_fn);

    println!();
    println!("3. `2 * i as i32 + 1` reads as `(2 * (i as i32)) + 1`");
    let i: usize = 4;
    println!("   i = {i}: 2 * i as i32 + 1 = {}", 2 * i as i32 + 1);
    println!("   `as` binds tighter than `*`, so only i is cast; the 2 and the 1");
    println!("   are then inferred as i32 from it. The closure's i is a usize");
    println!("   because from_fn passes an index: {}", type_name_of_val(&i));

    println!();
    println!("4. Twelve instead of ten: count the edits");
    let twelve: [i32; 12] = std::array::from_fn(|i| 2 * i as i32 + 1);
    println!("   from_fn, one edit (the type):          {twelve:?}");
    let twelve_block: [i32; 12] = {
        let mut arr = [0; 12];
        for (i, num) in (0..12).map(|x| 2 * x + 1).enumerate() {
            arr[i] = num;
        }
        arr
    };
    println!("   block, three edits ([i32; _], [0; _], 0.._): {twelve_block:?}");
    println!("   Miss the [0; 12] and it is E0308 at `arr`: \"expected an array with a");
    println!("   size of 12, found one with a size of 10\".");
    println!("   Miss the 0..12 instead and it compiles, leaving [.., 0, 0] at the end:");
    let forgot_range: [i32; 12] = {
        let mut arr = [0; 12];
        for (i, num) in (0..10).map(|x| 2 * x + 1).enumerate() {
            arr[i] = num;
        }
        arr
    };
    println!("   {forgot_range:?}");
}
```
<!-- /source -->

<!-- output:writing_an_array_down_kata -->
*Verified output of [`writing_an_array_down_kata.rs`](examples/writing_an_array_down_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A block that fills a zeroed array, then hands it out
   [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
   `arr` is mutable only inside the block; odd_numbers is not mut at all

2. from_fn: the closure is asked for element i, N comes from the type
   [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
   same array? true

3. `2 * i as i32 + 1` reads as `(2 * (i as i32)) + 1`
   i = 4: 2 * i as i32 + 1 = 9
   `as` binds tighter than `*`, so only i is cast; the 2 and the 1
   are then inferred as i32 from it. The closure's i is a usize
   because from_fn passes an index: usize

4. Twelve instead of ten: count the edits
   from_fn, one edit (the type):          [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23]
   block, three edits ([i32; _], [0; _], 0.._): [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23]
   Miss the [0; 12] and it is E0308 at `arr`: "expected an array with a
   size of 12, found one with a size of 10".
   Miss the 0..12 instead and it compiles, leaving [.., 0, 0] at the end:
   [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 0, 0]
```
<!-- /output -->

</details>

## See also

- [Arrays: the map](../README.md) — every array page in the library, in reading order
- [Arrays and slices](../../arrays_and_slices/README.md) — the length in the type, and why `&[T]` goes in a signature
- [Every array error, and its fix](../array_errors/README.md) — the refusals this page links to, with rustc's full text
- [Lints around arrays](../array_lints/README.md) — `needless_range_loop`, `needless_late_init` and the rest
- [Type inference](../../../15_First_Programs/type_inference/README.md) — the solver this page watches at work
- [Values](../../../15_First_Programs/values/README.md) — the widths behind `u8`, `i32` and `f64`
- [Meet the byte](../../../19_Numbers/meet_the_byte/README.md) — what a `u8` holds, and what `255 + 1` does in debug and release

## Sources

[The Book, ch. 3.2 — *The Array Type* ↗](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type); [The Reference — array expressions ↗](https://doc.rust-lang.org/reference/expressions/array-expr.html#array-expressions); [`array` in std ↗](https://doc.rust-lang.org/std/primitive.array.html). *Rust in Action* (McNamara, Manning 2021), ch. 2 "Language foundations", §2.10.1 "Arrays", listing 2.21 — [`ch2-3arrays.rs` ↗](https://github.com/rust-in-action/code/blob/1st-edition/ch2/ch2-3arrays.rs) is the listing this page's inference and overflow sections start from. Its repository has no licence, so it is linked, not copied.

## Po polsku

Tablicę zapisuje się na trzy sposoby: wypisując elementy (`[1, 2, 3]`), podając typ, który ustala typ elementu i długość (`let a: [u8; 3] = …`), albo **wyrażeniem powtórzenia** `[wartość; ile razy]`, np. `[0; 3]`. W typie można też napisać `[i32; _]` — wtedy długość policzy kompilator. Wartość w `[x; N]` liczy się raz i jest kopiowana, więc dla N ≥ 2 musi być `Copy` albo stałą. `["bb".to_string(); 3]` to błąd `E0277`, a poprawki są trzy: `[const { String::new() }; 3]`, `std::array::repeat(…)` (od 1.91.0) i `std::array::from_fn(|i| …)`.

Wszystkie elementy tablicy muszą mieć **ten sam typ**. Wystarczy jeden sufiks (`[0u8, 0, 0]`), żeby ustalić typ całej tablicy. Kompilator dobiera typ także z sąsiedztwa: samo `let one = [1, 2, 3];` to `[i32; 3]`, ale obok tablicy `[u8; 3]` w `[one, two, …]` staje się `[u8; 3]`. Jeśli wcześniej coś przypnie `one` do `i32`, dostaniesz `E0308`. Pułapka na koniec: w pętli `for i in 0..a.len() { sum += a[i]; }` zmienna `sum` jest typu `u8`, więc `[200, 100, 0]` w trybie debug kończy się paniką przepełnienia, a w wydaniu (release) daje po cichu 44.

**Szukaj po polsku:** inicjalizacja tablicy · wyrażenie powtórzenia · tablice muszą mieć ten sam typ · `rust array repeat expression` · `rust array from_fn`
