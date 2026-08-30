# `HashMap`

**Level:** 101 → 201 · for newcomers

**One line:** A key finds a value in roughly constant time, `entry` is the method the counting loop wants, and the iteration order is deliberately different on every run.

```rust
use std::collections::HashMap;

fn main() {
    let mut tally: HashMap<&str, u32> = HashMap::new();
    for name in ["Cara", "Ada", "Cara", "Ben", "Cara", "Ada"] {
        *tally.entry(name).or_insert(0) += 1;
    }
    println!("{:?}", tally.get("Cara"));   // Some(3)
}
```

`entry(k).or_insert(0)` returns a `&mut` to the value, inserting the default first if the key was absent. One hash lookup, where `if map.contains_key(k) { … } else { … }` costs two or three.

## Reading: `get` asks, `[]` asserts

| | on a missing key |
|---|---|
| `map.get(k)` | `None` |
| `map[k]` | panics |
| `map.get(k).copied().unwrap_or(0)` | `0` |

The same split as slice indexing. `[]` is a claim about your data; `.get` is a question about it. `map[k]` is a *place*, not a reference: `println!("{}", map[k])` borrows it and is fine for any `V`, but `let v: String = map[k];` is `E0507`, *"cannot move out of index of `HashMap<&str, String>`"*. That, plus the panic, is why `.get` is what you see in real code.

## `insert` returns what was there before

```rust
use std::collections::HashMap;

fn main() {
    let mut m: HashMap<&str, u32> = HashMap::new();
    println!("{:?}", m.insert("Ada", 1));   // None    — nothing was there
    println!("{:?}", m.insert("Ada", 9));   // Some(1) — the old value, handed back
}
```

`insert` **overwrites**, and that `Option` is the only warning you get. Discarding it is the usual way a duplicate key silently loses a row — see the practice below, where a tally written with `insert` reports 2 for a candidate who scored 11.

## Iteration order is not defined, and not stable between runs

std's `HashMap` seeds its hasher randomly per process, so two runs of the same program iterate in different orders. That is a defence against hash-flooding attacks, and it has one consequence you will meet immediately: **anything you print or compare must be sorted first**, or come from a `BTreeMap`, which is ordered by key by construction.

```rust
use std::collections::{BTreeMap, HashMap};

fn main() {
    let tally: HashMap<&str, u32> = [("Cara", 3), ("Ada", 2)].into_iter().collect();
    let ordered: BTreeMap<&str, u32> = tally.into_iter().collect();
    println!("{ordered:?}");   // {"Ada": 2, "Cara": 3}
}
```

Every example in this library that prints a map sorts it first, for exactly this reason — an answer key recorded from an unsorted `HashMap` would fail on its next run.

## What a key has to be

`HashMap<K, V>` requires `K: Eq + Hash`. Both are derivable, and the contract between them is the thing to know: **two keys that are equal must hash equally.** Hand-write one and not the other and lookups will miss entries that are provably in the map. `f64` is not a key, because `NaN != NaN` breaks `Eq`.

## The trap: a tie broken by the iteration order

```rust
use std::collections::HashMap;

fn main() {
    let tally: HashMap<&str, u32> = [("Ada", 3), ("Ben", 3)].into_iter().collect();
    let winner = tally.iter().max_by_key(|(name, count)| (**count, std::cmp::Reverse(**name)));
    println!("{winner:?}");   // Some(("Ada", 3))
}
```

Without that `Reverse(name)` tiebreak, `max_by_key` returns whichever equal entry the iterator reached last — which is a different candidate on a different run. A program that picks a winner out of a `HashMap` and has no explicit tiebreak is non-deterministic, and it will pass every test you write for it.

## If you are coming from another language

- **Python.** `dict`, with three differences that all matter. Python's dict has been **insertion-ordered** since 3.7 and Rust's `HashMap` is deliberately not ordered at all, so code that quietly relied on dict ordering has no direct translation — `BTreeMap` (sorted by key) or a `Vec<(K, V)>` (insertion order) is the port. `d[k]` raising `KeyError` is `map[k]` panicking, and `d.get(k)` returning `None` is `map.get(k)` returning `Option` — the same pair of methods, with Rust's version making you handle it. And `collections.defaultdict(int)` plus `d[k] += 1` is exactly `*map.entry(k).or_insert(0) += 1`; `dict.setdefault` is `or_insert` too. Keys need `__hash__`/`__eq__` to agree in Python as well, so that contract is already familiar; Rust just derives both for you.
- **ABAP.** A `HASHED TABLE OF ty WITH UNIQUE KEY k` is the same data structure with the same performance story, and `READ TABLE itab WITH TABLE KEY k = v` is `get` — `sy-subrc = 4` being `None`. Two transfers worth making explicit. `INSERT` into a hashed table with a duplicate key sets `sy-subrc = 4` and does **not** overwrite, where Rust's `insert` overwrites and hands the old value back; so the ABAP habit of checking `sy-subrc` after an insert has to become the habit of not ignoring the returned `Option`. And `COLLECT` is the closest thing ABAP has to `entry().or_insert()` — add-or-create in one statement — which is worth reaching for in exactly the same situations. Hashed tables have no defined order either, and the ABAP rule is the same one: `SORT` a copy, or use a `SORTED TABLE`, which is `BTreeMap`.
- **Java / C#.** `HashMap` / `Dictionary`, with `merge`/`computeIfAbsent` and `TryGetValue` filling the `entry` and `get` roles. Java's `HashMap` iteration order is unspecified but stable within a run; Rust's is randomised per process, which is stricter and catches the bug earlier.

