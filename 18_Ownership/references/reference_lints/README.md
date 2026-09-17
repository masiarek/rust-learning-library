# Lints around references: bad and good, run

[References](../README.md) › **Lints**

**Level:** 201 · a reference, by lint

**One line:** Twenty-five warnings from rustc and clippy that fire on `&`, `&mut`, `*`, `ref`, `Box`, `Rc` and raw pointers — each with a program that triggers it, what the tool prints, a version that is silent, and when the lint is right or noise — plus six mistakes with references and pointers that no lint catches.

Every transcript is the output of the command in its title, run on rustc 1.98.0 and clippy 0.1.98 against the program above it saved as `bad.rs`, and every silent version printed nothing under the same command. The programs are library crates — `pub fn`, no `main` — because that is how the build compiles them. The lines dropped from each transcript are the documentation link, the note naming the lint's level (*on by default*, *requested on the command line*, *implied by*, and how to override it) and the closing count. Both programs of every pair are compiled on every build by [`check_fences.py`](../../../tools/check_fences.py), as edition 2024; the two edition-2021 programs that 2024 refuses are marked `compile_fail`. The build does not re-run clippy, so a later clippy can reword a warning. A *since* version is the `clippy::version` attribute in clippy's source at the 1.98.0 tag; lints older than 1.29.0 carry none, and rustc's lints carry no version.

## Turning them on

rustc's lints and clippy's *warn by default* groups need nothing: `cargo clippy` prints them. The opt-in ones take a flag or a `Cargo.toml` entry:

```sh
cargo clippy -- -W clippy::mut_mut -W clippy::borrow_as_ptr -W rust-2024-compatibility
```

```toml
[lints.clippy]
mut_mut = "warn"
borrow_as_ptr = "warn"

[lints.rust]
rust_2024_compatibility = "warn"
```

## Find the lint

