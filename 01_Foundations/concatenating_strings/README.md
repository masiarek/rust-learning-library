# Concatenating strings

**Level:** 101 → 201 · working knowledge

**One line:** `format!("{a} {b}")` joins any two pieces of text, whoever owns them — and the reason there is no simpler answer is that `+` has exactly one impl, `String + &str`, so its left operand must **own** a buffer and its right must be borrowed.

| you write | left | right | result |
|---|---|---|---|
| `format!("{a}{b}")` | anything | anything | one new buffer, nothing consumed |
| `a + &b` | `String` | `&str` | `a` is consumed and its buffer becomes the answer |
| `parts.join(", ")` | a slice of either | | one buffer, sized before it is filled |
| `a + b` | `String` | `String` | **`E0308`** — expected `&str`; borrow the right side |
| `"a" + "b"` | `&str` | `&str` | **`E0369`** — neither side owns a buffer |

---

## Joining two names

```rust
let first = "Ada";
let last = "Lovelace";

let a = format!("{first} {last}");             // "Ada Lovelace" — both still usable
let b = first.to_owned() + " " + last;         // "Ada Lovelace" — an owned left operand
let c = String::from(first) + " " + last;      // "Ada Lovelace" — same thing, spelled differently
```

All three allocate exactly one buffer, and all three leave `first` and `last` usable. `to_owned()` and `String::from` are the same call wearing different names — [Making a `String`](../making_a_string/README.md) has the full set of five spellings and which one to prefer.

**Reach for `format!` by default.** It does not care who owns what, it takes as many pieces as you like, and it reads in the order the finished sentence does — where a `+` chain makes you sprinkle `&` and keep track of which variable you are about to lose.

## The one impl there is

```rust
impl Add<&str> for String
```

That single line is the whole subject. Left operand owned and **consumed**, right operand borrowed — no other combination has an implementation, so no other combination compiles.

Consuming the left side is not a wart. The result *is* that buffer, grown in place, so a chain like `a + ", " + &b + "!"` allocates nothing after the first piece:

```rust
let mut left = String::with_capacity(32);
left.push_str("Ada");
let full = left + " " + "Lovelace";   // "Ada Lovelace" — same heap buffer, no reallocation
```

The `&` on the right is compulsory for the same reason the left must be owned: `&str` is the only right-hand type there is.

## `+=` is `push_str` with an operator

```rust
let mut greeting = String::from("Hello, ");
greeting += first;     // "Hello, Ada"   — impl AddAssign<&str> for String
greeting += "!";       // "Hello, Ada!"
```

Same shape as `Add`, so the same rule: the thing on the left has to own a buffer.

## More than two pieces

```rust
let parts = ["Ada", "Ben", "Cara"];
parts.concat();       // "AdaBenCara"
parts.join(", ");     // "Ada, Ben, Cara"
```

Both work on a slice of `&str` **or** of `String`, and both walk the pieces once to size the buffer before filling it — one allocation, no growth. That is the thing a `+` chain cannot do, and the reason `join` is the right answer the moment the number of pieces stops being a number you typed.

## The leading piece is the one that must be owned

Because `+` only ever appends to its left operand, the **first** fragment of a sentence is the one that has to own bytes:

```rust
let g = String::from("Hello, ") + first + " " + last + "!";   // "Hello, Ada Lovelace!"
```

Pre-size that left operand with `String::with_capacity` and the whole four-piece chain performs zero allocations.

## When the compiler refuses

Three error codes, one fact. Each of these is a line that does **not** compile, so they are shown commented out:

```rust
// let full = first + last;                        E0369: cannot add `&str` to `&str`
// let full = String::from(first) + String::from(last);
//                                                 E0308: expected `&str`, found `String`
// let mut t = "Hello, "; t += first;              E0368: `+=` cannot be applied to `&str`
```

All three are the same complaint: **`+` grows its left operand, so the left operand has to own bytes.** `E0308` is the near miss — the left side qualifies and the right side is one `&` away. `E0369` and `E0368` are the same missing buffer in two spellings.

The `E0369` in full, because its `note:` line states the rule better than any prose here does:

```text
error[E0369]: cannot add `&str` to `&str`
 --> join.rs:4:22
  |
4 |     let full = first + last;
  |                ----- ^ ---- &str
  |                |     |
  |                |     `+` cannot be used to concatenate two `&str` strings
  |                &str
  |
  = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
  |
4 |     let full = first.to_owned() + last;
  |                     +++++++++++
```

A `&str` is a pointer and a length — a **view** of bytes somebody else owns. Two literals point into the compiled binary itself, which is read-only and exactly the size of the text already in it. So `first + last` has nowhere to put the joined bytes, and rather than allocate one behind your back, Rust declines to have an operator for it. The `help:` is worth taking literally: `.to_owned()` on the left is the smallest edit that compiles, and `format!` is the one that stops the question arising.

## If you are coming from another language

**Python.** `+` on two `str` just works, because every Python string owns its characters and the result is a third object — there is no borrowed-view type to be caught out by. Rust splits that one type in two, and `+` is only defined on the owning half.

