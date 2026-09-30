# A first `HashSet`

**Level:** 101 · for newcomers

**One line:** A `HashSet` remembers which values it has seen, once each: `insert` tells you whether the value was new, `contains` whether it is there, and a `Vec` collected into one loses its duplicates. This page walks §5.6 of *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) one printed step at a time, then hands over to the four set operations.

Read next: [`HashSet`](../the_hashset/README.md) — union, intersection, difference and symmetric difference, and the `Vec`-beside-the-set trick. Before this page, if a map is still new: [A first `HashMap`](../a_first_hashmap/README.md). Beside both: [every error](../hashmap_errors/README.md) · [lints](../hashmap_lints/README.md) · [resources](../hashmap_resources/README.md).

```rust
use std::collections::HashSet;

fn main() {
    let mut words: HashSet<&str> = HashSet::new();
    println!("{}", words.insert("Hello"));     // true
    println!("{}", words.insert("Hello"));     // false
    println!("{}", words.contains("Hello"));   // true
    println!("{}", words.len());               // 1
}
```

A set is a [`HashMap`](../a_first_hashmap/README.md) with nothing on the right-hand side — std's docs say it is a `HashMap<T, ()>` underneath — so everything about keys carries over: the `use` line, `T: Eq + Hash`, no order. What is new is that every question is a yes or a no, and the program [`first_hashset.rs`](examples/first_hashset.rs) prints the answers in the order below.

## 1. Making one, and `insert`'s answer

```rust
use std::collections::HashSet;

fn main() {
    let mut words: HashSet<&str> = HashSet::new();
    for word in ["Hello", "world", "Rust", "Programming"] {
        println!("{word}: {}", words.insert(word));   // Hello: true … Programming: true
    }
    println!("{}", words.len());   // 4
}
```

`HashSet` needs `use std::collections::HashSet;` for the same reason a map does. `insert(value)` returns a `bool`: `true` when the value was **not** already there and went in. The book shows the four inserts and moves on; the four `true`s are what it does not show.

## 2. The same value twice

```rust
use std::collections::HashSet;

fn main() {
    let mut words: HashSet<&str> = HashSet::new();
    words.insert("Hello");
    let again: bool = words.insert("Hello");
    println!("{again} {}", words.len());   // false 1
}
```

A second `insert("Hello")` returns `false` and changes nothing: `len` is still 1. That is the book's *"'Hello' appears only once"* as a return value, and it is the one to remember, because `if !words.insert(x) { … }` is how you find a duplicate the moment it arrives — one lookup, no `contains` first. The [next page](../the_hashset/README.md) opens with it.

## 3. `contains`: yes or no

```rust
use std::collections::HashSet;

fn main() {
    let mut words: HashSet<&str> = HashSet::new();
    words.insert("world");
    let has: bool = words.contains("world");
    println!("{has} {}", words.contains("World"));   // true false
}
```

