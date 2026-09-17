# Lints around arrays: bad and good, run

[Arrays: the map](../README.md) › **Beside the lessons** · the errors, not the warnings: [Every array error, and its fix](../array_errors/README.md)

**Level:** 101 → 201 · a reference, by lint

**One line:** Twenty-four warnings from rustc and clippy that fire on array code: literal forms left unused, index loops, hand-written copies and fills, a `vec!` that could have been an array, `into_iter` across editions, and arrays too big for a stack or a `const`. Each has a program that triggers it, what the tool prints, a version that is silent, and when the lint is right. At the end are the mistakes no lint catches.

Every transcript was recorded on rustc 1.98.0 and clippy 0.1.98 by running the command in its title. The lines dropped from each are the documentation link, the `#[warn(...)]` or "requested on the command line" note, and the closing warning count. Both programs of every pair are compiled on every build by [`check_fences.py`](../../../tools/check_fences.py), on edition 2024. What the build does not re-run is clippy, so a later clippy can reword a warning.

The deny-by-default rustc lint that stops a constant index past the end, `unconditional_panic`, is an error, so it is on the errors page: [a constant index past the end](../array_errors/README.md#10-a-constant-index-past-the-end).

## Turning them on

rustc's lints and clippy's default groups need nothing: `cargo clippy` prints them. The opt-in ones take a flag or a `Cargo.toml` entry:

```sh
cargo clippy -- -W clippy::large_stack_arrays -W clippy::explicit_iter_loop
```

```toml
[lints.clippy]
large_stack_arrays = "warn"
explicit_iter_loop = "warn"
```

## Find the lint

| Lint | From | On by default? | Fires on |
|---|---|---|---|
| [`unused_variables`](#unused_variables) | rustc | yes | array bindings that are never read |
| [`unused_parens`](#unused_parens) | rustc | yes | `let range = (1..6);` and `for (n) in range` |
| [`unused_must_use`](#unused_must_use) | rustc | yes | a `filter` whose result is thrown away |
| [`array_into_iter`](#array_into_iter) | rustc | yes | `array.into_iter()` on edition 2015 or 2018 |
| [`needless_range_loop`](#needless_range_loop) | clippy · style | yes | `for i in 0..a.len()` where `i` only indexes `a` |
| [`manual_memcpy`](#manual_memcpy) | clippy · perf | yes | a loop copying `src[i]` into `dst[i]` |
| [`manual_slice_fill`](#manual_slice_fill) | clippy · style | yes | a loop writing one value into every element |
| [`useless_vec`](#useless_vec) | clippy · perf | yes | `&vec![1, 2, 3]` passed where a slice will do |
| [`get_first`](#get_first) | clippy · style | yes | `xs.get(0)` |
| [`manual_contains`](#manual_contains) | clippy · perf | yes | `xs.iter().any(\|&p\| p == 5)` |
| [`into_iter_on_ref`](#into_iter_on_ref) | clippy · style | yes | `(&xs).into_iter()` |
| [`explicit_counter_loop`](#explicit_counter_loop) | clippy · complexity | yes | a counter incremented by hand in a `for` |
| [`same_item_push`](#same_item_push) | clippy · style | yes | `push(0)` in a counted loop |
| [`needless_late_init`](#needless_late_init) | clippy · style | yes | `let a1: [i64; 5];` then `a1 = […];` on the next line |
| [`zero_repeat_side_effects`](#zero_repeat_side_effects) | clippy · suspicious | yes | a call with side effects in `[call(); 0]` |
| [`large_const_arrays`](#large_const_arrays) | clippy · perf | yes | a `const` array over 16,384 bytes |
| [`useless_conversion`](#useless_conversion) | clippy · complexity | yes | `(1..6).into_iter()` |
| [`out_of_bounds_indexing`](#out_of_bounds_indexing) | clippy · correctness | yes, as an error | `&xs[..5]` on a three-element array |
| [`explicit_iter_loop`](#explicit_iter_loop) | clippy · pedantic | no | `for x in xs.iter()` |
| [`explicit_into_iter_loop`](#explicit_into_iter_loop) | clippy · pedantic | no | `for x in xs.into_iter()` |
| [`large_stack_arrays`](#large_stack_arrays) | clippy · pedantic | no | a local array over 16,384 bytes |
| [`large_types_passed_by_value`](#large_types_passed_by_value) | clippy · pedantic | no | an array over 256 bytes taken by value |
| [`range_plus_one`](#range_plus_one) | clippy · pedantic | no | `0..xs.len() + 1` |
| [`indexing_slicing`](#indexing_slicing) | clippy · restriction | no | every `xs[i]` and `&xs[a..b]` |

## rustc, on by default

### `unused_variables`

**rustc** · warn by default · fires on array bindings that are never read

```rust
fn main() {
    let scores: [i32; 5] = [90, 72, 85, 64, 88];
    let flags = [true, false, true];
    let letters = ['c'; 3];
    println!("{}", scores.len());
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: unused variable: `flags`
 --> bad.rs:3:9
  |
3 |     let flags = [true, false, true];
  |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_flags`
  |

warning: unused variable: `letters`
 --> bad.rs:4:9
  |
4 |     let letters = ['c'; 3];
  |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_letters`
```

**Silent:**

```rust
fn main() {
    let scores: [i32; 5] = [90, 72, 85, 64, 88];
    let flags = [true, false, true];
    let _letters = ['c'; 3];
    println!("{} {:?}", scores.len(), flags);
}
```

**When it is right.** Right almost every time. In a page that lists literal forms side by side, use each binding or name it `_letters`; the underscore says the value is deliberately unused.

### `unused_parens`

**rustc** · warn by default · fires on `let range = (1..6);` and `for (n) in range`

```rust
fn main() {
    let range = (1..6);
    for (n) in range {
        print!("{n} ");
    }
    println!();
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: unnecessary parentheses around assigned value
 --> bad.rs:2:17
  |
2 |     let range = (1..6);
  |                 ^    ^
  |
help: remove these parentheses
  |
2 -     let range = (1..6);
2 +     let range = 1..6;
  |

warning: unnecessary parentheses around pattern
 --> bad.rs:3:9
  |
3 |     for (n) in range {
  |         ^ ^
  |
help: remove these parentheses
  |
3 -     for (n) in range {
3 +     for n in range {
  |
```

**Silent:**

```rust
fn main() {
    let range = 1..6;
    for n in range {
        print!("{n} ");
    }
    println!();
}
```

**When it is right.** Always right: parentheses around a whole assigned value or a pattern do nothing. They are needed, and the lint stays quiet, where a method is called on a range: `(1..6).filter(…)`.

### `unused_must_use`

**rustc** · warn by default · fires on a `filter` whose result is thrown away

```rust
fn main() {
    let numbers = [1, 2, 3, 2];
    numbers.into_iter().filter(|&n| n == 2);
    println!("{numbers:?}");
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: unused `Filter` that must be used
 --> bad.rs:3:5
  |
3 |     numbers.into_iter().filter(|&n| n == 2);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
help: use `let _ = ...` to ignore the resulting value
  |
3 |     let _ = numbers.into_iter().filter(|&n| n == 2);
  |     +++++++
```

**Silent:**

```rust
fn main() {
    let numbers = [1, 2, 3, 2];
    let twos: Vec<i32> = numbers.into_iter().filter(|&n| n == 2).collect();
    println!("{numbers:?} {twos:?}");
}
```

**When it is right.** Always right for an iterator adapter. `filter` builds an iterator and runs nothing, so the statement has no effect. Collect it, loop over it, or delete it.

### `array_into_iter`

**rustc** · warn by default · fires on `array.into_iter()` on edition 2015 or 2018

```rust
fn main() {
    let numbers = [1, 2, 3];
    for n in numbers.into_iter() {
        println!("{}", n);
    }
}
```

```text title="rustc --edition 2018 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to `<[T; N] as IntoIterator>::into_iter` in Rust 2021
 --> bad.rs:3:22
  |
3 |     for n in numbers.into_iter() {
  |                      ^^^^^^^^^
  |
  = warning: this changes meaning in Rust 2021
  = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
  |
3 -     for n in numbers.into_iter() {
3 +     for n in numbers.iter() {
  |
help: or remove `.into_iter()` to iterate by value
  |
3 -     for n in numbers.into_iter() {
3 +     for n in numbers {
  |
```

**Silent:**

```rust
fn main() {
    let numbers = [1, 2, 3];
    for n in numbers.iter() {
        println!("{}", n);
    }
    for n in IntoIterator::into_iter(numbers) {
        println!("{}", n);
    }
}
```

**When it is right.** On edition 2015 or 2018 it always is, because the line means different things in different editions. Arrays have implemented `IntoIterator` by value since Rust 1.53, in every edition. The older editions keep one piece of backward compatibility: the method-call spelling `array.into_iter()` still resolves to `(&array).into_iter()` and yields references. On edition 2021 and later, the same line yields values. The [Edition Guide ↗](https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html) has the history, and [`iter`, `iter_mut` and `into_iter`](../../../24_Iterators/iter_iter_mut_into_iter/README.md#arrays-are-the-exception-worth-memorising) has the headline.

Checked on edition 2018:

- `for e in [1, 2, 3]` yields `i32` values, as it does in every edition since 1.53.
- `IntoIterator::into_iter(array)`, the fully qualified call the Edition Guide uses, yields values.
- `#[allow(array_into_iter)]` silences the warning but not the meaning: `array.into_iter().filter(|&x| *x == 2)` compiles on 2018 because the items are `&i32`. On 2021 the same line is [`E0614`](../array_errors/README.md#22-x-in-a-filter-over-into_iter).

The same call can compile on one edition and not the other:

```rust
fn main() {
    let v = [1, 2].into_iter().collect::<Vec<i32>>();
    println!("{:?}", v); // [1, 2] on edition 2021 and later
}
```

```text title="rustc --edition 2018 collect_2018.rs — rustc 1.98.0"
error[E0277]: a value of type `Vec<i32>` cannot be built from an iterator over elements of type `&{integer}`
 --> collect_2018.rs:2:42
  |
2 |     let v = [1, 2].into_iter().collect::<Vec<i32>>();
  |                                -------   ^^^^^^^^ value of type `Vec<i32>` cannot be built from `std::iter::Iterator<Item=&{integer}>`
  |                                |
  |                                required by a bound introduced by this call
  |
help: the trait `FromIterator<&{integer}>` is not implemented for `Vec<i32>`
      but trait `FromIterator<i32>` is implemented for it
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/vec/mod.rs:3938:0
  = help: for that trait implementation, expected `i32`, found `&{integer}`
note: the method call chain might not have had the expected associated types
 --> collect_2018.rs:2:20
  |
2 |     let v = [1, 2].into_iter().collect::<Vec<i32>>();
  |             ------ ^^^^^^^^^^^ `Iterator::Item` is `&{integer}` here
  |             |
  |             this expression has type `[{integer}; 2]`
note: required by a bound in `collect`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/iter/traits/iterator.rs:2077:4
```


## Clippy, on by default

### `needless_range_loop`

**clippy · style** · warn by default · fires on `for i in 0..a.len()` where `i` only indexes `a`

```rust
fn main() {
    let row = [1u8, 2, 3];
    let mut sum = 0;
    for i in 0..row.len() {
        sum += row[i];
    }
    println!("{sum}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: the loop variable `i` is only used to index `row`
 --> bad.rs:4:14
  |
4 |     for i in 0..row.len() {
  |              ^^^^^^^^^^^^
  |
help: consider using an iterator
  |
4 -     for i in 0..row.len() {
4 +     for <item> in &row {
  |
```

**Silent:**

```rust
fn main() {
    let row = [1u8, 2, 3];
    let mut sum = 0;
    for n in &row {
        sum += n;
    }
    println!("{sum}");
}
```

**When it is right.** Right when the index only reads one array, as in the [first sum most people write](../writing_an_array_down/README.md#the-sum-nobody-typed-is-a-u8). Use `.iter().enumerate()` when you also need `i`. It stays quiet when `i` indexes two arrays at once, as in `transposed[j][i] = matrix[i][j]`: it cannot suggest one iterator for two arrays.

### `manual_memcpy`

**clippy · perf** · warn by default · fires on a loop copying `src[i]` into `dst[i]`

```rust
fn main() {
    let src = [1, 2, 3, 4];
    let mut dst = [0; 4];
    for i in 0..4 {
        dst[i] = src[i];
    }
    println!("{dst:?}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: it looks like you're manually copying between slices
 --> bad.rs:4:5
  |
4 | /     for i in 0..4 {
5 | |         dst[i] = src[i];
6 | |     }
  | |_____^ help: try replacing the loop by: `dst.copy_from_slice(&src);`
  |
```

**Silent:**

```rust
fn main() {
    let src = [1, 2, 3, 4];
    let mut dst = [0; 4];
    dst.copy_from_slice(&src);
    println!("{dst:?}");
}
```

**When it is right.** Right. `copy_from_slice` copies in one call and checks that the lengths are equal: with 4 elements into 3 it panics, saying *"source slice length (4) does not match destination slice length (3)"*.

### `manual_slice_fill`

**clippy · style** · warn by default · fires on a loop writing one value into every element

```rust
fn main() {
    let mut buf = [7u8; 4];
    for b in &mut buf {
        *b = 0;
    }
    println!("{buf:?}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: manually filling a slice
 --> bad.rs:3:5
  |
3 | /     for b in &mut buf {
4 | |         *b = 0;
5 | |     }
  | |_____^ help: try: `buf.fill(0);`
  |
```

**Silent:**

```rust
fn main() {
    let mut buf = [7u8; 4];
    buf.fill(0);
    println!("{buf:?}");
}
```

**When it is right.** Right when every element gets the same value. The index form of the same loop, `for i in 0..buf.len() { buf[i] = 0; }`, also trips `needless_range_loop`.

### `useless_vec`

**clippy · perf** · warn by default · fires on `&vec![1, 2, 3]` passed where a slice will do

```rust
fn total(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

fn main() {
    println!("{}", total(&vec![1, 2, 3]));
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: useless use of `vec!`
 --> bad.rs:6:26
  |
6 |     println!("{}", total(&vec![1, 2, 3]));
  |                          ^^^^^^^^^^^^^^ help: you can use a slice directly: `&[1, 2, 3]`
  |
```

**Silent:**

```rust
fn total(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

fn main() {
    println!("{}", total(&[1, 2, 3]));
}
```

**When it is right.** Right: the `Vec` allocates, and nothing uses what makes it a `Vec`. It fired on the temporary `&vec![…]`. The same `vec!` bound with `let xs` first and passed as `&xs` was silent on 0.1.98.

### `get_first`

**clippy · style** · warn by default · fires on `xs.get(0)`

```rust
fn main() {
    let xs = [10, 20, 30];
    println!("{:?}", xs.get(0));
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: accessing first element with `xs.get(0)`
 --> bad.rs:3:22
  |
3 |     println!("{:?}", xs.get(0));
  |                      ^^^^^^^^^ help: try: `xs.first()`
  |
```

**Silent:**

```rust
fn main() {
    let xs = [10, 20, 30];
    println!("{:?}", xs.first());
}
```

**When it is right.** A style lint: `first()` says what you mean, and both return `Option<&T>`.

### `manual_contains`

**clippy · perf** · warn by default · fires on `xs.iter().any(|&p| p == 5)`

```rust
fn main() {
    let primes = [2, 3, 5, 7];
    println!("{}", primes.iter().any(|&p| p == 5));
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: using `contains()` instead of `iter().any()` is more efficient
 --> bad.rs:3:20
  |
3 |     println!("{}", primes.iter().any(|&p| p == 5));
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `primes.contains(&5)`
  |
```

**Silent:**

```rust
fn main() {
    let primes = [2, 3, 5, 7];
    println!("{}", primes.contains(&5));
}
```

**When it is right.** Right for plain equality. Keep `any` when the test is anything else.

### `into_iter_on_ref`

**clippy · style** · warn by default · fires on `(&xs).into_iter()`

```rust
fn main() {
    let xs = [10, 20, 30];
    let doubled: Vec<i32> = (&xs).into_iter().map(|n| n * 2).collect();
    println!("{doubled:?}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: this `.into_iter()` call is equivalent to `.iter()` and will not consume the `array`
 --> bad.rs:3:35
  |
3 |     let doubled: Vec<i32> = (&xs).into_iter().map(|n| n * 2).collect();
  |                                   ^^^^^^^^^ help: call directly: `iter`
  |
```

**Silent:**

```rust
fn main() {
    let xs = [10, 20, 30];
    let doubled: Vec<i32> = xs.iter().map(|n| n * 2).collect();
    println!("{doubled:?}");
}
```

**When it is right.** Right: on a reference, `into_iter()` is `iter()` under a name that suggests the array is consumed, and it is not.

### `explicit_counter_loop`

**clippy · complexity** · warn by default · fires on a counter incremented by hand in a `for`

```rust
fn main() {
    let names = ["ada", "grace", "linus"];
    let mut i = 0;
    for name in names {
        println!("{i}: {name}");
        i += 1;
    }
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: the variable `i` is used as a loop counter
 --> bad.rs:4:5
  |
4 |     for name in names {
  |     ^^^^^^^^^^^^^^^^^ help: consider using: `for (i, name) in names.into_iter().enumerate()`
  |
```

**Silent:**

```rust
fn main() {
    let names = ["ada", "grace", "linus"];
    for (i, name) in names.iter().enumerate() {
        println!("{i}: {name}");
    }
}
```

**When it is right.** Right. `enumerate` cannot drift from the loop the way a hand-kept counter can, for example after a `continue`.

### `same_item_push`

**clippy · style** · warn by default · fires on `push(0)` in a counted loop

```rust
fn main() {
    let mut counts = Vec::new();
    for _ in 0..6 {
        counts.push(0);
    }
    println!("{counts:?}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: it looks like the same item is being pushed into this `Vec`
 --> bad.rs:4:9
  |
4 |         counts.push(0);
  |         ^^^^^^
  |
  = help: consider using `vec![0;SIZE]`
  = help: or `counts.extend(std::iter::repeat_n(0, SIZE))`
```

**Silent:**

```rust
fn main() {
    let counts = vec![0; 6];
    println!("{counts:?}");
}
```

**When it is right.** Right when the count is known. The suggestion `vec![0; SIZE]` is the `Vec` form of [the repeat expression](../writing_an_array_down/README.md#value-how-many).

### `needless_late_init`

**clippy · style** · warn by default · fires on `let a1: [i64; 5];` then `a1 = […];` on the next line

```rust
fn main() {
    let a1: [i64; 5];
    a1 = [100, 101, 102, 103, 104];
    println!("{a1:?}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: unneeded late initialization
 --> bad.rs:2:5
  |
2 |     let a1: [i64; 5];
  |     ^^^^^^^^^^^^^^^^^ created here
3 |     a1 = [100, 101, 102, 103, 104];
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ initialised here
  |
help: move the declaration `a1` here
  |
2 ~     
3 ~     let a1: [i64; 5] = [100, 101, 102, 103, 104];
  |
```

**Silent:**

```rust
fn main() {
    let a1: [i64; 5] = [100, 101, 102, 103, 104];
    println!("{a1:?}");
}
```

**When it is right.** Right when nothing happens between the two lines, as here. [Writing an array down](../writing_an_array_down/README.md#declare-now-assign-later) shows the late form only to say that it is legal.

### `zero_repeat_side_effects`

**clippy · suspicious** · warn by default · fires on a call with side effects in `[call(); 0]`

```rust
fn next_id(counter: &mut u32) -> u32 {
    *counter += 1;
    *counter
}

fn main() {
    let mut counter = 0;
    let ids = [next_id(&mut counter); 0];
    println!("{ids:?} {counter}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: expression with side effects as the initial value in a zero-sized array initializer
 --> bad.rs:8:5
  |
8 |     let ids = [next_id(&mut counter); 0];
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: consider performing the side effect separately
  |
8 ~     next_id(&mut counter);
9 +     let ids: [u32; 0] = [];
  |
```

**Silent:**

```rust
fn next_id(counter: &mut u32) -> u32 {
    *counter += 1;
    *counter
}

fn main() {
    let mut counter = 0;
    next_id(&mut counter);
    let ids: [u32; 0] = [];
    println!("{ids:?} {counter}");
}
```

**When it is right.** Right. The call runs and its result is dropped straight away, so the array is empty and the side effect happened anyway. The claims page [shows both](../array_claims_checked/README.md).

### `large_const_arrays`

**clippy · perf** · warn by default · fires on a `const` array over 16,384 bytes

```rust
const TABLE: [u32; 65_536] = [0; 65_536];

fn main() {
    println!("{}", TABLE[1] + TABLE[2]);
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: large array defined as const
 --> bad.rs:1:1
  |
1 | const TABLE: [u32; 65_536] = [0; 65_536];
  | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  | |
  | help: make this a static item: `static`
  |
```

**Silent:**

```rust
static TABLE: [u32; 65_536] = [0; 65_536];

fn main() {
    println!("{}", TABLE[1] + TABLE[2]);
}
```

**When it is right.** Right for big tables. A `const` is a value substituted where it is named, while a `static` is one place in the binary; [`const` and `static`](../../../27_Modules/const_and_static/README.md) measures the difference. On 0.1.98 a 16,384-byte `const` was quiet and a 16,385-byte one warned.

### `useless_conversion`

**clippy · complexity** · warn by default · fires on `(1..6).into_iter()`

```rust
fn main() {
    let range = (1..6).into_iter();
    let evens: Vec<i32> = range.filter(|n| n % 2 == 0).collect();
    println!("{evens:?}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: useless conversion to the same type: `std::ops::Range<i32>`
 --> bad.rs:2:17
  |
2 |     let range = (1..6).into_iter();
  |                 ^^^^^^^^^^^^^^^^^^ help: consider removing `.into_iter()`: `(1..6)`
  |
```

**Silent:**

```rust
fn main() {
    let evens: Vec<i32> = (1..6).filter(|n| n % 2 == 0).collect();
    println!("{evens:?}");
}
```

**When it is right.** Right. A `Range` is already an iterator, so `into_iter()` gives back the same range.

### `out_of_bounds_indexing`

**clippy · correctness** · deny by default · fires on `&xs[..5]` on a three-element array

```rust
fn main() {
    let xs = [1, 2, 3];
    println!("{:?}", &xs[..5]);
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
error: range is out of bounds
 --> bad.rs:3:28
  |
3 |     println!("{:?}", &xs[..5]);
  |                            ^
  |
```

**Silent:**

```rust
fn main() {
    let xs = [1, 2, 3];
    println!("{:?}", xs.get(..5));
}
```

**When it is right.** Right, and it is an error rather than a warning. rustc alone compiles this line, and the program then panics with *"range end index 5 out of range for slice of length 3"*. The constant *index* `xs[5]` is rustc's own [`unconditional_panic`](../array_errors/README.md#10-a-constant-index-past-the-end).

## Clippy, opt-in

### `explicit_iter_loop`

**clippy · pedantic** · allow by default · fires on `for x in xs.iter()`

```rust
fn main() {
    let xs = [1, 2, 3];
    for x in xs.iter() {
        println!("{x}");
    }
}
```

```text title="clippy-driver --edition 2024 -W clippy::explicit_iter_loop bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: it is more concise to loop over references to containers instead of using explicit iteration methods
 --> bad.rs:3:14
  |
3 |     for x in xs.iter() {
  |              ^^^^^^^^^ help: to write this more concisely, try: `&xs`
  |
```

**Silent:**

```rust
fn main() {
    let xs = [1, 2, 3];
    for x in &xs {
        println!("{x}");
    }
}
```

**When it is right.** A matter of taste. `&xs` is shorter, while `.iter()` reads better in the middle of a chain. Pedantic, so it is off unless you ask for it.

### `explicit_into_iter_loop`

**clippy · pedantic** · allow by default · fires on `for x in xs.into_iter()`

```rust
fn main() {
    let xs = [1, 2, 3];
    for x in xs.into_iter() {
        println!("{x}");
    }
}
```

```text title="clippy-driver --edition 2024 -W clippy::explicit_into_iter_loop bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: it is more concise to loop over containers instead of using explicit iteration methods
 --> bad.rs:3:14
  |
3 |     for x in xs.into_iter() {
  |              ^^^^^^^^^^^^^^ help: to write this more concisely, try: `xs`
  |
```

**Silent:**

```rust
fn main() {
    let xs = [1, 2, 3];
    for x in xs {
        println!("{x}");
    }
}
```

**When it is right.** Also taste: `for x in xs` already calls `into_iter`.

### `large_stack_arrays`

**clippy · pedantic** · allow by default · fires on a local array over 16,384 bytes

```rust
fn main() {
    let buf = [0u8; 1_000_000];
    println!("{}", buf.len());
}
```

```text title="clippy-driver --edition 2024 -W clippy::large_stack_arrays bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: allocating a local array larger than 16384 bytes
 --> bad.rs:2:15
  |
2 |     let buf = [0u8; 1_000_000];
  |               ^^^^^^^^^^^^^^^^
  |
  = help: consider allocating on the heap with `vec![0u8; 1_000_000].into_boxed_slice()`
```

**Silent:**

```rust
fn main() {
    let buf = vec![0u8; 1_000_000];
    println!("{}", buf.len());
}
```

**When it is right.** Right wherever a thread may have a small stack: [a megabyte array aborts a 256 KiB thread](../where_an_array_lives/README.md#big-arrays-and-the-stack). On 0.1.98 it also flagged `Box::new([0u8; 1_000_000])`, which builds the array on the stack first in a debug build, and `vec![[0u8; 100_000]; 2]`. A 16,384-byte local was quiet.

### `large_types_passed_by_value`

**clippy · pedantic** · allow by default · fires on an array over 256 bytes taken by value

```rust
fn checksum(block: [u8; 4096]) -> u32 {
    block.iter().map(|&b| u32::from(b)).sum()
}

fn main() {
    println!("{}", checksum([1; 4096]));
}
```

```text title="clippy-driver --edition 2024 -W clippy::large_types_passed_by_value bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: this argument (4096 byte) is passed by value, but might be more efficient if passed by reference (limit: 256 byte)
 --> bad.rs:1:20
  |
1 | fn checksum(block: [u8; 4096]) -> u32 {
  |                    ^^^^^^^^^^ help: consider passing by reference instead: `&[u8; 4096]`
  |
```

**Silent:**

```rust
fn checksum(block: &[u8; 4096]) -> u32 {
    block.iter().map(|&b| u32::from(b)).sum()
}

fn main() {
    println!("{}", checksum(&[1; 4096]));
}
```

**When it is right.** Right for big arrays that are only read. Declared `pub`, the same function was silent: clippy's `avoid-breaking-exported-api` setting, on by default, keeps it from suggesting a change to a public signature. Set it to `false` in `clippy.toml` and the warning comes back.

### `range_plus_one`

**clippy · pedantic** · allow by default · fires on `0..xs.len() + 1`

```rust
fn main() {
    let xs = [1, 2, 3];
    for i in 0..xs.len() + 1 {
        println!("{i}: {:?}", xs.get(i));
    }
}
```

```text title="clippy-driver --edition 2024 -W clippy::range_plus_one bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: an inclusive range would be more readable
 --> bad.rs:3:14
  |
3 |     for i in 0..xs.len() + 1 {
  |              ^^^^^^^^^^^^^^^ help: use: `0..=xs.len()`
  |
```

**Silent:**

```rust
fn main() {
    let xs = [1, 2, 3];
    for (i, x) in xs.iter().enumerate() {
        println!("{i}: {x}");
    }
}
```

**When it is right.** It is about spelling, not bounds: the suggested `0..=xs.len()` visits the same out-of-range index. That is on purpose in [Rust by Example's `.get` loop](../array_claims_checked/README.md); anywhere else, the `+ 1` is usually the bug.

### `indexing_slicing`

**clippy · restriction** · allow by default · fires on every `xs[i]` and `&xs[a..b]`

```rust
fn nth(xs: &[i32], i: usize) -> i32 {
    xs[i]
}

fn main() {
    println!("{}", nth(&[1, 2, 3], 1));
}
```

```text title="clippy-driver --edition 2024 -W clippy::indexing_slicing bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: indexing may panic
 --> bad.rs:2:5
  |
2 |     xs[i]
  |     ^^^^^
  |
  = help: consider using `.get(n)` or `.get_mut(n)` instead
```

**Silent:**

```rust
fn nth(xs: &[i32], i: usize) -> Option<i32> {
    xs.get(i).copied()
}

fn main() {
    println!("{:?}", nth(&[1, 2, 3], 1));
}
```

**When it is right.** Right in code that must never panic, and noise everywhere else: most indexes are in range by construction. Restriction lints are meant to be turned on one at a time.
## What no lint catches

Each of these is silent under `cargo clippy`'s defaults. The first is also silent with `pedantic` and `nursery` turned on.

```rust
fn main() {
    let row = [200u8, 100, 0];
    let mut sum = 0;
    for n in &row {
        sum += n;
    }
    println!("{sum}");
}
```

`sum` is inferred as `u8` from the elements, so this panics in a debug build and wraps to 44 in a release build. Only the restriction lint `clippy::arithmetic_side_effects` points at the `+=`. [Writing an array down](../writing_an_array_down/README.md#the-sum-nobody-typed-is-a-u8).

```rust
fn total(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

fn main() {
    let xs = vec![1, 2, 3];
    println!("{}", total(&xs));
}
```

The same allocation [`useless_vec`](#useless_vec) catches in `total(&vec![1, 2, 3])` passes once the `vec!` has a name.

```rust
fn main() {
    let mut letters = ['x', 'c', 'z'];
    println!("{:?}", letters.reverse());
}
```

This prints `()` and reverses the array without a word from any tool: `reverse` works in place and returns nothing. [Arrays and slices](../../arrays_and_slices/README.md#practice) has it as a kata.

```rust
fn main() {
    let big = Box::new([0u8; 1_000_000]);
    println!("{}", big.len());
}
```

A debug build builds the array in the stack frame before moving it into the box, which [aborts a small thread](../where_an_array_lives/README.md#big-arrays-and-the-stack). Only the pedantic [`large_stack_arrays`](#large_stack_arrays) notices.

## See also

- [Every array error, and its fix](../array_errors/README.md) — what the compiler refuses, where this page is what it warns about
- [Arrays: the map](../README.md) — every array page, and which question each one answers
- [Strict clippy](../../../05_Tooling/strict_lints/README.md) — turning whole lint groups on for a project, and what that costs
- [Lints around `ToOwned`](../../../12_Traits/how_to_learn_to_owned/to_owned_lints/README.md) — the same kind of page for `clone` and `Cow`, including `iter_cloned_collect` and `ptr_arg`

## Po polsku

Dwadzieścia cztery ostrzeżenia rustc i clippy, które dotyczą kodu z tablicami. Dla każdego jest program, który je wywołuje, dokładny komunikat (rustc 1.98.0, clippy 0.1.98), wersja bez ostrzeżenia i opis, kiedy lint ma rację. Najczęściej spotkasz `needless_range_loop` (pętla `for i in 0..a.len()` tylko do indeksowania), `unused_variables` przy wypisywaniu kilku tablic dla przykładu, `unused_parens` przy `for (e) in …` i `unused_must_use`, gdy wynik `filter` nie jest użyty, bo iteratory są leniwe. Opcjonalny `large_stack_arrays` ostrzega przed tablicą większą niż 16 384 bajty na stosie.

Osobno opisano `array_into_iter`: w edycjach 2015 i 2018 wywołanie `array.into_iter()` zwraca referencje, a od edycji 2021 wartości. Dlatego ten sam kod może się kompilować w jednej edycji, a w innej nie. Żaden lint nie wyłapie natomiast przepełnienia sumy typu `u8` ani `println!("{:?}", a.reverse())`.

**Szukaj po polsku:** ostrzeżenia clippy tablice · `clippy needless_range_loop` · `rust array_into_iter edition 2021` · `clippy large_stack_arrays`
