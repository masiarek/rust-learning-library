# A first `HashMap`

**Level:** 101 · for newcomers

**One line:** A `HashMap` keeps a value under a key and finds it again by that key, so a word count is one line — `*counts.entry(word).or_insert(0) += 1` — instead of two `Vec`s tied together by position. This page builds that line up one printed step at a time, then runs §5.5 of *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) beside it.

Read next: [`HashMap`](../the_hashmap/README.md) — `entry` in depth, the `Eq + Hash` contract, and a tie broken by iteration order. Beside this page: [every error](../hashmap_errors/README.md) · [lints](../hashmap_lints/README.md) · [resources](../hashmap_resources/README.md).

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    word_counts.insert("world", 2);
    println!("{:?}", word_counts.get("Hello"));         // Some(5)
    println!("{:?}", word_counts.get("Zig"));           // None
    println!("{}", word_counts.contains_key("world"));  // true
}
```

Three lines make a map and three ask it questions. The rest of this page is what each line does, in the order [`first_hashmap.rs`](examples/first_hashmap.rs) prints them; its output is near the bottom, and every number in the prose comes from it.

## 0. The problem: two `Vec`s tied by position

The book starts where most people start, with the words in one vector and the counts in another:

```rust
fn main() {
    let words = vec!["Hello", "world", "Rust", "Programming"];
    let counts = vec![5, 2, 15, 5];
    let i = words.iter().position(|w| *w == "Rust").unwrap();
    println!("{} {}", words[i], counts[i]);   // Rust 15
}
```

To find the count for `"Rust"` you search `words` for its position (2), then read `counts[2]`. Nothing but care keeps the two in step: reorder one and every count is wrong. One [`Vec`](../the_vec/README.md) of [tuples](../tuples/README.md), `vec![("Hello", 5), ("world", 2), …]`, keeps each word beside its count, but finding one is still a loop over every pair — the book's Listing 5.31 — and nothing stops a second `("Rust", 3)` from being pushed after the first. A map fixes both: one lookup by the word, and one slot per word.

## 1. Making one: `use`, `new`, and the type

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    let mut inferred = HashMap::new();
    inferred.insert("Hello", 5);
    println!("{} {}", word_counts.len(), inferred.len());   // 0 1
}
```

Three things on the first two lines:

