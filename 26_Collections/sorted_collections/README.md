# `BTreeMap` and `BTreeSet`

**Level:** 101 → 201 · working knowledge

**One line:** The sorted pair: keys live in order, so iterating one is already sorted and collecting into one is a sort you did not write — paid for with [`Ord`](../../12_Traits/comparison_traits/README.md) instead of `Hash`, and with lookups that walk a tree instead of jumping straight there.

```rust
use std::collections::BTreeMap;

fn main() {
    let mut tally: BTreeMap<&str, u32> = BTreeMap::new();
    for b in ["Cara", "Ada", "Ben", "Cara"] {
        *tally.entry(b).or_insert(0) += 1;
    }
    println!("{tally:?}");   // {"Ada": 1, "Ben": 1, "Cara": 2}
}
```

Swap `BTreeMap` for [`HashMap`](../the_hashmap/README.md) and every line still compiles — `entry`, `or_insert`, `get`, `insert`, `len` are the same names — but the print comes out in a different order every run.

## Collecting into one is the sort

```rust
use std::collections::{BTreeMap, HashMap};

fn main() {
    let scores = [("Cara", 2), ("Ada", 5), ("Ben", 4)];

    let sorted: BTreeMap<&str, u32> = scores.into_iter().collect();

    let hashed: HashMap<&str, u32> = scores.into_iter().collect();
    let mut pairs: Vec<(&str, u32)> = hashed.into_iter().collect();
    pairs.sort();

    println!("{sorted:?}");   // {"Ada": 5, "Ben": 4, "Cara": 2}
    println!("{pairs:?}");    // [("Ada", 5), ("Ben", 4), ("Cara", 2)]
}
```

Both lines print the same three pairs in the same order. The second one had to leave the map, build a `Vec` and sort it to get there; the first was in order the whole time. That is the case for reaching past `HashMap`: not speed, but that the answer is **already ordered at every point**, including halfway through building it.

## What the ordering buys, and what it costs

