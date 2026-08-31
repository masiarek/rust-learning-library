# Collect the iterator into a `Vec`

**Level:** 101 → 201 · working knowledge

**One line:** `collect()` is a decision to **materialize** — run the sequence now, allocate once, and get back something you can measure, index, sort and read twice; most of the questions people collect in order to ask never needed any of that.

```rust
fn main() {
    let s = "a:b:c";

    // Option 1: collect the iterator into a Vec.
    let r: Vec<&str> = s.split(':').collect();
    println!("{r:?}");        // ["a", "b", "c"]

    // Option 2: walk it with a for loop.
    for part in s.split(':') {
        println!("{part}");   // a, then b, then c
    }
}
```

Both lines get the same three pieces. The first builds a `Vec` to hold them; the second never builds a collection at all. `split` hands back a lazy [`Split`](../../14_Strings/inside_a_split/README.md) — a cursor and a needle, no pieces yet — and `collect` is one of the things that can run it.

Write `split(':')` with a `char` rather than `split(":")` with a one-character string: the `&str` needle drags in std's full Two-Way search machinery, and — as the last part of *[What it costs](#what-it-costs-and-the-six-questions-that-need-no-vec)* shows — it also costs you `.rev()`.

## What the `Vec` buys

| You wrote | Because |
|---|---|
| `parts.len()` | an iterator has no length until it has been walked |
| `parts[1]`, [`parts.last()` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.last) | random access; an iterator only goes forward, one step at a time |
| [`parts.sort_unstable()` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable) | sorting cannot begin until the last item has arrived |
| [`parts.join(", ")` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.join) | same reason — it is a **slice** method, not an adapter |
| two passes over the pieces | an iterator is spent once; the `Vec` can be read as often as you like |
| `struct Ballot { names: Vec<String> }` | a field has to hold a value, and a half-run pipeline is not one |

There is no `Iterator::sort` and no `Iterator::join`, and their absence is not an oversight. Both need every element before they can produce the first byte of an answer, which is exactly the property `collect` supplies and laziness does not.

## What it costs, and the six questions that need no `Vec`

One heap allocation, and the whole sequence computed whether you needed all of it or not. Counting the closure calls on `"cara:ada:ben:ada"` shows which of those two you were paying for:

```text
collect().len() = 4   4 pieces built, 1 Vec allocated
.count()        = 4   the same answer, no allocation
.next()         = "cara"  1 closure call
.find(b…)       = Some("ben")  3 closure calls, then it stopped
.any(== ada)    = true   2 closure calls, then it stopped
.position(ben)  = Some(2) 3 closure calls
.max_by_key(len)= Some("cara")
```

`collect().len()` and `count()` return the same number; only one of them built a `Vec` to throw away. The three short-circuiting lines are worse than that — collecting first would have read the whole line to answer a question the iterator settles after one or two steps.

Reversal is free too, and that is the surprise: `Split` is a [`DoubleEndedIterator`](../double_ended_and_exact_size/README.md), so `for name in line.split(':').rev()` needs no `Vec` and no `sort`. But only for a `char` pattern —

```text title="Abridged — real rustc output for rev_str.rs, one of two errors"
error[E0277]: the trait bound `StrSearcher<'_, '_>: DoubleEndedSearcher<'_>` is not satisfied
 --> rev_str.rs:5:40
  |
5 |     let b: Vec<&str> = line.split(":").rev().collect();
  |                                        ^^^ the nightly-only, unstable trait `DoubleEndedSearcher<'_>` is not implemented for `StrSearcher<'_, '_>`
  |
  = note: required for `std::str::Split<'_, &str>` to implement `DoubleEndedIterator`
