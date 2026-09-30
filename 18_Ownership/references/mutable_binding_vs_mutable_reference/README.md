# Mutable binding, mutable reference: two choices, not one

[References](../README.md) › **Mutable binding, mutable reference**

**Level:** 101 → 201 · working knowledge

**One line:** `let` or `let mut` decides whether the *name* can be pointed at something else; `&T` or `&mut T` decides whether you can write *through* it. The two are chosen independently, so there are four combinations — and a nested type such as `&mut &T` is the same two choices taken again one level down, not a fifth kind of reference.

```rust
fn main() {
    let mut a = 1;
    let mut b = 2;

    let r = &a;              // let + &T: read through it, and that is all
    println!("{}", *r);      // 1

    let mut r = &a;          // let mut + &T: the name can be pointed elsewhere
    println!("{}", *r);      // 1
    r = &b;
    println!("{}", *r);      // 2

    let r = &mut a;          // let + &mut T: write through it, never re-point it
    *r += 10;
    println!("{a}");         // 11

    let mut r = &mut a;      // let mut + &mut T: both
    *r += 100;
    r = &mut b;
    *r += 100;
    println!("{a} {b}");     // 111 102
}
```

## The two choices

The `mut` before the name is the same `mut` as on any `let`: it is a property of the [binding, not of the value](../../../15_First_Programs/variables/README.md#mutability-belongs-to-the-binding-not-to-the-value). The `mut` after the `&` is part of the reference's *type*, and says what the reference lets you do to what it points at — [Where the `&` sits](../../where_the_sigil_sits/README.md) is the page on reading the two positions. Cross them:

| | `&T`, a shared reference | `&mut T`, a mutable reference |
|---|---|---|
| `let r` | read through `r` | read and write through `r` |
| `let mut r` | read through `r`, and re-point `r` | read and write through `r`, and re-point `r` |

Two refusals mark the edges, and their error codes say which choice was missing:

- **Re-pointing a `let` binding is `E0384`** — *cannot assign twice to immutable variable* — whether it holds a `&T` or a `&mut T`. The reference kind is irrelevant; the binding is.
- **Writing through a `&T` is `E0596` or `E0594`** — *cannot borrow as mutable* when a method needs `&mut self`, *cannot assign* when you write `*r = …` — whether the binding is `let` or `let mut`. The binding is irrelevant; the reference kind is.

The first is block 1 of the run below with a `mut` removed; the second is the same with a `&mut` removed.

## Refused, with the compiler's words

`let reference = &vec_1; reference = &vec_2;` is *Rust: The Practical Guide*'s Listing 4.29, the book's example of an immutable binding of an immutable reference:

```rust,compile_fail
// listing_4_29.rs
fn main() {
    let mut vec_1 = vec![1, 2, 3];
    let mut vec_2 = vec![4, 5, 6];
    let reference = &vec_1;
    reference = &vec_2; // Error
}
```

```text title="Abridged — real rustc output for listing_4_29.rs, four warnings dropped"
error[E0384]: cannot assign twice to immutable variable `reference`
 --> listing_4_29.rs:5:5
  |
4 |     let reference = &vec_1;
  |         --------- first assignment to `reference`
5 |     reference = &vec_2; // Error
  |     ^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
4 |     let mut reference = &vec_1;
  |         +++
```

The four dropped warnings are the book's own: `vec_1` and `vec_2` are `mut` for no reason (*variable does not need to be mutable*), `reference` is assigned and never read, and so is the value assigned to it. Listing 4.31 is the same refusal with `&mut vec_1` on the right — same `E0384`, same `help`.

A `&T` cannot be written through, and rustc says so in two ways. A method that needs `&mut self`:

```rust,compile_fail
// write_through_shared.rs
fn main() {
    let mut vec_1 = vec![1, 2, 3];
    let reference = &vec_1;
    reference.push(10);
    println!("{:?}", vec_1);
}
```

```text title="Abridged — real rustc output for write_through_shared.rs, one warning dropped"
error[E0596]: cannot borrow `*reference` as mutable, as it is behind a `&` reference
 --> write_through_shared.rs:4:5
  |
4 |     reference.push(10);
  |     ^^^^^^^^^ `reference` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
3 |     let reference = &mut vec_1;
  |                      +++
```

And an assignment through the `*`:

```rust,compile_fail
// assign_through_shared.rs
fn main() {
    let mut n = 1;
    let r = &n;
    *r = 2;
    println!("{n}");
}
```

```text title="Abridged — real rustc output for assign_through_shared.rs, one warning dropped"
error[E0594]: cannot assign to `*r`, which is behind a `&` reference
 --> assign_through_shared.rs:4:5
  |
4 |     *r = 2;
  |     ^^^^^^ `r` is a `&` reference, so it cannot be written to
  |
help: consider changing this to be a mutable reference
  |
3 |     let r = &mut n;
  |              +++
```

Both `help` lines add the `mut` after the `&`; neither touches the `let`. [Every reference error](../reference_errors/README.md#writing-through-a-reference) has the two side by side with their fixes.

## The run

<!-- output:pg_bind_lesson -->
*Verified output of [`pg_bind_lesson.rs`](examples/pg_bind_lesson.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Two choices, four combinations
   let r = &a;                              read *r = 1
   let mut r = &a; r = &b;                  re-pointed: *r was 1, now 2
   let r = &mut a; *r += 10;                wrote through: a = 11
   let mut r = &mut a; ...; r = &mut b;     both: a = 111, b = 102

2. Nesting: the same two choices, taken again at each level
   &mut &T:     let r = &mut inner; *r = &vec_2;      inner now [4, 5, 6], r itself never re-pointed
   & &mut T:    let rr = &m;                          read **rr = 1; **rr = 5 is E0594
   &&T:         let s2 = &s1;                         read **s2 = 7
   &mut &mut T: **rm += 10; *rm = &mut q; **rm += 10;  p = 11, q = 12

3. Listing 4.33: *reference = &mut &vec_2 is a coercion, and *reference = &vec_2 is its plain spelling
   after *reference = &mut &vec_2:  inner = [4, 5, 6]
   let plain: &Vec<i32> = &mut &vec_1; -> [1, 2, 3]  (the same coercion, on its own)
   after *reference = &vec_2:       inner = [4, 5, 6]

4. "Not possible with other types of references": an immutable binding of a &mut writes too
   let r = &mut vec_3; r.push(10); *r = vec![9];  -> vec_3 = [9]

5. Listing 4.35: &*z is a reborrow, and let y: &i32 = z; is the same one
   y = 45, y2 = 45: two shared reborrows of *z, and z is frozen while they are used
   after their last use, *z += 1 compiles: x = 46
```
<!-- /output -->

Block 1 is the opening program, with each combination named. Blocks 2 to 5 are the rest of this page.

## Nesting: the same two choices, one level down

A reference can point at a reference, and each level has its own `&` or `&mut`. The book's Table 4.1 lists `&mut &T` as a fifth and sixth "type of reference"; it is the 2 × 2 above applied twice, and the two combinations the table leaves out behave exactly as the grid predicts. Block 2 runs all four:

| `r` holds | `**r` — the value at the bottom | `*r` — the inner reference | `r` — the binding |
|---|---|---|---|
| `&&T` | read | fixed | `let` / `let mut` as above |
| `& &mut T` | read only — `**r = 5` is `E0594`, *behind a `&` reference* | fixed | ″ |
| `&mut &T` | read only | **re-pointable**: `*r = &vec_2` | ″ |
| `&mut &mut T` | read and write | re-pointable | ″ |

The rule underneath: writing at any depth needs a `&mut` at *every* level above it, because the first `&` on the way down freezes everything below it. Re-pointing the reference at depth *k* needs `&mut` at every level above *k*; re-pointing the binding itself needs `let mut`, and nothing about the type helps. So `let reference = &mut &vec_1; reference = &mut &vec_2;` is the same `E0384` as Listing 4.29:

```text title="Abridged — real rustc output for listing_4_33_rebind.rs, four warnings dropped"
error[E0384]: cannot assign twice to immutable variable `reference`
 --> listing_4_33_rebind.rs:5:5
  |
4 |     let reference = &mut &vec_1;
  |         --------- first assignment to `reference`
5 |     reference = &mut &vec_2;
  |     ^^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
```

And the `& &mut T` row — a `&mut` behind a `&` is as read-only as anything else behind a `&`:

```rust,compile_fail
// write_through_shared_to_mut.rs
fn main() {
    let mut n = 1;
    let mut m = &mut n;
    let r = &m;
    **r = 2;
    println!("{}", m);
}
```

```text title="Abridged — real rustc output for write_through_shared_to_mut.rs, one warning dropped"
error[E0594]: cannot assign to `**r`, which is behind a `&` reference
 --> write_through_shared_to_mut.rs:5:5
  |
5 |     **r = 2;
  |     ^^^^^^^ `r` is a `&` reference, so it cannot be written to
```

The bottom row, `&mut &mut T`, is the one with a job: a function taking `t: &mut &mut [T]` can move where the *caller's* slice starts, which a plain `&mut [T]` cannot — [Re-pointing a slice](../repointing_a_slice/README.md) is that page, and `Read for &[u8]` is std doing it.

## Listing 4.33, and the coercion in it

The book's example of the `&mut &T` row is `let reference = &mut &vec_1; *reference = &mut &vec_2;`. The second line compiles, and it is worth seeing why: the place `*reference` has type `&Vec<i32>`, and the right-hand side is a `&mut &Vec<i32>`. Rust coerces one to the other by the deref coercion — `&mut T` to `&U` when `T: Deref<Target = U>`, rule `coerce.types.deref` in [the Reference ↗](https://doc.rust-lang.org/reference/type-coercions.html#coercion-types) — with `T` the reference `&Vec<i32>`, whose `Deref` target is `Vec<i32>`. The `&mut` is made and dereferenced again on the same line, which is what clippy objects to:

```text title="Abridged — real clippy 0.1.98 output for listing_4_33.rs, two unused_mut warnings dropped"
warning: this expression creates a reference which is immediately dereferenced by the compiler
 --> listing_4_33.rs:5:18
  |
5 |     *reference = &mut &vec_2;
  |                  ^^^^^^^^^^^ help: change this to: `&vec_2`
  |
  = note: `#[warn(clippy::needless_borrow)]` on by default
```

Block 3 runs both spellings and the coercion on its own, as `let plain: &Vec<i32> = &mut &vec_1;`, and [Coercion](../../../29_Conversion/coercion/README.md) lists the sites where it happens. The book's closing sentence on this listing — that mutating what an immutable binding holds "is not possible with other types of references" — is block 4: `let r = &mut vec_3;` is an immutable binding too, and `r.push(10)` and `*r = vec![9]` both change `vec_3` through it. Every `&mut` writes through a `let` binding; `&mut &T` is the ordinary case with `T` happening to be a reference.

## The reborrow `&*z`

The sidebar's Listing 4.35 converts a `&mut i32` to a `&i32` with `let y = &*z;`. That is a [reborrow](../../reborrowing/README.md#a-mut-can-hand-out-shared-references-too): a fresh shared reference to the place `z` points at, and while `y` is in use `z` is frozen — the borrow checker records it as a loan on `*z`, [the same way it records every other borrow](../a_borrow_is_a_loan/README.md). Block 5 shows the plain form beside it, `let y2: &i32 = z;`, which is the same reborrow made by the `&mut T` to `&T` coercion (`coerce.types.mut-reborrow`). Both read 45, and after their last use `*z += 1` compiles.

The book says the conversion lets `y` "safely access the value without permitting further modifications". It permits them the moment `y` is done. Use `y` *after* writing through `z`, and the write is refused:

```rust,compile_fail
// listing_4_35_write_while_frozen.rs
fn main() {
    let mut x = 45;
    let z = &mut x;
    let y = &*z;
    *z += 1;
    println!("{y}");
}
```

```text title="Real rustc output for listing_4_35_write_while_frozen.rs"
error[E0506]: cannot assign to `*z` because it is borrowed
 --> listing_4_35_write_while_frozen.rs:5:5
  |
4 |     let y = &*z;
  |             --- `*z` is borrowed here
5 |     *z += 1;
  |     ^^^^^^^ `*z` is assigned to here but it was already borrowed
6 |     println!("{y}");
  |                - borrow later used here
```

The three labelled lines are the loan, the use against it, and the later use that kept the loan alive — [Borrowed state](../../borrowed_state/README.md) is the owner's-side page on `E0506`.

## Table 4.1, corrected

The book's table, *Types of References*, has six rows: `ref: &T`, `mut ref: &T`, `ref: &mut T`, `mut ref: &mut T`, `ref: &mut &T` and `mut ref: &mut &T`, described as immutable or mutable bindings of immutable or mutable references. Four things to correct, each measured above:

1. The first four rows are not four types of reference. They are two reference types under two binding choices, which is why the table's descriptions read as *binding* × *reference*.
2. Rows five and six are one nested type under the same two binding choices, and its siblings `& &mut T` and `&mut &mut T` are missing. The full set is 2 × 2 at every level.
3. Row five says that with `ref: &mut &T` "you can modify what it points to". You can re-point the *inner* reference through it. The binding still cannot be re-pointed: `E0384`, exactly as in row one.
4. "Not possible with other types of references": every `&mut` mutates through an immutable binding.

| binding | reference | re-point the binding? | write through it? | re-point the inner reference? |
|---|---|---|---|---|
| `let r` | `&T` | no, `E0384` | no, `E0594`/`E0596` | — |
| `let mut r` | `&T` | yes | no | — |
| `let r` | `&mut T` | no, `E0384` | yes | — |
| `let mut r` | `&mut T` | yes | yes | — |
| `let r` | `&&T` | no | no | no |
| `let r` | `& &mut T` | no | no, `E0594` | no |
| `let r` | `&mut &T` | no, `E0384` | no | yes, `*r = &other` |
| `let r` | `&mut &mut T` | no | yes | yes |

Add `mut` to any binding in the lower half and its first column turns to yes; nothing else in the row changes.

## If you are coming from another language

**C** has this exact 2 × 2, spelled with `const` on two sides of the `*`, and the transcripts are clang's (Apple clang 21, `-std=c17`). `const int *p` is a pointer to a constant: `p = &b` is fine and `*p = 5` is refused — Rust's `let mut r = &a`. `int *const p` is a constant pointer: `*p = 5` is fine and `p = &b` is refused — Rust's `let r = &mut a`. `const int *const p` is `let r = &a`, and a bare `int *p` is `let mut r = &mut a`.

```text title="Real clang output for pointer_to_const.c, const_pointer.c and middle_const.c"
pointer_to_const.c:5:8: error: read-only variable is not assignable
    5 |     *p = 5;
      |     ~~ ^
const_pointer.c:5:7: error: cannot assign to variable 'p' with const-qualified type 'int *const'
    5 |     p = &b;
      |     ~ ^
middle_const.c:6:9: error: read-only variable is not assignable
    6 |     *pp = &b;
      |     ~~~ ^
```

What transfers is the whole grid, and the reading rule: the `const` to the left of the `*` belongs to the pointee, the one to the right to the pointer, just as Rust's `mut` after the `&` belongs to the referent and the `mut` after `let` to the binding. What changes is what a `&T` promises beyond read-only: in C a `const int *` can alias an `int *` that somebody else writes through, and `(int *)p` casts the `const` away; a Rust `&T` promises nobody writes through *any* path while it lives, and there is nothing to cast. Nesting is where the two part company. In C each `const` freezes one level only: `middle_const.c` declares `int *const *pp`, and clang refuses `*pp = &b` on line 6 while accepting `**pp = 5` on the line before it. In Rust the `& &mut T` row above is `E0594`, because the first `&` on the way down makes everything below it read-only.

**C++** has references, and they are all `let r` bindings:

```cpp
// reference_reseat.cpp
int main() {
    int a = 1, b = 2;
    int &r = a;
    r = b;
    return a;
}
```

`r = b` does not re-seat `r`; it writes `b`'s value into `a`. Apple clang 21 builds this under `-std=c++17 -Wall` with no diagnostic, and the program exits with status 2. A C++ reference can never be re-pointed after initialization, so the only choice C++ gives you is `int&` against `const int&`, one column of the grid. To get the other column you go back to pointers.

**Python** has only names, and every name is `let mut r` with no `&mut` anywhere: `r = b` rebinds, never writes through, and for an `int` there is nothing to write through. For a list, `r.append(4)` mutates the one object that every name for it shares — [Assignment does not copy ↗](https://masiarek.github.io/python-learning-library/04_Names_and_Objects/assignment_does_not_copy/) in the Python library is that page. Rust makes the two acts different in the source: `r = &b` is the Python assignment, `*r = 5` has no Python spelling, and only a `&mut` allows it.

## Practice

**Eight one-line edits.** Start from `let mut a = 1; let mut b = 2;` and one line, `let r = &a;`. For each edit below, say whether it compiles, and if not, which error code — `E0384`, `E0594` or `E0506` — and which of the two choices it violates. Then compile each and check.

1. `let r = &a; r = &b;`
2. `let mut r = &a; r = &b;`
3. `let r = &a; *r = 5;`
4. `let r = &mut a; *r = 5;`
5. `let r = &mut a; *r = 5; r = &mut b;`
6. `let mut r = &mut a; *r = 5; r = &mut b; *r = 7;`
7. `let r = &mut a; let y = &*r; *r += 1; println!("{y}");`
8. `let r = &mut &a; *r = &b;`

<details markdown="1">
<summary><strong>Solution</strong></summary>

Edits 2, 4, 6 and 8 compile and run below; edits 1, 3, 5 and 7 are refused, and the compiler's words for each follow the run.

<!-- source:pg_bind_kata -->
*[`pg_bind_kata.rs`](examples/pg_bind_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: eight one-line edits to `let r = &a;`, each predicted, then
//! run here if it compiles. The four refusals are rustc transcripts on the
//! page; a program cannot print an error it did not get.
//!
//!   rustc --edition 2024 pg_bind_kata.rs -o /tmp/pgbk && /tmp/pgbk

fn main() {
    println!("edit  the line(s) after let a / let b                 verdict");
    println!("1     let r = &a; r = &b;                             E0384: cannot assign twice to immutable variable r");
    {
        let a = 1;
        let b = 2;
        let mut r = &a;
        let first = *r;
        r = &b;
        println!("2     let mut r = &a; r = &b;                         compiles: *r was {first}, now {}", *r);
    }
    println!("3     let r = &a; *r = 5;                             E0594: cannot assign to *r, which is behind a & reference");
    {
        let mut a = 1;
        let r = &mut a;
        *r = 5;
        println!("4     let r = &mut a; *r = 5;                         compiles: a = {a}");
    }
    println!("5     let r = &mut a; *r = 5; r = &mut b;             E0384: the binding is immutable, whatever it points at");
    {
        let mut a = 1;
        let mut b = 2;
        let mut r = &mut a;
        *r = 5;
        r = &mut b;
        *r = 7;
        println!("6     let mut r = &mut a; *r = 5; r = &mut b; *r = 7; compiles: a = {a}, b = {b}");
    }
    println!("7     let r = &mut a; let y = &*r; *r += 1; use y;    E0506: *r is borrowed by y until y's last use");
    {
        let a = 1;
        let b = 2;
        let r = &mut &a;
        *r = &b;
        println!("8     let r = &mut &a; *r = &b;                       compiles: **r reads {}", **r);
    }
}
```
<!-- /source -->

<!-- output:pg_bind_kata -->
*Verified output of [`pg_bind_kata.rs`](examples/pg_bind_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
edit  the line(s) after let a / let b                 verdict
1     let r = &a; r = &b;                             E0384: cannot assign twice to immutable variable r
2     let mut r = &a; r = &b;                         compiles: *r was 1, now 2
3     let r = &a; *r = 5;                             E0594: cannot assign to *r, which is behind a & reference
4     let r = &mut a; *r = 5;                         compiles: a = 5
5     let r = &mut a; *r = 5; r = &mut b;             E0384: the binding is immutable, whatever it points at
6     let mut r = &mut a; *r = 5; r = &mut b; *r = 7; compiles: a = 5, b = 7
7     let r = &mut a; let y = &*r; *r += 1; use y;    E0506: *r is borrowed by y until y's last use
8     let r = &mut &a; *r = &b;                       compiles: **r reads 2
```
<!-- /output -->

Edit 1 is the binding choice: `let`, then a second assignment.

```text title="Abridged — real rustc output for edit_1.rs, one warning dropped"
error[E0384]: cannot assign twice to immutable variable `r`
 --> edit_1.rs:5:5
  |
4 |     let r = &a;
  |         - first assignment to `r`
5 |     r = &b;
  |     ^^^^^^ cannot assign twice to immutable variable
```

Edit 3 is the reference choice: `&a`, then a write through it.

```text title="Abridged — real rustc output for edit_3.rs, one warning dropped"
error[E0594]: cannot assign to `*r`, which is behind a `&` reference
 --> edit_3.rs:4:5
  |
4 |     *r = 5;
  |     ^^^^^^ `r` is a `&` reference, so it cannot be written to
```

Edit 5 is edit 1 again, and the `&mut` does not help — the binding is still `let`.

```text title="Abridged — real rustc output for edit_5.rs"
error[E0384]: cannot assign twice to immutable variable `r`
 --> edit_5.rs:6:5
  |
4 |     let r = &mut a;
  |         - first assignment to `r`
5 |     *r = 5;
6 |     r = &mut b;
  |     ^^^^^^^^^^ cannot assign twice to immutable variable
```

Edit 7 is neither choice; it is the loan the reborrow `&*r` placed on `*r`, still alive at the `println!`.

```text title="Real rustc output for edit_7.rs"
error[E0506]: cannot assign to `*r` because it is borrowed
 --> edit_7.rs:5:5
  |
4 |     let y = &*r;
  |             --- `*r` is borrowed here
5 |     *r += 1;
  |     ^^^^^^^ `*r` is assigned to here but it was already borrowed
6 |     println!("{y}");
  |                - borrow later used here
```

Edit 8 compiles with `let r`, because `*r = &b` re-points the inner reference, not `r`; make it `r = &mut &b` and it is edit 1's `E0384` again.

</details>

## See also

- [Variables](../../../15_First_Programs/variables/README.md#mutability-belongs-to-the-binding-not-to-the-value) — the `mut` on a `let`, and why it belongs to the name
- [Where the `&` sits decides what it does](../../where_the_sigil_sits/README.md) — the `mut` after the `&` is part of a type
- [Borrowing](../../borrowing/README.md) — many readers or one writer, and where a borrow ends
- [Reborrowing](../../reborrowing/README.md) — `&mut *r` at a call site, and `&*r` for the shared kind
- [Borrowed state](../../borrowed_state/README.md) — `E0506` from the owner's side
- [A borrow is a loan](../a_borrow_is_a_loan/README.md) — the loan that `&*z` places on `*z`
- [Re-pointing a slice](../repointing_a_slice/README.md) — `&mut &mut [T]`, the bottom row of the nesting table put to work
- [When you need the `*`](../when_you_need_the_star/README.md) — `**rr` and the five places the `*` is written for you
- [A name is not a place](../../a_name_is_not_a_place/README.md) — `let mut` against a shadowing `let`, proved with the borrow checker
- [Copy or move?](../../copy_or_move/README.md) — `&T` is `Copy`, `&mut T` is not
- [Coercion](../../../29_Conversion/coercion/README.md) — the short list `&mut &vec_2` and `let y2: &i32 = z` are on
- [Every reference error](../reference_errors/README.md#writing-through-a-reference) — `E0596` and `E0594` by symptom
- [What *Rust: The Practical Guide* says about ownership in functions, run](../../ownership_in_functions_claims_checked/README.md) — §4.2 and §4.4 of the same chapter, checked
- [*Rust: The Practical Guide*, run](../../../10_Resources/rust_the_practical_guide/README.md) — the book's home in this library
- [E0384 ↗](https://doc.rust-lang.org/error_codes/E0384.html) · [E0594 ↗](https://doc.rust-lang.org/error_codes/E0594.html) · [E0596 ↗](https://doc.rust-lang.org/error_codes/E0596.html) · [E0506 ↗](https://doc.rust-lang.org/error_codes/E0506.html) · [Type coercions ↗](https://doc.rust-lang.org/reference/type-coercions.html)
- [Sets of sets](../../../26_Collections/sets_of_sets/README.md) — the binding rule as the reason Rust needs no `frozenset`

## Po polsku

`let` albo `let mut` decyduje, czy **nazwę** (*binding*) można skierować na coś innego; `&T` albo `&mut T` decyduje, czy **przez** referencję można pisać. To dwa niezależne wybory, więc kombinacje są cztery, a nie „sześć typów referencji" jak w tabeli 4.1 książki *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025). Ponowne przypisanie do nazwy bez `mut` to `E0384` (*cannot assign twice to immutable variable*), niezależnie od tego, czy trzyma `&T`, czy `&mut T`; zapis przez `&T` to `E0596` (metoda wymagająca `&mut self`) albo `E0594` (`*r = …`), niezależnie od `let` czy `let mut`. Zagnieżdżenia (`&mut &T`, `& &mut T`, `&&T`, `&mut &mut T`) to te same dwa wybory na każdym poziomie: pierwsze `&` na drodze w dół zamraża wszystko poniżej, a `&mut &T` pozwala przekierować referencję **wewnętrzną**, nie samą nazwę. Listing 4.33 kompiluje się dzięki koercji dereferencji (*deref coercion*): `&mut &Vec<i32>` staje się `&Vec<i32>`, a clippy zgłasza `needless_borrow`. `&*z` to pożyczka wtórna (*reborrow*): dopóki `y` jest używane, `*z += 1` to `E0506`.

**Szukaj po polsku:** zmienna mutowalna a referencja mutowalna · `rust let mut vs &mut` · `rust E0384 cannot assign twice` · `rust E0594 behind a & reference` · `rust &mut &T` · pożyczka wtórna `&*z` · wskaźnik na stałą a stały wskaźnik
