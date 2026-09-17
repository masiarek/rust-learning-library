# Every array error, and its fix

[Arrays: the map](../README.md) › **Beside the lessons** · the warnings, not the errors: [Lints around arrays](../array_lints/README.md)

**Level:** 101 → 201 · a reference, by symptom

**One line:** Twenty-nine refusals you meet when writing, indexing, iterating, returning and storing arrays. For each: the code that causes it, what rustc 1.98.0 prints, the mistake, and a fix. The broken code must fail and the fix must compile, and [`check_fences.py`](../../../tools/check_fences.py) holds both on every build.

Every transcript was recorded on rustc 1.98.0 by compiling the file named in its title with `rustc --edition 2024`. The trailing `aborting due to` and `rustc --explain` lines are dropped. Transcripts are not regenerated on each commit the way an example's output is, so a later compiler can reword one. The fences are checked, so a broken example cannot quietly start compiling.

One exception is marked `rust,ignore`: [a constant index past the end](#10-a-constant-index-past-the-end). Its refusal comes from a lint that runs after the metadata-only build `check_fences.py` does, so that gate cannot see it. The transcript is still a real 1.98.0 run.

## Find the error

| # | Code | What rustc says | The mistake |
|---|---|---|---|
| 1 | `E0308` | [mismatched types: expected an array with a size of 3, found one with a size of 4](#1-more-elements-than-the-type-says) | More elements than the type says |
| 2 | `E0308` | [mismatched types: expected an array with a size of 3, found one with a size of 4](#2-a-four-element-array-where-a-function-takes-three) | A four-element array where a function takes three |
| 3 | `E0277` | [can't compare `[i8; 3]` with `[i8; 4]`](#3-comparing-arrays-of-two-lengths) | Comparing arrays of two lengths |
| 4 | `E0435` | [attempt to use a non-constant value in a constant](#4-a-run-time-length-in-a-repeat-expression) | A run-time length in a repeat expression |
| 5 | `E0308` | [mismatched types: expected `[u8; 2]`, found `[{integer}]`](#5-a-slice-where-an-array-is-wanted) | A slice where an array is wanted |
| 6 | `E0308` | [mismatched types: expected integer, found `&str`](#6-a-number-and-a-string-in-one-array) | A number and a string in one array |
| 7 | `E0308` | [mismatched types: expected `[u8; 5]`, found `[u16; 5]`](#7-two-arrays-whose-element-types-differ) | Two arrays whose element types differ |
| 8 | `E0308` | [mismatched types: expected `[i32; 3]`, found `[u8; 3]`](#8-an-array-already-pinned-to-another-element-type) | An array already pinned to another element type |
| 9 | `E0277` | [the trait bound `String: Copy` is not satisfied](#9-a-string-in-a-repeat-expression) | A `String` in a repeat expression |
| 10 | lint | [this operation will panic at runtime](#10-a-constant-index-past-the-end) | A constant index past the end |
| 11 | `E0277` | [the type `[{integer}]` cannot be indexed by `i32`](#11-an-i32-as-an-index) | An `i32` as an index |
| 12 | `E0594` | [cannot assign to `xs[_]`, as `xs` is not declared as mutable](#12-writing-into-an-array-that-is-not-mut) | Writing into an array that is not `mut` |
| 13 | `E0508` | [cannot move out of type `[String; 2]`, a non-copy array](#13-moving-one-string-out-of-an-array) | Moving one `String` out of an array |
| 14 | `E0599` | [no method named `push` found for array `[{integer}; 3]` in the current scope](#14-push-on-an-array) | `push` on an array |
| 15 | `E0599` | [the method `enumerate` exists for array `[[{integer}; 2]; 2]`, but its trait bounds were not satisfied](#15-enumerate-on-an-array) | `enumerate` on an array |
| 16 | `E0277` | [`[{integer}; 3]` doesn't implement `std::fmt::Display`](#16-printing-an-array-with) | Printing an array with `{}` |
| 17 | `E0369` | [cannot add `[{integer}; 3]` to `[{integer}; 3]`](#17-adding-two-arrays-with) | Adding two arrays with `+` |
| 18 | `E0277` | [an array of type `[i32; 3]` cannot be built directly from an iterator](#18-collect-into-an-array) | `collect()` into an array |
| 19 | `E0277` | [the trait bound `[u8; 33]: Default` is not satisfied](#19-default-for-33-elements) | `Default` for 33 elements |
| 20 | `E0015` | [cannot call non-const associated function `<String as From<&str>>::from` in statics](#20-a-string-built-in-a-static) | A `String` built in a static |
| 21 | `E0382` | [borrow of moved value: `names`](#21-using-an-array-of-strings-after-moving-it) | Using an array of `String`s after moving it |
| 22 | `E0614` | [type `{integer}` cannot be dereferenced](#22-x-in-a-filter-over-into_iter) | `*x` in a filter over `into_iter()` |
| 23 | `E0308` | [mismatched types: expected integer, found `&_`](#23-x-from-an-answer-written-before-2021) | `\|&&x\|` from an answer written before 2021 |
| 24 | `E0382` | [use of moved value: `range_iter`](#24-using-an-iterator-after-filter-took-it) | Using an iterator after `filter` took it |
| 25 | `E0308`, `E0277` | [mismatched types: expected `i32`, found `u8`](#25-adding-u8-elements-to-an-i32-sum) | Adding `u8` elements to an `i32` sum |
| 26 | `E0507` | [cannot move out of `self.names` which is behind a shared reference](#26-a-getter-that-returns-an-array-of-strings) | A getter that returns an array of `String`s |
| 27 | `E0515` | [cannot return reference to temporary value](#27-returning-x-1-2-as-static-i32) | Returning `&[x, 1, 2]` as `&'static [i32]` |
| 28 | `E0106` | [missing lifetime specifier](#28-a-str-field-with-no-lifetime) | A `&str` field with no lifetime |
| 29 | `E0597` | [`name` does not live long enough](#29-a-borrowed-local-where-static-is-required) | A borrowed local where `'static` is required |

## The length is part of the type

### 1. More elements than the type says

```rust,compile_fail
fn main() {
    let rgb: [u8; 3] = [255, 128, 0, 255];
    println!("{rgb:?}");
}
```

```text title="rustc 1.98.0 on length_mismatch.rs"
error[E0308]: mismatched types
 --> length_mismatch.rs:2:24
  |
2 |     let rgb: [u8; 3] = [255, 128, 0, 255];
  |              -------   ^^^^^^^^^^^^^^^^^^ expected an array with a size of 3, found one with a size of 4
  |              |
  |              expected due to this
  |
help: consider specifying the actual array length
  |
2 -     let rgb: [u8; 3] = [255, 128, 0, 255];
2 +     let rgb: [u8; 4] = [255, 128, 0, 255];
  |
```

**The mistake.** `[u8; 3]` is a type, and the literal is a `[u8; 4]`. The length is checked like any other part of a type, so one element too many is a type mismatch, not a truncation.

**The fix.** Make the annotation match, or drop it and let the literal decide. `[u8; _]` also counts for you.

```rust
fn main() {
    let rgba: [u8; 4] = [255, 128, 0, 255];
    println!("{rgba:?}");
}
```

**Read:** [Writing an array down](../writing_an_array_down/README.md), [Arrays and slices](../../arrays_and_slices/README.md)

### 2. A four-element array where a function takes three

```rust,compile_fail
fn print_array(arr: [i32; 3]) {
    println!("{arr:?}");
}

fn main() {
    print_array([1, 2, 3, 4]);
}
```

```text title="rustc 1.98.0 on wrong_length_argument.rs"
error[E0308]: mismatched types
 --> wrong_length_argument.rs:6:17
  |
6 |     print_array([1, 2, 3, 4]);
  |     ----------- ^^^^^^^^^^^^ expected an array with a size of 3, found one with a size of 4
  |     |
  |     arguments to this function are incorrect
  |
note: function defined here
 --> wrong_length_argument.rs:1:4
  |
1 | fn print_array(arr: [i32; 3]) {
  |    ^^^^^^^^^^^ -------------
```

**The mistake.** A parameter of type `[i32; 3]` accepts exactly three. Older compilers worded it *"expected an array with a fixed size of 3 elements, found one with 4 elements"*; 1.98.0 says *"a size of 3"*.

**The fix.** Take `&[i32]` if the function works for any length, and pass `&[1, 2, 3, 4]`. Keep `[i32; 3]` only when three is a fact about the problem.

```rust
fn print_array(arr: &[i32]) {
    println!("{arr:?}");
}

fn main() {
    print_array(&[1, 2, 3]);
    print_array(&[1, 2, 3, 4]);
}
```

**Read:** [Arrays in and out of functions](../arrays_in_signatures/README.md#taking-an-array-exactly-n-or-any-length), [Array or `Vec`?](../../array_or_vec/README.md)

### 3. Comparing arrays of two lengths

```rust,compile_fail
const MY_DATA_3: [i8; 3] = [1, 2, 3];
const MY_DATA_4: [i8; 4] = [1, 2, 3, 4];

fn main() {
    assert_eq!(MY_DATA_3, MY_DATA_4);
}
```

```text title="rustc 1.98.0 on compare_lengths.rs"
error[E0277]: can't compare `[i8; 3]` with `[i8; 4]`
 --> compare_lengths.rs:5:5
  |
5 |     assert_eq!(MY_DATA_3, MY_DATA_4);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `[i8; 3] == [i8; 4]`
  |
  = help: the trait `PartialEq<[i8; 4]>` is not implemented for `[i8; 3]`
  = help: the following other types implement trait `PartialEq<Rhs>`:
            `&[T]` implements `PartialEq<Vec<U, A>>`
            `&[T]` implements `PartialEq<[U; N]>`
            `&[u8; N]` implements `PartialEq<ByteStr>`
            `&[u8; N]` implements `PartialEq<ByteString>`
            `&[u8]` implements `PartialEq<ByteStr>`
            `&[u8]` implements `PartialEq<ByteString>`
            `&mut [T]` implements `PartialEq<Vec<U, A>>`
            `&mut [T]` implements `PartialEq<[U; N]>`
          and 11 others
```

**The mistake.** `[i8; 3] == [i8; 4]` has no impl: `PartialEq` between arrays is only defined for equal `N`. The `help:` list is the impls that do exist, and `&[T]` against `[U; N]` is among them.

**The fix.** Compare them as slices, which is `false` here, or compare equal-length parts. `MY_DATA_3 == MY_DATA_4[..3]` is `true`.

```rust
const MY_DATA_3: [i8; 3] = [1, 2, 3];
const MY_DATA_4: [i8; 4] = [1, 2, 3, 4];

fn main() {
    println!("{}", MY_DATA_3[..] == MY_DATA_4[..]); // false
    assert_eq!(MY_DATA_3, MY_DATA_4[..3]);
}
```

**Read:** [An array in a `const` or a `static`](../static_arrays/README.md#a-const-array-and-a-local-that-changes-its-contents)

### 4. A run-time length in a repeat expression

```rust,compile_fail
fn main() {
    let scores = vec![90, 72, 85];
    let n = scores.len();
    let counts = [0u32; n];
    println!("{counts:?}");
}
```

```text title="rustc 1.98.0 on runtime_length.rs"
error[E0435]: attempt to use a non-constant value in a constant
 --> runtime_length.rs:4:25
  |
4 |     let counts = [0u32; n];
  |                         ^ non-constant value
  |
help: consider using `const` instead of `let`
  |
3 -     let n = scores.len();
3 +     const n: /* Type */ = scores.len();
  |
```

**The mistake.** `n` is a `let`, known only when the program runs, and an array's length must be a constant. The `help:` suggests a `const`, which cannot work here because `scores.len()` is not a constant.

**The fix.** A length that comes from data belongs to a `Vec`: `vec![0u32; n]`.

```rust
fn main() {
    let scores = vec![90, 72, 85];
    let n = scores.len();
    let counts = vec![0u32; n];
    println!("{counts:?}");
}
```

**Read:** [Array or `Vec`?](../../array_or_vec/README.md#where-the-vec-is-simply-right), [Writing an array down](../writing_an_array_down/README.md#value-how-many)

### 5. A slice where an array is wanted

```rust,compile_fail
fn main() {
    let bytes = [0xCA, 0xFE, 0xBA, 0xBE];
    let head: [u8; 2] = bytes[..2];
    println!("{head:?}");
}
```

```text title="rustc 1.98.0 on slice_to_array.rs"
error[E0308]: mismatched types
 --> slice_to_array.rs:3:25
  |
3 |     let head: [u8; 2] = bytes[..2];
  |               -------   ^^^^^^^^^^ expected `[u8; 2]`, found `[{integer}]`
  |               |
  |               expected due to this
```

**The mistake.** `bytes[..2]` is a `[u8]` slice, whose length lives in the value, not the type. Slices never coerce to arrays.

**The fix.** `try_into()` checks the length at run time and gives a `[u8; 2]`. A slice pattern `let [first, second, ..] = bytes;` takes elements out of the array without a check.

```rust
fn main() {
    let bytes = [0xCA, 0xFE, 0xBA, 0xBE];
    let head: [u8; 2] = bytes[..2].try_into().unwrap();
    let [first, second, ..] = bytes;
    println!("{head:?} {first} {second}");
}
```

**Read:** [Arrays and slices](../../arrays_and_slices/README.md)

## One element type per array

### 6. A number and a string in one array

```rust,compile_fail
fn main() {
    let row = [1, "two", 3];
    println!("{row:?}");
}
```

```text title="rustc 1.98.0 on mixed_element_types.rs"
error[E0308]: mismatched types
 --> mixed_element_types.rs:2:19
  |
2 |     let row = [1, "two", 3];
  |                   ^^^^^ expected integer, found `&str`
```

**The mistake.** An array holds one type. The first element made it an integer array, so `"two"` is the mismatch.

**The fix.** Make them one type, or use a [tuple](../../tuples/README.md) when the positions really hold different types.

```rust
fn main() {
    let row = ["1", "two", "3"];
    let pair = (1, "two");
    println!("{row:?} {pair:?}");
}
```

**Read:** [Writing an array down](../writing_an_array_down/README.md)

### 7. Two arrays whose element types differ

```rust,compile_fail
fn main() {
    let a1: [u8; 5] = [0; 5];
    let a2: [u16; 5] = [1; 5];
    let b = [[a1], [a2]];
    println!("{b:?}");
}
```

```text title="rustc 1.98.0 on nested_u8_u16.rs"
error[E0308]: mismatched types
 --> nested_u8_u16.rs:4:21
  |
4 |     let b = [[a1], [a2]];
  |                     ^^ expected `[u8; 5]`, found `[u16; 5]`
  |
  = note: expected array `[u8; 5]`
             found array `[u16; 5]`
```

**The mistake.** `[a1]` is a `[[u8; 5]; 1]` and `[a2]` is a `[[u16; 5]; 1]`, and the outer array needs its elements to be one type. The extra brackets add a level: the fixed `b` is a `[[[u8; 5]; 1]; 2]`.

**The fix.** Give both the same element type. If the values really are different widths, convert one: `a2.map(u8::try_from)` or `a1.map(u16::from)`.

```rust
fn main() {
    let a1: [u8; 5] = [0; 5];
    let a2: [u8; 5] = [1; 5];
    let b = [[a1], [a2]];
    println!("{b:?}"); // [[[0, 0, 0, 0, 0]], [[1, 1, 1, 1, 1]]]
}
```

**Read:** [Writing an array down](../writing_an_array_down/README.md#inference-reads-the-neighbours)

### 8. An array already pinned to another element type

```rust,compile_fail
fn takes_i32_array(a: [i32; 3]) -> i32 {
    a.iter().sum()
}

fn main() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [4, 5, 6];
    println!("{}", takes_i32_array(one));
    let arrays = [one, two];
    println!("{arrays:?}");
}
```

```text title="rustc 1.98.0 on pinned_to_i32.rs"
error[E0308]: mismatched types
 --> pinned_to_i32.rs:9:24
  |
9 |     let arrays = [one, two];
  |                        ^^^ expected `[i32; 3]`, found `[u8; 3]`
  |
  = note: expected array `[i32; 3]`
             found array `[u8; 3]`
```

**The mistake.** `let one = [1, 2, 3];` has no element type yet. The call `takes_i32_array(one)` fixes it to `i32`, and that choice is final, so the later `[one, two]` beside a `[u8; 3]` cannot change it. Put the array next to `two` first and it would have been `[u8; 3]`.

**The fix.** Decide the type where the array is written. One suffix is enough: `[1u8, 2, 3]`. Then convert at the call with `one.map(i32::from)`.

```rust
fn takes_i32_array(a: [i32; 3]) -> i32 {
    a.iter().sum()
}

fn main() {
    let one = [1u8, 2, 3];
    let two: [u8; 3] = [4, 5, 6];
    println!("{}", takes_i32_array(one.map(i32::from)));
    let arrays = [one, two];
    println!("{arrays:?}");
}
```

**Read:** [Writing an array down](../writing_an_array_down/README.md#inference-reads-the-neighbours)

### 9. A `String` in a repeat expression

```rust,compile_fail
fn main() {
    let names = ["bb".to_string(); 3];
    println!("{names:?}");
}
```

```text title="rustc 1.98.0 on repeat_string.rs"
error[E0277]: the trait bound `String: Copy` is not satisfied
 --> repeat_string.rs:2:18
  |
2 |     let names = ["bb".to_string(); 3];
  |                  ^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `String`
  |
  = note: the `Copy` trait is required because this value will be copied for each element of the array
  = help: consider using `core::array::from_fn` to initialize the array
  = help: see https://doc.rust-lang.org/stable/std/array/fn.from_fn.html for more information
```

**The mistake.** `[value; 3]` evaluates `value` once and copies it, so for two or more elements the value must be `Copy` or a constant, and `String` is neither.

**The fix.** `std::array::from_fn(|_| "bb".to_string())` builds each element, which is rustc's own suggestion. `[const { String::new() }; 3]` works for a constant value, and `std::array::repeat(value)` (stable since 1.91.0) clones one.

```rust
fn main() {
    let names: [String; 3] = std::array::from_fn(|_| "bb".to_string());
    let empty = [const { String::new() }; 3];
    println!("{names:?} {empty:?}");
}
```

**Read:** [Writing an array down](../writing_an_array_down/README.md#value-how-many)

## Indexing

### 10. A constant index past the end

```rust,ignore
fn main() {
    let xs = [1, 2, 3];
    println!("{}", xs[3]);
}
```

```text title="rustc 1.98.0 on index_constant_oob.rs"
error: this operation will panic at runtime
 --> index_constant_oob.rs:3:20
  |
3 |     println!("{}", xs[3]);
  |                    ^^^^^ index out of bounds: the length is 3 but the index is 3
  |
  = note: `#[deny(unconditional_panic)]` on by default
```

**The mistake.** The index is a constant, so rustc knows the program will panic and refuses it. This is the deny-by-default lint `unconditional_panic`, not an `E` code, and `#![allow(unconditional_panic)]` turns it back into a run-time panic. Any index the compiler cannot evaluate is checked when the program runs.

**The fix.** `xs.get(3)` returns `None` instead of panicking, which fits an index that may be out of range.

```rust
fn main() {
    let xs = [1, 2, 3];
    println!("{:?}", xs.get(3)); // None
}
```

**Read:** [What array explanations get wrong, run](../array_claims_checked/README.md), [Arrays and slices](../../arrays_and_slices/README.md#out-of-bounds-is-a-panic-not-a-wrong-answer)

### 11. An `i32` as an index

```rust,compile_fail
fn main() {
    let xs = [10, 20, 30];
    let i: i32 = 1;
    println!("{}", xs[i]);
}
```

```text title="rustc 1.98.0 on index_with_i32.rs"
error[E0277]: the type `[{integer}]` cannot be indexed by `i32`
 --> index_with_i32.rs:4:23
  |
4 |     println!("{}", xs[i]);
  |                       ^ slice indices are of type `usize` or ranges of `usize`
  |
  = help: the trait `SliceIndex<[{integer}]>` is not implemented for `i32`
help: `usize` implements trait `SliceIndex<T>`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/slice/index.rs:179:0
  |
  = note: `SliceIndex<[T]>`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/bstr/traits.rs:197:0
  |
  = note: `SliceIndex<ByteStr>`
  = note: required for `[{integer}]` to implement `Index<i32>`
  = note: 1 redundant requirement hidden
  = note: required for `[{integer}; 3]` to implement `Index<i32>`
```

**The mistake.** Indexes are `usize`. Rust converts no numbers for you, so an `i32` variable is refused, while a bare literal `xs[1]` would have been inferred as `usize`.

**The fix.** Convert once, and decide what a negative index means: `usize::try_from(i)` fails for one, while `i as usize` wraps it to a huge index.

```rust
fn main() {
    let xs = [10, 20, 30];
    let i: i32 = 1;
    let at = usize::try_from(i).unwrap();
    println!("{}", xs[at]);
}
```

**Read:** [Casting with `as`](../../../29_Conversion/casting_with_as/README.md)

### 12. Writing into an array that is not `mut`

```rust,compile_fail
fn main() {
    let xs = [0; 3];
    xs[0] = 1;
    println!("{xs:?}");
}
```

```text title="rustc 1.98.0 on assign_immutable.rs"
error[E0594]: cannot assign to `xs[_]`, as `xs` is not declared as mutable
 --> assign_immutable.rs:3:5
  |
3 |     xs[0] = 1;
  |     ^^^^^^^^^ cannot assign
  |
help: consider changing this to be mutable
  |
2 |     let mut xs = [0; 3];
  |         +++
```

**The mistake.** Changing one element changes the array, and a binding without `mut` cannot be changed.

**The fix.** `let mut xs`. The length still cannot change: `mut` lets you overwrite elements, not add them.

```rust
fn main() {
    let mut xs = [0; 3];
    xs[0] = 1;
    println!("{xs:?}");
}
```

**Read:** [Variables](../../../15_First_Programs/variables/README.md)

### 13. Moving one `String` out of an array

```rust,compile_fail
fn main() {
    let names = [String::from("ada"), String::from("grace")];
    let first = names[0];
    println!("{first}");
}
```

```text title="rustc 1.98.0 on move_out_of_array.rs"
error[E0508]: cannot move out of type `[String; 2]`, a non-copy array
 --> move_out_of_array.rs:3:17
  |
3 |     let first = names[0];
  |                 ^^^^^^^^
  |                 |
  |                 cannot move out of here
  |                 move occurs because `names[_]` has type `String`, which does not implement the `Copy` trait
  |
help: consider borrowing here
  |
3 |     let first = &names[0];
  |                 +
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let first = names[0].clone();
  |                         ++++++++
```

**The mistake.** `names[0]` would leave a hole in an array that still owns the other element and will drop both. rustc lets you move a non-`Copy` element out only by taking the whole array apart.

**The fix.** Borrow it with `&names[0]`, clone it, or destructure the whole array with `let [ada, grace] = names;`.

```rust
fn main() {
    let names = [String::from("ada"), String::from("grace")];
    let first = &names[0];
    println!("{first}");
    let [ada, grace] = names;
    println!("{ada} {grace}");
}
```

**Read:** [`Copy` vs `Clone`](../../../16_Structs/copy_vs_clone/README.md)

## Methods an array does not have

### 14. `push` on an array

```rust,compile_fail
fn main() {
    let mut xs = [1, 2, 3];
    xs.push(4);
    println!("{xs:?}");
}
```

```text title="rustc 1.98.0 on push_on_array.rs"
error[E0599]: no method named `push` found for array `[{integer}; 3]` in the current scope
 --> push_on_array.rs:3:8
  |
3 |     xs.push(4);
  |        ^^^^ method not found in `[{integer}; 3]`
```

**The mistake.** An array's length is part of its type, so nothing can add an element. `push`, `pop`, `insert` and `remove` exist on `Vec`.

**The fix.** Use a `Vec` if the length changes. Otherwise build the full array up front.

```rust
fn main() {
    let mut xs = vec![1, 2, 3];
    xs.push(4);
    println!("{xs:?}");
}
```

**Read:** [Array or `Vec`?](../../array_or_vec/README.md)

### 15. `enumerate` on an array

```rust,compile_fail
fn main() {
    let grid = [[1, 2], [3, 4]];
    for (i, row) in grid.enumerate() {
        println!("{i}: {row:?}");
    }
}
```

```text title="rustc 1.98.0 on enumerate_on_array.rs"
error[E0599]: the method `enumerate` exists for array `[[{integer}; 2]; 2]`, but its trait bounds were not satisfied
 --> enumerate_on_array.rs:3:26
  |
3 |     for (i, row) in grid.enumerate() {
  |                          ^^^^^^^^^ method cannot be called on `[[{integer}; 2]; 2]` due to unsatisfied trait bounds
  |
  = note: the following trait bounds were not satisfied:
          `[[{integer}; 2]; 2]: Iterator`
          which is required by `&mut [[{integer}; 2]; 2]: Iterator`
          `[[{integer}; 2]]: Iterator`
          which is required by `&mut [[{integer}; 2]]: Iterator`
```

**The mistake.** `enumerate` is an `Iterator` method, and an array is not an iterator: it is a collection you can get one *from*. The notes say exactly that, as trait bounds.

**The fix.** `grid.iter().enumerate()` borrows, and `grid.into_iter().enumerate()` takes the rows by value.

```rust
fn main() {
    let grid = [[1, 2], [3, 4]];
    for (i, row) in grid.iter().enumerate() {
        println!("{i}: {row:?}");
    }
}
```

**Read:** [`iter`, `iter_mut` and `into_iter`](../../../24_Iterators/iter_iter_mut_into_iter/README.md)

### 16. Printing an array with `{}`

```rust,compile_fail
fn main() {
    let xs = [1, 2, 3];
    println!("{}", xs);
}
```

```text title="rustc 1.98.0 on display_array.rs"
error[E0277]: `[{integer}; 3]` doesn't implement `std::fmt::Display`
 --> display_array.rs:3:20
  |
3 |     println!("{}", xs);
  |               --   ^^ `[{integer}; 3]` cannot be formatted with the default formatter
  |               |
  |               required by this formatting parameter
  |
  = help: the trait `std::fmt::Display` is not implemented for `[{integer}; 3]`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

**The mistake.** Arrays implement `Debug` but not `Display`: std does not choose a separator for you.

**The fix.** `{:?}` or `{:#?}`, or join the elements yourself when the output format matters.

```rust
fn main() {
    let xs = [1, 2, 3];
    println!("{:?}", xs);
}
```

**Read:** [`Debug` and `Display`](../../../15_First_Programs/debug_vs_display/README.md)

### 17. Adding two arrays with `+`

```rust,compile_fail
fn main() {
    let a = [1, 2, 3];
    let b = [10, 20, 30];
    let c = a + b;
    println!("{c:?}");
}
```

```text title="rustc 1.98.0 on add_arrays.rs"
error[E0369]: cannot add `[{integer}; 3]` to `[{integer}; 3]`
 --> add_arrays.rs:4:15
  |
4 |     let c = a + b;
  |             - ^ - [{integer}; 3]
  |             |
  |             [{integer}; 3]
```

**The mistake.** No arithmetic operator is defined on arrays. Element-wise maths is not built in, the way it is in NumPy.

**The fix.** `std::array::from_fn(|i| a[i] + b[i])`, or `a.iter().zip(&b)` when you want a `Vec`.

```rust
fn main() {
    let a = [1, 2, 3];
    let b = [10, 20, 30];
    let c: [i32; 3] = std::array::from_fn(|i| a[i] + b[i]);
    println!("{c:?}"); // [11, 22, 33]
}
```

**Read:** [Writing an array down](../writing_an_array_down/README.md#arrayfrom_fn-is-not-iterfrom_fn)

### 18. `collect()` into an array

```rust,compile_fail
fn main() {
    let xs = [1, 2, 3];
    let doubled: [i32; 3] = xs.iter().map(|x| x * 2).collect();
    println!("{doubled:?}");
}
```

```text title="rustc 1.98.0 on collect_into_array.rs"
error[E0277]: an array of type `[i32; 3]` cannot be built directly from an iterator
 --> collect_into_array.rs:3:54
  |
3 |     let doubled: [i32; 3] = xs.iter().map(|x| x * 2).collect();
  |                                                      ^^^^^^^ try collecting into a `Vec<{integer}>`, then using `.try_into()`
  |
  = help: the trait `FromIterator<{integer}>` is not implemented for `[i32; 3]`
note: the method call chain might not have had the expected associated types
 --> collect_into_array.rs:3:39
  |
2 |     let xs = [1, 2, 3];
  |              --------- this expression has type `[{integer}; 3]`
3 |     let doubled: [i32; 3] = xs.iter().map(|x| x * 2).collect();
  |                                ------ ^^^^^^^^^^^^^^ `Iterator::Item` changed to `{integer}` here
  |                                |
  |                                `Iterator::Item` is `&{integer}` here
note: required by a bound in `collect`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/iter/traits/iterator.rs:2077:4
```

**The mistake.** `collect` cannot know in advance that the iterator has exactly three items, so `[T; N]` does not implement `FromIterator`.

**The fix.** `xs.map(|x| x * 2)` keeps the length in the type. From an iterator, collect a `Vec` and `try_into()` it.

```rust
fn main() {
    let xs = [1, 2, 3];
    let doubled: [i32; 3] = xs.map(|x| x * 2);
    println!("{doubled:?}"); // [2, 4, 6]
}
```

**Read:** [`collect` and `FromIterator`](../../../24_Iterators/collect_and_fromiterator/README.md)

### 19. `Default` for 33 elements

```rust,compile_fail
fn main() {
    let small: [u8; 32] = Default::default();
    let big: [u8; 33] = Default::default();
    println!("{} {}", small.len(), big.len());
}
```

```text title="rustc 1.98.0 on default_33.rs"
error[E0277]: the trait bound `[u8; 33]: Default` is not satisfied
 --> default_33.rs:3:25
  |
3 |     let big: [u8; 33] = Default::default();
  |                         ^^^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `[u8; 33]`
  |
  = help: the following other types implement trait `Default`:
            &[T]
            &mut [T]
            [T; 0]
            [T; 1]
            [T; 2]
            [T; 3]
            [T; 4]
            [T; 5]
          and 27 others
```

**The mistake.** `Default` is implemented for arrays of length 0 to 32 only, a limit left over from before const generics. `Copy`, `Clone`, `Debug`, `PartialEq`, `Hash` and `IntoIterator` work at any length.

**The fix.** `[0u8; 33]`, or `std::array::from_fn(|_| T::default())` for a generic `T`.

```rust
fn main() {
    let small: [u8; 32] = Default::default();
    let big = [0u8; 33];
    let built: [u8; 33] = std::array::from_fn(|_| u8::default());
    println!("{} {} {}", small.len(), big.len(), built.len());
}
```

**Read:** [What array explanations get wrong, run](../array_claims_checked/README.md)

### 20. A `String` built in a static

```rust,compile_fail
pub static UNIT_NAMES: [String; 1] = [String::from("meter")];
```

```text title="rustc 1.98.0 on static_string.rs"
error[E0015]: cannot call non-const associated function `<String as From<&str>>::from` in statics
 --> static_string.rs:1:39
  |
1 | pub static UNIT_NAMES: [String; 1] = [String::from("meter")];
  |                                       ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: calls in statics are limited to constant functions, tuple structs and tuple variants
  = note: consider wrapping this expression in `std::sync::LazyLock::new(|| ...)`
```

**The mistake.** A `static` is built at compile time, and `String::from` is not a `const fn`. `vec![]` in a `const` fails the same way, with `E0015`.

**The fix.** Store `&'static str` in the table. `String::new()` is `const`, so an array of empty strings would compile. `LazyLock` is the answer when the values really have to be computed.

```rust
pub static UNIT_NAMES: [&str; 1] = ["meter"];
pub static EMPTY: [String; 1] = [String::new()];
```

**Read:** [An array in a `const` or a `static`](../static_arrays/README.md)

## Ownership, iteration and filtering

### 21. Using an array of `String`s after moving it

```rust,compile_fail
fn main() {
    let names = [String::from("ada"), String::from("grace")];
    let copy = names;
    println!("{names:?} {copy:?}");
}
```

```text title="rustc 1.98.0 on array_moved.rs"
error[E0382]: borrow of moved value: `names`
 --> array_moved.rs:4:16
  |
2 |     let names = [String::from("ada"), String::from("grace")];
  |         ----- move occurs because `names` has type `[String; 2]`, which does not implement the `Copy` trait
3 |     let copy = names;
  |                ----- value moved here
4 |     println!("{names:?} {copy:?}");
  |                ^^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let copy = names.clone();
  |                     ++++++++
```

**The mistake.** An array is `Copy` only when its element type is. `[String; 2]` is not, so `let copy = names;` moved it.

**The fix.** `names.clone()` if you need two, or borrow with `&names`.

```rust
fn main() {
    let names = [String::from("ada"), String::from("grace")];
    let copy = names.clone();
    println!("{names:?} {copy:?}");
}
```

**Read:** [`Copy` vs `Clone`](../../../16_Structs/copy_vs_clone/README.md)

### 22. `*x` in a filter over `into_iter()`

```rust,compile_fail
fn main() {
    let numbers = [1, 2, 3, 2];
    let twos: Vec<i32> = numbers.into_iter().filter(|&x| *x == 2).collect();
    println!("{twos:?}");
}
```

```text title="rustc 1.98.0 on filter_deref_value.rs"
error[E0614]: type `{integer}` cannot be dereferenced
 --> filter_deref_value.rs:3:58
  |
3 |     let twos: Vec<i32> = numbers.into_iter().filter(|&x| *x == 2).collect();
  |                                                          ^^ can't be dereferenced
```

**The mistake.** Since edition 2021, `array.into_iter()` yields values, so `filter` hands the closure a `&i32`. `|&x|` then binds an `i32`, and an `i32` cannot be dereferenced. On edition 2018, `into_iter()` yielded `&i32` and the same line compiled.

**The fix.** Drop the `*`: `|&x| x == 2`. Or use `|x| *x == 2` without the pattern.

```rust
fn main() {
    let numbers = [1, 2, 3, 2];
    let twos: Vec<i32> = numbers.into_iter().filter(|&x| x == 2).collect();
    println!("{twos:?}");
}
```

**Read:** [Lints around arrays: `array_into_iter`](../array_lints/README.md#array_into_iter)

### 23. `|&&x|` from an answer written before 2021

```rust,compile_fail
fn main() {
    let numbers = [1, 2, 3, 2];
    let twos: Vec<i32> = numbers.into_iter().filter(|&&x| x == 2).collect();
    println!("{twos:?}");
}
```

```text title="rustc 1.98.0 on filter_double_ref.rs"
error[E0308]: mismatched types
 --> filter_double_ref.rs:3:55
  |
3 |     let twos: Vec<i32> = numbers.into_iter().filter(|&&x| x == 2).collect();
  |                                                      -^^
  |                                                      ||
  |                                                      |expected integer, found `&_`
  |                                                      expected due to this
  |
  = note:   expected type `{integer}`
          found reference `&_`
help: consider removing `&` from the pattern
  |
3 -     let twos: Vec<i32> = numbers.into_iter().filter(|&&x| x == 2).collect();
3 +     let twos: Vec<i32> = numbers.into_iter().filter(|&x| x == 2).collect();
  |
```

**The mistake.** Answers from before 2021 used `|&&x|` because an array's `into_iter()` then yielded references. On edition 2021 and later it yields values, and there is only one `&` to strip.

**The fix.** `|&x|` over `into_iter()`. `|&&x|` is still right over `.iter()`, where the items are references.

```rust
fn main() {
    let numbers = [1, 2, 3, 2];
    let by_value: Vec<i32> = numbers.into_iter().filter(|&x| x == 2).collect();
    let by_ref: Vec<&i32> = numbers.iter().filter(|&&x| x == 2).collect();
    println!("{by_value:?} {by_ref:?}");
}
```

**Read:** [What array explanations get wrong, run](../array_claims_checked/README.md)

### 24. Using an iterator after `filter` took it

```rust,compile_fail
fn main() {
    let range_iter = 1..6;
    let evens = range_iter.filter(|n| n % 2 == 0);
    for n in range_iter {
        println!("{n}");
    }
    println!("{:?}", evens.collect::<Vec<_>>());
}
```

```text title="rustc 1.98.0 on iterator_moved.rs"
error[E0382]: use of moved value: `range_iter`
 --> iterator_moved.rs:4:14
  |
2 |     let range_iter = 1..6;
  |         ---------- move occurs because `range_iter` has type `std::ops::Range<i32>`, which does not implement the `Copy` trait
3 |     let evens = range_iter.filter(|n| n % 2 == 0);
  |                            ---------------------- `range_iter` moved due to this method call
4 |     for n in range_iter {
  |              ^^^^^^^^^^ value used here after move
  |
note: `filter` takes ownership of the receiver `self`, which moves `range_iter`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/iter/traits/iterator.rs:952:17
help: you can `clone` the value and consume it, but this might not be your desired behavior
  |
3 |     let evens = range_iter.clone().filter(|n| n % 2 == 0);
  |                           ++++++++
```

**The mistake.** `filter` takes `self`: it consumes the range and returns a new iterator. The `range_iter` binding is gone after that line. Its result is lazy too, so nothing had run yet.

**The fix.** Chain the calls and loop over the result.

```rust
fn main() {
    for n in (1..6).filter(|n| n % 2 == 0) {
        println!("{n}");
    }
}
```

**Read:** [Iterators are lazy](../../../24_Iterators/iterators_are_lazy/README.md)

### 25. Adding `u8` elements to an `i32` sum

```rust,compile_fail
fn main() {
    let row = [200u8, 100, 0];
    let mut sum: i32 = 0;
    for n in row {
        sum += n;
    }
    println!("{sum}");
}
```

```text title="rustc 1.98.0 on sum_u8_into_i32.rs"
error[E0308]: mismatched types
 --> sum_u8_into_i32.rs:5:16
  |
5 |         sum += n;
  |                ^ expected `i32`, found `u8`

error[E0277]: cannot add-assign `u8` to `i32`
 --> sum_u8_into_i32.rs:5:13
  |
5 |         sum += n;
  |             ^^ no implementation for `i32 += u8`
  |
  = help: the trait `AddAssign<u8>` is not implemented for `i32`
help: `i32` implements trait `AddAssign<Rhs>`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/ops/arith.rs:786:8
  |
  = note: `AddAssign`
 ::: /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/ops/arith.rs:799:0
  |
  = note: in this macro invocation
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/internal_macros.rs:61:8
  |
  = note: `AddAssign<&i32>`
  = note: this error originates in the macro `add_assign_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
```

**The mistake.** There is no implicit widening, so `i32 += u8` does not exist. This fixes the *other* sum bug: with no annotation, `sum` would have been a `u8` and overflowed.

**The fix.** `i32::from(n)`, which cannot fail, and says so.

```rust
fn main() {
    let row = [200u8, 100, 0];
    let mut sum: i32 = 0;
    for n in row {
        sum += i32::from(n);
    }
    println!("{sum}"); // 300
}
```

**Read:** [Writing an array down](../writing_an_array_down/README.md#the-sum-nobody-typed-is-a-u8)

## Arrays in signatures, structs and statics

### 26. A getter that returns an array of `String`s

```rust,compile_fail
struct Roster {
    names: [String; 3],
}

impl Roster {
    fn names(&self) -> [String; 3] {
        self.names
    }
}

fn main() {
    let r = Roster { names: [String::from("a"), String::from("b"), String::from("c")] };
    println!("{:?}", r.names());
}
```

```text title="rustc 1.98.0 on return_array_of_strings.rs"
error[E0507]: cannot move out of `self.names` which is behind a shared reference
 --> return_array_of_strings.rs:7:9
  |
7 |         self.names
  |         ^^^^^^^^^^ move occurs because `self.names` has type `[String; 3]`, which does not implement the `Copy` trait
  |
help: consider cloning the value if the performance cost is acceptable
  |
7 |         self.names.clone()
  |                   ++++++++
```

**The mistake.** `self.data` behind `&self` can only be copied out, and `[String; 3]` is not `Copy`. The same body with `[i32; 3]` compiles and returns a copy.

**The fix.** Return `&[String; 3]` or `&[String]`, or clone when the caller really needs its own.

```rust
struct Roster {
    names: [String; 3],
}

impl Roster {
    fn names(&self) -> &[String; 3] {
        &self.names
    }
}

fn main() {
    let r = Roster { names: [String::from("a"), String::from("b"), String::from("c")] };
    println!("{:?}", r.names());
}
```

**Read:** [Arrays in and out of functions](../arrays_in_signatures/README.md#a-getter-returns-a-copy-or-refuses)

### 27. Returning `&[x, 1, 2]` as `&'static [i32]`

```rust,compile_fail
fn create_slice(x: i32) -> &'static [i32] {
    &[x, 1, 2]
}

fn main() {
    println!("{:?}", create_slice(0));
}
```

```text title="rustc 1.98.0 on promoted_slice_runtime.rs"
error[E0515]: cannot return reference to temporary value
 --> promoted_slice_runtime.rs:2:5
  |
2 |     &[x, 1, 2]
  |     ^---------
  |     ||
  |     |temporary value created here
  |     returns a reference to data owned by the current function
```

**The mistake.** `&[1, 2, 3]` can be returned as `'static` because it is a constant, which the compiler promotes to a static. With a run-time `x` there is nothing to promote, and the array is a temporary in the function's frame.

**The fix.** Return the array by value, `[i32; 3]`, or a `Vec` when the length varies.

```rust
fn create_array(x: i32) -> [i32; 3] {
    [x, 1, 2]
}

fn main() {
    println!("{:?}", create_array(0));
}
```

**Read:** [Arrays in and out of functions](../arrays_in_signatures/README.md#returning-static-i32-a-promoted-constant)

### 28. A `&str` field with no lifetime

```rust,compile_fail
pub enum Property {
    Length,
}

pub struct BaseAtom {
    pub names: [&str; 1],
    pub property: Property,
}
```

```text title="rustc 1.98.0 on struct_field_no_lifetime.rs"
error[E0106]: missing lifetime specifier
 --> struct_field_no_lifetime.rs:6:17
  |
6 |     pub names: [&str; 1],
  |                 ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
5 ~ pub struct BaseAtom<'a> {
6 ~     pub names: [&'a str; 1],
  |
```

**The mistake.** A struct that holds a reference must name how long the reference lives. Elision works in function signatures and in `static`/`const` types, but not in a struct definition.

**The fix.** Declare it on the struct: `BaseAtom<'a>` with `[&'a str; 1]`. The `static` can then write `[BaseAtom; 1]`, because a lifetime left out there means `'static`.

```rust
pub enum Property {
    Length,
}

pub struct BaseAtom<'a> {
    pub names: [&'a str; 1],
    pub property: Property,
}

pub static BASE_ATOMS: [BaseAtom; 1] = [BaseAtom { names: ["meter"], property: Property::Length }];
```

**Read:** [An array in a `const` or a `static`](../static_arrays/README.md#where-the-lifetime-has-to-be-written)

### 29. A borrowed local where `'static` is required

```rust,compile_fail
pub struct BaseAtom<'a> {
    pub names: [&'a str; 1],
}

fn register(atom: BaseAtom<'static>) -> usize {
    atom.names[0].len()
}

fn main() {
    let name = String::from("meter");
    let atom = BaseAtom { names: [name.as_str()] };
    println!("{}", register(atom));
}
```

```text title="rustc 1.98.0 on static_needs_static_borrow.rs"
error[E0597]: `name` does not live long enough
  --> static_needs_static_borrow.rs:11:35
   |
10 |     let name = String::from("meter");
   |         ---- binding `name` declared here
11 |     let atom = BaseAtom { names: [name.as_str()] };
   |                                   ^^^^ borrowed value does not live long enough
12 |     println!("{}", register(atom));
   |                    -------------- argument requires that `name` is borrowed for `'static`
13 | }
   | - `name` dropped here while still borrowed
```

**The mistake.** `BaseAtom<'static>` in the parameter says the `&str`s inside must live for the whole program. `name.as_str()` borrows a local that dies at the end of `main`.

**The fix.** Accept any lifetime (`BaseAtom<'_>`) if the function does not keep the value. Otherwise store string literals or owned `String`s.

```rust
pub struct BaseAtom<'a> {
    pub names: [&'a str; 1],
}

fn register(atom: BaseAtom<'_>) -> usize {
    atom.names[0].len()
}

fn main() {
    let name = String::from("meter");
    let atom = BaseAtom { names: [name.as_str()] };
    println!("{}", register(atom));
}
```

**Read:** [An array in a `const` or a `static`](../static_arrays/README.md#what-static-says-in-unitstatic)
## See also

- [Arrays: the map](../README.md) — every array page, and which question each one answers
- [Lints around arrays](../array_lints/README.md) — what compiles with a warning, where this page is what does not compile
- [What array explanations get wrong, run](../array_claims_checked/README.md) — several of these errors, met through a claim that says they should not happen
- [The error-code map](../../../ERRORS.md) — every code the library teaches, and the lesson for each
- [Every `ToOwned` error, and its fix](../../../12_Traits/how_to_learn_to_owned/to_owned_errors/README.md) — the same kind of page, for `ToOwned`, `Clone` and `Cow`

## Po polsku

Dwadzieścia dziewięć błędów kompilacji związanych z tablicami: kod, który je wywołuje, dokładny komunikat rustc 1.98.0, na czym polega pomyłka i poprawka, która się kompiluje. Najczęstsze grupy to długość jako część typu (`[u8; 3]` i `[u8; 4]` to różne typy, stąd `E0308` i „can't compare” `E0277`), jeden typ elementu na tablicę („array - must be the same type”, `E0308`), wyrażenie powtórzenia z wartością, która nie jest `Copy` (`E0277`, poprawka: `std::array::from_fn`), oraz metody, których tablica nie ma (`push`, `enumerate`, `collect`, `+`, `{}`).

Uwaga na błąd, który nie ma numeru: stały indeks poza zakresem (`xs[3]` dla trzech elementów) zatrzymuje lint `unconditional_panic`, domyślnie ustawiony na „deny”. Przy indeksie znanym dopiero w czasie działania program kończy się paniką. Filtrowanie tablicy po edycji 2021: `into_iter()` zwraca wartości, więc `|&x| *x == 2` daje `E0614`, a stare odpowiedzi z `|&&x|` dają `E0308`.

**Szukaj po polsku:** błędy tablic w Ruscie · `rust array E0308 expected an array with a size of` · `rust String is not Copy array repeat` · `rust array enumerate E0599`
