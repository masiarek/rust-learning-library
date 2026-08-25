# String slices

**Level:** 101 → 201 · working knowledge

**One line:** A slice is a *view* — a pointer and a length into text somebody else owns. `&s[0..5]` copies nothing, the borrow checker keeps it from outliving what it looks at, and its numbers are **byte** offsets, which is the one way it can panic.

| | a byte index (`usize`) | a slice (`&str`) |
|---|---|---|
| what it knows | a number | where the text starts, and how much |
| tied to the string | not at all | by the borrow checker |
| survives `s.clear()` | yes — and now means nothing | no: the code does not compile |
| what it costs | 8 bytes | 16 bytes, no allocation |

---

## The bug a slice removes

Write "return the first word" without slices and the only thing you can return is an index:

```rust
fn first_word_index(s: &str) -> usize {
    match s.find(' ') {
        Some(i) => i,
        None => s.len(),
    }
}

let mut s = String::from("hello world");
let end = first_word_index(&s);   // 5
s.clear();                        // s is now ""
// `end` is still 5, and indexes nothing at all
```

That compiles, runs, and is wrong. `end` was computed from a state `s` no longer has, and nothing connects the two — a second `second_word` returning `(usize, usize)` gives you three loose numbers to keep in sync by hand.

Return the text itself instead:

```rust
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}
```

Now the same mistake is a compile error, because the returned `&str` still borrows `s`:

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> e0502.rs:7:5
  |
6 |     let word = first_word(&s);
  |                           -- immutable borrow occurs here
7 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
8 |     println!("the first word is: {word}");
  |                                   ---- immutable borrow later used here
```

`clear` needs `&mut s`; `word` is still holding a `&s`. [Borrowing](../../18_Ownership/borrowing/README.md) is the rule being applied — the string-specific part is that a slice is what *puts you under* that rule. An index escapes it, which is exactly the problem.

## What a slice is made of

Two words: where to start, and how far to go. No capacity, because a view owns no buffer to grow.

```rust
let s = String::from("hello world");   // ptr | len 11 | capacity 11
let world = &s[6..11];                 // ptr → byte 6 | len 5
size_of::<&str>()     // 16 — pointer + length
size_of::<String>()   // 24 — pointer + length + capacity
```

Nothing is copied and nothing is allocated: `world` points *into* the buffer `s` owns. That is the whole trick, and the reason the borrow checker has to care.

## The range shorthands

Drop either end and Rust fills it in — all three pairs below name the same slice:

```rust
&s[0..5]    &s[..5]      // from the start
&s[6..len]  &s[6..]      // to the end
&s[0..len]  &s[..]       // the whole thing
```

## The indices are bytes

This is the trap, and it hides in every ASCII test suite. Slice indices count **bytes**, not characters, and cutting a multi-byte character in half is a runtime panic:

```rust
let word = "bête";
word.len()            // 5 — not 4
&word[0..2]           // PANIC: byte index 2 is not a char boundary
word.get(0..2)        // None      — the total version, no panic
word.get(0..3)        // Some("bê") — 3 is a boundary
word.is_char_boundary(2)   // false
```

Three tools, in order of how much you should reach for them:

| you want | use | on a bad index |
|---|---|---|
| the slice, and a bug is a bug | `&s[a..b]` | panics |
| the slice, or nothing | `s.get(a..b)` | `None` |
| the legal cut points | `s.char_indices()` | yields only boundaries |

`char_indices()` is the one to know: it yields `(byte_offset, char)` pairs, so every index it hands you is a boundary by construction. It is **not** `chars().enumerate()` — that numbers the characters 0, 1, 2 and those numbers are not offsets you can slice with. [Meet the `char`](../meet_the_char/README.md) is the full story of why `.len()` counts bytes.

## A literal is already a slice

```rust
let literal = "hello world";   // &'static str — a view into your executable
&literal[..5]                  // "hello" — slicing needs no String
```

Which is why one parameter type serves everything:

```rust
fn first_word(s: &str) -> &str { /* … */ }

