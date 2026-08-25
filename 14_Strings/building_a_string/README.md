# Building a `String`

**Level:** 101 → 201 · working knowledge

**One line:** `push_str` and `push` append in place; `+` **consumes** its left operand and reuses that buffer; `format!` borrows everything and allocates a fresh one — and every edit that takes an index takes a *byte* index, so `truncate` can panic exactly where a slice can.

| you write | left operand | allocates | reach for it when |
|---|---|---|---|
| `s.push_str("…")` | borrowed `&mut` | only if it must grow | accumulating in a loop |
| `s.push('c')` | borrowed `&mut` | only if it must grow | one character |
| `a + &b` | **moved** — `a` is gone | no — `a`'s buffer becomes the answer | you are done with `a` |
| `format!("{a}{b}")` | borrowed | yes, always | two or three known pieces |
| `write!(s, "{x}")` | borrowed `&mut` | only if it must grow | formatting into an existing buffer |

---

## `push_str` and `push`

```rust
let mut s = String::from("Hi");
s.push_str(" Adam");   // "Hi Adam"  — takes a &str
s.push('!');           // "Hi Adam!" — takes a char
```

The difference is the argument, not the effect: `push_str("!")` would do the same thing. `'!'` is a `char` — four bytes, one Unicode scalar — and `"!"` is a `&str`, a pointer and a length. Use `push` when you have a character, which mostly means when you are working out of a `chars()` iterator.

## `+` moves its left operand

```rust
let a = String::from("equal ");
let b = String::from("vote");
let joined = a + &b;     // "equal vote"
// a is gone; b is still usable
```

There is exactly one `Add` impl for `String`, and it is `impl Add<&str> for String`. So the left side must be an owned `String` and is consumed, while the right side is only borrowed. Start from two `&str`s instead — two literals, say — and neither side qualifies, which is a different error again: [Concatenating strings](../concatenating_strings/README.md) is that whole family, `E0369` first. Forget the `&` here and you get:

```text
error[E0308]: mismatched types
 --> e0308.rs:4:22
  |
4 |     let joined = a + b;
  |                      ^ expected `&str`, found `String`
  |
help: consider borrowing here
  |
4 |     let joined = a + &b;
  |                      +
```

Consuming the left operand is not a wart — it is what makes `+` cheap. The result *is* `a`'s buffer, grown. A chain like `a + ", " + &b + ", " + &c` allocates nothing new at all, which is why it beats `format!` when you are genuinely finished with `a`.

## `format!` borrows everything

```rust
let made = format!("{c}{d}");   // c and d both still alive afterwards
```

One fresh allocation, nothing consumed, and it can reshape as well as join — padding, precision, `{:?}`, named arguments. That flexibility is the reason to prefer it for two or three known pieces, and the reason **not** to put it inside a loop: each pass allocates a whole `String` you immediately append and throw away.

## `write!` when you are already holding the buffer

```rust
use std::fmt::Write;

let mut report = String::new();
for (name, score) in [("Ada", 5), ("Ben", 2)] {
    writeln!(report, "{name:<5} {score}").unwrap();
}
```

Same formatting syntax as `format!`, but it writes *into* `report` instead of building a new `String` per item. The `unwrap()` is noise you have to write and can safely ignore — the `Result` exists because `write!` also serves `io::Write`, where a write really can fail; writing into a `String` cannot. Note the import: it is `std::fmt::Write`, and forgetting it produces a confusing "no method named `write_fmt`".

## Editing in the middle

```rust
let mut e = String::from("hello world");
e.insert(5, ',');        // "hello, world"
e.insert_str(0, ">> ");  // ">> hello, world"
e.pop();                 // Some('d')  — from the end
e.remove(0);             // '>'        — by byte index
e.truncate(8);           // ">> hello,"
e.clear();               // ""  — length 0, capacity kept
```

`clear` is the one worth remembering: it drops the length to zero and **keeps the buffer**, which is what makes reusing one `String` across loop iterations cheap. `insert` and `remove` shift everything after them, so they are O(n) — fine occasionally, wrong in a loop over a large string.

## The edits are byte-indexed, so they panic too

```rust
let mut f = String::from("bête");   // 5 bytes, 4 chars
f.truncate(2);   // PANIC — byte 2 is inside 'ê'
f.truncate(3);   // "bê"  — 3 is a char boundary
```

Same rule, same failure, same fix as [String slices](../string_slices/README.md): `insert`, `remove`, `truncate` and `split_off` all take byte offsets, and `char_indices()` is where legal ones come from. An ASCII test suite never finds this.

## Pre-paying for the growth

```rust
String::new()              // 64 pushes -> 4 reallocations
String::with_capacity(64)  // 64 pushes -> 0
```

