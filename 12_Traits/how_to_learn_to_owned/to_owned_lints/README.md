# Lints around `ToOwned`: bad and good, run

[How to learn `ToOwned`](../README.md) › **Beside the steps** · the errors, not the warnings: [Every `ToOwned` error, and its fix](../to_owned_errors/README.md)

**Level:** 201 · a reference, by lint

**One line:** Twenty-two warnings from rustc and clippy that concern `ToOwned`, `clone`, `Cow` and their neighbours — each with a program that triggers it, what the tool prints, a version that is silent, and when the lint is right — plus the mistakes no lint catches at all.

Every transcript was recorded on rustc 1.98.0 and clippy 0.1.98 by running the file shown, and a sample of them re-run byte for byte for this page; the lines dropped from each are only the documentation link and the `#[warn(...)]` note. Both programs of every pair are compiled on every build by [`check_fences.py`](../../../tools/check_fences.py). What the build does not re-run is clippy, so a later clippy can reword a warning.

## Turning them on

rustc's lints and clippy's *warn by default* groups need nothing: `cargo clippy` prints them. The opt-in ones take a flag or a `Cargo.toml` entry:

```sh
cargo clippy -- -W clippy::implicit_clone -W clippy::assigning_clones
```

```toml
[lints.clippy]
implicit_clone = "warn"
assigning_clones = "warn"
```

## Find the lint