first_word("hello world");   // a literal
first_word(&s);              // a &String, coerced for free
first_word(&s[6..]);         // a slice of a slice
```

Take `&str`, never `&String` — the caller with a literal cannot produce a `&String` without allocating one. [`String` vs `&str`](../string_vs_str/README.md) works through what each caller pays.

## Slices are not a string feature

`&str` is one instance of a general shape:

```rust
let a = [1, 2, 3, 4, 5];
let part = &a[1..3];         // &[i32] — [2, 3]
size_of::<&[i32]>()          // 16 — the same two words
```

`&str` is to `String` what `&[T]` is to `Vec<T>`: the borrowed view of an owned buffer. The only thing `&str` adds is a promise that the bytes are valid UTF-8 — which is precisely why its indices can be rejected and `&[u8]`'s cannot.

## If you are coming from another language

**Python.** `s[4:7]` builds a **new string** — it copies. That cost is why you learned to pass indices around instead of slices, and passing indices around is exactly the bug above. Rust inverts both: the slice is free, and the index is the dangerous one.

| Python | | Rust |
|---|---|---|
| `s[6:11]` | copies out a new `str` | `&s[6..11]` — a view, nothing copied |
| `memoryview(b)[6:11]` | a view, no copy | `&s[6..11]` — but for text, and checked |
| `s[6:]`, `s[:5]`, `s[:]` | open-ended slices | `&s[6..]`, `&s[..5]`, `&s[..]` — identical spelling |
| `s[2]` on `"bête"` | `'ê'` — indexes *characters* | `&s[2..3]` panics — indexes *bytes* |
| a stale index after `s = ""` | silently wrong later | `E0502` at compile time |

The last two rows are the ones to internalise. Python 3 strings index by character, so `s[2]` is always a character and never a half of one; Rust stores UTF-8 and indexes bytes, so the same instinct panics. And Python's slice is a snapshot — it keeps working after the original changes, because it is a copy. Rust's is a live window, so the compiler forbids changing the thing you are looking through.

**ABAP.** `lv+6(5)` is offset/length access, and the offsets are in *characters* because ABAP strings are UCS-2 internally — one unit per character, always. Rust's are bytes, and a character is 1–4 of them.

| ABAP | | Rust |
|---|---|---|
| `lv+6(5)` | offset/length substring | `&s[6..11]` |
| — produces a **copy** | | — produces a **view** |
| `FIELD-SYMBOLS <fs>` `ASSIGNING` | a view into data you did not copy | `&str` / `&[T]` |
| `sy-tabix` kept after `DELETE` | stale index, dumps later | the same code will not compile |

The transfer is the field-symbol instinct: you already know a view is cheaper than a copy, and that a view into something you then modify is how you get a short dump. Rust makes the second half unwriteable rather than untested — `GETWA_NOT_ASSIGNED` at 3am becomes `E0502` at your desk.

---

## Practice

**Cut a name in half without panicking.** Write `halves(s: &str) -> (&str, &str)` that splits a string at its midpoint. Start with the obvious `let mid = s.len() / 2;` and run it on `"bete noir"`, `"bête noir"`, `"naïve café"` and `"日本語"` — note which ones survive and which one panics, and why the accented pair is not the deciding case you would expect.

Then fix it two ways: by walking `mid` down until `is_char_boundary` agrees, and by picking the last offset `char_indices()` yields at or below the midpoint. Confirm both give the same answers.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:string_slices_kata -->
*[`string_slices_kata.rs`](examples/string_slices_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: cut a string in half without panicking.
//!
//!   rustc --edition 2024 string_slices_kata.rs -o /tmp/ssk && /tmp/ssk

use std::panic;

/// The obvious version. Correct for ASCII, a panic for everything else.
fn halves_naive(s: &str) -> (&str, &str) {
    let mid = s.len() / 2;
    (&s[..mid], &s[mid..])
}

/// Walk back from the midpoint until the index is a real char boundary.
fn halves(s: &str) -> (&str, &str) {
    let mut mid = s.len() / 2;
    while mid > 0 && !s.is_char_boundary(mid) {
        mid -= 1;
    }
    (&s[..mid], &s[mid..])
}

/// Same idea, expressed as a search through the boundaries the string reports.
fn halves_via_indices(s: &str) -> (&str, &str) {
    let target = s.len() / 2;
    let mid = s
        .char_indices()
        .map(|(i, _)| i)
        .take_while(|&i| i <= target)
        .last()
        .unwrap_or(0);
    (&s[..mid], &s[mid..])
}

fn try_naive(s: &str) -> String {
    panic::set_hook(Box::new(|_| {}));
    let r = panic::catch_unwind(|| {
        let (a, b) = halves_naive(s);
        format!("{a:?} + {b:?}")
    });
    let _ = panic::take_hook();
    r.unwrap_or_else(|_| "PANIC — byte index is not a char boundary".to_string())
}

fn main() {
    let cases = ["bete noir", "bête noir", "naïve café", "日本語"];

    println!("naive: &s[..len/2]");
    for s in cases {
        println!("   {:?}  ({} bytes)  ->  {}", s, s.len(), try_naive(s));
    }

    println!("\nis_char_boundary, walking back:");
    for s in cases {
        let (a, b) = halves(s);
        println!("   {:?}  ->  {:?} + {:?}", s, a, b);
    }

    println!("\nchar_indices, same answers:");
    for s in cases {
        let (a, b) = halves_via_indices(s);
        println!("   {:?}  ->  {:?} + {:?}", s, a, b);
    }

    println!("\nWhy 'naive' passes and 'naïve' does not:");
    for s in ["naive café", "naïve café"] {
        let mid = s.len() / 2;
        println!("   {:?}  len {}  mid {}  boundary? {}", s, s.len(), mid, s.is_char_boundary(mid));
    }
    println!("   A test suite of ASCII names never finds this. One accent does.");
}
```
<!-- /source -->

