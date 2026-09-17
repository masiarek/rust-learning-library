# Every `size_of`, `len` and `capacity` error, and its fix

[The anatomy of a `String`](../anatomy_of_a_string/README.md) › **Beside the lesson** · the three numbers these errors are about, measured: [Measuring the picture](../anatomy_of_a_string/README.md#measuring-the-picture)

**Level:** 101 → 201 · a reference, by symptom

**One line:** Twenty-four refusals you meet when you measure a `String`: asking for its size, reading its length, buying capacity, and setting the length by hand. For each: the code that causes it, what rustc 1.98.0 prints, the mistake, and a fix. The broken code must fail and the fix must compile, and [`check_fences.py`](../../tools/check_fences.py) holds both on every build.

Every transcript was recorded on rustc 1.98.0 by compiling the file named in its title with `rustc --edition 2024`. The trailing `aborting due to` and `rustc --explain` lines are dropped. Transcripts are not regenerated on each commit the way an example's output is, so a later compiler can reword one. The fences are checked, so a broken example cannot quietly start compiling. Every trailing comment in a fix is what that fix printed when it ran, on a 64-bit target.

**Read the `help:` lines here with care.** In nine entries the suggestion compiles into something you did not mean, fails in a new way, or names an unrelated method: [1](#1-passing-a-value-to-size_of), [2](#2-a-variable-where-the-type-goes), [9](#9-the-length-stored-in-a-u32), [10](#10-charslen-to-count-letters), [12](#12-an-array-as-long-as-the-text), [14](#14-comparing-a-length-with-1), [15](#15-capacity64-to-buy-room), [16](#16-stringnew64), [20](#20-with_capacity64push_str-on-one-line). Each entry's *mistake* says which.

## Find the error

| # | Code | What rustc says | The mistake |
|---|---|---|---|
| 1 | `E0061` | [this function takes 0 arguments but 1 argument was supplied](#1-passing-a-value-to-size_of) | Passing a value to `size_of` |
| 2 | `E0435` | [attempt to use a non-constant value in a constant](#2-a-variable-where-the-type-goes) | A variable where the type goes |
| 3 | `E0308` | [mismatched types: expected `&_`, found `String`](#3-a-string-handed-to-size_of_val-by-value) | A `String` handed to `size_of_val` by value |
| 4 | `E0277` | [the size for values of type `str` cannot be known at compilation time](#4-size_ofstr) | `size_of::<str>()` |
| 5 | `E0433` | [cannot find module or crate `mem` in this scope](#5-memsize_of-with-no-use) | `mem::size_of` with no `use` |
| 6 | `E0277` | [`fn() -> usize {std::mem::size_of::<String>}` doesn't implement `std::fmt::Display`](#6-size_ofstring-without-the-call) | `size_of::<String>` without the call |
| 7 | `E0615` | [attempted to take value of method `len` on type `String`](#7-slen-without-the-call) | `s.len` without the call |
| 8 | `E0599` | [no method named `size` found for struct `String` in the current scope](#8-size-or-length) | `size()` or `length()` |
| 9 | `E0308` | [mismatched types: expected `u32`, found `usize`](#9-the-length-stored-in-a-u32) | The length stored in a `u32` |
| 10 | `E0599` | [no method named `len` found for struct `Chars<'a>` in the current scope](#10-charslen-to-count-letters) | `chars().len()` to count letters |
| 11 | `E0277` | [the type `str` cannot be indexed by `usize`](#11-the-last-character-as-sslen-1) | The last character as `s[s.len() - 1]` |
| 12 | `E0435` | [attempt to use a non-constant value in a constant](#12-an-array-as-long-as-the-text) | An array as long as the text |
| 13 | `E0015`, `E0493` | [cannot call non-const associated function `<String as From<&str>>::from` in constants](#13-a-strings-length-in-a-const) | A `String`'s length in a `const` |
| 14 | `E0600` | [cannot apply unary operator `-` to type `usize`](#14-comparing-a-length-with-1) | Comparing a length with `-1` |
| 15 | `E0061` | [this method takes 0 arguments but 1 argument was supplied](#15-capacity64-to-buy-room) | `capacity(64)` to buy room |
| 16 | `E0061` | [this function takes 0 arguments but 1 argument was supplied](#16-stringnew64) | `String::new(64)` |
| 17 | `E0070` | [invalid left-hand side of assignment](#17-assigning-to-capacity) | Assigning to `capacity()` |
| 18 | `E0596` | [cannot borrow `s` as mutable, as it is not declared as mutable](#18-room-bought-but-no-mut) | Room bought, but no `mut` |
| 19 | `E0308` | [mismatched types: expected `usize`, found `()`](#19-keeping-what-reserve-returns) | Keeping what `reserve` returns |
| 20 | `E0599` | [no method named `len` found for unit type `()` in the current scope](#20-with_capacity64push_str-on-one-line) | `with_capacity(64).push_str(…)` on one line |
| 21 | `E0308` | [mismatched types: expected `usize`, found `u32`](#21-a-u32-capacity) | A `u32` capacity |
| 22 | `E0599` | [no method named `capacity` found for reference `&str` in the current scope](#22-the-capacity-of-a-str) | The capacity of a `&str` |
| 23 | `E0599` | [no method named `set_len` found for struct `String` in the current scope](#23-set_len-on-a-string) | `set_len` on a `String` |
| 24 | `E0133` | [call to unsafe function `Vec::<T, A>::set_len` is unsafe and requires unsafe block](#24-as_mut_vecset_len-without-unsafe) | `as_mut_vec().set_len(…)` without `unsafe` |

## Asking for a size

### 1. Passing a value to `size_of`

```rust,compile_fail
fn main() {
    let s = String::from("Hello");
    println!("{}", size_of(s));
}
```

```text title="rustc 1.98.0 on size_of_a_value.rs"
error[E0061]: this function takes 0 arguments but 1 argument was supplied
 --> size_of_a_value.rs:3:20
  |
3 |     println!("{}", size_of(s));
  |                    ^^^^^^^ - unexpected argument of type `String`
  |
note: function defined here
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/mem/mod.rs:373:13
help: remove the extra argument
  |
3 -     println!("{}", size_of(s));
3 +     println!("{}", size_of());
  |
```

**The mistake.** `size_of` takes no arguments. It takes a type, written between `::<` and `>`, and its answer is fixed at compile time. The help deletes the argument, which leaves `size_of()` with nothing to measure: that line fails too, with `E0282`, *type annotations needed*.

**The fix.** Name the type, or hand the value to `size_of_val` by reference.

```rust
fn main() {
    let s = String::from("Hello");
    println!("{}", size_of::<String>()); // 24
    println!("{}", size_of_val(&s)); // 24
}
```

**Read:** [Measuring the picture](../anatomy_of_a_string/README.md#measuring-the-picture)

### 2. A variable where the type goes

```rust,compile_fail
fn main() {
    let s = String::from("Hello");
    println!("{}", size_of::<s>());
}
```

```text title="rustc 1.98.0 on variable_as_type.rs"
error[E0435]: attempt to use a non-constant value in a constant
 --> variable_as_type.rs:3:30
  |
3 |     println!("{}", size_of::<s>());
  |                              ^ non-constant value
  |
help: consider using `const` instead of `let`
  |
2 -     let s = String::from("Hello");
2 +     const s: /* Type */ = String::from("Hello");
  |
```

**The mistake.** The brackets after `size_of::` take generic arguments, and a generic argument can be a type or a constant. `s` is not a type, so rustc reads it as a constant and then refuses it for not being one. The `const s` help is a dead end: with a real `const s`, the line fails with `E0747`, *constant provided when a type was expected*.

**The fix.** Put the type in the brackets, or measure the variable with `size_of_val`.

```rust
fn main() {
    let s = String::from("Hello");
    println!("{}", size_of::<String>()); // 24
    println!("{}", size_of_val(&s)); // 24
}
```

**Read:** [Measuring the picture](../anatomy_of_a_string/README.md#measuring-the-picture)

### 3. A `String` handed to `size_of_val` by value

```rust,compile_fail
fn main() {
    let s = String::from("Hello");
    println!("{}", size_of_val(s));
}
```

```text title="rustc 1.98.0 on size_of_val_by_value.rs"
error[E0308]: mismatched types
 --> size_of_val_by_value.rs:3:32
  |
3 |     println!("{}", size_of_val(s));
  |                    ----------- ^ expected `&_`, found `String`
  |                    |
  |                    arguments to this function are incorrect
  |
  = note: expected reference `&_`
                found struct `String`
note: function defined here
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/mem/mod.rs:408:13
help: consider borrowing here
  |
3 |     println!("{}", size_of_val(&s));
  |                                +
```

**The mistake.** `size_of_val` takes a reference, `&T`. A reference is what lets it measure values with no fixed size, such as a `str`. By value, it would also have to take ownership of `s`. The help is right.

**The fix.** Borrow it.

```rust
fn main() {
    let s = String::from("Hello");
    println!("{}", size_of_val(&s)); // 24
    println!("{}", s.len()); // 5
}
```

**Read:** [Stack and heap](../../18_Ownership/stack_and_heap/README.md)

### 4. `size_of::<str>()`

```rust,compile_fail
fn main() {
    println!("{}", size_of::<str>());
}
```

```text title="rustc 1.98.0 on size_of_str.rs"
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> size_of_str.rs:2:30
  |
2 |     println!("{}", size_of::<str>());
  |                              ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
note: required by an implicit `Sized` bound in `std::mem::size_of`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/mem/mod.rs:373:0
```

**The mistake.** A `str` is as long as its text, so the type has no size. `size_of` needs a type whose every value has the same size, which is what `Sized` means.

**The fix.** Measure one particular `str` with `size_of_val`, or measure the reference to it, which is `Sized`: a pointer and a length.

```rust
fn main() {
    println!("{}", size_of_val("Hello")); // 5
    println!("{}", size_of::<&str>()); // 16
}
```

**Read:** [`str` is unsized](../str_is_unsized/README.md)

### 5. `mem::size_of` with no `use`

```rust,compile_fail
fn main() {
    println!("{}", mem::size_of::<String>());
}
```

```text title="rustc 1.98.0 on mem_not_imported.rs"
error[E0433]: cannot find module or crate `mem` in this scope
 --> mem_not_imported.rs:2:20
  |
2 |     println!("{}", mem::size_of::<String>());
  |                    ^^^ use of unresolved module or unlinked crate `mem`
  |
  = help: you might be missing a crate named `mem`
help: consider importing this module
  |
1 + use std::mem;
  |
```

**The mistake.** `mem` is a module inside `std`. A path that starts with `mem::` needs `use std::mem;` first. The bare names need nothing: `size_of` and `size_of_val` have been in the prelude since Rust 1.80.

**The fix.** Drop the `mem::`, or spell the whole path.

```rust
fn main() {
    println!("{}", size_of::<String>()); // 24
    println!("{}", std::mem::size_of::<String>()); // 24
}
```

**Read:** [Bringing names in with `use`](../../27_Modules/the_use_declaration/README.md)

### 6. `size_of::<String>` without the call

```rust,compile_fail
fn main() {
    let n = size_of::<String>;
    println!("{n}");
}
```

```text title="rustc 1.98.0 on size_of_without_parens.rs"
error[E0277]: `fn() -> usize {std::mem::size_of::<String>}` doesn't implement `std::fmt::Display`
 --> size_of_without_parens.rs:3:15
  |
3 |     println!("{n}");
  |               ^^^ `fn() -> usize {std::mem::size_of::<String>}` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for fn item `fn() -> usize {std::mem::size_of::<String>}`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

**The mistake.** Without `()`, `n` is the function itself, not the number it returns. A function cannot be printed with `{}`, and the `{:?}` the note offers is refused the same way, for `Debug`.

**The fix.** Call it.

```rust
fn main() {
    let n = size_of::<String>();
    println!("{n}"); // 24
}
```

**Read:** [Measuring the picture](../anatomy_of_a_string/README.md#measuring-the-picture)

## Reading the length

### 7. `s.len` without the call

```rust,compile_fail
fn main() {
    let s = String::from("Hello");
    println!("{}", s.len);
}
```

```text title="rustc 1.98.0 on len_without_parens.rs"
error[E0615]: attempted to take value of method `len` on type `String`
 --> len_without_parens.rs:3:22
  |
3 |     println!("{}", s.len);
  |                      ^^^ method, not a field
  |
help: use parentheses to call the method
  |
3 |     println!("{}", s.len());
  |                         ++
```

**The mistake.** `len` is a method, not a field. The length is stored in the `String`, but its fields are private, so the method is the only way to read it.

**The fix.** Call it.

```rust
fn main() {
    let s = String::from("Hello");
    println!("{}", s.len()); // 5
}
```

**Read:** [`String::len`](../string_methods/string_len/README.md)

### 8. `size()` or `length()`

```rust,compile_fail
fn main() {
    let s = String::from("Hello");
    println!("{}", s.size());
}
```

```text title="rustc 1.98.0 on size_method.rs"
error[E0599]: no method named `size` found for struct `String` in the current scope
 --> size_method.rs:3:22
  |
3 |     println!("{}", s.size());
  |                      ^^^^
  |
help: you might have meant to use `len`
  |
3 -     println!("{}", s.size());
3 +     println!("{}", s.len());
  |
```

**The mistake.** The name comes from another language: C++ has `size()` and `length()`, and Java has `size()` on collections and `length()` on strings. Every std collection in Rust calls it `len`. `s.length()` gets the same `help: you might have meant to use len`.

**The fix.** Use `len`.

```rust
fn main() {
    let s = String::from("Hello");
    println!("{}", s.len()); // 5
}
```

**Read:** ["No method named …"](../../12_Traits/no_method_named/README.md)

### 9. The length stored in a `u32`

```rust,compile_fail
fn main() {
    let s = String::from("Hello");
    let n: u32 = s.len();
    println!("{n}");
}
```

```text title="rustc 1.98.0 on len_into_u32.rs"
error[E0308]: mismatched types
 --> len_into_u32.rs:3:18
  |
3 |     let n: u32 = s.len();
  |            ---   ^^^^^^^ expected `u32`, found `usize`
  |            |
  |            expected due to this
  |
help: you can convert a `usize` to a `u32` and panic if the converted value doesn't fit
  |
3 |     let n: u32 = s.len().try_into().unwrap();
  |                         ++++++++++++++++++++
```

**The mistake.** `len` returns `usize`, and Rust never converts between integer types on its own. The help's `try_into().unwrap()` compiles, and panics for a text longer than `u32::MAX` bytes.

**The fix.** Keep lengths as `usize`. Where a `u32` is really needed, convert with `u32::try_from` and decide what a too-long text means.

```rust
fn main() {
    let s = String::from("Hello");
    let n: usize = s.len();
    let small = u32::try_from(s.len()).expect("under 4 GiB of text");
    println!("{n} {small}"); // 5 5
}
```

**Read:** [`TryFrom` and `TryInto`](../../29_Conversion/tryfrom_and_tryinto/README.md)

### 10. `chars().len()` to count letters

```rust,compile_fail
fn main() {
    let s = String::from("zażółć");
    println!("{}", s.chars().len());
}
```

```text title="rustc 1.98.0 on chars_len.rs"
error[E0599]: no method named `len` found for struct `Chars<'a>` in the current scope
 --> chars_len.rs:3:30
  |
3 |     println!("{}", s.chars().len());
  |                    -         ^^^
  |                    |
  |                    method `len` is available on `&str`
  |
help: there is a method `le` with a similar name, but with different arguments
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/iter/traits/iterator.rs:3977:4
```

**The mistake.** `len()` counts bytes, so reaching for the characters is right. But one character is one to four bytes, so `Chars` cannot know how many are left without walking the text. It has no `len`. The help about `le` is a spelling match: `le` is *less than or equal*.

**The fix.** Count them, and know that the count walks the whole string.

```rust
fn main() {
    let s = String::from("zażółć");
    println!("{}", s.chars().count()); // 6
    println!("{}", s.len()); // 10
}
```

**Read:** [Four lengths](../four_lengths/README.md), [Meet the `char`](../meet_the_char/README.md)

### 11. The last character as `s[s.len() - 1]`

```rust,compile_fail
fn main() {
    let s = String::from("Hello");
    let last = s[s.len() - 1];
    println!("{last}");
}
```

```text title="rustc 1.98.0 on index_by_len.rs"
error[E0277]: the type `str` cannot be indexed by `usize`
 --> index_by_len.rs:3:18
  |
3 |     let last = s[s.len() - 1];
  |                  ^^^^^^^^^^^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `usize`
help: `usize` implements trait `SliceIndex<T>`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/slice/index.rs:179:0
  |
  = note: `SliceIndex<[T]>`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/bstr/traits.rs:197:0
  |
  = note: `SliceIndex<ByteStr>`
  = note: required for `String` to implement `Index<usize>`
```

**The mistake.** `s.len() - 1` is a byte position, and a `String` cannot be indexed by one. The byte there may be the middle of a character, which is not a `char`.

**The fix.** Ask for what you mean: the last byte from `as_bytes()`, or the last character from `chars()`.

```rust
fn main() {
    let s = String::from("Hello");
    let last_byte = s.as_bytes()[s.len() - 1];
    let last_char = s.chars().last();
    println!("{last_byte} {last_char:?}"); // 111 Some('o')
}
```

**Read:** [String slices](../string_slices/README.md), [Walking a `String`](../walking_a_string/README.md)

### 12. An array as long as the text

```rust,compile_fail
fn main() {
    let s = String::from("Hello");
    let buf = [0u8; s.len()];
    println!("{buf:?}");
}
```

```text title="rustc 1.98.0 on len_as_array_length.rs"
error[E0435]: attempt to use a non-constant value in a constant
 --> len_as_array_length.rs:3:21
  |
3 |     let buf = [0u8; s.len()];
  |                     ^ non-constant value
  |
help: consider using `const` instead of `let`
  |
2 -     let s = String::from("Hello");
2 +     const s: /* Type */ = String::from("Hello");
  |
```

**The mistake.** An array's length is part of its type, so the compiler has to know it. `s.len()` is known only when the program runs. The `const s` help cannot work: `String::from` cannot run in a constant, which is the next error. `size_of::<String>()` *is* a constant, so it can size an array.

**The fix.** Use a `Vec` for a length that comes from data.

```rust
fn main() {
    let s = String::from("Hello");
    let buf = vec![0u8; s.len()];
    let header = [0u8; size_of::<String>()];
    println!("{} {}", buf.len(), header.len()); // 5 24
}
```

**Read:** [Every array error: a run-time length in a repeat expression](../../26_Collections/arrays/array_errors/README.md#4-a-run-time-length-in-a-repeat-expression)

### 13. A `String`'s length in a `const`

```rust,compile_fail
const LEN: usize = String::from("Hello").len();

fn main() {
    println!("{LEN}");
}
```

```text title="rustc 1.98.0 on string_from_in_const.rs"
error[E0015]: cannot call non-const associated function `<String as From<&str>>::from` in constants
 --> string_from_in_const.rs:1:20
  |
1 | const LEN: usize = String::from("Hello").len();
  |                    ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: calls in constants are limited to constant functions, tuple structs and tuple variants

error[E0493]: destructor of `String` cannot be evaluated at compile-time
 --> string_from_in_const.rs:1:20
  |
1 | const LEN: usize = String::from("Hello").len();
  |                    ^^^^^^^^^^^^^^^^^^^^^     - value is dropped here
  |                    |
  |                    the destructor for this type cannot be evaluated in constants
  |
  = note: see issue #133214 <https://github.com/rust-lang/rust/issues/133214> for more information
```

**The mistake.** The compiler computes a constant, and only `const fn`s can run there. `String::from` allocates, which a constant cannot, and dropping the `String` at the end of the line is refused too. `str::len` and `size_of` are `const fn`s.

**The fix.** Take the length of the literal, which needs no `String`.

```rust
const LEN: usize = "Hello".len();
const HEADER: usize = size_of::<String>();

fn main() {
    println!("{LEN} {HEADER}"); // 5 24
}
```

**Read:** [`const` and `static`](../../27_Modules/const_and_static/README.md)

### 14. Comparing a length with `-1`

```rust,compile_fail
fn main() {
    let s = String::new();
    if s.len() > -1 {
        println!("not empty");
    }
}
```

```text title="rustc 1.98.0 on len_minus_one_compare.rs"
error[E0600]: cannot apply unary operator `-` to type `usize`
 --> len_minus_one_compare.rs:3:18
  |
3 |     if s.len() > -1 {
  |                  ^^ cannot apply unary operator `-`
  |
  = note: unsigned values cannot be negated
help: you may have meant the maximum value of `usize`
  |
3 -     if s.len() > -1 {
3 +     if s.len() > usize::MAX {
  |
```

**The mistake.** `usize` has no negative values, so `-1` has no type it could be. The help's `usize::MAX` compiles and is never true, since no length is greater than it.

**The fix.** Ask the question directly.

```rust
fn main() {
    let s = String::new();
    if !s.is_empty() {
        println!("not empty");
    }
    println!("{}", s.is_empty()); // true
}
```

**Read:** [`String::is_empty`](../string_methods/string_is_empty/README.md)

## Buying and reading capacity

### 15. `capacity(64)` to buy room

```rust,compile_fail
fn main() {
    let mut s = String::new();
    s.capacity(64);
    s.push_str("Hi");
}
```

```text title="rustc 1.98.0 on capacity_with_argument.rs"
error[E0061]: this method takes 0 arguments but 1 argument was supplied
 --> capacity_with_argument.rs:3:7
  |
3 |     s.capacity(64);
  |       ^^^^^^^^ -- unexpected argument of type `{integer}`
  |
note: method defined here
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/string.rs:1177:17
help: remove the extra argument
  |
3 -     s.capacity(64);
3 +     s.capacity();
  |
```

**The mistake.** `capacity()` only reads the number. The help deletes the `64`, which compiles and buys nothing.

**The fix.** `reserve` buys room in a `String` you already have. `reserve(64)` promises room for at least 64 more bytes.

```rust
fn main() {
    let mut s = String::new();
    s.reserve(64);
    s.push_str("Hi");
    println!("{}", s.capacity() >= 64); // true
}
```

**Read:** [`String::reserve`](../string_methods/string_reserve/README.md), [Growth runs ahead of you](../anatomy_of_a_string/README.md#growth-runs-ahead-of-you)

### 16. `String::new(64)`

```rust,compile_fail
fn main() {
    let mut s = String::new(64);
    s.push_str("Hi");
}
```

```text title="rustc 1.98.0 on new_with_argument.rs"
error[E0061]: this function takes 0 arguments but 1 argument was supplied
 --> new_with_argument.rs:2:17
  |
2 |     let mut s = String::new(64);
  |                 ^^^^^^^^^^^ -- unexpected argument of type `{integer}`
  |
note: associated function defined here
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/string.rs:446:17
help: remove the extra argument
  |
2 -     let mut s = String::new(64);
2 +     let mut s = String::new();
  |
```

**The mistake.** `String::new()` takes nothing and allocates nothing. The constructor that takes a size has its own name. The help deletes the `64` and the room with it.

**The fix.** Use `with_capacity`.

```rust
fn main() {
    let mut s = String::with_capacity(64);
    s.push_str("Hi");
    println!("{} {}", s.len(), s.capacity()); // 2 64
}
```

**Read:** [`String::with_capacity`](../string_methods/string_with_capacity/README.md)

### 17. Assigning to `capacity()`

```rust,compile_fail
fn main() {
    let mut s = String::new();
    s.capacity() = 64;
    s.push_str("Hi");
}
```

```text title="rustc 1.98.0 on assign_to_capacity.rs"
error[E0070]: invalid left-hand side of assignment
 --> assign_to_capacity.rs:3:18
  |
3 |     s.capacity() = 64;
  |     ------------ ^
  |     |
  |     cannot assign to this expression
```

**The mistake.** `capacity()` returns a copy of the number, and a copy is not a place you can assign to. Capacity changes only through methods that allocate or free: `reserve`, `reserve_exact`, `shrink_to` and `shrink_to_fit`.

**The fix.** Reserve the room.

```rust
fn main() {
    let mut s = String::new();
    s.reserve_exact(64);
    s.push_str("Hi");
    println!("{}", s.capacity() >= 64); // true
}
```

**Read:** [`String::reserve_exact`](../string_methods/string_reserve_exact/README.md)

### 18. Room bought, but no `mut`

```rust,compile_fail
fn main() {
    let s = String::with_capacity(8);
    s.push_str("Hi");
    println!("{s}");
}
```

```text title="rustc 1.98.0 on with_capacity_not_mut.rs"
error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
 --> with_capacity_not_mut.rs:3:5
  |
3 |     s.push_str("Hi");
  |     ^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut s = String::with_capacity(8);
  |         +++
```

**The mistake.** `with_capacity` buys room, not permission to write. Any method that changes the `String` needs a mutable binding, whatever its capacity.

**The fix.** Declare it `mut`.

```rust
fn main() {
    let mut s = String::with_capacity(8);
    s.push_str("Hi");
    println!("{s}"); // Hi
}
```

**Read:** [Building a `String`](../building_a_string/README.md)

### 19. Keeping what `reserve` returns

```rust,compile_fail
fn main() {
    let mut s = String::new();
    let cap: usize = s.reserve(10);
    println!("{cap}");
}
```

```text title="rustc 1.98.0 on reserve_returns_unit.rs"
error[E0308]: mismatched types
 --> reserve_returns_unit.rs:3:22
  |
3 |     let cap: usize = s.reserve(10);
  |              -----   ^^^^^^^^^^^^^ expected `usize`, found `()`
  |              |
  |              expected due to this
  |
note: method `reserve` modifies its receiver in-place
 --> reserve_returns_unit.rs:3:24
  |
3 |     let cap: usize = s.reserve(10);
  |                        ^^^^^^^ this call modifies `s` in-place
```

**The mistake.** `reserve` changes `s` and returns `()`. The note says so: *this call modifies `s` in-place*.

**The fix.** Call it, then read the capacity.

```rust
fn main() {
    let mut s = String::new();
    s.reserve(10);
    let cap = s.capacity();
    println!("{}", cap >= 10); // true
}
```

**Read:** [`String::reserve`](../string_methods/string_reserve/README.md)

### 20. `with_capacity(64).push_str(…)` on one line

```rust,compile_fail
fn main() {
    let s = String::with_capacity(64).push_str("Hi");
    println!("{}", s.len());
}
```

```text title="rustc 1.98.0 on chained_push_str.rs"
error[E0599]: no method named `len` found for unit type `()` in the current scope
 --> chained_push_str.rs:3:22
  |
3 |     println!("{}", s.len());
  |                      ^^^
  |
help: there is a method `le` with a similar name, but with different arguments
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/cmp.rs:1428:4
```

**The mistake.** `push_str` returns `()`, so `s` is `()`. The `String` it pushed into was a temporary, dropped at the end of the line. The help about `le` is a spelling match.

**The fix.** Name the `String` first, then push.

```rust
fn main() {
    let mut s = String::with_capacity(64);
    s.push_str("Hi");
    println!("{} {}", s.len(), s.capacity()); // 2 64
}
```

**Read:** [`String::push_str`](../string_methods/string_push_str/README.md)

### 21. A `u32` capacity

```rust,compile_fail
fn main() {
    let n: u32 = 64;
    let s = String::with_capacity(n);
    println!("{}", s.capacity());
}
```

```text title="rustc 1.98.0 on with_capacity_u32.rs"
error[E0308]: mismatched types
 --> with_capacity_u32.rs:3:35
  |
3 |     let s = String::with_capacity(n);
  |             --------------------- ^ expected `usize`, found `u32`
  |             |
  |             arguments to this function are incorrect
  |
note: associated function defined here
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/string.rs:493:11
help: you can convert a `u32` to a `usize` and panic if the converted value doesn't fit
  |
3 |     let s = String::with_capacity(n.try_into().unwrap());
  |                                    ++++++++++++++++++++
```

**The mistake.** Sizes and capacities are `usize` everywhere in std. A size read as a `u32`, from a file header or a network packet, needs an explicit conversion.

**The fix.** Convert with `usize::try_from`. On a 64-bit target it cannot fail.

```rust
fn main() {
    let n: u32 = 64;
    let s = String::with_capacity(usize::try_from(n).expect("fits in usize"));
    println!("{}", s.capacity()); // 64
}
```

**Read:** [`TryFrom` and `TryInto`](../../29_Conversion/tryfrom_and_tryinto/README.md)

### 22. The capacity of a `&str`

```rust,compile_fail
fn main() {
    let s = "Hello";
    println!("{}", s.capacity());
}
```

```text title="rustc 1.98.0 on str_capacity.rs"
error[E0599]: no method named `capacity` found for reference `&str` in the current scope
 --> str_capacity.rs:3:22
  |
3 |     println!("{}", s.capacity());
  |                      ^^^^^^^^ method not found in `&str`
```

**The mistake.** A `&str` borrows text that something else owns. It is a pointer and a length, with no buffer of its own, so it has no capacity. Only the owner has one.

**The fix.** Ask the owner, or ask the `&str` for its length.

```rust
fn main() {
    let owned = String::from("Hello");
    println!("{}", owned.capacity()); // 5
    println!("{}", "Hello".len()); // 5
}
```

**Read:** [`String` vs `&str`](../string_vs_str/README.md)

## Setting the length by hand

### 23. `set_len` on a `String`

```rust,compile_fail
fn main() {
    let mut s = String::from("Hello");
    s.set_len(3);
    println!("{s}");
}
```

```text title="rustc 1.98.0 on string_set_len.rs"
error[E0599]: no method named `set_len` found for struct `String` in the current scope
 --> string_set_len.rs:3:7
  |
3 |     s.set_len(3);
  |       ^^^^^^^ method not found in `String`
```

**The mistake.** `Vec` has an unsafe `set_len`. `String` does not, because a length set by hand can end inside a character.

**The fix.** `truncate` shortens the text, and panics rather than cut a character in half. The buffer is kept.

```rust
fn main() {
    let mut s = String::from("Hello");
    s.truncate(3);
    println!("{s} {} {}", s.len(), s.capacity()); // Hel 3 5
}
```

**Read:** [`String::truncate`](../string_methods/string_truncate/README.md)

### 24. `as_mut_vec().set_len(…)` without `unsafe`

```rust,compile_fail
fn main() {
    let mut s = String::from("zażółć");
    s.as_mut_vec().set_len(3);
    println!("{s}");
}
```

```text title="rustc 1.98.0 on set_len_without_unsafe.rs"
error[E0133]: call to unsafe function `Vec::<T, A>::set_len` is unsafe and requires unsafe block
 --> set_len_without_unsafe.rs:3:5
  |
3 |     s.as_mut_vec().set_len(3);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function `String::as_mut_vec` is unsafe and requires unsafe block
 --> set_len_without_unsafe.rs:3:5
  |
3 |     s.as_mut_vec().set_len(3);
  |     ^^^^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior
```

**The mistake.** Both calls are `unsafe`, and rustc reports each. `as_mut_vec` lets you write bytes that are not UTF-8. `set_len` trusts you that the new length is right. Here it is not: byte 3 of `"zażółć"` is the middle of `ż`, so the `String` would stop being UTF-8.

**The fix.** Find a character boundary at or below the byte count, then `truncate` there. No `unsafe` is needed.

```rust
fn main() {
    let mut s = String::from("zażółć");
    let cut = s.floor_char_boundary(3);
    s.truncate(cut);
    println!("{s} {cut}"); // za 2
}
```

**Read:** [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md), [`str::floor_char_boundary`](../str_methods/str_floor_char_boundary/README.md)

## If you are coming from another language

**C and C++.** `sizeof` takes an expression or a type, so `sizeof buf` and `sizeof(char *)` are both C. Rust splits the two jobs: `size_of::<T>()` for a type and `size_of_val(&v)` for a value, and mixing them up is entries [1](#1-passing-a-value-to-size_of) to [3](#3-a-string-handed-to-size_of_val-by-value). A C++ `std::string` has both `size()` and `length()`, and Rust calls both `len()` ([8](#8-size-or-length)). `reserve` returns `void` in C++ too, and `auto cap = s.reserve(10);` is refused there as well ([19](#19-keeping-what-reserve-returns)).

What changes: C99 accepts `char buf[n]` with `n` from `strlen`, and puts the array on the stack. Rust refuses a run-time array length ([12](#12-an-array-as-long-as-the-text)), and the buffer goes on the heap in a `Vec` instead. [A string is bytes up to a NUL ↗](https://masiarek.github.io/c-learning-library/03_Strings/a_string_is_bytes_up_to_a_nul/) runs `sizeof` against `strlen` in C.

**Java.** `String.length()` and `List.size()` are both `len()` here ([8](#8-size-or-length)). `new StringBuilder(64)` and `new ArrayList<>(64)` are `String::with_capacity(64)` ([16](#16-stringnew64)). `StringBuilder.capacity()` only reads, like Rust's ([15](#15-capacity64-to-buy-room)). `ArrayList` has no `capacity()` method.

What changes: Java's `length()` counts UTF-16 units, and Rust's `len()` counts UTF-8 bytes. Neither counts letters, which is [10](#10-charslen-to-count-letters) from both sides. [Length is three different numbers ↗](https://masiarek.github.io/java-text-learning-library/01_Char_and_String/length_is_three_numbers/) is the Java half, and [Capacity, and the right tool ↗](https://masiarek.github.io/java-text-learning-library/06_Performance/capacity_and_the_right_tool/) is `StringBuilder`'s growth.

**Python.** `len(s)` is a built-in function there, and here it is a method, `s.len()`. [7](#7-slen-without-the-call) is what the method looks like without its `()`. It also counts something else: code points, not bytes, which [Counting characters ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/counting_characters/index.html) runs. `sys.getsizeof(s)` takes a value, and Rust's value-taking measure, `size_of_val`, borrows it ([3](#3-a-string-handed-to-size_of_val-by-value)). A Python `str` has no capacity you can read or buy, so entries 15 to 22 have no Python counterpart.

## See also

- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — the three numbers, measured, and a kata that predicts them for five strings
- [Stack and heap](../../18_Ownership/stack_and_heap/README.md) — what `size_of` and `size_of_val` can and cannot see
- [`str` is unsized](../str_is_unsized/README.md) — why `size_of::<str>()` has no answer
- [Four lengths, and which one the other system means](../four_lengths/README.md) — `len()`, `chars().count()` and two counts std does not give you
- [Building a `String`](../building_a_string/README.md) — `push_str`, `truncate`, and the operator that eats its left operand
- [Every array error, and its fix](../../26_Collections/arrays/array_errors/README.md) — the same kind of page for arrays; its `E0435` entry is [12](#12-an-array-as-long-as-the-text) with a `let n`
- [The error-code map](../../ERRORS.md) — every code the library teaches, and the lesson for each
- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order

## Po polsku

Dwadzieścia cztery błędy kompilacji, które spotkasz, mierząc `String`: rozmiar (`size_of`, `size_of_val`), długość (`len`) i pojemność (`capacity`). Przy każdym jest kod, który go wywołuje, dokładny komunikat rustc 1.98.0, opis pomyłki i poprawka, która się kompiluje.

Trzy rzeczy tłumaczą większość z nich. `size_of` bierze **typ**, a nie wartość: `size_of::<String>()`, a dla konkretnej wartości `size_of_val(&s)`. `len()` liczy **bajty**, nie litery: „zażółć” ma sześć liter i dziesięć bajtów, a `s[s.len() - 1]` się nie kompiluje. `capacity()` tylko **odczytuje** pojemność, a kupują ją `String::with_capacity(n)` i `reserve(n)`.

Uważaj na podpowiedzi `help:`. W dziewięciu przypadkach prowadzą w złą stronę: usuwają argument, który był zamiarem, proponują `usize::MAX` albo metodę `le`, której nazwa jest po prostu podobna.

**Szukaj po polsku:** rozmiar a długość a pojemność w Ruscie · `rust size_of vs len vs capacity` · `rust E0599 no method named size String` · `rust chars len count`
