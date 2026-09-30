# Sets of sets: why Rust needs no `frozenset`

**Level:** 201 · working knowledge

**One line:** A set of sets is `BTreeSet<BTreeSet<T>>` or `HashSet<BTreeSet<T>>`, never `HashSet<HashSet<T>>`, because `HashSet` does not implement `Hash` — and there is no `frozenset`, because in Rust mutability belongs to the binding (`let` against `let mut`), not to the type, and no safe API hands out a `&mut` to a member while it sits inside a set.

Before this page: [`HashSet`](../the_hashset/README.md), [`BTreeMap` and `BTreeSet`](../sorted_collections/README.md) and [Set operations](../set_operations/README.md). The mathematics — a set whose members are sets, ∅ against {∅}, the power set — is the Math library's [what is a set? ↗](https://masiarek.github.io/math-learning-library/04_Sets/what_is_a_set/index.html).

```rust
use std::collections::BTreeSet;

fn main() {
    let family = BTreeSet::from([
        BTreeSet::from([1, 2]),
        BTreeSet::from([2, 3]),
        BTreeSet::from([2, 1]),
    ]);
    println!("{family:?}");                                   // {{1, 2}, {2, 3}}
    println!("{}", family.len());                             // 2
    println!("{}", family.contains(&BTreeSet::from([2, 1]))); // true
}
```

`{1, 2}` and `{2, 1}` are one member: a `BTreeSet` keeps its members sorted, so two sets with the same members are the same value however they were built. `HashSet<BTreeSet<i32>>` holds the same two members; the program shows both.

## Why Python needs two set types

A Python `set` can change after it is stored. If it were a member of another set, a change would move its hash while the outer set still filed it under the old one, and lookups would miss it. So Python refuses to hash a `set` at all, and `frozenset` is the second type: the same set with `add`, `remove` and `|=` taken away, hashable in exchange. The Python library's [a frozenset can be a member ↗](https://masiarek.github.io/python-learning-library/04_Names_and_Objects/a_frozenset_can_be_a_member/index.html) runs it.

Rust needs neither half of that bargain:

| Python's reason for `frozenset` | Rust's answer |
|---|---|
| a `set` value can always be changed, so immutability has to be a separate type | whether a set can change is decided by the **binding**: `let s` cannot, `let mut s` can, and the same value moves from one to the other |
| a member could change while it is filed, so its hash would go stale | nothing hands out a `&mut` to a member: [`iter` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.iter) yields `&T`, [`get` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.get) returns `Option<&T>`, and there is no `iter_mut` or `get_mut` |

What Rust does refuse is a `HashSet` as a member, for a different reason.

## `HashSet<HashSet<T>>` does not compile

```rust,compile_fail
use std::collections::HashSet;

fn main() {
    let mut family: HashSet<HashSet<i32>> = HashSet::new();
    family.insert(HashSet::from([1, 2]));
    println!("{}", family.len());
}
```

```text title="Abridged — rustc 1.98.0; the pointer into std's source is left out"
error[E0599]: the method `insert` exists for struct `HashSet<HashSet<i32>>`, but its trait bounds were not satisfied
   --> hash_of_hash.rs:5:12
    |
  5 |       family.insert(HashSet::from([1, 2]));
    |              ^^^^^^
    |
    = note: the following trait bounds were not satisfied:
            `HashSet<i32>: Hash`
```

The type can be named and `new()` works; `insert` is where the element bound `T: Eq + Hash` is checked. A struct field fails the same way: `#[derive(Hash)]` on a struct holding a `HashSet<i32>` is `E0277`, *the trait `Hash` is not implemented for `HashSet<i32>`*. And `BTreeSet<HashSet<i32>>` is `E0277` too, since `HashSet` has no `Ord` either.