| Python | | Rust |
|---|---|---|
| `a + b` on two literals | fine — both are `str` | `E0369` — both are `&str` |
| `str(a) + b` | a no-op, already a `str` | `a.to_owned() + b` — now the left owns a buffer |
| `f"{a} {b}"` | new string, `a` and `b` untouched | `format!("{a} {b}")` — the same, and the default answer |
| `", ".join(parts)` | separator first | `parts.join(", ")` — separator last, list first |
| `a += b` | rebinds `a` to a new object | `a += b` — appends into `a`'s existing buffer |

What actually changes: Python's `+` is always an allocation and never a mutation, so `a + b` costs the same whoever wrote it. Rust's is a **move** — `a + &b` consumes `a`, and using `a` afterwards is `E0382`. In exchange the operator is free of allocation, which is the trade the ownership system exists to offer. See [Ownership and moves](../ownership_and_moves/README.md) if that `E0382` is the part that stings.

**ABAP.** `&&` concatenates two `string`s into a third and never consumes either, exactly like Python's `+` — and exactly unlike Rust's.

| ABAP | | Rust |
|---|---|---|
| `lv = lv_a && lv_b.` | builds a new string | `format!("{a}{b}")` — the honest equivalent |
| | | `a + &b` — cheaper, but `a` is gone afterwards |
| `CONCATENATE a b INTO c.` | statement form of the same | `format!("{a}{b}")` |
| `CONCATENATE LINES OF it INTO c SEPARATED BY ', '.` | joins an internal table | `v.join(", ")` |
| `\|Hello, { lv_name }!\|` | string template | `format!("Hello, {name}!")` |
| `lv = lv && lv_x.` in a loop | a new string each pass | `lv.push_str(&x)` — grows one buffer |

What actually changes: ABAP has one text type where Rust has two, so the question "does this side own its characters?" never comes up — `&&` accepts any two `string`s, any two `c` fields, and any mix. Rust makes you answer it at every call. The payoff is the loop in the last row: ABAP's `&&` is quadratic there and the standard advice is to avoid it, while `push_str` appends into the buffer you already have.

---

## Practice

**Greet two people three ways, then find the three refusals behind it.** Build `"Hello, Ada Lovelace!"` with `format!`, with a chain of `+`, and with `join` — and for each, say which inputs are still usable afterwards. Then pre-size the `+` chain's left operand with `String::with_capacity(64)` and prove it never reallocates.

