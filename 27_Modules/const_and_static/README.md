# `const` and `static`

**Level:** 201 · working knowledge

**One line:** A `const` is a value substituted at every use; a `static` is one object at one address for the whole program — and that difference, not mutability, is what decides between them.

```rust
const MAX_SCORE: u8 = 5;
static METHOD: &str = "STAR";

fn main() {
    println!("{MAX_SCORE} {METHOD}");   // 5 STAR
}
```

Both are compile-time values, both need an explicit type — there is no inference at item level — and both are `SCREAMING_SNAKE_CASE` by convention, with a rustc warning if they are not.

## The difference, measured

A `static` **is** a single object at a fixed address, guaranteed, and `&METHOD` has type `&'static str` because of it. A `const` has no address to take: `&MAX_SCORE` **const-promotes** the value into an anonymous static, and the compiler is free to share one or emit several. So comparing `&MAX_SCORE` with `&MAX_SCORE` happens to print `true` on this build, and building anything on that is a mistake.

Where the substitution *is* visible:

```rust
const SCALE: [u8; 6] = [0, 1, 2, 3, 4, 5];

fn main() {
    let a = Box::new(SCALE);
    let b = Box::new(SCALE);
    println!("{}", std::ptr::eq(&*a, &*b));   // false — two copies
}
```

The const was copied into each `Box`, because that is what "substituted at every use" means.

## Which to reach for

| | Use |
|---|---|
| a limit, a scale, a tuning knob | **`const`** — the default |
| a large table you do not want duplicated at every use site | `static` |
| an FFI symbol, or anything whose identity is part of its meaning | `static` |
| shared mutable state | `static` holding a `Mutex`, `OnceLock` or atomic |

A `const` array of 10,000 entries named at three call sites is 30,000 entries in the binary. That is the one case where the default is wrong, and the fix is one keyword.

## `const fn`

```rust
const fn quorum(voters: u32) -> u32 {
    let half = voters / 2;
    if half == 0 { 1 } else { half + 1 }
}
const QUORUM: u32 = quorum(450);
```

A `const fn` may branch, loop, index and do arithmetic. It may **not** allocate, call a non-const function, or read a `static`. Calling a plain function in a const context is `E0015`, *"cannot call non-const function `f` in constants"*.

Marking a function `const` is a promise about what it does not do, and it is part of your public API — un-`const`-ing one later is a breaking change for anyone who used it in a const context.

## Associated consts

```rust
impl Election {
    const DEFAULT_SEATS: u32 = 1;
    const MAX_SEATS: u32 = 10;
}
```

Namespaced under the type, so two types can each have a `MAX_SEATS` and neither needs to be called `MAX_SEATS_FOR_ELECTION`. This is how `u8::MAX` and `f64::EPSILON` are written, and it is where most consts in real code belong.

## The trap: `static mut`

It exists, it is mutable global state, and in the 2024 edition even `&COUNT` inside an `unsafe` block is refused — *"creating a shared reference to mutable static"*, from the deny-by-default `static_mut_refs` lint, with `&raw const COUNT` offered as the replacement.