<!-- output:string_slices_kata -->
*Verified output of [`string_slices_kata.rs`](examples/string_slices_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
naive: &s[..len/2]
   "bete noir"  (9 bytes)  ->  "bete" + " noir"
   "bête noir"  (10 bytes)  ->  "bête" + " noir"
   "naïve café"  (12 bytes)  ->  "naïve" + " café"
   "日本語"  (9 bytes)  ->  PANIC — byte index is not a char boundary

is_char_boundary, walking back:
   "bete noir"  ->  "bete" + " noir"
   "bête noir"  ->  "bête" + " noir"
   "naïve café"  ->  "naïve" + " café"
   "日本語"  ->  "日" + "本語"

char_indices, same answers:
   "bete noir"  ->  "bete" + " noir"
   "bête noir"  ->  "bête" + " noir"
   "naïve café"  ->  "naïve" + " café"
   "日本語"  ->  "日" + "本語"

Why 'naive' passes and 'naïve' does not:
   "naive café"  len 11  mid 5  boundary? true
   "naïve café"  len 12  mid 6  boundary? true
   A test suite of ASCII names never finds this. One accent does.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:string_slices -->
*Verified output of [`string_slices.rs`](examples/string_slices.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The bug a slice removes
   first_word_index(&s) = 5       <- a bare usize
   s.clear()   ->   s = "", 0 bytes
   `end` is still 5, indexing text that is gone. Nothing warned.
   first_word() cannot reach here: holding its &str freezes `s` (E0502).

2. What a slice is made of
   s      String   "hello world"   len 11   capacity 11
   hello  &str     "hello"         len 5
   world  &str     "world"         len 5   <- points at byte 6
   size_of::<&str>()    = 16   (pointer + length, no capacity)
   size_of::<String>()  = 24   (pointer + length + capacity)
   first_word(&s) = "hello"   <- borrowed from `s`, nothing copied

3. The range shorthands name the same slice
   &s[0..5]  "hello"        &s[..5]  "hello"
   &s[6..11] "world"        &s[6..]  "world"
   &s[0..11] "hello world"  &s[..]   "hello world"

4. The indices are BYTES — the one way a slice panics
   "bete noir"   9 bytes, 9 chars   <- ASCII: they agree
   "bête noir"   10 bytes, 9 chars   <- they do not
      byte 0 -> 'b' (1 byte(s))
      byte 1 -> 'ê' (2 byte(s))
      byte 3 -> 't' (1 byte(s))
      byte 4 -> 'e' (1 byte(s))
   &accented[0..2] PANICKED — byte 2 is inside 'ê', not a boundary
   accented.get(0..2) = None      <- the total version: no panic
   accented.get(0..3) = Some("bê")   <- 3 is a boundary
   is_char_boundary: 2 -> false, 3 -> true

5. A literal is already a slice
   literal        &'static str   "hello world"
   &literal[..5]  &str           "hello"   <- slicing a literal needs no String
   first_word(literal)      = "hello"
   first_word(&s)           = "hello"   <- &String coerced to &str
   first_word(&s[6..])      = "world"   <- a slice of a slice

6. Slices are not a string feature
   let a = [1, 2, 3, 4, 5];
   &a[1..3] = [2, 3]   len 2
   size_of::<&[i32]>() = 16   <- same two words as &str
   &str is to String what &[T] is to Vec<T>: the borrowed view of the owned buffer.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/string_slices/examples/string_slices.rs -o /tmp/ss && /tmp/ss
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [`String` vs `&str`](../string_vs_str/README.md) — the owner and the view, and why parameters take `&str`
- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — the buffer a slice points into
- [Meet the `char`](../meet_the_char/README.md) — why the indices are bytes in the first place
- [Borrowing](../../18_Ownership/borrowing/README.md) — the `E0502` above, as a rule rather than a case
- [The Rust Book, ch. 4.3 — The Slice Type ↗](https://doc.rust-lang.org/book/ch04-03-slices.html) · [`str::char_indices` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.char_indices) · [`str::get` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.get)