`contains` returns a `bool` — here the book is right, and says so. There is no value to hand back, so there is no `get`-with-an-`Option` step the way a map has; `HashSet` does have a `get`, which returns `Option<&T>` of the *stored* value, and the [word-ladder kata](../the_hashset/README.md#practice) shows the one situation where that matters. Values are case-sensitive, as keys are.

## 4. `remove`: `bool`, true the first time only

```rust
use std::collections::HashSet;

fn main() {
    let mut words: HashSet<&str> = HashSet::new();
    words.insert("Rust");
    println!("{}", words.remove("Rust"));   // true
    println!("{}", words.remove("Rust"));   // false
    println!("{}", words.len());            // 0
}
```

`remove` answers whether there was anything to remove. A map's `remove` returns `Option<V>`, the value; a set has no value, so a `bool` says the same thing.

## 5. `take`: the value itself, in an `Option`

```rust
use std::collections::HashSet;

fn main() {
    let mut words: HashSet<&str> = HashSet::new();
    words.insert("world");
    if let Some(word) = words.take("world") {
        println!("Removed: {word}");   // Removed: world
    }
    println!("{:?} {}", words.take("world"), words.len());   // None 0
}
```

`take` is `remove` that gives the value back: `Some("world")` the first time, `None` once it is gone. The book's example is this `if let`, and it holds. What it adds — *"we'll see more examples of its usage in Chapter 11, Section 11.1.4"* — is not checked here: chapter 11 is not read on these pages, and `Option` has a method called `take` too, a different one, so which `take` that section means is left open.

## 6. Iterating: every value once, in no particular order

```rust
use std::collections::{BTreeSet, HashSet};

fn main() {
    let words: HashSet<&str> = ["Hello", "Programming"].into_iter().collect();
    let mut seen: Vec<&str> = Vec::new();
    for word in &words {
        seen.push(*word);
    }
    seen.sort();
    println!("{seen:?}");   // ["Hello", "Programming"]
    let ordered: BTreeSet<&str> = words.iter().copied().collect();
    println!("{ordered:?}");   // {"Hello", "Programming"}
}
```

`for word in &words` visits each value once, as a reference. The order is not the insertion order and differs between runs, for the same reason [a map's does](../a_first_hashmap/README.md#8-iterating-and-why-the-order-is-not-the-insertion-order): the hash function is seeded afresh for every process. The book prints its set with `println!("{:?}", words)` and reads the result off; on your machine the four words come out in some other order. Sort first, or use a [`BTreeSet`](../sorted_collections/README.md), which prints in order every time. Every example on this page sorts.

## 7. A set from a `Vec`: `collect` drops the duplicates

```rust
use std::collections::HashSet;

fn main() {
    let visits = vec!["Ada", "Ben", "Ada", "Cara", "Ben", "Ada"];
    let people: HashSet<&str> = visits.iter().copied().collect();
    println!("{} {}", visits.len(), people.len());   // 6 3
    let mut first_seen: Vec<&str> = Vec::new();
    let mut known: HashSet<&str> = HashSet::new();
    for name in visits.iter().copied() {
        if known.insert(name) {
            first_seen.push(name);
        }
    }
    println!("{first_seen:?}");   // ["Ada", "Ben", "Cara"]
}
```

Collecting six visits into a set leaves three people. The set cannot tell you who came first — it has no order — so when the order matters, keep a `Vec` beside it and let `insert`'s `bool` decide what gets pushed. That is step 2 doing real work.

## 8. One set operation, as a preview

```rust
use std::collections::HashSet;

fn main() {
    let monday: HashSet<&str> = ["Ada", "Ben", "Cara"].into_iter().collect();
    let tuesday: HashSet<&str> = ["Ben", "Cara", "Dan"].into_iter().collect();
    let mut both: Vec<&str> = monday.intersection(&tuesday).copied().collect();
    both.sort();
    println!("{both:?}");   // ["Ben", "Cara"]
}
```

`intersection` is *in both*. It returns an iterator, not a set, so the `collect` and the `sort` are what make it printable. The other three — `union`, `difference`, `symmetric_difference` — and the trap in `difference`'s argument order are the [next page](../the_hashset/README.md#the-four-operations).

## What the book says, run

The §5.6 rows of [`first_hashmap_claims.rs`](../a_first_hashmap/examples/first_hashmap_claims.rs), whose output is on [A first `HashMap`](../a_first_hashmap/README.md#the-verified-output):

| §5.6 says | Run | Verdict |
|---|---|---|
| a `HashSet` stores only unique keys, with no values | `HashSet<T>` is `HashMap<T, ()>` in std's own description; the second `insert("Hello")` changes nothing | holds |
| `HashSet` needs `use std::collections::HashSet;` | not in the prelude | holds |
| insert `"Hello"` again and it *"appears only once"* | `insert("Hello")` returns `false`, `len` stays 4 | holds; the `bool` is not mentioned |
| `contains` returns a `bool` | `true`, type `bool` | holds |
| `take` removes the item and returns it if it exists | `Some("world")`, then `None` | holds |
| more `take` in Chapter 11, Section 11.1.4 | chapter 11 not read here; `Option::take` is a different method of the same name | not checked |
| `println!("{:?}", words)` shows the words | an order that differs between runs | holds, in no fixed order |

## The verified output

<!-- output:first_hashset -->
*Verified output of [`first_hashset.rs`](examples/first_hashset.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. HashSet::new(), then insert: true means it went in
   insert("Hello") = true
   insert("world") = true
   insert("Rust") = true
   insert("Programming") = true
   len = 4   sorted: ["Hello", "Programming", "Rust", "world"]

2. A second insert of the same value: false, and nothing changes
   insert("Hello") = false   type: bool   len = 4

3. contains: bool
   contains("world") = true   contains("World") = false   type: bool

4. remove: bool, true only the first time
   remove("Rust") = true   remove("Rust") again = false   len = 3

5. take: the value itself, wrapped in an Option
   take("world") = Some("world")   take("world") again = None   len = 2
   Removed: world

6. Iterating: every value once, in no particular order
   sorted after the loop: ["Hello", "Programming"]
   the same in a BTreeSet: {"Hello", "Programming"}
   The loop's own order changes from run to run, which is why this
   program never prints a HashSet directly.

7. A set from a Vec: collect drops the duplicates
   ["Ada", "Ben", "Ada", "Cara", "Ben", "Ada"]
   6 visits, 3 people: ["Ada", "Ben", "Cara"]
   first-seen order, kept in a Vec beside the set: ["Ada", "Ben", "Cara"]

8. One set operation, as a preview: intersection
   monday & tuesday = ["Ben", "Cara"]
```
<!-- /output -->

## Practice

**Two days of sign-ins.** Monday's sheet reads `["Ada", "Ben", "Ada", "Cara", "Ben"]` and Tuesday's `["Cara", "Dan", "Ben", "Dan"]`. Make a set for each day and print how many sign-ins and how many people each had. Answer with `contains` whether Ada came on Tuesday and Dan on Monday, and with `intersection` who came both days. Then print Monday's names in the order they first signed in, with the repeats dropped, using nothing but `insert`'s `bool` and a `Vec`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:first_hashset_kata -->
*[`first_hashset_kata.rs`](examples/first_hashset_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: two days of sign-ins, one set per day.
//!
//!   rustc --edition 2024 first_hashset_kata.rs -o /tmp/fhsk && /tmp/fhsk

use std::collections::HashSet;

/// Sets print in no fixed order, so every set here goes through this first.
fn sorted<'a>(set: &'a HashSet<&'a str>) -> Vec<&'a str> {
    let mut items: Vec<&str> = set.iter().copied().collect();
    items.sort();
    items
}

fn main() {
    let monday = ["Ada", "Ben", "Ada", "Cara", "Ben"];
    let tuesday = ["Cara", "Dan", "Ben", "Dan"];

    println!("1. One set per day");
    let mon: HashSet<&str> = monday.into_iter().collect();
    let tue: HashSet<&str> = tuesday.into_iter().collect();
    println!("   Monday:  {} sign-ins, {} people: {:?}", monday.len(), mon.len(), sorted(&mon));
    println!("   Tuesday: {} sign-ins, {} people: {:?}", tuesday.len(), tue.len(), sorted(&tue));

    println!();
    println!("2. Three questions, answered with contains and intersection");
    println!("   was Ada there on Tuesday? contains(\"Ada\") = {}", tue.contains("Ada"));
    println!("   was Dan there on Monday?  contains(\"Dan\") = {}", mon.contains("Dan"));
    let mut both: Vec<&str> = mon.intersection(&tue).copied().collect();
    both.sort();
    println!("   came both days: {both:?}");

    println!();
    println!("3. Monday's sign-ins in first-seen order, repeats dropped by insert's bool");
    let mut seen: HashSet<&str> = HashSet::new();
    let mut order: Vec<&str> = Vec::new();
    for name in monday {
        if seen.insert(name) {
            order.push(name);
        }
    }
    println!("   {order:?}");
    println!("   insert returned false for the second \"Ada\" and the second \"Ben\", so");
    println!("   neither was pushed; the set remembers, the Vec keeps the order.");
}
```
<!-- /source -->

<!-- output:first_hashset_kata -->
*Verified output of [`first_hashset_kata.rs`](examples/first_hashset_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One set per day
   Monday:  5 sign-ins, 3 people: ["Ada", "Ben", "Cara"]
   Tuesday: 4 sign-ins, 3 people: ["Ben", "Cara", "Dan"]

2. Three questions, answered with contains and intersection
   was Ada there on Tuesday? contains("Ada") = false
   was Dan there on Monday?  contains("Dan") = false
   came both days: ["Ben", "Cara"]

3. Monday's sign-ins in first-seen order, repeats dropped by insert's bool
   ["Ada", "Ben", "Cara"]
   insert returned false for the second "Ada" and the second "Ben", so
   neither was pushed; the set remembers, the Vec keeps the order.
```
<!-- /output -->

</details>

## If you are coming from another language

- **Python.** A `HashSet` is a [`set` ↗](https://docs.python.org/3/tutorial/datastructures.html#sets): `s.add(x)` is `insert`, `x in s` is `contains`, `s.discard(x)` is `remove` (Python's `s.remove(x)` raises `KeyError` on a missing value; Rust's returns `false`), `set(some_list)` is `collect`, and `a & b` is `intersection`, with the same operator available in Rust. The difference to carry across is the return value of `add`: it is `None` in Python, so the habit is `if x not in s: s.add(x)`; in Rust `insert` returns the `bool`, and that two-step is one call. Python's `set` iterates in the same order within a process for the same history; Rust's changes between runs. `frozenset` has no counterpart — a set nobody has a `&mut` to cannot change.
- **ABAP.** A hashed internal table whose row is only the key: `TYPE HASHED TABLE OF string WITH UNIQUE KEY table_line` ([internal tables ↗](https://masiarek.github.io/abap-learning-library/03_Topics/internal_tables/index.html)). `INSERT … INTO TABLE` followed by `sy-subrc` — 0 inserted, 4 already there — is `insert`'s `bool`, one lookup, the same pattern as step 2; `line_exists( itab[ table_line = word ] )` is `contains`. What ABAP has no name for is a set operation: the intersection of two tables is a `LOOP` with a `READ TABLE … WITH TABLE KEY` inside it, which is what `intersection` does for you.
- **Java.** `java.util.HashSet<T>` ([Javadoc ↗](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/HashSet.html)): `add` returns the same `boolean` that Rust's `insert` does, `contains` and `remove` return booleans too, and `new HashSet<>(list)` is `collect`. `retainAll` is `intersection` done in place; Rust's returns a new iterator and leaves both sets alone.

## See also

- [`HashSet`](../the_hashset/README.md) — read next: the four operations, `difference`'s argument order, and the `len()` difference that counts ballots, not people
- [A first `HashMap`](../a_first_hashmap/README.md) — the same table with a value on the right, at the same pace
- [Every `HashMap` and `HashSet` error, and its fix](../hashmap_errors/README.md) — the refusals a first set meets, and the `insert` that nothing refuses
- [Lints around `HashMap` and `HashSet`](../hashmap_lints/README.md) — `get(x).is_none()` for `!contains(x)`, and a key with interior mutability
- [`HashMap` and `HashSet`: resources](../hashmap_resources/README.md) — the reading list
- [`BTreeMap` and `BTreeSet`](../sorted_collections/README.md) — the set that prints in order
- [Set operations](../set_operations/README.md) — all four operations as iterators and as operators, and the questions that build nothing
- [`Vec`](../the_vec/README.md) — where the order goes when you need it back
- [*Rust: The Practical Guide*, run](../../10_Resources/rust_the_practical_guide/README.md) — the book's home here
- [`std::collections::HashSet` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html) — every method, and the `HashMap<T, ()>` description this page leans on

## Po polsku

`HashSet` to **zbiór** (*set*) oparty na tablicy mieszającej: pamięta, które wartości już widział, każdą raz. To ta sama struktura co `HashMap`, tylko bez prawej strony — std opisuje ją jako `HashMap<T, ()>` — więc obowiązują te same zasady: `use std::collections::HashSet;`, wartości muszą spełniać `Eq + Hash`, kolejności nie ma. Nowe jest to, że każde pytanie ma odpowiedź tak/nie: `insert` zwraca `bool` — `true`, gdy wartości jeszcze nie było i weszła, `false`, gdy już była (książka tego nie mówi); `contains` zwraca `bool`; `remove` zwraca `bool`; `take` zwraca `Option<T>` z samą wartością. Zebranie wektora do zbioru (`collect`) usuwa duplikaty — z sześciu wizyt zostają trzy osoby — ale gubi kolejność; jeśli kolejność pierwszego wystąpienia jest potrzebna, trzymaj obok `Vec` i pozwól, by o `push` decydował `bool` z `insert`. Wydruk zbioru przez `{:?}` ma inną kolejność przy każdym uruchomieniu, więc przed wydrukiem sortuj albo weź `BTreeSet`. Operacje na zbiorach — część wspólna (`intersection`), suma, różnica, różnica symetryczna — są na następnej stronie.

**Szukaj po polsku:** zbiór w Ruscie · `rust HashSet insert zwraca bool` · `rust HashSet contains remove take` · `rust HashSet z wektora collect duplikaty`
