# Every `HashMap` and `HashSet` error, and its fix

Beside [A first `HashMap`](../a_first_hashmap/README.md) and [A first `HashSet`](../a_first_hashset/README.md) · the warnings, not the errors: [Lints around `HashMap` and `HashSet`](../hashmap_lints/README.md)

**Level:** 101 → 201 · a reference, by symptom

**One line:** Eighteen compiler refusals, one run-time panic and one thing that is not an error at all, met by a first map or set: the missing `use`, an `Option` used as the number it wraps, `[]` on a key that is not there, a `String` key looked up the wrong way round, a value written through `get`, a map consumed by its own loop, a key type with no `Hash`. For each: the code, what rustc 1.98.0 prints, the mistake in a sentence, and a fix that compiles — and every fix is also a numbered block of [`hashmap_err_fixes.rs`](examples/hashmap_err_fixes.rs), which CI runs.

The transcripts were recorded from rustc 1.98.0 with `--crate-type lib --emit=metadata`, the way [`check_fences.py`](../../tools/check_fences.py) checks a fence, from a file named in each title, and stop before rustc's closing `aborting due to` summary. Where a transcript also pointed into the toolchain's own source (`--> /Users/…/library/std/src/collections/hash/map.rs`) those lines are dropped and the title says *Abridged*; nothing is substituted. Every fix was built the same way and printed nothing.

## Find the error