| | [`HashMap`](../the_hashmap/README.md) / [`HashSet`](../the_hashset/README.md) | `BTreeMap` / `BTreeSet` |
|---|---|---|
| the key must be | `Hash + Eq` | [`Ord` ↗](https://doc.rust-lang.org/std/cmp/trait.Ord.html) |
| lookup | O(1) | O(log *n*) |
| iteration order | arbitrary, and different each run | sorted by key, always |
| first / last | — | [`first_key_value` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.first_key_value) · [`last_key_value` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.last_key_value) |
| "everything between two keys" | — | [`range` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.range) |

The bottom three rows are not slower on a `HashMap` — they are **unaskable**. There is no order for a first, a last or a range to mean anything against.

```rust
use std::collections::BTreeMap;

fn main() {
    let tally: BTreeMap<&str, u32> =
        [("Ada", 3), ("Ben", 2), ("Cara", 3), ("Dan", 1)].into_iter().collect();
    let window: Vec<&str> = tally.range("B".."D").map(|(k, _)| *k).collect();
    println!("{window:?}");   // ["Ben", "Cara"]
}
```

`range` takes the same `..`, `..=` and `a..b` forms a slice does, and yields borrowed pairs in order. Note what the bound is being compared against: `"D"` is a **string**, so `"Dan"` sorts after it and stays out — and `range("B"..="D")` does not change that, because the `=` includes the key `"D"` itself, which nobody has.

## The trap: it sorts by the key, and you wanted the value sorted

This is the reason most reaches for `BTreeMap` disappoint. A vote tally keyed on the candidate is *alphabetical*, and a leaderboard is what you were after:

```rust
use std::collections::BTreeMap;

fn main() {
    let tally: BTreeMap<&str, u32> =
        [("Ada", 3), ("Ben", 2), ("Cara", 3), ("Dan", 1)].into_iter().collect();

    let mut board: Vec<(&str, u32)> = tally.iter().map(|(k, v)| (*k, *v)).collect();
    board.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("{board:?}");   // [("Ada", 3), ("Cara", 3), ("Ben", 2), ("Dan", 1)]
}
```

The other way is to put the count **into** the key, where the container can see it. A tuple is `Ord` when its parts are, compared left to right, so a `BTreeSet` of `(Reverse(count), name)` pairs is a leaderboard with no sort call in it anywhere — [`Reverse` ↗](https://doc.rust-lang.org/std/cmp/struct.Reverse.html) being the std wrapper that flips a single comparison. The practice below builds the leaderboard both ways and checks they agree.

## `BTreeSet` is `BTreeMap<T, ()>`

Same relationship as `HashSet` to `HashMap`, and the same consequence: collecting into one **sorts and deduplicates in one step**. Its `union`, `intersection` and `difference` yield their items in order too, so a `Vec` built from one of those needs no sort afterwards.

## The key you cannot use

`f64` is not `Ord`. `NaN` compares `false` against everything including itself, which leaves floats with `PartialOrd` only — so a float-keyed map is refused. The refusal is later than you would expect:

```rust
use std::collections::BTreeMap;

fn main() {
    let scores: BTreeMap<f64, &str> = BTreeMap::new();   // fine — no `Ord` needed to exist
    println!("{}", scores.len());                        // fine — nor to be measured
    // scores.insert(9.1, "Ada");                        // E0277 — this is where it bites
}
```

`BTreeMap::new` and `len` carry no `K: Ord` bound, so the declaration compiles and runs. The error arrives at the first call that has to *compare* something:

```text title="Abridged — real rustc 1.98.0 output, without the note locating the bound in std"
error[E0277]: the trait bound `f64: Ord` is not satisfied
    --> float_key.rs:5:12
     |
   5 |     scores.insert(9.1, "Ada");
     |            ^^^^^^ the trait `Ord` is not implemented for `f64`
     |
     = help: the following other types implement trait `Ord`:
               i128
               i16
               i32
               i64
               i8
               isize
               u128
               u16
             and 4 others
```

Scale to an integer — basis points, cents, milliseconds — and the ordering survives intact. The alternative is a wrapper type that promises `Ord` and rejects `NaN` when it is built.

## If you are coming from another language

- **Python.** There is no sorted dict in the standard library, which is why the habit transfers badly: a Python programmer reaches for `sorted(d.items())` at the point of *use*, and Rust's answer is to have picked the container at the point of *construction*. `dict` has been insertion-ordered since 3.7 — that is a third thing again, and neither Rust map gives it to you (keep a `Vec` alongside, as the [`HashSet`](../the_hashset/README.md) page does). The closest analogues are outside the language core: `sortedcontainers`' `SortedDict` on PyPI, and `bisect` over a list you keep sorted by hand — which is what `range` replaces, without the "did I remember to re-sort after inserting" question. `collections.Counter(...).most_common()` is the leaderboard trap above, already solved for you; Rust makes you say which order you meant.
- **ABAP.** This is the closest correspondence in the language. `TYPES ty TYPE SORTED TABLE OF ... WITH UNIQUE KEY name` **is** a `BTreeMap`, and `HASHED TABLE ... WITH UNIQUE KEY` is a `HashMap`; both keep entries by a declared key, both refuse duplicates, and the choice is made in the type rather than at the read. `READ TABLE it WITH TABLE KEY name = 'Ada'` is `map.get("Ada")`, and `LOOP AT it WHERE name BETWEEN 'B' AND 'D'` is `range` — with the same performance story, since a sorted table binary-searches the key and a `STANDARD TABLE` does not. Two differences worth holding. ABAP's sorted table can have a **non-unique** key and hold several rows per value; a `BTreeMap` holds exactly one, so the ABAP habit of appending duplicates and looping over them becomes `BTreeMap<K, Vec<V>>` here. And ABAP lets you declare secondary keys on one table, so the same internal table can be read sorted *and* hashed; Rust makes you keep two containers, or one and an index.
- **C++.** `std::map` and `std::set` are this pair almost exactly — ordered, `O(log n)`, iterable in key order — and `std::unordered_map` / `std::unordered_set` are `HashMap` / `HashSet`. `lower_bound`/`upper_bound` are what `range` wraps into one call. The difference that bites: `std::map::operator[]` **inserts a default** when the key is missing, so a read can silently grow the map; Rust has no `operator[]` that does that — `map[k]` panics on a missing key and `entry(k).or_insert(0)` is the explicit form of the C++ behaviour. C++'s comparator is a type parameter (`std::map<K, V, Cmp>`), which is how a C++ programmer keys on a float; Rust's equivalent is a newtype that implements `Ord`.
- **Java.** `TreeMap` / `TreeSet` against `HashMap` / `HashSet`, with `SortedMap` as the interface. `subMap(from, to)`, `firstKey()` and `lastKey()` map onto `range`, `first_key_value` and `last_key_value`. Java's `Comparable` is `Ord` and `Comparator` is the argument to `sort_by`; the difference is that Java lets a `TreeMap` take `Double` keys and simply behaves oddly around `NaN` at run time, where Rust refuses the type at compile time.

---

## The verified output

<!-- output:sorted_collections -->
*Verified output of [`sorted_collections.rs`](examples/sorted_collections.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Collecting into a BTreeMap is a sort you did not write
   BTreeMap  {"Ada": 3, "Ben": 2, "Cara": 3, "Dan": 1}
   HashMap   [("Ada", 3), ("Ben", 2), ("Cara", 3), ("Dan", 1)]
   The same four counts. The second line needed a Vec and a sort to be
   printable in a fixed order at all; the first was already in one.

2. Ordering buys three questions a HashMap cannot answer
   first_key_value()   Some(("Ada", 3))
   last_key_value()    Some(("Dan", 1))
   range("B".."D")     [("Ben", 2), ("Cara", 3)]
   A HashMap has no first, no last and no range: there is no order for
   those questions to be asked against.

3. The trap: sorted by KEY, and what you wanted sorted is the value
   Ada 3
   Ben 2
   Cara 3
   Dan 1
   That is alphabetical, not a leaderboard. Votes-descending still
   leaves the map:
   [("Ada", 3), ("Cara", 3), ("Ben", 2), ("Dan", 1)]
   Ada and Cara tie at 3, and `then` breaks it by name — so the order is
   total and the answer is the same on every run.

4. BTreeSet is BTreeMap<T, ()>: collecting into one sorts and dedups
   {"Ada", "Ben", "Cara", "Dan"}
   eligible but never voted  ["Eve"]
   The set operations yield their items in order too, so that Vec
   needed no sort either.

5. The price is Ord, not Hash
   keyed on an integer score  {55: "Ben", 72: "Cara", 91: "Ada"}
   Integers, char, &str, String, and tuples and Vecs of those are all
   Ord. f64 is not — NaN leaves it PartialOrd only — so a BTreeMap keyed
   on a float does not compile. Scale to an integer, as above.
```
<!-- /output -->

## Practice

**Two orders from one tally, and the key you cannot use.** Count a list of ballots into a `BTreeMap`, then produce the roll alphabetically and the leaderboard by votes descending — and say which of the two the container gave you for free. Build the leaderboard twice: once by leaving the map for a `Vec`, and once by putting the count into the key so that nothing sorts. Both have ties; make the tie-break explicit rather than inherited, and say what would change if you swapped `sort_by` for `sort_unstable_by`.

Then ask for the candidates whose names fall in `"B".."D"`, and check whether `..=` moves the boundary. Finally, key the same data on each candidate's **share of the vote**: say why `BTreeMap<f64, _>` will not compile, scale it to an integer instead — and then count the entries that came out, because that number is the point of the exercise.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:sorted_collections_kata -->
*[`sorted_collections_kata.rs`](examples/sorted_collections_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: two orders from one tally, and the key you cannot use.
//!
//!   rustc --edition 2024 sorted_collections_kata.rs -o /tmp/sck && /tmp/sck

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

const BALLOTS: [&str; 11] = [
    "Cara", "Ada", "Ben", "Cara", "Dan", "Ada", "Cara", "Ben", "Ada", "Eve", "Dan",
];

/// The tally. Alphabetical order is a property of the container, not of this code.
fn tally() -> BTreeMap<&'static str, u32> {
    let mut t = BTreeMap::new();
    for b in BALLOTS {
        *t.entry(b).or_insert(0) += 1;
    }
    t
}

fn main() {
    let t = tally();

    println!("1. The roll, alphabetical — nothing here sorts");
    for (name, votes) in &t {
        println!("   {name} {votes}");
    }

    println!();
    println!("2. The leaderboard, by leaving the map");
    let mut board: Vec<(&str, u32)> = t.iter().map(|(k, v)| (*k, *v)).collect();
    board.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for (name, votes) in &board {
        println!("   {name} {votes}");
    }
    println!("   Two ties — Ada/Cara at 3 and Ben/Dan at 2 — and `then` settles both");
    println!("   by name. Without it `sort_by` is still *stable*, so the order would");
    println!("   be the map's alphabetical one; but the rule would be implicit, and a");
    println!("   switch to sort_unstable_by would silently change the answer.");

    println!();
    println!("3. The leaderboard, by making the count part of the key");
    let ranked: BTreeSet<(Reverse<u32>, &str)> =
        t.iter().map(|(k, v)| (Reverse(*v), *k)).collect();
    for (Reverse(votes), name) in &ranked {
        println!("   {name} {votes}");
    }
    println!("   Same order, and no sort call anywhere. A tuple is Ord when its parts");
    println!("   are, compared left to right, so (Reverse(count), name) orders by");
    println!("   count descending and then by name — the tie-break is in the type.");

    println!();
    println!("4. A question a HashMap cannot be asked");
    let window: Vec<&str> = t.range("B".."D").map(|(k, _)| *k).collect();
    println!("   candidates in \"B\"..\"D\"  {window:?}");
    println!("   half-open, so \"D\" is a bound and Dan is excluded; range(\"B\"..=\"D\")");
    println!("   would still exclude him, because \"Dan\" > \"D\" as a string.");
    let inclusive: Vec<&str> = t.range("B"..="D").map(|(k, _)| *k).collect();
    println!("   candidates in \"B\"..=\"D\" {inclusive:?}");

    println!();
    println!("5. The key you cannot use");
    // let mut by_share: BTreeMap<f64, &str> = BTreeMap::new();
    // by_share.insert(0.273, "Ada");        // E0277: the trait bound `f64: Ord` is not satisfied
    println!("   f64 is PartialOrd but not Ord, because NaN compares false against");
    println!("   everything including itself, so a BTreeMap keyed on one does not");
    println!("   compile. Scale to an integer and the ordering survives:");
    let total: u32 = t.values().sum();
    let by_share: BTreeMap<u32, &str> = t.iter().map(|(k, v)| (v * 10_000 / total, *k)).collect();
    println!("   share in basis points  {by_share:?}");

    println!();
    println!("6. ...and look at what that just cost");
    println!("   Five candidates went in and {} came out.", by_share.len());
    println!("   Ada and Cara both hold 2727 basis points, Ben and Dan both 1818, and");
    println!("   a map keeps one value per key — so the second insert of each pair");
    println!("   overwrote the first. Keying on a DERIVED value drops ties silently:");
    println!("   nothing errors, the type is right, and two candidates are gone.");
    let by_share_kept: BTreeSet<(u32, &str)> =
        t.iter().map(|(k, v)| (v * 10_000 / total, *k)).collect();
    println!("   as a set of pairs      {by_share_kept:?}");
    println!("   The pair is the key, so equal shares no longer collide — and the");
    println!("   ordering is still share-then-name, for the same left-to-right reason");
    println!("   as the leaderboard in 3.");
}
```
<!-- /source -->

<!-- output:sorted_collections_kata -->
*Verified output of [`sorted_collections_kata.rs`](examples/sorted_collections_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The roll, alphabetical — nothing here sorts
   Ada 3
   Ben 2
   Cara 3
   Dan 2
   Eve 1

2. The leaderboard, by leaving the map
   Ada 3
   Cara 3
   Ben 2
   Dan 2
   Eve 1
   Two ties — Ada/Cara at 3 and Ben/Dan at 2 — and `then` settles both
   by name. Without it `sort_by` is still *stable*, so the order would
   be the map's alphabetical one; but the rule would be implicit, and a
   switch to sort_unstable_by would silently change the answer.

3. The leaderboard, by making the count part of the key
   Ada 3
   Cara 3
   Ben 2
   Dan 2
   Eve 1
   Same order, and no sort call anywhere. A tuple is Ord when its parts
   are, compared left to right, so (Reverse(count), name) orders by
   count descending and then by name — the tie-break is in the type.

4. A question a HashMap cannot be asked
   candidates in "B".."D"  ["Ben", "Cara"]
   half-open, so "D" is a bound and Dan is excluded; range("B"..="D")
   would still exclude him, because "Dan" > "D" as a string.
   candidates in "B"..="D" ["Ben", "Cara"]

5. The key you cannot use
   f64 is PartialOrd but not Ord, because NaN compares false against
   everything including itself, so a BTreeMap keyed on one does not
   compile. Scale to an integer and the ordering survives:
   share in basis points  {909: "Eve", 1818: "Dan", 2727: "Cara"}

6. ...and look at what that just cost
   Five candidates went in and 3 came out.
   Ada and Cara both hold 2727 basis points, Ben and Dan both 1818, and
   a map keeps one value per key — so the second insert of each pair
   overwrote the first. Keying on a DERIVED value drops ties silently:
   nothing errors, the type is right, and two candidates are gone.
   as a set of pairs      {(909, "Eve"), (1818, "Ben"), (1818, "Dan"), (2727, "Ada"), (2727, "Cara")}
   The pair is the key, so equal shares no longer collide — and the
   ordering is still share-then-name, for the same left-to-right reason
   as the leaderboard in 3.
```
<!-- /output -->

</details>

---

## See also

- [`HashMap`](../the_hashmap/README.md) — the same API without the order, and `entry`, which both share
- [`HashSet`](../the_hashset/README.md) — the unordered half of the set story, and the `Vec`-beside-the-set trick for insertion order
- [`Vec`](../the_vec/README.md) — where a leaderboard ends up, and `sort_by` against `sort_unstable_by`
- [Comparison traits](../../12_Traits/comparison_traits/README.md) — what `Ord` demands over `PartialOrd`, and why `f64` has only the second
- [`collect` and `FromIterator`](../../24_Iterators/collect_and_fromiterator/README.md) — the call that turns any iterator into one of these
- [Collect the iterator into a `Vec`](../../24_Iterators/collect_into_a_vec/README.md) — the prior question: whether to materialize at all

## Sources

[`BTreeMap` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) and [`BTreeSet` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html); the module page for [`std::collections` ↗](https://doc.rust-lang.org/std/collections/index.html) opens with the decision table this page's own comparison is a longer form of. The `E0277` transcript is a real compile of the six-line program above it, on rustc 1.98.0.

## Po polsku

`BTreeMap` i `BTreeSet` to para uporządkowana: klucze leżą **w kolejności**, więc przejście po takiej kolekcji jest już posortowane, a zebranie czegoś do niej jest sortowaniem, którego nie musiałeś pisać. Płaci się za to cechą `Ord` zamiast `Hash` oraz wyszukiwaniem, które schodzi po drzewie, zamiast skoczyć prosto pod adres.

Dla polskiego czytelnika jest tu jedna rzecz, której angielski oryginał nie musi zauważać: **`Ord` dla `String` porównuje punkty kodowe Unicode, a nie porządek alfabetyczny polszczyzny**. Skutek jest natychmiastowy i widoczny — `ą` wypada *za* `z`, a nie tuż po `a`, więc lista nazwisk posortowana przez `BTreeMap` nie jest listą posortowaną po polsku. To nie jest wada `BTreeMap`, tylko poprawne zachowanie porównania bajt po bajcie; prawdziwe sortowanie językowe wymaga reguł kolacji (biblioteki spod hasła *collation*, np. ICU), których biblioteka standardowa nie ma i nie udaje, że ma.

Pułapka, którą strona nazywa wprost, warta powtórzenia: **sortuje po kluczu, a ty zwykle chcesz posortować po wartości.** Mapa uporządkowana nie pomoże ustawić wyników od największego — do tego zbiera się do `Vec` i sortuje jawnie. Przydaje się też wiedzieć, że `BTreeSet<T>` to w istocie `BTreeMap<T, ()>`, czyli ten sam mechanizm z pustą wartością.

**Szukaj po polsku:** kolekcje uporządkowane · sortowanie polskich znaków · reguły kolacji · `rust BTreeMap ordering` · `rust sort by value`