| Lint | From | On by default? | Fires on |
|---|---|---|---|
| [`noop_method_call`](#noop_method_call) | rustc | yes | `.clone()` on a `&str` |
| [`suspicious_double_ref_op`](#suspicious_double_ref_op) | rustc | yes | `.clone()` on a `&&String` |
| [`mismatched_lifetime_syntaxes`](#mismatched_lifetime_syntaxes) | rustc | yes | `-> Cow<str>` borrowing from an argument |
| [`unused_must_use`](#unused_must_use) | rustc | yes | `s.to_owned();` as a statement |
| [`suspicious_to_owned`](#suspicious_to_owned) | clippy · suspicious | yes | `.to_owned()` on a `Cow` |
| [`unnecessary_to_owned`](#unnecessary_to_owned) | clippy · perf | yes | an owned copy made only to be borrowed back |
| [`cmp_owned`](#cmp_owned) | clippy · perf | yes | an owned copy made only to compare |
| [`owned_cow`](#owned_cow) | clippy · style | yes | `Cow<'_, String>` |
| [`ptr_arg`](#ptr_arg) | clippy · style | yes | `&String`, `&Vec<T>` and `&PathBuf` parameters |
| [`iter_cloned_collect`](#iter_cloned_collect) | clippy · style | yes | `.iter().cloned().collect()` on a slice |
| [`map_clone`](#map_clone) | clippy · style | yes | `.map(\|n\| n.clone())` |
| [`unnecessary_owned_empty_strings`](#unnecessary_owned_empty_strings) | clippy · style | yes | `&String::new()` where a `&str` will do |
| [`clone_on_copy`](#clone_on_copy) | clippy · complexity | yes | `.clone()` on an `i32` |
| [`useless_format`](#useless_format) | clippy · complexity | yes | `format!` with nothing to format |
| [`implicit_clone`](#implicit_clone) | clippy · pedantic | no | `.to_owned()` and `.to_vec()` on values already owned |
| [`assigning_clones`](#assigning_clones) | clippy · pedantic | no | `a = b.clone()` and `a = s.to_owned()` |
| [`inefficient_to_string`](#inefficient_to_string) | clippy · pedantic | no | `.to_string()` on a `&&str` |
| [`cloned_instead_of_copied`](#cloned_instead_of_copied) | clippy · pedantic | no | `.cloned()` on `Copy` items |
| [`manual_string_new`](#manual_string_new) | clippy · pedantic | no | `"".to_owned()` |
| [`redundant_clone`](#redundant_clone) | clippy · nursery | no | a copy of a value that is never used again |
| [`str_to_string`](#str_to_string) | clippy · restriction | no | `"hello".to_string()` |
| [`clone_on_ref_ptr`](#clone_on_ref_ptr) | clippy · restriction | no | `rc.clone()` |

## `rustc`, on by default

### `noop_method_call`

**rustc** · warn by default · since 1.73.0 · fires on `.clone()` on a `&str`

```rust
fn main() {
    let greeting: &str = "hello";
    let copy: &str = greeting.clone();
    println!("{copy}");
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: call to `.clone()` on a reference in this situation does nothing
 --> bad.rs:3:30
  |
3 |     let copy: &str = greeting.clone();
  |                              ^^^^^^^^ help: remove this redundant call
  |
  = note: the type `str` does not implement `Clone`, so calling `clone` on `&str` copies the reference, which does not do anything and can be removed
```

**Silent:**

```rust
fn main() {
    let greeting: &str = "hello";
    let copy: &str = greeting;
    println!("{copy}");
}
```

**When it is right.** Always right in concrete code: the call copies a reference and nothing else. It watches `.clone()` only — `.to_owned()` on the same `&str` is a real conversion, and on a `&T` whose `T` is not `Clone` it copies the reference just as silently ([step 6](../the_blanket_to_owned/README.md#steps-4-and-5-meet-here)).

### `suspicious_double_ref_op`

**rustc** · warn by default · fires on `.clone()` on a `&&String`

```rust
fn main() {
    let names = [String::from("ada"), String::from("grace")];
    let refs: Vec<&String> = names.iter().collect();
    for r in &refs {
        let name = r.clone();
        println!("{name}");
    }
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: using `.clone()` on a double reference, which returns `&std::string::String` instead of cloning the inner type
 --> bad.rs:5:21
  |
5 |         let name = r.clone();
  |                     ^^^^^^^^
  |
```

**Silent:**

```rust
fn main() {
    let names = [String::from("ada"), String::from("grace")];
    let refs: Vec<&String> = names.iter().collect();
    for r in &refs {
        let name: String = String::clone(r);
        println!("{name}");
    }
}
```

**When it is right.** Right when you meant the `String`; if you only wanted the reference, write `*r`. Its blind spot is below: `r.to_owned()` does the same thing and nothing warns.

### `mismatched_lifetime_syntaxes`

**rustc** · warn by default · since 1.89.0 · fires on `-> Cow<str>` borrowing from an argument

```rust
use std::borrow::Cow;

fn exclaim(s: &str) -> Cow<str> {
    if s.ends_with('!') {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{s}!"))
    }
}

fn main() {
    println!("{} {}", exclaim("hi"), exclaim("hey!"));
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: hiding a lifetime that's elided elsewhere is confusing
 --> bad.rs:3:15
  |
3 | fn exclaim(s: &str) -> Cow<str> {
  |               ^^^^     ^^^^^^^^ the same lifetime is hidden here
  |               |
  |               the lifetime is elided here
  |
  = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
  |
3 | fn exclaim(s: &str) -> Cow<'_, str> {
  |                            +++
```

**Silent:**

```rust
use std::borrow::Cow;

fn exclaim(s: &str) -> Cow<'_, str> {
    if s.ends_with('!') {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{s}!"))
    }
}

fn main() {
    println!("{} {}", exclaim("hi"), exclaim("hey!"));
}
```

**When it is right.** Right: `Cow<'_, str>` shows at a glance that the result borrows from `s`. It is about how the signature reads — the code was already correct.

### `unused_must_use`

**rustc** · warn by default · `to_owned` and `clone` are `#[must_use]` since 1.27.0 · fires on `s.to_owned();` as a statement

```rust
fn main() {
    let s = "hello";
    s.to_owned();
    println!("{s}");
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: unused return value of `std::borrow::ToOwned::to_owned` that must be used
 --> bad.rs:3:5
  |
3 |     s.to_owned();
  |     ^^^^^^^^^^^^
  |
  = note: cloning is often expensive and is not expected to have side effects
help: use `let _ = ...` to ignore the resulting value
  |
3 |     let _ = s.to_owned();
  |     +++++++
```

**Silent:**

```rust
fn main() {
    let s = "hello";
    let owned: String = s.to_owned();
    println!("{owned}");
}
```

**When it is right.** Always right: a copy thrown away is an allocation for nothing. `.to_string();`, `.to_vec();` and `cow.clone().into_owned();` as statements do not warn.

## Clippy, on by default

### `suspicious_to_owned`

**clippy · suspicious** · warn by default · since 1.65.0 · fires on `.to_owned()` on a `Cow`

```rust
use std::borrow::Cow;

fn main() {
    let cow: Cow<'_, str> = Cow::Borrowed("hello");
    let copy = cow.to_owned();
    println!("{copy}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: this `to_owned` call clones the `Cow<'_, str>` itself and does not cause its contents to become owned
 --> bad.rs:5:16
  |
5 |     let copy = cow.to_owned();
  |                ^^^^^^^^^^^^^^
  |
help: depending on intent, either make the `Cow` an `Owned` variant
  |
5 |     let copy = cow.into_owned();
  |                    ++
help: or clone the `Cow` itself
  |
5 -     let copy = cow.to_owned();
5 +     let copy = cow.clone();
  |
```

**Silent:**

```rust
use std::borrow::Cow;

fn main() {
    let cow: Cow<'_, str> = Cow::Borrowed("hello");
    let owned: String = cow.into_owned();
    println!("{owned}");
}
```

**When it is right.** Right nearly every time — `into_owned()` was meant ([step 8](../to_owned_traps/README.md#cow-to_owned-is-not-into_owned)). If a copy of the `Cow` itself was really wanted, say `.clone()`, as its second `help:` offers.

### `unnecessary_to_owned`

**clippy · perf** · warn by default · since 1.59.0 · fires on an owned copy made only to be borrowed back

```rust
fn shout(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    let word: &str = "hello";
    println!("{}", shout(&word.to_owned()));
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: unnecessary use of `to_owned`
 --> bad.rs:7:26
  |
7 |     println!("{}", shout(&word.to_owned()));
  |                          ^^^^^^^^^^^^^^^^ help: use: `word`
  |
```

```rust
fn main() {
    let bytes: &[u8] = b"abc";
    for b in bytes.to_vec() {
        println!("{b}");
    }
}
```

```text title="clippy-driver --edition 2024 bad_iter.rs — rustc 1.98.0, clippy 0.1.98"
warning: unnecessary use of `to_vec`
 --> bad_iter.rs:3:14
  |
3 |     for b in bytes.to_vec() {
  |              ^^^^^^^^^^^^^^
  |
help: remove any references to the binding
  |
3 -     for b in bytes.to_vec() {
3 +     for b in bytes {
  |
```

**Silent:**

```rust
fn shout(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    let word: &str = "hello";
    println!("{}", shout(word));
}
```

```rust
fn main() {
    let bytes: &[u8] = b"abc";
    for b in bytes {
        println!("{b}");
    }
}
```

**When it is right.** Right: the allocation exists only to be lent straight back. Its reach is narrower than its name — `word.to_string().as_str()` and `&owned.clone()` on a `String` pass unflagged — and its docs list a false positive (rust-clippy#8148) for `into_iter` on a copy whose original is mutated later.

### `cmp_owned`

**clippy · perf** · warn by default · fires on an owned copy made only to compare

```rust
fn main() {
    let input: &str = "yes";
    let expected = String::from("yes");
    if input.to_owned() == expected {
        println!("match");
    }
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: this creates an owned instance just for comparison
 --> bad.rs:4:8
  |
4 |     if input.to_owned() == expected {
  |        ^^^^^^^^^^^^^^^^ help: try: `input`
  |
```

**Silent:**

```rust
fn main() {
    let input: &str = "yes";
    let expected = String::from("yes");
    if input == expected {
        println!("match");
    }
}
```

**When it is right.** Right: `&str == String` already works, so the copy compares nothing a borrow could not.

### `owned_cow`

**clippy · style** · warn by default · since 1.87.0 · fires on `Cow<'_, String>`

```rust
use std::borrow::Cow;

fn describe(name: Cow<'_, String>) -> usize {
    name.len()
}

fn main() {
    let owned = String::from("ferris");
    println!("{}", describe(Cow::Borrowed(&owned)));
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: needlessly owned Cow type
 --> bad.rs:3:27
  |
3 | fn describe(name: Cow<'_, String>) -> usize {
  |                           ^^^^^^ help: use: `str`
  |
```

**Silent:**

```rust
use std::borrow::Cow;

fn describe(name: Cow<'_, str>) -> usize {
    name.len()
}

fn main() {
    let owned = String::from("ferris");
    println!(
        "{} {}",
        describe(Cow::Borrowed(&owned)),
        describe(Cow::Borrowed("crab"))
    );
}
```

**When it is right.** Right: a `Cow` of the *owned* type cannot borrow a literal, and `Cow<'_, str>` can. It skips public items by default (`avoid-breaking-exported-api`), and `CString`/`CStr` are not a drop-in swap.

### `ptr_arg`

**clippy · style** · warn by default · fires on `&String`, `&Vec<T>` and `&PathBuf` parameters

```rust
use std::path::PathBuf;

fn report(name: &String, scores: &Vec<i32>, file: &PathBuf) -> String {
    let total: i32 = scores.iter().sum();
    format!("{}: {total} in {}", name.to_uppercase(), file.display())
}

fn main() {
    let name = String::from("ada");
    let scores = vec![90, 85];
    println!("{}", report(&name, &scores, &PathBuf::from("a.txt")));
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: writing `&String` instead of `&str` involves a new object where a slice will do
 --> bad.rs:3:17
  |
3 | fn report(name: &String, scores: &Vec<i32>, file: &PathBuf) -> String {
  |                 ^^^^^^^
  |
help: change this to
  |
3 - fn report(name: &String, scores: &Vec<i32>, file: &PathBuf) -> String {
3 + fn report(name: &str, scores: &Vec<i32>, file: &PathBuf) -> String {
  |

warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
 --> bad.rs:3:34
  |
3 | fn report(name: &String, scores: &Vec<i32>, file: &PathBuf) -> String {
  |                                  ^^^^^^^^^
  |
help: change this to
  |
3 - fn report(name: &String, scores: &Vec<i32>, file: &PathBuf) -> String {
3 + fn report(name: &String, scores: &[i32], file: &PathBuf) -> String {
  |

warning: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do
 --> bad.rs:3:51
  |
3 | fn report(name: &String, scores: &Vec<i32>, file: &PathBuf) -> String {
  |                                                   ^^^^^^^^
  |
help: change this to
  |
3 - fn report(name: &String, scores: &Vec<i32>, file: &PathBuf) -> String {
3 + fn report(name: &String, scores: &Vec<i32>, file: &Path) -> String {
  |
```

**Silent:**

```rust
use std::path::{Path, PathBuf};

fn report(name: &str, scores: &[i32], file: &Path) -> String {
    let total: i32 = scores.iter().sum();
    format!("{}: {total} in {}", name.to_uppercase(), file.display())
}

fn main() {
    let name = String::from("ada");
    let scores = vec![90, 85];
    println!("{}", report(&name, &scores, &PathBuf::from("a.txt")));
}
```

**When it is right.** Right: a borrowed-half parameter accepts every caller ([step 3](../owned_and_borrowed_types/README.md)). Noise only where a signature must match an existing function-pointer type. It does not fire on a `&String` used only inside `format!("{name}")`.

### `iter_cloned_collect`

**clippy · style** · warn by default · fires on `.iter().cloned().collect()` on a slice

```rust
fn main() {
    let slice: &[i32] = &[1, 2, 3];
    let v: Vec<i32> = slice.iter().cloned().collect();
    println!("{v:?}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: called `.iter().cloned().collect()` on a slice to create a `Vec`
 --> bad.rs:3:28
  |
3 |     let v: Vec<i32> = slice.iter().cloned().collect();
  |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: calling `.to_vec()` is both faster and more readable
  |
3 -     let v: Vec<i32> = slice.iter().cloned().collect();
3 +     let v: Vec<i32> = slice.to_vec();
  |
```

**Silent:**

```rust
fn main() {
    let slice: &[i32] = &[1, 2, 3];
    let v: Vec<i32> = slice.to_vec();
    println!("{v:?}");
}
```

**When it is right.** Right: `to_vec` is the slice's own `ToOwned`, and says so.

### `map_clone`

**clippy · style** · warn by default · fires on `.map(|n| n.clone())`

```rust
fn main() {
    let names = [String::from("ada"), String::from("grace")];
    let copies: Vec<String> = names.iter().map(|n| n.clone()).collect();
    println!("{copies:?}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: you are using an explicit closure for cloning elements
 --> bad.rs:3:31
  |
3 |     let copies: Vec<String> = names.iter().map(|n| n.clone()).collect();
  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider calling the dedicated `cloned` method: `names.iter().cloned()`
  |
```

**Silent:**

```rust
fn main() {
    let names = [String::from("ada"), String::from("grace")];
    let copies: Vec<String> = names.to_vec();
    println!("{copies:?}");
}
```

**When it is right.** Right. Applying its suggestion literally, `.iter().cloned().collect()`, then trips `iter_cloned_collect` above — so the good version goes straight to `to_vec()`. `.map(|n| n.to_owned())` is not caught.

### `unnecessary_owned_empty_strings`

**clippy · style** · warn by default · since 1.62.0 · fires on `&String::new()` where a `&str` will do

```rust
fn main() {
    let parts = ["a", "b", "c"];
    let joined = parts.join(&String::new());
    let cleaned = "a-b".replace('-', &"".to_owned());
    println!("{joined} {cleaned}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: usage of `&String::new()` for a function expecting a `&str` argument
 --> bad.rs:3:29
  |
3 |     let joined = parts.join(&String::new());
  |                             ^^^^^^^^^^^^^^ help: try: `""`
  |

warning: unnecessary use of `to_owned`
 --> bad.rs:4:38
  |
4 |     let cleaned = "a-b".replace('-', &"".to_owned());
  |                                      ^^^^^^^^^^^^^^ help: use: `""`
  |
```

**Silent:**

```rust
fn main() {
    let parts = ["a", "b", "c"];
    let joined = parts.join("");
    let cleaned = "a-b".replace('-', "");
    println!("{joined} {cleaned}");
}
```

**When it is right.** Right, for readability rather than speed — `String::new()` does not allocate. The second warning is `unnecessary_to_owned` catching `&"".to_owned()`.

### `clone_on_copy`

**clippy · complexity** · warn by default · fires on `.clone()` on an `i32`

```rust
fn main() {
    let n: i32 = 42;
    let m = n.clone();
    println!("{n} {m}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: using `clone` on type `i32` which implements the `Copy` trait
 --> bad.rs:3:13
  |
3 |     let m = n.clone();
  |             ^^^^^^^^^ help: try removing the `clone` call: `n`
  |
```

**Silent:**

```rust
fn main() {
    let n: i32 = 42;
    let m = n;
    println!("{n} {m}");
}
```

**When it is right.** Always right. `n.to_owned()`, which does the same, is not flagged by anything — see below.

### `useless_format`

**clippy · complexity** · warn by default · fires on `format!` with nothing to format

```rust
fn main() {
    let greeting = format!("hello");
    let name: &str = "ferris";
    let copy = format!("{name}");
    println!("{greeting} {copy}");
}
```

```text title="clippy-driver --edition 2024 bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: useless use of `format!`
 --> bad.rs:2:20
  |
2 |     let greeting = format!("hello");
  |                    ^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"hello".to_string()`
  |

warning: useless use of `format!`
 --> bad.rs:4:16
  |
4 |     let copy = format!("{name}");
  |                ^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `name.to_string()`
  |
```

**Silent:**

```rust
fn main() {
    let greeting = "hello".to_owned();
    let name: &str = "ferris";
    let copy = name.to_owned();
    println!("{greeting} {copy}");
}
```

**When it is right.** Right. Its suggestion is `.to_string()`, which `str_to_string` below flags if you turn that on — the good version uses `to_owned()` and satisfies both.

## Clippy, opt-in

### `implicit_clone`

**clippy · pedantic** · allow by default · since 1.52.0 · fires on `.to_owned()` and `.to_vec()` on values already owned

```rust
fn main() {
    let name = String::from("ferris");
    let copy = name.to_owned();
    let nums = vec![1, 2, 3];
    let more = nums.to_vec();
    println!("{name} {copy} {nums:?} {more:?}");
}
```

```text title="clippy-driver --edition 2024 -W clippy::implicit_clone bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: implicitly cloning a `String` by calling `to_owned` on its dereferenced type
 --> bad.rs:3:16
  |
3 |     let copy = name.to_owned();
  |                ^^^^^^^^^^^^^^^ help: consider using: `name.clone()`
  |

warning: implicitly cloning a `Vec` by calling `to_vec` on its dereferenced type
 --> bad.rs:5:16
  |
5 |     let more = nums.to_vec();
  |                ^^^^^^^^^^^^^ help: consider using: `nums.clone()`
  |
```

**Silent:**

```rust
fn main() {
    let name = String::from("ferris");
    let copy = name.clone();
    let nums = vec![1, 2, 3];
    let more = nums.clone();
    println!("{name} {copy} {nums:?} {more:?}");
}
```

**When it is right.** Right when the value is already owned: `clone()` says what happens. It also catches `to_string()` on a `String` and `to_owned()` on an `Rc`. It is off by default because some code writes `to_owned` on purpose, so the line keeps working if the type later becomes a borrow.

### `assigning_clones`

**clippy · pedantic** · allow by default · since 1.78.0 · fires on `a = b.clone()` and `a = s.to_owned()`

```rust
fn main() {
    let mut label = String::from("draft");
    println!("{label}");
    let src = String::from("final");
    label = src.clone();
    println!("{label} {src}");
    label = "done".to_owned();
    println!("{label}");
}
```

```text title="clippy-driver --edition 2024 -W clippy::assigning_clones bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: assigning the result of `Clone::clone()` may be inefficient
 --> bad.rs:5:5
  |
5 |     label = src.clone();
  |     ^^^^^^^^^^^^^^^^^^^ help: use `clone_from()`: `label.clone_from(&src)`
  |

warning: assigning the result of `ToOwned::to_owned()` may be inefficient
 --> bad.rs:7:5
  |
7 |     label = "done".to_owned();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `clone_into()`: `"done".clone_into(&mut label)`
  |
```

**Silent:**

```rust
fn main() {
    let mut label = String::from("draft");
    println!("{label}");
    let src = String::from("final");
    label.clone_from(&src);
    println!("{label} {src}");
    "done".clone_into(&mut label);
    println!("{label}");
}
```

**When it is right.** Right when the target holds a buffer worth reusing — a `String` refilled in a loop ([step 9](../clone_into_refills/README.md)). Noise when there is nothing to reuse; the rewrite reads worse.

### `inefficient_to_string`

**clippy · pedantic** · allow by default · since 1.40.0 · fires only below MSRV 1.82 · fires on `.to_string()` on a `&&str`

```rust
#[clippy::msrv = "1.81"]
fn main() {
    let words = ["alpha", "beta"];
    let owned: Vec<String> = words.iter().map(|w| w.to_string()).collect();
    println!("{owned:?}");
}
```

```text title="clippy-driver --edition 2024 -W clippy::inefficient_to_string bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: calling `to_string` on `&&str`
 --> bad.rs:4:51
  |
4 |     let owned: Vec<String> = words.iter().map(|w| w.to_string()).collect();
  |                                                   ^^^^^^^^^^^^^ help: try dereferencing the receiver: `(*w).to_string()`
  |
  = help: `&str` implements `ToString` through a slower blanket impl, but `str` has a fast specialization of `ToString`
```

**Silent:**

```rust
#[clippy::msrv = "1.81"]
fn main() {
    let words = ["alpha", "beta"];
    let owned: Vec<String> = words.iter().map(|&w| w.to_owned()).collect();
    println!("{owned:?}");
}
```

**When it is right.** History now: `&&str` stopped taking the slow path in Rust 1.82, so at a current MSRV clippy stays silent even when asked. It still fires for a crate that declares an older MSRV, which is what the attribute in the bad version does. (The inner form `#![clippy::msrv = "1.81"]` is `E0658` on stable.)

### `cloned_instead_of_copied`

**clippy · pedantic** · allow by default · since 1.53.0 · fires on `.cloned()` on `Copy` items

```rust
fn main() {
    let nums = [3, 1, 2];
    let max = nums.iter().cloned().max();
    println!("{max:?}");
}
```

```text title="clippy-driver --edition 2024 -W clippy::cloned_instead_of_copied bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: used `cloned` where `copied` could be used instead
 --> bad.rs:3:27
  |
3 |     let max = nums.iter().cloned().max();
  |                           ^^^^^^ help: try: `copied`
  |
```

**Silent:**

```rust
fn main() {
    let nums = [3, 1, 2];
    let max = nums.iter().copied().max();
    println!("{max:?}");
}
```

**When it is right.** Right: `copied` promises a cheap copy. Noise if the item type may stop being `Copy`.

### `manual_string_new`

**clippy · pedantic** · allow by default · since 1.65.0 · fires on `"".to_owned()`

```rust
fn main() {
    let mut buf = "".to_owned();
    buf.push_str("hello");
    println!("{buf}");
}
```

```text title="clippy-driver --edition 2024 -W clippy::manual_string_new bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: empty String is being created manually
 --> bad.rs:2:19
  |
2 |     let mut buf = "".to_owned();
  |                   ^^^^^^^^^^^^^ help: consider using: `String::new()`
  |
```

**Silent:**

```rust
fn main() {
    let mut buf = String::new();
    buf.push_str("hello");
    println!("{buf}");
}
```

**When it is right.** Consistency only — neither form allocates.

### `redundant_clone`

**clippy · nursery** · allow by default · since 1.32.0 · fires on a copy of a value that is never used again

```rust
fn consume(s: String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("ferris");
    println!("{}", consume(name.clone()));
}
```

```text title="clippy-driver --edition 2024 -W clippy::redundant_clone bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: redundant clone
 --> bad.rs:7:32
  |
7 |     println!("{}", consume(name.clone()));
  |                                ^^^^^^^^ help: remove this
  |
note: this value is dropped without further use
 --> bad.rs:7:28
  |
7 |     println!("{}", consume(name.clone()));
  |                            ^^^^
```

**Silent:**

```rust
fn consume(s: String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("ferris");
    println!("{}", consume(name));
}
```

**When it is right.** Right here: the original is dropped unused. `name.to_owned()` gets the same warning. It lives in nursery because its analysis misses cases and can misjudge complex borrows.

### `str_to_string`

**clippy · restriction** · allow by default · fires on `"hello".to_string()`

```rust
fn main() {
    let s: String = "hello".to_string();
    println!("{s}");
}
```

```text title="clippy-driver --edition 2024 -W clippy::str_to_string bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: `to_string()` called on a `&str`
 --> bad.rs:2:21
  |
2 |     let s: String = "hello".to_string();
  |                     ^^^^^^^^^^^^^^^^^^^ help: try: `"hello".to_owned()`
  |
```

**Silent:**

```rust
fn main() {
    let s: String = "hello".to_owned();
    println!("{s}");
}
```

**When it is right.** A house style, not a bug: `to_owned` for *make this owned*, `to_string` for *format this*. Noise for a team that prefers `to_string()` everywhere — the two cost the same ([the measurements](../../to_owned/README.md#for-everything-else-they-are-the-same-call)).

### `clone_on_ref_ptr`

**clippy · restriction** · allow by default · fires on `rc.clone()`

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared"));
    let b = a.clone();
    println!("{a} {b} {}", Rc::strong_count(&a));
}
```

```text title="clippy-driver --edition 2024 -W clippy::clone_on_ref_ptr bad.rs — rustc 1.98.0, clippy 0.1.98"
warning: using `.clone()` on a ref-counted pointer
 --> bad.rs:5:13
  |
5 |     let b = a.clone();
  |             ^^^^^^^^^ help: try: `std::rc::Rc::<std::string::String>::clone(&a)`
  |
```

**Silent:**

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    println!("{a} {b} {}", Rc::strong_count(&a));
}
```

**When it is right.** Right when readers should see that only a pointer is copied ([step 8](../to_owned_traps/README.md#rc-the-outer-type-is-the-first-candidate)). `a.to_owned()` slips past it — only `implicit_clone` catches that one.

## What no lint catches

Each of these is silent under `cargo clippy`'s defaults and under the lint that catches its `.clone()` twin. With clippy's `pedantic`, `nursery` and `restriction` groups all on, the first and third are still invisible; the notes say what sees the other two. They matter because `to_owned` is not one of the methods rustc's no-op lints watch.

```rust
fn main() {
    let names = [String::from("ada"), String::from("grace")];
    let refs: Vec<&String> = names.iter().collect();
    for r in &refs {
        let name = r.to_owned();
        println!("{name}");
    }
}
```

`.to_owned()` on a `&&String` hands back a `&String`, exactly as `.clone()` does — but only `.clone()` gets `suspicious_double_ref_op`. [Step 5](../the_dot_picks_first/README.md#where-clone-goes-further-and-where-it-never-does).

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared"));
    let b = a.to_owned();
    println!("{a} {b} {}", Rc::strong_count(&a));
}
```

`.to_owned()` on an `Rc` copies the pointer, and `clone_on_ref_ptr` — which catches `rc.clone()` — does not see it. `implicit_clone` does, if you turn it on. [Step 8](../to_owned_traps/README.md#rc-the-outer-type-is-the-first-candidate).

```rust
fn main() {
    let n: i32 = 42;
    let m = n.to_owned();
    println!("{n} {m}");
}
```

`.to_owned()` on an `i32` is `clone_on_copy`'s case under another name, and passes. [Step 6](../the_blanket_to_owned/README.md).

```rust
fn main() {
    let names = [String::from("ada"), String::from("grace")];
    let copies: Vec<String> = names.iter().map(|n| n.to_owned()).collect();
    println!("{copies:?}");
}
```

`.map(|n| n.to_owned())` is `map_clone`'s case, and passes. Only pedantic's `redundant_closure_for_method_calls` mentions it, suggesting `ToOwned::to_owned` as a path. [Where `Clone` will not do, §10](../where_clone_will_not_do/README.md#10-owning-a-list-of-words-and-the-that-still-gets-in-the-way).

And the one that matters most: `.to_owned()` on a `&T` whose `T` is not `Clone` returns the `&T` with no warning from anything, and the first sign is `E0308` wherever the result meets a type annotation — [error 4](../to_owned_errors/README.md#4-to_owned-on-a-ticket-hands-back-a-ticket).

## Removed

`clippy::string_to_string` no longer exists. Asking for it prints only this — the case it covered, `to_string()` on a `String`, is `implicit_clone`'s now:

```text title="clippy-driver --edition 2024 -W clippy::string_to_string bad.rs — clippy 0.1.98"
warning: lint `clippy::string_to_string` has been removed: `clippy::implicit_clone` covers those cases
  |
```

## See also

- [Every `ToOwned` error, and its fix](../to_owned_errors/README.md) — what the compiler refuses, where this page is what it merely warns about
- [Strict clippy](../../../05_Tooling/strict_lints/README.md) — turning whole lint groups on for a project, and what that costs
- [Where `Clone` will not do](../where_clone_will_not_do/README.md) — code that does not compile at all, beside code that does
- [Lints around borrowing forever](../../../18_Ownership/borrowing_forever_lints/README.md) — the same kind of page for lifetimes in structs, and a pattern no lint flags

## Po polsku

Dwadzieścia dwa ostrzeżenia rustc i clippy związane z `ToOwned`, `clone` i `Cow`: dla każdego program, który je wywołuje, dokładny komunikat (rustc 1.98.0, clippy 0.1.98), wersja bez ostrzeżenia i kiedy lint ma rację. Część działa domyślnie (`cargo clippy`), część trzeba włączyć flagą `-W clippy::nazwa` albo w sekcji `[lints.clippy]` w `Cargo.toml`.

Najważniejsza jest sekcja o tym, czego żaden lint nie łapie: `.to_owned()` na `&&String`, na `Rc`, na `i32` czy w `.map(|n| n.to_owned())` przechodzi bez słowa, choć te same wywołania z `.clone()` dostają ostrzeżenie. A `.to_owned()` na `&T`, gdy `T` nie jest `Clone`, zwraca referencję i pierwszym sygnałem jest dopiero błąd `E0308`.

**Szukaj po polsku:** `clippy unnecessary_to_owned` · `clippy implicit_clone` · `rust suspicious_to_owned Cow`