| # | Code | What rustc says | The mistake |
|---|---|---|---|
| 1 | `E0425`, `E0433` | [cannot find type `HashMap` in this scope](#1-no-use-line) | no `use` line |
| 2 | `E0282` | [type annotations needed for `HashMap<_, _>`](#2-no-type-for-new) | `HashMap::new()` with no type in sight |
| 3 | `E0308` | [expected `u32`, found `Option<&u32>`](#3-get-used-as-the-number) | `get` used as the number |
| 4 | `E0308` | [expected `Option<&u32>`, found integer](#4-get-compared-to-a-number) | `get(k) == 5` |
| 5 | `E0369` | [cannot add `{integer}` to `Option<&u32>`](#5-adding-to-get) | `get(k) + 1` |
| 6 | `E0277` | [can't compare `&u32` with `{integer}`](#6-a-borrowed-value-compared-to-a-number) | `count == 5` inside `for (_, count) in &map` |
| 7 | `E0507` | [cannot move out of index of `HashMap<u32, String>`](#7-moving-out-of-mapkey) | `let s: String = map[&k];` |
| 8 | panic | [no entry found for key](#8-a-missing-key-indexed) | `map["hello"]` when only `"Hello"` is there |
| 9 | `E0277` | [the trait bound `&str: Borrow<String>` is not satisfied](#9-str-keys-looked-up-with-a-string) | `&str` keys, `get(&some_string)` |
| 10 | `E0308` | [expected `&_`, found `String`](#10-a-string-passed-by-value-to-get) | `get(some_string)` |
| 11 | `E0594` | [cannot assign to `*count`, which is behind a `&` reference](#11-writing-through-get) | `*count += 1` on what `get` returned |
| 12 | `E0596` | [cannot borrow `*word_counts` as mutable, as it is behind a `&` reference](#12-entry-on-a-shared-reference) | `entry` through a `&HashMap` parameter |
| 13 | `E0596` | [cannot borrow `word_counts` as mutable, as it is not declared as mutable](#13-insert-without-mut) | `insert` on a `let` without `mut` |
| 14 | `E0368` | [binary assignment operation `+=` cannot be applied to type `&mut u32`](#14-the-missing-star) | `map.entry(k).or_insert(0) += 1` with no `*` |
| 15 | `E0382` | [borrow of moved value: `word_counts`](#15-the-map-is-gone-after-the-loop) | `for … in word_counts`, then `word_counts.len()` |
| 16 | `E0502` | [cannot borrow `word_counts` as mutable because it is also borrowed as immutable](#16-insert-inside-the-loop) | `insert` inside `for … in &word_counts` |
| 17 | `E0599` | [the method `insert` exists … but its trait bounds were not satisfied](#17-a-struct-key-without-eq-and-hash) | a struct key with no `Eq`, no `Hash` |
| 18 | `E0599` | [`f64: Eq` and `f64: Hash` not satisfied](#18-an-f64-key) | an `f64` key |
| 19 | `E0597` | [`text` does not live long enough](#19-str-keys-outliving-the-text) | `&str` keys borrowed from a `String` dropped first |
| 20 | none | [compiles under `#![deny(unused_must_use)]`](#20-a-discarded-insert-and-the-lint-that-does-not-fire) | `set.insert(x);` with the `bool` dropped |

Entries 1 to 7 and 9 to 19 are refusals. Entry 8 is a panic and entry 20 is not an error; both are here because a reader expects them to be.

## The map is not there yet

### 1. No `use` line

```rust,compile_fail
fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
}
```

```text title="rustc 1.98.0 on no_use.rs"
error[E0425]: cannot find type `HashMap` in this scope
 --> no_use.rs:2:26
  |
2 |     let mut word_counts: HashMap<&str, u32> = HashMap::new();
  |                          ^^^^^^^ not found in this scope
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |

error[E0433]: cannot find type `HashMap` in this scope
 --> no_use.rs:2:47
  |
2 |     let mut word_counts: HashMap<&str, u32> = HashMap::new();
  |                                               ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |
```

**The mistake.** `HashMap` is not in the prelude, the names every file gets for free. One line, two errors: one for the type in the annotation, one for the path in `HashMap::new()`. rustc's help is the fix, and `HashSet` needs the same line with its own name.

**The fix.**

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
}
```

**Read:** [A first `HashMap` — making one](../a_first_hashmap/README.md#1-making-one-use-new-and-the-type)

### 2. No type for `new`

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let word_counts = HashMap::new();
    println!("{}", word_counts.len());
}
```

```text title="rustc 1.98.0 on no_type_annotation.rs"
error[E0282]: type annotations needed for `HashMap<_, _>`
 --> no_type_annotation.rs:4:9
  |
4 |     let word_counts = HashMap::new();
  |         ^^^^^^^^^^^   -------------- type must be known at this point
  |
help: consider giving `word_counts` an explicit type, where the type for type parameter `K` is specified
  |
4 |     let word_counts: HashMap<K, V> = HashMap::new();
  |                    +++++++++++++++
```

**The mistake.** `HashMap::new()` has two type parameters and nothing in this program fills them in: `len()` works for a map of anything, so rustc is never told what the keys and values are. Usually the first `insert` settles it; here there is none.

**The fix.** Annotate, or insert something first.

```rust
use std::collections::HashMap;

fn main() {
    let annotated: HashMap<&str, u32> = HashMap::new();
    let mut inferred = HashMap::new();
    inferred.insert("Hello", 5);
    println!("{} {}", annotated.len(), inferred.len());   // 0 1
}
```

**Read:** [A first `HashMap` — making one](../a_first_hashmap/README.md#1-making-one-use-new-and-the-type)

## The `Option` is not the number

### 3. `get` used as the number

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    let count: u32 = word_counts.get("Hello");
    println!("{count}");
}
```

```text title="rustc 1.98.0 on get_as_value.rs"
error[E0308]: mismatched types
 --> get_as_value.rs:6:22
  |
6 |     let count: u32 = word_counts.get("Hello");
  |                ---   ^^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `Option<&u32>`
  |                |
  |                expected due to this
  |
  = note: expected type `u32`
             found enum `Option<&u32>`
```

**The mistake.** `get` cannot promise a value, because the key may be absent, so it returns `Option<&u32>`: `Some` of a reference, or `None`. That is not a `u32` and never converts to one on its own.

**The fix.** Keep the `Option`, or open it with a default.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    let found: Option<&u32> = word_counts.get("Hello");
    let count: u32 = word_counts.get("Hello").copied().unwrap_or(0);
    println!("{found:?} {count}");   // Some(5) 5
}
```

**Read:** [A first `HashMap` — `get` asks](../a_first_hashmap/README.md#3-get-asks-the-answer-comes-in-an-option), [`Some` and `None`](../../17_Option_and_Result/some_and_none/README.md), [`unwrap_or`](../../17_Option_and_Result/unwrap_or/README.md)

### 4. `get` compared to a number

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    if word_counts.get("Hello") == 5 {
        println!("five");
    }
}
```

```text title="rustc 1.98.0 on compare_get.rs"
error[E0308]: mismatched types
 --> compare_get.rs:6:36
  |
6 |     if word_counts.get("Hello") == 5 {
  |                                    ^ expected `Option<&u32>`, found integer
  |
  = note: expected enum `Option<&u32>`
             found type `{integer}`
```

**The mistake.** Both sides of `==` must be the same type, and the left side is an `Option<&u32>`. In Python `d.get(k) == 5` is quietly `False` for a missing key; here it does not compile.

**The fix.** Compare with an `Option`, or open the `Option` first.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    println!("{}", word_counts.get("Hello") == Some(&5));           // true
    println!("{}", word_counts.get("Hello").copied() == Some(5));   // true
}
```

**Read:** [A first `HashMap` — `get` asks](../a_first_hashmap/README.md#3-get-asks-the-answer-comes-in-an-option)

### 5. Adding to `get`

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    let more = word_counts.get("Hello") + 1;
    println!("{more}");
}
```

```text title="Abridged — real rustc output for add_to_get.rs"
error[E0369]: cannot add `{integer}` to `Option<&u32>`
   --> add_to_get.rs:6:41
    |
  6 |     let more = word_counts.get("Hello") + 1;
    |                ------------------------ ^ - {integer}
    |                |
    |                Option<&u32>
    |
note: `Option<&u32>` does not implement `Add<{integer}>`
```

**The mistake.** Arithmetic is defined on numbers, and an `Option<&u32>` is a question about a number. The dropped lines pointed at `Option`'s definition in the toolchain's source.

**The fix.** Add to the number inside.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    let more = word_counts.get("Hello").copied().unwrap_or(0) + 1;
    println!("{more}");   // 6
}
```

**Read:** [`unwrap_or`](../../17_Option_and_Result/unwrap_or/README.md)

### 6. A borrowed value compared to a number

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    for (word, count) in &word_counts {
        if count == 5 {
            println!("{word}");
        }
    }
}
```

```text title="rustc 1.98.0 on compare_in_loop.rs"
error[E0277]: can't compare `&u32` with `{integer}`
 --> compare_in_loop.rs:7:18
  |
7 |         if count == 5 {
  |                  ^^ no implementation for `&u32 == {integer}`
  |
  = help: the trait `PartialEq<{integer}>` is not implemented for `&u32`
help: consider dereferencing here
  |
7 |         if *count == 5 {
  |            +
```

**The mistake.** A loop over `&word_counts` borrows the map, so each `count` is a `&u32`, a reference to the number, and a reference is not compared to a literal. This is the loop-shaped version of entry 4.

**The fix.** Dereference, or destructure the reference in the pattern.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    for (word, count) in &word_counts {
        if *count == 5 {
            println!("{word}");   // Hello
        }
    }
    for (word, &count) in &word_counts {
        if count == 5 {
            println!("{word}");   // Hello
        }
    }
}
```

**Read:** [A first `HashMap` — iterating](../a_first_hashmap/README.md#8-iterating-and-why-the-order-is-not-the-insertion-order), [Borrowing](../../18_Ownership/borrowing/README.md)

## Indexing

### 7. Moving out of `map[key]`

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut names: HashMap<u32, String> = HashMap::new();
    names.insert(1, String::from("Hello"));
    let first: String = names[&1];
    println!("{first}");
}
```

```text title="rustc 1.98.0 on index_moves_out.rs"
error[E0507]: cannot move out of index of `HashMap<u32, String>`
 --> index_moves_out.rs:6:25
  |
6 |     let first: String = names[&1];
  |                         ^^^^^^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
  |
help: consider borrowing here
  |
6 |     let first: String = &names[&1];
  |                         +
help: consider cloning the value if the performance cost is acceptable
  |
6 |     let first: String = names[&1].clone();
  |                                  ++++++++
```

**The mistake.** `names[&1]` names the `String` where it sits, inside the map, and a `let` of type `String` would move it out and leave the map holding a hole. A `u32` value would be copied out instead; the message says why this one is not: `String` is not `Copy`. (The first help is off by a type: `&names[&1]` is a `&String`, so the annotation has to change with it.)

**The fix.** Borrow the value, or clone it.

```rust
use std::collections::HashMap;

fn main() {
    let mut names: HashMap<u32, String> = HashMap::new();
    names.insert(1, String::from("Hello"));
    let first: &String = &names[&1];
    let owned: String = names[&1].clone();
    println!("{first} {owned}");   // Hello Hello
}
```

**Read:** [`HashMap` — `get` asks, `[]` asserts](../the_hashmap/README.md), [Copy or move](../../18_Ownership/copy_or_move/README.md)

### 8. A missing key, indexed

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    println!("{}", word_counts["hello"]);
}
```

This compiles. Run, it panics:

```text title="rustc --edition 2024 index_missing_key.rs && ./index_missing_key; echo exit $? — rustc 1.98.0, macOS"

thread 'main' (41476231) panicked at index_missing_key.rs:6:31:
no entry found for key
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
exit 101
```

**The mistake.** `[]` is a claim that the key is there, and `"hello"` is not `"Hello"`. The number in brackets is the thread's id and differs on every run; exit status 101 is what a panic leaves behind.

**The fix.** Ask with `get` when the key might be missing.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    println!("{:?}", word_counts.get("hello"));   // None
}
```

**Read:** [A first `HashMap` — `map[key]` asserts](../a_first_hashmap/README.md#5-mapkey-asserts-the-value-or-a-panic), [A first `HashMap` — keys are case-sensitive](../a_first_hashmap/README.md#9-keys-are-case-sensitive)

## `String` keys and `&str` lookups

### 9. `&str` keys looked up with a `&String`

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    let wanted = String::from("Hello");
    println!("{:?}", word_counts.get(&wanted));
}
```

```text title="Abridged — real rustc output for lookup_with_string_ref.rs"
error[E0277]: the trait bound `&str: Borrow<String>` is not satisfied
    --> lookup_with_string_ref.rs:7:38
     |
   7 |     println!("{:?}", word_counts.get(&wanted));
     |                                  --- ^^^^^^^ the trait `Borrow<String>` is not implemented for `&str`
     |                                  |
     |                                  required by a bound introduced by this call
     |
help: the trait `Borrow<str>` is implemented for `String`
note: required by a bound in `HashMap::<K, V, S, A>::get`
1035 |     pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
     |            --- required by a bound in this associated function
1036 |     where
1037 |         K: Borrow<Q>,
     |            ^^^^^^^^^ required by this bound in `HashMap::<K, V, S, A>::get`
```

**The mistake.** `get` accepts any `&Q` that the key type can be *borrowed as*: a `String` key can be borrowed as a `str`, so `HashMap<String, _>` accepts `get("Hello")`. The other way round does not hold — a `&str` key cannot be borrowed as a `String` — and `&wanted` is a `&String`. The dropped lines gave the toolchain paths of the two `impl`s.

**The fix.** Hand `get` the `&str` inside the `String`.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    let wanted = String::from("Hello");
    println!("{:?}", word_counts.get(wanted.as_str()));   // Some(5)
}
```

**Read:** [`Borrow`: look up an owned key with a borrowed one](../../12_Traits/borrow_trait/README.md), [A first `HashMap` — `&str` keys borrow](../a_first_hashmap/README.md#10-str-keys-borrow-the-text-string-keys-own-a-copy)

### 10. A `String` passed by value to `get`

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    word_counts.insert(String::from("Hello"), 5);
    let wanted = String::from("Hello");
    println!("{:?}", word_counts.get(wanted));
}
```

```text title="Abridged — real rustc output for lookup_by_value.rs"
error[E0308]: mismatched types
    --> lookup_by_value.rs:7:38
     |
   7 |     println!("{:?}", word_counts.get(wanted));
     |                                  --- ^^^^^^ expected `&_`, found `String`
     |                                  |
     |                                  arguments to this method are incorrect
     |
     = note: expected reference `&_`
                   found struct `String`
help: consider borrowing here
     |
   7 |     println!("{:?}", word_counts.get(&wanted));
     |                                      +
```

**The mistake.** `get` only ever looks; it takes a reference, never ownership. `get(wanted)` would also move `wanted` into the call, which is rarely what a lookup means.

**The fix.** A reference, in either spelling.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    word_counts.insert(String::from("Hello"), 5);
    let wanted = String::from("Hello");
    println!("{:?} {:?}", word_counts.get(&wanted), word_counts.get("Hello"));   // Some(5) Some(5)
}
```

**Read:** [`String` vs `&str`](../../14_Strings/string_vs_str/README.md)

## Changing a value

### 11. Writing through `get`

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    if let Some(count) = word_counts.get("Hello") {
        *count += 1;
    }
    println!("{:?}", word_counts.get("Hello"));
}
```

```text title="rustc 1.98.0 on mutate_through_get.rs"
error[E0594]: cannot assign to `*count`, which is behind a `&` reference
 --> mutate_through_get.rs:7:9
  |
6 |     if let Some(count) = word_counts.get("Hello") {
  |                 ----- consider changing this binding's type to be: `&mut u32`
7 |         *count += 1;
  |         ^^^^^^^^^^^ `count` is a `&` reference, so it cannot be written to
  |
```

**The mistake.** `get` returns `Option<&V>`, a shared reference, which is read-only. Changing the value needs `get_mut`, which returns `Option<&mut V>` and needs the map to be `mut`.

**The fix.**

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    if let Some(count) = word_counts.get_mut("Hello") {
        *count += 1;
    }
    println!("{:?}", word_counts.get("Hello"));   // Some(6)
}
```

**Read:** [Borrowing](../../18_Ownership/borrowing/README.md), [`if let`](../../17_Option_and_Result/if_let/README.md)

### 12. `entry` on a shared reference

```rust,compile_fail
use std::collections::HashMap;

fn bump(word_counts: &HashMap<&str, u32>) {
    *word_counts.entry("Hello").or_insert(0) += 1;
}

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    bump(&word_counts);
    word_counts.insert("world", 2);
}
```

```text title="rustc 1.98.0 on entry_on_shared_ref.rs"
error[E0596]: cannot borrow `*word_counts` as mutable, as it is behind a `&` reference
 --> entry_on_shared_ref.rs:4:6
  |
4 |     *word_counts.entry("Hello").or_insert(0) += 1;
  |      ^^^^^^^^^^^ `word_counts` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
3 | fn bump(word_counts: &mut HashMap<&str, u32>) {
  |                       +++
```

**The mistake.** `entry` may insert, so it takes `&mut self`, and a `&HashMap` parameter cannot provide that. The function's signature has to say that it changes the map.

**The fix.**

```rust
use std::collections::HashMap;

fn bump(word_counts: &mut HashMap<&str, u32>) {
    *word_counts.entry("Hello").or_insert(0) += 1;
}

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    bump(&mut word_counts);
    println!("{:?}", word_counts.get("Hello"));   // Some(1)
}
```

**Read:** [Borrowing](../../18_Ownership/borrowing/README.md)

### 13. `insert` without `mut`

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
}
```

```text title="rustc 1.98.0 on insert_without_mut.rs"
error[E0596]: cannot borrow `word_counts` as mutable, as it is not declared as mutable
 --> insert_without_mut.rs:5:5
  |
5 |     word_counts.insert("Hello", 5);
  |     ^^^^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
4 |     let mut word_counts: HashMap<&str, u32> = HashMap::new();
  |         +++
```

**The mistake.** A binding is read-only unless it says `mut`, and `insert` changes the map. The same error, one word earlier than entry 12.

**The fix.**

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    println!("{}", word_counts.len());   // 1
}
```

**Read:** [A first `HashMap` — making one](../a_first_hashmap/README.md#1-making-one-use-new-and-the-type)

### 14. The missing star

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in ["the", "cat", "the"] {
        word_counts.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", word_counts.get("the"));
}
```

```text title="rustc 1.98.0 on missing_star.rs"
error[E0368]: binary assignment operation `+=` cannot be applied to type `&mut u32`
 --> missing_star.rs:6:9
  |
6 |         word_counts.entry(word).or_insert(0) += 1;
  |         ------------------------------------^^^^^
  |         |
  |         cannot use `+=` on type `&mut u32`
  |
help: `+=` can be used on `u32` if you dereference the left-hand side
  |
6 |         *word_counts.entry(word).or_insert(0) += 1;
  |         +
```

**The mistake.** `or_insert` returns a `&mut u32`, a reference to the count, and `+=` wants the count. The `*` reaches through the reference; it is the first character of the counting loop for a reason.

**The fix.**

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in ["the", "cat", "the"] {
        *word_counts.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", word_counts.get("the"));   // Some(2)
}
```

**Read:** [A first `HashMap` — `entry`](../a_first_hashmap/README.md#6-entry-the-slot-for-a-key-filled-or-empty)

## Loops and the borrow checker

### 15. The map is gone after the loop

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    let mut total = 0;
    for (_word, count) in word_counts {
        total += count;
    }
    println!("{total} over {} words", word_counts.len());
}
```

```text title="Abridged — real rustc output for use_after_for.rs"
error[E0382]: borrow of moved value: `word_counts`
   --> use_after_for.rs:10:39
    |
  4 |     let mut word_counts: HashMap<&str, u32> = HashMap::new();
    |         --------------- move occurs because `word_counts` has type `HashMap<&str, u32>`, which does not implement the `Copy` trait
...
  7 |     for (_word, count) in word_counts {
    |                           ----------- `word_counts` moved due to this implicit call to `.into_iter()`
...
 10 |     println!("{total} over {} words", word_counts.len());
    |                                       ^^^^^^^^^^^ value borrowed here after move
    |
note: `into_iter` takes ownership of the receiver `self`, which moves `word_counts`
help: consider iterating over a slice of the `HashMap<&str, u32>`'s content to avoid moving into the `for` loop
    |
  7 |     for (_word, count) in &word_counts {
    |                           +
```

**The mistake.** `for … in word_counts` hands the map to the loop, which takes it apart pair by pair; there is no map left to ask `len()` of. The dropped line pointed at `into_iter` in the toolchain's source.

**The fix.** Loop over a borrow.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    let mut total = 0;
    for (_word, count) in &word_counts {
        total += count;
    }
    println!("{total} over {} words", word_counts.len());   // 5 over 1 words
}
```

**Read:** [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md), [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md)

### 16. `insert` inside the loop

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    for (word, count) in &word_counts {
        if *count > 3 {
            word_counts.insert("frequent", *count);
        }
        println!("{word}");
    }
}
```

```text title="rustc 1.98.0 on insert_while_iterating.rs"
error[E0502]: cannot borrow `word_counts` as mutable because it is also borrowed as immutable
 --> insert_while_iterating.rs:8:13
  |
6 |     for (word, count) in &word_counts {
  |                          ------------
  |                          |
  |                          immutable borrow occurs here
  |                          immutable borrow later used here
7 |         if *count > 3 {
8 |             word_counts.insert("frequent", *count);
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

**The mistake.** The loop holds a shared borrow of the map for as long as it runs, and `insert` needs a mutable one. An insert can make the map reallocate its table, which would leave the loop reading freed memory — so the rule is not pedantry.

**The fix.** Collect what to insert during the loop, and insert after it.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    let mut frequent: Vec<u32> = Vec::new();
    for (_word, count) in &word_counts {
        if *count > 3 {
            frequent.push(*count);
        }
    }
    for count in frequent {
        word_counts.insert("frequent", count);
    }
    println!("{:?}", word_counts.get("frequent"));   // Some(5)
}
```

**Read:** [Borrowing](../../18_Ownership/borrowing/README.md)

## What a key has to be

### 17. A struct key without `Eq` and `Hash`

```rust,compile_fail
use std::collections::HashMap;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut labels: HashMap<Point, &str> = HashMap::new();
    labels.insert(Point { x: 0, y: 0 }, "origin");
    println!("{}", labels.len());
}
```

```text title="rustc 1.98.0 on struct_key.rs"
error[E0599]: the method `insert` exists for struct `HashMap<Point, &str>`, but its trait bounds were not satisfied
  --> struct_key.rs:10:12
   |
 3 | struct Point {
   | ------------ doesn't satisfy `Point: Eq` or `Point: Hash`
...
10 |     labels.insert(Point { x: 0, y: 0 }, "origin");
   |            ^^^^^^
   |
   = note: the following trait bounds were not satisfied:
           `Point: Eq`
           `Point: Hash`
help: consider annotating `Point` with `#[derive(Eq, Hash, PartialEq)]`
   |
 3 + #[derive(Eq, Hash, PartialEq)]
 4 | struct Point {
   |
```

**The mistake.** Declaring the map is allowed for any `K`; using it is not. `insert` has to hash the key and compare it with what is stored, so it needs `K: Eq + Hash`, and a bare struct has neither. rustc's help is the fix, and deriving both together keeps the two in step.

**The fix.**

```rust
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut labels: HashMap<Point, &str> = HashMap::new();
    labels.insert(Point { x: 0, y: 0 }, "origin");
    println!("{:?}", labels.get(&Point { x: 0, y: 0 }));   // Some("origin")
}
```

**Read:** [`HashMap` — what a key has to be](../the_hashmap/README.md), [Marker traits](../../12_Traits/marker_traits/README.md)

### 18. An `f64` key

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut prices: HashMap<f64, &str> = HashMap::new();
    prices.insert(9.99, "book");
    println!("{}", prices.len());
}
```

```text title="rustc 1.98.0 on f64_key.rs"
error[E0599]: the method `insert` exists for struct `HashMap<f64, &str>`, but its trait bounds were not satisfied
 --> f64_key.rs:5:12
  |
5 |     prices.insert(9.99, "book");
  |            ^^^^^^
  |
  = note: the following trait bounds were not satisfied:
          `f64: Eq`
          `f64: Hash`
```

**The mistake.** `f64` is neither `Eq` nor `Hash`, and there is nothing to derive: `NaN != NaN`, so equality is not an equivalence relation, and a key that is not equal to itself could never be found again.

**The fix.** Store the price as an integer number of cents.

```rust
use std::collections::HashMap;

fn main() {
    let mut prices: HashMap<u32, &str> = HashMap::new();
    prices.insert(999, "book");
    println!("{:?}", prices.get(&999));   // Some("book")
}
```

**Read:** [Comparison traits](../../12_Traits/comparison_traits/README.md), [`HashMap` — what a key has to be](../the_hashmap/README.md)

## Lifetimes

### 19. `&str` keys outliving the text

```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    {
        let text = String::from("the cat and the dog");
        for word in text.split_whitespace() {
            *word_counts.entry(word).or_insert(0) += 1;
        }
    }
    println!("{:?}", word_counts.get("the"));
}
```

```text title="rustc 1.98.0 on keys_outlive_text.rs"
error[E0597]: `text` does not live long enough
  --> keys_outlive_text.rs:7:21
   |
 6 |         let text = String::from("the cat and the dog");
   |             ---- binding `text` declared here
 7 |         for word in text.split_whitespace() {
   |                     ^^^^ borrowed value does not live long enough
...
10 |     }
   |     - `text` dropped here while still borrowed
11 |     println!("{:?}", word_counts.get("the"));
   |                      ----------- borrow later used here
```

**The mistake.** Each `&str` key points into `text`, and `text` is dropped at the closing brace while the map that still points into it is used on the next line. The map owns no text of its own.

**The fix.** Keys that must outlive the text are `String`s.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    {
        let text = String::from("the cat and the dog");
        for word in text.split_whitespace() {
            *word_counts.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    println!("{:?}", word_counts.get("the"));   // Some(2)
}
```

**Read:** [A first `HashMap` — `&str` keys borrow](../a_first_hashmap/README.md#10-str-keys-borrow-the-text-string-keys-own-a-copy), [`String` vs `&str`](../../14_Strings/string_vs_str/README.md)

## What is not an error

### 20. A discarded `insert`, and the lint that does not fire

```rust
#![deny(unused_must_use)]
use std::collections::{HashMap, HashSet};

fn main() {
    let mut words: HashSet<&str> = HashSet::new();
    words.insert("Hello");
    words.insert("Hello");
    words.remove("Hello");
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    word_counts.insert("Hello", 15);
    word_counts.remove("Hello");
    println!("{} {}", words.len(), word_counts.len());   // 0 0
}
```

```text title="rustc --edition 2024 set_insert_deny.rs && ./set_insert_deny; echo exit $? — rustc 1.98.0"
0 0
exit 0
```

**What happened.** Nothing. `HashSet::insert` returns a `bool` and `HashMap::insert` an `Option<V>`, and neither method, nor either type, is marked `#[must_use]` in the 1.98.0 standard library, so `deny(unused_must_use)` has nothing to deny. The second `insert("Hello", 15)` replaced a 5 that nobody looked at. The [lints page](../hashmap_lints/README.md#what-no-lint-catches) shows that clippy's default, pedantic and nursery groups say nothing either.

**Read:** [A first `HashSet` — `insert`'s answer](../a_first_hashset/README.md#1-making-one-and-inserts-answer), [`HashMap` — `insert` returns what was there before](../the_hashmap/README.md)

## The fixes, run

Every fix above is a numbered block of one program, so that none of them can quietly stop compiling:

<!-- output:hashmap_err_fixes -->
*Verified output of [`hashmap_err_fixes.rs`](examples/hashmap_err_fixes.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. use std::collections::HashMap; sits at the top of this file
2. The type comes from an annotation, or from the first insert
   annotated.len() = 0, inferred.len() = 1
3. get is an Option: keep it as one, or open it with a default
   found = Some(5), count = 5
4. Compare the Option with an Option, or open it first
   get("Hello") == Some(&5): true
   get("Hello").copied() == Some(5): true
5. Arithmetic on the number inside, not on the Option
   more = 6
6. In a loop over &map the value is a &u32: dereference it, or destructure the &
   keys holding 5, counted both ways: 2
7. Indexing names a place: borrow it or clone it, never move it
   first = Hello, owned = Hello
8. A key that might be missing is asked for with get, not []
   get("hello") = None
9. &str keys, a String in hand: look up with as_str()
   get(wanted.as_str()) = Some(5)
10. String keys, a String in hand: pass a reference
   get(&wanted) = Some(5), get("Hello") = Some(5)
11. Change a value through get_mut
   Hello -> Some(6)
12. entry through a &mut parameter
   Hello -> Some(7)
13. insert needs let mut
   fresh.len() = 1
14. The * in front: += works on the u32, not on the &mut u32
   the -> Some(2)
15. Loop over &map to keep the map
   9 over 2 words
16. Decide during the loop, insert after it
   frequent -> Some(7)
17. A struct key with the traits derived
   Point { x: 0, y: 0 } -> Some("origin")
18. No f64 keys: keep the price in cents, as an integer
   999 cents -> Some("book")
19. Keys that must outlive the text are Strings
   the -> Some(2)
20. HashSet::insert's bool is yours to use or to drop; nothing here is #[must_use]
   "Hello" was already there
```
<!-- /output -->

## If you are coming from another language

- **Python.** Most of these are run-time surprises there and compile-time refusals here. `d.get(k) == 5` (entry 4) is quietly `False` for a missing key in Python and does not compile in Rust; `d[k]` on a missing key raises `KeyError` where entry 8 panics; `for k in d: d[k2] = v` raises *dictionary changed size during iteration* where entry 16 is refused before the program runs; and a `float` key is fine in Python where entry 18 is not. Entries 3, 5, 6 and 11 have no Python counterpart at all, because Python has no `Option` and no references to write through.
- **ABAP.** Entry 17 is the `TYPE HASHED TABLE … WITH UNIQUE KEY` clause: ABAP names the key fields where Rust derives `Eq + Hash` for the type. Entry 16 is the rule that a `LOOP AT itab` must not `INSERT` into the same hashed table. Entry 8, `map[k]` panicking, is a table expression `itab[ key = v ]` raising `CX_SY_ITAB_LINE_NOT_FOUND`, and `READ TABLE` with `sy-subrc = 4` is the `get` that fixes it.
- **Java.** Entry 17 is `equals` and `hashCode` overridden together, which Java asks for by convention and Rust by the `insert` bound; entry 16 is `ConcurrentModificationException`, thrown at run time where rustc refuses at compile time; entry 8 is a `null` from `get` used as a value.

## See also

- [A first `HashMap`](../a_first_hashmap/README.md) — the lesson these errors interrupt, in order
- [A first `HashSet`](../a_first_hashset/README.md) — the set at the same pace
- [`HashMap`](../the_hashmap/README.md) — `get` asks and `[]` asserts, and what a key has to be
- [Lints around `HashMap` and `HashSet`](../hashmap_lints/README.md) — what the compiler only warns about, and what nothing flags
- [`HashMap` and `HashSet`: resources](../hashmap_resources/README.md) — where each error is explained at length
- [`Borrow`: look up an owned key with a borrowed one](../../12_Traits/borrow_trait/README.md) — entry 9's bound, in full
- [Every reference error, and its fix](../../18_Ownership/references/reference_errors/README.md) — the same kind of page for `&` and `&mut`
- [Every returned-by-value error, and its fix](../../18_Ownership/returned_by_value_errors/README.md) — the same kind of page for return values
- [rustc's error index ↗](https://doc.rust-lang.org/error_codes/error-index.html) — the long explanation behind each code, also printed by `rustc --explain E0599`
- [Sets of sets](../sets_of_sets/README.md) — the `Hash` bound again, when the element is itself a `HashSet`

## Po polsku

Osiemnaście odmów kompilatora, jedna panika w czasie wykonania i jedna rzecz, która błędem nie jest, spotykane przy pierwszej mapie albo pierwszym zbiorze: brak `use std::collections::HashMap;` (`E0425`, `E0433`), `HashMap::new()` bez typu (`E0282`), wynik `get` użyty jako liczba, porównany z liczbą albo dodany do liczby (`E0308`, `E0369`) — bo `get` zwraca `Option<&V>`, a nie wartość — `&u32` porównane z literałem w pętli (`E0277`), przeniesienie `String` spod `map[klucz]` (`E0507`), brakujący klucz pod `[]` (panika *no entry found for key*), klucz `String` szukany przez wartość albo klucz `&str` szukany przez `&String` (`E0308`, `E0277`), zapis przez `get` zamiast `get_mut` (`E0594`), `entry` albo `insert` bez `mut` (`E0596`), brak gwiazdki przed `entry(k).or_insert(0) += 1` (`E0368`), pętla, która zjada mapę (`E0382`), `insert` wewnątrz pętli po mapie (`E0502`), klucz bez `Eq + Hash` albo klucz `f64` (`E0599`) i klucze `&str` pożyczone z tekstu, który zniknął (`E0597`). Przy każdym: kod, dokładny komunikat rustc 1.98.0, pomyłka w jednym zdaniu i poprawka, która się kompiluje. Na końcu to, czego kompilator nie odrzuca: zignorowany wynik `insert`, bo ani `bool` z `HashSet::insert`, ani `Option` z `HashMap::insert` nie są oznaczone `#[must_use]`.

**Szukaj po polsku:** `rust HashMap E0308 expected u32 found Option` · `rust HashMap E0502 insert w pętli` · `rust E0599 insert exists but its trait bounds were not satisfied Hash` · `rust HashMap no entry found for key` · `rust E0597 does not live long enough HashMap &str`