**Why no `Hash`.** std's documentation does not say; its trait list for `HashSet` simply has `PartialEq` and `Eq` and no `Hash`. What the [`Hash` docs ↗](https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq) do require is `k1 == k2 -> hash(k1) == hash(k2)`, and that is the difficulty. `==` on two `HashSet`s ignores order, but a `Hash` impl gets one `&mut Hasher` and feeds values into it one after another, so the result depends on the order they are fed. A `HashSet`'s iteration order depends on its hasher, its capacity and its history: section 3 of the program builds two equal sets with the same fixed hasher and different capacities, and they iterate `[20, 50, 30, 40, 10]` and `[10, 20, 40, 30, 50]`. Hashed in iteration order, equal sets would get different hashes. An impl that ignored order would have to sort the members first (needing `T: Ord` and an allocation) or hash each member separately and combine the results with `+` or `^`, which needs a fresh hasher per member — and `Hash::hash` is handed one hasher, with no way to make another. That is our reading of the constraint, not a sentence from std.

`BTreeSet` has none of the difficulty. It implements `Hash`, `Eq` and `Ord`; its `Hash` feeds the length and then each member in sorted order, so equal sets feed identical sequences. The program builds `x` from `[3, 1, 2]` and `y` by inserting `2, 3, 1`, and prints `x == y`, `hash(x) == hash(y)` and `x.cmp(&y) == Equal`, all true.

## Mutability belongs to the binding

```rust,compile_fail
use std::collections::HashSet;

fn main() {
    let s = HashSet::from([1, 2]);
    s.insert(3);
    println!("{}", s.len());
}
```

```text title="rustc 1.98.0"
error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
 --> let_not_mut.rs:5:5
  |
5 |     s.insert(3);
  |     ^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
4 |     let mut s = HashSet::from([1, 2]);
  |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
```

`let mut s` makes it compile. But the refusal is about the name `s`, not about the set:

```rust
use std::collections::BTreeSet;

fn main() {
    let frozen = BTreeSet::from([2, 3]);
    let mut thawed = frozen;   // the same value, moved to a `let mut` name
    thawed.insert(4);
    println!("{thawed:?}");    // {2, 3, 4}
    let refrozen = thawed;     // and back: no `mut`, no more changes
    println!("{refrozen:?}");  // {2, 3, 4}
}
```

A Python `frozenset` is frozen for its whole life, whoever holds it; `frozenset({2, 3}).add(4)` is an `AttributeError` because the method does not exist on the type. A Rust set is as frozen as the binding that owns it. That is safe because a move leaves the old name unusable: once `thawed` exists nobody can still be reading the set through `frozen`, and while any `&` borrow of a set is alive, no `&mut` to it can be taken. A function parameter is a binding too — `fn add_one(mut s: BTreeSet<i32>)` takes a set from a caller's immutable `let` and changes its own copy of the name. [Variables](../../15_First_Programs/variables/README.md#mutability-belongs-to-the-binding-not-to-the-value) states the rule, and [mutable binding, mutable reference](../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md) separates it from `&mut`.

## A member cannot change while it is stored

```rust,compile_fail
use std::collections::BTreeSet;

fn main() {
    let mut family: BTreeSet<BTreeSet<i32>> = BTreeSet::new();
    family.insert(BTreeSet::from([1, 2]));
    for inner in family.iter() {
        inner.insert(3);
    }
    println!("{family:?}");
}
```

```text title="Abridged — rustc 1.98.0, up to the summary lines"
error[E0596]: cannot borrow `*inner` as mutable, as it is behind a `&` reference
 --> inner_insert.rs:7:9
  |
6 |     for inner in family.iter() {
  |                  ------------- this iterator yields `&` references
7 |         inner.insert(3);
  |         ^^^^^ `inner` is a `&` reference, so it cannot be borrowed as mutable
```

`family` is `let mut`, and it does not help: the outer set can gain and lose members, but nothing lends out a member mutably. `BTreeSet` has no `iter_mut` (`E0599`, *no method named `iter_mut`*), and writing through `get` is `E0594`, *cannot assign to `*n`, which is behind a `&` reference*. The way to change a member is to take it out, change it, and put it back, so the set files it again under its new value:

