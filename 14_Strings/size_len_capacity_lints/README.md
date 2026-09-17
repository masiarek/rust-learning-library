# Lints around `size_of`, `len` and `capacity`: bad and good, run

[The anatomy of a `String`](../anatomy_of_a_string/README.md) › **Beside the lesson** · the errors, not the warnings: [Every `size_of`, `len` and `capacity` error, and its fix](../size_len_capacity_errors/README.md)

**Level:** 101 → 201 · a reference, by lint

**One line:** Twenty-six warnings from rustc and clippy that fire when you measure a `String` or a `Vec`: a length compared with zero the long way or counted by walking, a size taken of the wrong thing, and capacity bought and then wasted or trusted as if it were elements. Each has a program that triggers it, what the tool prints, a version that is silent, and when the lint is right. At the end are six mistakes no lint catches.

Every transcript was recorded on rustc 1.98.0 and clippy 0.1.98 by running the command in its title. The lines dropped from each are the documentation link, the `#[warn(...)]` or "requested on the command line" note, and the closing warning count. Both programs of every pair are compiled on every build by [`check_fences.py`](../../tools/check_fences.py), on edition 2024. What the build does not re-run is clippy, so a later clippy can reword a warning. Every trailing comment is what that program printed when it ran, on a 64-bit target, and every silent program was checked to print nothing else under the same command.

Four of these stop the build rather than warn: `absurd_extreme_comparisons`, `size_of_in_element_count`, `uninit_vec` and `vec_resize_to_zero` are deny-by-default correctness lints. Only `cargo clippy` runs them, so `cargo build` compiles all four bad programs without a word.

## Turning them on

rustc's lints and clippy's default groups need nothing: `cargo clippy` prints them. The opt-in ones take a flag or a `Cargo.toml` entry:

```sh
cargo clippy -- -W clippy::read_zero_byte_vec -W clippy::cast_possible_truncation
```

```toml
[lints.rust]
redundant_imports = "warn"

[lints.clippy]
read_zero_byte_vec = "warn"
cast_possible_truncation = "warn"
```

## Find the lint