The buffer doubles as it fills, so appending is *amortised* cheap without any help. `with_capacity` matters when you know the final size and the string is large or the loop is hot — [The anatomy of a `String`](../anatomy_of_a_string/README.md) has the growth curve in full.

## If you are coming from another language

**Python.** `str` is immutable, so there is no `push_str` — every `+=` builds a whole new string, and the standard advice is to collect into a list and `join`. Rust's `String` is genuinely mutable, so the loop you were taught to avoid is the right one here.

| Python | | Rust |
|---|---|---|
| `s += "x"` in a loop | O(n²) — a new object each pass | `s.push_str("x")` — amortised O(1) |
| `"".join(parts)` | the idiom that avoids that | `parts.concat()` / `parts.join(", ")` |
| `io.StringIO()` | a growable buffer | `String` — already one |
| `f"{a}{b}"` | a new string | `format!("{a}{b}")` — also a new string |
| `s[0] = "c"` | `TypeError` — immutable | `E0277` — `str` cannot be mutably indexed |

The habit to *unlearn*: reaching for `join` because `+=` is quadratic. It is not quadratic here. The habit to keep: `format!` in a loop is the Python `+=` mistake wearing different clothes — it allocates per pass — so accumulate with `push_str` or `write!`.

**ABAP.** `CONCATENATE` and `&&` build a new string every time; string templates are the modern form and map straight onto `format!`.

| ABAP | | Rust |
|---|---|---|
| `CONCATENATE a b INTO c.` | builds a new string | `format!("{a}{b}")` |
| `c = a && b.` | same, operator form | `a + &b` — but `a` is consumed |
| `\|{ a }{ b }\|` | string template | `format!("{a}{b}")` |
| `c = c && x.` in a loop | a new string per pass | `c.push_str(&x)` — grows in place |
| `SHIFT` / `REPLACE` | edit by character offset | `insert` / `remove` — by **byte** offset |

What changes: ABAP offsets are character offsets on a UCS-2 string, so `lv+2(1)` always lands on a character. Rust's are byte offsets into UTF-8, so the same arithmetic can land mid-character and panic. And `&&` copies both sides where `+` consumes the left one — the Rust version is cheaper and costs you the variable.

---

## Practice

**Build one line four ways, and find out what each one costs.** Join `"Ada"`, `"Ben"` and `"Cara"` into `"Ada, Ben, Cara"` using a chain of `+`, using `format!`, using `push_str` in a loop, and using `write!` into a pre-sized buffer.