| Lint | From | On by default? | Fires on |
|---|---|---|---|
| [`static_mut_refs`](#static_mut_refs) | rustc | yes; deny on edition 2024 | `&COUNTER` on a `static mut` |
| [`dangling_pointers_from_temporaries`](#dangling_pointers_from_temporaries) | rustc | yes | `.as_ptr()` on a `CString` that dies at the `;` |
| [`dangling_pointers_from_locals`](#dangling_pointers_from_locals) | rustc | yes | returning `&raw const n` for a local `n` |
| [`unused_mut`](#unused_mut) | rustc | yes | `mut scores: &mut Vec<i32>` |
| [`mut_from_ref`](#mut_from_ref) | clippy · correctness | yes, deny | `fn get(&self) -> &mut i32` with `unsafe` inside |
| [`borrowed_box`](#borrowed_box) | clippy · complexity | yes | `&Box<dyn Shape>` |
| [`bool_comparison`](#bool_comparison) | clippy · complexity | yes | `*flag == true` |
| [`deref_addrof`](#deref_addrof) | clippy · complexity | yes | `*&n` |
| [`borrow_deref_ref`](#borrow_deref_ref) | clippy · complexity | yes | `&*line` on a `&str` |
| [`explicit_auto_deref`](#explicit_auto_deref) | clippy · complexity | yes | `&**r` where a coercion would do it |
| [`redundant_allocation`](#redundant_allocation) | clippy · perf | yes | `Box<&str>`, `Rc<Box<u32>>` |
| [`needless_borrow`](#needless_borrow) | clippy · style | yes | `&name` on a `name: &str` |
| [`needless_borrows_for_generic_args`](#needless_borrows_for_generic_args) | clippy · style | yes | `&name` passed to an `S: AsRef<str>` |
| [`unnecessary_mut_passed`](#unnecessary_mut_passed) | clippy · style | yes | `&mut scores` passed to a `&[i32]` |
| [`ptr_arg`](#ptr_arg) | clippy · style | yes | `&Vec<i32>` parameters |
| [`op_ref`](#op_ref) | clippy · style | yes | `&a == &b` |
| [`toplevel_ref_arg`](#toplevel_ref_arg) | clippy · style | yes | `let ref first = …` |
| [`rust_2024_incompatible_pat`](#rust_2024_incompatible_pat) | rustc | no | `Some(ref n)` matched on a `&Option<String>`, edition 2021 |
| [`mut_mut`](#mut_mut) | clippy · pedantic | no | `&mut n` on a `&mut`, and `&mut &mut [i32]` |
| [`trivially_copy_pass_by_ref`](#trivially_copy_pass_by_ref) | clippy · pedantic | no | an `&i32` parameter |
| [`borrow_as_ptr`](#borrow_as_ptr) | clippy · pedantic | no | `&n as *const i32` |
| [`ref_as_ptr`](#ref_as_ptr) | clippy · pedantic | no | `r as *const i32` |
| [`ptr_as_ptr`](#ptr_as_ptr) | clippy · pedantic | no | `p as *const u8` |
| [`needless_pass_by_ref_mut`](#needless_pass_by_ref_mut) | clippy · nursery | no | a `&mut [i32]` parameter that is only read |
| [`ref_patterns`](#ref_patterns) | clippy · restriction | no | every `ref` in a pattern |
| [`noop_method_call`](../../../12_Traits/how_to_learn_to_owned/to_owned_lints/README.md#noop_method_call) | rustc | yes | `.clone()` on a `&str` — on the `ToOwned` lints page |
| [`suspicious_double_ref_op`](../../../12_Traits/how_to_learn_to_owned/to_owned_lints/README.md#suspicious_double_ref_op) | rustc | yes | `.clone()` on a `&&String` — on the `ToOwned` lints page |
| [`clone_on_ref_ptr`](../../../12_Traits/how_to_learn_to_owned/to_owned_lints/README.md#clone_on_ref_ptr) | clippy · restriction | no | `rc.clone()` — on the `ToOwned` lints page |

## `rustc`, on by default

### `static_mut_refs`

**rustc** · warn by default on edition 2021, deny on 2024 · fires on `&COUNTER` when `COUNTER` is a `static mut`

```rust,compile_fail
static mut COUNTER: u32 = 0;

pub fn bump() -> u32 {
    unsafe {
        COUNTER += 1;
        let current: &u32 = &COUNTER;
        *current
    }
}
```

```text title="rustc --edition 2021 --crate-type lib bad.rs — rustc 1.98.0"
warning: creating a shared reference to mutable static
 --> bad.rs:6:29
  |
6 |         let current: &u32 = &COUNTER;
  |                             ^^^^^^^^ shared reference to mutable static
  |
  = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives
help: use `&raw const` instead to create a raw pointer
  |
6 |         let current: &u32 = &raw const COUNTER;
  |                              +++++++++
```

**Silent:**

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn bump() -> u32 {
    COUNTER.fetch_add(1, Ordering::Relaxed) + 1
}
```

**When it is right.** Always, for the reason in its note: nothing checks that the static is not written while `current` lives. The `help:` does not compile on this line — `let current: &u32 = &raw const COUNTER;` is `E0308`, *expected `&u32`, found `*const u32`* — and with the annotation changed to `*const u32` the program is silent on both editions and keeps the hazard. The atomic above removes the `static mut`. The bad program is marked `compile_fail` because edition 2024 makes this lint an error; that refusal is on [the errors page](../reference_errors/README.md), and a published tutorial that meets it is on [*Learn Rust the Dangerous Way*](../reference_claims_checked/README.md#learn-rust-the-dangerous-way-a-mutable-reference-to-a-static-mut).

### `dangling_pointers_from_temporaries`

**rustc** · warn by default · fires on `.as_ptr()` on a temporary that is dropped at the end of the statement

```rust
use std::ffi::{CStr, CString, c_char};

pub fn greeting_len() -> usize {
    let ptr: *const c_char = CString::new("hello").unwrap().as_ptr();
    unsafe { CStr::from_ptr(ptr) }.count_bytes()
}
```

```text title="rustc --edition 2024 --crate-type lib bad.rs — rustc 1.98.0"
warning: this creates a dangling pointer because temporary `CString` is dropped at end of statement
 --> bad.rs:4:61
  |
4 |     let ptr: *const c_char = CString::new("hello").unwrap().as_ptr();
  |                              ------------------------------ ^^^^^^ pointer created here
  |                              |
  |                              this `CString` is dropped at end of statement
  |
  = help: bind the `CString` to a variable such that it outlives the pointer returned by `as_ptr`
  = note: a dangling pointer is safe, but dereferencing one is undefined behavior
  = note: returning a pointer to a local variable will always result in a dangling pointer
```

**Silent:**

```rust
use std::ffi::{CStr, CString, c_char};

pub fn greeting_len() -> usize {
    let owned = CString::new("hello").unwrap();
    let ptr: *const c_char = owned.as_ptr();
    unsafe { CStr::from_ptr(ptr) }.count_bytes()
}
```

**When it is right.** Always: the `CString` dies at the `;` ([a temporary dies at the semicolon](../../temporary_lifetimes/README.md)), and `ptr` is read on the next line. It catches `String::from("hi").as_ptr()` the same way. A `String` that has a name and dies at the end of a function is outside its reach — [below](#what-no-lint-catches).

### `dangling_pointers_from_locals`

**rustc** · warn by default · fires on returning a pointer to a local variable

```rust
pub fn address() -> *const i32 {
    let n = 5;
    &raw const n
}
```

```text title="rustc --edition 2024 --crate-type lib bad.rs — rustc 1.98.0"
warning: function returns a dangling pointer to dropped local variable `n`
 --> bad.rs:3:5
  |
1 | pub fn address() -> *const i32 {
  |                     ---------- return type is `*const i32`
2 |     let n = 5;
  |         - local variable `n` is dropped at the end of the function
3 |     &raw const n
  |     ^^^^^^^^^^^^
  |
  = note: a dangling pointer is safe, but dereferencing one is undefined behavior
```

**Silent:**

```rust
pub fn address(n: &i32) -> *const i32 {
    n
}
```

**When it is right.** Always: `n` lives in the function's frame, which is gone when the caller gets the address. It watches the local itself — `s.as_ptr()` returned from a local `String` points into the heap buffer the `String` frees, and passes ([below](#what-no-lint-catches)).

### `unused_mut`

**rustc** · warn by default · fires on `mut` on a binding that is never reassigned or mutably borrowed — including one whose type is `&mut`

```rust
pub fn add_one(mut scores: &mut Vec<i32>) {
    let mut first = &mut scores[0];
    *first += 1;
    scores.push(1);
}
```

```text title="rustc --edition 2024 --crate-type lib bad.rs — rustc 1.98.0"
warning: variable does not need to be mutable
 --> bad.rs:1:16
  |
1 | pub fn add_one(mut scores: &mut Vec<i32>) {
  |                ----^^^^^^
  |                |
  |                help: remove this `mut`
  |

warning: variable does not need to be mutable
 --> bad.rs:2:9
  |
2 |     let mut first = &mut scores[0];
  |         ----^^^^^
  |         |
  |         help: remove this `mut`
```

**Silent:**

```rust
pub fn add_one(scores: &mut Vec<i32>) {
    let first = &mut scores[0];
    *first += 1;
    scores.push(1);
}
```

**When it is right.** Always. The `&mut` in the type is what lets `*first += 1` and `scores.push(1)` write; `mut` before the name would let the function point the name somewhere else, and `add_one` does that to neither. [Re-pointing a slice](../repointing_a_slice/README.md#a-plain-mut-i32-re-points-its-own-copy) has a `mut t: &mut [i32]` that needs both.

## Clippy, on by default

### `mut_from_ref`

**clippy · correctness** · deny by default · fires on a function that takes `&` and returns `&mut`, when its body has `unsafe`

```rust
pub struct Slot {
    ptr: *mut i32,
}

impl Slot {
    pub fn get(&self) -> &mut i32 {
        unsafe { &mut *self.ptr }
    }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
error: mutable borrow from immutable input(s)
 --> bad.rs:6:26
  |
6 |     pub fn get(&self) -> &mut i32 {
  |                          ^^^^^^^^
  |
note: immutable borrow here
 --> bad.rs:6:16
  |
6 |     pub fn get(&self) -> &mut i32 {
  |                ^^^^^
```

**Silent:**

```rust
pub struct Slot {
    ptr: *mut i32,
}

impl Slot {
    pub fn get(&mut self) -> &mut i32 {
        unsafe { &mut *self.ptr }
    }
}
```

**When it is right.** Right: two calls to `get` on one `&Slot` hand out two live `&mut i32` to the same place. Deny means `cargo clippy` stops; rustc alone compiles the bad program, which is why the build accepts the fence. It looks only at functions with `unsafe` inside — a `get(&self) -> &mut i32` that returns `Box::leak(Box::new(0))` is not flagged, since each call gets a fresh allocation. If writing through a shared `&` is the design, that is [interior mutability](../../../09_Advanced/interior_mutability/README.md).

### `borrowed_box`

**clippy · complexity** · warn by default · fires on `&Box<T>` in a signature

```rust
pub trait Shape {
    fn area(&self) -> f64;
}

pub fn describe(shape: &Box<dyn Shape>) -> String {
    format!("area {}", shape.area())
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: you seem to be trying to use `&Box<T>`
 --> bad.rs:5:24
  |
5 | pub fn describe(shape: &Box<dyn Shape>) -> String {
  |                        ^^^^^^^^^^^^^^^ help: consider using just `&T`: `&dyn Shape`
  |
```

**Silent:**

```rust
pub trait Shape {
    fn area(&self) -> f64;
}

pub fn describe(shape: &dyn Shape) -> String {
    format!("area {}", shape.area())
}
```

**When it is right.** Right: `&Box<dyn Shape>` accepts only a shape that is already boxed — passing `&square` for a stack `Square` is `E0308` — where `&dyn Shape` takes both. A caller holding the box passes `&*boxed`: plain `&boxed` is `E0277`, because the compiler then tries to use the `Box` itself as the `dyn Shape`.

### `bool_comparison`

**clippy · complexity** · warn by default · fires on `*flag == true`

```rust
pub fn label(flag: &bool) -> &'static str {
    if *flag == true { "on" } else { "off" }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: equality checks against true are unnecessary
 --> bad.rs:2:8
  |
2 |     if *flag == true { "on" } else { "off" }
  |        ^^^^^^^^^^^^^ help: try: `*flag`
  |
```

**Silent:**

```rust
pub fn label(flag: &bool) -> &'static str {
    if *flag { "on" } else { "off" }
}
```

**When it is right.** Always. `flag == true` on a `&bool` is refused, so the `*` goes in and the `== true` stays behind; `*flag` alone is the condition. [`==` wants both sides at the same depth](../when_you_need_the_star/README.md#wants-both-sides-at-the-same-depth).

### `deref_addrof`

**clippy · complexity** · warn by default · fires on `*&n`

```rust
pub fn twice(n: i32) -> i32 {
    let m = *&n;
    n + m
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: immediately dereferencing a reference
 --> bad.rs:2:13
  |
2 |     let m = *&n;
  |             ^^^ help: try: `n`
  |
```

**Silent:**

```rust
pub fn twice(n: i32) -> i32 {
    let m = n;
    n + m
}
```

**When it is right.** Always: `&n` points at `n`, and `*` of that is [the place `n`](../../where_the_sigil_sits/README.md#r-is-a-place-not-a-value) again.

### `borrow_deref_ref`

**clippy · complexity** · warn by default · since 1.63.0 · fires on `&*r` where `r` is already a `&T`

```rust
pub fn first_word(line: &str) -> &str {
    let view: &str = &*line;
    view.split(' ').next().unwrap_or("")
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: deref on an immutable reference
 --> bad.rs:2:22
  |
2 |     let view: &str = &*line;
  |                      ^^^^^^ help: if you would like to reborrow, try removing `&*`: `line`
  |
```

**Silent:**

```rust
pub fn first_word(line: &str) -> &str {
    let view: &str = line;
    view.split(' ').next().unwrap_or("")
}
```

**When it is right.** Right: `&*` on a `&str` makes another `&str` to the same text, and a `&str` is `Copy` anyway. On a `&mut T` it is the way to get a `&T`, and there it is silent — `let shared: &String = &*r;` with `r: &mut String` passes.

### `explicit_auto_deref`

**clippy · complexity** · warn by default · since 1.64.0 · fires on `&**r` where a deref coercion would do the same

```rust
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn greet() -> String {
    let name = String::from("ferris");
    let r: &String = &name;
    shout(&**r)
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: deref which would be done by auto-deref
 --> bad.rs:8:11
  |
8 |     shout(&**r)
  |           ^^^^ help: try: `r`
  |
```

**Silent:**

```rust
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn greet() -> String {
    let name = String::from("ferris");
    let r: &String = &name;
    shout(r)
}
```

**When it is right.** Right: `&String` to `&str` at a call is a deref coercion, [one of the five places the `*` is written for you](../when_you_need_the_star/README.md#five-places-the-is-written-for-you).

### `redundant_allocation`

**clippy · perf** · warn by default · since 1.44.0 · fires on `Box<&T>`, `Rc<Box<T>>` and the other pointer-in-a-pointer types

```rust
use std::rc::Rc;

struct Cache<'a> {
    name: Box<&'a str>,
    count: Rc<Box<u32>>,
}

pub fn describe() -> String {
    let cache = Cache {
        name: Box::new("ferris"),
        count: Rc::new(Box::new(3)),
    };
    format!("{} {}", cache.name, cache.count)
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: usage of `Box<&'a str>`
 --> bad.rs:4:11
  |
4 |     name: Box<&'a str>,
  |           ^^^^^^^^^^^^ help: try: `&'a str`
  |
  = note: `&'a str` is already a pointer, `Box<&'a str>` allocates a pointer on the heap

warning: usage of `Rc<Box<u32>>`
 --> bad.rs:5:12
  |
5 |     count: Rc<Box<u32>>,
  |            ^^^^^^^^^^^^
  |
  = note: `Box<u32>` is already on the heap, `Rc<Box<u32>>` makes an extra allocation
  = help: consider using just `Rc<u32>` or `Box<u32>`
```

**Silent:**

```rust
use std::rc::Rc;

struct Cache<'a> {
    name: &'a str,
    count: Rc<u32>,
}

pub fn describe() -> String {
    let cache = Cache {
        name: "ferris",
        count: Rc::new(3),
    };
    format!("{} {}", cache.name, cache.count)
}
```

**When it is right.** Right: `Box<&str>` allocates only to hold a pointer, and `Rc<Box<u32>>` allocates twice for one `u32`. Two cases it left alone here: a `pub` field of a `pub` struct (`avoid-breaking-exported-api`), and `Rc<Box<[u8]>>` — which is 8 bytes where `Rc<[u8]>` is 16 on this 64-bit target, because the inner `Box` [carries the length](../pointer_types_compared/README.md#boxt-is-neither-always-one-pointer-wide-nor-only-a-pointer).

### `needless_borrow`

**clippy · style** · warn by default · fires on `&name` where `name` is already a reference

```rust
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn greet() -> String {
    let name: &str = "ferris";
    shout(&name)
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: this expression creates a reference which is immediately dereferenced by the compiler
 --> bad.rs:7:11
  |
7 |     shout(&name)
  |           ^^^^^ help: change this to: `name`
  |
```

**Silent:**

```rust
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn greet() -> String {
    let name: &str = "ferris";
    shout(name)
}
```

**When it is right.** Always: `&name` is a `&&str` that the compiler derefs straight back. It watches only a borrow of something that is already a reference — `shout(&name)` with `name: String` needs its `&` and is silent.

### `needless_borrows_for_generic_args`

**clippy · style** · warn by default · since 1.74.0 · fires on `&x` passed to a generic parameter that `x` satisfies by itself

```rust
pub fn shout<S: AsRef<str>>(s: S) -> String {
    s.as_ref().to_uppercase()
}

pub fn greet() -> String {
    let name: &str = "ferris";
    shout(&name)
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: the borrowed expression implements the required traits
 --> bad.rs:7:11
  |
7 |     shout(&name)
  |           ^^^^^ help: change this to: `name`
  |
```

**Silent:**

```rust
pub fn shout<S: AsRef<str>>(s: S) -> String {
    s.as_ref().to_uppercase()
}

pub fn greet() -> String {
    let name: &str = "ferris";
    shout(name)
}
```

**When it is right.** Right when the trait does the same thing for `T` and `&T`, which the lint cannot check — its docs name that as the known problem. With `name: String` and no later use, `shout(&name)` was not flagged.

### `unnecessary_mut_passed`

**clippy · style** · warn by default · fires on `&mut x` passed where the parameter is `&`

```rust
pub fn total(scores: &[i32]) -> i32 {
    scores.iter().sum()
}

pub fn run() -> i32 {
    let mut scores = [90, 85];
    scores[0] += 1;
    total(&mut scores)
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: the function `total` doesn't need a mutable reference
 --> bad.rs:8:11
  |
8 |     total(&mut scores)
  |           ^^^^^^^^^^^
  |
help: remove this `mut`
  |
8 -     total(&mut scores)
8 +     total(&scores)
  |
```

**Silent:**

```rust
pub fn total(scores: &[i32]) -> i32 {
    scores.iter().sum()
}

pub fn run() -> i32 {
    let mut scores = [90, 85];
    scores[0] += 1;
    total(&scores)
}
```

**When it is right.** Always. The `&mut` coerces to `&`, so it compiles, but the call site claims [the one writer](../../borrowing/README.md#the-one-rule) for a function that only reads.

### `ptr_arg`

**clippy · style** · warn by default · fires on `&Vec<T>`, `&String` and `&PathBuf` parameters, and on a `&mut Vec<T>` that is only read

```rust
pub fn total(scores: &Vec<i32>) -> i32 {
    scores.iter().sum()
}

pub fn record(scores: &mut Vec<i32>, score: i32) {
    scores.push(score);
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
 --> bad.rs:1:22
  |
1 | pub fn total(scores: &Vec<i32>) -> i32 {
  |                      ^^^^^^^^^
  |
help: change this to
  |
1 - pub fn total(scores: &Vec<i32>) -> i32 {
1 + pub fn total(scores: &[i32]) -> i32 {
  |
```

**Silent:**

```rust
pub fn total(scores: &[i32]) -> i32 {
    scores.iter().sum()
}

pub fn record(scores: &mut Vec<i32>, score: i32) {
    scores.push(score);
}
```

**When it is right.** Right: `&Vec<i32>` borrows the owner where the function needs only the view, and `&[i32]` accepts a `&Vec<i32>`, an array and a slice ([one parameter serves every caller](../../../14_Strings/string_vs_str/README.md#one-parameter-serves-every-caller)). `record` keeps `&mut Vec<i32>` because it pushes; the same parameter only read is flagged as *writing `&mut Vec` instead of `&mut [_]`*. It fires on a `pub fn` in a library crate, which `trivially_copy_pass_by_ref` and `needless_pass_by_ref_mut` skip. The `&String` and `&PathBuf` transcript is on [the `ToOwned` lints page](../../../12_Traits/how_to_learn_to_owned/to_owned_lints/README.md#ptr_arg).

### `op_ref`

**clippy · style** · warn by default · fires on `&a == &b`

```rust
pub fn same(a: i32, b: i32) -> bool {
    &a == &b
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: needlessly taken reference of both operands
 --> bad.rs:2:5
  |
2 |     &a == &b
  |     ^^^^^^^^
  |
help: use the values directly
  |
2 -     &a == &b
2 +     a == b
  |
```

**Silent:**

```rust
pub fn same(a: i32, b: i32) -> bool {
    a == b
}
```

**When it is right.** Always: `==` on two `&i32` compares the integers ([both sides at the same depth](../when_you_need_the_star/README.md#wants-both-sides-at-the-same-depth)), so the borrows add nothing. The mistake in the other direction — wanting to compare addresses — is [below](#what-no-lint-catches).

### `toplevel_ref_arg`

**clippy · style** · warn by default · fires on `ref` on a whole `let` pattern or a whole parameter

```rust
pub fn first_len(words: Vec<String>) -> usize {
    let ref first = words[0];
    first.len()
}
```

```text title="clippy-driver --edition 2024 --crate-type lib bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: `ref` on an entire `let` pattern is discouraged, take a reference with `&` instead
 --> bad.rs:2:9
  |
2 |     let ref first = words[0];
  |     ----^^^^^^^^^------------ help: try: `let first = &words[0];`
  |
```

**Silent:**

```rust
pub fn first_len(words: Vec<String>) -> usize {
    let first = &words[0];
    first.len()
}
```

**When it is right.** Always: [`let ref z = x;` is `let z = &x;`](../the_ref_keyword/README.md#let-ref-z-x-is-let-z-x) in a spelling fewer readers know. On a parameter, `fn len(ref s: String)`, the message changes to *`ref` directly on a function parameter does not prevent taking ownership of the passed argument*.

## `rustc`, opt-in

### `rust_2024_incompatible_pat`

**rustc** · allow by default, on with `-W rust-2024-compatibility` · edition 2021 · fires on an explicit `ref` in a pattern that already borrows

```rust,compile_fail
pub fn name_len(name: &Option<String>) -> usize {
    match name {
        Some(ref n) => n.len(),
        None => 0,
    }
}
```

```text title="rustc --edition 2021 --crate-type lib -W rust-2024-compatibility bad.rs — rustc 1.98.0"
warning: cannot explicitly borrow within an implicitly-borrowing pattern in Rust 2024
 --> bad.rs:3:14
  |
3 |         Some(ref n) => n.len(),
  |              ^^^ explicit `ref` binding modifier not allowed when implicitly borrowing
  |
note: matching on a reference type with a non-reference pattern implicitly borrows the contents
 --> bad.rs:3:9
  |
3 |         Some(ref n) => n.len(),
  |         ^^^^^^^^^^^ this non-reference pattern matches on a reference type `&_`
  = warning: this changes meaning in Rust 2024
help: remove the unnecessary binding modifier
  |
3 -         Some(ref n) => n.len(),
3 +         Some(n) => n.len(),
  |
```

**Silent:**

```rust
pub fn name_len(name: &Option<String>) -> usize {
    match name {
        Some(n) => n.len(),
        None => 0,
    }
}
```

**When it is right.** Always: matching `Some(n)` on a `&Option<String>` already binds `n` by reference, so the `ref` repeats it — and on edition 2024 the bad program stops compiling, *cannot explicitly borrow within an implicitly-borrowing pattern*, which is why it is marked `compile_fail`. [Edition 2024: `ref` where the borrow is already implied](../the_ref_keyword/README.md#edition-2024-ref-where-the-borrow-is-already-implied).

## Clippy, opt-in

### `mut_mut`

**clippy · pedantic** · allow by default · fires on `&mut x` where `x` is already a `&mut`, and on a `&mut &mut T` type

```rust
pub fn bump(mut n: &mut i32) {
    let r = &mut n;
    **r += 1;
}

fn skip_first(t: &mut &mut [i32]) {
    let whole = std::mem::take(t);
    *t = &mut whole[1..];
}

pub fn rest_len() -> usize {
    let mut data = [1, 2, 3];
    let mut view: &mut [i32] = &mut data;
    skip_first(&mut view);
    view.len()
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::mut_mut bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: this expression mutably borrows a mutable reference
 --> bad.rs:2:13
  |
2 |     let r = &mut n;
  |             ^^^^^^ help: reborrow instead: `&mut *n`
  |

warning: a type of form `&mut &mut _`
 --> bad.rs:6:18
  |
6 | fn skip_first(t: &mut &mut [i32]) {
  |                  ^^^^^^^^^^^^^^^ help: remove the extra `&mut`: `&mut [i32]`
  |
```

**Silent:**

```rust
pub fn bump(n: &mut i32) {
    let r = &mut *n;
    *r += 1;
}

#[expect(clippy::mut_mut, reason = "re-points the caller's slice")]
fn skip_first(t: &mut &mut [i32]) {
    let whole = std::mem::take(t);
    *t = &mut whole[1..];
}

pub fn rest_len() -> usize {
    let mut data = [1, 2, 3];
    let mut view: &mut [i32] = &mut data;
    skip_first(&mut view);
    view.len()
}
```

**When it is right.** Right on the expression: `&mut n` makes a `&mut &mut i32` and needs `mut n` on the parameter (without it, `E0596`), where [the reborrow](../../reborrowing/README.md#what-actually-got-passed) `&mut *n` needs neither. Noise on the type: `skip_first` moves the caller's slice, and the `&mut [i32]` its `help:` offers can only [re-point its own copy](../repointing_a_slice/README.md#a-plain-mut-i32-re-points-its-own-copy) — `#[expect]` with a reason keeps the signature and the lint. It does not flag the call `skip_first(&mut view)`, which builds the same type on purpose, nor `std::mem::swap(&mut a, &mut b)` [below](#what-no-lint-catches), which builds it by mistake.

### `trivially_copy_pass_by_ref`

**clippy · pedantic** · allow by default · fires on a reference parameter to a small `Copy` type

```rust
fn double(n: &i32) -> i32 {
    *n * 2
}

pub fn run() -> i32 {
    let n = 21;
    double(&n)
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::trivially_copy_pass_by_ref bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: this argument (4 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
 --> bad.rs:1:14
  |
1 | fn double(n: &i32) -> i32 {
  |              ^^^^ help: consider passing by value instead: `i32`
  |
```

**Silent:**

```rust
fn double(n: i32) -> i32 {
    n * 2
}

pub fn run() -> i32 {
    let n = 21;
    double(n)
}
```

**When it is right.** Right for an `i32`: the value is 4 bytes, the reference to it 8, and the body needs a `*` to use it. The limit defaults to the target's pointer width, so its docs warn that a case flagged on a 64-bit target may pass on a 32-bit one. It skips `pub fn` (`avoid-breaking-exported-api`), which is why `double` is private. Noise where the address matters: its docs describe a false positive that led to undefined behaviour in `unsafe` code.

### `borrow_as_ptr`

**clippy · pedantic** · allow by default · since 1.60.0 · fires on `&x as *const T` and `&mut x as *mut T`

```rust
pub fn bump() -> i32 {
    let mut n = 5;
    let p = &mut n as *mut i32;
    unsafe { *p += 1 };
    let q = &n as *const i32;
    unsafe { *q }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::borrow_as_ptr bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: borrow as raw pointer
 --> bad.rs:3:13
  |
3 |     let p = &mut n as *mut i32;
  |             ^^^^^^^^^^^^^^^^^^ help: try: `&raw mut n`
  |

warning: borrow as raw pointer
 --> bad.rs:5:13
  |
5 |     let q = &n as *const i32;
  |             ^^^^^^^^^^^^^^^^ help: try: `&raw const n`
  |
```

**Silent:**

```rust
pub fn bump() -> i32 {
    let mut n = 5;
    let p = &raw mut n;
    unsafe { *p += 1 };
    let q = &raw const n;
    unsafe { *q }
}
```

**When it is right.** Right: `&raw const n` makes the pointer without making a reference first, and its docs give the reason — no reference to an unaligned or uninitialized place is created on the way. [The `*` that never dereferences](../../where_the_sigil_sits/README.md#the-that-never-dereferences).

### `ref_as_ptr`

**clippy · pedantic** · allow by default · since 1.78.0 · fires on an `as` cast from a reference to a raw pointer

```rust
pub fn address(r: &i32) -> *const i32 {
    r as *const i32
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::ref_as_ptr bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: reference as raw pointer
 --> bad.rs:2:5
  |
2 |     r as *const i32
  |     ^^^^^^^^^^^^^^^ help: try: `std::ptr::from_ref::<i32>(r)`
  |
```

**Silent:**

```rust
pub fn address(r: &i32) -> *const i32 {
    std::ptr::from_ref(r)
}
```

**When it is right.** Right: an `as` cast can change mutability or pointee type without a word, and `ptr::from_ref` cannot. Its suggestion spells the type, `from_ref::<i32>`; inference does as well. Where the return type already says `*const i32`, a plain `r` coerces and is silent too.

### `ptr_as_ptr`

**clippy · pedantic** · allow by default · since 1.51.0 · fires on an `as` cast between raw pointers of the same constness

```rust
pub fn first_byte(n: &u32) -> u8 {
    let p: *const u32 = n;
    let bytes = p as *const u8;
    unsafe { *bytes }
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::ptr_as_ptr bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: `as` casting between raw pointers without changing their constness
 --> bad.rs:3:17
  |
3 |     let bytes = p as *const u8;
  |                 ^^^^^^^^^^^^^^ help: try `pointer::cast`, a safer alternative: `p.cast::<u8>()`
  |
```

**Silent:**

```rust
pub fn first_byte(n: &u32) -> u8 {
    let p: *const u32 = n;
    let bytes = p.cast::<u8>();
    unsafe { *bytes }
}
```

**When it is right.** Right: `.cast::<u8>()` can change only the pointee type, where `as` would as quietly turn a `*const` into a `*mut` or into a `usize`. [`*const T` and `*mut T` differ in more than a lint](../pointer_types_compared/README.md#const-t-and-mut-t-differ-in-more-than-a-lint).

### `needless_pass_by_ref_mut`

**clippy · nursery** · allow by default · since 1.73.0 · fires on a `&mut` parameter that is never used mutably

```rust
fn total(scores: &mut [i32]) -> i32 {
    scores.iter().sum()
}

pub fn report() -> i32 {
    let mut scores = [90, 85];
    total(&mut scores)
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::needless_pass_by_ref_mut bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: this parameter is a mutable reference but is not used mutably
 --> bad.rs:1:18
  |
1 | fn total(scores: &mut [i32]) -> i32 {
  |                  ^----^^^^^
  |                   |
  |                   help: consider removing this `mut`
  |
```

**Silent:**

```rust
fn total(scores: &[i32]) -> i32 {
    scores.iter().sum()
}

pub fn report() -> i32 {
    let scores = [90, 85];
    total(&scores)
}
```

**When it is right.** Right: the caller gives up access to `scores` for the call, and declares it `mut`, for a function that only reads. It skips `pub fn`. A `&mut Vec<i32>` that is only read also gets `ptr_arg`, on by default.

### `ref_patterns`

**clippy · restriction** · allow by default · since 1.71.0 · fires on every `ref` in a pattern

```rust
pub fn name_len(name: Option<String>) -> usize {
    let len = match name {
        Some(ref n) => n.len(),
        None => 0,
    };
    drop(name);
    len
}
```

```text title="clippy-driver --edition 2024 --crate-type lib -W clippy::ref_patterns bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: usage of ref pattern
 --> bad.rs:3:14
  |
3 |         Some(ref n) => n.len(),
  |              ^^^^^
  |
  = help: consider using `&` for clarity instead
```

**Silent:**

```rust
pub fn name_len(name: Option<String>) -> usize {
    let len = match &name {
        Some(n) => n.len(),
        None => 0,
    };
    drop(name);
    len
}
```

**When it is right.** A house style, not a bug: its docs say `ref` confuses readers who have not met it, and `match &name` with a plain `Some(n)` binds the same `&String` through [match ergonomics](../../../30_Pattern_Matching/match_ergonomics/README.md). Noise where `ref` is [still the natural spelling](../the_ref_keyword/README.md#where-ref-is-still-the-natural-spelling).

## What no lint catches

Each program below compiles with no output under rustc and clippy's defaults. With clippy's `pedantic`, `nursery` and `restriction` groups all on, what fires is generic — `must_use_candidate`, `missing_inline_in_public_items`, `implicit_return`, `undocumented_unsafe_blocks` and the like — and none of it names the mistake. The first three are undefined behaviour: Miri (nightly 2026-08-27, [a tool that needs nightly](../../../05_Tooling/nightly/README.md#when-nightly-is-right)) reported each one. The last three ran on rustc 1.98.0.

```rust
pub fn first_after_push() -> i32 {
    let mut v = vec![1, 2, 3];
    let first: *const i32 = v.as_ptr();
    v.push(4);
    unsafe { *first }
}
```

`v.push(4)` reallocates the three-element buffer, and `first` still points at the old one — Miri: *memory access failed: … has been freed, so this pointer is dangling*. With `let first: &i32 = &v[0];` the same program is `E0502` ([a borrow is a loan](../a_borrow_is_a_loan/README.md#reading-a-borrow-error)); a raw pointer records no loan. [Iterator invalidation](../../../31_C_and_Cpp/iterator_invalidation/README.md) is the C++ version.

```rust
fn first_byte_ptr() -> *const u8 {
    let s = String::from("hi");
    s.as_ptr()
}

pub fn first_byte() -> u8 {
    unsafe { *first_byte_ptr() }
}
```

`s` frees its buffer when `first_byte_ptr` returns — Miri: *memory access failed: … has been freed*. `dangling_pointers_from_locals` catches `&raw const n` returned this way and `dangling_pointers_from_temporaries` catches `String::from("hi").as_ptr()` on one line; a named `String` whose buffer pointer leaves the function is neither. [Use-after-free](../../../31_C_and_Cpp/use_after_free/README.md).

```rust
fn writable(p: *const i32) -> *mut i32 {
    p.cast_mut()
}

pub fn overwrite() -> i32 {
    let x = 5;
    let p = writable(&x);
    unsafe { *p = 6 };
    x
}
```

`writable` turns a pointer made from `&x` into a `*mut`, and the write lands on `x`, which is not `mut` — Miri: *attempting a write access … but that tag only grants SharedReadOnly permission for this location*. rustc's `invalid_reference_casting`, deny by default, refuses the same write when the cast is in the same function (`&x as *const i32 as *mut i32`: *assigning to `&T` is undefined behavior, consider using an `UnsafeCell`*). Through a helper it sees nothing; pedantic's `borrow_as_ptr` suggests `&raw const x` for the argument, which is about the borrow, not the write.

```rust
use std::cell::RefCell;

pub fn log_len() -> usize {
    let log = RefCell::new(Vec::<i32>::new());
    let mut writer = log.borrow_mut();
    writer.push(1);
    log.borrow().len()
}
```

Compiles, and panics when called: *RefCell already mutably borrowed*. `writer` is still alive at `log.borrow()` — the many-readers-or-one-writer rule, checked at run time because [`RefCell`](../../../09_Advanced/interior_mutability/README.md) moved it there. Writing `log.borrow_mut().push(1);` as one statement ends that borrow at the `;`, and the function returns `1`.

```rust
pub fn swap_values() -> (i32, i32) {
    let mut x = 1;
    let mut y = 2;
    let mut a: &mut i32 = &mut x;
    let mut b: &mut i32 = &mut y;
    std::mem::swap(&mut a, &mut b);
    *a += 10;
    (x, y)
}
```

Returns `(1, 12)`: `mem::swap(&mut a, &mut b)` swapped the two references, so `a` points at `y`. `std::mem::swap(a, b)` swaps the integers, and the function returns `(12, 1)`. `mut_mut` flags `let r = &mut n;` on a `&mut` but not this argument.

```rust
pub fn same_place(a: &i32, b: &i32) -> bool {
    a == b
}

pub fn two_fives() -> bool {
    let x = 5;
    let y = 5;
    same_place(&x, &y)
}
```

Returns `true`: `==` on two `&i32` compares the integers, and two variables that both hold `5` are equal but not the same place. `std::ptr::eq(a, b)` compares the addresses and returns `false` — [`==` looks through the pointer](../pointer_types_compared/README.md#looks-through-the-pointer-ptreq-does-not).

The `.to_owned()` twins of the `.clone()` lints — on a `&&String`, on an `Rc`, on an `i32` — are on [the `ToOwned` page's list](../../../12_Traits/how_to_learn_to_owned/to_owned_lints/README.md#what-no-lint-catches).

## See also

- [References](../README.md) — the path this page sits beside
- [Reference errors](../reference_errors/README.md) — what the compiler refuses, where this page is what it warns about
- [A borrow is a loan](../a_borrow_is_a_loan/README.md), [When you need the `*`](../when_you_need_the_star/README.md), [The `ref` keyword](../the_ref_keyword/README.md), [Re-pointing a slice](../repointing_a_slice/README.md) — where the ideas behind most of these lints are taught
- [Six pointer types, one table](../pointer_types_compared/README.md) — `Box`, `Rc` and raw pointers beside `&` and `&mut`
- [What reference explanations get wrong, run](../reference_claims_checked/README.md) — including a `static mut` reference from a published tutorial
- [Lints around `ToOwned`](../../../12_Traits/how_to_learn_to_owned/to_owned_lints/README.md) — the same kind of page, for `clone`, `to_owned` and `Cow`
- [Strict clippy](../../../05_Tooling/strict_lints/README.md) — turning whole lint groups on for a project, and what that costs

## Po polsku

Dwadzieścia pięć ostrzeżeń rustc i clippy, które dotyczą `&`, `&mut`, `*`, `ref`, `Box`, `Rc` i surowych wskaźników: dla każdego program, który je wywołuje, dokładny komunikat (rustc 1.98.0, clippy 0.1.98), wersja bez ostrzeżenia i kiedy lint ma rację, a kiedy to szum. Przykład szumu: `mut_mut` zgłasza parametr `&mut &mut [i32]`, choć funkcja przesuwająca wycinek wywołującego potrzebuje dokładnie tego typu. `static_mut_refs` w edycji 2021 jest ostrzeżeniem, a w 2024 zatrzymuje kompilację; `mut_from_ref` w clippy domyślnie jest błędem.

Sześć pomyłek przechodzi bez słowa: surowy wskaźnik do bufora `Vec` po `push`, wskaźnik do bufora lokalnego `String` zwrócony z funkcji, zapis przez `*mut` zrobiony z `&x` w funkcji pomocniczej (te trzy to niezdefiniowane zachowanie, potwierdzone przez Miri), podwójne `borrow_mut`/`borrow` na `RefCell` (panika w czasie działania), `mem::swap(&mut a, &mut b)` zamieniające referencje zamiast wartości oraz `==` na referencjach, które porównuje wartości, a nie adresy.

**Szukaj po polsku:** `clippy needless_borrow` · `clippy mut_mut` · `rust static_mut_refs` · `rust dangling pointer as_ptr` · wiszący wskaźnik · referencja do referencji
