# Concatenating strings

**Level:** 101 → 201 · working knowledge

**One line:** `+` has exactly one impl — `String + &str` — so the left operand must **own** a buffer and the right one must be borrowed; two literals are neither, which is why `"Adam" + "Masiarek"` is `E0369` and why `format!` is the answer that never asks the question.

| you write | left | right | result |
|---|---|---|---|
| `"a" + "b"` | `&str` | `&str` | **`E0369`** — nothing owns a buffer |
| `"a" + s` | `&str` | `String` | **`E0369`** — the left side still owns nothing |
| `a + b` | `String` | `String` | **`E0308`** — expected `&str`; borrow the right side |
| `a + &b` | `String` | `&str` | compiles — `a` is consumed and its buffer becomes the answer |
| `format!("{a}{b}")` | anything | anything | compiles — one new buffer, nothing consumed |

---

## The refusal

```rust
fn main() {
    let s1 = "Adam";
    let s2 = "Masiarek";
    println!("Hello, {}!", s1 + s2);
}
```

```text
error[E0369]: cannot add `&str` to `&str`
 --> adam.rs:5:31
  |
5 |     println!("Hello, {}!", s1 + s2);
  |                            -- ^ -- &str
  |                            |  |
  |                            |  `+` cannot be used to concatenate two `&str` strings
  |                            &str
  |
  = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
  |
5 |     println!("Hello, {}!", s1.to_owned() + s2);
  |                              +++++++++++
```

A `&str` is a pointer and a length — a **view** of bytes somebody else owns. `s1` and `s2` here point into the compiled binary itself, which is read-only and exactly the size of the text already in it. So `s1 + s2` has nowhere to put the joined bytes, and rather than allocate one behind your back, Rust declines to have an operator for it.

## The compiler's fix, and what it does

```rust
println!("Hello, {}!", s1.to_owned() + s2);  // Hello, AdamMasiarek!
```

`to_owned()` copies the four bytes of `"Adam"` onto the heap as a `String`. Now the left operand owns a buffer, so there is an impl to call — and there is only ever one:

```rust
impl Add<&str> for String
```

Left side owned and **consumed**, right side borrowed. That asymmetry is not a wart: the result *is* the left buffer, grown in place, so a chain like `a + ", " + &b + "!"` allocates nothing after the first piece. It is also why the `&` on the right is compulsory — `a + b` with two `String`s is `E0308`, and the help text tells you to write `a + &b`.

## Three ways to say the same thing

```rust
let a = format!("{s1} {s2}");            // "Adam Masiarek" — s1, s2 both still usable
let b = s1.to_owned() + " " + s2;        // "Adam Masiarek" — the compiler's suggestion
let c = String::from(s1) + " " + s2;     // "Adam Masiarek" — same thing, spelled differently
```

All three allocate exactly one buffer. `to_owned()` and `String::from` are the same call wearing different names — [Making a `String`](../making_a_string/README.md) has the full set of five spellings and which one to prefer.

Reach for `format!` by default. It does not care who owns what, it takes as many pieces as you like, and it reads in the order the finished sentence does — where a `+` chain makes you sprinkle `&` and think about which variable you are about to lose.

## `+=` works where `+` does not

```rust
let mut greeting = String::from("Hello, ");
greeting += s1;     // "Hello, Adam" — this is push_str with an operator
greeting += "!";    // "Hello, Adam!"
```

`impl AddAssign<&str> for String` — same shape as `Add`, so the same rule holds: the thing on the left has to own a buffer. Try it on a view and you get the third member of the family:

```text
error[E0368]: binary assignment operation `+=` cannot be applied to type `&str`
```

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
let g = String::from("Hello, ") + s1 + " " + s2 + "!";  // Hello, Adam Masiarek!
```

`"Hello, " + s1` would be the same `E0369` again. Once the left operand is owned, every `+` after it hands the same buffer to the next one — pre-size it with `String::with_capacity` and a four-piece chain performs zero allocations.

## If you are coming from another language

**Python.** `+` on two `str` just works, because every Python string owns its characters and the result is a third object — there is no borrowed-view type to be caught out by. Rust splits that one type in two, and `+` is only defined on the owning half.

| Python | | Rust |
|---|---|---|
| `s1 + s2` on two literals | fine — both are `str` | `E0369` — both are `&str` |
| `s1 + s2` after `s1 = str(s1)` | no-op, already a `str` | `s1.to_owned() + s2` — now the left owns a buffer |
| `f"{a} {b}"` | new string, `a` and `b` untouched | `format!("{a} {b}")` — the same, and the default answer |
| `", ".join(parts)` | separator first | `parts.join(", ")` — separator last, list first |
| `s1 += s2` | rebinds `s1` to a new object | `s1 += s2` — appends into `s1`'s existing buffer |

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

**Greet two people — but make the compiler refuse it three times first.** Start from the two-literal program above and write, in turn, `first + last`, then the same thing with two `String`s and no `&`, then `let mut t = "Hello, "; t += first;`. Record the error code for each and say, in one line, what all three are complaining about — it is the same fact three times.

Then build `"Hello, Adam Masiarek!"` three ways: with `format!`, with a chain of `+`, and with `join`. For each, say which inputs are still usable afterwards. Finally, pre-size the `+` chain's left operand with `String::with_capacity(64)` and prove it never reallocates.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:concatenating_strings_kata -->
*[`concatenating_strings_kata.rs`](examples/concatenating_strings_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: greet two people, and make the compiler refuse it three ways first.
//!
//!   rustc --edition 2024 concatenating_strings_kata.rs -o /tmp/csk && /tmp/csk

fn main() {
    let first = "Adam";
    let last = "Masiarek";

    println!("PART 1 — the three refusals");
    //
    //   let x = first + last;
    //   error[E0369]: cannot add `&str` to `&str`
    //     = note: string concatenation requires an owned `String` on the left
    //
    //   let a = String::from("Adam");
    //   let b = String::from("Masiarek");
    //   let y = a + b;
    //   error[E0308]: mismatched types — expected `&str`, found `String`
    //   help: consider borrowing here:  a + &b
    //
    //   let mut t = "Hello, ";
    //   t += first;
    //   error[E0368]: binary assignment operation `+=` cannot be applied to type `&str`
    //
    println!("   &str  + &str    E0369  neither side owns a buffer to grow");
    println!("   String + String E0308  the ONE impl is Add<&str>; borrow the right side");
    println!("   &str  += &str   E0368  same missing buffer, assignment spelling");
    println!("   All three are the same fact: `+` grows the left operand, so the left");
    println!("   operand has to be something that owns bytes.");

    println!("\nPART 2 — the greeting, three ways");

    let by_format = format!("Hello, {first} {last}!");
    println!("   format!      {by_format:?}");
    println!("                first = {first:?}, last = {last:?} — both still usable");
    println!("                1 buffer, sized from the finished string");

    let by_plus = String::from("Hello, ") + first + " " + last + "!";
    println!("   chained +    {by_plus:?}");
    println!("                first = {first:?}, last = {last:?} — only borrowed on the right");
    println!("                1 buffer, but grown in four steps — see PART 3");

    let by_join = ["Hello,", first, last].join(" ") + "!";
    println!("   join         {by_join:?}");
    println!("                join sizes the buffer up front, then + adds the '!'");

    println!("\n   agree: {}", by_format == by_plus && by_plus == by_join);

    println!("\nPART 3 — why the LEADING piece is the one that must be owned");
    // `+` can only ever append to its left operand, so the first piece of the
    // sentence is the one that has to own a buffer. That is the whole reason
    // this reads String::from("Hello, ") + ... and not "Hello, " + ...
    let mut left = String::with_capacity(64);
    left.push_str("Hello, ");
    let ptr = left.as_ptr();
    let cap = left.capacity();
    let grown = left + first + " " + last + "!";
    println!("   capacity {cap} before, {} after", grown.capacity());
    println!("   same buffer the whole way: {}", grown.as_ptr() == ptr);
    println!("   {grown:?}");
    println!("   Four appends, zero new allocations — because the left operand was");
    println!("   pre-sized and every `+` handed the same buffer to the next one.");

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
PART 1 — the three refusals
   &str  + &str    E0369  neither side owns a buffer to grow
   String + String E0308  the ONE impl is Add<&str>; borrow the right side
   &str  += &str   E0368  same missing buffer, assignment spelling
   All three are the same fact: `+` grows the left operand, so the left
   operand has to be something that owns bytes.

PART 2 — the greeting, three ways
   format!      "Hello, Adam Masiarek!"
                first = "Adam", last = "Masiarek" — both still usable
                1 buffer, sized from the finished string
   chained +    "Hello, Adam Masiarek!"
                first = "Adam", last = "Masiarek" — only borrowed on the right
                1 buffer, but grown in four steps — see PART 3
   join         "Hello, Adam Masiarek!"
                join sizes the buffer up front, then + adds the '!'

   agree: true

PART 3 — why the LEADING piece is the one that must be owned
   capacity 64 before, 64 after
   same buffer the whole way: true
   "Hello, Adam Masiarek!"
   Four appends, zero new allocations — because the left operand was
   pre-sized and every `+` handed the same buffer to the next one.

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
1. The refusal
   s1 and s2 are both &str — a pointer and a length, owning nothing.
   There is nowhere to put the joined bytes, so there is no `+` to call.

2. The one impl there is: String + &str
   left        right      compiles   why
   &str        &str       NO         E0369 — neither side owns a buffer
   &str        String     NO         E0369 — the left side still owns nothing
   String      String     NO         E0308 — expected `&str`, found `String`
   String      &str       yes        the left buffer is taken and grown

3. Three ways to say what Adam meant
   format!("{s1} {s2}")   "Adam Masiarek"   s1, s2 both still usable
   s1.to_owned() + " " + s2  "Adam Masiarek"   the compiler's own suggestion
   String::from(s1) + " " + s2  "Adam Masiarek"   same thing, spelled differently
   All three allocate exactly one buffer. Only format! reads as the sentence it builds.

4. What `+` actually does to its left operand
   capacity before 32, after 32
   same heap buffer reused: true
   "Adam Masiarek"
   That is why `+` consumes the left side: the answer IS the left buffer, grown.
   The right side is only borrowed, so s2 is still usable: "Masiarek"

5. `+=` works where `+` does not — on a String
   "Hello, Adam!"
   `let mut t = "Hello, "; t += s1;` is E0368 — a &str has no buffer to append to.

6. More than two pieces
   parts.concat()        "AdaBenCara"
   parts.join(", ")      "Ada, Ben, Cara"
   owned.join(" | ")     "Ada | Ben | Cara"
   Both take a slice of &str OR of String, and both allocate once,
   sized up front — which a chain of `+` cannot do.

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
- [`std::ops::Add` for `String`](https://doc.rust-lang.org/std/string/struct.String.html#impl-Add%3C%26str%3E-for-String) · [`rustc --explain E0369`](https://doc.rust-lang.org/error_codes/E0369.html) · [The Rust Book, ch. 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro)