```rust
use std::collections::BTreeSet;

fn main() {
    let mut groups = BTreeSet::from([BTreeSet::from([1, 2]), BTreeSet::from([5])]);
    let mut g = groups.take(&BTreeSet::from([1, 2])).expect("was there");
    g.insert(3);
    groups.insert(g);
    println!("{groups:?}");   // {{1, 2, 3}, {5}}
}
```

[`take` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.take) hands the member back owned, which is what lets you change it without a clone; `remove` followed by `insert` of a replacement works when the new value does not need the old one. This is the guarantee Python had to buy with a second type, and Rust gets from the borrow rules for every element type at once — the same reason a `HashMap` key cannot be changed in place.

## The one gap: interior mutability

`Cell` and `RefCell` let you write through a `&`, which is exactly what the borrow rules above rely on nobody doing. For a `HashSet` they cannot get in: neither implements `Hash`.

```rust,compile_fail
use std::cell::Cell;
use std::collections::HashSet;

fn main() {
    let mut s: HashSet<Cell<i32>> = HashSet::new();
    s.insert(Cell::new(1));
    println!("{}", s.len());
}
```

```text title="Abridged — rustc 1.98.0; the pointer into std's source is left out"
error[E0599]: the method `insert` exists for struct `HashSet<Cell<i32>>`, but its trait bounds were not satisfied
   --> cell_hash.rs:6:7
    |
  6 |     s.insert(Cell::new(1));
    |       ^^^^^^
    |
    = note: the following trait bounds were not satisfied:
            `Cell<i32>: Hash`
```

`HashSet<RefCell<i32>>` is the same `E0599`, *`RefCell<i32>: Hash`*. But the gap is not closed everywhere, and it is worth knowing where it stays open:

