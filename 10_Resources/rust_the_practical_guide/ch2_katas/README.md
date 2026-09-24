# *Rust: The Practical Guide*, chapter 2 exercises, run

[*Rust: The Practical Guide*, run](../README.md) › **Chapter 2 exercises** · the book: *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025), §2.5 Practice Exercises and §2.6 Solutions

**Level:** 101 · a companion to the book's chapter 2

**One line:** The twelve exercises in §2.5 are one compiler refusal each — a missing `let`, a missing `mut`, `-1` in a `u8`, a float in an `i32`, a tuple passed as two arguments — and every solution in §2.6 compiles on rustc 1.98.0. Three notes survive the run: the fix for 2.2 keeps a warning that `let x2;` avoids, the sentence under it says the opposite of what `#![allow(unused)]` does, and the solution for 2.8 quietly changes the year the exercise printed. Each exercise below is restated, compiled as given, and solved by a program whose output is the answer key.

## The verdict

| # | Exercise | What rustc says about the given code | The book's solution | Verdict | Lesson |
|---|---|---|---|---|---|
| 2.1 | Introduce the variable | `E0425` cannot find value `my_age`, and it suggests `let my_age = 40;` | `let my_age = 40;` | correct | [Variables](../../../15_First_Programs/variables/README.md) |
| 2.2 | Give `x2` permission | `E0384` cannot assign twice to immutable variable, plus a warning that the first value is never read | `let mut x2 = x1;` | compiles, keeps the warning; `let x2;` avoids it. The note says `#![allow(unused)]` is for when you *do* want the warning; it is the line that silences it | [Variables](../../../15_First_Programs/variables/README.md) · [What a warning is asking](../../../15_First_Programs/what_a_warning_is_asking/README.md) |
| 2.3 | Will it compile? | compiles, no warnings, prints `x1 is: 120, x2 is: 118` | "yes: an immutable variable can be assigned once" | correct | [Variables](../../../15_First_Programs/variables/README.md) · [`and`, `or` claims, run](../../../15_First_Programs/and_or_claims_checked/README.md) |
| 2.4 | Shadow `a` | `E0308` expected `&str`, found integer | a second `let a = 10;` | correct; as written it warns `unused variable: a` | [When to shadow](../../../18_Ownership/when_to_shadow/README.md) · [The shadowing map](../../../SHADOWING.md) |
| 2.5 | A value a `u8` can hold | `E0600` cannot apply unary operator `-` to type `u8`; it suggests `u8::MAX` | `x = 1;` | correct | [The integer types](../../../19_Numbers/the_integer_types/README.md) |
| 2.6 | A type for `3.14159` | `E0308` expected `i32`, found floating-point number | `let pi: f32;` | correct; `f64` works too, and is what an unannotated literal gets | [What a float stores](../../../19_Numbers/what_a_float_stores/README.md) · [Type inference](../../../15_First_Programs/type_inference/README.md) |
| 2.7 | Types for `-15`, `170`, `"Michael"` | `E0425` cannot find type `DATA_TYPES_PLEASE`, three times | `i16`, `i16`, `&str` | correct, and `i16` is the smallest that works: `i8` refuses `170`, `u8` refuses `-15`; `i32` works too | [The integer types](../../../19_Numbers/the_integer_types/README.md) · [`String` vs `&str`](../../../14_Strings/string_vs_str/README.md) |
| 2.8 | Name the tuple's type | a parse error: expected type, found keyword `let` | `type Book = (String, String, u32);` | correct; the solution also changes the year from 2010 to 2020 | [A type alias is not a new type](../../../16_Structs/type_aliases/README.md) · [Tuples](../../../26_Collections/tuples/README.md) |
| 2.9 | Three functions to fit a call | `E0425` cannot find function, three times | three one-line functions over `i32` | correct, prints 54 | [Functions](../../../25_Control_Flow/functions/README.md) |
| 2.10 | Drop the intermediate variables | compiles, prints `Answer: 90` | `triple(triple(double(5)))` | correct | [Functions](../../../25_Control_Flow/functions/README.md) |
| 2.11 | Pass the point as one argument | `E0061` function takes 1 argument but 2 arguments were supplied; it suggests the extra parentheses | `print_distance((5.0, 4.0))` | correct; the function prints nothing, and the exercise's message is garbled | [Tuples](../../../26_Collections/tuples/README.md) · [Functions](../../../25_Control_Flow/functions/README.md) |
| 2.12 | `quadruple` from `double` | `E0308` expected `i32`, found `()` | `double(double(x))` | correct | [Functions](../../../25_Control_Flow/functions/README.md) |

