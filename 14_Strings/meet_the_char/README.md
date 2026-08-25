# Meet the `char`

**Level:** 101 → 201 · working knowledge

**One line:** A `char` is one Unicode scalar value — four bytes, always. Inside a `String` the same character is one to four UTF-8 bytes, which is why `.len()` is not "how many characters", `s[0]` refuses to compile, and "how long is this string" has three honest answers.

```rust
let c: char = 'a';    // single quotes: a char
let s: &str = "a";    // double quotes: a string that happens to hold one char
```

Different types, and the difference is not pedantry: `'a'` is a **decoded** character you can compare, classify and range over (`'0'..='9'`); `"a"` is **encoded** bytes. The same letter costs 4 bytes as a `char` value and 1 byte inside a string — decoded values are fixed-width so they can be worked on, encoded bytes are variable-width so they can be stored small.

---

## Three answers to "how long"

```rust
let name = "Zoë";
name.len()             // 4 — bytes
name.chars().count()   // 3 — chars (Unicode scalar values)
```

The `ë` costs two bytes in UTF-8, so byte count and char count part ways on the first accent. [Meet the byte](../../19_Numbers/meet_the_byte/README.md) already warned that `.len()` counts bytes; this page is the other half — what those bytes encode:

| answer | asks | costs |
|---|---|---|
| **bytes** | `.len()` | O(1) — it is the `len` field |
| **chars** | `.chars().count()` | O(n) — UTF-8 must be walked |
| **graphemes** | what a *reader* calls one character | a crate — std stops before this |