| Lint | From | On by default? | Fires on |
|---|---|---|---|
| [`unused_comparisons`](#unused_comparisons) | rustc | yes | `s.len() >= 0` |
| [`unused_must_use`](#unused_must_use) | rustc | yes | `s.capacity();` and `s.len();` as statements |
| [`unused_mut`](#unused_mut) | rustc | yes | a `with_capacity` buffer that is never written |
| [`redundant_imports`](#redundant_imports) | rustc | no | `use std::mem::{size_of, size_of_val};` |
| [`len_zero`](#len_zero) | clippy · style | yes | `s.len() == 0` |
| [`comparison_to_empty`](#comparison_to_empty) | clippy · style | yes | `s == ""` |
| [`absurd_extreme_comparisons`](#absurd_extreme_comparisons) | clippy · correctness | yes, as an error | `s.len() <= 0` |
| [`bytes_count_to_len`](#bytes_count_to_len) | clippy · complexity | yes | `s.bytes().count()` |
| [`iter_count`](#iter_count) | clippy · complexity | yes | `.iter().count()` on a `Vec` |
| [`get_last_with_len`](#get_last_with_len) | clippy · complexity | yes | `bytes.get(bytes.len() - 1)` |
| [`len_without_is_empty`](#len_without_is_empty) | clippy · style | yes | a `pub fn len` with no `is_empty` beside it |
| [`size_of_ref`](#size_of_ref) | clippy · suspicious | yes | `size_of_val(&s)` when `s` is already a reference |
| [`manual_slice_size_calculation`](#manual_slice_size_calculation) | clippy · complexity | yes | `values.len() * size_of::<u32>()` on a slice |
| [`manual_bits`](#manual_bits) | clippy · style | yes | `size_of::<usize>() * 8` |
| [`size_of_in_element_count`](#size_of_in_element_count) | clippy · correctness | yes, as an error | a byte count where `copy_nonoverlapping` takes an element count |
| [`reserve_after_initialization`](#reserve_after_initialization) | clippy · complexity | yes | `Vec::new()` then `reserve(n)` |
| [`slow_vector_initialization`](#slow_vector_initialization) | clippy · perf | yes | `with_capacity(n)` then `resize(n, 0)` |
| [`repeat_vec_with_capacity`](#repeat_vec_with_capacity) | clippy · suspicious | yes | `vec![Vec::with_capacity(64); 3]` |
| [`vec_init_then_push`](#vec_init_then_push) | clippy · perf | yes | `with_capacity(2)` then two `push` calls |
| [`uninit_vec`](#uninit_vec) | clippy · correctness | yes, as an error | `with_capacity(n)` then `set_len(n)` |
| [`vec_resize_to_zero`](#vec_resize_to_zero) | clippy · correctness | yes, as an error | `v.resize(0, 5)` |
| [`cast_possible_truncation`](#cast_possible_truncation) | clippy · pedantic | no | `s.len() as u32` |
| [`with_capacity_zero`](#with_capacity_zero) | clippy · pedantic | no | `with_capacity(0)` |
| [`needless_collect`](#needless_collect) | clippy · nursery | no | `s.chars().collect::<Vec<_>>().len()` |
| [`read_zero_byte_vec`](#read_zero_byte_vec) | clippy · nursery | no | `read` into `Vec::with_capacity(64)` |
| [`string_slice`](#string_slice) | clippy · restriction | no | `&s[..s.len() - 1]` |

## rustc, on by default

### `unused_comparisons`

**rustc** · warn by default · fires on `s.len() >= 0`

```rust
fn main() {
    let s = String::from("Hello");
    if s.len() >= 0 {
        println!("{s}"); // Hello
    }
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: comparison is useless due to type limits
 --> bad.rs:3:8
  |
3 |     if s.len() >= 0 {
  |        ^^^^^^^^^^^^
  |
```

**Silent:**

```rust
fn main() {
    let s = String::from("Hello");
    if !s.is_empty() {
        println!("{s}"); // Hello
    }
}
```

**When it is right.** Always right. `len()` returns a `usize`, which is never negative, so `>= 0` is always true and the `if` always runs. The test you meant is usually `!s.is_empty()`. With `<= 0` instead, rustc says nothing and clippy stops the build: [`absurd_extreme_comparisons`](#absurd_extreme_comparisons).

### `unused_must_use`

**rustc** · warn by default · fires on `s.capacity();` and `s.len();` as statements

```rust
fn main() {
    let mut s = String::from("Hello");
    s.capacity();
    s.len();
    s.push('!');
    println!("{s}");
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: unused return value of `String::capacity` that must be used
 --> bad.rs:3:5
  |
3 |     s.capacity();
  |     ^^^^^^^^^^^^
  |
help: use `let _ = ...` to ignore the resulting value
  |
3 |     let _ = s.capacity();
  |     +++++++

warning: unused return value of `String::len` that must be used
 --> bad.rs:4:5
  |
4 |     s.len();
  |     ^^^^^^^
  |
help: use `let _ = ...` to ignore the resulting value
  |
4 |     let _ = s.len();
  |     +++++++
```

**Silent:**

```rust
fn main() {
    let mut s = String::from("Hello");
    s.push('!');
    println!("{} {}", s.len(), s.capacity() >= s.len()); // 6 true
}
```

**When it is right.** Always right. Both methods only read a number, so a call whose result is dropped does nothing. It often means `capacity()` was expected to change something: that is [error 15](../size_len_capacity_errors/README.md#15-capacity64-to-buy-room) when it has an argument, and this warning when it does not.

### `unused_mut`

**rustc** · warn by default · fires on a `with_capacity` buffer that is never written

```rust
fn main() {
    let mut s = String::with_capacity(64);
    println!("{}", s.capacity());
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: variable does not need to be mutable
 --> bad.rs:2:9
  |
2 |     let mut s = String::with_capacity(64);
  |         ----^
  |         |
  |         help: remove this `mut`
  |
```

**Silent:**

```rust
fn main() {
    let s = String::with_capacity(64);
    println!("{}", s.capacity()); // 64
}
```

**When it is right.** Right. Buying capacity does not need `mut`, and writing does ([error 18](../size_len_capacity_errors/README.md#18-room-bought-but-no-mut) is the other side). When this fires on a buffer, either the `mut` is left over, or the writes you meant to add are missing.

## rustc, opt-in

### `redundant_imports`

**rustc** · allow by default · fires on `use std::mem::{size_of, size_of_val};`

```rust
use std::mem::{size_of, size_of_val};

fn main() {
    let s = String::from("Hello");
    println!("{} {}", size_of::<String>(), size_of_val(&s));
}
```

```text title="rustc --edition 2024 -W redundant_imports bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: the item `size_of` is imported redundantly
 --> bad.rs:1:16
  |
1 | use std::mem::{size_of, size_of_val};
  |                ^^^^^^^
  |
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/std/src/prelude/mod.rs:166:12
  |
  = note: the item `size_of` is already defined here
  |

warning: the item `size_of_val` is imported redundantly
 --> bad.rs:1:25
  |
1 | use std::mem::{size_of, size_of_val};
  |                         ^^^^^^^^^^^
  |
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/std/src/prelude/mod.rs:166:12
  |
  = note: the item `size_of_val` is already defined here
```

**Silent:**

```rust
fn main() {
    let s = String::from("Hello");
    println!("{} {}", size_of::<String>(), size_of_val(&s)); // 24 24
}
```

**When it is right.** Right, and harmless. `size_of` and `size_of_val` have been in the prelude since Rust 1.80, so the import adds nothing. The lint is allow-by-default, and code that still builds on a compiler older than 1.80 needs the line. [Measuring the picture](../anatomy_of_a_string/README.md#measuring-the-picture) shows the line in a program that works.

## Clippy, on by default: the length

### `len_zero`

**clippy · style** · warn by default · fires on `s.len() == 0`

```rust
fn main() {
    let s = String::new();
    if s.len() == 0 {
        println!("empty"); // empty
    }
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: length comparison to zero
 --> bad.rs:3:8
  |
3 |     if s.len() == 0 {
  |        ^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `s.is_empty()`
  |
```

**Silent:**

```rust
fn main() {
    let s = String::new();
    if s.is_empty() {
        println!("empty"); // empty
    }
}
```

**When it is right.** Right. `is_empty()` asks the question directly, and every std collection has it. On your own type it needs an `is_empty` method, which [`len_without_is_empty`](#len_without_is_empty) asks for.

### `comparison_to_empty`

**clippy · style** · warn by default · fires on `s == ""`

```rust
fn main() {
    let s = String::new();
    if s == "" {
        println!("empty"); // empty
    }
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: comparison to empty slice
 --> bad.rs:3:8
  |
3 |     if s == "" {
  |        ^^^^^^^ help: using `is_empty` is clearer and more explicit: `s.is_empty()`
  |
```

**Silent:**

```rust
fn main() {
    let s = String::new();
    if s.is_empty() {
        println!("empty"); // empty
    }
}
```

**When it is right.** Right, for the same reason as `len_zero`. The comparison gives the same answer, and `is_empty()` says what you are asking.

### `absurd_extreme_comparisons`

**clippy · correctness** · deny by default · fires on `s.len() <= 0`

```rust
fn main() {
    let s = String::new();
    if s.len() <= 0 {
        println!("empty");
    }
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
 --> bad.rs:3:8
  |
3 |     if s.len() <= 0 {
  |        ^^^^^^^^^^^^
  |
  = help: because `0` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `s.len() == 0` instead
```

**Silent:**

```rust
fn main() {
    let s = String::new();
    if s.is_empty() {
        println!("empty"); // empty
    }
}
```

**When it is right.** Right, and it is an error: the build stops. A `usize` cannot be below 0, so `<= 0` can only mean `== 0`. The help suggests `s.len() == 0`, which [`len_zero`](#len_zero) then flags, so go straight to `is_empty()`. rustc's own `unused_comparisons` says nothing about `<= 0`.

### `bytes_count_to_len`

**clippy · complexity** · warn by default · fires on `s.bytes().count()`

```rust
fn main() {
    let s = String::from("zażółć");
    println!("{}", s.bytes().count()); // 10
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: using long and hard to read `.bytes().count()`
 --> bad.rs:3:20
  |
3 |     println!("{}", s.bytes().count()); // 10
  |                    ^^^^^^^^^^^^^^^^^ help: consider calling `.len()` instead: `s.len()`
  |
```

**Silent:**

```rust
fn main() {
    let s = String::from("zażółć");
    println!("{}", s.len()); // 10
}
```

**When it is right.** Right. `len()` reads the stored length, and `bytes().count()` walks every byte to reach the same number. If you wanted letters, neither is the answer: `chars().count()` is, and nothing flags that one.

### `iter_count`

**clippy · complexity** · warn by default · fires on `.iter().count()` on a `Vec`

```rust
fn main() {
    let bytes: Vec<u8> = "zażółć".bytes().collect();
    println!("{}", bytes.iter().count()); // 10
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: called `.iter().count()` on a `Vec`
 --> bad.rs:3:20
  |
3 |     println!("{}", bytes.iter().count()); // 10
  |                    ^^^^^^^^^^^^^^^^^^^^ help: try: `bytes.len()`
  |
```

**Silent:**

```rust
fn main() {
    let bytes: Vec<u8> = "zażółć".bytes().collect();
    println!("{}", bytes.len()); // 10
}
```

**When it is right.** Right. A `Vec` stores its length, so counting its items is a walk to a number it already has.

### `get_last_with_len`

**clippy · complexity** · warn by default · fires on `bytes.get(bytes.len() - 1)`

```rust
fn main() {
    let bytes = "Hello".as_bytes();
    println!("{:?}", bytes.get(bytes.len() - 1)); // Some(111)
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: accessing last element with `bytes.get(bytes.len() - 1)`
 --> bad.rs:3:22
  |
3 |     println!("{:?}", bytes.get(bytes.len() - 1)); // Some(111)
  |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `bytes.last()`
  |
```

**Silent:**

```rust
fn main() {
    let bytes = "Hello".as_bytes();
    println!("{:?}", bytes.last()); // Some(111)
}
```

**When it is right.** Right. `last()` returns the same `Option` and cannot go wrong on an empty slice. `bytes.len() - 1` can: on an empty slice the subtraction panics in a debug build, before `get` is called. That subtraction on its own is in [what no lint catches](#what-no-lint-catches).

### `len_without_is_empty`

**clippy · style** · warn by default · fires on a `pub fn len` with no `is_empty` beside it

```rust
pub struct Buffer {
    bytes: Vec<u8>,
}

impl Buffer {
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

fn main() {
    let b = Buffer { bytes: vec![1, 2, 3] };
    println!("{}", b.len()); // 3
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: struct `Buffer` has a public `len` method, but no `is_empty` method
 --> bad.rs:6:5
  |
6 |     pub fn len(&self) -> usize {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
```

**Silent:**

```rust
pub struct Buffer {
    bytes: Vec<u8>,
}

impl Buffer {
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

fn main() {
    let b = Buffer { bytes: vec![1, 2, 3] };
    println!("{} {}", b.len(), b.is_empty()); // 3 false
}
```

**When it is right.** Right for a public type. Callers expect the pair, and without `is_empty` they write `b.len() == 0`, which is `len_zero` in their code.

## Clippy, on by default: the size

### `size_of_ref`

**clippy · suspicious** · warn by default · fires on `size_of_val(&s)` when `s` is already a reference

```rust
fn header_bytes(s: &String) -> usize {
    size_of_val(&s)
}

fn main() {
    let s = String::from("Hello");
    println!("{}", header_bytes(&s)); // 8
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: argument to `size_of_val()` is a reference to a reference
 --> bad.rs:2:5
  |
2 |     size_of_val(&s)
  |     ^^^^^^^^^^^^^^^
  |
  = help: dereference the argument to `size_of_val()` to get the size of the value instead of the size of the reference-type
```

**Silent:**

```rust
fn header_bytes(s: &String) -> usize {
    size_of_val(s)
}

fn main() {
    let s = String::from("Hello");
    println!("{}", header_bytes(&s)); // 24
}
```

**When it is right.** Right, and it catches a wrong number. Inside `header_bytes`, `s` is already a `&String`, so `&s` is a `&&String` and the answer is 8: the size of a reference. Passing `s` itself gives 24, the `String`'s three words. [Measuring the picture](../anatomy_of_a_string/README.md#measuring-the-picture) has all three numbers.

### `manual_slice_size_calculation`

**clippy · complexity** · warn by default · fires on `values.len() * size_of::<u32>()` on a slice

```rust
fn bytes_of(values: &[u32]) -> usize {
    values.len() * size_of::<u32>()
}

fn main() {
    println!("{}", bytes_of(&[1, 2, 3])); // 12
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: manual slice size calculation
 --> bad.rs:2:5
  |
2 |     values.len() * size_of::<u32>()
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `std::mem::size_of_val(values)`
  |
```

**Silent:**

```rust
fn bytes_of(values: &[u32]) -> usize {
    size_of_val(values)
}

fn main() {
    println!("{}", bytes_of(&[1, 2, 3])); // 12
}
```

**When it is right.** Right. For a slice, `size_of_val` is the length times the element size, and it cannot pair the length of one slice with the element type of another. It only fires on a slice: the same expression on a `Vec` is silent.

### `manual_bits`

**clippy · style** · warn by default · fires on `size_of::<usize>() * 8`

```rust
fn main() {
    println!("{}", size_of::<usize>() * 8); // 64
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: usage of `size_of::<T>()` to obtain the size of `T` in bits
 --> bad.rs:2:20
  |
2 |     println!("{}", size_of::<usize>() * 8); // 64
  |                    ^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `(usize::BITS as usize)`
  |
```

**Silent:**

```rust
fn main() {
    println!("{}", usize::BITS); // 64
}
```

**When it is right.** Right. `usize::BITS` is the number, as a `u32` constant. The help wraps it in `as usize` to keep the expression's type. Drop the cast where a `u32` will do.

### `size_of_in_element_count`

**clippy · correctness** · deny by default · fires on a byte count where `copy_nonoverlapping` takes an element count

```rust
fn main() {
    let src = [1u32, 2, 3];
    let mut dst = [0u32; 3];
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), size_of::<u32>() * src.len());
    }
    println!("{dst:?}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
error: found a count of bytes instead of a count of elements of `T`
 --> bad.rs:5:71
  |
5 |         std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), size_of::<u32>() * src.len());
  |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: use a count of elements instead of a count of bytes, it already gets multiplied by the size of the type
```

**Silent:**

```rust
fn main() {
    let src = [1u32, 2, 3];
    let mut dst = [0u32; 3];
    dst.copy_from_slice(&src);
    println!("{dst:?}"); // [1, 2, 3]
}
```

**When it is right.** Right, and it is a memory bug, not a style point: the build stops. The count is in elements and gets multiplied by the element size again, so this asks for 12 `u32`s to be copied into room for 3. With `src.len()` as the count it would be correct. `copy_from_slice` needs no `unsafe` and takes the count from the slices.

## Clippy, on by default: the capacity

### `reserve_after_initialization`

**clippy · complexity** · warn by default · fires on `Vec::new()` then `reserve(n)`

```rust
fn main() {
    let mut v: Vec<u8> = Vec::new();
    v.reserve(64);
    v.push(1);
    println!("{}", v.len()); // 1
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: call to `reserve` immediately after creation
 --> bad.rs:2:5
  |
2 | /     let mut v: Vec<u8> = Vec::new();
3 | |     v.reserve(64);
  | |__________________^ help: consider using `Vec::with_capacity(/* Space hint */)`: `let mut v: Vec<u8> = Vec::with_capacity(64);`
  |
```

**Silent:**

```rust
fn main() {
    let mut v: Vec<u8> = Vec::with_capacity(64);
    v.push(1);
    println!("{}", v.len()); // 1
}
```

**When it is right.** Right, as a spelling: `with_capacity` says it in one line. It only knows `Vec`. The same two lines on a `String` are silent, which is in [what no lint catches](#what-no-lint-catches).

### `slow_vector_initialization`

**clippy · perf** · warn by default · fires on `with_capacity(n)` then `resize(n, 0)`

```rust
fn main() {
    let n = 64;
    let mut buf = Vec::with_capacity(n);
    buf.resize(n, 0u8);
    println!("{}", buf.len()); // 64
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: slow zero-filling initialization
 --> bad.rs:3:19
  |
3 |       let mut buf = Vec::with_capacity(n);
  |  ___________________^
4 | |     buf.resize(n, 0u8);
  | |______________________^ help: consider replacing this with: `vec![0; n]`
  |
```

**Silent:**

```rust
fn main() {
    let n = 64;
    let buf = vec![0u8; n];
    println!("{}", buf.len()); // 64
}
```

**When it is right.** Right. clippy's reason is that `vec![0; n]` can take zeroed memory from the allocator in one step, where `resize` writes each zero. Both build the same `Vec`.

### `repeat_vec_with_capacity`

**clippy · suspicious** · warn by default · fires on `vec![Vec::with_capacity(64); 3]`

```rust
fn main() {
    let rows = vec![Vec::<u8>::with_capacity(64); 3];
    let caps: Vec<usize> = rows.iter().map(Vec::capacity).collect();
    println!("{caps:?}"); // [0, 0, 64]
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: repeating `Vec::with_capacity` using `vec![x; n]`, which does not retain capacity
 --> bad.rs:2:16
  |
2 |     let rows = vec![Vec::<u8>::with_capacity(64); 3];
  |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: only the last `Vec` will have the capacity
help: if you intended to initialize multiple `Vec`s with an initial capacity, try
  |
2 -     let rows = vec![Vec::<u8>::with_capacity(64); 3];
2 +     let rows = (0..3).map(|_| Vec::<u8>::with_capacity(64)).collect::<Vec<_>>();
  |
```

**Silent:**

```rust
fn main() {
    let rows: Vec<Vec<u8>> = (0..3).map(|_| Vec::with_capacity(64)).collect();
    let caps: Vec<usize> = rows.iter().map(Vec::capacity).collect();
    println!("{caps:?}"); // [64, 64, 64]
}
```

**When it is right.** Right, and the note is exact. `vec![x; n]` clones `x` for all but the last element, and a clone of an empty `Vec` has no capacity. The silent version calls `with_capacity` once per row. The same mistake with `String::with_capacity` gets no warning, which is in [what no lint catches](#what-no-lint-catches).

### `vec_init_then_push`

**clippy · perf** · warn by default · fires on `with_capacity(2)` then two `push` calls

```rust
fn main() {
    let mut v = Vec::with_capacity(2);
    v.push(1u8);
    v.push(2);
    println!("{v:?}"); // [1, 2]
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: calls to `push` immediately after creation
 --> bad.rs:2:5
  |
2 | /     let mut v = Vec::with_capacity(2);
3 | |     v.push(1u8);
4 | |     v.push(2);
  | |______________^ help: consider using the `vec![]` macro: `let v = vec![..];`
  |
```

**Silent:**

```rust
fn main() {
    let v = vec![1u8, 2];
    println!("{v:?} {}", v.capacity()); // [1, 2] 2
}
```

**When it is right.** Right when the elements are known where the `Vec` is made. `vec![1, 2]` sizes the buffer to its elements, so the capacity you worked out by hand is not needed.

### `uninit_vec`

**clippy · correctness** · deny by default · fires on `with_capacity(n)` then `set_len(n)`

```rust
fn main() {
    let n = 64;
    let mut buf: Vec<u8> = Vec::with_capacity(n);
    unsafe {
        buf.set_len(n);
    }
    println!("{}", buf.len());
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
error: calling `set_len()` immediately after reserving a buffer creates uninitialized values
 --> bad.rs:3:5
  |
3 |     let mut buf: Vec<u8> = Vec::with_capacity(n);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
4 |     unsafe {
5 |         buf.set_len(n);
  |         ^^^^^^^^^^^^^^
  |
  = help: initialize the buffer or wrap the content in `MaybeUninit`
```

**Silent:**

```rust
fn main() {
    let n = 64;
    let buf = vec![0u8; n];
    println!("{}", buf.len()); // 64
}
```

**When it is right.** Right, and it stops the build: the bad program is undefined behaviour. `set_len(n)` claims `n` initialised bytes, and `with_capacity` only bought room for them. The length has to count values that exist. [Error 24](../size_len_capacity_errors/README.md#24-as_mut_vecset_len-without-unsafe) is the same promise broken on a `String`.

### `vec_resize_to_zero`

**clippy · correctness** · deny by default · fires on `v.resize(0, 5)`

```rust
fn main() {
    let mut v = vec![1u8, 2, 3];
    v.resize(0, 5);
    println!("{v:?}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
error: emptying a vector with `resize`
 --> bad.rs:3:5
  |
3 |     v.resize(0, 5);
  |     ^^------------
  |       |
  |       help: ...or you can empty the vector with: `clear()`
  |
  = help: the arguments may be inverted...
```

**Silent:**

```rust
fn main() {
    let mut v = vec![1u8, 2, 3];
    v.clear();
    println!("{} {}", v.len(), v.capacity()); // 0 3
}
```

**When it is right.** Right, and it stops the build. Either the arguments are swapped and `resize(5, 0)` was meant, or the `Vec` is being emptied, which `clear()` says. `clear` sets the length to 0 and keeps the capacity.

## Clippy, opt-in

### `cast_possible_truncation`

**clippy · pedantic** · allow by default · fires on `s.len() as u32`

```rust
fn main() {
    let s = String::from("Hello");
    let n = s.len() as u32;
    println!("{n}"); // 5
}
```

```text title="clippy-driver --edition 2024 -W clippy::cast_possible_truncation bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: casting `usize` to `u32` may truncate the value on targets with 64-bit wide pointers
 --> bad.rs:3:13
  |
3 |     let n = s.len() as u32;
  |             ^^^^^^^^^^^^^^
  |
  = help: if this is intentional allow the lint with `#[allow(clippy::cast_possible_truncation)]` ...
help: ... or use `try_from` and handle the error accordingly
  |
3 -     let n = s.len() as u32;
3 +     let n = u32::try_from(s.len());
  |
```

**Silent:**

```rust
fn main() {
    let s = String::from("Hello");
    let n = u32::try_from(s.len());
    println!("{n:?}"); // Ok(5)
}
```

**When it is right.** Right on a 64-bit target, where a length can pass `u32::MAX` and `as` then drops the high bits without a word. Noise where a check above the cast already bounds the length. [Error 9](../size_len_capacity_errors/README.md#9-the-length-stored-in-a-u32) is what happens with no `as` at all.

### `with_capacity_zero`

**clippy · pedantic** · allow by default · fires on `with_capacity(0)`

```rust
fn main() {
    let mut s = String::with_capacity(0);
    s.push('a');
    println!("{s}"); // a
}
```

```text title="clippy-driver --edition 2024 -W clippy::with_capacity_zero bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: calling `with_capacity(0)` is equivalent to `new()`
 --> bad.rs:2:17
  |
2 |     let mut s = String::with_capacity(0);
  |                 ^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: use `new()` instead
  |
2 -     let mut s = String::with_capacity(0);
2 +     let mut s = String::new();
  |
```

**Silent:**

```rust
fn main() {
    let mut s = String::new();
    s.push('a');
    println!("{s}"); // a
}
```

**When it is right.** Right: `with_capacity(0)` buys nothing, which is what `new()` already does, and `new()` says it.

### `needless_collect`

**clippy · nursery** · allow by default · fires on `s.chars().collect::<Vec<_>>().len()`

```rust
fn main() {
    let s = String::from("zażółć");
    println!("{}", s.chars().collect::<Vec<_>>().len()); // 6
}
```

```text title="clippy-driver --edition 2024 -W clippy::needless_collect bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: avoid using `collect()` when not needed
 --> bad.rs:3:30
  |
3 |     println!("{}", s.chars().collect::<Vec<_>>().len()); // 6
  |                              ^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace with: `count()`
  |
```

**Silent:**

```rust
fn main() {
    let s = String::from("zażółć");
    println!("{}", s.chars().count()); // 6
}
```

**When it is right.** Right. The `Vec` is built only to be counted and dropped, and `count()` walks the same characters without allocating. It is a nursery lint, still being developed, so it is off by default.

### `read_zero_byte_vec`

**clippy · nursery** · allow by default · fires on `read` into `Vec::with_capacity(64)`

```rust
use std::io::Read;

fn main() {
    let mut input: &[u8] = b"Hello";
    let mut buf: Vec<u8> = Vec::with_capacity(64);
    let n = input.read(&mut buf).unwrap();
    println!("{n} {buf:?}"); // 0 []
}
```

```text title="clippy-driver --edition 2024 -W clippy::read_zero_byte_vec bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: reading zero byte data to `Vec`
 --> bad.rs:6:13
  |
6 |     let n = input.read(&mut buf).unwrap();
  |             ^^^^^^^^^^^^^^^^^^^^
  |
help: try
  |
6 ~     buf.resize(64, 0);
7 ~     let n = input.read(&mut buf).unwrap();
  |
```

**Silent:**

```rust
use std::io::Read;

fn main() {
    let mut input: &[u8] = b"Hello";
    let mut buf = vec![0u8; 64];
    let n = input.read(&mut buf).unwrap();
    buf.truncate(n);
    println!("{n} {buf:?}"); // 5 [72, 101, 108, 108, 111]
}
```

**When it is right.** Right, and it is the capacity-is-not-length mistake by name. `read` fills the slice it is given, and a `Vec` with capacity 64 and length 0 is an empty slice, so it reads nothing and returns `Ok(0)`. It is a nursery lint, so turn it on for any code that reads into buffers.

### `string_slice`

**clippy · restriction** · allow by default · fires on `&s[..s.len() - 1]`

```rust
fn main() {
    let s = String::from("Hello");
    println!("{}", &s[..s.len() - 1]); // Hell
}
```

```text title="clippy-driver --edition 2024 -W clippy::string_slice bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: indexing into a string may panic if the index is within a UTF-8 character
 --> bad.rs:3:21
  |
3 |     println!("{}", &s[..s.len() - 1]); // Hell
  |                     ^^^^^^^^^^^^^^^^
  |
```

**Silent:**

```rust
fn main() {
    let s = String::from("Hello");
    let mut chars = s.chars();
    chars.next_back();
    println!("{}", chars.as_str()); // Hell
}
```

**When it is right.** Right in the case that matters: `s.len() - 1` is a byte position, and on `"zażółć"` it lands inside `ć`, so the slice panics. The silent version drops the last character whatever its width. It is a restriction lint, so it also fires on slices that are fine, such as one that ends at a position `find` returned.

## What no lint catches

Each of these is silent under `clippy-driver --edition 2024 -W clippy::pedantic -W clippy::nursery`, so it is silent under `cargo clippy`'s defaults too. Two of them panic when run.

```rust
fn main() {
    let s = String::from("zażółć");
    let bytes_of_text = size_of_val(&s);
    println!("{bytes_of_text}"); // 24
}
```

The name says text, and the answer is the `String`'s own three words. The text is 10 bytes, `s.len()`. [`size_of_ref`](#size_of_ref) fires one reference deeper, and this is the plain case it cannot tell from a correct one. [Measuring the picture](../anatomy_of_a_string/README.md#measuring-the-picture).

```rust
fn main() {
    let rows = vec![String::with_capacity(64); 3];
    let caps: Vec<usize> = rows.iter().map(String::capacity).collect();
    println!("{caps:?}"); // [0, 0, 64]
}
```

[`repeat_vec_with_capacity`](#repeat_vec_with_capacity) catches this for `Vec`, and the `String` version is silent. Two of the three buffers you paid for up front are gone. Build each one: `(0..3).map(|_| String::with_capacity(64)).collect::<Vec<_>>()`.

```rust
fn main() {
    let mut v: Vec<u8> = Vec::with_capacity(4);
    v[0] = 1;
    println!("{v:?}");
}
```

It panics with *index out of bounds: the len is 0 but the index is 0*. Capacity is room, not elements, and indexing checks the length. [`read_zero_byte_vec`](#read_zero_byte_vec) catches this mistake when `read` makes it. Nothing catches it for indexing. Use `push`, or `vec![0; 4]` when the elements should exist.

```rust
fn main() {
    let name = String::from("Łódź");
    println!("{}", name.len()); // 7
    if name.len() > 4 {
        println!("{name} is longer than 4 letters"); // Łódź is longer than 4 letters
    }
}
```

`Łódź` has 4 letters and 7 bytes, so a length limit written in letters and checked with `len()` rejects it. No lint knows which of the two you meant. Count with `chars().count()`, and read [Four lengths](../four_lengths/README.md) before you pick one for a database column.

```rust
fn main() {
    let s = String::new();
    let last_index = s.len() - 1;
    println!("{last_index}");
}
```

It panics with *attempt to subtract with overflow* in a debug build, and prints 18446744073709551615 in a release build, where the subtraction wraps. Only the restriction lint `clippy::arithmetic_side_effects` points at it. Check `is_empty()` first, or use `s.len().checked_sub(1)`.

```rust
fn main() {
    let mut s = String::new();
    s.reserve(64);
    s.push_str("Hi");
    println!("{s}"); // Hi
}
```

[`reserve_after_initialization`](#reserve_after_initialization) flags these lines for a `Vec` and says nothing for a `String`. This one is harmless: `String::with_capacity(64)` is only shorter.
## If you are coming from another language

**C and C++.** The deny-by-default pair is the C bug in Rust clothing. `memcpy(dst, src, n * sizeof *src)` takes **bytes**, so C code multiplies by the element size, and that habit carried into `copy_nonoverlapping`, which takes **elements**, is [`size_of_in_element_count`](#size_of_in_element_count). `std::vector::reserve` followed by writing through `v[i]` is undefined behaviour in C++, and Rust makes it a panic ([what no lint catches](#what-no-lint-catches)) or, with `set_len`, [`uninit_vec`](#uninit_vec). `sizeof(void *) * CHAR_BIT` is [`manual_bits`](#manual_bits). [A string is bytes up to a NUL ↗](https://masiarek.github.io/c-learning-library/03_Strings/a_string_is_bytes_up_to_a_nul/) runs `sizeof` against `strlen` in C.

What changes: in C each of these compiles silently and misbehaves at run time. Here four stop `cargo clippy` outright, and the rest of the table is advice.

**Java.** `list.size() == 0` is the Java habit behind [`len_zero`](#len_zero), and `isEmpty()` is the fix in both languages. `new ArrayList<>(64)` followed by `list.set(0, x)` throws `IndexOutOfBoundsException` for the same reason `v[0] = 1` panics here: capacity is not size. [Capacity, and the right tool ↗](https://masiarek.github.io/java-text-learning-library/06_Performance/capacity_and_the_right_tool/) is `StringBuilder`'s side of it.

**Python.** `if len(s) == 0:` is the habit behind [`len_zero`](#len_zero), and `if not s:` is Python's `is_empty()`. Python has no capacity you can buy, so the capacity lints have no counterpart there, and `len(s)` counts code points, which is [the letters gap](#what-no-lint-catches) from the other side: [Counting characters ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/counting_characters/index.html).

## See also

- [Every `size_of`, `len` and `capacity` error, and its fix](../size_len_capacity_errors/README.md) — what the compiler refuses, where this page is what it warns about
- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — the three numbers these lints are about, measured, with a kata that predicts them
- [Stack and heap](../../18_Ownership/stack_and_heap/README.md) — what `size_of` and `size_of_val` can and cannot see
- [Four lengths, and which one the other system means](../four_lengths/README.md) — the length limit that `len()` checks in the wrong unit
- [Lints around arrays](../../26_Collections/arrays/array_lints/README.md) — the same kind of page for arrays, with `useless_vec` and `manual_memcpy`
- [Strict clippy](../../05_Tooling/strict_lints/README.md) — turning whole lint groups on for a project

## Po polsku

Dwadzieścia sześć ostrzeżeń rustc i clippy przy mierzeniu `String` i `Vec`: długość porównywana z zerem na okrętkę albo liczona chodzeniem po bajtach, rozmiar wzięty z niewłaściwej rzeczy (`size_of_val(&s)`, gdy `s` już jest referencją, daje 8 zamiast 24) oraz pojemność kupiona i zmarnowana. Przy każdym jest program, który je wywołuje, dokładny komunikat, cicha wersja i ocena, kiedy lint ma rację.

Cztery z nich to błędy, a nie ostrzeżenia, i zatrzymują `cargo clippy`: `absurd_extreme_comparisons`, `size_of_in_element_count`, `uninit_vec` i `vec_resize_to_zero`. Najważniejsza lekcja jest jednak na końcu strony: **pojemność to nie elementy**. `Vec::with_capacity(4)` ma długość 0, więc `v[0] = 1` kończy się paniką, a żaden lint tego nie zauważa. Tak samo `vec![String::with_capacity(64); 3]` daje pojemności `[0, 0, 64]` bez słowa ostrzeżenia, choć ta sama pomyłka dla `Vec` jest wyłapywana. A „Łódź” ma cztery litery i siedem bajtów, więc limit „4 litery” sprawdzany przez `len()` ją odrzuca.

**Szukaj po polsku:** lint clippy długość zero `is_empty` · pojemność a długość `Vec` · `rust with_capacity index out of bounds len is 0` · `rust size_of_val reference to a reference`