- **The `use` line.** `HashMap` is not in the prelude, the set of names every file gets for free (`Vec`, `String` and `Option` are). Name it without `use std::collections::HashMap;` and the file does not compile — `E0425` and `E0433`, [entry 1 of the errors page](../hashmap_errors/README.md#1-no-use-line).
- **`HashMap<&str, u32>`** reads *keys are `&str`, values are `u32`*. `HashMap::new()` on its own does not know what will go in, so either annotate the type or let the first `insert` decide it, as `inferred` does. With neither, rustc stops with *type annotations needed* (`E0282`, [entry 2](../hashmap_errors/README.md#2-no-type-for-new)).
- **`mut`.** Adding to a map changes it, so the binding is `let mut`. Without it, `insert` is refused (`E0596`, [entry 13](../hashmap_errors/README.md#13-insert-without-mut)).

## 2. `insert`, and what it hands back

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    println!("{:?}", word_counts.insert("Hello", 5));          // None
    println!("{:?}", word_counts.insert("Programming", 5));    // None
    println!("{:?}", word_counts.insert("Programming", 15));   // Some(5)
    println!("{}", word_counts.len());                         // 2
}
```

`insert(key, value)` returns an `Option`: `None` when the key was new, `Some(old)` when it was already there — in which case the old value is **replaced**, not added to, and `len` does not grow. That is the book's *"keys are unique"* seen as a return value: after the third line there is one `"Programming"`, holding 15, and the 5 came back to you inside the `Some`. Dropping that `Some` on the floor is how a running total silently disappears, and [nothing warns](../hashmap_lints/README.md#what-no-lint-catches).

## 3. `get` asks: the answer comes in an `Option`

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Rust", 15);
    let rust: Option<&u32> = word_counts.get("Rust");
    println!("{rust:?} {:?}", word_counts.get("Zig"));   // Some(15) None
    match word_counts.get("Rust") {
        Some(count) => println!("Rust appears {count} times"),   // Rust appears 15 times
        None => println!("Rust is not in the map"),
    }
    println!("{}", word_counts.get("Zig").copied().unwrap_or(0));   // 0
}
```

`get` returns `Option<&u32>`: [`Some`](../../17_Option_and_Result/some_and_none/README.md) wrapping a *reference* to the value when the key is there, `None` when it is not. The program printed the type with `type_name_of_val`: `core::option::Option<&u32>`. It is not a number until you open it — with `match`, or with [`copied().unwrap_or(0)`](../../17_Option_and_Result/unwrap_or/README.md) when a missing key should count as zero. `let count: u32 = word_counts.get("Hello")` is `E0308`, *expected `u32`, found `Option<&u32>`*, the most common first error on this page ([entry 3](../hashmap_errors/README.md#3-get-used-as-the-number)).

## 4. `contains_key` answers yes or no

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Programming", 5);
    let has: bool = word_counts.contains_key("Programming");
    println!("{has} {}", word_counts.contains_key("programming"));   // true false
}
```

`contains_key` returns a plain `bool`. The book says it *"returns an Option wrapping the value inside Some if the key exists, and None otherwise"* — that describes `get`. `contains_key` is the yes/no question, and the program printed its type: `bool`. When you want the value, call `get` once rather than `contains_key` and then `get`, which looks the key up twice ([`unnecessary_get_then_check`](../hashmap_lints/README.md#unnecessary_get_then_check) points the other way round, at `get(k).is_some()`). The `false` on the second line is step 9.

## 5. `map[key]` asserts: the value, or a panic

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Rust", 15);
    println!("{}", word_counts["Rust"]);   // 15
    // println!("{}", word_counts["Zig"]);   // panics: no entry found for key
}
```

Square brackets give the value itself, no `Option` — and a missing key is a **panic** that ends the program:

```text title="rustc --edition 2024 index_missing_key.rs && ./index_missing_key — a run, not a compile error; rustc 1.98.0"
thread 'main' (41476231) panicked at index_missing_key.rs:6:31:
no entry found for key
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The number in brackets is the thread's id and changes every run. So `[]` is for a key you *know* is there and `get` for a key you are asking about — the same split as `v[i]` against `v.get(i)` on a `Vec`. `first_hashmap.rs` catches the panic with `std::panic::catch_unwind` so that it can go on printing; a real program would not.

## 6. `entry`: the slot for a key, filled or empty

This is the method the book introduces last and explains least, and it is the one the counting loop is built on. Take it in three steps.

**An `Entry` is the slot a key would occupy.** `word_counts.entry("Hello")` does one lookup and returns an [`Entry` ↗](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html), an enum with two variants: `Occupied` when the key is there, `Vacant` when it is not. Printed with `{:?}` by the program:

```text
entry("Hello") = Entry(OccupiedEntry { key: "Hello", value: 5, .. })
entry("Zig")   = Entry(VacantEntry("Zig"))
```

You can `match` on it like any other enum:

```rust
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    match word_counts.entry("Hello") {
        Entry::Occupied(slot) => println!("holding {}", slot.get()),   // holding 5
        Entry::Vacant(slot) => println!("{:?} is empty", slot.key()),
    }
}
```

**`or_insert(v)` fills the slot if it is empty, then hands back a `&mut` to whatever is in it.** For an `Occupied` slot the `v` is ignored; for a `Vacant` one it goes in. Either way you get a mutable reference to the value now in the map:

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    word_counts.insert("Hello", 5);
    let present: &mut u32 = word_counts.entry("Hello").or_insert(0);
    println!("{present}");   // 5   (already there: the 0 was not used)
    let absent: &mut u32 = word_counts.entry("Zig").or_insert(0);
    println!("{absent}");    // 0   (was absent: the 0 went in)
    *absent += 1;
    println!("{:?} {}", word_counts.get("Zig"), word_counts.len());   // Some(1) 2
}
```

The book stops at *"since the key 'Hello' already exists, the value will not be inserted"*, which is true, and leaves out the return value, which is the point.

**The counting loop** is those two facts on one line:

```rust
use std::collections::HashMap;

fn main() {
    let text = "the cat and the dog and the bird";
    let mut tally: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *tally.entry(word).or_insert(0) += 1;
    }
    let mut rows: Vec<(&str, u32)> = tally.iter().map(|(w, c)| (*w, *c)).collect();
    rows.sort();
    println!("{rows:?}");   // [("and", 2), ("bird", 1), ("cat", 1), ("dog", 1), ("the", 3)]
}
```

Read `*tally.entry(word).or_insert(0) += 1` from the inside out. `tally.entry(word)` finds the slot. `.or_insert(0)` makes sure it holds a number, writing 0 there if it was empty, and returns a `&mut u32`. The `*` in front reaches through that reference to the number itself, and `+= 1` adds one to it. Leave the `*` out and rustc refuses: `+=` cannot be applied to `&mut u32` (`E0368`, [entry 14](../hashmap_errors/README.md#14-the-missing-star)).

Worked, for `"the"`: the first time round, the slot is `Vacant`; `or_insert(0)` writes a 0 and returns a reference to it; `+= 1` makes it 1. The second time, the slot is `Occupied` with 1; the 0 is ignored; `+= 1` makes it 2. The third time, 3. Every other word is met once and ends at 1. One lookup per word and no `if` — where `if tally.contains_key(word) { … } else { … }` looks each word up two or three times and needs five lines.

## 7. `remove`: `Option` of what was there

```rust
use std::collections::HashMap;

fn main() {
    let mut tally: HashMap<&str, u32> = HashMap::new();
    tally.insert("cat", 1);
    println!("{:?}", tally.remove("cat"));   // Some(1)
    println!("{:?}", tally.remove("cat"));   // None
    println!("{}", tally.len());             // 0
}
```

The same shape as `insert`'s return: the value comes back if the key was there, and removing a key that is not there is `None`, not an error.

## 8. Iterating, and why the order is not the insertion order

```rust
use std::collections::HashMap;

fn main() {
    let mut tally: HashMap<&str, u32> = HashMap::new();
    for word in "the cat and the dog and the bird".split_whitespace() {
        *tally.entry(word).or_insert(0) += 1;
    }
    let mut pairs: Vec<(&str, u32)> = Vec::new();
    for (word, count) in &tally {
        pairs.push((*word, *count));
    }
    pairs.sort();
    println!("{pairs:?}");   // [("and", 2), ("bird", 1), ("cat", 1), ("dog", 1), ("the", 3)]
}
```

`for (word, count) in &tally` visits every pair once, as references (`&&str`, `&u32`), because the loop borrows the map rather than consuming it: loop over `tally` without the `&` and the map is gone afterwards (`E0382`, [entry 15](../hashmap_errors/README.md#15-the-map-is-gone-after-the-loop)). The order it visits them in is **not the order you inserted them, and not the same from one run to the next**. The book prints its map with `println!("{:?}", word_counts)`; that program, compiled once and run five times on this machine:

```text title="rustc --edition 2024 print_order.rs && ./print_order, five runs — rustc 1.98.0"
{"Programming": 5, "Hello": 5, "world": 2, "Rust": 15}
{"Rust": 15, "Programming": 5, "Hello": 5, "world": 2}
{"Rust": 15, "Programming": 5, "world": 2, "Hello": 5}
{"Rust": 15, "world": 2, "Programming": 5, "Hello": 5}
{"world": 2, "Rust": 15, "Hello": 5, "Programming": 5}
```

Five runs, five orders. std's `HashMap` seeds its hash function differently for every process, on purpose ([`RandomState` ↗](https://doc.rust-lang.org/std/hash/struct.RandomState.html); the reason is on [the next page](../the_hashmap/README.md)). So anything you print or compare must be sorted first — every example on this page collects the pairs into a `Vec` and sorts it — or must live in a [`BTreeMap`](../sorted_collections/README.md), which keeps its keys in order by construction. That the book's output *"will show a count of 15 for the key Programming"* is true; that it looks like the book's listing on your machine is luck.

## 9. Keys are case-sensitive

```rust
use std::collections::HashMap;

fn main() {
    let mut greetings: HashMap<&str, u32> = HashMap::new();
    greetings.insert("Hello", 5);
    println!("{:?} {:?}", greetings.get("Hello"), greetings.get("hello"));   // Some(5) None
    let mut folded: HashMap<String, u32> = HashMap::new();
    for word in ["Hello", "hello", "HELLO", "world"] {
        *folded.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    println!("{:?}", folded.get("hello"));   // Some(3)
}
```

`"Hello"` and `"hello"` are two keys. The book's loop in Listing 5.31 compared `word.to_lowercase() == target_word.to_lowercase()`, so it found `"Hello"` when asked for `"hello"`; the `HashMap` version that replaces it does nothing of the kind, and the book does not say so. If you want case folded, fold it once, at insert time, and look up with the folded form — which makes the key a `String`, the next step. What "lowercase" means for text beyond ASCII is [its own page](../../14_Strings/comparing_strings/README.md).

## 10. `&str` keys borrow the text; `String` keys own a copy

```rust
use std::collections::HashMap;

fn main() {
    let line = String::from("the cat and the dog and the bird");
    let mut owned: HashMap<String, u32> = HashMap::new();
    for word in line.split_whitespace() {
        *owned.entry(word.to_string()).or_insert(0) += 1;
    }
    drop(line);
    println!("{:?}", owned.get("the"));   // Some(3)
}
```

A `HashMap<&str, u32>` holds no text of its own: each key points into text that lives somewhere else — a literal in the program, or a `String` that must outlive the map. Build one from a `String` that is then dropped and rustc refuses (`E0597`, [entry 19](../hashmap_errors/README.md#19-str-keys-outliving-the-text)). A `HashMap<String, u32>` owns a copy of each word (`to_string()`), costs one allocation per key, and lives as long as it likes. Lookups read the same either way: `owned.get("the")` takes a `&str` although the keys are `String`s, because of [`Borrow`](../../12_Traits/borrow_trait/README.md); the reverse — a `HashMap<&str, _>` looked up with a `&String` — is `E0277` ([entry 9](../hashmap_errors/README.md#9-str-keys-looked-up-with-a-string)). It is the [`String` versus `&str`](../../14_Strings/string_vs_str/README.md) choice with the usual answer: borrow while the text is in scope, own when the map has to outlive it.

## What the book says, run

[`first_hashmap_claims.rs`](examples/first_hashmap_claims.rs) runs each sentence of §5.5 and §5.6 on rustc 1.98.0 with the book's own types (`HashMap<&str, u8>`, `HashSet<&str>`); its output is under the main program's below. The §5.6 rows are on [A first `HashSet`](../a_first_hashset/README.md#what-the-book-says-run), and every row is also on [the book's page](../../10_Resources/rust_the_practical_guide/README.md).

| §5.5 says | Run | Verdict |
|---|---|---|
| `HashMap` must be brought in with `use std::collections::HashMap;` | `no_use.rs` fails with `E0425` and `E0433` | holds |
| keys are unique: `insert("Programming", 15)` on a present key shows 15, once | returns `Some(5)`, `len` stays 4 | holds; the return value is the part worth knowing |
| `contains_key` returns an `Option` wrapping the value | returns `bool`, printed by `type_name_of_val` | **fails** — that is `get` |
| `entry` returns an `Entry` enum with variants `Occupied` and `Vacant` | `Entry(OccupiedEntry { key: "Hello", value: 5, .. })`, `Entry(VacantEntry("Zig"))` | holds |
| `or_insert(0)` on `"Hello"` inserts nothing, because the key exists | `len` 4 → 4, and the `&mut` it returns holds 5 | holds; the return value is not mentioned |
| searching for a key is extremely fast | not measured here; std documents the expected cost of a lookup as constant | not checked |
| `println!("{:?}", word_counts)` shows the counts | five runs, five orders | holds, in an order that differs every run |
| `HashMap<&str, u8>` for counts | `250u8.checked_add(10)` is `None`; `+= 1` past 255 panics in a debug build | a `u32` or `usize` is the usual count |

The dropped comparison is the one to carry away: Listing 5.31 lowercased both sides and the map does not, so `get("programming")` is `None` where the loop said `found = true`.

## The verified output

<!-- output:first_hashmap -->
*Verified output of [`first_hashmap.rs`](examples/first_hashmap.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
0. The problem: four words, four counts, two Vecs
   words[2] = "Rust", so counts[2] = 15
   The two Vecs are tied only by position: reorder one and every count is wrong.

   One Vec of pairs, searched with the book's loop:
   looking for "hello" with to_lowercase on both sides: found = true
   position of "Rust" = Some(2), count = 15
   Nothing stops a second ("Rust", 3) pair from being pushed after it.

1. A HashMap: HashMap::new(), then insert
   insert("Hello", 5) returned None   (nothing was there before)
   len = 4   sorted: [("Hello", 5), ("Programming", 5), ("Rust", 15), ("world", 2)]

2. Keys are unique: a second insert of the same key replaces the value
   insert("Programming", 15) returned Some(5)   (the value that was replaced)
   len is still 4   sorted: [("Hello", 5), ("Programming", 15), ("Rust", 15), ("world", 2)]

3. get asks: Option<&u32>
   get("Rust") = Some(15)   get("Zig") = None
   the type of what get returns: core::option::Option<&u32>
   Rust appears 15 times
   get("Zig").copied().unwrap_or(0) = 0

4. contains_key answers yes or no: bool
   contains_key("Programming") = true   type: bool
   contains_key("programming") = false   (keys are case-sensitive)

5. Indexing asserts: map[key] is the value, or a panic
   word_counts["Rust"] = 15
   word_counts["Zig"]  -> panicked

6. entry: the slot for a key, whether or not it is filled
   entry("Hello") = Entry(OccupiedEntry { key: "Hello", value: 5, .. })
   entry("Zig")   = Entry(VacantEntry("Zig"))
   "Hello" is Occupied, holding 5
   entry("Hello").or_insert(0) -> &mut holding 5   (already there: the 0 was not used)
   entry("Zig").or_insert(0)   -> &mut holding 0   (was absent: the 0 went in)
   after *absent += 1: get("Zig") = Some(1)   len = 5

   The counting loop, on a sentence:
   "the cat and the dog and the bird"
   -> [("and", 2), ("bird", 1), ("cat", 1), ("dog", 1), ("the", 3)]
   "the" is met three times. The first time the slot is Vacant, so
   or_insert(0) puts a 0 there and hands back a &mut to it; += 1 makes it 1.
   The next two times the slot is Occupied, the 0 is ignored, and += 1
   makes it 2, then 3.

7. remove: Option of what was there
   remove("cat") = Some(1)   remove("cat") again = None   len = 4

8. Iterating: every pair, in no particular order
   sorted after the loop: [("and", 2), ("bird", 1), ("dog", 1), ("the", 3)]
   the same in a BTreeMap: {"and": 2, "bird": 1, "dog": 1, "the": 3}
   The loop's own order is not the insertion order and changes from run
   to run, which is why this program never prints a HashMap directly.

9. Case matters: "Hello" and "hello" are two keys
   get("Hello") = Some(5)   get("hello") = None
   lowercased at insert time: [("hello", 3), ("world", 1)]

10. &str keys borrow the text; String keys own a copy of it
   HashMap<&str, u32>: 4 keys pointing into text that must outlive the map
   HashMap<String, u32> after the line is dropped: get("the") = Some(3)
   A String key is looked up with a plain &str, so get("the") reads the same on both.
```
<!-- /output -->

<!-- output:first_hashmap_claims -->
*Verified output of [`first_hashmap_claims.rs`](examples/first_hashmap_claims.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
§5.5 HashMaps, with the book's map: HashMap<&str, u8>

1. "keys are unique": insert("Programming", 15) on a key already there
   returned Some(5)   len 4   sorted [("Hello", 5), ("Programming", 15), ("Rust", 15), ("world", 2)]
   HOLDS: the value is 15 and "Programming" is there once. The book prints
   the map itself, whose order is not fixed, so this program sorts first.

2. "contains_key returns an Option wrapping the value in Some"
   contains_key("Programming") = true   type: bool
   FAILS: it is a bool. The method that returns the value in an Option is get:
   get("Programming") = Some(15)   type: core::option::Option<&u8>

3. "entry returns an instance of the Entry enum, with two variants, Occupied and Vacant"
   entry("Hello") = Entry(OccupiedEntry { key: "Hello", value: 5, .. })
   entry("Zig")   = Entry(VacantEntry("Zig"))
   HOLDS.

4. "or_insert inserts a value if the key does not exist; Hello exists, so nothing is inserted"
   entry("Hello").or_insert(0): the &mut holds 5   len 4 -> 4
   HOLDS, and the book stops there: or_insert also returns a &mut to the value,
   which is what the counting loop needs.

5. The book's earlier loop compared lowercased words; the map does not
   get("programming") = None   get("Programming") = Some(15)
   Not a claim the book makes, but a change it does not mention: keys are
   case-sensitive, so "programming" is a different key from "Programming".

6. The book's value type is u8
   250u8.checked_add(10) = None: a u8 count cannot pass 255, and += 1 past it
   panics in a debug build. A count is usually a u32 or a usize.

§5.6 HashSets, with the book's set: HashSet<&str>

7. "insert a duplicate entry: Hello appears only once"
   insert("Hello") = false   len 4   sorted ["Hello", "Programming", "Rust", "world"]
   HOLDS, and the book does not say that insert returns a bool: false here,
   true for the four that went in.

8. "contains returns a bool"
   contains("world") = true   type: bool
   HOLDS.

9. "take removes the item and returns it if it exists"
   Removed: world
   take("world") again = None   len 3
   HOLDS: Some("world") the first time, None once it is gone.
```
<!-- /output -->

## Practice

**Count a sentence, then look three words up.** Count the words of `"the quick brown fox jumps over the lazy dog the end"` with `entry`, print the counts sorted by word and then by count, and look up `"the"`, `"fox"` and `"cat"` three ways each: `contains_key`, `get`, and `get(…).copied().unwrap_or(0)`. Say where the 0 for `"cat"` comes from.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:first_hashmap_kata -->
*[`first_hashmap_kata.rs`](examples/first_hashmap_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: count the words of a sentence with entry, then look three of them up.
//!
//!   rustc --edition 2024 first_hashmap_kata.rs -o /tmp/fhmk && /tmp/fhmk

use std::collections::HashMap;

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog the end";

    println!("1. Count every word");
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in sentence.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    let mut rows: Vec<(&str, u32)> = counts.iter().map(|(word, count)| (*word, *count)).collect();
    rows.sort();
    println!("   {} words, {} distinct", sentence.split_whitespace().count(), counts.len());
    println!("   by word:  {rows:?}");
    rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("   by count: {rows:?}");

    println!();
    println!("2. Look three words up");
    for word in ["the", "fox", "cat"] {
        let present = counts.contains_key(word);
        let count = format!("{:?}", counts.get(word));
        let number = counts.get(word).copied().unwrap_or(0);
        println!("   {word:>3}: contains_key = {present:<5}  get = {count:<8}  count = {number}");
    }

    println!();
    println!("3. What the three lookups say");
    println!("   contains_key answers yes or no; get hands back the count in a Some, or a");
    println!("   None for a word that was never counted. The 0 for \"cat\" is not in the");
    println!("   map: unwrap_or(0) supplied it.");
}
```
<!-- /source -->

<!-- output:first_hashmap_kata -->
*Verified output of [`first_hashmap_kata.rs`](examples/first_hashmap_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Count every word
   11 words, 9 distinct
   by word:  [("brown", 1), ("dog", 1), ("end", 1), ("fox", 1), ("jumps", 1), ("lazy", 1), ("over", 1), ("quick", 1), ("the", 3)]
   by count: [("the", 3), ("brown", 1), ("dog", 1), ("end", 1), ("fox", 1), ("jumps", 1), ("lazy", 1), ("over", 1), ("quick", 1)]

2. Look three words up
   the: contains_key = true   get = Some(3)   count = 3
   fox: contains_key = true   get = Some(1)   count = 1
   cat: contains_key = false  get = None      count = 0

3. What the three lookups say
   contains_key answers yes or no; get hands back the count in a Some, or a
   None for a word that was never counted. The 0 for "cat" is not in the
   map: unwrap_or(0) supplied it.
```
<!-- /output -->

</details>

## If you are coming from another language

- **Python.** A `HashMap` is a [`dict` ↗](https://docs.python.org/3/tutorial/datastructures.html#dictionaries), method for method: `d[k] = v` is `insert` (Python says nothing about the old value; Rust hands it back), `d.get(k)` is `get` — except that Rust's `None` is a variant you must open with `match` or `unwrap_or`, not a value you can compare to 5 by accident — `k in d` is `contains_key`, `d[k]` raising `KeyError` is `map[k]` panicking, and `del d[k]` is `remove`. The counting loop is [`collections.Counter` ↗](https://docs.python.org/3/library/collections.html#collections.Counter), or a [`defaultdict(int)` ↗](https://docs.python.org/3/library/collections.html#collections.defaultdict) with `d[w] += 1`; `d.setdefault(k, 0)` is the nearest thing to `entry(k).or_insert(0)`. Two things do not carry over. A `dict` keeps insertion order (since 3.7) and a `HashMap` keeps none, so a loop that relied on the order needs a `BTreeMap` or a sorted `Vec` here. And a Python key can be a `str` or a `float`; a Rust key needs `Eq + Hash`, which `f64` does not have, and a `&str` key borrows text the map does not own. The [Python library ↗](https://masiarek.github.io/python-learning-library/) has no `dict` page yet, so the links above are the official docs.
- **ABAP.** A `HashMap` is a [hashed internal table ↗](https://masiarek.github.io/abap-learning-library/03_Topics/internal_tables/index.html), `TYPE HASHED TABLE OF … WITH UNIQUE KEY word`, with the same promises: one row per key, a lookup that does not scan, and no order to rely on. [`READ TABLE … WITH TABLE KEY` ↗](https://masiarek.github.io/abap-learning-library/02_Keywords/read_table/index.html) is `get`, with `sy-subrc = 4` standing in for `None`, and a table expression `itab[ word = 'Rust' ]` that raises `CX_SY_ITAB_LINE_NOT_FOUND` is `map[k]` panicking. The counting loop is [`COLLECT` ↗](https://masiarek.github.io/abap-learning-library/02_Keywords/collect/index.html): add the numeric fields to the row with the same key, or append a new row. One difference to hold onto: `INSERT` into a hashed table whose key is already present *fails* (`sy-subrc = 4`) and keeps the old row; Rust's `insert` replaces it and returns the old value.
- **Java.** `java.util.HashMap<K, V>` ([Javadoc ↗](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/HashMap.html)): `put` returns the previous value or `null` where Rust returns `Some(old)` or `None`; `get` returns `null` for a missing key where Rust returns `None`, which the compiler makes you handle; `containsKey` is `contains_key`; `merge(w, 1, Integer::sum)` is the counting loop. Java's iteration order is unspecified but stays the same within a run; Rust's changes between runs, which surfaces a dependency on it at once.

## See also

- [`HashMap`](../the_hashmap/README.md) — read next: `entry` and `and_modify`, the `Eq + Hash` contract, and the tie that iteration order breaks
- [A first `HashSet`](../a_first_hashset/README.md) — the same table with nothing on the right-hand side, at the same pace
- [Every `HashMap` and `HashSet` error, and its fix](../hashmap_errors/README.md) — the twenty refusals a first map meets, in rustc's words
- [Lints around `HashMap` and `HashSet`](../hashmap_lints/README.md) — what clippy says about `contains_key` then `insert`, and what nothing says
- [`HashMap` and `HashSet`: resources](../hashmap_resources/README.md) — the reading list
- [`BTreeMap` and `BTreeSet`](../sorted_collections/README.md) — the map that prints in order
- [`Vec`](../the_vec/README.md) — where the pairs go when you need them in an order
- [Tuples](../tuples/README.md) — the `(&str, u32)` pairs of step 0
- [`Some` and `None`: reading an `Option`](../../17_Option_and_Result/some_and_none/README.md) — what `get` hands back
- [`unwrap_or`: the default you already have](../../17_Option_and_Result/unwrap_or/README.md) — `get(k).copied().unwrap_or(0)`
- [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) — the key-type choice of step 10
- [`Borrow`: look up an owned key with a borrowed one](../../12_Traits/borrow_trait/README.md) — why `get("the")` works on `String` keys
- [*Rust: The Practical Guide*, run](../../10_Resources/rust_the_practical_guide/README.md) — the book's home here, with every checked claim
- [`std::collections::HashMap` ↗](https://doc.rust-lang.org/std/collections/struct.HashMap.html) — every method, and the opening note on `RandomState`
- [*Rust: The Practical Guide*, chapter 5 exercises, run](../../10_Resources/rust_the_practical_guide/ch5_katas/README.md) — the book's student register, a first `HashMap` keyed by ID with a refused duplicate

## Po polsku

`HashMap` to **tablica mieszająca** (*hash map*), po pythonowemu „słownik”: przechowuje wartość pod kluczem i odnajduje ją po tym kluczu jednym wyszukaniem, zamiast dwóch wektorów związanych tylko pozycją. Trzeba ją zaimportować (`use std::collections::HashMap;`), bo nie ma jej w preludium (*prelude*). `insert` zwraca `Option`: `None` dla nowego klucza, `Some(stara_wartość)` gdy klucz już był — a wtedy wartość została **zastąpiona**, nie dodana; to jest „klucze są unikalne” z książki. `get` odpowiada `Option<&V>`, czyli opcją z referencją, którą trzeba otworzyć przez `match` albo `unwrap_or`; `contains_key` zwraca zwykły `bool`, a nie `Option`, jak twierdzi książka; `map[klucz]` daje samą wartość albo panikuje (*panic*), gdy klucza nie ma. Najważniejsza metoda to `entry` (*entry API*): `entry(k)` to miejsce na klucz, zajęte (`Occupied`) albo wolne (`Vacant`), a `or_insert(0)` wypełnia wolne miejsce zerem i oddaje `&mut` do wartości — stąd pętla licząca `*tally.entry(word).or_insert(0) += 1`, jedno wyszukanie na słowo. Kolejność iteracji nie jest kolejnością wstawiania i zmienia się przy każdym uruchomieniu (pięć uruchomień, pięć kolejności), więc przed wydrukiem trzeba posortować albo wziąć `BTreeMap`. Klucze rozróżniają wielkość liter: `"Hello"` i `"hello"` to dwa klucze. Klucz `&str` tylko pożycza tekst, który musi żyć dłużej niż mapa; klucz `String` ma własną kopię.

**Szukaj po polsku:** tablica mieszająca w Ruscie · słownik w Ruscie · `rust HashMap insert get contains_key` · `rust HashMap entry or_insert` · `rust HashMap kolejność iteracji`