Every transcript on this page is rustc 1.98.0 compiling a file with the name in the fence title. Every solution is a `pg2_*` program compiled and run by `tools/run_examples.py`, with its output pasted in below the source.

## Practice

### 1. Introduce the variable

The program assigns `my_age` and prints it. The printing line must stay as it is; make the program compile.

```rust,compile_fail
fn main() {
    my_age = 40;
    println!("My age is: {}", my_age); // do not change this line
}
```

```text title="Abridged — real rustc output for no_let.rs, the second E0425 on the printing line dropped"
error[E0425]: cannot find value `my_age` in this scope
 --> no_let.rs:2:5
  |
2 |     my_age = 40;
  |     ^^^^^^
  |
help: you might have meant to introduce a new binding
  |
2 |     let my_age = 40;
  |     +++
```

The book's fix, `let my_age = 40;`, is the one rustc suggests. An assignment needs a binding that already exists, and `let` is what creates one — [Variables](../../../15_First_Programs/variables/README.md).

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_01_let -->
*[`pg2_01_let.rs`](examples/pg2_01_let.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.1: a name has to be introduced with `let` before it can be assigned.
fn main() {
    let my_age = 40;
    println!("My age is: {}", my_age); // My age is: 40
}
```
<!-- /source -->

<!-- output:pg2_01_let -->
*Verified output of [`pg2_01_let.rs`](examples/pg2_01_let.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
My age is: 40
```
<!-- /output -->

</details>

### 2. Give `x2` permission

The assignment `x2 = x1 - 2;` and the `println!` are fixed. Change what comes before them so the program compiles.

```rust,compile_fail
fn main() {
    let x1 = 40;
    let x2 = x1;
    x2 = x1 - 2; // do not change this
    println!("x1 is: {} and x2 is: {}", x1, x2); // do not change this
}
```

```text title="Abridged — real rustc output for x2_not_mut.rs"
error[E0384]: cannot assign twice to immutable variable `x2`
 --> x2_not_mut.rs:4:5
  |
3 |     let x2 = x1;
  |         -- first assignment to `x2`
4 |     x2 = x1 - 2; // do not change this
  |     ^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
3 |     let mut x2 = x1;
  |         +++

warning: value assigned to `x2` is never read
 --> x2_not_mut.rs:3:14
  |
3 |     let x2 = x1;
  |              ^^ this value is reassigned later and never used
```

The book's fix is the `mut` rustc suggests, and it compiles — with the warning, because the first value of `x2` is still overwritten before anything reads it:

```rust
fn main() {
    let x1 = 40;
    let mut x2 = x1;
    x2 = x1 - 2; // do not change this
    println!("x1 is: {} and x2 is: {}", x1, x2); // do not change this
}
```

```text title="Real rustc output for x2_mut.rs, which then prints x1 is: 40 and x2 is: 38"
warning: value assigned to `x2` is never read
 --> x2_mut.rs:3:18
  |
3 |     let mut x2 = x1;
  |                  ^^ this value is reassigned later and never used
4 |     x2 = x1 - 2; // do not change this
  |     ----------- `x2` is overwritten here before the previous value is read
  |
  = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: 1 warning emitted
```

Two things about the note the book prints under this solution. It says to add `#![allow(unused)]` above `main` "if you do want to see the warning". The line does the opposite: `unused_assignments` is part of the `unused` group, so allowing the group silences it — the same file with that line on top compiled with no output at all. `#[allow(unused_assignments)]` on the function is the narrower version, and [What a warning is asking](../../../15_First_Programs/what_a_warning_is_asking/README.md#the-four-levels-and-where-you-set-them) has the four levels. And the warning is avoidable within the rules of the exercise: declare `x2` without a value. Then `x2 = x1 - 2;` is its *first* assignment, which needs no `mut` and throws nothing away. The solution below compiles clean, and it is exercise 3's whole point.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_02_mut -->
*[`pg2_02_mut.rs`](examples/pg2_02_mut.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.2: the two fixed lines assign x2 and print it. Declaring x2 with no
// value makes that assignment the first one, so it needs neither `mut` nor a
// value that is thrown away.
fn main() {
    let x1 = 40;
    let x2;
    x2 = x1 - 2;
    println!("x1 is: {} and x2 is: {}", x1, x2); // x1 is: 40 and x2 is: 38
}
```
<!-- /source -->

<!-- output:pg2_02_mut -->
*Verified output of [`pg2_02_mut.rs`](examples/pg2_02_mut.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
x1 is: 40 and x2 is: 38
```
<!-- /output -->

</details>

### 3. Will it compile?

Without running it, say whether this compiles, and what it prints if it does.

```rust
fn main() {
    let mut x1 = 40;
    let x2;
    x1 = x1 * 3;
    x2 = x1 - 2;
    println!("x1 is: {}, x2 is: {}", x1, x2);
}
```

It compiles, without a warning, and prints `x1 is: 120, x2 is: 118`. The book's explanation is right: a binding without `mut` may be assigned once, and the `let` line need not be where that happens — block 4 of [`and`, `or` and a first program's explanation, run](../../../15_First_Programs/and_or_claims_checked/README.md#the-claims-run) is the same rule, run. A *second* assignment is what `E0384` refuses:

```text title="Abridged — real rustc output for x2_assigned_twice.rs, the same file with x2 = x1 - 3; added"
error[E0384]: cannot assign twice to immutable variable `x2`
 --> x2_assigned_twice.rs:6:5
  |
5 |     x2 = x1 - 2;
  |     ----------- first assignment to `x2`
6 |     x2 = x1 - 3;
  |     ^^^^^^^^^^^ cannot assign twice to immutable variable
```

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_03_assign_once -->
*[`pg2_03_assign_once.rs`](examples/pg2_03_assign_once.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.3: it compiles. A binding without `mut` may be assigned once, and
// the `let` line does not have to be where that happens.
fn main() {
    let mut x1 = 40;
    let x2;
    x1 = x1 * 3;
    x2 = x1 - 2;
    println!("x1 is: {}, x2 is: {}", x1, x2); // x1 is: 120, x2 is: 118
}
```
<!-- /source -->

<!-- output:pg2_03_assign_once -->
*Verified output of [`pg2_03_assign_once.rs`](examples/pg2_03_assign_once.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
x1 is: 120, x2 is: 118
```
<!-- /output -->

</details>

### 4. Shadow `a`

`a` must keep its name and its first line. Make the program print `a is: 10`.

```rust,compile_fail
fn main() {
    let a = "three"; // don't change this line
    a = 10; // don't change the name of this variable
    println!("a is: {}", a);
}
```

```text title="Abridged — real rustc output for a_is_str.rs"
error[E0308]: mismatched types
 --> a_is_str.rs:3:9
  |
2 |     let a = "three"; // don't change this line
  |             ------- expected due to this value
3 |     a = 10; // don't change the name of this variable
  |         ^^ expected `&str`, found integer
```

This is not exercise 2's `E0384`: `a` is a `&str`, and no `mut` lets an integer into it. The book's fix, a second `let a = 10;`, is a new variable that takes over the name — [When to shadow](../../../18_Ownership/when_to_shadow/README.md) has the idioms and [the shadowing map](../../../SHADOWING.md) the whole territory. As printed, the fix compiles with one warning, because the first `a` is never read before the second hides it:

```text title="Abridged — real rustc output for a_shadowed_unused.rs, the book's fix as written"
warning: unused variable: `a`
 --> a_shadowed_unused.rs:2:9
  |
2 |     let a = "three"; // don't change this line
  |         ^ help: if this is intentional, prefix it with an underscore: `_a`
```

The solution below reads the first `a`, and prints the type on both sides of the second `let`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_04_shadow -->
*[`pg2_04_shadow.rs`](examples/pg2_04_shadow.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.4: `a` holds a &str, so `a = 10` is a type error. A second `let`
// makes a new variable with the same name and its own type.
use std::any::type_name_of_val;

fn main() {
    let a = "three"; // don't change this line
    println!("before the second let, a is {:?}, a {}", a, type_name_of_val(&a)); // before the second let, a is "three", a &str
    let a = 10; // don't change the name of this variable
    println!("a is: {}", a); // a is: 10
    println!("after it, a is an {}", type_name_of_val(&a)); // after it, a is an i32
}
```
<!-- /source -->

<!-- output:pg2_04_shadow -->
*Verified output of [`pg2_04_shadow.rs`](examples/pg2_04_shadow.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
before the second let, a is "three", a &str
a is: 10
after it, a is an i32
```
<!-- /output -->

</details>

### 5. A value a `u8` can hold

The declaration `let x: u8;` is fixed. Assign a value the program accepts.

```rust,compile_fail
fn main() {
    let x: u8; // Don't change this line!
    x = -1;
    println!("x is: {}", x);
}
```

```text title="Abridged — real rustc output for minus_one_u8.rs"
error[E0600]: cannot apply unary operator `-` to type `u8`
 --> minus_one_u8.rs:3:9
  |
3 |     x = -1;
  |         ^^ cannot apply unary operator `-`
  |
  = note: unsigned values cannot be negated
help: you may have meant the maximum value of `u8`
  |
3 -     x = -1;
3 +     x = u8::MAX;
  |
```

The book assigns `1`. Anything in `0..=255` works, and rustc's own guess, `u8::MAX`, is what a C programmer writing `-1` usually meant: the solution prints the range, and the `255` that `-1i32 as u8` produces once the conversion is written down. [The integer types](../../../19_Numbers/the_integer_types/README.md) is the map of the twelve widths.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_05_u8 -->
*[`pg2_05_u8.rs`](examples/pg2_05_u8.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.5: a u8 holds 0..=255, so `-1` is refused before anything runs.
fn main() {
    let x: u8; // Don't change this line!
    x = 1;
    println!("x is: {}", x); // x is: 1
    println!("a u8 holds {}..={}", u8::MIN, u8::MAX); // a u8 holds 0..=255
    println!("the u8 that C stores for -1: {}", -1i32 as u8); // the u8 that C stores for -1: 255
}
```
<!-- /source -->

<!-- output:pg2_05_u8 -->
*Verified output of [`pg2_05_u8.rs`](examples/pg2_05_u8.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
x is: 1
a u8 holds 0..=255
the u8 that C stores for -1: 255
```
<!-- /output -->

</details>

### 6. A type for `3.14159`

Replace the type in `let pi: i32;` so that the assignment compiles.

```rust,compile_fail
fn main() {
    let pi: i32;
    pi = 3.14159; // This value represents pi
    println!("pi is: {}", pi);
}
```

```text title="Abridged — real rustc output for pi_as_i32.rs"
error[E0308]: mismatched types
 --> pi_as_i32.rs:3:10
  |
2 |     let pi: i32;
  |             --- expected due to this type
3 |     pi = 3.14159; // This value represents pi
  |          ^^^^^^^ expected `i32`, found floating-point number
```

`f32`, the book's answer, and `f64` both print `3.14159`. An integer type could keep only the `3`, and Rust makes you write that with `as` rather than doing it on assignment. With no annotation at all the literal is an `f64` — [the two fallbacks](../../../15_First_Programs/type_inference/README.md#the-two-fallbacks) — and [What a float stores](../../../19_Numbers/what_a_float_stores/README.md) is why neither width holds π, or even `3.14159`, exactly.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_06_float -->
*[`pg2_06_float.rs`](examples/pg2_06_float.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.6: 3.14159 is a floating-point literal, so the binding needs a
// float type. Either width holds it; an integer type keeps only the 3.
use std::any::type_name_of_val;

fn main() {
    let pi: f32;
    pi = 3.14159; // This value represents pi
    println!("pi is: {}", pi); // pi is: 3.14159
    let pi64: f64 = 3.14159;
    println!("as f64: {}", pi64); // as f64: 3.14159
    println!("what an i32 can keep of it: {}", pi64 as i32); // what an i32 can keep of it: 3
    let unannotated = 3.14159;
    println!("with no annotation the literal is {}", type_name_of_val(&unannotated)); // with no annotation the literal is f64
}
```
<!-- /source -->

<!-- output:pg2_06_float -->
*Verified output of [`pg2_06_float.rs`](examples/pg2_06_float.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
pi is: 3.14159
as f64: 3.14159
what an i32 can keep of it: 3
with no annotation the literal is f64
```
<!-- /output -->

</details>

### 7. Types for `-15`, `170` and `"Michael"`

Replace each `DATA_TYPES_PLEASE` with a type that lets the program compile and multiply.

```rust,compile_fail
fn main() {
    let a: DATA_TYPES_PLEASE = -15;
    let b: DATA_TYPES_PLEASE = 170;
    let name: DATA_TYPES_PLEASE = "Michael";
    println!("name is: {}, and the multiplication result is {}", name, a * b);
}
```

```text title="Abridged — real rustc output for data_types_please.rs, the same error for b and name dropped"
error[E0425]: cannot find type `DATA_TYPES_PLEASE` in this scope
 --> data_types_please.rs:2:12
  |
2 |     let a: DATA_TYPES_PLEASE = -15;
  |            ^^^^^^^^^^^^^^^^^ not found in this scope
```

The book picks `i16`, `i16` and `&str`, and the choice is tighter than it looks. `-15` rules out every unsigned type (`u8` is exercise 5's `E0600` again), `170` is past `i8::MAX`, and the product `-2550` is past eight bits in either direction — `i16` is the smallest type that holds all three, and `i32`, the default, works too. Ask for `i8` and rustc refuses twice, once for the literal and once for the product it can already see overflowing:

```text title="Abridged — real rustc output for i8_literals.rs"
error: this arithmetic operation will overflow
 --> i8_literals.rs:5:72
  |
5 |     println!("name is: {}, and the multiplication result is {}", name, a * b);
  |                                                                        ^^^^^ attempt to compute `-15_i8 * -86_i8`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default

error: literal out of range for `i8`
 --> i8_literals.rs:3:17
  |
3 |     let b: i8 = 170;
  |                 ^^^
  |
  = note: the literal `170` does not fit into the type `i8` whose range is `-128..=127`
```

The `-86` in the first message is `170` wrapped into an `i8`. A string literal is a `&str`; `String` would need `String::from("Michael")` — [`String` vs `&str`](../../../14_Strings/string_vs_str/README.md).

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_07_types -->
*[`pg2_07_types.rs`](examples/pg2_07_types.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.7: -15 needs a signed type, 170 does not fit in 7 bits, and the
// product -2550 does not fit in 8. i16 is the smallest type that holds all three.
fn main() {
    let a: i16 = -15;
    let b: i16 = 170;
    let name: &str = "Michael";
    println!("name is: {}, and the multiplication result is {}", name, a * b); // name is: Michael, and the multiplication result is -2550
    println!("i16 holds {}..={}", i16::MIN, i16::MAX); // i16 holds -32768..=32767
    let (a, b): (i32, i32) = (-15, 170);
    println!("the same with i32: {}", a * b); // the same with i32: -2550
    let (a, b): (i8, i8) = (-15, 100);
    println!("i8 fits -15 but not 170; even -15 * 100 overflows it: {:?}", a.checked_mul(b)); // None
}
```
<!-- /source -->

<!-- output:pg2_07_types -->
*Verified output of [`pg2_07_types.rs`](examples/pg2_07_types.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
name is: Michael, and the multiplication result is -2550
i16 holds -32768..=32767
the same with i32: -2550
i8 fits -15 but not 170; even -15 * 100 overflows it: None
```
<!-- /output -->

</details>

### 8. Name the tuple's type

Complete `type Book = …` so that `book1` type-checks.

```rust,compile_fail
fn main() {
    type Book = // Add your code here
    let book1: Book = (
        String::from("Rust Programming Language"),
        String::from("RUST Community"),
        2010,
    );
    println!("Book name: {}, Author: {}, Year {}", book1.0, book1.1, book1.2);
}
```

```text title="Abridged — real rustc output for alias_missing.rs"
error: expected type, found keyword `let`
 --> alias_missing.rs:3:5
  |
3 |     let book1: Book = (
  |     ^^^ expected type
```

`type Book = (String, String, u32);` is the book's answer and the right one; `type` is an item, so it is legal inside `main`, where the exercise put it. Two notes. The book's solution also changes the year to `2020`, so what it prints is not what the exercise's line prints. And an alias is a second name, not a second type: the solution prints `type_name::<Book>()` and gets the tuple back, then moves `book1` into a plain `(String, String, u32)` with no conversion — [A type alias is not a new type](../../../16_Structs/type_aliases/README.md), and [Tuples](../../../26_Collections/tuples/README.md) for `.0`, `.1` and `.2`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_08_alias -->
*[`pg2_08_alias.rs`](examples/pg2_08_alias.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.8: a type alias gives the tuple's shape a name. It is an item, so
// it may sit inside main, and it makes no new type: the name expands to the tuple.
use std::any::type_name;

fn main() {
    type Book = (String, String, u32);
    let book1: Book = (
        String::from("Rust Programming Language"),
        String::from("RUST Community"),
        2010,
    );
    println!("Book name: {}, Author: {}, Year {}", book1.0, book1.1, book1.2); // Book name: Rust Programming Language, Author: RUST Community, Year 2010
    println!("Book is {}", type_name::<Book>()); // Book is (alloc::string::String, alloc::string::String, u32)
    let plain: (String, String, u32) = book1; // no conversion: same type
    println!("year of the plain tuple: {}", plain.2); // year of the plain tuple: 2010
}
```
<!-- /source -->

<!-- output:pg2_08_alias -->
*Verified output of [`pg2_08_alias.rs`](examples/pg2_08_alias.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Book name: Rust Programming Language, Author: RUST Community, Year 2010
Book is (alloc::string::String, alloc::string::String, u32)
year of the plain tuple: 2010
```
<!-- /output -->

</details>

### 9. Three functions to fit a call

Write `add_3`, `add_5` and `times` so that this `main` compiles unchanged.

```rust,compile_fail
fn main() {
    let x = 3;
    let y = 4;
    println!("The result of x+3 times y+5 is {}", times(add_3(x), add_5(y)));
}
```

```text title="Abridged — real rustc output for add_3_missing.rs, one of three E0425"
error[E0425]: cannot find function `add_3` in this scope
 --> add_3_missing.rs:4:57
  |
4 |     println!("The result of x+3 times y+5 is {}", times(add_3(x), add_5(y)));
  |                                                         ^^^^^ not found in this scope
```

The call site fixes every signature: one `i32` in and one out for the two adders, two in for `times`. `(3 + 3) * (4 + 5)` is 54, and the book's three one-liners print it. A [signature is never inferred](../../../25_Control_Flow/functions/README.md#the-signature-is-never-inferred), so each parameter and return type is written out even though `main` would have determined them.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_09_arith -->
*[`pg2_09_arith.rs`](examples/pg2_09_arith.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.9: three functions with the signatures the call site asks for.
fn add_3(x: i32) -> i32 {
    x + 3
}

fn add_5(x: i32) -> i32 {
    x + 5
}

fn times(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let x = 3;
    let y = 4;
    println!("The result of x+3 times y+5 is {}", times(add_3(x), add_5(y))); // The result of x+3 times y+5 is 54
}
```
<!-- /source -->

<!-- output:pg2_09_arith -->
*Verified output of [`pg2_09_arith.rs`](examples/pg2_09_arith.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The result of x+3 times y+5 is 54
```
<!-- /output -->

</details>

### 10. Drop the intermediate variables

`main` computes its answer through two variables. Rewrite it with none.

```rust
fn double(x: i32) -> i32 {
    x * 2
}
fn triple(x: i32) -> i32 {
    x * 3
}
fn main() {
    let x = triple(double(5));
    let y = triple(x);
    println!("Answer: {}", y);
}
```

This one compiles as given; the exercise is a refactor. The book's `triple(triple(double(5)))` prints `Answer: 90` — 5 doubled is 10, tripled twice is 90 — and the innermost call runs first, because [arguments are evaluated before the call](../../../25_Control_Flow/functions/README.md#arguments-are-evaluated-first-left-to-right).

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_10_nested -->
*[`pg2_10_nested.rs`](examples/pg2_10_nested.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.10: the two intermediate variables become nested calls.
fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

fn main() {
    println!("Answer: {}", triple(triple(double(5)))); // Answer: 90
}
```
<!-- /source -->

<!-- output:pg2_10_nested -->
*Verified output of [`pg2_10_nested.rs`](examples/pg2_10_nested.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Answer: 90
```
<!-- /output -->

</details>

### 11. Pass the point as one argument

Only the call may change.

```rust,compile_fail
fn print_distance(point: (f32, f32)) -> f32 {
    let (x, y) = point;
    (x.powf(2.0) + y.powf(2.0)).sqrt() // Formula for computing distance
}
fn main() {
    println!(
        "The distance of the number the point from the origin is {}",
        print_distance(5.0, 4.0) // concentrate on the call to the function
    );
}
```

```text title="Abridged — real rustc output for distance_two_args.rs"
error[E0061]: function takes 1 argument but 2 arguments were supplied
 --> distance_two_args.rs:8:9
  |
8 |         print_distance(5.0, 4.0) // concentrate on the call to the function
  |         ^^^^^^^^^^^^^^
  |
note: function defined here
 --> distance_two_args.rs:1:4
  |
1 | fn print_distance(point: (f32, f32)) -> f32 {
  |    ^^^^^^^^^^^^^^ -----------------
help: wrap these arguments in parentheses to construct a tuple
  |
8 |         print_distance((5.0, 4.0)) // concentrate on the call to the function
  |                        +        +
```

The book's fix is the doubled parentheses rustc suggests, with the right reason: the function takes one tuple, not two floats. A [tuple](../../../26_Collections/tuples/README.md) is one value however many fields it has. Three smaller things. `√41` is `6.4031243` as an `f32` and `6.4031242374328485` as an `f64`, so the digits printed depend on the type the book chose. The function is named `print_distance` and prints nothing — it returns the distance, and `distance` would say so. And the exercise's format string, "The distance of the number the point from the origin", is garbled; the solution prints a repaired one, and adds `f32::hypot`, std's spelling of the formula.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_11_tuple -->
*[`pg2_11_tuple.rs`](examples/pg2_11_tuple.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.11: the function takes one argument, a tuple, so the call wraps
// the two numbers in a second pair of parentheses.
fn print_distance(point: (f32, f32)) -> f32 {
    let (x, y) = point;
    (x.powf(2.0) + y.powf(2.0)).sqrt() // Formula for computing distance
}

fn main() {
    println!("The distance of the point from the origin is {}", print_distance((5.0, 4.0))); // The distance of the point from the origin is 6.4031243
    println!("the same in f64: {}", (5.0f64.powi(2) + 4.0f64.powi(2)).sqrt()); // the same in f64: 6.4031242374328485
    println!("what std offers for it: {}", 5.0f32.hypot(4.0)); // what std offers for it: 6.4031243
}
```
<!-- /source -->

<!-- output:pg2_11_tuple -->
*Verified output of [`pg2_11_tuple.rs`](examples/pg2_11_tuple.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The distance of the point from the origin is 6.4031243
the same in f64: 6.4031242374328485
what std offers for it: 6.4031243
```
<!-- /output -->

</details>

### 12. `quadruple` from `double`

Fill in `quadruple` using `double`, so that the four checks in `main` print matching numbers.

```rust,compile_fail
fn double(x: i32) -> i32 {
    x * 2
}
fn quadruple(x: i32) -> i32 {
    // your code here //
}
fn main() {
    println!("For 1: the expected value is 4 while the output is {}", quadruple(1));
    println!("For 2: the expected value is 8 while the output is {}", quadruple(2));
    println!("For 3: the expected value is 12 while the output is {}", quadruple(3));
    println!("For 4: the expected value is 16 while the output is {}", quadruple(4));
}
```

```text title="Abridged — real rustc output for quadruple_empty.rs"
error[E0308]: mismatched types
 --> quadruple_empty.rs:4:25
  |
4 | fn quadruple(x: i32) -> i32 {
  |    ---------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
```

An empty body is worth `()`, and the signature promised an `i32` — [no `->` means `()`](../../../25_Control_Flow/functions/README.md#no-means-and-so-does-a-final) is the same error seen from the other side. `double(double(x))`, the book's answer, is the whole body.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg2_12_quadruple -->
*[`pg2_12_quadruple.rs`](examples/pg2_12_quadruple.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Exercise 2.12: quadrupling is doubling twice.
fn double(x: i32) -> i32 {
    x * 2
}

fn quadruple(x: i32) -> i32 {
    double(double(x))
}

fn main() {
    println!("For 1: the expected value is 4 while the output is {}", quadruple(1)); // For 1: the expected value is 4 while the output is 4
    println!("For 2: the expected value is 8 while the output is {}", quadruple(2)); // For 2: the expected value is 8 while the output is 8
    println!("For 3: the expected value is 12 while the output is {}", quadruple(3)); // For 3: the expected value is 12 while the output is 12
    println!("For 4: the expected value is 16 while the output is {}", quadruple(4)); // For 4: the expected value is 16 while the output is 16
}
```
<!-- /source -->

<!-- output:pg2_12_quadruple -->
*Verified output of [`pg2_12_quadruple.rs`](examples/pg2_12_quadruple.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
For 1: the expected value is 4 while the output is 4
For 2: the expected value is 8 while the output is 8
For 3: the expected value is 12 while the output is 12
For 4: the expected value is 16 while the output is 16
```
<!-- /output -->

</details>

## If you are coming from another language

**Python.** There is no `let`, no `mut` and no declared type, so most of these exercises have no Python form: `a = "three"` followed by `a = 10` is an ordinary rebinding (exercise 4), `pi = 3.14159` lands anywhere (6), and an `int` has no width, so `-1`, `170` and `-15 * 170` fit wherever they are put (5 and 7). Exercise 11 is the one that carries over exactly, one step later: a one-parameter function called as `print_distance(5.0, 4.0)` raises `TypeError: print_distance() takes 1 positional argument but 2 were given` — when the line runs, where rustc refuses the file. A `Book = tuple[str, str, int]` alias is read by a type checker and ignored by the interpreter; Rust's alias is checked, but as the tuple it names, not as a new type.

**C.** The two numeric refusals are C's silent conversions. `unsigned char x = -1;` compiles and stores 255, and `int pi = 3.14159;` compiles and stores 3; clang 21 mentions either only under `-Wconversion` (*implicit conversion changes signedness*, *changes value from 3.14159 to 3*). The solution to exercise 5 prints the same 255 with `-1i32 as u8`, which is the difference: in Rust the conversion is written where it happens. `short a = -15, b = 170;` multiply as `int`s to -2550, so C never asks exercise 7's question about the product. `typedef` is `type`: a second name, not a second type. Signed overflow is undefined behaviour in C and a panic or a wrap in Rust — [Signed overflow](../../../31_C_and_Cpp/signed_overflow/README.md).

**ABAP.** `DATA my_age TYPE i.` is exercise 1's `let`, and `DATA(my_age) = 40.` is the inferred form — [`DATA` ↗](https://masiarek.github.io/abap-learning-library/02_Keywords/data/index.html). Nothing corresponds to `mut`: every variable is assignable, and `CONSTANTS` is the only binding that is not — [`TYPES` and `CONSTANTS` ↗](https://masiarek.github.io/abap-learning-library/02_Keywords/types/index.html), whose `TYPES` is exercise 8's alias. Exercise 6 is not refused: a value with decimals assigned to an `i` field is *rounded* on assignment, by the table on [Conversion and comparison rules ↗](https://masiarek.github.io/abap-learning-library/03_Topics/conversion_and_comparison_rules/index.html), where Rust asks for an `as` and then truncates. Exercise 11 has no ABAP shape at all, because a method call names its parameters (`EXPORTING point = …`), and a parameter count can never be off by one without a name being wrong too.

## See also

- [*Rust: The Practical Guide*, run](../README.md) — the shelf page for the book, and the [chapter 3 exercises, run](../ch3_katas/README.md), where the exercises become loops
- [Variables](../../../15_First_Programs/variables/README.md) — `let`, `mut`, assign-once and the second `let`: exercises 1 to 4 in one lesson
- [What a warning is asking](../../../15_First_Programs/what_a_warning_is_asking/README.md) — what `#![allow(unused)]` does to the warning exercise 2 leaves behind
- [`and`, `or` and a first program's explanation, run](../../../15_First_Programs/and_or_claims_checked/README.md) — block 4 is exercise 3's rule, run
- [When to shadow](../../../18_Ownership/when_to_shadow/README.md) and [The shadowing map](../../../SHADOWING.md) — exercise 4
- [The integer types](../../../19_Numbers/the_integer_types/README.md) and [What a float stores](../../../19_Numbers/what_a_float_stores/README.md) — exercises 5 to 7
- [Type inference](../../../15_First_Programs/type_inference/README.md) — why an unannotated `3.14159` is an `f64`
- [A type alias is not a new type](../../../16_Structs/type_aliases/README.md) and [Tuples](../../../26_Collections/tuples/README.md) — exercises 8 and 11
- [Functions](../../../25_Control_Flow/functions/README.md) — exercises 9 to 12
- [`String` vs `&str`](../../../14_Strings/string_vs_str/README.md) — the type the book gives `"Michael"`
- [Exercises](../../exercises/README.md) — the other practice tracks, and which to start with
- [Katas](../../../KATAS.md) — every kata in the library, in order

## Po polsku

Dwanaście ćwiczeń z §2.5 książki *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) to po jednej odmowie kompilatora każde, a wszystkie rozwiązania z §2.6 kompilują się na rustc 1.98.0. Zmienną trzeba wprowadzić przez `let` (ćw. 1); przypisanie do istniejącej zmiennej wymaga `mut` (2), ale wiązanie bez `mut` wolno przypisać **raz**, także później niż w linii `let` (3) — dlatego `let x2;` rozwiązuje ćwiczenie 2 bez ostrzeżenia (*warning*), które zostawia `let mut x2 = x1;`. Uwaga z książki o `#![allow(unused)]` jest odwrócona: ta linia ostrzeżenie **wycisza** (*allow*), a nie pokazuje. Przesłanianie (*shadowing*, ćw. 4) to nowa zmienna pod starą nazwą, więc `&str` może „stać się” liczbą; `-1` w `u8` to błąd `E0600`, a `3.14159` w `i32` to `E0308` — Rust nie konwertuje po cichu tak jak C. Dla `-15` i `170` najmniejszym wspólnym typem jest `i16` (`i8` odrzuca `170`, `u8` odrzuca `-15`); alias typu (*type alias*, `type Book = (String, String, u32);`) to tylko druga nazwa, a nie nowy typ, a rozwiązanie z książki zmienia przy okazji rok z 2010 na 2020. Funkcja przyjmująca krotkę (*tuple*) bierze **jeden** argument: `print_distance((5.0, 4.0))` z podwójnym nawiasem, inaczej `E0061`.

**Szukaj po polsku:** `rust let mut przypisanie E0384` · `rust shadowing przesłanianie zmiennej drugi let` · `rust u8 minus jeden E0600` · `rust krotka jako argument funkcji E0061` · `rust alias typu type` · `rust allow unused ostrzeżenie`

---

[*Rust: The Practical Guide*, run](../README.md) › **Chapter 2 exercises** · next: [Chapter 3 exercises, run](../ch3_katas/README.md)