- **`Cell<T>` and `RefCell<T>` implement `Ord`** (for `T: Ord`, and `Copy` for `Cell`), so `BTreeSet<Cell<i32>>` compiles. Section 6 of the program sets the first of `{1, 2, 3}` to 5 through `&Cell`, and then the set answers `contains(2)` with `false` while 2 is still in it: the search compares with 5 first and stops.
- **A type of your own** can implement `Hash` by hand over a `Cell` it contains; [`mutable_key_type`](../hashmap_lints/README.md#mutable_key_type) shows one in a `HashSet`.

std's docs for both sets call this a *logic error*: "normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code", with unspecified results but no undefined behaviour. Clippy's `mutable_key_type`, warn by default, fires on `BTreeSet<Cell<i32>>` too. So the honest version of the claim is: the borrow checker keeps every ordinary member from changing while stored, and interior mutability is the named exception that a lint, not the compiler, watches. [Interior mutability](../../09_Advanced/interior_mutability/README.md) is where `Cell` and `RefCell` are explained.

## Freezing a `HashSet`: collect it into a `BTreeSet`

When the inner sets are built as `HashSet`s — the fast one to fill — the freeze step is a conversion:

```rust
use std::collections::{BTreeSet, HashSet};

fn main() {
    let working: HashSet<&str> = HashSet::from(["bob", "ann"]);
    let frozen: BTreeSet<&str> = working.into_iter().collect();
    let mut meetings: HashSet<BTreeSet<&str>> = HashSet::new();
    meetings.insert(frozen);
    meetings.insert(BTreeSet::from(["ann", "bob"]));
    println!("{}", meetings.len());   // 1
}
```

`into_iter().collect()` moves the members into a `BTreeSet` without cloning them, and the result is sorted, hashable and orderable. An unordered pair used as a key is the everyday case: `{ann, bob}` and `{bob, ann}` are one meeting. A sorted, de-duplicated `Vec<T>` would also work as a hashable key, since `Vec` implements `Hash`; the `BTreeSet` keeps `contains` and the set operations. Unlike Python's `frozenset(s)`, this is not a new kind of value, just the other set type, and whether it can change is still up to the binding.

---

## The verified output

<!-- output:sets_of_sets -->
*Verified output of [`sets_of_sets.rs`](examples/sets_of_sets.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A set whose members are sets: the inner set is a BTreeSet
   BTreeSet<BTreeSet<i32>> from [{1, 2}, {2, 3}, {2, 1}] = {{1, 2}, {2, 3}}
   len = 2   ({1, 2} and {2, 1} are one member)
   contains {2, 1}? true
   contains {1, 3}? false
   HashSet<BTreeSet<i32>> gets the same 2 members: {{1, 2}, {2, 3}}

2. Built in two orders: equal, and they hash the same
   x from [3, 1, 2] = {1, 2, 3}   y inserted 2, 3, 1 = {1, 2, 3}
   x == y: true   hash(x) == hash(y): true   x.cmp(&y): Equal
   ∅ has 0 members; {∅} has 1: {{}}
   the power set of {1, 2, 3} has 8 members:
   {{}, {1}, {1, 2}, {1, 2, 3}, {1, 3}, {2}, {2, 3}, {3}}

3. Why HashSet is not Hash: equal sets, different iteration orders
   same five members, one fixed hasher, capacities 7 and 112
   small == large: true
   small iterates [20, 50, 30, 40, 10]
   large iterates [10, 20, 40, 30, 50]
   hashed in iteration order, equal? false

4. Mutability belongs to the binding, not to the set
   let frozen = {2, 3}; let mut thawed = frozen; thawed.insert(4) -> {2, 3, 4}
   let refrozen = thawed -> {2, 3, 4}, and refrozen cannot change
   fn add_one(mut s: BTreeSet<i32>) on it -> {1, 2, 3, 4}

5. No &mut to a member: take it out, change it, put it back
   before: {{1, 2}, {5}}
   after take, insert(3), insert: {{1, 2, 3}, {5}}
   iter() yields &T, get() returns Option<&T>: Some({5})

6. The gap: Cell is Ord, not Hash, so it can sit in a BTreeSet
   first member set to 5 through &Cell; members now [5, 2, 3]
   contains(2)? false   contains(5)? true
   (a logic error by std's docs: the result is unspecified; clippy::mutable_key_type warns)

7. Freezing a HashSet: collect it into a BTreeSet
   pairs [("ann", "bob"), ("bob", "ann"), ("bob", "cy")]
   as unordered pairs: 2 -> {{"ann", "bob"}, {"bob", "cy"}}
```
<!-- /output -->

## If you are coming from another language

- **Python.** `frozenset` exists because a `set` is always mutable and so unhashable: `{frozenset({1, 2})}` works and `{{1, 2}}` is `TypeError: unhashable type: 'set'`. Rust's `BTreeSet` plays `frozenset`'s part as the element type, but it is not a read-only type: it has `insert` and `remove`, and what keeps a stored member from changing is that the outer set never lends it out mutably. A frozenset is immutable wherever it goes; a Rust set is immutable exactly while the name that owns it has no `mut`, and a move to a `let mut` name thaws it. `HashSet<HashSet<T>>` failing to compile is Python's `unhashable type: 'set'`, found at compile time and for a different reason: not mutability but that nobody could hash an unordered set in iteration order. The Python library's [a frozenset can be a member ↗](https://masiarek.github.io/python-learning-library/04_Names_and_Objects/a_frozenset_can_be_a_member/index.html) is the matching page, and [a set is a hash table ↗](https://masiarek.github.io/python-learning-library/04_Names_and_Objects/a_set_is_a_hash_table/index.html) explains why a member's hash has to stay put.
- **ABAP.** There is no set type, so no frozen one either: a set is a `HASHED TABLE … WITH UNIQUE KEY table_line`, and read-only is a property of how a table is reached (an importing parameter, a `READ-ONLY` attribute) rather than of the table — the same shape as Rust's binding rule. A set of sets needs each inner table turned into something that can be a key first, which is what collecting into a `BTreeSet` does here. The ABAP library's [sets in ABAP ↗](https://masiarek.github.io/abap-learning-library/03_Topics/sets_in_abap/index.html#sets-of-sets-and-freezing-a-set) has the idioms.
- **Mathematics.** Sets of sets are ordinary there: the power set of {1, 2, 3} has eight members, each a set, and ∅ ≠ {∅} because the second has one member. Section 2 of the program builds both as `BTreeSet<BTreeSet<i32>>` and prints `{{}}` for {∅}. A set cannot contain itself in Rust for a reason of types: `BTreeSet<T>` would need `T` to be `BTreeSet<T>`, an infinite type. The Math library's [what is a set? ↗](https://masiarek.github.io/math-learning-library/04_Sets/what_is_a_set/index.html) covers ∅ against {∅}, and Russell's paradox with Python's frozensets.

## See also

- [Set operations](../set_operations/README.md) — union, intersection, difference and `^`, which work on a set of sets as on any other
- [`BTreeMap` and `BTreeSet`](../sorted_collections/README.md) — the sorted set, which is the one that can be a member
- [`HashSet`](../the_hashset/README.md) — the fast set, and the one that cannot
- [Every `HashMap` and `HashSet` error](../hashmap_errors/README.md#17-a-struct-key-without-eq-and-hash) — the same `Hash` bound on a struct key
- [Lints around `HashMap` and `HashSet`](../hashmap_lints/README.md#mutable_key_type) — `mutable_key_type`, the lint that covers the interior-mutability gap
- [Mutable binding, mutable reference](../../18_Ownership/references/mutable_binding_vs_mutable_reference/README.md) — `let mut` against `&mut`, two separate choices
- [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) — the three ways to walk a collection, and the one a set lacks
- [`std::collections::BTreeSet` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) · [`std::hash::Hash` ↗](https://doc.rust-lang.org/std/hash/trait.Hash.html)

## Po polsku

Python ma dwa typy zbiorów, bo zwykły `set` można zmienić po umieszczeniu go w innym zbiorze — jego skrót (hash) by się wtedy zmienił, a zbiór zewnętrzny szukałby go w złym miejscu. Dlatego `set` w ogóle nie ma skrótu, a `frozenset` to ten sam zbiór bez metod zmieniających, który za to może być elementem innego zbioru albo kluczem słownika. Rust nie potrzebuje zamrożonego zbioru z dwóch powodów. Po pierwsze, to, czy wartość wolno zmieniać, zależy od **wiązania**, a nie od typu: `let s` nie pozwala na `insert` (błąd `E0596`), `let mut s` pozwala, a tę samą wartość można przenieść z jednej nazwy do drugiej. Po drugie, żadne bezpieczne API nie daje `&mut` do elementu, który leży w zbiorze: `iter` daje `&T`, `get` daje `Option<&T>`; żeby zmienić element, trzeba go wyjąć (`take`), zmienić i włożyć z powrotem.

Zbiór zbiorów w Ruscie to `BTreeSet<BTreeSet<T>>` albo `HashSet<BTreeSet<T>>`. `HashSet<HashSet<T>>` się nie kompiluje (`E0599`), bo `HashSet` nie implementuje `Hash`: skrót liczony po kolei zależałby od kolejności elementów, a dwa równe `HashSet`-y potrafią przechodzić po elementach w różnej kolejności. `BTreeSet` trzyma elementy posortowane, więc dwa równe zbiory mają zawsze ten sam skrót. „Zamrożenie” zbioru to po prostu `into_iter().collect::<BTreeSet<_>>()`. Jedyna furtka to mutowalność wewnętrzna: `Cell` i `RefCell` nie mają `Hash`, ale mają `Ord`, więc `BTreeSet<Cell<i32>>` się kompiluje i da się zepsuć porządek zbioru — przed tym ostrzega tylko clippy (`mutable_key_type`).

**Szukaj po polsku:** zbiór zbiorów w Ruscie · `rust frozenset` · `rust HashSet<HashSet>` nie kompiluje się · niemutowalny zbiór · zbiór potęgowy · zbiór pusty a zbiór zawierający zbiór pusty
