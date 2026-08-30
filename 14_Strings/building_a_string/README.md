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

**Five edits, one buffer.** Strip the vowels out of a `String` in place with `retain`, printing `len()` and `capacity()` on both sides of the call. Insert a `|` at the middle *character* of `"vote🦀here"` — and work out what `insert(len()/2, '|')` would have done instead. Then drain a range out of the middle of a row and keep what came out.

Then concatenate `a: String`, `b: &str` and `c: String` with `+`, and say which of the three you can still use afterwards. Finish by pushing `'A'` through `'Z'` onto an empty `String` and popping five characters back off — and say what `pop` returns on an empty string, and what it returns on `"go🦀"`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:editing_in_place_kata -->
*[`editing_in_place_kata.rs`](examples/editing_in_place_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: five edits that reuse the buffer — retain, insert, drain,
//! the `+` that eats its left operand, and push/pop.
//!
//!   rustc --edition 2024 editing_in_place_kata.rs -o /tmp/eipk && /tmp/eipk

/// Drop every vowel, in place. `retain` keeps what the closure says `true` to
/// and shifts the rest down — one pass, no second allocation.
fn strip_vowels(s: &mut String) {
    s.retain(|c| !"aeiouAEIOU".contains(c));
}

/// The byte offset of character `n` — what every String edit actually wants.
/// `insert(4, …)` means byte 4, and byte 4 of "café" is inside the é.
fn byte_of_char(s: &str, n: usize) -> usize {
    s.char_indices().nth(n).map(|(b, _)| b).unwrap_or(s.len())
}

fn main() {
    println!("1. retain — keep the consonants");
    let mut motto = String::from("Score Then Automatic Runoff");
    let before = (motto.len(), motto.capacity());
    strip_vowels(&mut motto);
    println!("   before  {:?}", "Score Then Automatic Runoff");
    println!("   after   {motto:?}");
    println!("   len {} -> {}, capacity {} -> {}  <- same buffer, nothing allocated",
        before.0, motto.len(), before.1, motto.capacity());

    println!("\n2. insert — at the middle character, not the middle byte");
    let mut name = String::from("vote🦀here");
    let half_byte = name.len() / 2;
    println!("   {name:?}: {} chars, {} bytes", name.chars().count(), name.len());
    println!("   the middle BYTE is {half_byte}, and is_char_boundary({half_byte}) = {}",
        name.is_char_boundary(half_byte));
    println!("   so insert({half_byte}, '|') would panic — it is inside the crab.");
    let middle_char = name.chars().count() / 2;
    let at = byte_of_char(&name, middle_char);
    println!("   the middle CHARACTER is #{middle_char}, which starts at byte {at}");
    name.insert(at, '|');
    println!("   after insert({at}, '|')  {name:?}");
    println!("   Every String edit is byte-indexed: insert, remove, replace_range,");
    println!("   truncate, split_off. len()/2 is a byte, and text is not bytes.");

    println!("\n3. drain — remove a range and keep what came out");
    let mut ballot = String::from("Ada,Ben,Cara,Dev");
    let removed: String = ballot.drain(4..8).collect();
    println!("   removed {removed:?}");
    println!("   left    {ballot:?}");
    println!("   drain returns an iterator over the removed chars; the String is");
    println!("   edited whether you collect them or not.");

    println!("\n4. `+` moves its left operand");
    let a = String::from("Score");
    let b = " then ";
    let c = String::from("Runoff");
    let joined = a + b + &c;
    // println!("{a}");   // error[E0382]: borrow of moved value: `a`
    println!("   let joined = a + b + &c;   -> {joined:?}");
    println!("   a: String  MOVED   — `+` takes it by value and reuses its buffer");
    println!("   b: &str    borrowed — the right side is always a &str");
    println!("   c: String  borrowed — because it was passed as &c");
    println!("   `c` is still here: {c:?}, `a` is gone. One allocation total,");
    println!("   which is why `+` exists at all.");

    println!("\n5. push and pop");
    let mut alphabet = String::new();
    for c in 'A'..='Z' {
        alphabet.push(c);
    }
    println!("   after 26 pushes  {alphabet:?}  (len {}, capacity {})",
        alphabet.len(), alphabet.capacity());
    let mut popped = String::new();
    for _ in 0..5 {
        if let Some(c) = alphabet.pop() {
            popped.push(c);
        }
    }
    println!("   popped 5         {popped:?}   <- reversed: pop takes from the end");
    println!("   left             {alphabet:?}  (len {}, capacity {})",
        alphabet.len(), alphabet.capacity());
    println!("   pop returns Option<char> — None on an empty String, never a panic —");
    println!("   and it pops a whole character, however many bytes that is.");
    let mut crab = String::from("go🦀");
    println!("   {:?}.pop() = {:?}, leaving {:?}", "go🦀", crab.pop(), crab);
}
```
<!-- /source -->

<!-- output:editing_in_place_kata -->
*Verified output of [`editing_in_place_kata.rs`](examples/editing_in_place_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. retain — keep the consonants
   before  "Score Then Automatic Runoff"
   after   "Scr Thn tmtc Rnff"
   len 27 -> 17, capacity 27 -> 27  <- same buffer, nothing allocated

2. insert — at the middle character, not the middle byte
   "vote🦀here": 9 chars, 12 bytes
   the middle BYTE is 6, and is_char_boundary(6) = false
   so insert(6, '|') would panic — it is inside the crab.
   the middle CHARACTER is #4, which starts at byte 4
   after insert(4, '|')  "vote|🦀here"
   Every String edit is byte-indexed: insert, remove, replace_range,
   truncate, split_off. len()/2 is a byte, and text is not bytes.

3. drain — remove a range and keep what came out
   removed "Ben,"
   left    "Ada,Cara,Dev"
   drain returns an iterator over the removed chars; the String is
   edited whether you collect them or not.

4. `+` moves its left operand
   let joined = a + b + &c;   -> "Score then Runoff"
   a: String  MOVED   — `+` takes it by value and reuses its buffer
   b: &str    borrowed — the right side is always a &str
   c: String  borrowed — because it was passed as &c
   `c` is still here: "Runoff", `a` is gone. One allocation total,
   which is why `+` exists at all.

5. push and pop
   after 26 pushes  "ABCDEFGHIJKLMNOPQRSTUVWXYZ"  (len 26, capacity 32)
   popped 5         "ZYXWV"   <- reversed: pop takes from the end
   left             "ABCDEFGHIJKLMNOPQRSTU"  (len 21, capacity 32)
   pop returns Option<char> — None on an empty String, never a panic —
   and it pops a whole character, however many bytes that is.
   "go🦀".pop() = Some('🦀'), leaving "go"
```
<!-- /output -->

</details>

---

**Run-length encoding, and the input that breaks it.** Turn `"AAABBBCCDAA"` into `"3A3B2C1D2A"`, then write the inverse and check the round trip. Make the decoder return a `Result` rather than panicking: it is the half of the pair that meets input it did not produce.

Then break your own pair with three inputs — a run longer than nine, a string that contains digits, and a string with no runs at all. Two of those change the answer and one only changes the size. Write down the precondition your encoding actually has, in the place a caller would look.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:run_length_encoding_kata -->
*[`run_length_encoding_kata.rs`](examples/run_length_encoding_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: run-length encoding both ways — and the three inputs that
//! turn the round trip into a lie.
//!
//!   rustc --edition 2024 run_length_encoding_kata.rs -o /tmp/rlek && /tmp/rlek

/// "AAABBBCCDAA" -> "3A3B2C1D2A". Counts characters, not bytes, so a multibyte
/// run survives; `push_str` and `push` grow one buffer instead of allocating
/// a String per run.
fn encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        let mut run = 1usize;
        while chars.peek() == Some(&c) {
            chars.next();
            run += 1;
        }
        out.push_str(&run.to_string());
        out.push(c);
    }
    out
}