Now go the other way and make the compiler say no, on purpose, three times: `first + last`, then the same thing with two `String`s and no `&`, then `let mut t = "Hello, "; t += first;`. Record the error code for each, and say in one line what all three are complaining about — it is the same fact three times.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:concatenating_strings_kata -->
*[`concatenating_strings_kata.rs`](examples/concatenating_strings_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: greet two people three ways, then find the three refusals behind it.
//!
//!   rustc --edition 2024 concatenating_strings_kata.rs -o /tmp/csk && /tmp/csk

fn main() {
    let first = "Ada";
    let last = "Lovelace";

    println!("PART 1 — the greeting, three ways");

    let by_format = format!("Hello, {first} {last}!");
    println!("   format!      {by_format:?}");
    println!("                first = {first:?}, last = {last:?} — both still usable");
    println!("                1 buffer, sized from the finished string");

    let by_plus = String::from("Hello, ") + first + " " + last + "!";
    println!("   chained +    {by_plus:?}");
    println!("                first and last are only borrowed on the right, so both live");
    println!("                1 buffer, grown in four steps — see PART 3");

    let by_join = ["Hello,", first, last].join(" ") + "!";
    println!("   join         {by_join:?}");
    println!("                join sizes the buffer up front, then + adds the '!'");

    println!("\n   agree: {}", by_format == by_plus && by_plus == by_join);

    println!("\nPART 2 — why the LEADING piece is the one that must be owned");
    // `+` only ever appends to its left operand, so the first fragment of the
    // sentence is the one that has to own a buffer. That is why PART 1 reads
    // String::from("Hello, ") + ... and not "Hello, " + ...
    let mut left = String::with_capacity(64);
    left.push_str("Hello, ");
    let ptr = left.as_ptr();
    let cap = left.capacity();
    let grown = left + first + " " + last + "!";
    println!("   capacity {cap} before, {} after", grown.capacity());
    println!("   same buffer the whole way: {}", grown.as_ptr() == ptr);
    println!("   {grown:?}");
    println!("   Four appends, zero new allocations — the left operand was pre-sized");
    println!("   and every `+` handed the same buffer to the next one.");

    println!("\nPART 3 — the three refusals");
    //   let x = first + last;
    //   error[E0369]: cannot add `&str` to `&str`
    //     = note: string concatenation requires an owned `String` on the left
    //
    //   let a = String::from("Ada");
    //   let b = String::from("Lovelace");
    //   let y = a + b;
    //   error[E0308]: mismatched types — expected `&str`, found `String`
    //   help: consider borrowing here:  a + &b
    //
    //   let mut t = "Hello, ";
    //   t += first;
    //   error[E0368]: binary assignment operation `+=` cannot be applied to type `&str`
    println!("   &str  + &str     E0369   neither side owns a buffer to grow");
    println!("   String + String  E0308   the ONE impl is Add<&str>; borrow the right side");
    println!("   &str  += &str    E0368   same missing buffer, assignment spelling");
    println!("   One fact, three spellings: `+` grows the left operand, so the left");
    println!("   operand has to own bytes. PART 2 is that fact used deliberately.");

    println!("\nPART 4 — what to write");
    println!("   Reach for format! by default: it does not care who owns what, and it");
    println!("   reads as the sentence it produces. Reach for + only when you already");
    println!("   hold an owned String on the LEFT and are finished with it.");
}
```
<!-- /source -->

<!-- output:concatenating_strings_kata -->
*Verified output of [`concatenating_strings_kata.rs`](examples/concatenating_strings_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
PART 1 — the greeting, three ways
   format!      "Hello, Ada Lovelace!"
                first = "Ada", last = "Lovelace" — both still usable
                1 buffer, sized from the finished string
   chained +    "Hello, Ada Lovelace!"
                first and last are only borrowed on the right, so both live
                1 buffer, grown in four steps — see PART 3
   join         "Hello, Ada Lovelace!"
                join sizes the buffer up front, then + adds the '!'

   agree: true

PART 2 — why the LEADING piece is the one that must be owned
   capacity 64 before, 64 after
   same buffer the whole way: true
   "Hello, Ada Lovelace!"
   Four appends, zero new allocations — the left operand was pre-sized
   and every `+` handed the same buffer to the next one.

PART 3 — the three refusals
   &str  + &str     E0369   neither side owns a buffer to grow
   String + String  E0308   the ONE impl is Add<&str>; borrow the right side
   &str  += &str    E0368   same missing buffer, assignment spelling
   One fact, three spellings: `+` grows the left operand, so the left
   operand has to own bytes. PART 2 is that fact used deliberately.

PART 4 — what to write
   Reach for format! by default: it does not care who owns what, and it
   reads as the sentence it produces. Reach for + only when you already
   hold an owned String on the LEFT and are finished with it.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:concatenating_strings -->
*Verified output of [`concatenating_strings.rs`](examples/concatenating_strings.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Joining two names
   format!("{first} {last}")     "Ada Lovelace"
   first.to_owned() + " " + last   "Ada Lovelace"
   String::from(first) + " " + last  "Ada Lovelace"
   All three allocate exactly one buffer, and all three leave
   first = "Ada" and last = "Lovelace" usable afterwards.
   Only format! reads in the order the finished sentence does.

2. The one impl there is: String + &str
   left        right      compiles   why
   String      &str       yes        the left buffer is taken and grown
   String      String     NO         E0308 — expected `&str`, borrow the right
   &str        &str       NO         E0369 — neither side owns a buffer
   &str        String     NO         E0369 — the left side still owns nothing

3. What `+` does to its left operand
   capacity before 32, after 32
   same heap buffer reused: true
   "Ada Lovelace"
   The answer IS the left buffer, grown — which is why `+` consumes it,
   and why the right side, being only borrowed, survives: "Lovelace"

4. `+=` is push_str wearing an operator
   "Hello, Ada!"

5. More than two pieces
   parts.concat()        "AdaBenCara"
   parts.join(", ")      "Ada, Ben, Cara"
   owned.join(" | ")     "Ada | Ben | Cara"
   Both take a slice of &str OR of String, and both size the buffer
   up front — which a chain of `+` cannot do.

6. The three refusals, and the one fact under them
   &str  + &str     E0369   neither side owns a buffer to grow
   String + String  E0308   the one impl is Add<&str>; borrow the right
   &str  += &str    E0368   same missing buffer, assignment spelling
   All three say one thing: `+` grows its LEFT operand, so the left
   operand has to be something that owns bytes.

7. Which to reach for
   two or three known pieces        -> format!
   a whole collection               -> .join(sep) / .concat()
   a left value you are done with   -> + , and let it eat the buffer
   appending in a loop              -> push_str — see Building a String
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/concatenating_strings/examples/concatenating_strings.rs -o /tmp/cs && /tmp/cs
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [Building a `String`](../building_a_string/README.md) — the other half of this: `push_str`, `write!`, editing in the middle, and what to do inside a loop
- [Making a `String`](../making_a_string/README.md) — `to_owned` vs `to_string` vs `String::from` vs `into`
- [`String` vs `&str`](../string_vs_str/README.md) — why the two types exist, if the split is still new
- [Ownership and moves](../ownership_and_moves/README.md) — the `E0382` you get from touching the left operand afterwards
- [The anatomy of a `String`](../anatomy_of_a_string/README.md) — the capacity the `+` chain is reusing
- [`std::ops::Add` for `String` ↗](https://doc.rust-lang.org/std/string/struct.String.html#impl-Add%3C%26str%3E-for-String) · [`rustc --explain E0369` ↗](https://doc.rust-lang.org/error_codes/E0369.html) · [The Rust Book, ch. 8.2 ↗](https://doc.rust-lang.org/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro)