The third row is real: `"e\u{301}"` — an `e` plus a combining accent — is **two** chars that render as one `é`. The [verified output](#the-verified-output) shows the two spellings printing identically and comparing unequal. Normalizing them into one form is the [`unicode-normalization` ↗](https://crates.io/crates/unicode-normalization) crate's job; counting reader-characters is [`unicode-segmentation` ↗](https://crates.io/crates/unicode-segmentation)'s.

## Why `s[0]` does not compile

```rust
let first = name[0];
```

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
```

An index promises O(1), and the only O(1) thing in a `String` is a byte — but byte 0 might be the *middle third* of some character, and handing you a raw byte from text would be an answer to a question you did not ask. Rust refuses to guess which you meant, and offers each meaning under its own name:

```rust
name.bytes().nth(0)     // Some(90) — the byte, if you really mean bytes
name.chars().nth(0)     // Some('Z') — the char, at the honest O(n) price
name.get(0..1)          // Some("Z") — a subslice, refused as None off-boundary
```

Byte-range slicing (`&name[0..1]`) works too — but **panics** on a range that cuts a character in half. `get` is the same question with `None` instead of a crash, and `is_char_boundary(i)` is how you ask first. The output below shows `get(2..3)` refusing the middle of `ë`.

## Case is not one-to-one

```rust
let upper: String = 'ß'.to_uppercase().collect();   // "SS"
```

`to_uppercase` on a `char` returns an **iterator**, not a `char` — because German `ß` uppercases to two letters. A whole `&str` has `.to_uppercase() -> String`, which absorbs that quietly. Anything assuming "one char in, one char out" — a fixed-size buffer, an index math shortcut — is wrong in exactly the languages where you will not notice for months.

## If you are coming from another language

**Python.** The two languages disagree about what `len` counts, and it is the #1 cross-language string surprise in both directions.

| Python | | Rust |
|---|---|---|
| `len("Zoë")` → 3 | counts code points | `.len()` → 4 counts **bytes**; `.chars().count()` → 3 |
| `s[0]` → `'Z'` | O(1), allowed | refused — `.chars().nth(0)`, honestly O(n) |
| no char type — `'a'` is a 1-char `str` | | `char` is its own 4-byte type |
| `"é" != "é"` | same problem | same problem — `unicodedata.normalize` ↔ the normalization crate |

Python affords `s[0]` by storing each string at a fixed width — 1, 2 or 4 bytes per character, chosen per string (PEP 393) — paying memory for O(1) indexing. Rust stores UTF-8 and refuses to pretend indexing is cheap. Neither counts graphemes; both leave that to a library.

**ABAP.** ABAP text is UTF-16, so it has the *same* mismatch one layer up.

| ABAP | | Rust |
|---|---|---|
| `strlen( lv )` | counts UTF-16 **code units** | `.len()` counts UTF-8 **bytes** |
| `lv+2(1)` | slices by code unit — can cut a surrogate pair in half | `&s[2..3]` panics / `get` returns `None` at a cut |
| an emoji costs 2 units | the BMP boundary | an emoji costs 4 bytes |
| `string` vs `xstring` | text vs raw bytes | `String` vs `Vec<u8>` — the same split |

What changes: ABAP lets the bad slice happen and hands you half a surrogate at runtime; Rust makes the cut refuse loudly (`None`) or immediately (a panic at the slice), never silently.

---

## Practice

**One name, three lengths.** Write `inventory(s: &str)` printing one row per char: byte offset, the char, its UTF-8 width, its `U+XXXX` code point (`char_indices`, `len_utf8`, `as u32`). Run it on `"Łódź"`.

Then spell `"Zoé"` twice — composed (`é`) and decomposed (`e\u{301}`) — and: prove the two are `!=` while printing identically; give byte and char counts for both; and say which of the three length answers std cannot compute. Finish with `get` on a range that cuts `Ł` in half.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:meet_the_char_kata -->
*[`meet_the_char_kata.rs`](examples/meet_the_char_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one name, three lengths — a per-char inventory, and the
//! accent that makes two equal-looking strings unequal.
//!
//!   rustc --edition 2024 meet_the_char_kata.rs -o /tmp/mtck && /tmp/mtck

/// One row per char: byte offset, the char, its UTF-8 width, its code point.
fn inventory(label: &str, s: &str) {
    println!("   {label} = {s:?}");
    for (offset, c) in s.char_indices() {
        println!("      byte {offset}: {c:?}   {} byte(s)   U+{:04X}", c.len_utf8(), c as u32);
    }
    println!("      -> {} bytes, {} chars", s.len(), s.chars().count());
}

fn main() {
    println!("Round 1 — the inventory");
    inventory("city", "Łódź");

    println!("\nRound 2 — the same-looking name, spelled two ways");
    let composed = "Zoé"; // é as one char, U+00E9
    let decomposed = "Zoe\u{301}"; // e + combining acute, two chars
    inventory("composed", composed);
    inventory("decomposed", decomposed);
    println!("   composed == decomposed?  {}", composed == decomposed);

    println!("\nRound 3 — which counts can std give you?");
    println!("   bytes:     .len()            -> {} vs {}", composed.len(), decomposed.len());
    println!("   chars:     .chars().count()  -> {} vs {}", composed.chars().count(), decomposed.chars().count());
    println!("   graphemes: what a reader sees -> 3 vs 3, but std cannot count");
    println!("              these; the unicode-segmentation crate can");

    println!("\nRound 4 — safe slicing needs a boundary");
    let city = "Łódź";
    println!("   city.get(0..1) = {:?}   <- inside 'Ł', refused as None", city.get(0..1));
    println!("   city.get(0..2) = {:?}", city.get(0..2));
    let first = city.chars().next();
    println!("   city.chars().next() = {first:?}   <- the honest way to ask for one char");
}
```
<!-- /source -->

<!-- output:meet_the_char_kata -->
*Verified output of [`meet_the_char_kata.rs`](examples/meet_the_char_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Round 1 — the inventory
   city = "Łódź"
      byte 0: 'Ł'   2 byte(s)   U+0141
      byte 2: 'ó'   2 byte(s)   U+00F3
      byte 4: 'd'   1 byte(s)   U+0064
      byte 5: 'ź'   2 byte(s)   U+017A
      -> 7 bytes, 4 chars

Round 2 — the same-looking name, spelled two ways
   composed = "Zoé"
      byte 0: 'Z'   1 byte(s)   U+005A
      byte 1: 'o'   1 byte(s)   U+006F
      byte 2: 'é'   2 byte(s)   U+00E9
      -> 4 bytes, 3 chars
   decomposed = "Zoe\u{301}"
      byte 0: 'Z'   1 byte(s)   U+005A
      byte 1: 'o'   1 byte(s)   U+006F
      byte 2: 'e'   1 byte(s)   U+0065
      byte 3: '\u{301}'   2 byte(s)   U+0301
      -> 5 bytes, 4 chars
   composed == decomposed?  false

Round 3 — which counts can std give you?
   bytes:     .len()            -> 4 vs 5
   chars:     .chars().count()  -> 3 vs 4
   graphemes: what a reader sees -> 3 vs 3, but std cannot count
              these; the unicode-segmentation crate can

Round 4 — safe slicing needs a boundary
   city.get(0..1) = None   <- inside 'Ł', refused as None
   city.get(0..2) = Some("Ł")
   city.chars().next() = Some('Ł')   <- the honest way to ask for one char
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:meet_the_char -->
*Verified output of [`meet_the_char.rs`](examples/meet_the_char.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One string, two counts
   "Zoë".len()           = 4 bytes
   "Zoë".chars().count() = 3 chars

2. Where each char sits, and what it costs in the string
   byte 0: 'Z'   1 byte(s) in UTF-8, U+005A
   byte 1: 'o'   1 byte(s) in UTF-8, U+006F
   byte 2: 'ë'   2 byte(s) in UTF-8, U+00EB

3. A char VALUE is always 4 bytes — decoded, not encoded
   size_of::<char>() = 4 bytes
   'ë' costs 4 bytes as a char, 2 bytes inside a String

4. Slicing is by byte, and only at char boundaries
   name.get(0..1) = Some("Z")
   name.get(2..4) = Some("ë")
   name.get(2..3) = None      <- byte 3 is the middle of 'ë'
   name.is_char_boundary(3) = false

5. Case is not one-to-one
   'ß'.to_uppercase() = "SS"   <- one char in, two out

6. Two spellings that look identical
   composed   "é"  1 char(s), 2 bytes
   decomposed "e\u{301}"  2 char(s), 3 bytes
   composed == decomposed?  false
   what a READER calls one character is a third counting — the
   grapheme — and std stops before it; that one needs a crate
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/meet_the_char/examples/meet_the_char.rs -o /tmp/mtc && /tmp/mtc
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [Meet the byte](../../19_Numbers/meet_the_byte/README.md) — the unit; this page is what the bytes *mean*
- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — where those bytes live
- [Six kinds of string](../six_kinds_of_string/README.md) — the types for text that does *not* keep UTF-8's promise
- [The Rust Book, ch. 8.2 ↗](https://doc.rust-lang.org/book/ch08-02-strings.html) — its "bytes, scalar values, grapheme clusters" section is this page's origin story