```

Searching backwards for a multi-byte needle is a different algorithm, so std does not claim it. `rsplit(":")` is the one that works there, and it walks the pieces in reverse without ever being an `Iterator::rev`.

## The pieces are slices *of* the original

```text
"cara:ada:ben:ada"
byte offsets of the pieces -> [0, 5, 9, 13]
```

Not one character was copied. `Vec<&str>` is four `(pointer, length)` pairs aimed into the string they came from, which is why collecting a split is cheap — and why the `Vec` can never outlive that string.

## `Vec<&str>` or `Vec<String>`

The same pipeline, one annotation apart:

```rust
let borrowed: Vec<&str>   = line.split(':').collect();                     // the Vec, and nothing else
let owned:    Vec<String> = line.split(':').map(String::from).collect();   // the Vec, plus one String per piece
```

Reach for the owned one when the result has to outlive its source. This is where the borrowed spelling stops compiling, and the error is worth meeting on purpose:

```text
error[E0515]: cannot return value referencing local variable `lower`
 --> tidy.rs:3:5
  |
3 |     lower.split(':').map(str::trim).collect()
  |     -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     returns a value referencing data owned by the current function
  |     `lower` is borrowed here
```

That was `fn tidy(raw: &str) -> Vec<&str>` with a `let lower = raw.to_lowercase();` in the middle. The pieces point into `lower`, `lower` dies at the closing brace, and the `Vec` would outlive what it borrows. Taking the string **by value** fails one step earlier, and rustc spells out all three fixes:

```text
error[E0106]: missing lifetime specifier
 --> owned_arg.rs:1:31
  |