Start by writing `a + b` with two `String`s and read the `E0308` — say which operand it is complaining about and why only one of the two needs the `&`. Then, for each of the four, record which of the three inputs are still usable afterwards and how many buffers were allocated. One of the four is the wrong answer inside a loop; name it.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:building_a_string_kata -->
*[`building_a_string_kata.rs`](examples/building_a_string_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: build one line four ways, and see which inputs survive.
//!
//!   rustc --edition 2024 building_a_string_kata.rs -o /tmp/bsk && /tmp/bsk

use std::fmt::Write as _;

fn main() {
    // The one that does not compile:
    //
    //   let a = String::from("Ada");
    //   let b = String::from("Ben");
    //   let joined = a + b;
    //
    //   error[E0308]: mismatched types
    //     |
    //   4 |     let joined = a + b;
    //     |                      ^ expected `&str`, found `String`
    //     |
    //   help: consider borrowing here
    //     |
    //   4 |     let joined = a + &b;
    //     |                      +
    //
    // `impl Add<&str> for String` is the only one there is: the left side is
    // consumed and reused, the right side is borrowed.

    println!("A — chained +");
    let a1 = String::from("Ada");
    let b1 = String::from("Ben");
    let c1 = String::from("Cara");
    let joined = a1 + ", " + &b1 + ", " + &c1;
    println!("   {joined:?}");
    println!("   a1 is gone (moved into the result). b1 = {b1:?}, c1 = {c1:?} still alive.");
    println!("   Allocations: 0 new buffers — a1's buffer grew and became the answer.");

    println!("\nB — format!");
    let a2 = String::from("Ada");
    let b2 = String::from("Ben");
    let c2 = String::from("Cara");
    let made = format!("{a2}, {b2}, {c2}");
    println!("   {made:?}");
    println!("   all three still alive: {a2:?} {b2:?} {c2:?}");
    println!("   Allocations: 1 new buffer. Nothing was consumed.");

    println!("\nC — push_str in a loop");
    let names = [String::from("Ada"), String::from("Ben"), String::from("Cara")];
    let mut built = String::new();
    for (i, n) in names.iter().enumerate() {
        if i > 0 {
            built.push_str(", ");
        }
        built.push_str(n);
    }
    println!("   {built:?}");
    println!("   all three still alive; `built` grew from empty, capacity {}", built.capacity());
    println!("   Allocations: however many times the buffer doubled — pre-pay with");
    println!("   String::with_capacity if you know the size.");

    println!("\nD — write!");
    let mut out = String::with_capacity(32);
    for (i, n) in names.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write!(out, "{n}").unwrap();
    }
    println!("   {out:?}");
    println!("   Same shape as C, but the formatter writes straight into `out` — no");
    println!("   intermediate String per item, which is what format!-inside-a-loop costs.");

    println!("\nAll four agree: {}", joined == made && made == built && built == out);
    println!("\nWhich to reach for:");
    println!("   two or three known pieces      -> format!, and read it out loud");
    println!("   accumulating in a loop         -> push_str / write! into one buffer");
    println!("   a left value you are done with -> + reuses its buffer");
    println!("   format! inside a loop          -> the one to avoid: an allocation per pass");
}
```
<!-- /source -->

<!-- output:building_a_string_kata -->
*Verified output of [`building_a_string_kata.rs`](examples/building_a_string_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
A — chained +
   "Ada, Ben, Cara"
   a1 is gone (moved into the result). b1 = "Ben", c1 = "Cara" still alive.
   Allocations: 0 new buffers — a1's buffer grew and became the answer.

B — format!
   "Ada, Ben, Cara"
   all three still alive: "Ada" "Ben" "Cara"
   Allocations: 1 new buffer. Nothing was consumed.

C — push_str in a loop
   "Ada, Ben, Cara"
   all three still alive; `built` grew from empty, capacity 16
   Allocations: however many times the buffer doubled — pre-pay with
   String::with_capacity if you know the size.

D — write!
   "Ada, Ben, Cara"
   Same shape as C, but the formatter writes straight into `out` — no
   intermediate String per item, which is what format!-inside-a-loop costs.

All four agree: true

Which to reach for:
   two or three known pieces      -> format!, and read it out loud
   accumulating in a loop         -> push_str / write! into one buffer
   a left value you are done with -> + reuses its buffer
   format! inside a loop          -> the one to avoid: an allocation per pass
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:building_a_string -->
*Verified output of [`building_a_string.rs`](examples/building_a_string.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. push_str takes a slice; push takes one char
   after push_str(" Adam")   "Hi Adam"
   after push('!')            "Hi Adam!"
   push_str("!") would work too — the difference is the argument type,
   not the effect: 'a' is a char, "a" is a &str. One is 4 bytes, the
   other is a pointer and a length.

2. + consumes its left operand
   let joined = a + &b;   "equal vote"
   `a` is MOVED into the result — the buffer is reused, not copied.
   `b` is only borrowed, and is still usable: "vote"
   a + b would be E0308: expected `&str`, found `String`

3. format! borrows everything
   format!("{c}{d}")   "equal vote"
   both still alive: "equal " "vote"
   Cost: a fresh allocation. `+` reuses the left buffer, so a long chain
   of `+` beats format! — and format! beats a chain you cannot read.

4. write! appends without allocating a second buffer
Ada   5
Ben   2
Cara  0
   (needs `use std::fmt::Write`; the Result is always Ok for a String)

5. Editing in the middle
   insert(5, ',')       "hello, world"
   insert_str(0, ">> ") ">> hello, world"
   pop()                ">> hello, worl"   returned Some('d')
   remove(0)            "> hello, worl"   returned '>'
   truncate(8)          "> hello,"
   clear()              ""   len 0 capacity 22
   clear() keeps the buffer — that is why it is the cheap way to reuse one.

6. The edits are byte-indexed too, so they can panic
   "bête" is 5 bytes
   truncate(2) PANICKED — byte 2 is inside 'ê'
   truncate(3) -> "bê"   <- 3 is a char boundary

7. Pre-paying for the growth
   String::new()             64 pushes -> 4 reallocation(s), capacity 64
   String::with_capacity(64) 64 pushes -> 0 reallocation(s), capacity 64
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/building_a_string/examples/building_a_string.rs -o /tmp/bs && /tmp/bs
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [Concatenating strings](../concatenating_strings/README.md) — the other half of this: joining two pieces you already have, and the `E0369` two literals produce
- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — capacity, growth, and why appending is cheap
- [Making a `String`](../making_a_string/README.md) — getting one in the first place
- [String slices](../string_slices/README.md) — the same byte-index panic, on the reading side
- [Borrowing](../../18_Ownership/borrowing/README.md) — why a `&str` into a string you are about to `push_str` is refused
- [`String::push_str` ↗](https://doc.rust-lang.org/std/string/struct.String.html#method.push_str) · [`std::fmt::Write` ↗](https://doc.rust-lang.org/std/fmt/trait.Write.html) · [The Rust Book, ch. 8.2 ↗](https://doc.rust-lang.org/book/ch08-02-strings.html#updating-a-string)
