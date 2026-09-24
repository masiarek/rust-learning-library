# Lints around `HashMap` and `HashSet`

Beside [A first `HashMap`](../a_first_hashmap/README.md) and [`HashMap`](../the_hashmap/README.md) · the errors, not the warnings: [Every `HashMap` and `HashSet` error, and its fix](../hashmap_errors/README.md)

**Level:** 201 · a reference, by lint

**One line:** Twenty-two clippy lints, in twenty-one entries, that a map or a set draws: `contains_key` then `insert`, `get(k).is_some()` where `contains_key` was meant, `len() == 0`, `or_insert(Vec::new())`, a loop over `(_, v)` that wanted `values()`, a key with interior mutability, a `for` over a hash type when the order was going to matter. Each has the program that draws it, what clippy 0.1.98 prints, a silent version, and when the lint is right. Then what nothing flags: an `insert` that overwrites a running total, a winner picked by iteration order, and six discarded return values, because nothing on `HashMap` is `#[must_use]`.

Every transcript was recorded on rustc 1.98.0 and clippy 0.1.98 by running the command in its title from the folder holding `bad.rs`. The lines dropped are the documentation link, the `#[warn(...)] on by default` or *requested on the command line* note, and the closing warning count. Each silent version was run with that lint plus `clippy::all`, `clippy::pedantic` and `rust_2018_idioms`, and printed nothing. Clippy is not re-run on each commit the way an example is, so a later clippy can reword a warning.

## Turning them on

Clippy's default groups need nothing: `cargo clippy` prints them. The opt-in ones take a flag or a `Cargo.toml` entry:

```sh
cargo clippy -- -W clippy::or_fun_call -W clippy::needless_collect -W clippy::collection_is_never_read -W clippy::implicit_hasher -W clippy::explicit_iter_loop -W clippy::map_unwrap_or -W clippy::cloned_instead_of_copied -W clippy::iter_over_hash_type -W clippy::get_unwrap -W clippy::indexing_slicing
```

```toml
[lints.clippy]
or_fun_call = "warn"
needless_collect = "warn"
collection_is_never_read = "warn"
implicit_hasher = "warn"
explicit_iter_loop = "warn"
map_unwrap_or = "warn"
cloned_instead_of_copied = "warn"
iter_over_hash_type = "warn"
get_unwrap = "warn"
indexing_slicing = "warn"
```

`disallowed_types` is on by default and does nothing until a `clippy.toml` beside `Cargo.toml` names a type; its entry shows the file.

## Find the lint