1 | fn parts(line: String) -> Vec<&str> {
  |                               ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: instead, you are more likely to want to change the argument to be borrowed...
  |
1 | fn parts(line: &String) -> Vec<&str> {
help: ...or alternatively, you might want to return an owned value
  |
1 + fn parts(line: String) -> Vec<String> {
```

The rule that decides it, and it is shorter than the lifetimes: **a borrowing pipeline can only do transformations that hand back a piece of the input.** `trim`, `strip_prefix` and `split` return subslices, so they stay in `Vec<&str>`; `to_lowercase`, `to_uppercase` and `replace` each build a new `String`, and that `String` has to belong to somebody. When most pieces need no change, [`Cow`](../../18_Ownership/clone_on_write/README.md) borrows the ones that are already right and owns only the rest.

## The `Vec` is never empty

```text
""                             -> len 1  [""]
"a"                            -> len 1  ["a"]
"a::b"                         -> len 3  ["a", "", "b"]
"a:b:"                         -> len 3  ["a", "b", ""]
"a:b:".split_terminator(':')   -> len 2  ["a", "b"]
"".split_whitespace()          -> len 0  []
```

`split` yields one more piece than there are separators — always, with no special case for the empty string. So `parts.is_empty()` is `false` for every input, and a `Vec` of length 1 tells you nothing about whether the line held anything: check `parts[0].is_empty()`, or use `split_whitespace`, which drops empties by design. A trailing separator is the other half of the same arithmetic, and `split_terminator` is the version that expects one.

## For `key=value` you want neither

```text
"port=8080"
   splitn(2, '=').collect() -> ["port", "8080"], and pieces.get(1) = Some("8080")
   split_once('=')          -> Some(("port", "8080"))
"debug"
   splitn(2, '=').collect() -> ["debug"], and pieces.get(1) = None
   split_once('=')          -> None
```

`pieces[1]` on the second line is a panic, and nothing in the types warned you: collecting turns "the separator was missing" into an out-of-bounds index at runtime. [`split_once` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.split_once) returns `Option<(&str, &str)>`, so the same malformed line is a `None` the compiler makes you handle — and it allocates nothing. Reach for it whenever you are about to collect a pair.

## If you are coming from another language

- **Python.** `s.split(":")` already *is* the list — Python materializes, and the lazy form is the awkward one (`re.finditer`, or a generator you wrote). Rust inverts that default, which is why the first `for part in s.split(':')` looks like it is missing a step and is in fact the cheaper of the two. Two habits to unlearn. `parts = s.split(":")` followed by `len(parts)`, `parts[0]` and a `for` over the same list is three uses of one allocation in Python and three separate decisions in Rust — `count()`, `next()`, and the loop, none of which need the `Vec`. And Python's `"".split(":")` returns `['']` exactly as Rust's does, while `"".split()` returns `[]` exactly as `split_whitespace` does: the surprising arithmetic is not Rust's, you have simply never had to name it. What is genuinely new is the borrow — a Python string slice is a fresh object, so no Python list can be invalidated by its source going away, and `E0515` has no counterpart to transfer.
- **ABAP.** `SPLIT lv_line AT ':' INTO TABLE lt_parts` always materializes into an internal table; there is no lazy split in the language, so *collect or not* is a decision ABAP never asked you to make. The nearest thing to Option 2 is `FIND` in a loop with an offset, which nobody writes. Three things transfer badly. The internal table **copies the text** — each `lt_parts` row owns its characters — so an ABAP habit of freeing or overwriting `lv_line` afterwards is fine there and is exactly the `E0515` mistake here. `LINES( lt_parts )` after a `SPLIT` is the `collect().len()` reflex, and `.count()` is the version that builds nothing. And ABAP's `SPLIT` on an empty string gives one empty row, matching Rust's `[""]` — the same one-more-piece-than-separators arithmetic, which is worth noticing because `sy-subrc` will not tell you the line was blank either. The 7.40 table comprehension `VALUE ty_tab( FOR wa IN lt_src ( wa-name ) )` is the closest ABAP comes to `collect`, and like Rust it names the target type on the left.
- **Java / C#.** `String.split` returns a `String[]` — eager, like Python. The stream form (`Arrays.stream(...)`, `.Split(...).Where(...)`) is where `.collect(Collectors.toList())` / `.ToList()` becomes the same decision Rust is asking about, with the same answer: `count()`, `anyMatch`, `findFirst` need no list. The difference Rust adds is the one Java and C# cannot have, because both copy: `String[]` elements are independent objects, so the array outlives the string it was split from without anyone thinking about it. `Vec<&str>` does not, and the compiler is what tells you.

---

## Practice

**The `Vec` that could not outlive its string.** Write `fn tidy(raw: &str) -> Vec<&str>` that lowercases the line and trims each piece, and read the `E0515` it earns. Then fix it four ways and say what each one allocates: return `Vec<String>`; move the lowercasing to the call site so the caller owns it; drop the lowercasing and keep only `trim`; and return `Vec<Cow<'_, str>>` so only the pieces that really changed pay for a `String`. Finally, write `fn parts(line: String) -> Vec<&str>` and read `E0106` — its third suggestion is one of the four fixes you just wrote.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:collect_into_a_vec_kata -->
*[`collect_into_a_vec_kata.rs`](examples/collect_into_a_vec_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the `Vec` that could not outlive its string.
//!
//!   rustc --edition 2024 collect_into_a_vec_kata.rs -o /tmp/civk && /tmp/civk
//!
//! The version that does not compile, kept as a comment so the file still does:
//!
//!     fn tidy(raw: &str) -> Vec<&str> {
//!         let lower = raw.to_lowercase();             // a NEW String, owned here
//!         lower.split(':').map(str::trim).collect()   // E0515
//!     }
//!
//! error[E0515]: cannot return value referencing local variable `lower`
//!   returns a value referencing data owned by the current function

use std::borrow::Cow;

/// Fix 1 — own the pieces. One allocation per piece, and the caller is free.
fn tidy_owned(raw: &str) -> Vec<String> {
    raw.to_lowercase()
        .split(':')
        .map(|p| p.trim().to_string())
        .collect()
}

/// Fix 2 — make the caller own the lowercase copy, so the borrow has something to point at.
fn tidy_borrowed(lower: &str) -> Vec<&str> {
    lower.split(':').map(str::trim).collect()
}

/// Fix 3 — do only the transformations that hand back a piece of the input.
fn tidy_trimmed(raw: &str) -> Vec<&str> {
    raw.split(':').map(str::trim).collect()
}

/// Fix 4 — borrow when you can, own when you must.
fn tidy_cow(raw: &str) -> Vec<Cow<'_, str>> {
    raw.split(':')
        .map(str::trim)
        .map(|p| {
            if p.bytes().any(|b| b.is_ascii_uppercase()) {
                Cow::Owned(p.to_lowercase())
            } else {
                Cow::Borrowed(p)
            }
        })
        .collect()
}

fn main() {
    let raw = "Cara : ada : Ben";

    println!("The four fixes, on {raw:?}");
    println!();

    let owned = tidy_owned(raw);
    println!("1. Vec<String>      {owned:?}");
    println!("   Allocates the lowercase copy, one String per piece, and the Vec.");
    println!("   Works for any transformation, and the result borrows nothing, so");
    println!("   it outlives everything it was made from.");
    println!();

    let lower = raw.to_lowercase();
    let borrowed = tidy_borrowed(&lower);
    println!("2. Vec<&str>, caller owns the lowercase copy");
    println!("   {borrowed:?}");
    println!("   Allocates one String at the call site, plus the Vec. The body of");
    println!("   the function did not change at all: what changed is WHERE the");
    println!("   lowercase copy lives, and now it outlives the call.");
    println!();

    let trimmed = tidy_trimmed(raw);
    println!("3. Vec<&str>, no new text at all");
    println!("   {trimmed:?}");
    println!("   Allocates the Vec and nothing else — no character was copied.");
    println!("   `trim` hands back a slice OF its argument, which is the only kind");
    println!("   of transformation a borrowing pipeline can do: `to_lowercase`,");
    println!("   `to_uppercase` and `replace` each build a new String, and that");
    println!("   String has to belong to somebody.");
    println!();

    let mixed = tidy_cow(raw);
    let borrowed_pieces = mixed.iter().filter(|c| matches!(c, Cow::Borrowed(_))).count();
    println!("4. Vec<Cow<str>>    {mixed:?}");
    println!("   {borrowed_pieces} of the {} pieces borrowed. Only a piece that really held a", mixed.len());
    println!("   capital letter paid for a String — which is what `Cow` is for when");
    println!("   most of the input is already the thing you wanted.");
    println!();

    println!("The rule underneath all four:");
    println!("  a Vec<&str> is a Vec of borrows, so it is only ever as long-lived");
    println!("  as the text it points into. Ask where that text lives, and the");
    println!("  choice between &str and String makes itself.");
}
```
<!-- /source -->

<!-- output:collect_into_a_vec_kata -->
*Verified output of [`collect_into_a_vec_kata.rs`](examples/collect_into_a_vec_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The four fixes, on "Cara : ada : Ben"

1. Vec<String>      ["cara", "ada", "ben"]
   Allocates the lowercase copy, one String per piece, and the Vec.
   Works for any transformation, and the result borrows nothing, so
   it outlives everything it was made from.

2. Vec<&str>, caller owns the lowercase copy
   ["cara", "ada", "ben"]
   Allocates one String at the call site, plus the Vec. The body of
   the function did not change at all: what changed is WHERE the
   lowercase copy lives, and now it outlives the call.

3. Vec<&str>, no new text at all
   ["Cara", "ada", "Ben"]
   Allocates the Vec and nothing else — no character was copied.
   `trim` hands back a slice OF its argument, which is the only kind
   of transformation a borrowing pipeline can do: `to_lowercase`,
   `to_uppercase` and `replace` each build a new String, and that
   String has to belong to somebody.

4. Vec<Cow<str>>    ["cara", "ada", "ben"]
   1 of the 3 pieces borrowed. Only a piece that really held a
   capital letter paid for a String — which is what `Cow` is for when
   most of the input is already the thing you wanted.

The rule underneath all four:
  a Vec<&str> is a Vec of borrows, so it is only ever as long-lived
  as the text it points into. Ask where that text lives, and the
  choice between &str and String makes itself.
```
<!-- /output -->

</details>

**Six questions, one `Vec`.** Take `"cara:ada:ben:ada"` and answer all six with the cheapest correct tool: how many names; the alphabetically first; the last one; the names in reverse; whether any name appears twice; and the names sorted. Before you write anything, predict which of the six cannot be answered without building a collection — then instrument the pipeline with a `Cell<usize>` counter and check how many pieces each one actually walked. Two of the six will surprise you.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:the_cheapest_answer_kata -->
*[`the_cheapest_answer_kata.rs`](examples/the_cheapest_answer_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: six questions about one line, and only one of them needs a `Vec`.
//!
//!   rustc --edition 2024 the_cheapest_answer_kata.rs -o /tmp/tca && /tmp/tca

use std::cell::Cell;
use std::collections::HashSet;

fn main() {
    let roster = "cara:ada:ben:ada";
    let seen = Cell::new(0usize);
    let count = |p| {
        seen.set(seen.get() + 1);
        p
    };
    let reset = || seen.set(0);

    println!("roster = {roster:?}   (4 names, one of them twice)");
    println!();

    // 1 -----------------------------------------------------------------
    reset();
    let n = roster.split(':').map(count).count();
    println!("1. How many names?            {n}");
    println!("   .count()          — {} pieces walked, nothing kept", seen.get());

    // 2 -----------------------------------------------------------------
    reset();
    let first = roster.split(':').map(count).min();
    println!("2. Alphabetically first?      {first:?}");
    println!("   .min()            — {} pieces walked, one &str kept", seen.get());

    // 3 -----------------------------------------------------------------
    reset();
    let last = roster.split(':').map(count).next_back();
    println!("3. The last name?             {last:?}");
    println!("   .next_back()      — {} piece walked: it read from the far end", seen.get());

    // 4 -----------------------------------------------------------------
    reset();
    print!("4. In reverse order?         ");
    for name in roster.split(':').map(count).rev() {
        print!(" {name}");
    }
    println!();
    println!("   .rev()            — no sort, no second pass, and no Vec: the");
    println!("                       names went straight to the screen. `Split` walks");
    println!("                       backwards on its own, so reversing is free.");
    println!("                       True for split(':') and NOT for split(\":\") —");
    println!("                       only the char searcher is double-ended.");

    // 5 -----------------------------------------------------------------
    reset();
    let mut met = HashSet::new();
    let repeated = roster.split(':').map(count).any(|name| !met.insert(name));
    let walked_here = seen.get();
    reset();
    let mut met_early = HashSet::new();
    let early = "cara:ada:ada:ben";
    let repeated_early = early.split(':').map(count).any(|name| !met_early.insert(name));
    println!("5. Any name twice?            {repeated}");
    println!("   HashSet + .any()  — {walked_here} pieces walked here, because the repeat");
    println!("                       IS the last name. On {early:?}");
    println!("                       the same line answers {repeated_early} after {} pieces and stops.", seen.get());
    println!("                       A collection, but a set — with a Vec you would");
    println!("                       be writing the inner loop yourself.");

    // 6 -----------------------------------------------------------------
    reset();
    let mut sorted: Vec<&str> = roster.split(':').map(count).collect();
    sorted.sort_unstable();
    println!("6. The names in order?        {sorted:?}");
    println!("   collect + sort    — {} pieces walked, all of them kept. THIS is", seen.get());
    println!("                       the question that needs the Vec: sorting cannot");
    println!("                       begin until the last name has arrived, which is");
    println!("                       why there is no `Iterator::sort` to reach for.");
    println!();

    println!("Four of the six answers built nothing at all; the fifth wanted a set");
    println!("rather than a Vec. The reflex to `collect` first and ask afterwards");
    println!("pays for a Vec on all six — and question 3 shows what that costs");
    println!("beyond the allocation: collecting reads the whole line to find a name");
    println!("the iterator hands back after a single step.");
}
```
<!-- /source -->

<!-- output:the_cheapest_answer_kata -->
*Verified output of [`the_cheapest_answer_kata.rs`](examples/the_cheapest_answer_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
roster = "cara:ada:ben:ada"   (4 names, one of them twice)

1. How many names?            4
   .count()          — 4 pieces walked, nothing kept
2. Alphabetically first?      Some("ada")
   .min()            — 4 pieces walked, one &str kept
3. The last name?             Some("ada")
   .next_back()      — 1 piece walked: it read from the far end
4. In reverse order?          ada ben ada cara
   .rev()            — no sort, no second pass, and no Vec: the
                       names went straight to the screen. `Split` walks
                       backwards on its own, so reversing is free.
                       True for split(':') and NOT for split(":") —
                       only the char searcher is double-ended.
5. Any name twice?            true
   HashSet + .any()  — 4 pieces walked here, because the repeat
                       IS the last name. On "cara:ada:ada:ben"
                       the same line answers true after 3 pieces and stops.
                       A collection, but a set — with a Vec you would
                       be writing the inner loop yourself.
6. The names in order?        ["ada", "ada", "ben", "cara"]
   collect + sort    — 4 pieces walked, all of them kept. THIS is
                       the question that needs the Vec: sorting cannot
                       begin until the last name has arrived, which is
                       why there is no `Iterator::sort` to reach for.

Four of the six answers built nothing at all; the fifth wanted a set
rather than a Vec. The reflex to `collect` first and ask afterwards
pays for a Vec on all six — and question 3 shows what that costs
beyond the allocation: collecting reads the whole line to find a name
the iterator hands back after a single step.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:collect_into_a_vec -->
*Verified output of [`collect_into_a_vec.rs`](examples/collect_into_a_vec.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One iterator, two ways to spend it
   collect  -> ["a", "b", "c"]
   for loop -> a b c
   Same three pieces either way. The first line built a Vec to
   hold them; the second never built a collection at all.

2. What the Vec buys — five things the iterator cannot do
   len          -> 4
   index [1]    -> ada
   sorted       -> ["ada", "ada", "ben", "cara"]
   join         -> "cara, ada, ben, ada"
   two passes   -> longest 4, 13 letters in all
   `sort` and `join` are SLICE methods, not iterator ones: neither
   can start until the last item has arrived, so neither can be an
   adapter. Two passes need the pieces to still be somewhere.

3. What it costs — and the questions that need no Vec
   collect().len() = 4   4 pieces built, 1 Vec allocated
   .count()        = 4   the same answer, no allocation
   .next()         = "cara"  1 closure call
   .find(b…)       = Some("ben")  3 closure calls, then it stopped
   .any(== ada)    = true   2 closure calls, then it stopped
   .position(ben)  = Some(2) 3 closure calls
   .max_by_key(len)= Some("cara")
   Only the first line allocated. A `collect` written to answer one
   of the other six is a Vec built, read once, and dropped.

4. The pieces are slices OF the original, not copies of it
   "cara:ada:ben:ada"
   byte offsets of the pieces -> [0, 5, 9, 13]
   Not one character was copied: the Vec holds four (pointer, len)
   pairs aimed into the string it came from. Cheap — and the reason
   a Vec<&str> can never outlive the string it was split from.

5. Borrowed or owned — one annotation apart
   Vec<&str>   -> ["cara", "ada", "ben", "ada"]  (1 allocation: the Vec)
   Vec<String> -> ["cara", "ada", "ben", "ada"]  (5: the Vec, and one per piece)
   the owned one outlives its source -> ["cara", "ada", "ben"]
   The same block returning Vec<&str> is E0515: the pieces would
   point into a String that was dropped one line earlier.

6. How many pieces? The answer is never zero
   "".split(':')                  -> len 1  [""]
   "a".split(':')                 -> len 1  ["a"]
   "a::b".split(':')              -> len 3  ["a", "", "b"]
   "a:b:".split(':')              -> len 3  ["a", "b", ""]
   "a:b:".split_terminator(':')   -> len 2  ["a", "b"]
   "".split_whitespace()          -> len 0  []
   `split` yields one more piece than there are separators, always,
   so the empty string gives one empty piece and the Vec is never
   empty. `is_empty()` on it answers a question you did not ask.

7. For a key=value line you want neither
   "port=8080"
      splitn(2, '=').collect() -> ["port", "8080"], and pieces.get(1) = Some("8080")
      split_once('=')          -> Some(("port", "8080"))
   "debug"
      splitn(2, '=').collect() -> ["debug"], and pieces.get(1) = None
      split_once('=')          -> None
   `pieces[1]` on the second line is a panic, and the type system
   said nothing: indexing a Vec is where a missing separator turns
   into a crash. `split_once` returns Option<(&str, &str)>, so the
   same mistake is a `None` the compiler makes you handle.
```
<!-- /output -->

---

## See also

- [`collect` and `FromIterator`](../collect_and_fromiterator/README.md) — once you have decided to collect, what else you can collect *into*, and the `Result<Vec<_>, _>` that flips a sequence of failures into one
- [Iterators are lazy](../iterators_are_lazy/README.md) — why nothing happened until `collect` asked
- [When a `for` loop beats a chain](../when_a_loop_beats_a_chain/README.md) — Option 2, taken seriously
- [`DoubleEndedIterator` and `ExactSizeIterator`](../double_ended_and_exact_size/README.md) — where the free `.rev()` comes from, and what `len()` would have needed
- [Inside a `Split`](../../14_Strings/inside_a_split/README.md) — the plan `split` hands back, field by field
- [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) — the choice `Vec<String>` versus `Vec<&str>` is made of
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md) — the fourth fix in the first kata
- [`Vec`](../../26_Collections/the_vec/README.md) — what you allocated, and what it costs to grow

## Sources

[`Iterator::collect` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect), [`str::split` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.split) and [`str::split_once` ↗](https://doc.rust-lang.org/std/primitive.str.html#method.split_once). The `DoubleEndedSearcher` bound that decides whether `.rev()` compiles is on [`std::str::pattern` ↗](https://doc.rust-lang.org/std/str/pattern/index.html), which is unstable as an API and observable as an error message. Every transcript on this page was produced on rustc 1.98.0; the counts and offsets come from the program below.

## Po polsku

Odruch, który przynosi się tu z Pythona, brzmi: „podziel łańcuch i mam listę”. W Ruscie `split` nie daje listy, tylko plan — leniwy `Split`, czyli kursor i wzorzec — a `collect()` jest decyzją, żeby ten plan **zmaterializować**: policzyć wszystko teraz, zaalokować raz i dostać coś, co da się zmierzyć, zaindeksować, posortować i przeczytać dwa razy. Słowo „materializacja” pasuje tu w tym samym sensie co przy zmaterializowanym widoku w bazie danych, i tak samo warto pytać, czy jest potrzebna. Kilka rzeczy faktycznie jej wymaga: `len()`, indeksowanie, `sort_unstable()`, `join(", ")`, drugi przebieg po tych samych danych i pole struktury, które musi trzymać wartość. Dwie z nich nie są nawet adapterami, tylko metodami **wycinka** (*slice*) — i to nie przypadek: sortowanie nie może się zacząć, zanim dotrze ostatni element, więc leniwie zrobić się nie da.

Reszta pytań wektora nie potrzebuje i to jest właściwa lekcja tej strony. `collect().len()` i `.count()` zwracają tę samą czwórkę, tylko pierwsze buduje wektor po to, żeby go zaraz wyrzucić. Gorzej wypadają pytania, które i tak kończą się wcześniej: na `"cara:ada:ben:ada"` `.find(…)` zatrzymuje się po trzech kawałkach, `.any(== "ada")` po dwóch, a `.next()` po jednym — zebranie wszystkiego najpierw oznacza przeczytanie całej linii po to, by odpowiedzieć na pytanie rozstrzygnięte po dwóch krokach. Darmowe jest nawet odwrócenie kolejności, bo `Split` jest `DoubleEndedIterator`em — i właśnie tam czeka pułapka, w którą wchodzi się z pythonowym odruchem: w Pythonie nie ma typu znakowego, więc pisze się `split(":")`. W Ruscie `':'` to znak, a `":"` to łańcuch znaków, i stoją za nimi dwa różne algorytmy wyszukiwania. Wersja z łańcuchem ciągnie za sobą pełną maszynerię Two-Way i **traci `.rev()`**: `E0277`, bo `DoubleEndedSearcher` nie jest zaimplementowane dla `StrSearcher`. Gdy separator naprawdę ma kilka znaków, a idziesz od końca, właściwą metodą jest `rsplit`.

Trzecią rzecz wystarczy zobaczyć raz, żeby zapamiętać ją na stałe: kawałki są **wycinkami oryginału**, a nie kopiami. Przesunięcia bajtowe `[0, 5, 9, 13]` i ani jeden skopiowany znak — `Vec<&str>` to cztery pary (wskaźnik, długość) wycelowane w łańcuch, z którego powstały. Dlatego dzielenie jest tanie i dlatego taki wektor **nigdy nie przeżyje** tekstu, w który wskazuje. Kompilator mówi to wprost — `E0515, cannot return value referencing local variable` — gdy w środku funkcji zrobisz `let lower = raw.to_lowercase()` i spróbujesz zwrócić `Vec<&str>` z kawałkami celującymi w `lower`. Reguła, która o tym decyduje, jest krótsza niż czasy życia (*lifetimes*): **pożyczający potok może wykonywać wyłącznie przekształcenia oddające kawałek wejścia.** `trim`, `strip_prefix` i `split` zwracają podwycinki, więc zostają w `Vec<&str>`; `to_lowercase`, `to_uppercase` i `replace` budują nowy `String`, a ten musi do kogoś należeć — stąd `Vec<String>`, albo `Cow`, kiedy zmienia się tylko część kawałków.

Na koniec dwie liczby, które zaskakują. Po pierwsze, ten wektor nigdy nie jest pusty: `split` oddaje o jeden kawałek więcej, niż było separatorów, bez wyjątku dla pustego wejścia — `"".split(':')` daje `[""]` o długości 1, więc `parts.is_empty()` zawsze odpowiada `false`, i to na pytanie, którego nie zadałeś. Sprawdzaj `parts[0].is_empty()` albo sięgnij po `split_whitespace`, które puste kawałki pomija z założenia (Python zachowuje się dokładnie tak samo, tylko rzadko trzeba to tam nazwać po imieniu). Po drugie, dla linii typu `klucz=wartość` nie chcesz ani zbierania, ani pętli: `splitn(2, '=').collect()` na wejściu bez separatora daje wektor jednoelementowy, a `pieces[1]` to panika, przed którą typy w niczym nie ostrzegły. `split_once('=')` zwraca `Option<(&str, &str)>`, więc ten sam zepsuty wiersz staje się `None`, które kompilator każe obsłużyć — i nie alokuje przy tym niczego.

**Szukaj po polsku:** materializacja iteratora · wycinek łańcucha znaków · `rust collect vs count` · `rust E0515 cannot return value referencing local variable` · `rust split_once`
