# What *Rust: The Practical Guide* says about ownership in functions, run

**Level:** 101 → 201 · a companion to [Ownership and moves](../ownership_and_moves/README.md) and [Borrowing](../borrowing/README.md)

**One line:** Chapter 4 of *Rust: The Practical Guide* sorts functions into three groups — taking, giving, and taking-and-returning ownership — and then replaces the third with borrowing. Its listings compile or fail as it says on rustc 1.98.0, with one error message misquoted. Three statements around them do not hold: a move does not move the vector's data, "stack-only" is not why an integer is copied, and Listing 4.25's error is a signature error the borrow checker never saw. Five more are true but narrower than they read.

```rust
fn takes_ownership(vec: Vec<i32>) {
    println!("vec is: {:?}", vec);                  // vec is: [1, 2, 3]
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6]
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("vec is: {:?}", vec);                  // vec is: [1, 2, 3, 10]
}

fn main() {
    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1.clone());                 // Listing 4.4: the clone goes, vec_1 stays
    let vec_2 = gives_ownership();                  // Listing 4.5
    let vec_1 = takes_and_gives_ownership(vec_1);   // Listing 4.22: shadowed on the way back
    borrows_vec(&vec_1);                            // Listing 4.17: lent, not moved
    println!("{:?} {:?}", vec_1, vec_2);            // [1, 2, 3, 10] [4, 5, 6]
}
```

## Where it comes from

*Rust: The Practical Guide* by Nouman Azam (Rheinwerk Computing, 2025, 627 pp.). Chapter 4 is ownership. §4.2, "Ownership in Functions", has the three groups and a sidebar on stack-only types (Listings 4.3 to 4.7); §4.4, "Borrowing in Functions", rewrites them with references (Listings 4.16 to 4.25). §4.6, on mutable bindings against mutable references, is its own lesson here, [Mutable binding, mutable reference](../references/mutable_binding_vs_mutable_reference/README.md); the chapter's exercises are run on [Chapter 4 exercises](../../10_Resources/rust_the_practical_guide/ch4_katas/README.md). The book's home in this library is [*Rust: The Practical Guide*, run](../../10_Resources/rust_the_practical_guide/README.md).

The listings above are the book's, joined into one program. The chapter's functions are written with `&Vec<i32>` parameters, which this page keeps so that the transcripts match; [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) is where `&[i32]` and `&str` become the parameter types to write.

## What the listings do on 1.98.0

| Listing | The book says | rustc 1.98.0 |
|---|---|---|
| 4.3 `takes_ownership(vec_1)`, then `vec_1` | error, "borrowed of a moved value" | `E0382`, *borrow of moved value: `vec_1`* |
| 4.4 `takes_ownership(vec_1.clone())` | compiles | compiles, no warnings |
| 4.5, 4.6 give, take and give | compile | compile; `[4, 5, 6]` and `[1, 2, 3, 10]` |
| 4.7 the sidebar's `stack_function` | prints 56, then 10 | prints 56, then 10, and warns *value passed to `var` is never read* |
| 4.16, 4.17, 4.18 by value, then by `&` | compile | compile, no warnings |
| 4.19 `&mut vec_1` while `ref1` is still to be used | error | `E0502`, plus *unused variable: `ref2`* |
| 4.20 `borrows_vec(ref1)` before `&mut vec_1` | compiles | compiles, with *unused variable: `ref2`* |
| 4.21 `takes_and_gives_ownership(vec_1)` unassigned | error | `E0382`, plus *variable does not need to be mutable* on `vec_1` |
| 4.22 `let vec_1 = takes_and_gives_ownership(vec_1)` | compiles | compiles, with the same `unused_mut` warning |
| 4.23 `mutably_borrows_vec(&mut vec_1)` | compiles | compiles, no warnings |
| 4.24 `gives_onwership` | compiles | compiles; the `dead_code` warning names the typo |
| 4.25 `-> &Vec<i32>` returning `&vec` | error, missing lifetime specifier | `E0106`; with `'static` written in, `E0515` |

## Statements that do not hold