---

## The verified output

<!-- output:the_hashmap -->
*Verified output of [`the_hashmap.rs`](examples/the_hashmap.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Counting, the way you will actually write it
   ["Cara", "Ada", "Cara", "Ben", "Cara", "Ada"]
   -> [("Ada", 2), ("Ben", 1), ("Cara", 3)]
   `entry(k).or_insert(0)` returns a &mut to the value, inserting
   the default first if the key was absent. One lookup, not the
   two that `if map.contains_key(k)` costs.

2. Reading: `get` asks, `[]` asserts
   tally.get("Ada")   = Some(2)
   tally.get("Nobody") = None
   tally["Ada"]        = 2
   tally["Nobody"]     -> panicked
   Same split as slice indexing: `[]` is a claim, `.get` is a question.
   tally.get("x").copied().unwrap_or(0) = 0

3. `insert` returns what was there before
   insert("Ada", 1) -> None   (nothing was there)
   insert("Ada", 9) -> Some(1)   (the old value, handed back)
   m is now [("Ada", 9)] — insert OVERWRITES. The return value is the only
   warning you get, and ignoring it is the usual way a duplicate key
   silently loses a row.

4. Iteration order is not defined, and not stable between runs
   sorted for printing:    [("Ada", 2), ("Ben", 1), ("Cara", 3)]
   the same in a BTreeMap: [("Ada", 2), ("Ben", 1), ("Cara", 3)]
   std's HashMap seeds its hasher randomly per process, so two runs
   of the same program iterate in different orders. That is a defence
   against hash-flooding, and it means any output you compare against
   must be sorted first — or must come from a BTreeMap, which is
   ordered by key by construction.

5. What a key has to be
   `HashMap<K, V>` needs K: Eq + Hash. Both are derivable, and both
   must agree: two keys that are `==` must hash the same, or lookups
   miss entries that are provably in the map.
   the max, tie broken by name: Some(("Cara", 3))
   `max_by_key` over a HashMap needs an explicit tiebreak for exactly
   the reason above: with no tiebreak, a tie is resolved by whichever
   equal entry the iterator happened to reach last.
```
<!-- /output -->

## Practice

**Four ways to count, and the two that are wrong.** Total a list of `(name, score)` ballots per name, four times: with `entry().or_insert(0)`, with `entry().or_default()`, with a bare `insert` in the loop, and with `contains_key` followed by `insert`.

Two of the four give the right answer. Of the two that do not, one is wrong and one is merely bad — say which is which, and what it costs. Then rank the results: a `HashMap` cannot be sorted in place, so say what you have to do instead and why.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:the_hashmap_kata -->
*[`the_hashmap_kata.rs`](examples/the_hashmap_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: four ways to count, and the two that are wrong.
//!
//!   rustc --edition 2024 the_hashmap_kata.rs -o /tmp/hmk && /tmp/hmk

use std::collections::HashMap;

const BALLOTS: [(&str, u32); 7] = [
    ("Cara", 5),
    ("Ada", 3),
    ("Cara", 4),
    ("Ben", 0),
    ("Cara", 2),
    ("Ada", 5),
    ("Dan", 1),
];

fn sorted<'a>(m: &'a HashMap<&'a str, u32>) -> Vec<(&'a str, u32)> {
    let mut rows: Vec<(&str, u32)> = m.iter().map(|(k, v)| (*k, *v)).collect();
    rows.sort();
    rows
}

fn main() {
    println!("1. entry().or_insert() — one lookup, and the right answer");
    let mut totals: HashMap<&str, u32> = HashMap::new();
    for (name, score) in BALLOTS {
        *totals.entry(name).or_insert(0) += score;
    }
    println!("   {:?}", sorted(&totals));

    println!();
    println!("2. or_default() — the same, with the type choosing the zero");
    let mut d: HashMap<&str, u32> = HashMap::new();
    for (name, score) in BALLOTS {
        *d.entry(name).or_default() += score;
    }
    println!("   {:?}   identical: {}", sorted(&d), sorted(&d) == sorted(&totals));

    println!();
    println!("3. The wrong one that compiles: insert in the loop");
    let mut lost: HashMap<&str, u32> = HashMap::new();
    for (name, score) in BALLOTS {
        lost.insert(name, score);
    }
    println!("   {:?}", sorted(&lost));
    println!("   Cara scored 5 + 4 + 2 = 11 and this says {}. `insert` overwrites,",
             lost["Cara"]);
    println!("   so every repeat key threw away the running total. Nothing warns:");
    println!("   the return value that would have told you is discarded.");

    println!();
    println!("4. The wrong one that is subtler: contains_key then insert");
    let mut two_pass: HashMap<&str, u32> = HashMap::new();
    for (name, score) in BALLOTS {
        if two_pass.contains_key(name) {
            let old = two_pass[name];
            two_pass.insert(name, old + score);
        } else {
            two_pass.insert(name, score);
        }
    }
    println!("   {:?}   correct: {}", sorted(&two_pass), sorted(&two_pass) == sorted(&totals));
    println!("   Right answer, three hash lookups per ballot instead of one, and");
    println!("   five lines where `entry` is one. This is the shape a Python or");
    println!("   Java habit produces, and it is why `entry` exists.");

    println!();
    println!("5. and_modify().or_insert() — when the first sighting is special");
    let mut seen: HashMap<&str, u32> = HashMap::new();
    for (name, _) in BALLOTS {
        seen.entry(name).and_modify(|n| *n += 1).or_insert(1);
    }
    println!("   ballots per voter: {:?}", sorted(&seen));
    println!("   `or_insert(1)` runs only for a name never seen before, so the");
    println!("   two branches can differ. With `or_insert(0)` and a `+= 1` after,");
    println!("   they cannot.");

    println!();
    println!("6. The answer the tally was for");
    let mut rows = sorted(&totals);
    rows.sort_by_key(|(name, total)| (std::cmp::Reverse(*total), *name));
    println!("   ranked: {rows:?}");
    println!("   Sorting a HashMap means leaving it: collect the pairs into a Vec");
    println!("   and sort that. A hash map has no order to sort in place.");
}
```
<!-- /source -->

<!-- output:the_hashmap_kata -->
*Verified output of [`the_hashmap_kata.rs`](examples/the_hashmap_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. entry().or_insert() — one lookup, and the right answer
   [("Ada", 8), ("Ben", 0), ("Cara", 11), ("Dan", 1)]

2. or_default() — the same, with the type choosing the zero
   [("Ada", 8), ("Ben", 0), ("Cara", 11), ("Dan", 1)]   identical: true

3. The wrong one that compiles: insert in the loop
   [("Ada", 5), ("Ben", 0), ("Cara", 2), ("Dan", 1)]
   Cara scored 5 + 4 + 2 = 11 and this says 2. `insert` overwrites,
   so every repeat key threw away the running total. Nothing warns:
   the return value that would have told you is discarded.

4. The wrong one that is subtler: contains_key then insert
   [("Ada", 8), ("Ben", 0), ("Cara", 11), ("Dan", 1)]   correct: true
   Right answer, three hash lookups per ballot instead of one, and
   five lines where `entry` is one. This is the shape a Python or
   Java habit produces, and it is why `entry` exists.

5. and_modify().or_insert() — when the first sighting is special
   ballots per voter: [("Ada", 2), ("Ben", 1), ("Cara", 3), ("Dan", 1)]
   `or_insert(1)` runs only for a name never seen before, so the
   two branches can differ. With `or_insert(0)` and a `+= 1` after,
   they cannot.

6. The answer the tally was for
   ranked: [("Cara", 11), ("Ada", 8), ("Dan", 1), ("Ben", 0)]
   Sorting a HashMap means leaving it: collect the pairs into a Vec
   and sort that. A hash map has no order to sort in place.
```
<!-- /output -->

</details>

---

## See also

- [`HashSet`](../the_hashset/README.md) — the same table with nothing on the right-hand side
- [`Vec`](../the_vec/README.md) — where you put the pairs when you need them in an order
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — what `get` hands back, and how to open it
- [Marker traits](../../12_Traits/marker_traits/README.md) — `Eq` is one, and this page is where its contract starts to matter
- [Comparing and sorting text](../../14_Strings/comparing_strings/README.md) — what `sort()` on `&str` keys actually compares

## Sources

[Std library types: HashMap ↗](https://doc.rust-lang.org/rust-by-example/std/hash.html) in Rust by Example, and [`std::collections::HashMap` ↗](https://doc.rust-lang.org/std/collections/struct.HashMap.html) — whose opening note about `RandomState` and HashDoS is the source for the ordering section above. The [`std::collections` ↗](https://doc.rust-lang.org/std/collections/index.html) module page has the *which one should I use* table.