That is the language telling you the answer is a different type. A `static` holding a [`Mutex` ↗](https://doc.rust-lang.org/std/sync/struct.Mutex.html), a [`OnceLock` ↗](https://doc.rust-lang.org/std/sync/struct.OnceLock.html) or an [`AtomicU32` ↗](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU32.html) is safe, needs no `unsafe`, and says what the sharing rule is.

## If you are coming from another language

- **Python.** Module-level `MAX_SCORE = 5` is a `static` in the address sense — one object, referenced from everywhere — and Python has no equivalent of `const` substitution at all, because everything is a reference. Two habits transfer badly. Python's module constants are *mutable* (nothing stops `module.MAX_SCORE = 9`), where Rust's are enforced; and Python's mutable-default and shared-global bugs are exactly what `static mut` would let you write, which is why Rust makes you spell them with a `Mutex` instead. `functools.cache` on a pure function is the nearest thing to `const fn` evaluation, and it happens at run time.
- **ABAP.** `CONSTANTS lc_max_score TYPE i VALUE 5.` is `const`, down to the naming convention and the compile-time value, and it has the same substitution semantics — the value is baked in where it is used. `CLASS-DATA` is closer to `static`: one instance shared by every use, for the life of the session, and mutable — which is precisely the shared mutable global that Rust refuses to make easy. The ABAP habit worth keeping is that `CONSTANTS` inside a class or interface is namespaced by it (`zcl_election=>c_max_seats`), which is exactly Rust's associated const and for the same reason. And ABAP has no `const fn`: a constant must be a literal, so the compile-time arithmetic Rust allows has no counterpart at all.
- **C / C++.** `#define` is the substitution half of `const` and `static const int` is the address half, so Rust's split is the one C++ programmers already carry — with the difference that `const` here is typed and scoped rather than a preprocessor string. `constexpr` is `const fn`, arriving at the same restrictions from the same direction.
- **Java / C#.** `static final` and `const`/`static readonly`. C#'s split is nearly identical to Rust's and for the same reason: `const` is inlined into every calling assembly (which is why changing one is a breaking change there too), and `static readonly` is a single field read at run time.

---

## The verified output

<!-- output:const_and_static -->
*Verified output of [`const_and_static.rs`](examples/const_and_static.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Both are compile-time values, and neither may be `let`
   MAX_SCORE = 5, METHOD = STAR, SEATS = 4
   Both need an explicit type — there is no inference at item level.
   Both are SCREAMING_SNAKE_CASE by convention, and rustc warns if
   they are not.

2. The difference is whether it HAS an address
   &METHOD == &METHOD (static): true
   &MAX_SCORE == &MAX_SCORE (const): true
   Both true, and only the first one is a promise. A `static` IS a
   single object at a fixed address, guaranteed. A `const` is a value
   substituted at each use, and `&MAX_SCORE` has no address to take —
   so rustc CONST-PROMOTES it into an anonymous static, and is free
   to share one or make several. Do not build anything on that `true`.
   Where the substitution is visible is a const that is not promoted:
   Box::new(MAX_SCORE) twice, same address: false
   Two separate values, because the const was copied into each.

3. Which to reach for
   const   a value with a name: limits, scales, tuning knobs. The
           default, and what you want almost every time.
   static  when the address matters: a large table you do not want
           copied into every use site, an FFI symbol, or anything
           whose identity is part of its meaning.
   A const of a big array is duplicated at each use; a static is not.

4. `const fn`, and what it may not do
   seats_for(450) = 4 — evaluated at compile time, so it can
   initialise a const. A const fn may branch, loop and do arithmetic,
   and may not allocate, call a non-const fn, or read a static.
   It is still an ordinary function at run time: seats_for(50) = 1

5. Associated consts, and `static mut`
   Ballot::SCALE   = 0-5
   Ballot::describe() = STAR on a 0-5 scale
   `static mut` exists and is mutable global state: every access is
   `unsafe`, and in the 2024 edition even `&COUNT` inside an unsafe
   block is refused — "creating a shared reference to mutable
   static", from the deny-by-default `static_mut_refs` lint, with
   `&raw const COUNT` offered as the replacement. What you want
   instead is a `static` holding a Mutex, a OnceLock or an atomic —
   safe, because those are the types that make sharing sound.
```
<!-- /output -->

## Practice

**The address you may not rely on.** Take a `const` array and a `static` array, take two references to each, and compare the pointers. Both comparisons come out the same way; only one of the two answers is a guarantee, so say which and why. Then `Box` the const twice and compare again, and explain the different result in one sentence.

Then two questions about `const fn`. Write one that computes a quorum, use it to initialise a `const`, and then add a version that returns a `String` — the second will not compile in a const context, and the error names the reason. And say why marking a function `const` is a semver decision rather than an optimisation.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:const_and_static_kata -->
*[`const_and_static_kata.rs`](examples/const_and_static_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the address you may not rely on, and the const fn that will not.
//!
//!   rustc --edition 2024 const_and_static_kata.rs -o /tmp/csk && /tmp/csk

const SCALE: [u8; 6] = [0, 1, 2, 3, 4, 5];
static METHODS: [&str; 3] = ["STAR", "Approval", "Plurality"];

const fn quorum(voters: u32) -> u32 {
    // Allowed in a const fn: arithmetic, comparison, if, loops, indexing.
    let half = voters / 2;
    if half == 0 { 1 } else { half + 1 }
}

/// Not const: it allocates.
fn quorum_message(voters: u32) -> String {
    format!("{} of {voters}", quorum(voters))
}

struct Election;
impl Election {
    const DEFAULT_SEATS: u32 = 1;
    const MAX_SEATS: u32 = 10;

    fn seats(requested: u32) -> u32 {
        requested.clamp(Self::DEFAULT_SEATS, Self::MAX_SEATS)
    }
}

fn main() {
    println!("1. Where they may appear, and where they may not");
    println!("   const QUORUM: u32 = quorum(450);  ->  {}", quorum(450));
    println!("   const N: u32 = some_fn();         ->  E0015 unless some_fn is const");
    println!("   static as an array length: [u8; SCALE.len()] is fine, because");
    println!("   SCALE is a const. A `static`'s value cannot be used that way.");
    let sized: [u8; SCALE.len()] = SCALE;
    println!("   [u8; SCALE.len()] built: {sized:?}");

    println!();
    println!("2. The address question, measured");
    let c1: *const u8 = &SCALE[0];
    let c2: *const u8 = &SCALE[0];
    let s1: *const &str = &METHODS[0];
    let s2: *const &str = &METHODS[0];
    println!("   &SCALE[0] twice   (const):  {}", c1 == c2);
    println!("   &METHODS[0] twice (static): {}", s1 == s2);
    println!("   Both true, and only the second is guaranteed. Taking a reference");
    println!("   to a const promotes the value into an anonymous static, and rustc");
    println!("   may share one or emit several — so equality here is an");
    println!("   observation about this build, not a rule. The static is a rule.");
    let a = Box::new(SCALE);
    let b = Box::new(SCALE);
    println!("   Box::new(SCALE) twice, same address: {}",
             std::ptr::eq(&*a as *const _, &*b as *const _));
    println!("   Two copies, because the const was substituted into each Box.");

    println!();
    println!("3. What a const fn may not do");
    println!("   quorum(450)         = {}   compile time or run time, either", quorum(450));
    println!("   quorum_message(450) = {}   run time only", quorum_message(450));
    println!("   The second allocates a String, and allocation is not allowed in a");
    println!("   const context. Neither is calling a non-const fn, or reading a");
    println!("   `static`. Marking a function `const` is a PROMISE about what it");
    println!("   does not do, and it is part of your public API: un-consting one");
    println!("   later is a breaking change.");

    println!();
    println!("4. Associated consts, which is where most consts belong");
    println!("   Election::DEFAULT_SEATS = {}", Election::DEFAULT_SEATS);
    println!("   Election::seats(0)  = {}", Election::seats(0));
    println!("   Election::seats(50) = {}", Election::seats(50));
    println!("   Namespaced under the type, so two types can each have a MAX_SEATS");
    println!("   and neither has to be MAX_SEATS_FOR_ELECTION. A trait can declare");
    println!("   one too, which is how `u8::MAX` and `f64::EPSILON` are written.");

    println!();
    println!("5. The rule");
    println!("   const   unless you can say why the address matters.");
    println!("   static  for a large table, an FFI symbol, or shared mutable state");
    println!("           behind a Mutex / OnceLock / atomic.");
    println!("   A const array of 10_000 entries copied into three call sites is");
    println!("   30_000 entries in the binary. That is the one case where the");
    println!("   default is wrong, and the fix is one keyword.");
}
```
<!-- /source -->

<!-- output:const_and_static_kata -->
*Verified output of [`const_and_static_kata.rs`](examples/const_and_static_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Where they may appear, and where they may not
   const QUORUM: u32 = quorum(450);  ->  226
   const N: u32 = some_fn();         ->  E0015 unless some_fn is const
   static as an array length: [u8; SCALE.len()] is fine, because
   SCALE is a const. A `static`'s value cannot be used that way.
   [u8; SCALE.len()] built: [0, 1, 2, 3, 4, 5]

2. The address question, measured
   &SCALE[0] twice   (const):  true
   &METHODS[0] twice (static): true
   Both true, and only the second is guaranteed. Taking a reference
   to a const promotes the value into an anonymous static, and rustc
   may share one or emit several — so equality here is an
   observation about this build, not a rule. The static is a rule.
   Box::new(SCALE) twice, same address: false
   Two copies, because the const was substituted into each Box.

3. What a const fn may not do
   quorum(450)         = 226   compile time or run time, either
   quorum_message(450) = 226 of 450   run time only
   The second allocates a String, and allocation is not allowed in a
   const context. Neither is calling a non-const fn, or reading a
   `static`. Marking a function `const` is a PROMISE about what it
   does not do, and it is part of your public API: un-consting one
   later is a breaking change.

4. Associated consts, which is where most consts belong
   Election::DEFAULT_SEATS = 1
   Election::seats(0)  = 1
   Election::seats(50) = 10
   Namespaced under the type, so two types can each have a MAX_SEATS
   and neither has to be MAX_SEATS_FOR_ELECTION. A trait can declare
   one too, which is how `u8::MAX` and `f64::EPSILON` are written.

5. The rule
   const   unless you can say why the address matters.
   static  for a large table, an FFI symbol, or shared mutable state
           behind a Mutex / OnceLock / atomic.
   A const array of 10_000 entries copied into three call sites is
   30_000 entries in the binary. That is the one case where the
   default is wrong, and the fix is one keyword.
```
<!-- /output -->

</details>

---

## See also

- [Modules and visibility](../modules_and_visibility/README.md) — `pub const` and where it is visible from
- [`&'static str`](../../14_Strings/static_str/README.md) — the other `'static`, and the confusion between them
- [What an attribute is](../what_an_attribute_is/README.md) — `#[allow]`, and the other item-level metadata
- [Sharing across threads: `Arc`](../../18_Ownership/sharing_across_threads/README.md) — where a `static` holding a `Mutex` ends up
- [What a type annotation does](../../15_First_Programs/what_an_annotation_does/README.md) — why both of these need their type written out, where a `let` does not

## Sources

[constants ↗](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html) in Rust by Example; the Reference's [constant items ↗](https://doc.rust-lang.org/reference/items/constant-items.html) and [static items ↗](https://doc.rust-lang.org/reference/items/static-items.html), plus the edition guide's [static mut references ↗](https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html), which rustc's own note links to.