| Lint | From | On by default? | Fires on |
|---|---|---|---|
| [`map_entry`](#map_entry) | clippy · perf | yes | `if !m.contains_key(&k) { m.insert(k, v); }` with a `String` key |
| [`unnecessary_get_then_check`](#unnecessary_get_then_check) | clippy · complexity | yes | `m.get(k).is_some()`, `s.get(x).is_none()` |
| [`redundant_pattern_matching`](#redundant_pattern_matching) | clippy · style | yes | `if let Some(_) = m.get(k)` |
| [`len_zero`](#len_zero) | clippy · style | yes | `m.len() == 0` |
| [`unwrap_or_default`](#unwrap_or_default) | clippy · style | yes | `entry(k).or_insert(Vec::new())`, `or_insert_with(Vec::new)` |
| [`manual_unwrap_or_default`](#manual_unwrap_or_default) | clippy · suspicious | yes | a `match` on `get` with `None => 0` |
| [`for_kv_map`](#for_kv_map) | clippy · style | yes | `for (_, v) in &m` |
| [`iter_kv_map`](#iter_kv_map) | clippy · complexity | yes | `m.iter().map(\|(k, _)\| k)` |
| [`unnecessary_to_owned`](#unnecessary_to_owned) | clippy · perf | yes | `m.get(&word.to_string())` on `String` keys |
| [`mutable_key_type`](#mutable_key_type) | clippy · suspicious | yes | a key holding a `Cell` |
| [`disallowed_types`](#disallowed_types) | clippy · style | yes, once configured | `HashMap` where `clippy.toml` bans it |
| [`or_fun_call`](#or_fun_call) | clippy · nursery | no | `or_insert(Vec::with_capacity(4))` |
| [`needless_collect`](#needless_collect) | clippy · nursery | no | `m.keys().collect::<Vec<_>>().len()` |
| [`collection_is_never_read`](#collection_is_never_read) | clippy · nursery | no | a map filled and never asked anything |
| [`implicit_hasher`](#implicit_hasher) | clippy · pedantic | no | `pub fn f(m: &HashMap<K, V>)` |
| [`explicit_iter_loop`](#explicit_iter_loop) | clippy · pedantic | no | `for (k, v) in m.iter()` |
| [`map_unwrap_or`](#map_unwrap_or), with `map_clone` | clippy · pedantic, style | no, yes | `get(k).map(\|c\| *c).unwrap_or(0)` |
| [`cloned_instead_of_copied`](#cloned_instead_of_copied) | clippy · pedantic | no | `get(k).cloned()` on a `u32` |
| [`iter_over_hash_type`](#iter_over_hash_type) | clippy · restriction | no | any `for` over a `HashMap` or `HashSet` |
| [`get_unwrap`](#get_unwrap-and-indexing_slicing) | clippy · restriction | no | `m.get(k).unwrap()` |
| [`indexing_slicing`](#get_unwrap-and-indexing_slicing) | clippy · restriction | no | `m[k]` — the previous lint's fix |

## Clippy, on by default

### `map_entry`

**clippy · perf** · warn by default · fires on `contains_key` followed by `insert`, when the key is an owned value

```rust
use std::collections::HashMap;

fn main() {
    let mut first_seen: HashMap<String, usize> = HashMap::new();
    for (position, word) in ["the", "cat", "the"].into_iter().enumerate() {
        let key = word.to_string();
        if !first_seen.contains_key(&key) {
            first_seen.insert(key, position);
        }
    }
    let mut rows: Vec<(String, usize)> = first_seen.into_iter().collect();
    rows.sort_unstable();
    println!("{rows:?}"); // [("cat", 1), ("the", 0)]
}
```

```text title="clippy-driver --edition 2024 bad.rs — clippy 0.1.98"
warning: usage of `contains_key` followed by `insert` on a `HashMap`
 --> bad.rs:7:9
  |
7 | /         if !first_seen.contains_key(&key) {
8 | |             first_seen.insert(key, position);
9 | |         }
  | |_________^ help: try: `first_seen.entry(key).or_insert(position);`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let mut first_seen: HashMap<String, usize> = HashMap::new();
    for (position, word) in ["the", "cat", "the"].into_iter().enumerate() {
        let key = word.to_string();
        first_seen.entry(key).or_insert(position);
    }
    let mut rows: Vec<(String, usize)> = first_seen.into_iter().collect();
    rows.sort_unstable();
    println!("{rows:?}"); // [("cat", 1), ("the", 0)]
}
```

**When it is right.** Always right: `contains_key` hashes and finds the key, then `insert` hashes and finds it again, and `entry` does the work once. Its reach is narrower than its name. The same shape with `&str` keys — `if !counts.contains_key(word) { counts.insert(word, 1); }`, with an `else` branch and without one — drew nothing on 0.1.98, so a two-lookup loop over borrowed keys goes unflagged; write it with `entry` anyway, as [the counting loop](../a_first_hashmap/README.md#6-entry-the-slot-for-a-key-filled-or-empty) does.

### `unnecessary_get_then_check`

**clippy · complexity** · warn by default · fires on `get(k).is_some()` and `get(x).is_none()`

```rust
use std::collections::{HashMap, HashSet};

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let words: HashSet<&str> = HashSet::from(["the", "cat"]);
    if counts.get("the").is_some() {
        println!("the is counted"); // the is counted
    }
    if words.get("dog").is_none() {
        println!("dog is not a word here"); // dog is not a word here
    }
}
```

```text title="clippy-driver --edition 2024 bad.rs — clippy 0.1.98"
warning: unnecessary use of `get("the").is_some()`
 --> bad.rs:6:15
  |
6 |     if counts.get("the").is_some() {
  |               ^^^^^^^^^^^^^^^^^^^^ help: replace it with: `contains_key("the")`

warning: unnecessary use of `get("dog").is_none()`
 --> bad.rs:9:14
  |
9 |     if words.get("dog").is_none() {
  |        ------^^^^^^^^^^^^^^^^^^^^
  |        |
  |        help: replace it with: `!words.contains("dog")`
```

**Silent:**

```rust
use std::collections::{HashMap, HashSet};

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let words: HashSet<&str> = HashSet::from(["the", "cat"]);
    if counts.contains_key("the") {
        println!("the is counted"); // the is counted
    }
    if !words.contains("dog") {
        println!("dog is not a word here"); // dog is not a word here
    }
}
```

**When it is right.** Always right; the two forms do the same lookup and one of them says what it means. The book's sentence that `contains_key` returns an `Option` describes the shape this lint replaces.

### `redundant_pattern_matching`

**clippy · style** · warn by default · fires on `if let Some(_) = m.get(k)`

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    if let Some(_) = counts.get("the") {
        println!("the is counted"); // the is counted
    }
}
```

```text title="clippy-driver --edition 2024 bad.rs — clippy 0.1.98"
warning: redundant pattern matching
 --> bad.rs:5:12
  |
5 |     if let Some(_) = counts.get("the") {
  |            ^^^^^^^
  |
help: consider using `is_some()`
  |
5 -     if let Some(_) = counts.get("the") {
5 +     if counts.get("the").is_some() {
  |
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    if counts.contains_key("the") {
        println!("the is counted"); // the is counted
    }
}
```

**When it is right.** Right, and one step short: take its suggestion and the previous lint fires on the result, *unnecessary use of `get("the").is_some()`*, pointing at `contains_key`. Two clippy runs to reach the one-word answer; the silent version skips to it.

### `len_zero`

**clippy · style** · warn by default · fires on `len() == 0`

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    if counts.len() == 0 {
        println!("nothing counted");
    } else {
        println!("{} words", counts.len()); // 2 words
    }
}
```

```text title="clippy-driver --edition 2024 bad.rs — clippy 0.1.98"
warning: length comparison to zero
 --> bad.rs:5:8
  |
5 |     if counts.len() == 0 {
  |        ^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `counts.is_empty()`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    if counts.is_empty() {
        println!("nothing counted");
    } else {
        println!("{} words", counts.len()); // 2 words
    }
}
```

**When it is right.** A matter of reading: `len()` on a `HashMap` is a stored count, so the two cost the same, and the lint is about the word. Same on a `Vec`, a `String` and a `HashSet`.

### `unwrap_or_default`

**clippy · style** · warn by default · fires on `or_insert(Vec::new())` and `or_insert_with(Vec::new)`

```rust
use std::collections::HashMap;

fn main() {
    let mut by_length: HashMap<usize, Vec<&str>> = HashMap::new();
    for word in ["the", "cat", "elephant", "dog"] {
        by_length.entry(word.len()).or_insert(Vec::new()).push(word);
    }
    println!("{:?}", by_length[&3]); // ["the", "cat", "dog"]
}
```

```text title="clippy-driver --edition 2024 bad.rs — clippy 0.1.98"
warning: use of `or_insert` to construct default value
 --> bad.rs:6:37
  |
6 |         by_length.entry(word.len()).or_insert(Vec::new()).push(word);
  |                                     ^^^^^^^^^^^^^^^^^^^^^ help: try: `or_default()`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let mut by_length: HashMap<usize, Vec<&str>> = HashMap::new();
    for word in ["the", "cat", "elephant", "dog"] {
        by_length.entry(word.len()).or_default().push(word);
    }
    println!("{:?}", by_length[&3]); // ["the", "cat", "dog"]
}
```

**When it is right.** Right whenever the value type has a `Default` and the default is what you want: `Vec::new()`, `String::new()`, `0`. `or_insert_with(Vec::new)` gets the same warning with *use of `or_insert_with` to construct default value*. It stays quiet about `or_insert(0)`, so the counting loop is not touched, though `or_default()` works there too.

### `manual_unwrap_or_default`

**clippy · suspicious** · warn by default · fires on a `match` over `get` whose `None` arm is the default

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let dogs = match counts.get("dog").copied() {
        Some(count) => count,
        None => 0,
    };
    println!("{dogs}"); // 0
}
```

```text title="clippy-driver --edition 2024 bad.rs — clippy 0.1.98"
warning: match can be simplified with `.unwrap_or_default()`
 --> bad.rs:5:16
  |
5 |       let dogs = match counts.get("dog").copied() {
  |  ________________^
6 | |         Some(count) => count,
7 | |         None => 0,
8 | |     };
  | |_____^ help: ascribe the type u32 and replace your expression with: `counts.get("dog").copied().unwrap_or_default()`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let dogs = counts.get("dog").copied().unwrap_or(0);
    println!("{dogs}"); // 0
}
```

**When it is right.** Right that the `match` is four lines for one method call. `unwrap_or(0)` is the silent spelling this library prefers, because the 0 is visible; `unwrap_or_default()` is silent too. The same `match` with `Some(count) => *count` and no `copied()` drew nothing, so the lint depends on the exact shape.

### `for_kv_map`

**clippy · style** · warn by default · fires on `for (_, v) in &m`

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let mut total = 0;
    for (_, count) in &counts {
        total += count;
    }
    println!("{total}"); // 4
}
```

```text title="clippy-driver --edition 2024 bad.rs — clippy 0.1.98"
warning: you seem to want to iterate on a map's values
 --> bad.rs:6:23
  |
6 |     for (_, count) in &counts {
  |                       ^^^^^^^
  |
help: use the corresponding method
  |
6 -     for (_, count) in &counts {
6 +     for count in counts.values() {
  |
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let mut total = 0;
    for count in counts.values() {
        total += count;
    }
    println!("{total}"); // 4
}
```

**When it is right.** Always: `values()` says what the loop reads, and `keys()` the same for the other half. `counts.values().sum::<u32>()` is one line shorter again.

### `iter_kv_map`

**clippy · complexity** · warn by default · fires on `iter().map(|(k, _)| k)`

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let mut words: Vec<&&str> = counts.iter().map(|(word, _)| word).collect();
    words.sort();
    println!("{words:?}"); // ["cat", "the"]
}
```

```text title="clippy-driver --edition 2024 bad.rs — clippy 0.1.98"
warning: iterating on a map's keys
 --> bad.rs:5:33
  |
5 |     let mut words: Vec<&&str> = counts.iter().map(|(word, _)| word).collect();
  |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `counts.keys()`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let mut words: Vec<&&str> = counts.keys().collect();
    words.sort();
    println!("{words:?}"); // ["cat", "the"]
}
```

**When it is right.** Always; it is the iterator-chain form of `for_kv_map`. Note the `sort()` after the `collect()`: the keys come out in [no fixed order](../a_first_hashmap/README.md#8-iterating-and-why-the-order-is-not-the-insertion-order), and no lint on this page says so.

### `unnecessary_to_owned`

**clippy · perf** · warn by default · fires on `get(&word.to_string())` on a map with `String` keys

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<String, u32> = HashMap::from([(String::from("the"), 3)]);
    let word = "the";
    println!("{:?}", counts.get(&word.to_string())); // Some(3)
}
```

```text title="clippy-driver --edition 2024 bad.rs — clippy 0.1.98"
warning: unnecessary use of `to_string`
 --> bad.rs:6:33
  |
6 |     println!("{:?}", counts.get(&word.to_string())); // Some(3)
  |                                 ^^^^^^^^^^^^^^^^^ help: replace it with: `word`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<String, u32> = HashMap::from([(String::from("the"), 3)]);
    let word = "the";
    println!("{:?}", counts.get(word)); // Some(3)
}
```

**When it is right.** Always: `get` takes any `&Q` the key can be borrowed as, and a `String` key is borrowed as a `str`, so the `to_string()` allocated a `String` only to look through it as a `&str`. [`Borrow`](../../12_Traits/borrow_trait/README.md) is the trait that makes the silent version compile.

### `mutable_key_type`

**clippy · suspicious** · warn by default · fires on a key type with interior mutability

```rust
use std::cell::Cell;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

struct Counter(Cell<u32>);

impl PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl Eq for Counter {}

impl Hash for Counter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.get().hash(state);
    }
}

fn main() {
    let mut seen: HashSet<Counter> = HashSet::new();
    seen.insert(Counter(Cell::new(1)));
    println!("{}", seen.len()); // 1
}
```

```text title="clippy-driver --edition 2024 bad.rs — clippy 0.1.98"
warning: mutable key type
  --> bad.rs:22:5
   |
22 |     let mut seen: HashSet<Counter> = HashSet::new();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: ... because it contains `Counter`, which has interior mutability
   = note: ... because it contains `Cell<u32>`, which has interior mutability
   = note: ... because it contains `UnsafeCell<u32>`, which has interior mutability
```

**Silent:**

```rust
use std::collections::HashSet;

fn main() {
    let mut seen: HashSet<u32> = HashSet::new();
    seen.insert(1);
    println!("{}", seen.len()); // 1
}
```

**When it is right.** Right whenever the hash reads the mutable part, as here: a `Counter` stored under the hash of 1 and then set to 2 through the `Cell` is filed in the wrong bucket and can never be found again, although it is in the set. The contract is the one on [`HashMap`](../the_hashmap/README.md): two equal keys hash equally, forever. Noise when the mutable part is left out of `Hash` and `Eq` on purpose, which the lint cannot see.

### `disallowed_types`

**clippy · style** · warn by default, once `clippy.toml` names a type · fires on every mention of the type

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    println!("{}", counts.len()); // 2
}
```

```toml
disallowed-types = [
    { path = "std::collections::HashMap", reason = "use BTreeMap here so that output is ordered" },
]
```

```text title="CLIPPY_CONF_DIR=. clippy-driver --edition 2024 bad.rs — clippy 0.1.98, with the clippy.toml above"
warning: use of a disallowed type `std::collections::HashMap`
 --> bad.rs:1:1
  |
1 | use std::collections::HashMap;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: use BTreeMap here so that output is ordered

warning: use of a disallowed type `std::collections::HashMap`
 --> bad.rs:4:38
  |
4 |     let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
  |                                      ^^^^^^^
  |
  = note: use BTreeMap here so that output is ordered

warning: use of a disallowed type `std::collections::HashMap`
 --> bad.rs:4:17
  |
4 |     let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
  |                 ^^^^^^^^^^^^^^^^^^
  |
  = note: use BTreeMap here so that output is ordered
```

**Silent:**

```rust
use std::collections::BTreeMap;

fn main() {
    let counts: BTreeMap<&str, u32> = BTreeMap::from([("the", 3), ("cat", 1)]);
    println!("{}", counts.len()); // 2
}
```

**When it is right.** Right for a project with a rule: every map is a `BTreeMap` so that output is reproducible, or every map is a faster third-party one. The `reason` is printed under each warning, so the rule explains itself. Three warnings for one line, at the `use`, the annotation and the constructor.

## Clippy, opt-in

### `or_fun_call`

**clippy · nursery** · off by default · fires on a function call as the argument of `or_insert`

```rust
use std::collections::HashMap;

fn main() {
    let mut by_length: HashMap<usize, Vec<&str>> = HashMap::new();
    for word in ["the", "cat", "elephant", "dog"] {
        by_length.entry(word.len()).or_insert(Vec::with_capacity(4)).push(word);
    }
    println!("{:?}", by_length[&3]); // ["the", "cat", "dog"]
}
```

```text title="clippy-driver --edition 2024 -W clippy::or_fun_call bad.rs — clippy 0.1.98"
warning: function call inside of `or_insert`
 --> bad.rs:6:37
  |
6 |         by_length.entry(word.len()).or_insert(Vec::with_capacity(4)).push(word);
  |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `or_insert_with(|| Vec::with_capacity(4))`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let mut by_length: HashMap<usize, Vec<&str>> = HashMap::new();
    for word in ["the", "cat", "elephant", "dog"] {
        by_length.entry(word.len()).or_insert_with(|| Vec::with_capacity(4)).push(word);
    }
    println!("{:?}", by_length[&3]); // ["the", "cat", "dog"]
}
```

**When it is right.** Right when the argument costs something: `or_insert(expr)` evaluates `expr` on every pass, needed or not, so `Vec::with_capacity(4)` allocates four slots for every word and drops them whenever the slot was already filled. `or_insert_with` takes a closure and runs it only for a `Vacant` slot. It is in *nursery*: `or_insert(0)` and `or_insert(Vec::new())` cost nothing and are the previous lint's business, not this one's.

### `needless_collect`

**clippy · nursery** · off by default · fires on a `collect` whose only use is `len()`

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let distinct = counts.keys().collect::<Vec<_>>().len();
    println!("{distinct}"); // 2
}
```

```text title="clippy-driver --edition 2024 -W clippy::needless_collect bad.rs — clippy 0.1.98"
warning: avoid using `collect()` when not needed
 --> bad.rs:5:34
  |
5 |     let distinct = counts.keys().collect::<Vec<_>>().len();
  |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace with: `count()`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let distinct = counts.len();
    println!("{distinct}"); // 2
}
```

**When it is right.** Right that the `Vec` was built to be thrown away; its suggestion, `count()`, still walks every key. The silent version is `len()`, which a map keeps as a number. The number of distinct keys *is* the map's length.

### `collection_is_never_read`

**clippy · nursery** · off by default · fires on a map that is filled and never asked anything

```rust
use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<&str, u32> = HashMap::new();
    counts.insert("the", 3);
    counts.insert("cat", 1);
    println!("counted"); // counted
}
```

```text title="clippy-driver --edition 2024 -W clippy::collection_is_never_read bad.rs — clippy 0.1.98"
warning: collection is never read
 --> bad.rs:4:5
  |
4 |     let mut counts: HashMap<&str, u32> = HashMap::new();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<&str, u32> = HashMap::new();
    counts.insert("the", 3);
    counts.insert("cat", 1);
    println!("counted {} words", counts.len()); // counted 2 words
}
```

**When it is right.** Right for a map that outlived its purpose in a refactor. Its idea of *read* is broad: the same program with the counting loop, `*counts.entry(word).or_insert(0) += 1`, and no later read drew nothing, because `entry` hands a value back.

### `implicit_hasher`

**clippy · pedantic** · off by default · fires on a `pub fn` taking a `&HashMap<K, V>`

```rust
use std::collections::HashMap;

pub fn total(counts: &HashMap<&str, u32>) -> u32 {
    counts.values().sum()
}

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    println!("{}", total(&counts)); // 4
}
```

```text title="clippy-driver --edition 2024 -W clippy::implicit_hasher bad.rs — clippy 0.1.98"
warning: parameter of type `HashMap` should be generalized over different hashers
 --> bad.rs:3:23
  |
3 | pub fn total(counts: &HashMap<&str, u32>) -> u32 {
  |                       ^^^^^^^^^^^^^^^^^^
  |
help: add a type parameter for `BuildHasher`
  |
3 | pub fn total<S: ::std::hash::BuildHasher>(counts: &HashMap<&str, u32, S>) -> u32 {
  |             +++++++++++++++++++++++++++++                           +++
```

**Silent:**

```rust
use std::collections::HashMap;
use std::hash::BuildHasher;

pub fn total<S: BuildHasher>(counts: &HashMap<&str, u32, S>) -> u32 {
    counts.values().sum()
}

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    println!("{}", total(&counts)); // 4
}
```

**When it is right.** `HashMap<K, V>` is short for `HashMap<K, V, RandomState>`, the default hasher, so a function written that way refuses a caller's map built with any other hasher. Right for a library's public API. Noise for a program that has one kind of map; the third parameter is the [next page's](../the_hashmap/README.md) subject rather than this one's.

### `explicit_iter_loop`

**clippy · pedantic** · off by default · fires on `for … in m.iter()`

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    for (word, count) in counts.iter() {
        if *count == 3 {
            println!("{word}"); // the
        }
    }
}
```

```text title="clippy-driver --edition 2024 -W clippy::explicit_iter_loop bad.rs — clippy 0.1.98"
warning: it is more concise to loop over references to containers instead of using explicit iteration methods
 --> bad.rs:5:26
  |
5 |     for (word, count) in counts.iter() {
  |                          ^^^^^^^^^^^^^ help: to write this more concisely, try: `&counts`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    for (word, count) in &counts {
        if *count == 3 {
            println!("{word}"); // the
        }
    }
}
```

**When it is right.** A matter of spelling; the two loops compile to the same thing. `&counts` is the form this library uses, and the form rustc suggests in [entry 15 of the errors page](../hashmap_errors/README.md#15-the-map-is-gone-after-the-loop).

### `map_unwrap_or`

**clippy · pedantic** · off by default · fires on `map(f).unwrap_or(a)`; **`map_clone`** (style, on by default) fires on the same line

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let dogs = counts.get("dog").map(|count| *count).unwrap_or(0);
    println!("{dogs}"); // 0
}
```

```text title="clippy-driver --edition 2024 -W clippy::map_unwrap_or bad.rs — clippy 0.1.98"
warning: called `map(<f>).unwrap_or(<a>)` on an `Option` value
 --> bad.rs:5:16
  |
5 |     let dogs = counts.get("dog").map(|count| *count).unwrap_or(0);
  |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: use `map_or(<a>, <f>)` instead
  |
5 -     let dogs = counts.get("dog").map(|count| *count).unwrap_or(0);
5 +     let dogs = counts.get("dog").map_or(0, |count| *count);
  |

warning: you are using an explicit closure for copying elements
 --> bad.rs:5:16
  |
5 |     let dogs = counts.get("dog").map(|count| *count).unwrap_or(0);
  |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider calling the dedicated `copied` method: `counts.get("dog").copied()`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let dogs = counts.get("dog").copied().unwrap_or(0);
    println!("{dogs}"); // 0
}
```

**When it is right.** Two lints, two suggestions, and the second is the one to take: `.map(|c| *c)` is `copied()`, and `copied().unwrap_or(0)` is the spelling every page here uses for *the count, or zero*. `map_or(0, |c| *c)` would satisfy the first lint and still draw the second.

### `cloned_instead_of_copied`

**clippy · pedantic** · off by default · fires on `cloned()` where the value is `Copy`

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let the = counts.get("the").cloned().unwrap_or(0);
    println!("{the}"); // 3
}
```

```text title="clippy-driver --edition 2024 -W clippy::all -W clippy::pedantic bad.rs — clippy 0.1.98"
warning: used `cloned` where `copied` could be used instead
 --> bad.rs:5:33
  |
5 |     let the = counts.get("the").cloned().unwrap_or(0);
  |                                 ^^^^^^ help: try: `copied`
```

**Silent:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let the = counts.get("the").copied().unwrap_or(0);
    println!("{the}"); // 3
}
```

**When it is right.** Both compile to the same copy of a `u32`; `copied` says that the value is cheap to copy and would stop compiling if the value type changed to a `String`, which is the safety it buys. For a `HashMap<&str, String>`, `cloned()` is the right call and this lint stays quiet.

### `iter_over_hash_type`

**clippy · restriction** · off by default · fires on any `for` over a `HashMap` or a `HashSet`

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("elephant", 2), ("cat", 1)]);
    let mut longest_repeated = "";
    for (word, count) in &counts {
        if *count > 1 && word.len() > longest_repeated.len() {
            longest_repeated = word;
        }
    }
    println!("{longest_repeated}"); // elephant
}
```

```text title="clippy-driver --edition 2024 -W clippy::iter_over_hash_type bad.rs — clippy 0.1.98"
warning: iteration over unordered hash-based type
  --> bad.rs:6:5
   |
 6 | /     for (word, count) in &counts {
 7 | |         if *count > 1 && word.len() > longest_repeated.len() {
 8 | |             longest_repeated = word;
 9 | |         }
10 | |     }
   | |_____^
```

**Silent:**

```rust
use std::collections::BTreeMap;

fn main() {
    let counts: BTreeMap<&str, u32> = BTreeMap::from([("the", 3), ("elephant", 2), ("cat", 1)]);
    let mut longest_repeated = "";
    for (word, count) in &counts {
        if *count > 1 && word.len() > longest_repeated.len() {
            longest_repeated = word;
        }
    }
    println!("{longest_repeated}"); // elephant
}
```

**When it is right.** It is in *restriction*, the group for rules a project opts into knowingly, and it flags every loop over a hash type whether or not the order matters — this one does not, since only the longest of the repeated words survives and there is one. Where the order does matter, the loop is a bug that no default lint sees: the last section shows one. Turn it on for a crate whose output must be reproducible, and answer it with a `BTreeMap`, or by sorting before the loop.

### `get_unwrap` and `indexing_slicing`

**clippy · restriction**, both · off by default · the first fires on `get(k).unwrap()`, the second on the `m[k]` it suggests

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let the = *counts.get("the").unwrap();
    println!("{the}"); // 3
}
```

```text title="clippy-driver --edition 2024 -W clippy::get_unwrap bad.rs — clippy 0.1.98"
warning: called `.get().unwrap()` on a HashMap
 --> bad.rs:5:15
  |
5 |     let the = *counts.get("the").unwrap();
  |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: using `[]` is clearer and more concise
  |
5 -     let the = *counts.get("the").unwrap();
5 +     let the = counts["the"];
  |
```

Take the suggestion, and the other restriction lint fires on the result:

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let the = counts["the"];
    println!("{the}"); // 3
}
```

```text title="clippy-driver --edition 2024 -W clippy::indexing_slicing bad.rs — clippy 0.1.98"
warning: indexing may panic
 --> bad.rs:5:15
  |
5 |     let the = counts["the"];
  |               ^^^^^^^^^^^^^
  |
  = help: consider using `.get(n)` or `.get_mut(n)` instead
```

**Silent under both:**

```rust
use std::collections::HashMap;

fn main() {
    let counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    let the = counts.get("the").copied().unwrap_or(0);
    println!("{the}"); // 3
}
```

**When it is right.** Each is right about its own rule and they cannot both be on: `get_unwrap` says a lookup you know will succeed should look like one, `indexing_slicing` says nothing may panic on a missing key. A project picks one. Both are quiet by default, and `unwrap_or(0)` satisfies both — which is right only when zero is the honest answer for a missing key.

## What no lint catches

```rust
use std::collections::HashMap;

fn main() {
    let scores = [("Ada", 3), ("Ben", 4), ("Ada", 5), ("Ben", 3)];
    let mut totals: HashMap<&str, u32> = HashMap::new();
    for (name, score) in scores {
        totals.insert(name, score);
    }
    let mut rows: Vec<(&str, u32)> = totals.iter().map(|(name, total)| (*name, *total)).collect();
    rows.sort_unstable();
    println!("{rows:?}"); // [("Ada", 5), ("Ben", 3)]
    let best = totals.iter().max_by_key(|(_, total)| **total).map(|(name, _)| *name);
    println!("{best:?}"); // Some("Ada")
}
```

```text title="clippy-driver --edition 2024 -W clippy::all -W clippy::pedantic -W clippy::nursery -W rust_2018_idioms -W unused bad.rs; echo exit $? — clippy 0.1.98"
exit 0
```

Nothing but the exit status, and both lines are wrong. Ada scored 3 + 5 = 8 and Ben 4 + 3 = 7; `insert` replaced each running total with the latest score, so the map says 5 and 3. Then `max_by_key` picks Ada — but change the scores so that two people tie, and it picks whichever the iterator reached last, which is a different person on a different run. The first is the `Some(old)` that `insert` returned and nobody read; the second is the order that `iter_over_hash_type` would have flagged, had it been on. The fixes are `*totals.entry(name).or_insert(0) += score` and an explicit tie-break, both on [`HashMap`](../the_hashmap/README.md).

And the reason the first goes unflagged:

```rust
use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<&str, u32> = HashMap::from([("the", 3), ("cat", 1)]);
    counts.insert("the", 1);
    counts.get("the");
    counts.remove("cat");
    counts.contains_key("cat");
    counts.entry("dog");
    counts.iter();
    counts.keys().map(|word| word.len());
    println!("{}", counts.len()); // 2
}
```

```text title="clippy-driver --edition 2024 -W unused -W clippy::all -W clippy::pedantic bad.rs — clippy 0.1.98"
warning: unused `std::iter::Map` that must be used
  --> bad.rs:11:5
   |
11 |     counts.keys().map(|word| word.len());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: iterators are lazy and do nothing unless consumed
help: use `let _ = ...` to ignore the resulting value
   |
11 |     let _ = counts.keys().map(|word| word.len());
   |     +++++++
```

Seven statements throw their result away and one warning comes back, for the `Map` adaptor, which is `#[must_use]` because it does nothing until consumed. `insert`'s `Option<V>`, `get`'s `Option<&V>`, `remove`, `contains_key`, `entry` and `iter()` are all silent: none of them, and neither `Option` nor `bool`, carries `#[must_use]` in the 1.98.0 standard library. rustc's `unused_must_use` is the lint that would say *you dropped the old value*, and it has nothing to fire on. The same six lines compile under `#![deny(unused_must_use)]` on [the errors page](../hashmap_errors/README.md#20-a-discarded-insert-and-the-lint-that-does-not-fire).

## See also

- [Every `HashMap` and `HashSet` error, and its fix](../hashmap_errors/README.md) — what the compiler refuses, where this page is what it only warns about
- [A first `HashMap`](../a_first_hashmap/README.md) — the lesson, with the counting loop that `map_entry` and `unwrap_or_default` point towards
- [A first `HashSet`](../a_first_hashset/README.md) — the set, with the `insert` whose `bool` nothing makes you read
- [`HashMap`](../the_hashmap/README.md) — the tie-break and the `entry` loop that fix the last section
- [`HashMap` and `HashSet`: resources](../hashmap_resources/README.md) — where each of these is explained at length
- [`BTreeMap` and `BTreeSet`](../sorted_collections/README.md) — the answer to `iter_over_hash_type` and to `disallowed_types`
- [`Borrow`: look up an owned key with a borrowed one](../../12_Traits/borrow_trait/README.md) — why `unnecessary_to_owned`'s silent version compiles
- [Lints around returning by value](../../18_Ownership/returned_by_value_lints/README.md) — the same kind of page for return values
- [Lints around arrays](../arrays/array_lints/README.md) — the same kind of page for arrays
- [Strict clippy](../../05_Tooling/strict_lints/README.md) — turning whole groups on for a project, and what that costs
- [Clippy's lint list ↗](https://rust-lang.github.io/rust-clippy/master/index.html) — every lint, with its group and the version it arrived in

## Po polsku

Dwadzieścia dwa linty clippy (0.1.98), które odzywają się przy mapie albo zbiorze, w dwudziestu jeden hasłach: `map_entry` (`contains_key`, a potem `insert` — ale tylko przy kluczu `String`, przy `&str` milczy), `unnecessary_get_then_check` (`get(k).is_some()` zamiast `contains_key`), `redundant_pattern_matching`, `len_zero`, `unwrap_or_default` (`or_insert(Vec::new())` → `or_default()`), `manual_unwrap_or_default`, `for_kv_map` i `iter_kv_map` (chciałeś `values()` albo `keys()`), `unnecessary_to_owned`, `mutable_key_type` (klucz z `Cell` w środku, którego skrót może się zmienić) i `disallowed_types` (wymaga `clippy.toml`) działają domyślnie; `or_fun_call`, `needless_collect`, `collection_is_never_read`, `implicit_hasher`, `explicit_iter_loop`, `map_unwrap_or`, `cloned_instead_of_copied`, `iter_over_hash_type` (grupa *restriction*: każda pętla po typie mieszającym) oraz para `get_unwrap` i `indexing_slicing`, która przeczy sobie nawzajem, trzeba włączyć. Przy każdym: program, który go wywołuje, dokładny komunikat, wersja cicha i kiedy lint ma rację.

Najważniejsza jest część o tym, czego żaden lint nie łapie: `insert` w pętli nadpisuje sumę bieżącą (Ada ma 3 + 5 = 8, a mapa mówi 5), zwycięzca wybrany przez `max_by_key` przy remisie zależy od kolejności iteracji, a sześć wyrzuconych wyników — `insert`, `get`, `remove`, `contains_key`, `entry`, `iter` — nie daje żadnego ostrzeżenia, bo nic w `HashMap` nie ma atrybutu `#[must_use]`.

**Szukaj po polsku:** `clippy map_entry` · `clippy unnecessary_get_then_check contains_key` · `clippy unwrap_or_default or_insert` · `clippy iter_over_hash_type` · `clippy mutable_key_type` · `rust HashMap insert nadpisuje wartość`