/// "3A3B2C1D2A" -> "AAABBBCCDAA". The count may be several digits, so the
/// digits are accumulated until a non-digit arrives — that character is the
/// one being repeated.
fn decode(s: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut count = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            count.push(c);
        } else if count.is_empty() {
            return Err(format!("character {c:?} has no count in front of it"));
        } else {
            let n: usize = count.parse().map_err(|e| format!("bad count {count:?}: {e}"))?;
            for _ in 0..n {
                out.push(c);
            }
            count.clear();
        }
    }
    if count.is_empty() {
        Ok(out)
    } else {
        Err(format!("input ended with the count {count:?} and no character"))
    }
}

fn round_trip(s: &str) {
    let encoded = encode(s);
    let decoded = decode(&encoded);
    let ok = decoded.as_deref() == Ok(s);
    println!("   {:<24} -> {:<24} -> {:<24} {}",
        format!("{s:?}"),
        format!("{encoded:?}"),
        match &decoded {
            Ok(d) => format!("{d:?}"),
            Err(e) => format!("Err({e})"),
        },
        if ok { "round trip ok" } else { "MISMATCH" });
}

fn main() {
    println!("1. Encoding");
    for s in ["AAABBBCCDAA", "AAAAAAAAAAAA", "ABCDEF", "", "🦀🦀🦀ss"] {
        println!("   {:<16} -> {:?}", format!("{s:?}"), encode(s));
    }
    println!("   Twelve As encode as \"12A\", not \"9A3A\" — which is the whole reason");
    println!("   the decoder cannot just read one digit.");

    println!("\n2. Decoding");
    for s in ["3A3B2C1D2A", "12A", "1A1B1C", "A3B", "3A2"] {
        match decode(s) {
            Ok(d) => println!("   {:<14} -> {d:?}", format!("{s:?}")),
            Err(e) => println!("   {:<14} -> Err: {e}", format!("{s:?}")),
        }
    }
    println!("   Malformed input is a Result, not a panic: a decoder is the half of");
    println!("   this pair that meets data it did not produce.");

    println!("\n3. The round trip");
    for s in ["AAABBBCCDAA", "ABCDEF", "🦀🦀🦀ss", "Mississippi"] {
        round_trip(s);
    }

    println!("\n4. Where the round trip breaks");
    let digits = "AA3BB";
    println!("   Input containing digits: {digits:?}");
    let encoded = encode(digits);
    let back = decode(&encoded).unwrap_or_default();
    println!("     encode -> {encoded:?}");
    println!("     decode -> {} chars, starting {:?}", back.chars().count(), &back[..4]);
    println!("     round trip holds: {}", back == digits);
    println!("   The lone '3' encoded as \"13\" — one 3 — and its digit then ran into");
    println!("   the count in front of the Bs, so the decoder read \"132B\" as 132 Bs.");
    println!("   This encoding has no escape, so its real precondition is \"the alphabet");
    println!("   contains no digits\" — say that in a comment or in the signature,");
    println!("   rather than discovering it in production.");

    println!("\n5. And it is not always compression");
    for s in ["ABCDEF", "AAAAAA"] {
        let e = encode(s);
        println!("   {:<10} {} chars -> {} chars   {}",
            format!("{s:?}"), s.chars().count(), e.chars().count(),
            if e.chars().count() > s.chars().count() { "BIGGER" } else { "smaller" });
    }
    println!("   Run-length encoding pays only when runs are long. On text with no");
    println!("   repeats it doubles the size, which is why real formats keep a literal");
    println!("   mode and switch between the two.");
}
```
<!-- /source -->

<!-- output:run_length_encoding_kata -->
*Verified output of [`run_length_encoding_kata.rs`](examples/run_length_encoding_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Encoding
   "AAABBBCCDAA"    -> "3A3B2C1D2A"
   "AAAAAAAAAAAA"   -> "12A"
   "ABCDEF"         -> "1A1B1C1D1E1F"
   ""               -> ""
   "🦀🦀🦀ss"          -> "3🦀2s"
   Twelve As encode as "12A", not "9A3A" — which is the whole reason
   the decoder cannot just read one digit.

2. Decoding
   "3A3B2C1D2A"   -> "AAABBBCCDAA"
   "12A"          -> "AAAAAAAAAAAA"
   "1A1B1C"       -> "ABC"
   "A3B"          -> Err: character 'A' has no count in front of it
   "3A2"          -> Err: input ended with the count "2" and no character
   Malformed input is a Result, not a panic: a decoder is the half of
   this pair that meets data it did not produce.

3. The round trip
   "AAABBBCCDAA"            -> "3A3B2C1D2A"             -> "AAABBBCCDAA"            round trip ok
   "ABCDEF"                 -> "1A1B1C1D1E1F"           -> "ABCDEF"                 round trip ok
   "🦀🦀🦀ss"                  -> "3🦀2s"                   -> "🦀🦀🦀ss"                  round trip ok
   "Mississippi"            -> "1M1i2s1i2s1i2p1i"       -> "Mississippi"            round trip ok

4. Where the round trip breaks
   Input containing digits: "AA3BB"
     encode -> "2A132B"
     decode -> 134 chars, starting "AABB"
     round trip holds: false
   The lone '3' encoded as "13" — one 3 — and its digit then ran into
   the count in front of the Bs, so the decoder read "132B" as 132 Bs.
   This encoding has no escape, so its real precondition is "the alphabet
   contains no digits" — say that in a comment or in the signature,
   rather than discovering it in production.

5. And it is not always compression
   "ABCDEF"   6 chars -> 12 chars   BIGGER
   "AAAAAA"   6 chars -> 2 chars   smaller
   Run-length encoding pays only when runs are long. On text with no
   repeats it doubles the size, which is why real formats keep a literal
   mode and switch between the two.
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