**1. Moving the vector into a function "involves moving the entire vector's data to the function's scope, which incurs the overhead of transferring ownership … resulting in unnecessary data movement" (Listing 4.22).** A move copies the vector's three-word header — pointer, length, capacity — and nothing else. Block 3 of the run below passes a vector through `takes_and_gives_ownership` and finds the same heap pointer before the call, inside it and after it, and zero allocations. What costs is the clone the section prefers this to: one allocation, sixteen bytes for four `i32`s, and a different heap pointer. [What an address shows](../what_an_address_shows/README.md) is the same measurement on a `String`, [Stack and heap](../stack_and_heap/README.md) prices move, `Copy`, clone and `Arc::clone` in one table, and [What a clone costs](../what_a_clone_costs/README.md) adds up the fields. The `&mut` of Listing 4.23 is the better shape, but not because it moves fewer bytes on the heap — both move none (block 7). It is better because the caller keeps its name and the callee cannot forget to hand the vector back.

**2. "Stack-only data types, such as integers, floats, bools, and chars, are copied and not moved … heap-allocated types like vectors" are moved.** Where the bytes live is not the criterion; the [`Copy` trait](../copy_or_move/README.md) is. `struct Plain { x: i32 }` is four bytes on the stack and it moves:

```rust,compile_fail
// plain_struct_moves.rs
struct Plain {
    x: i32,
}

fn main() {
    let a = Plain { x: 1 };
    let b = a;
    println!("{} {}", a.x, b.x);
}
```

```text title="Abridged — real rustc output for plain_struct_moves.rs"
error[E0382]: borrow of moved value: `a`
 --> plain_struct_moves.rs:8:23
  |
6 |     let a = Plain { x: 1 };
  |         - move occurs because `a` has type `Plain`, which does not implement the `Copy` trait
7 |     let b = a;
  |             - value moved here
8 |     println!("{} {}", a.x, b.x);
  |                       ^^^ value borrowed here after move
```

Add `#[derive(Clone, Copy)]` and the same four bytes copy — block 4 runs that `Flat` beside `Plain`. In the other direction, a `&String` points at heap data and is `Copy`, and so is `[u8; 3]`. The compiler says which rule it applied in the message above, and it says it as an absence: [There is no `Move` trait](../no_move_trait/README.md).

**3. Listing 4.25 fails "because we're violating borrowing rule 2, which states that references must always remain valid".** The refusal is `E0106`, *missing lifetime specifier*, and it comes from type checking the signature `-> &Vec<i32>`, which names a reference with nothing to borrow from. The borrow checker has not run; the body has not been looked at:

```rust,compile_fail
// listing_4_25.rs
fn main() {}

fn gives_ownership() -> &Vec<i32> { // Error
    let vec = vec![4, 5, 6];
    &vec
}
```

```text title="Abridged — real rustc output for listing_4_25.rs"
error[E0106]: missing lifetime specifier
 --> listing_4_25.rs:3:25
  |
3 | fn gives_ownership() -> &Vec<i32> { // Error
  |                         ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
3 | fn gives_ownership() -> &'static Vec<i32> { // Error
  |                          +++++++
help: instead, you are more likely to want to return an owned value
  |
3 - fn gives_ownership() -> &Vec<i32> { // Error
3 + fn gives_ownership() -> Vec<i32> { // Error
```

Take the first `help` and the borrow checker gets its turn. Then it says what the book meant:

```text title="Abridged — real rustc output for listing_4_25_static.rs, the signature changed to -> &'static Vec<i32>"
error[E0515]: cannot return reference to local variable `vec`
 --> listing_4_25_static.rs:5:5
  |
5 |     &vec
  |     ^^^^ returns a reference to data owned by the current function
```

