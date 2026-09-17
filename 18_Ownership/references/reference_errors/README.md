# Every reference error, and its fix

[References](../README.md) › **Errors, by symptom**

**Level:** 101 → 201 · a reference, by symptom

**One line:** Twenty-six compiler errors a learner meets around `&`, `&mut`, `*`, `ref` and raw pointers — for each, the code that causes it, what rustc 1.98.0 prints, the mistake in one or two sentences, and the fix. The broken code must fail and the fix must compile: [`check_fences.py`](../../../tools/check_fences.py) holds both on every build.

Each transcript is the fence above it, saved under the file name in its title and compiled with `rustc --edition 2024 --crate-type lib`, as the fences are checked; the closing `error: aborting…` and `For more information…` lines are dropped. Transcripts are not regenerated on each commit the way an example's output is, so an upgrade can reword one; the fences, which are checked, cannot silently stop failing.

## Find the error

| # | Code | What rustc says | The mistake |
|---|---|---|---|
| 1 | `E0596` | [cannot borrow `*some_string` as mutable, as it is behind a `&` reference](#1-push_str-through-a-string-parameter) | `push_str` through a `&String` parameter |
| 2 | `E0596` | [cannot borrow `x` as mutable, as it is not declared as mutable](#2-mut-x-on-a-let-without-mut) | `&mut x` on a `let` without `mut` |
| 3 | `E0594` | [cannot assign to `*r`, which is behind a `&` reference](#3-r-2-through-a-i32) | `*r = 2` through a `&i32` |
| 4 | `E0308` | [mismatched types: expected `&mut bool`, found `bool`](#4-slot-true-on-a-mut-bool) | `slot = true` on a `&mut bool` |
| 5 | `E0499` | [cannot borrow `total` as mutable more than once at a time](#5-two-mut-to-one-variable-both-still-used) | Two `&mut` to one variable, both still used |
| 6 | `E0502` | [cannot borrow `v` as mutable because it is also borrowed as immutable](#6-v0-held-across-vpush) | `&v[0]` held across `v.push` |
| 7 | `E0502` | [cannot borrow `v` as mutable because it is also borrowed as immutable](#7-vpush-inside-for-x-in-v) | `v.push` inside `for x in &v` |
| 8 | `E0503` | [cannot use `count` because it was mutably borrowed](#8-reading-a-variable-while-a-mut-to-it-is-live) | Reading a variable while a `&mut` to it is live |
| 9 | `E0506` | [cannot assign to `name` because it is borrowed](#9-assigning-to-a-variable-while-it-is-borrowed) | Assigning to a variable while it is borrowed |
| 10 | `E0505` | [cannot move out of `boxed` because it is borrowed](#10-dropboxed-while-boxed-is-still-used) | `drop(boxed)` while `&*boxed` is still used |
| 11 | `E0507` | [cannot move out of `self.name` which is behind a shared reference](#11-returning-a-string-field-from-a-self-method) | Returning a `String` field from a `&self` method |
| 12 | `E0382` | [borrow of moved value: `r`](#12-let-r2-r-on-a-mut-then-r-again) | `let r2 = r;` on a `&mut`, then `r` again |
| 13 | `E0308` | [mismatched types: expected `bool`, found `&bool`](#13-if-flag-on-a-bool) | `if flag` on a `&bool` |
| 14 | `E0277` | [can't compare `&bool` with `bool`](#14-flag-true-on-a-bool) | `flag == true` on a `&bool` |
| 15 | `E0382` | [use of partially moved value: `maybe_name`](#15-match-on-an-optionstring-then-using-it) | `match` on an `Option<String>`, then using it |
| 16 | no code | [cannot explicitly borrow within an implicitly-borrowing pattern](#16-ref-in-a-pattern-that-already-borrows) | `ref` in a pattern that already borrows |
| 17 | `E0597` | [`x` does not live long enough](#17-a-local-borrowed-in-a-block-read-after-it) | A local borrowed in a block, read after it |
| 18 | `E0515` | [cannot return reference to local variable `text`](#18-returning-a-reference-to-a-local) | Returning a reference to a local |
| 19 | `E0716` | [temporary value dropped while borrowed](#19-a-temporary-borrowed-past-its-statement) | A temporary borrowed past its statement |
| 20 | `E0373` | [closure may outlive the current function, but it borrows `name`, which is owned by the current function](#20-a-closure-for-threadspawn-that-borrows-a-local) | A closure for `thread::spawn` that borrows a local |
| 21 | `E0521` | [borrowed data escapes outside of function](#21-a-borrowed-parameter-moved-into-threadspawn) | A borrowed parameter moved into `thread::spawn` |
| 22 | `E0106` | [missing lifetime specifier](#22-two-reference-parameters-a-reference-returned-no-lifetime) | Two reference parameters, a reference returned, no lifetime |
| 23 | no code | [lifetime may not live long enough](#23-t-mut-t1-on-a-mut-mut-i32) | `*t = &mut t[1..]` on a `&mut &mut [i32]` |
| 24 | `E0133` | [dereference of raw pointer is unsafe and requires unsafe block](#24-reading-through-a-raw-pointer-outside-unsafe) | Reading through a raw pointer outside `unsafe` |
| 25 | no code, the `static_mut_refs` lint | [creating a mutable reference to mutable static](#25-mut-to-a-static-mut) | `&mut` to a `static mut` |
| 26 | `E0782` | [expected a type, found a trait](#26-mut-shape-without-dyn) | `*mut Shape` without `dyn` |

## Writing through a reference

### 1. `push_str` through a `&String` parameter

```rust,compile_fail
pub fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

```text title="rustc 1.98.0 on push_str_through_shared.rs"
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> push_str_through_shared.rs:2:5
  |
2 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
1 | pub fn change(some_string: &mut String) {
  |                             +++
```

**The mistake.** `push_str` takes `&mut self`. A `&String` lends the string for reading, and nothing turns a `&` into a `&mut`.

**The fix.** Ask for a `&mut String`, as rustc suggests. The caller then passes `&mut s` from a `let mut s`.

```rust
pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**Read:** [Borrowing — the one rule](../../borrowing/README.md#the-one-rule), [The Book's Listing 4-6, run](../reference_claims_checked/README.md#the-book-listing-4-6-writing-through-a)

### 2. `&mut x` on a `let` without `mut`

```rust,compile_fail
pub fn bump() -> i32 {
    let x = 1;
    let r = &mut x;
    *r += 1;
    x
}
```

```text title="rustc 1.98.0 on mut_borrow_of_immutable.rs"
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> mut_borrow_of_immutable.rs:3:13
  |
3 |     let r = &mut x;
  |             ^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut x = 1;
  |         +++
```

**The mistake.** A `&mut x` is permission to change `x`, and a binding declared without `mut` has none to lend.

**The fix.** `let mut x`, as the `help:` shows.

```rust
pub fn bump() -> i32 {
    let mut x = 1;
    let r = &mut x;
    *r += 1;
    x
}
```

**Read:** [The `ref` keyword — `ref mut`](../the_ref_keyword/README.md#ref-mut-write-into-the-place-you-matched), [the Stack Overflow pointer snippet, run](../reference_claims_checked/README.md#the-stack-overflow-pointer-snippet)

### 3. `*r = 2` through a `&i32`

```rust,compile_fail
pub fn reset(r: &i32) {
    *r = 2;
}
```

```text title="rustc 1.98.0 on assign_through_shared.rs"
error[E0594]: cannot assign to `*r`, which is behind a `&` reference
 --> assign_through_shared.rs:2:5
  |
2 |     *r = 2;
  |     ^^^^^^ `r` is a `&` reference, so it cannot be written to
  |
help: consider changing this to be a mutable reference
  |
1 | pub fn reset(r: &mut i32) {
  |                  +++
```

**The mistake.** `*r` names the place `r` points at. A shared reference may read that place, never write it.

**The fix.** Take `r: &mut i32`. A value that has to be written while shared needs a type built for it — [interior mutability](../../../09_Advanced/interior_mutability/README.md).

```rust
pub fn reset(r: &mut i32) {
    *r = 2;
}
```

**Read:** [`*r` is a place, not a value](../../where_the_sigil_sits/README.md#r-is-a-place-not-a-value), [Six pointer types — the refusals behind the noes](../pointer_types_compared/README.md#the-refusals-behind-the-noes)

### 4. `slot = true` on a `&mut bool`

```rust,compile_fail
pub fn switch_on(slot: &mut bool) {
    slot = true;
}
```

```text title="rustc 1.98.0 on assign_without_star.rs"
error[E0308]: mismatched types
 --> assign_without_star.rs:2:12
  |
1 | pub fn switch_on(slot: &mut bool) {
  |                        --------- expected due to this parameter type
2 |     slot = true;
  |            ^^^^ expected `&mut bool`, found `bool`
  |
help: consider dereferencing here to assign to the mutably borrowed value
  |
2 |     *slot = true;
  |     +
```

**The mistake.** Without the `*`, the assignment targets the reference `slot` itself, and `true` is not a `&mut bool`.

**The fix.** `*slot = true` writes the place behind it.

```rust
pub fn switch_on(slot: &mut bool) {
    *slot = true;
}
```

**Read:** [When you need the `*` — writing through `&mut bool`](../when_you_need_the_star/README.md#writing-through-mut-bool)

## Two borrows at once

### 5. Two `&mut` to one variable, both still used

```rust,compile_fail
pub fn bump_twice() -> i32 {
    let mut total = 0;
    let a = &mut total;
    let b = &mut total;
    *a += 1;
    *b += 1;
    total
}
```

```text title="rustc 1.98.0 on two_mut_borrows.rs"
error[E0499]: cannot borrow `total` as mutable more than once at a time
 --> two_mut_borrows.rs:4:13
  |
3 |     let a = &mut total;
  |             ---------- first mutable borrow occurs here
4 |     let b = &mut total;
  |             ^^^^^^^^^^ second mutable borrow occurs here
5 |     *a += 1;
  |     ------- first borrow later used here
```

**The mistake.** `a` is used on line 5, so its loan on `total` is still outstanding when line 4 takes a second one.

**The fix.** Finish with `a` before taking `b`. A borrow ends at its last use, not at the closing brace.

```rust
pub fn bump_twice() -> i32 {
    let mut total = 0;
    let a = &mut total;
    *a += 1;
    let b = &mut total;
    *b += 1;
    total
}
```

**Read:** [A borrow is a loan — reading a borrow error](../a_borrow_is_a_loan/README.md#reading-a-borrow-error), [Borrowing — where a borrow ends](../../borrowing/README.md#where-a-borrow-ends-the-part-that-decides-everything)

### 6. `&v[0]` held across `v.push`

```rust,compile_fail
pub fn first_after_push() -> i32 {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    *first
}
```

```text title="rustc 1.98.0 on push_while_first_held.rs"
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> push_while_first_held.rs:4:5
  |
3 |     let first = &v[0];
  |                  - immutable borrow occurs here
4 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     *first
  |     ------ immutable borrow later used here
```

**The mistake.** `v.push(4)` is `Vec::push(&mut v, 4)`: a `&mut` borrow of `v` while `first` still points into it. `push` may move the buffer, which would leave `first` pointing at freed memory.

**The fix.** Copy the element out — `v[0]` is an `i32` — or read `*first` before the `push`.

```rust
pub fn first_after_push() -> i32 {
    let mut v = vec![1, 2, 3];
    let first = v[0];
    v.push(4);
    first
}
```

**Read:** [A borrow is a loan — the model](../a_borrow_is_a_loan/README.md#the-model-a-loan-and-two-conditions), [Borrowing — the bug the rule exists to prevent](../../borrowing/README.md#the-bug-the-rule-exists-to-prevent)

### 7. `v.push` inside `for x in &v`

```rust,compile_fail
pub fn repeat_evens() -> Vec<i32> {
    let mut v = vec![3, 2, 4, 5];
    for x in &v {
        if *x % 2 == 0 {
            v.push(*x);
        }
    }
    v
}
```

```text title="rustc 1.98.0 on push_while_iterating.rs"
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> push_while_iterating.rs:5:13
  |
3 |     for x in &v {
  |              --
  |              |
  |              immutable borrow occurs here
  |              immutable borrow later used here
4 |         if *x % 2 == 0 {
5 |             v.push(*x);
  |             ^^^^^^^^^^ mutable borrow occurs here
```

**The mistake.** The loop's iterator holds `&v` for every turn, which is why both the borrow and its later use are labelled on `&v`; `push` needs `&mut v` in the middle of it.

**The fix.** Decide first, mutate after: collect what to add, then `extend`.

```rust
pub fn repeat_evens() -> Vec<i32> {
    let mut v = vec![3, 2, 4, 5];
    let evens: Vec<i32> = v.iter().filter(|x| **x % 2 == 0).copied().collect();
    v.extend(evens);
    v
}
```

**Read:** [Borrowing — the bug the rule exists to prevent](../../borrowing/README.md#the-bug-the-rule-exists-to-prevent), [Iterator invalidation](../../../31_C_and_Cpp/iterator_invalidation/README.md)

### 8. Reading a variable while a `&mut` to it is live

```rust,compile_fail
pub fn doubled_before_bump() -> i32 {
    let mut count = 1;
    let r = &mut count;
    let doubled = count * 2;
    *r += 1;
    doubled
}
```

```text title="rustc 1.98.0 on use_while_mut_borrowed.rs"
error[E0503]: cannot use `count` because it was mutably borrowed
 --> use_while_mut_borrowed.rs:4:19
  |
3 |     let r = &mut count;
  |             ---------- `count` is borrowed here
4 |     let doubled = count * 2;
  |                   ^^^^^ use of borrowed `count`
5 |     *r += 1;
  |     ------- borrow later used here
```

**The mistake.** `count * 2` copies an `i32` and takes no borrow, so the code is `E0503` rather than `E0502` — but a live `&mut` excludes every other use of the place, reads included.

**The fix.** Read before the `&mut` is taken. Reading through it, `*r * 2`, also compiles.

```rust
pub fn doubled_before_bump() -> i32 {
    let mut count = 1;
    let doubled = count * 2;
    let r = &mut count;
    *r += 1;
    doubled
}
```

**Read:** [Borrowed state — what the lock forbids](../../borrowed_state/README.md#what-the-lock-forbids), [Borrowing — the one rule](../../borrowing/README.md#the-one-rule)

## The owner, while it is lent

### 9. Assigning to a variable while it is borrowed

```rust,compile_fail
pub fn rename() -> String {
    let mut name = String::from("Ada");
    let first = &name;
    name = String::from("Ben");
    format!("{first} then {name}")
}
```

```text title="rustc 1.98.0 on assign_to_borrowed_name.rs"
error[E0506]: cannot assign to `name` because it is borrowed
 --> assign_to_borrowed_name.rs:4:5
  |
3 |     let first = &name;
  |                 ----- `name` is borrowed here
4 |     name = String::from("Ben");
  |     ^^^^ `name` is assigned to here but it was already borrowed
5 |     format!("{first} then {name}")
  |               ----- borrow later used here
```

**The mistake.** Assigning to `name` drops the old `String`, and `first` still points at it.

**The fix.** Take an owned copy when both values are needed afterwards; otherwise finish with `first` before the assignment.

```rust
pub fn rename() -> String {
    let mut name = String::from("Ada");
    let first = name.clone();
    name = String::from("Ben");
    format!("{first} then {name}")
}
```

**Read:** [Borrowed state — what the lock forbids](../../borrowed_state/README.md#what-the-lock-forbids), [Assignment is a drop](../../assignment_is_a_drop/README.md)

### 10. `drop(boxed)` while `&*boxed` is still used

```rust,compile_fail
pub fn read_after_drop() -> i32 {
    let boxed = Box::new(42);
    let reference = &*boxed;
    drop(boxed);
    *reference
}
```

```text title="rustc 1.98.0 on drop_box_early.rs"
error[E0505]: cannot move out of `boxed` because it is borrowed
 --> drop_box_early.rs:4:10
  |
2 |     let boxed = Box::new(42);
  |         ----- binding `boxed` declared here
3 |     let reference = &*boxed;
  |                     ------- borrow of `*boxed` occurs here
4 |     drop(boxed);
  |          ^^^^^ move out of `boxed` occurs here
5 |     *reference
  |     ---------- borrow later used here
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 -     let reference = &*boxed;
3 +     let reference = &boxed.clone();
  |
```

**The mistake.** `reference` points into the box's heap allocation, and moving `boxed` into `drop` frees it before `*reference` is read.

**The fix.** Read what you need before the drop. rustc's `help:` clones the whole `Box` to have something else to borrow; applied as printed it is `E0308`, because `*reference` is now a `Box`, and it compiles only once line 5 becomes `**reference`.

```rust
pub fn read_after_drop() -> i32 {
    let boxed = Box::new(42);
    let reference = &*boxed;
    let value = *reference;
    drop(boxed);
    value
}
```

**Read:** [Six pointer types — `Box<T>`](../pointer_types_compared/README.md#boxt-is-neither-always-one-pointer-wide-nor-only-a-pointer), [Borrowed state](../../borrowed_state/README.md#what-the-lock-forbids)

## A move where you meant a borrow

### 11. Returning a `String` field from a `&self` method

```rust,compile_fail
pub struct User {
    pub name: String,
}

impl User {
    pub fn name(&self) -> String {
        self.name
    }
}
```

```text title="rustc 1.98.0 on move_field_out_of_self.rs"
error[E0507]: cannot move out of `self.name` which is behind a shared reference
 --> move_field_out_of_self.rs:7:9
  |
7 |         self.name
  |         ^^^^^^^^^ move occurs because `self.name` has type `String`, which does not implement the `Copy` trait
  |
help: consider cloning the value if the performance cost is acceptable
  |
7 |         self.name.clone()
  |                  ++++++++
```

**The mistake.** `&self` lends the whole `User`, and returning `self.name` would move the `String` out of it. The same code answers `let s = *r;` for `r: &String` — *cannot move out of `*r` which is behind a shared reference* — and `names[0]` for `names: &Vec<String>` — *cannot move out of index of `Vec<String>`*.

**The fix.** Lend it on: return `&str`. When the caller needs its own `String`, `self.name.clone()`, as rustc suggests.

```rust
pub struct User {
    pub name: String,
}

impl User {
    pub fn name(&self) -> &str {
        &self.name
    }
}
```

**Read:** [Where the `&` sits — the read that is refused](../../where_the_sigil_sits/README.md#the-read-that-is-refused), [Ownership and moves — getting one out](../../ownership_and_moves/README.md#getting-a-second-one-or-getting-one-out)

### 12. `let r2 = r;` on a `&mut`, then `r` again

```rust,compile_fail
fn bump(n: &mut i32) {
    *n += 1;
}

pub fn bump_after_let() -> i32 {
    let mut total = 0;
    let r = &mut total;
    let r2 = r;
    bump(r2);
    bump(r);
    total
}
```

```text title="rustc 1.98.0 on mut_ref_moved_by_let.rs"
error[E0382]: borrow of moved value: `r`
  --> mut_ref_moved_by_let.rs:10:10
   |
 7 |     let r = &mut total;
   |         - move occurs because `r` has type `&mut i32`, which does not implement the `Copy` trait
 8 |     let r2 = r;
   |              - value moved here
 9 |     bump(r2);
10 |     bump(r);
   |          ^ value borrowed here after move
```

**The mistake.** `&mut i32` is not `Copy`, so `let r2 = r;` moves it. Only a place that expects a `&mut i32`, such as the argument of `bump`, reborrows instead.

**The fix.** Reborrow: `&mut *r`. `let r2: &mut i32 = r;` reborrows as well. Either way `r` works again after `r2`'s last use.

```rust
fn bump(n: &mut i32) {
    *n += 1;
}

pub fn bump_after_let() -> i32 {
    let mut total = 0;
    let r = &mut total;
    let r2 = &mut *r;
    bump(r2);
    bump(r);
    total
}
```

**Read:** [Reborrowing — the `let` that does not do it](../../reborrowing/README.md#the-let-that-does-not-do-it)

## Comparing and testing through a reference

### 13. `if flag` on a `&bool`

```rust,compile_fail
pub fn describe(flag: &bool) -> &'static str {
    if flag { "on" } else { "off" }
}
```

```text title="rustc 1.98.0 on if_on_ref_bool.rs"
error[E0308]: mismatched types
 --> if_on_ref_bool.rs:2:8
  |
2 |     if flag { "on" } else { "off" }
  |        ^^^^ expected `bool`, found `&bool`
  |
help: consider dereferencing the borrow
  |
2 |     if *flag { "on" } else { "off" }
  |        +
```

**The mistake.** A condition takes exactly `bool`, and no trait or coercion turns a `&bool` into one. `!flag` compiles on the same `&bool`, because std implements `Not` for `&bool`.

**The fix.** `*flag`.

```rust
pub fn describe(flag: &bool) -> &'static str {
    if *flag { "on" } else { "off" }
}
```

**Read:** [When you need the `*` — a condition wants exactly `bool`](../when_you_need_the_star/README.md#a-condition-wants-exactly-bool)

### 14. `flag == true` on a `&bool`

```rust,compile_fail
pub fn is_on(flag: &bool) -> bool {
    flag == true
}
```

```text title="rustc 1.98.0 on ref_bool_eq_true.rs"
error[E0277]: can't compare `&bool` with `bool`
 --> ref_bool_eq_true.rs:2:10
  |
2 |     flag == true
  |          ^^ no implementation for `&bool == bool`
  |
  = help: the trait `PartialEq<bool>` is not implemented for `&bool`
help: consider dereferencing here
  |
2 |     *flag == true
  |     +
```

**The mistake.** std compares a reference with a reference and a `bool` with a `bool`; nothing compares `&bool` with `bool`. The same slip with `>` on a `&i32`, `n > 1`, is a different code: `E0308`, *expected `&i32`, found integer*.

**The fix.** `*flag` — comparing a `bool` with `true` gives back the `bool`. `*flag == true` and `flag == &true` compile too.

```rust
pub fn is_on(flag: &bool) -> bool {
    *flag
}
```

**Read:** [When you need the `*` — `==` wants both sides at the same depth](../when_you_need_the_star/README.md#wants-both-sides-at-the-same-depth)

## Patterns

### 15. `match` on an `Option<String>`, then using it

```rust,compile_fail
pub fn greet(maybe_name: Option<String>) -> String {
    match maybe_name {
        Some(n) => println!("Hello, {n}"),
        None => println!("Hello, world"),
    }
    maybe_name.unwrap_or_default()
}
```

```text title="rustc 1.98.0 on match_moves_payload.rs"
error[E0382]: use of partially moved value: `maybe_name`
 --> match_moves_payload.rs:6:5
  |
3 |         Some(n) => println!("Hello, {n}"),
  |              - value partially moved here
...
6 |     maybe_name.unwrap_or_default()
  |     ^^^^^^^^^^ value used here after partial move
  |
  = note: partial move occurs because value has type `String`, which does not implement the `Copy` trait
help: borrow this binding in the pattern to avoid moving the value
  |
3 |         Some(ref n) => println!("Hello, {n}"),
  |              +++
```

**The mistake.** `Some(n)` binds by value, so the `String` moves out of `maybe_name` into `n`, and after the `match` only part of the `Option` is left.

**The fix.** Match on `&maybe_name`, and every binding borrows: `n: &String`. rustc's `help:`, `Some(ref n)`, compiles too.

```rust
pub fn greet(maybe_name: Option<String>) -> String {
    match &maybe_name {
        Some(n) => println!("Hello, {n}"),
        None => println!("Hello, world"),
    }
    maybe_name.unwrap_or_default()
}
```

**Read:** [The `ref` keyword — without `ref`: a partial move](../the_ref_keyword/README.md#without-ref-a-partial-move), [Match ergonomics](../../../30_Pattern_Matching/match_ergonomics/README.md)

### 16. `ref` in a pattern that already borrows

```rust,compile_fail
pub fn name_len(maybe_name: &Option<String>) -> usize {
    match maybe_name {
        Some(ref n) => n.len(),
        None => 0,
    }
}
```

```text title="rustc 1.98.0 on ref_in_ref_match.rs"
error: cannot explicitly borrow within an implicitly-borrowing pattern
 --> ref_in_ref_match.rs:3:14
  |
3 |         Some(ref n) => n.len(),
  |              ^^^ explicit `ref` binding modifier not allowed when implicitly borrowing
  |
  = note: for more information, see <https://doc.rust-lang.org/reference/patterns.html#binding-modes>
note: matching on a reference type with a non-reference pattern implicitly borrows the contents
 --> ref_in_ref_match.rs:3:9
  |
3 |         Some(ref n) => n.len(),
  |         ^^^^^^^^^^^ this non-reference pattern matches on a reference type `&_`
help: remove the unnecessary binding modifier
  |
3 -         Some(ref n) => n.len(),
3 +         Some(n) => n.len(),
  |
```

**The mistake.** `maybe_name` is a `&Option<String>`, so `Some(n)` already binds `n: &String`. Edition 2021 accepts the extra `ref`; edition 2024 refuses it, with no error code.

**The fix.** Delete the `ref`.

```rust
pub fn name_len(maybe_name: &Option<String>) -> usize {
    match maybe_name {
        Some(n) => n.len(),
        None => 0,
    }
}
```

**Read:** [The `ref` keyword — edition 2024](../the_ref_keyword/README.md#edition-2024-ref-where-the-borrow-is-already-implied)

## A borrow that outlives its value

### 17. A local borrowed in a block, read after it

```rust,compile_fail
pub fn read_after_block() -> i32 {
    let r;
    {
        let x = 1;
        r = &x;
    }
    *r
}
```

```text title="rustc 1.98.0 on read_after_block.rs"
error[E0597]: `x` does not live long enough
 --> read_after_block.rs:5:13
  |
4 |         let x = 1;
  |             - binding `x` declared here
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     *r
  |     -- borrow later used here
```

**The mistake.** `x` ends at the `}` and `r` is read after it. `x` is an `i32` with no destructor, and rustc still says *dropped*: the end of its storage is the use that breaks the loan.

**The fix.** Declare `x` where it outlives every read of `r` — here, before the block.

```rust
pub fn read_after_block() -> i32 {
    let r;
    let x = 1;
    {
        r = &x;
    }
    *r
}
```

**Read:** [A borrow is a loan — going out of scope is a use too](../a_borrow_is_a_loan/README.md#going-out-of-scope-is-a-use-too), [What `&'a T` claims](../../what_a_reference_claims/README.md#what-may-be-assigned-into-a-reference)

### 18. Returning a reference to a local

```rust,compile_fail
pub fn greeting(name: &str) -> &str {
    let text = format!("Hello, {name}");
    &text
}
```

```text title="rustc 1.98.0 on return_ref_to_local.rs"
error[E0515]: cannot return reference to local variable `text`
 --> return_ref_to_local.rs:3:5
  |
3 |     &text
  |     ^^^^^ returns a reference to data owned by the current function
```

**The mistake.** `text` is freed when the function returns, so no reference to it can leave. The signature is accepted — elision ties the result to `name` — and the body is refused.

**The fix.** Return the `String` itself.

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello, {name}")
}
```

**Read:** [A stack slot is reused — what Rust does instead](../../a_stack_slot_is_reused/README.md#what-rust-does-instead)

### 19. A temporary borrowed past its statement

```rust,compile_fail
pub fn word_count() -> usize {
    let words: Vec<&str> = String::from("a b c").split(' ').collect();
    words.len()
}
```

```text title="rustc 1.98.0 on temporary_dropped.rs"
error[E0716]: temporary value dropped while borrowed
 --> temporary_dropped.rs:2:28
  |
2 |     let words: Vec<&str> = String::from("a b c").split(' ').collect();
  |                            ^^^^^^^^^^^^^^^^^^^^^                     - temporary value is freed at the end of this statement
  |                            |
  |                            creates a temporary value which is freed while still in use
3 |     words.len()
  |     ----- borrow later used here
  |
help: consider using a `let` binding to create a longer lived value
  |
2 ~     let binding = String::from("a b c");
3 ~     let words: Vec<&str> = binding.split(' ').collect();
  |
```

**The mistake.** Nothing owns `String::from("a b c")`, so it is dropped at the `;`, and `words` holds `&str`s into it.

**The fix.** Give the `String` a name on its own line, as the `help:` does; it then lives to the end of the block.

```rust
pub fn word_count() -> usize {
    let line = String::from("a b c");
    let words: Vec<&str> = line.split(' ').collect();
    words.len()
}
```

**Read:** [Temporary lifetime extension — the rule](../../temporary_lifetimes/README.md#the-rule)

### 20. A closure for `thread::spawn` that borrows a local

```rust,compile_fail
use std::thread;

pub fn length_in_thread() -> usize {
    let name = String::from("Ada");
    let handle = thread::spawn(|| name.len());
    handle.join().unwrap()
}
```

```text title="rustc 1.98.0 on closure_borrows_into_thread.rs"
error[E0373]: closure may outlive the current function, but it borrows `name`, which is owned by the current function
 --> closure_borrows_into_thread.rs:5:32
  |
5 |     let handle = thread::spawn(|| name.len());
  |                                ^^ ---- `name` is borrowed here
  |                                |
  |                                may outlive borrowed value `name`
  |
note: function requires argument type to outlive `'static`
 --> closure_borrows_into_thread.rs:5:18
  |
5 |     let handle = thread::spawn(|| name.len());
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `name` (and any other referenced variables), use the `move` keyword
  |
5 |     let handle = thread::spawn(move || name.len());
  |                                ++++
```

**The mistake.** The closure captures `&name`, and `thread::spawn` requires `'static`: the thread may outlive the function that owns `name`.

**The fix.** `move` the `String` into the closure.

```rust
use std::thread;

pub fn length_in_thread() -> usize {
    let name = String::from("Ada");
    let handle = thread::spawn(move || name.len());
    handle.join().unwrap()
}
```

**Read:** [The `move` keyword — the two errors that ask for it](../../../23_Closures/the_move_keyword/README.md#the-two-errors-that-ask-for-it), [Spawning a thread — `scope`](../../../09_Advanced/spawning_a_thread/README.md#scope-is-the-other-answer-and-usually-the-better-one)

### 21. A borrowed parameter moved into `thread::spawn`

```rust,compile_fail
use std::thread;

pub fn print_in_thread(name: &str) {
    let handle = thread::spawn(move || println!("{name}"));
    handle.join().unwrap();
}
```

```text title="rustc 1.98.0 on borrowed_param_into_thread.rs"
error[E0521]: borrowed data escapes outside of function
 --> borrowed_param_into_thread.rs:4:18
  |
3 | pub fn print_in_thread(name: &str) {
  |                        ----  - let's call the lifetime of this reference `'1`
  |                        |
  |                        `name` is a reference that is only valid in the function body
4 |     let handle = thread::spawn(move || println!("{name}"));
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |                  |
  |                  `name` escapes the function body here
  |                  argument requires that `'1` must outlive `'static`
```

**The mistake.** `move` is already there, and what it moves is `name` — a `&str` borrowed for `'1`. The closure still holds a borrow, and `thread::spawn` still needs `'static`.

**The fix.** Move an owned copy in. Or keep the borrow and use `thread::scope`: `thread::scope(|s| { s.spawn(|| println!("{name}")); });` compiles with `name: &str`.

```rust
use std::thread;

pub fn print_in_thread(name: &str) {
    let name = name.to_owned();
    let handle = thread::spawn(move || println!("{name}"));
    handle.join().unwrap();
}
```

**Read:** [The `move` keyword](../../../23_Closures/the_move_keyword/README.md#the-two-errors-that-ask-for-it), [Spawning a thread — `scope`](../../../09_Advanced/spawning_a_thread/README.md#scope-is-the-other-answer-and-usually-the-better-one)

## Lifetimes in signatures

### 22. Two reference parameters, a reference returned, no lifetime

```rust,compile_fail
pub fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}
```

```text title="rustc 1.98.0 on longest_no_lifetime.rs"
error[E0106]: missing lifetime specifier
 --> longest_no_lifetime.rs:1:37
  |
1 | pub fn longest(a: &str, b: &str) -> &str {
  |                   ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
  |
1 | pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
  |               ++++     ++          ++          ++
```

**The mistake.** With two input references, elision cannot say which one the result borrows from, and rustc does not read the body to find out.

**The fix.** Name the relationship, as the `help:` writes it. `'a` makes nothing live longer: at a call it is the shorter of the two borrows.

```rust
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

**Read:** [Lifetime annotations — the error that asks for one](../../lifetime_annotations/README.md#the-error-that-asks-for-one), [it grants nothing](../../lifetime_annotations/README.md#it-grants-nothing)

### 23. `*t = &mut t[1..]` on a `&mut &mut [i32]`

```rust,compile_fail
pub fn skip_first(t: &mut &mut [i32]) {
    *t = &mut t[1..];
}
```

```text title="rustc 1.98.0 on repoint_through_mut.rs"
error: lifetime may not live long enough
 --> repoint_through_mut.rs:2:5
  |
1 | pub fn skip_first(t: &mut &mut [i32]) {
  |                      -    - let's call the lifetime of this reference `'2`
  |                      |
  |                      let's call the lifetime of this reference `'1`
2 |     *t = &mut t[1..];
  |     ^^^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
  |
help: consider introducing a named lifetime parameter
  |
1 | pub fn skip_first<'a>(t: &'a mut &'a mut [i32]) {
  |                  ++++     ++      ++
```

**The mistake.** `&mut t[1..]` is reborrowed through `t`, so it lasts only `'1`, the outer borrow. `*t` is the caller's slice reference and needs something that lasts `'2`.

**The fix.** `mem::take` moves the caller's slice reference out of `*t`, and the re-slice borrows from that directly. rustc's `help:` — one `'a` on both layers — compiles here and breaks a caller that reads its slice afterwards.

```rust
pub fn skip_first(t: &mut &mut [i32]) {
    let whole = std::mem::take(t);
    *t = &mut whole[1..];
}
```

**Read:** [Re-pointing a slice — why not `*t = &mut t[1..]`](../repointing_a_slice/README.md#why-not-t-mut-t1), [rustc's `help:` compiles, and breaks the caller](../repointing_a_slice/README.md#rustcs-help-compiles-and-breaks-the-caller)

## Raw pointers and statics

### 24. Reading through a raw pointer outside `unsafe`

```rust,compile_fail
pub fn read_back() -> i32 {
    let x = 5;
    let p: *const i32 = &x;
    *p
}
```

```text title="rustc 1.98.0 on deref_raw_outside_unsafe.rs"
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> deref_raw_outside_unsafe.rs:4:5
  |
4 |     *p
  |     ^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
```

**The mistake.** Making `p` is safe. Reading through it is not, because nothing checks that it still points at a live `i32`.

**The fix.** An `unsafe` block, with a `// SAFETY:` comment saying why the read is sound. Here a `&i32` would need neither.

```rust
pub fn read_back() -> i32 {
    let x = 5;
    let p: *const i32 = &x;
    // SAFETY: `p` points at `x`, which is alive and not written to.
    unsafe { *p }
}
```

**Read:** [What `unsafe` turns off — making a raw pointer is safe](../../../09_Advanced/what_unsafe_turns_off/README.md#making-a-raw-pointer-is-safe-using-one-is-not), [Where the `&` sits — the `*` that never dereferences](../../where_the_sigil_sits/README.md#the-that-never-dereferences)

### 25. `&mut` to a `static mut`

```rust,compile_fail
static mut COUNT: u32 = 0;

pub fn bump() -> u32 {
    unsafe {
        let count = &mut COUNT;
        *count += 1;
        *count
    }
}
```

```text title="rustc 1.98.0 on mut_ref_to_static_mut.rs"
error: creating a mutable reference to mutable static
 --> mut_ref_to_static_mut.rs:5:21
  |
5 |         let count = &mut COUNT;
  |                     ^^^^^^^^^^ mutable reference to mutable static
  |
  = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives
  = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
  = note: `#[deny(static_mut_refs)]` (part of `#[deny(rust_2024_compatibility)]`) on by default
help: use `&raw mut` instead to create a raw pointer
  |
5 |         let count = &raw mut COUNT;
  |                      +++
```

**The mistake.** A `&mut` promises no other access while it lives, and nothing can check that for a global. Edition 2024 makes `static_mut_refs` deny-by-default, so this is an error with no code; under `--edition 2021` it is a warning.

**The fix.** A type that says how it is shared: an `AtomicU32` needs no `unsafe`. rustc's `help:`, `&raw mut COUNT`, compiles and keeps the hazard.

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static COUNT: AtomicU32 = AtomicU32::new(0);

pub fn bump() -> u32 {
    COUNT.fetch_add(1, Ordering::Relaxed) + 1
}
```

**Read:** [`const` and `static` — the trap: `static mut`](../../../27_Modules/const_and_static/README.md#the-trap-static-mut), [*Learn Rust the Dangerous Way*, run](../reference_claims_checked/README.md#learn-rust-the-dangerous-way-a-mutable-reference-to-a-static-mut)

### 26. `*mut Shape` without `dyn`

```rust,compile_fail
pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Canvas {
    pub current: *mut Shape,
}
```

```text title="rustc 1.98.0 on pointer_without_dyn.rs"
error[E0782]: expected a type, found a trait
 --> pointer_without_dyn.rs:6:23
  |
6 |     pub current: *mut Shape,
  |                       ^^^^^
  |
help: you can add the `dyn` keyword if you want a trait object
  |
6 |     pub current: *mut dyn Shape,
  |                       +++
```

**The mistake.** `Shape` is a trait, not a type. A pointer to any value that implements it is a pointer to a trait object, written `dyn Shape`; from `--edition 2021` on, the bare name is an error.

**The fix.** `*mut dyn Shape`.

```rust
pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Canvas {
    pub current: *mut dyn Shape,
}
```

**Read:** [`*mut SomeTrait`, run](../reference_claims_checked/README.md#mut-sometrait), [Static vs dynamic dispatch](../../../12_Traits/static_vs_dynamic_dispatch/README.md)

## See also

- [References](../README.md) — the path these errors are stations on
- [A borrow is a loan](../a_borrow_is_a_loan/README.md) — how to read the labelled lines of errors 5 to 10 and 17
- [When you need the `*`](../when_you_need_the_star/README.md) · [The `ref` keyword](../the_ref_keyword/README.md) · [Re-pointing a slice](../repointing_a_slice/README.md) · [Six pointer types, one table](../pointer_types_compared/README.md) — the pages in this folder the entries link to
- [What reference explanations get wrong, run](../reference_claims_checked/README.md) — the snippets from The Book, Stack Overflow and *Learn Rust the Dangerous Way* behind errors 1, 2, 25 and 26
- [Lints around references](../reference_lints/README.md) — the warnings, where this page is the refusals
- [Helpful resources](../references_reading_list/README.md) — chapters, docs and articles for each part of the path
- [ERRORS.md](../../../ERRORS.md) — every error code the library explains, by code

## Po polsku

Dwadzieścia sześć błędów kompilatora wokół `&`, `&mut`, `*`, `ref` i surowych wskaźników: dla każdego kod, który go wywołuje, komunikat rustc 1.98.0, na czym polega pomyłka i poprawka. Kod z błędem musi się nie kompilować, a poprawka musi się kompilować — sprawdza to `check_fences.py` przy każdym budowaniu. Tabela na górze pozwala znaleźć błąd po kodzie (`E0499`, `E0502`, `E0597`…) albo po treści komunikatu; trzy błędy nie mają kodu — dwa z edycji 2024 i *lifetime may not live long enough*.

Najczęstsze pułapki: `push_str` przez `&String` (potrzebny `&mut String`), `&v[0]` trzymane w czasie `v.push`, zwracanie referencji do zmiennej lokalnej, `if flag` na `&bool` (trzeba `*flag`), `match` na `Option<String>` bez `&` (wartość zostaje przeniesiona) oraz domknięcie w `thread::spawn`, które pożycza zmienną lokalną — wtedy pomaga `move`.

**Szukaj po polsku:** `rust E0502 cannot borrow as mutable because it is also borrowed as immutable` · `rust E0597 does not live long enough` · `rust E0507 cannot move out of behind a shared reference` · pożyczanie · referencja mutowalna · dereferencja
