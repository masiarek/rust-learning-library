# *Rust: The Practical Guide*, chapter 4 exercises, run

[*Rust: The Practical Guide*, run](../README.md) › **Chapter 4 exercises**

**Level:** 101 → 201 · a companion to the book's §4.7 and §4.8

**One line:** The six exercises that close chapter 4 of *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) each hand you a program that does not compile, and rustc 1.98.0 refuses every one for the reason the book intends. The book's six solutions all compile — two with warnings, two with a clippy lint — and for five of them there is a fix worth preferring: borrow a `&str`, borrow instead of cloning in the loop, move a `let` instead of wrapping it in a block, return a number instead of a reference into the vector, and drop a `mut` nothing needs.

## The verdicts

| # | Exercise | What rustc says about the given code | The book's solution | Verdict | Lesson |
|---|---|---|---|---|---|
| 1 | [Fix the compilation error](#1-fix-the-compilation-error) | `E0382` *borrow of moved value: `s1`* | `some_function(&s1, s2)` with `a1: &String` | compiles, clean; `a1: &str` is the type to write, and clippy's `ptr_arg` asks for it the moment the body calls a method on `a1` | [`String` vs `&str`](../../../14_Strings/string_vs_str/README.md) |
| 2 | [Ownership in a loop](#2-ownership-in-a-loop) | `E0382` *value moved here, in previous iteration of loop* | `temp = my_vec.clone()` | compiles, clean; five clones where a borrow costs nothing — and `temp` can go | [Borrowing](../../../18_Ownership/borrowing/README.md) |
| 3 | [Correcting ownership transfer](#3-correcting-ownership-transfer) | `E0425` *cannot find value `str1` in this scope* | `let str1 = { let str1 = generate_string(); str1 };` | compiles with *unused variable: `str2`* and two clippy `let_and_return`; the book's own alternative, the `let` moved out of the block, is the fix | [Variables](../../../15_First_Programs/variables/README.md#a-binding-is-scoped-to-its-block) |
| 4 | [Fix the borrowing issue](#4-fix-the-borrowing-issue) | `E0502` *cannot borrow `some_vec` as mutable because it is also borrowed as immutable* | the `push` moved below the `println!` | compiles; clippy `ptr_arg` on `&Vec<i32>`; returning the number ends the borrow at the call | [Borrowing](../../../18_Ownership/borrowing/README.md#where-a-borrow-ends-the-part-that-decides-everything) |
| 5 | [Correcting reference assignment](#5-correcting-reference-assignment) | `E0308` *expected `&Vec<i32>`, found `Vec<{integer}>`*, twice | `vec_ptr = &vec_1;` and `vec_ptr = &vec_2;` | compiles with four warnings: a `mut` on `vec_1` nothing needs, and a pointer assigned twice and never read | [Mutable binding, mutable reference](../../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md) |
| 6 | [Resolve the mutable reference conflict](#6-resolve-the-mutable-reference-conflict) | `E0596` *cannot borrow `first_num` as mutable, as it is not declared as mutable*, twice | `let mut` on both numbers | compiles, clean; the explanation of `let mut ref2` describes the binding as if it were the reference, and `ref2 = ref1` is a reborrow, not a move | [Mutable binding, mutable reference](../../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md) |

The given programs are the book's, unchanged, so each transcript names the exercise's file. The book's solutions are shown where they differ from the one folded under each exercise, and the folded one is compiled and run by CI. Terms the exercises assume are on [What *Rust: The Practical Guide* says about ownership in functions, run](../../../18_Ownership/ownership_in_functions_claims_checked/README.md).

## Practice

### 1. Fix the compilation error

A `String` and a `&str` are passed to `some_function`, and the line after the call prints both. Make it compile.

```rust,compile_fail
// exercise_1.rs
fn main() {
    let s1: String = String::from("this is me, ");
    let s2: &str = "Nouman";
    some_function(s1, s2); // Something is wrong here
    println!("{} {}", s1, s2);
}

fn some_function(a1: String, a2: &str) {
    println!("{} {}", a1, a2);
}
```

```text title="Abridged — real rustc output for exercise_1.rs"
error[E0382]: borrow of moved value: `s1`
 --> exercise_1.rs:5:23
  |
2 |     let s1: String = String::from("this is me, ");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2: &str = "Nouman";
4 |     some_function(s1, s2); // Something is wrong here
  |                   -- value moved here
5 |     println!("{} {}", s1, s2);
  |                       ^^ value borrowed here after move
  |
note: consider changing this parameter type in function `some_function` to borrow instead if owning the value isn't necessary
```

`s2` is a `&str`, which is `Copy`, so only `s1` moved. The book lends it — `some_function(&s1, s2)` with `a1: &String` — and that compiles without a warning. It is still the wrong parameter type: a `&String` can only be made from a `String`, while a `&str` is made from a `String` by deref coercion at the call site *and* from a literal, a slice or anything else that holds text ([`String` vs `&str`](../../../14_Strings/string_vs_str/README.md#one-parameter-serves-every-caller)). Clippy's `ptr_arg` lint says so, but only once the body does something with `a1` beyond printing it — on the book's body, which formats `a1` and nothing else, it stays quiet. Add one call:

```rust
// ptr_arg_len.rs
fn main() {
    let s1 = String::from("this is me, ");
    some_function(&s1, "Nouman");
}

fn some_function(a1: &String, a2: &str) {
    println!("{} {}", a1.len(), a2);
}
```

```text title="Abridged — real clippy 0.1.98 output for ptr_arg_len.rs"
warning: writing `&String` instead of `&str` involves a new object where a slice will do
 --> ptr_arg_len.rs:6:22
  |
6 | fn some_function(a1: &String, a2: &str) {
  |                      ^^^^^^^
  |
  = note: `#[warn(clippy::ptr_arg)]` on by default
help: change this to
  |
6 - fn some_function(a1: &String, a2: &str) {
6 + fn some_function(a1: &str, a2: &str) {
```

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg4_01_borrow_the_string -->
*[`pg4_01_borrow_the_string.rs`](examples/pg4_01_borrow_the_string.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 1 of chapter 4: `some_function(s1, s2)` moved `s1` into the call,
//! so the `println!` after it is E0382. Lend it instead — and take `&str`,
//! which a `&String` becomes at the call site by deref coercion.
//!
//!   rustc --edition 2024 pg4_01_borrow_the_string.rs -o /tmp/pg401 && /tmp/pg401

fn main() {
    let s1: String = String::from("this is me, ");
    let s2: &str = "Nouman";
    some_function(&s1, s2);
    println!("{} {}", s1, s2); // this is me,  Nouman
}

fn some_function(a1: &str, a2: &str) {
    println!("{} {}", a1, a2); // this is me,  Nouman
}
```
<!-- /source -->

<!-- output:pg4_01_borrow_the_string -->
*Verified output of [`pg4_01_borrow_the_string.rs`](examples/pg4_01_borrow_the_string.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
this is me,  Nouman
this is me,  Nouman
```
<!-- /output -->

The two spaces after the comma are the book's: `"this is me, "` ends with one, and `{} {}` adds another.

</details>

### 2. Ownership in a loop

A `while` loop copies `my_vec` into `temp`, prints `temp`, then pops from `my_vec`. Fix it, and say why the compiler complains about a line inside a loop.

```rust,compile_fail
// exercise_2.rs
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let mut temp;
    while !my_vec.is_empty() {
        temp = my_vec; // Something wrong on this line
        println!("Elements in temporary vector are: {:?}", temp);
        if let Some(last_element) = my_vec.pop() {
            println!("Popped element: {}", last_element);
        }
    }
}
```

```text title="Abridged — real rustc output for exercise_2.rs"
error[E0382]: borrow of moved value: `my_vec`
 --> exercise_2.rs:4:12
  |
2 |     let mut my_vec = vec![1, 2, 3, 4, 5];
  |         ---------- move occurs because `my_vec` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     let mut temp;
4 |     while !my_vec.is_empty() {
  |     -------^^^^^^-----------
  |     |      |
  |     |      value borrowed here after move
  |     inside of this loop
5 |         temp = my_vec; // Something wrong on this line
  |                ------ value moved here, in previous iteration of loop
  |
help: consider cloning the value if the performance cost is acceptable
```

The `^` is under the loop condition, and the label says *in previous iteration of loop*: the move on line 5 happens on the first pass, and the `is_empty()` that starts the second pass is the use after it. The book's explanation is exactly that, and its fix is the compiler's `help`, `temp = my_vec.clone()` — a new allocation on every pass for a value that is only read. A borrow does the same job for nothing, because `temp`'s last use is the `println!` on the next line, so the loan has ended before `pop` needs `&mut my_vec` — [where a borrow ends](../../../18_Ownership/borrowing/README.md#where-a-borrow-ends-the-part-that-decides-everything). And once `temp` is only a name for `my_vec`, the loop does not need it.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg4_02_borrow_in_the_loop -->
*[`pg4_02_borrow_in_the_loop.rs`](examples/pg4_02_borrow_in_the_loop.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 2 of chapter 4: `temp = my_vec` moves the vector on the first
//! pass, so the second pass finds it gone. The book clones on every pass. A
//! borrow costs nothing and ends at its last use, before `pop` — and the
//! `temp` can go altogether.
//!
//!   rustc --edition 2024 pg4_02_borrow_in_the_loop.rs -o /tmp/pg402 && /tmp/pg402

fn with_a_borrowed_temp() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let mut temp;
    while !my_vec.is_empty() {
        temp = &my_vec;
        println!("Elements in temporary vector are: {:?}", temp);
        if let Some(last_element) = my_vec.pop() {
            println!("Popped element: {}", last_element);
        }
    }
}

fn without_a_temp() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    while let Some(last_element) = my_vec.pop() {
        println!("Popped element: {last_element}, leaving {my_vec:?}");
    }
}

fn main() {
    println!("temp = &my_vec, a loan that ends before pop:");
    with_a_borrowed_temp();
    println!();
    println!("no temp at all:");
    without_a_temp();
}
```
<!-- /source -->

<!-- output:pg4_02_borrow_in_the_loop -->
*Verified output of [`pg4_02_borrow_in_the_loop.rs`](examples/pg4_02_borrow_in_the_loop.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
temp = &my_vec, a loan that ends before pop:
Elements in temporary vector are: [1, 2, 3, 4, 5]
Popped element: 5
Elements in temporary vector are: [1, 2, 3, 4]
Popped element: 4
Elements in temporary vector are: [1, 2, 3]
Popped element: 3
Elements in temporary vector are: [1, 2]
Popped element: 2
Elements in temporary vector are: [1]
Popped element: 1

no temp at all:
Popped element: 5, leaving [1, 2, 3, 4]
Popped element: 4, leaving [1, 2, 3]
Popped element: 3, leaving [1, 2]
Popped element: 2, leaving [1]
Popped element: 1, leaving []
```
<!-- /output -->

The book's `temp = my_vec.clone();` compiles and prints the first ten lines above; `clone` allocates five times on the way. In Python the given line is not a bug at all — `temp = my_vec` makes a second name for the same list, and the pop shows through it ([Assignment does not copy ↗](https://masiarek.github.io/python-learning-library/04_Names_and_Objects/assignment_does_not_copy/) in the Python library); the borrowed `temp` above is that, with the compiler checking that nothing pops while the second name is in use.

</details>

### 3. Correcting ownership transfer

`str1` is created inside a block and moved into `str2` after the block closes. Make it compile.

```rust,compile_fail
// exercise_3.rs
fn main() {
    {
        let str1 = generate_string();
    }
    let str2 = str1;   // Something wrong with this line
}

fn generate_string() -> String {
    let some_string = String::from("I will generate a string");
    some_string
}
```

```text title="Real rustc output for exercise_3.rs"
error[E0425]: cannot find value `str1` in this scope
 --> exercise_3.rs:5:16
  |
5 |     let str2 = str1;   // Something wrong with this line
  |                ^^^^
  |
help: the binding `str1` is available in a different scope in the same function
 --> exercise_3.rs:3:13
  |
3 |         let str1 = generate_string();
  |             ^^^^
```

Not an ownership error: `E0425` is name resolution, and the `help` says where the name went. The book's solution keeps the block and makes it an expression, `let str1 = { let str1 = generate_string(); str1 };`, then mentions the alternative — move the `let` out of the block — as an aside. The alternative is the fix. The block version compiles, but with the exercise's `str2` unused, and clippy's `let_and_return` fires twice on it, once on the block and once on `generate_string` as the exercise wrote it:

```rust
// solution_3.rs — the book's
fn main() {
    let str1 = {
        let str1 = generate_string();
        str1
    };
    let str2 = str1;
}

fn generate_string() -> String {
    let some_string = String::from("I will generate a string");
    some_string
}
```

```text title="Abridged — real clippy 0.1.98 output for solution_3.rs, the unused-variable warning dropped"
warning: returning the result of a `let` binding from a block
 --> solution_3.rs:4:9
  |
3 |         let str1 = generate_string();
  |         ----------------------------- unnecessary `let` binding
4 |         str1
  |         ^^^^
  |
  = note: `#[warn(clippy::let_and_return)]` on by default
help: return the expression directly

warning: returning the result of a `let` binding from a block
  --> solution_3.rs:11:5
   |
10 |     let some_string = String::from("I will generate a string");
   |     ----------------------------------------------------------- unnecessary `let` binding
11 |     some_string
   |     ^^^^^^^^^^^
```

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg4_03_let_outside_the_block -->
*[`pg4_03_let_outside_the_block.rs`](examples/pg4_03_let_outside_the_block.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 3 of chapter 4: `str1` was declared inside a block and used after
//! it, E0425. Declare it where it is used, and return the `String` directly
//! rather than through a `let` (clippy's let_and_return, twice in the book).
//!
//!   rustc --edition 2024 pg4_03_let_outside_the_block.rs -o /tmp/pg403 && /tmp/pg403

fn main() {
    let str1 = generate_string();
    let str2 = str1;
    println!("{str2}"); // I will generate a string
}

fn generate_string() -> String {
    String::from("I will generate a string")
}
```
<!-- /source -->

<!-- output:pg4_03_let_outside_the_block -->
*Verified output of [`pg4_03_let_outside_the_block.rs`](examples/pg4_03_let_outside_the_block.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
I will generate a string
```
<!-- /output -->

</details>

### 4. Fix the borrowing issue

`first` borrows the first element of `some_vec`, the vector is pushed to, and then `first` is printed. Make it compile.

```rust,compile_fail
// exercise_4.rs
fn main() {
    let mut some_vec = vec![1, 2, 3];
    let first = get_first_element(&some_vec);
    some_vec.push(4);
    println!("The first number is: {}", first);
}

fn get_first_element(num_vec: &Vec<i32>) -> &i32 {
    &num_vec[0]
}
```

```text title="Real rustc output for exercise_4.rs"
error[E0502]: cannot borrow `some_vec` as mutable because it is also borrowed as immutable
 --> exercise_4.rs:4:5
  |
3 |     let first = get_first_element(&some_vec);
  |                                   --------- immutable borrow occurs here
4 |     some_vec.push(4);
  |     ^^^^^^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("The first number is: {}", first);
  |                                         ----- immutable borrow later used here
```

The book's explanation says the vector cannot be modified "if immutable references to it are still in scope", and the transcript is more exact: the reference is still *used*, on line 5, and that use is what keeps the loan alive across the `push` — the three labelled lines of [A borrow is a loan](../../../18_Ownership/references/a_borrow_is_a_loan/README.md#reading-a-borrow-error). The book's fix moves the `push` below the `println!`, which ends the loan before the write and compiles without a rustc warning. Clippy has one thing to say about it, the same `ptr_arg` as exercise 1 — here the body indexes `num_vec`, so the lint does fire:

```text title="Abridged — real clippy 0.1.98 output for solution_4.rs"
warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
 --> solution_4.rs:9:31
  |
9 | fn get_first_element(num_vec: &Vec<i32>) -> &i32 {
  |                               ^^^^^^^^^
  |
  = note: `#[warn(clippy::ptr_arg)]` on by default
help: change this to
  |
9 - fn get_first_element(num_vec: &Vec<i32>) -> &i32 {
9 + fn get_first_element(num_vec: &[i32]) -> &i32 {
```

Reordering works when you can reorder. The fix that survives any order is to stop returning a reference into the vector at all: an `i32` is `Copy`, so `num_vec.first().copied()` hands back the number, the loan ends at the call, and the empty-vector case that `&num_vec[0]` would panic on becomes `None`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg4_04_first_by_copy -->
*[`pg4_04_first_by_copy.rs`](examples/pg4_04_first_by_copy.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 4 of chapter 4: `first` borrows `some_vec` until the `println!`,
//! so the `push` between them is E0502. The book moves the push below the
//! print. Returning the number — an `i32` is Copy — ends the borrow at the
//! call, and `&[i32]` takes any slice, not only a `Vec`.
//!
//!   rustc --edition 2024 pg4_04_first_by_copy.rs -o /tmp/pg404 && /tmp/pg404

fn main() {
    let mut some_vec = vec![1, 2, 3];
    let first = get_first_element(&some_vec);
    some_vec.push(4);
    if let Some(first) = first {
        println!("The first number is: {}", first); // The first number is: 1
    }
    println!("some_vec is now {:?}", some_vec); // some_vec is now [1, 2, 3, 4]
    println!("of an empty slice: {:?}", get_first_element(&[])); // of an empty slice: None
}

fn get_first_element(num_vec: &[i32]) -> Option<i32> {
    num_vec.first().copied()
}
```
<!-- /source -->

<!-- output:pg4_04_first_by_copy -->
*Verified output of [`pg4_04_first_by_copy.rs`](examples/pg4_04_first_by_copy.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The first number is: 1
some_vec is now [1, 2, 3, 4]
of an empty slice: None
```
<!-- /output -->

</details>

### 5. Correcting reference assignment

`vec_ptr` is declared as a `&Vec<i32>` and then assigned `vec_1`, and later `vec_2`. Make it compile.

```rust,compile_fail
// exercise_5.rs
fn main() {
    let mut vec_1 = vec![1, 2, 3];
    let vec_2 = vec![4, 5, 6];
    let mut vec_ptr: &Vec<i32>;
    vec_ptr = vec_1;
    println!("vec ptr is pointing to vec_1");
    vec_ptr = vec_2;
    println!("vec ptr is updated and now pointing to vec_2");
}
```

```text title="Abridged — real rustc output for exercise_5.rs, the second E0308 (line 7, the same words) dropped"
error[E0308]: mismatched types
 --> exercise_5.rs:5:15
  |
4 |     let mut vec_ptr: &Vec<i32>;
  |                      --------- expected due to this type
5 |     vec_ptr = vec_1;
  |               ^^^^^ expected `&Vec<i32>`, found `Vec<{integer}>`
  |
  = note: expected reference `&Vec<i32>`
                found struct `Vec<{integer}>`
help: consider borrowing here
  |
5 |     vec_ptr = &vec_1;
  |               +
```

A type error, not a borrow error: the place wants a reference and is handed a vector, and nothing coerces an owned value to a `&`. The book's fix is the `help`, `&vec_1` and `&vec_2`, and it compiles — with four warnings. `vec_1` is `mut` and nothing writes to it (*variable does not need to be mutable*), and `vec_ptr` is assigned twice and never read, so rustc reports *variable `vec_ptr` is assigned to, but never used* and *value assigned to `vec_ptr` is never read*, once per assignment. The `let mut vec_ptr` is right, and it is the point of the exercise: the *binding* is re-pointed, which is what `mut` on a `let` permits, while the `&Vec<i32>` it holds permits no writing through — [Mutable binding, mutable reference](../../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md) is that grid. Read the pointer and drop the `mut` on `vec_1`, and every warning goes.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg4_05_borrow_not_move -->
*[`pg4_05_borrow_not_move.rs`](examples/pg4_05_borrow_not_move.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 5 of chapter 4: `vec_ptr` is declared `&Vec<i32>`, and
//! `vec_ptr = vec_1` hands it the vector itself, E0308. Borrow with `&`. The
//! book keeps a `mut` on `vec_1` that nothing writes to, and never reads
//! `vec_ptr`, so its solution compiles with four warnings; reading the
//! pointer settles them.
//!
//!   rustc --edition 2024 pg4_05_borrow_not_move.rs -o /tmp/pg405 && /tmp/pg405

fn main() {
    let vec_1 = vec![1, 2, 3];
    let vec_2 = vec![4, 5, 6];
    let mut vec_ptr: &Vec<i32> = &vec_1;
    println!("vec ptr is pointing to vec_1: {vec_ptr:?}"); // vec ptr is pointing to vec_1: [1, 2, 3]
    vec_ptr = &vec_2;
    println!("vec ptr is updated and now pointing to vec_2: {vec_ptr:?}"); // vec ptr is updated and now pointing to vec_2: [4, 5, 6]
}
```
<!-- /source -->

<!-- output:pg4_05_borrow_not_move -->
*Verified output of [`pg4_05_borrow_not_move.rs`](examples/pg4_05_borrow_not_move.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
vec ptr is pointing to vec_1: [1, 2, 3]
vec ptr is updated and now pointing to vec_2: [4, 5, 6]
```
<!-- /output -->

</details>

### 6. Resolve the mutable reference conflict

Two numbers, two `&mut` references to them, a write through each, then `ref2 = ref1` and a print through `ref2`. Make it compile.

```rust,compile_fail
// exercise_6.rs
fn main() {
    let first_num = 42;
    let second_num = 64;
    let ref1 = &mut first_num;
    let mut ref2 = &mut second_num;
    *ref1 = 15;
    *ref2 = 10;
    ref2 = ref1;
    println!("Updated first number: {ref2}");
}
```

```text title="Abridged — real rustc output for exercise_6.rs, the second E0596 (second_num, line 5) dropped"
error[E0596]: cannot borrow `first_num` as mutable, as it is not declared as mutable
 --> exercise_6.rs:4:16
  |
4 |     let ref1 = &mut first_num;
  |                ^^^^^^^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut first_num = 42;
  |         +++
```

No conflict between the references, despite the exercise's title: `&mut first_num` needs `first_num` to be a `let mut`, and the same for `second_num`. The book's fix adds both `mut`s, and the program compiles clean and prints 15. Two things in its explanation to correct. On `let mut ref2`, the book says "a mutable reference means that the reference can be updated to point to some other variable" — that is the `mut` on the *binding*, and it would be needed for `ref2 = ref1` even if `ref2` held a `&i32`; the `&mut` in the type is what permits `*ref2 = 10`. And `ref2 = ref1` does not move `ref1`, although a `&mut` is [not `Copy`](../../../18_Ownership/copy_or_move/README.md): the assignment target already has the type `&mut i32`, so the compiler inserts a [reborrow](../../../18_Ownership/reborrowing/README.md), `&mut *ref1`, and `ref1` is usable again after `ref2`'s last use. Use `ref1` *before* that and the refusal is `E0502`, a borrow error, where a move would have given `E0382`:

```text title="Abridged — real rustc output for assign_reborrows.rs, the book's solution with ref1 printed before ref2"
error[E0502]: cannot borrow `ref1` as immutable because it is also borrowed as mutable
  --> assign_reborrows.rs:9:28
   |
 8 |     ref2 = ref1;
   |            ---- mutable borrow occurs here
 9 |     println!("ref1 again: {ref1}");
   |                            ^^^^ immutable borrow occurs here
10 |     println!("Updated first number: {ref2}");
   |                                      ---- mutable borrow later used here
```

Write `let ref3 = ref1;` instead — a `let` with no type to aim at — and it is a move, `E0382` *borrow of moved value: `ref1`* (`let_moves.rs`), which is [the `let` that does not reborrow](../../../18_Ownership/reborrowing/README.md#the-let-that-does-not-do-it).

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg4_06_two_muts -->
*[`pg4_06_two_muts.rs`](examples/pg4_06_two_muts.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 6 of chapter 4: `&mut first_num` needs `first_num` declared `mut`,
//! E0596, and the same for `second_num`. With both `mut` the book's program
//! compiles. `ref2 = ref1` then reborrows `ref1` rather than moving it — the
//! assignment target's type is known — so `ref1` works again once `ref2` is
//! done with it.
//!
//!   rustc --edition 2024 pg4_06_two_muts.rs -o /tmp/pg406 && /tmp/pg406

fn main() {
    let mut first_num = 42;
    let mut second_num = 64;
    let ref1 = &mut first_num;
    let mut ref2 = &mut second_num;
    *ref1 = 15;
    *ref2 = 10;
    ref2 = ref1;
    println!("Updated first number: {ref2}"); // Updated first number: 15
    println!("ref1 again, after ref2's last use: {ref1}"); // ref1 again, after ref2's last use: 15
    println!("first_num = {first_num}, second_num = {second_num}"); // first_num = 15, second_num = 10
}
```
<!-- /source -->

<!-- output:pg4_06_two_muts -->
*Verified output of [`pg4_06_two_muts.rs`](examples/pg4_06_two_muts.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Updated first number: 15
ref1 again, after ref2's last use: 15
first_num = 15, second_num = 10
```
<!-- /output -->

</details>

## If you are coming from another language

**Python.** Exercises 1, 2 and 3 are not bugs in Python: passing `s1` to a function, assigning `temp = my_vec`, and reading a name after the block that bound it all work, because every name is a reference and blocks do not scope names. [Assignment does not copy ↗](https://masiarek.github.io/python-learning-library/04_Names_and_Objects/assignment_does_not_copy/) in the Python library is exercise 2's line, run: `temp` and `my_vec` are one list. Exercise 4 is a Python bug that Python does not catch — hold a reference into a list, mutate the list, read the reference — except that Python's "reference into a list" is a copy of the element, so the printed number is stale rather than invalid. Exercises 5 and 6 have no Python spelling: there is no `&`, and no `mut` on a name.

**C.** Exercise 5 is the C mistake `int *p; p = x;` for an `int x`, which C also refuses (*incompatible integer to pointer conversion*), and exercise 6's `&mut` on a non-`mut` variable has no C counterpart, since every C variable is writable unless declared `const`. Exercise 4 compiles in C and reads memory that `push` may have moved — the same bug as [holding a reference across `push_back`](../../../18_Ownership/references/reference_claims_checked/README.md#c-has-no-reference-variables-c-does) in C++, and `E0502` is Rust refusing it before it runs. Exercise 1 in C is a question of who calls `free`: a `char *` parameter says nothing about it, and `&str` against `String` does.

**C++.** Exercise 1's given code is what `std::string` does by default — pass by value, copy — so it compiles and runs, one allocation per call; the book's `&String` is `const std::string&`, and `&str` is `std::string_view`. Exercise 2 is a `std::vector` copied on every pass, as the book's `clone()` does explicitly. Exercise 6's two `&mut` are two non-`const` references, which C++ allows to alias freely; `ref2 = ref1` in C++ would write through both, not re-seat, since a C++ reference cannot be re-seated — [the lesson's bridge](../../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md#if-you-are-coming-from-another-language) runs that.

## See also

- [What *Rust: The Practical Guide* says about ownership in functions, run](../../../18_Ownership/ownership_in_functions_claims_checked/README.md) — §4.2 and §4.4, the sections the exercises come from, claim by claim
- [Mutable binding, mutable reference](../../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md) — §4.6, the grid exercises 5 and 6 sit in
- [*Rust: The Practical Guide*, run](../README.md) — the book's home in this library
- [Borrowing](../../../18_Ownership/borrowing/README.md) — where a borrow ends, which decides exercises 2 and 4
- [A borrow is a loan](../../../18_Ownership/references/a_borrow_is_a_loan/README.md) — the three labelled lines of exercise 4's error
- [Reborrowing](../../../18_Ownership/reborrowing/README.md) — why exercise 6's `ref2 = ref1` leaves `ref1` usable
- [Copy or move?](../../../18_Ownership/copy_or_move/README.md) — `&str` copies, `String` moves, `&mut` moves though it owns nothing
- [`String` vs `&str`](../../../14_Strings/string_vs_str/README.md) — exercise 1's parameter type, and why `&String` is the wrong one
- [Variables](../../../15_First_Programs/variables/README.md#a-binding-is-scoped-to-its-block) — exercise 3's `E0425`, and the `help` that points at the other scope
- [When to shadow](../../../18_Ownership/when_to_shadow/README.md) — exercise 3's `let str1 = { let str1 = … }`, a shadow inside a block
- [Every kata in the library](../../../KATAS.md) — where these six sit in the sequence
- [E0382 ↗](https://doc.rust-lang.org/error_codes/E0382.html) · [E0425 ↗](https://doc.rust-lang.org/error_codes/E0425.html) · [E0502 ↗](https://doc.rust-lang.org/error_codes/E0502.html) · [E0308 ↗](https://doc.rust-lang.org/error_codes/E0308.html) · [E0596 ↗](https://doc.rust-lang.org/error_codes/E0596.html) · [clippy `ptr_arg` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#ptr_arg) · [clippy `let_and_return` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#let_and_return)

## Po polsku

Sześć ćwiczeń zamykających rozdział 4 książki *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) daje program, który się nie kompiluje, i prosi o poprawkę. rustc 1.98.0 odrzuca każdy z powodu, o który chodzi w książce: `E0382` (wartość przeniesiona do funkcji, a potem użyta; w pętli — przeniesiona „w poprzedniej iteracji"), `E0425` (nazwa z zamkniętego bloku), `E0502` (`push` w trakcie żywej pożyczki), `E0308` (miejsce typu `&Vec<i32>` dostaje `Vec`), `E0596` (`&mut` do zmiennej bez `mut`). Wszystkie rozwiązania z książki się kompilują, ale dwa z ostrzeżeniami (nieużywane `str2`, zbędne `mut` i nieodczytany wskaźnik) i dwa z lintem clippy (`let_and_return`, `ptr_arg`). Lepsze poprawki: parametr `&str` zamiast `&String`, pożyczka `&my_vec` zamiast `clone()` w każdej iteracji (albo brak `temp`), `let` wyniesione przed blok zamiast bloku-wyrażenia, zwracanie liczby (`Option<i32>`) zamiast referencji do elementu, `let mut vec_ptr` bez zbędnego `mut` na `vec_1`. W ćwiczeniu 6 `ref2 = ref1` nie przenosi `ref1`, tylko pożycza ponownie (*reborrow*), bo typ celu przypisania jest znany.

**Szukaj po polsku:** ćwiczenia z własności · `rust E0382 moved in previous iteration of loop` · `rust E0425 cannot find value in this scope` · `rust E0502 cannot borrow as mutable` · `rust E0308 expected reference found struct` · `rust E0596 not declared as mutable` · clippy `ptr_arg` · pożyczka wtórna przy przypisaniu