That is the dangling reference — `vec` is dropped at the `}`, and [going out of scope is a use](../references/a_borrow_is_a_loan/README.md#going-out-of-scope-is-a-use-too) that the loan cannot survive. The book's rule of thumb holds: a value made inside a function is returned by value, and [Returned by value](../returned_by_value/README.md) is what the caller reserves for it. Block 8 shows the heap bytes built inside `gives_ownership` arriving in `main` untouched.

**4. The error is "borrowed of a moved value".** rustc's words are *borrow of moved value*:

```text title="Abridged — real rustc output for listing_4_3.rs"
error[E0382]: borrow of moved value: `vec_1`
 --> listing_4_3.rs:4:32
  |
2 |     let vec_1 = vec![1, 2, 3];
  |         ----- move occurs because `vec_1` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     takes_ownership(vec_1);
  |                     ----- value moved here
4 |     println!("vec 1 is: {:?}", vec_1); // Error
  |                                ^^^^^ value borrowed here after move
  |
note: consider changing this parameter type in function `takes_ownership` to borrow instead if owning the value isn't necessary
```

The `note` is the whole of §4.4 in one line, and the `help` under it (dropped above) is Listing 4.4's `.clone()`.

**5. Listing 4.20 "compiles since mutable and immutable do not coexist within the same scope".** They do coexist in the same scope: `ref1` and `ref2` are declared in the same block of `main`, two lines apart. What may not coexist is two *live* borrows, and a borrow is live from where it is made to its last use — [where a borrow ends](../borrowing/README.md#where-a-borrow-ends-the-part-that-decides-everything). Block 6 is Listing 4.20 with `ref2` used. The book's own explanation of Listing 4.19 has it right — `ref1`'s reach "extends from the line in which it is defined until the line in which it is passed to the function" — and the transcript labels that line:

```text title="Abridged — real rustc output for listing_4_19.rs, one warning dropped"
error[E0502]: cannot borrow `vec_1` as mutable because it is also borrowed as immutable
 --> listing_4_19.rs:4:16
  |
3 |     let ref1 = &vec_1;
  |                ------ immutable borrow occurs here
4 |     let ref2 = &mut vec_1; // Error
  |                ^^^^^^^^^^ mutable borrow occurs here
5 |     borrows_vec(ref1);
  |                 ---- immutable borrow later used here
```

[A borrow is a loan](../references/a_borrow_is_a_loan/README.md#reading-a-borrow-error) reads those three labelled lines as the loan, the conflicting use, and the later use that kept the loan alive.

## The claims, run

<!-- output:pg_own_claims -->
*Verified output of [`pg_own_claims.rs`](examples/pg_own_claims.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The three groups, run (Listings 4.5, 4.6 and 4.17)
   gives_ownership()                    -> [4, 5, 6]
   takes_and_gives_ownership(vec_1)     -> [1, 2, 3, 10]
   borrows_vec: vec is [1, 2, 3, 10]
   after borrows_vec(&vec_1), vec_1 is still [1, 2, 3, 10]
   takes_ownership: vec is [1, 2, 3, 10]
   after takes_ownership(vec_1), any use of vec_1 is E0382 (Listing 4.3)

2. "At the end of the function, the variable will be dropped" — when, exactly
   takes_loud holds A
   drop(A)
   back in main

3. Listing 4.22: "moving the entire vector's data" — measured
   a Vec<i32> is 3 words on the stack: pointer, length, capacity
   heap pointer before == inside == after the move: true true
   allocations during the move there and back: 0 (0 bytes)
   allocations during vec_1.clone() (Listing 4.4): 1 (16 bytes); clone shares the heap: false
   allocations during a borrow, &vec_1 (Listing 4.17): 0 (len read: 4)

4. "Stack-only data types are copied": the criterion is Copy, not the stack
   Flat { x: 1 }, 4 bytes, derive(Clone, Copy): a used after let b = a: a.x = 1, b.x = 1
   Plain { x: 1 }, 4 bytes, no derive: p.x = 1 — let q = p; then p.x is E0382 (transcript on the page)
   [u8; 3] is Copy: arr [1, 2, 3] still usable beside arr2 [1, 2, 3]
   &String is Copy: r "hi" and r2 "hi" both read
   Vec<i32> is 3 words on the stack and moves: the type, not the region, decides

5. Listing 4.7: the sidebar's stack_function
   In func, var arrived as 10
   In func, var is: 56
   In main, x is: 10

6. Listing 4.20: ref1 and ref2 in the same block; a borrow ends at its last use
   borrows_vec: vec is [1, 2, 3]
   let ref1 = &vec_1; borrows_vec(ref1); let ref2 = &mut vec_1; ref2.push(4); -> [1, 2, 3, 4]

7. Listings 4.22 and 4.23: shadowing the returned vector, or a &mut — both move nothing on the heap
   let vec_1 = takes_and_gives_ownership(vec_1): [1, 2, 3, 10], allocations 0, heap pointer kept: true
   mutably_borrows_vec(&mut vec_1):              [1, 2, 3, 10], allocations 0, heap pointer kept: true

8. Listing 4.25: return the Vec, not a &Vec — the heap bytes built inside come back untouched
   gives_ownership() -> [4, 5, 6]; heap pointer inside == outside: true
```
<!-- /output -->

Reading the blocks in order. Blocks 1, 2, 5, 7 and 8 carry the five narrower statements.

1. **The three groups.** `gives_ownership` hands back a vector; `takes_and_gives_ownership` takes one, pushes, and hands it back; `borrows_vec` takes a `&Vec<i32>` and the caller's `vec_1` is still there afterwards; `takes_ownership` consumes it. "Passing a variable to a function has the same effect as assigning a variable to another variable" holds — the transcript in statement 4 uses the same words for both, *value moved here*. One thing assignment does that a call also does, and the book does not mention: when the target's type is already known to be a `&mut`, a `&mut` on the right is *reborrowed* rather than moved — [Reborrowing](../reborrowing/README.md), and [exercise 6](../../10_Resources/rust_the_practical_guide/ch4_katas/README.md#6-resolve-the-mutable-reference-conflict) meets it.
2. **"At the end of the function, the variable will be dropped."** True, and datable: `drop(A)` prints before `back in main`. Narrower than it reads in one way — the callee drops what it owns at its `}` *unless it moved it on*, by returning it (group three) or by storing it somewhere that outlives the call. [Ownership and moves](../ownership_and_moves/README.md) has the drop announcing itself in each case.
3. **Statement 1, measured.** Three words of header; the same heap pointer before, inside and after the move; zero allocations for the move and zero for the borrow; one allocation of sixteen bytes for the clone, which shares nothing.
4. **Statement 2, measured.** `Flat` and `Plain` are the same four bytes; one derive line is the difference between copy and move. `[u8; 3]` and `&String` copy.
5. **Listing 4.7 prints 56 then 10**, as the sidebar says. The book's version assigns to `var` before ever reading it, so rustc adds *value passed to `var` is never read* (`unused_assignments`); the run reads it first, which is why there is no warning here. The sidebar's conclusion — an `i32` argument is a copy, so `main`'s `x` is untouched — is the `Copy` rule of statement 2 rather than a stack rule.
6. **Statement 5, run.** `ref1` and `ref2` in one block, with `ref2` pushing after `ref1`'s last use.
7. **Listing 4.22 against 4.23.** Shadowing the returned vector (4.22) and lending a `&mut` (4.23) both keep the heap pointer and both allocate nothing. The book is right to prefer 4.23 and gives the wrong reason; see statement 1. "Passing a reference is also cheaper than cloning because you aren't making any new heap allocations" holds — zero against one — and it is cheaper than *cloning*, not than moving.
8. **"When you create a value within a function and intend to return it, you must transfer ownership of that value."** Holds, and the transfer is the same three-word copy: the heap pointer inside `gives_ownership` is the one `main` receives. "In most cases, we want to use references" holds with exactly this exception — a value born inside the function has no owner outside it to borrow from, which is what `E0106`'s `help` says.

Two more things the listings show that the prose does not say. Listing 4.24's `gives_onwership` is a typo the compiler repeats back in its `dead_code` warning, *function `gives_onwership` is never used*. And Listings 4.21, 4.22, 4.29, 4.30, 4.33 and 4.34 declare `let mut` on vectors nothing writes to, so each compiles, or fails, with *variable does not need to be mutable* on top; [Variables](../../15_First_Programs/variables/README.md#mut-is-the-permission-assignment-needs) is the page on why that warning is worth keeping at zero.

## Practice

**Zero moves, one allocation.** The chapter's `main` builds a vector, shows it through a function, appends 10 through a function, and shows it again — and the chapter's first attempt at that moves the vector out, clones it back, or shadows it. Write `make() -> Vec<i32>`, `show(label: &str, vec: &[i32])` and `append_ten(vec: &mut Vec<i32>)` so that `main` keeps the one name `vec_1` from start to finish, with no `clone` and no second `let vec_1`. Then prove it: count heap allocations with a `#[global_allocator]` the way block 3 does, and compare `vec_1.as_ptr()` before and after — printing `true` or `false`, [never the address](../what_an_address_shows/README.md#why-no-example-in-this-library-prints-one). Give `make` a spare slot so the push does not reallocate, and use `show` on a slice of the middle of the vector as well, which is what taking `&[i32]` rather than `&Vec<i32>` buys.

Before you run it, say what happens if `main` calls `show("made", vec_1)` without the `&`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg_own_kata -->
*[`pg_own_kata.rs`](examples/pg_own_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the chapter's three helpers rewritten with references, so
//! the vector is allocated once, cloned never and moved never — and the
//! program proves it with an allocation count and a heap pointer.
//!
//!   rustc --edition 2024 pg_own_kata.rs -o /tmp/pgok && /tmp/pgok

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Gives ownership: the one allocation, with a spare slot for the push.
fn make() -> Vec<i32> {
    let mut vec = Vec::with_capacity(4);
    vec.extend([1, 2, 3]);
    vec
}

/// Reads: a slice, so a `&Vec<i32>`, an array or a slice all fit.
fn show(label: &str, vec: &[i32]) {
    println!("   {label}: {vec:?}");
}

/// Writes: a `&mut Vec<i32>`, because only a `Vec` can grow.
fn append_ten(vec: &mut Vec<i32>) {
    vec.push(10);
}

fn main() {
    println!("Zero moves, one allocation");
    let before = ALLOCS.load(Relaxed);
    let mut vec_1 = make();
    let heap = vec_1.as_ptr();
    show("made", &vec_1);
    append_ten(&mut vec_1);
    show("after append_ten", &vec_1);
    show("as a slice of the middle", &vec_1[1..3]);
    let allocations = ALLOCS.load(Relaxed) - before;
    println!("   allocations in total: {allocations}");
    println!("   heap pointer unchanged: {}", heap == vec_1.as_ptr());
    println!("   vec_1 still owned by main, never shadowed, never cloned: {vec_1:?}");
}
```
<!-- /source -->

<!-- output:pg_own_kata -->
*Verified output of [`pg_own_kata.rs`](examples/pg_own_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Zero moves, one allocation
   made: [1, 2, 3]
   after append_ten: [1, 2, 3, 10]
   as a slice of the middle: [2, 3]
   allocations in total: 1
   heap pointer unchanged: true
   vec_1 still owned by main, never shadowed, never cloned: [1, 2, 3, 10]
```
<!-- /output -->

One allocation, made by `make`; the push fits in the spare slot; the heap pointer never changes; `vec_1` is never moved, so it never needs to come back. Without the `&`, `show("made", vec_1)` is not a move followed by an `E0382` later — it is refused at the call, because a `Vec<i32>` is not a `&[i32]` and nothing coerces an owned value to a reference:

```text title="Abridged — real rustc output for kata_show_by_value.rs"
error[E0308]: mismatched types
 --> kata_show_by_value.rs:7:18
  |
7 |     show("made", vec_1);
  |     ----         ^^^^^ expected `&[i32]`, found `Vec<{integer}>`
  |     |
  |     arguments to this function are incorrect
  |
help: consider borrowing here
  |
7 |     show("made", &vec_1);
  |                  +
```

With the `&`, `&Vec<i32>` becomes `&[i32]` by deref coercion at the call site — [Coercion](../../29_Conversion/coercion/README.md) — and the same call takes an array or a slice.

</details>

## If you are coming from another language

**Python** passes every argument the way Rust passes `&vec_1`, and has no spelling for the other two groups: `def takes(vec): …` never takes anything away from the caller, and a function that "gives ownership" just returns a reference to a new object like every other function. The nearest thing to a move is what Listing 4.3 warns about, seen from the other side — `temp = my_vec` makes two names for one list, and `my_vec.pop()` shows through `temp`; [Assignment does not copy ↗](https://masiarek.github.io/python-learning-library/04_Names_and_Objects/assignment_does_not_copy/) in the Python library runs that. What Rust adds is that the *signature* says which group a function is in: `Vec<i32>` means the caller's name is dead after the call, `&Vec<i32>` means it is not, and `&mut Vec<i32>` means it may come back changed. Python's equivalent is a docstring.

**C** has the three groups as conventions and no way to write them down. `void takes(int *v, size_t n)` may or may not `free(v)`; the caller finds out from the documentation or from a double free. `Vec<i32>` in a Rust signature is that documentation, checked. Listing 4.25's mistake compiles in C — `return vec;` from a function whose `vec` is a local array builds under `-std=c17 -Wall` with a warning, *address of stack memory associated with local variable 'vec' returned* (`-Wreturn-stack-address`, Apple clang 21, the file `return_local.c`), and runs. Rust's `E0515` is that warning made an error. And C's pass-by-value copies a struct's bytes exactly as a Rust move does; what C lacks is the second half, marking the source as no longer usable, which is why a C struct holding a pointer needs a rule about who frees it and a Rust `Vec` does not.

**C++** gives `std::vector<int>` all three groups and one more. Passing it by value *copies* — that is Listing 4.4's clone, done silently; `std::move` makes a move that leaves the source in a valid but unspecified state you may still use; `const std::vector<int>&` is `&Vec<i32>` and `std::vector<int>&` is `&mut Vec<i32>`. The difference is what happens at the source: after `f(std::move(v))`, C++ lets you read `v` and gives you whatever the move left; after `takes_ownership(vec_1)`, Rust refuses the read at compile time. The dangling return of Listing 4.25 is the same warning-not-error in C++ as in C; [What reference explanations get wrong](../references/reference_claims_checked/README.md#c-has-no-reference-variables-c-does) runs it.

## See also

- [Ownership and moves](../ownership_and_moves/README.md) — the three rules, and the drop that announces itself
- [Borrowing](../borrowing/README.md) — many readers or one writer, and where a borrow ends
- [Copy or move?](../copy_or_move/README.md) — the rule statement 2 replaces "stack-only" with, one program with the value swapped
- [There is no `Move` trait](../no_move_trait/README.md) — why rustc explains a move as "does not implement the `Copy` trait"
- [What an address shows](../what_an_address_shows/README.md) — the header moves, the text stays, measured on a `String`
- [Stack and heap](../stack_and_heap/README.md) — one table pricing move, `Copy`, clone and `Arc::clone`
- [What a clone costs](../what_a_clone_costs/README.md) — the allocation block 3 counts, added up per field
- [Returned by value](../returned_by_value/README.md) — what the caller reserves for Listing 4.5's return
- [A borrow is a loan](../references/a_borrow_is_a_loan/README.md) — the three labelled lines of Listing 4.19's error, and why going out of scope is a use
- [Reborrowing](../reborrowing/README.md) — the one place a call and an assignment do more than move
- [When to shadow](../when_to_shadow/README.md) — Listing 4.22's `let vec_1 = …(vec_1)`, and when a second name is better
- [The global allocator](../../09_Advanced/the_global_allocator/README.md) — the counter behind block 3
- [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) — the parameter types to write instead of `&String` and `&Vec<T>`
- [Mutable binding, mutable reference](../references/mutable_binding_vs_mutable_reference/README.md) — the same chapter's §4.6, as a lesson
- [Chapter 4 exercises, run](../../10_Resources/rust_the_practical_guide/ch4_katas/README.md) — the six exercises, the book's solutions, and the better fixes
- [*Rust: The Practical Guide*, run](../../10_Resources/rust_the_practical_guide/README.md) — the book's home in this library
- [`and`, `or` and a first program's explanation, run](../../15_First_Programs/and_or_claims_checked/README.md) and [What reference explanations get wrong, run](../references/reference_claims_checked/README.md) — the same kind of check on other books
- [E0382 ↗](https://doc.rust-lang.org/error_codes/E0382.html) · [E0502 ↗](https://doc.rust-lang.org/error_codes/E0502.html) · [E0106 ↗](https://doc.rust-lang.org/error_codes/E0106.html) · [E0515 ↗](https://doc.rust-lang.org/error_codes/E0515.html)

## Po polsku

Rozdział 4 książki *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) dzieli funkcje na trzy grupy: przejmujące własność (*taking ownership*), oddające ją (*giving ownership*) i przejmujące-i-zwracające, a potem zastępuje trzecią grupę pożyczaniem (*borrowing*). Listingi kompilują się lub nie dokładnie tak, jak mówi książka, na rustc 1.98.0; jeden komunikat jest źle zacytowany („borrowed of a moved value" zamiast *borrow of moved value*). Trzy zdania nie wytrzymują sprawdzenia. Po pierwsze, przeniesienie (*move*) wektora do funkcji nie „przenosi całych danych wektora": kopiuje trzy słowa nagłówka, wskaźnik na stertę jest ten sam przed, w środku i po wywołaniu, alokacji jest zero — to `clone()` kosztuje jedną alokację. Po drugie, „typy tylko na stosie są kopiowane" myli kryterium: decyduje cecha `Copy`, a nie miejsce w pamięci — `struct Plain { x: i32 }` leży na stosie i się przenosi (`E0382`), a `&String` wskazuje na stertę i się kopiuje. Po trzecie, błąd z listingu 4.25 to `E0106` (brak specyfikatora czasu życia), błąd sygnatury, którego sprawdzacz pożyczek (*borrow checker*) jeszcze nie widział; dopiero z `'static` w sygnaturze pojawia się `E0515`, czyli wisząca referencja, o której mówi książka. Do tego listing 4.20 kompiluje się nie dlatego, że pożyczki „nie współistnieją w tym samym zakresie" — współistnieją — tylko dlatego, że pożyczka kończy się na ostatnim użyciu.

**Szukaj po polsku:** własność w funkcjach · przeniesienie a kopia nagłówka · `rust E0382 borrow of moved value` · `rust E0106 missing lifetime specifier` · `rust E0515 cannot return reference to local variable` · pożyczka kończy się na ostatnim użyciu · typy Copy a stos
