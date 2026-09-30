# Set operations: methods borrow, operators build

**Level:** 201 · working knowledge

**One line:** `a.union(&b)` and its three siblings borrow both sets and hand back a lazy iterator of `&T`; `&a | &b`, `&a & &b`, `&a - &b` and `&a ^ &b` clone into a brand-new set; `is_subset`, `is_superset` and `is_disjoint` answer without building anything — and owned `a | b` does not compile at all.

Before this page: [A first `HashSet`](../a_first_hashset/README.md) (`insert`, `contains`, `remove`, `take`) and [`HashSet`](../the_hashset/README.md) (the four operations in a table, `difference`'s argument order). Beside it: [`BTreeMap` and `BTreeSet`](../sorted_collections/README.md), the set that prints in order. The mathematics behind every line here is on the Math library's [algebra of sets ↗](https://masiarek.github.io/math-learning-library/04_Sets/algebra_of_sets/index.html).

```rust
use std::collections::BTreeSet;

fn main() {
    let a = BTreeSet::from([1, 2, 3, 4]);
    let b = BTreeSet::from([3, 4, 5]);
    let lazy: Vec<&i32> = a.intersection(&b).collect();
    let built: BTreeSet<i32> = &a & &b;
    println!("{lazy:?} {built:?}");   // [3, 4] {3, 4}
    println!("{} {}", a.len(), b.len());   // 4 3
}
```

Both lines give the same members. The first borrowed `a` and `b` and yielded references into them; the second cloned the members into a set of its own. `a` and `b` are untouched either way.

## Two spellings, two costs

| method, lazy, yields `&T` | operator on `&a`, `&b`, builds a set | members |
|---|---|---|
| [`a.union(&b)` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.union) | <code>&amp;a &#124; &amp;b</code> | in either |
| [`a.intersection(&b)` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.intersection) | `&a & &b` | in both |
| [`a.difference(&b)` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.difference) | `&a - &b` | in `a`, not in `b` |
| [`a.symmetric_difference(&b)` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.symmetric_difference) | `&a ^ &b` | in exactly one |

- **The method** returns a struct that implements `Iterator<Item = &T>` — the program prints its type, a `std::collections::hash::set::Union`, where the operator's is a plain `HashSet<i32>`. Nothing is allocated. `a.intersection(&b).count()` counts without collecting; `.next()` stops after one; `.copied().collect()` into whichever collection you want is the moment you pay.
- **The operator** is `impl BitOr<&HashSet<T, S>> for &HashSet<T, S>` (and `BitAnd`, `Sub`, `BitXor`), which needs `T: Clone` and returns a fresh `HashSet<T, S>`. Convenient in an expression; a clone of every member it keeps.
- **`BTreeSet`** has the same four methods and the same four operators, needing `T: Ord + Clone` instead of `Eq + Hash + Clone`. Its iterators yield in sorted order, so what they produce prints the same on every run.

## Owned `a | b` does not compile

std implements the operators on **references only**. Two owned sets have no `BitOr`:

```rust,compile_fail
use std::collections::HashSet;

fn main() {
    let a: HashSet<i32> = [1, 2].into_iter().collect();
    let b: HashSet<i32> = [2, 3].into_iter().collect();
    let c = a | b;
    println!("{}", c.len());
}
```

```text title="Abridged — rustc 1.98.0; the note's std source paths are left out"
error[E0369]: no implementation for `HashSet<i32> | HashSet<i32>`
 --> owned.rs:6:15
  |
6 |     let c = a | b;
  |             - ^ - HashSet<i32>
  |             |
  |             HashSet<i32>
  |
note: `HashSet<i32>` does not implement `BitOr`
```

`BTreeSet` refuses the same way (`E0369`, *`BTreeSet<i32>` does not implement `BitOr`*), and half-borrowing, `&a | b`, is `E0308`: *expected `&HashSet<i32>`, found `HashSet<i32>`*. Write both `&`s. The reason is ownership: an operator that took `a` and `b` by value would consume them, and the reference impl lets the result be built while both inputs stay usable — the program prints their lengths afterwards. If you *do* want to consume one set into another, that is `a.extend(b)`, which moves `b`'s members into `a` without cloning.

## Questions that build nothing

```rust
use std::collections::BTreeSet;

fn main() {
    let a = BTreeSet::from([1, 2, 3, 4]);
    let small = BTreeSet::from([3, 4]);
    println!("{}", small.is_subset(&a));                   // true
    println!("{}", a.is_superset(&small));                 // true
    println!("{}", a.is_subset(&a));                       // true
    println!("{}", small.is_subset(&a) && small != a);     // true
    println!("{}", a.is_disjoint(&BTreeSet::from([9])));   // true
}
```

[`is_subset` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.is_subset) is ⊆, not ⊂: every set is a subset of itself, and the empty set is a subset of everything. There is no proper-subset method; `is_subset && !=` is the spelling. [`is_disjoint` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.is_disjoint) is *the intersection is empty*, answered without making one. `contains(&x)` is membership, ∈.

## `insert` and `remove` answer with a `bool`

```rust
use std::collections::HashSet;

fn main() {
    let mut s: HashSet<i32> = HashSet::new();
    println!("{} {}", s.insert(7), s.insert(7));   // true false
    println!("{} {}", s.remove(&7), s.remove(&7)); // true false
    s.insert(8);
    println!("{:?} {:?}", s.take(&8), s.take(&8)); // Some(8) None
}
```

A second `insert` of a value already there returns `false` and changes nothing. Python's `s.add(x)` ignores a repeat just as silently, but returns `None`, so the question *was it new?* costs a separate `x in s` there and nothing here. `remove` is the same `bool` the other way; `take` hands the removed value back in an `Option`.

## What a set can hold

`HashSet<T>` needs `T: Eq + Hash`; `BTreeSet<T>` needs `T: Ord`. `f64` is neither, because `NaN != NaN` — equality is not an equivalence relation, so there is no hash consistent with it and no total order. The type `HashSet<f64>` can be named, and `HashSet::new()` makes one, but the first `insert` is refused:

```rust,compile_fail
use std::collections::HashSet;

fn main() {
    let mut s: HashSet<f64> = HashSet::new();
    s.insert(1.0);
    println!("{}", s.len());
}
```

```text title="Abridged — rustc 1.98.0, up to the closing summary lines"
error[E0599]: the method `insert` exists for struct `HashSet<f64>`, but its trait bounds were not satisfied
 --> f64.rs:5:7
  |
5 |     s.insert(1.0);
  |       ^^^^^^
  |
  = note: the following trait bounds were not satisfied:
          `f64: Eq`
          `f64: Hash`
```

The `BTreeSet<f64>` version is `E0277`, *the trait `Ord` is not implemented for `f64`*. The usual escape is to store `x.to_bits()`, a `u64` — and the program shows what that changes: `0.0 == -0.0` is `true`, yet as bit patterns they are two members, while two `NaN`s with the same bits become one. Bits give you a set of *representations*, not of numbers. [Every `HashMap` and `HashSet` error](../hashmap_errors/README.md#18-an-f64-key) has the same refusal for a map key, and [what a float stores](../../19_Numbers/what_a_float_stores/README.md) has the reason.

Python goes the other way: `{1, 1.0, True}` is a set of **one** member, because `1 == 1.0 == True` and all three hash alike, and which spelling survives is whichever arrived first. In Rust the question cannot come up: `1`, `1.0` and `true` are three types, and a set holds one type.

## Print through a `BTreeSet`

A `HashSet`'s iteration order comes from its hasher, and the default `RandomState` is seeded afresh for every process — so `println!("{:?}", set)` prints the same members in a different order from one run to the next. Every `HashSet` in this page's program is printed through `show`, which collects it into a `BTreeSet` first; sorting a `Vec` does the same job. When the order matters to the program itself, and not only to the printout, that is the reason to choose [`BTreeSet`](../sorted_collections/README.md) in the first place: O(log *n*) instead of O(1), and ordered at every moment.

## Chained symmetric difference: an odd number of sets

```rust
use std::collections::BTreeSet;

fn main() {
    let x = BTreeSet::from([0, 1, 2, 3, 4]);
    let y = BTreeSet::from([2, 3, 4]);
    let z = BTreeSet::from([2, 5]);
    println!("{:?}", &(&x ^ &y) ^ &z);   // {0, 1, 2, 5}
}
```

`2` is in all three sets and survives; `3` and `4` are in two and cancel. `^` is exclusive or applied member by member, so a chain keeps exactly the members that appear in an **odd** number of the sets. The program folds four sets — `{1, 2, 3} ^ {3, 4, 5} ^ {5, 6, 7} ^ {7, 8, 1}` — to `{2, 4, 6, 8}`, then counts how many sets hold each number in a `BTreeMap` and checks that the odd counts are the same set. (The fourth set prints as `{1, 7, 8}`: a `BTreeSet` has no memory of the order it was written in.) The Math library derives the same rule as [symmetric difference as XOR ↗](https://masiarek.github.io/math-learning-library/04_Sets/algebra_of_sets/index.html); the program also checks one De Morgan law, `a - (b | c) == (a - b) & (a - c)`, and `a ^ b == (a | b) - (a & b)` on its own sets.

## Building, comparing, filtering in place

- `==` is set equality: same members, order and repeats ignored. `HashSet::from([1, 2, 3]) == [3, 2, 1, 2].into_iter().collect()` is `true`.
- **Building:** `HashSet::from([..])` from an array, `collect()` from any iterator (that is `FromIterator`), `extend` to add many at once.
- **[`retain` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.retain)** is the in-place filter: `set.retain(|n| n % 2 == 0)` keeps the even members and drops the rest, with no second set.

## Deduplicating a `Vec`: two answers, two orders

```rust
use std::collections::HashSet;

fn main() {
    let visits = vec![3, 1, 1, 2, 3, 1];
    let mut seen: HashSet<i32> = HashSet::new();
    let first_seen: Vec<i32> = visits.iter().copied().filter(|n| seen.insert(*n)).collect();
    let mut sorted = visits.clone();
    sorted.sort();
    sorted.dedup();
    println!("{first_seen:?} {sorted:?}");   // [3, 1, 2] [1, 2, 3]
}
```

`seen.insert` returns `true` the first time a value arrives, so as a filter it keeps each value at its first position. `sort` then `dedup` gives the sorted order instead, with no set at all. The trap is `dedup` without the `sort`: it removes **adjacent** repeats only, so `[3, 1, 1, 2, 3, 1]` becomes `[3, 1, 2, 3, 1]`.

---

## The verified output

<!-- output:set_operations -->
*Verified output of [`set_operations.rs`](examples/set_operations.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Methods: borrow both sets, return a lazy iterator of &T
   a = {1, 2, 3, 4}   b = {3, 4, 5}
   a.union(&b) is a std::collections::hash::set::Union<'_, i32, std::hash::random::RandomState>
   a.union(&b)                    {1, 2, 3, 4, 5}
   a.intersection(&b)             {3, 4}
   a.difference(&b)               {1, 2}
   a.symmetric_difference(&b)     {1, 2, 5}
   b.difference(&a)               {5}   <- not the same question
   a.intersection(&b).count() = 2   (counted, nothing collected)

2. Operators on references: a new, owned set every time
   &a | &b is a std::collections::hash::set::HashSet<i32>
   &a | &b = {1, 2, 3, 4, 5}
   &a & &b = {3, 4}
   &a - &b = {1, 2}
   &a ^ &b = {1, 2, 5}
   a and b are still usable afterwards: 4 and 3 members
   BTreeSet has the same four: {1, 2}

3. Yes-or-no questions build nothing
   small.is_subset(&a)       = true
   a.is_superset(&small)     = true
   a.is_subset(&a)           = true   (subset, not proper subset)
   proper: small ⊂ a         = true
   empty.is_subset(&small)   = true
   a.is_disjoint(&b)         = false
   a.is_disjoint(&hs([9]))   = true
   a.contains(&5)            = false

4. insert and remove answer with a bool
   insert(7) = true   insert(7) again = false   len = 1
   remove(&7) = true   remove(&7) again = false   len = 0
   take(&8) = Some(8)   take(&8) again = None

5. The element type decides what a set can hold
   HashSet<T> needs T: Eq + Hash; BTreeSet<T> needs T: Ord.
   f64 is neither: NaN != NaN, so HashSet<f64>::insert does not compile.
   0.0 == -0.0 is true, but as to_bits() keys: 3 members from [0.0, -0.0, NaN, NaN]
   [1, 1, 1] -> {1}   (1, 1.0 and true are three types, not three spellings of one value)

6. Chained symmetric difference: members of an ODD number of sets
   {0, 1, 2, 3, 4} ^ {2, 3, 4} ^ {2, 5} = {0, 1, 2, 5}
   {1, 2, 3} ^ {3, 4, 5} ^ {5, 6, 7} ^ {1, 7, 8} = {2, 4, 6, 8}
   how many sets hold each: {1: 2, 2: 1, 3: 2, 4: 1, 5: 2, 6: 1, 7: 2, 8: 1}
   the odd counts: {2, 4, 6, 8}   same set: true

7. Two laws from the algebra of sets, checked on these sets
   a - (b | c) == (a - b) & (a - c)   true
   a ^ b == (a | b) - (a & b)         true

8. Building, comparing, filtering in place
   {1, 2, 3} == collect([3, 2, 1, 2]) is true   (order and repeats ignored)
   {1, 2}.extend([2, 3, 4]) -> {1, 2, 3, 4}
   retain(even)             -> {2, 4}

9. Deduplicating a Vec: two answers, two orders
   [3, 1, 1, 2, 3, 1]
   seen.insert as the filter -> [3, 1, 2]   first-seen order
   sort then dedup           -> [1, 2, 3]   sorted order
   dedup without the sort    -> [3, 1, 2, 3, 1]   adjacent repeats only
```
<!-- /output -->

## If you are coming from another language

- **Mathematics.** Every operation here is one from the [Math library's sets chapter ↗](https://masiarek.github.io/math-learning-library/04_Sets/what_is_a_set/index.html): ∪, ∩, ∖, △, ⊆, ∈. The [algebra of sets ↗](https://masiarek.github.io/math-learning-library/04_Sets/algebra_of_sets/index.html) states the laws — commutativity, De Morgan, symmetric difference as XOR, ⊆ as a partial order — that section 7 of the program checks on concrete sets. What Rust adds is the *cost* of each: a set in mathematics is not stored anywhere, and here the choice between an iterator and a new set is yours to make.
- **Python.** The operators are the same characters — `a | b`, `a & b`, `a - b`, `a ^ b` — and `a <= b` is `is_subset`, `a < b` a proper subset, which Rust has no operator for. In Python `a | b` builds a new set and leaves both alone, which is what `&a | &b` does here; Rust makes you write the `&`s because an owned `a | b` would consume the operands. Python's methods `a.union(b)` also build a set eagerly, where Rust's return an iterator. `s.add(x)` returns `None` where `insert` returns the `bool`; `{1, 1.0, True}` is one member in Python and three types in Rust. The Math library's [sets in Python ↗](https://masiarek.github.io/math-learning-library/04_Sets/python_sets/index.html) and the Python library's [a set is a hash table ↗](https://masiarek.github.io/python-learning-library/04_Names_and_Objects/a_set_is_a_hash_table/index.html) are the matching pages, including why a Python set iterates in the same order within one process and a Rust `HashSet` does not.
- **ABAP.** There is no set type and no set operators: a set is a `HASHED TABLE … WITH UNIQUE KEY table_line` (or `SORTED` for the `BTreeSet`), `INSERT … INTO TABLE` with `sy-subrc = 4` on a duplicate is `insert` returning `false`, and each operation is a loop — intersection is `LOOP AT a` with `line_exists( b[ table_line = … ] )`, difference the same with `NOT`. The ABAP library's [sets in ABAP ↗](https://masiarek.github.io/abap-learning-library/03_Topics/sets_in_abap/index.html) writes the four out. What Rust gives you is the operation as a name, and the laws above as something you can rely on rather than re-derive in each loop.

## See also

- [`HashSet`](../the_hashset/README.md) — the four operations in a table, `difference`'s argument order, and the `len()` difference that counts ballots, not people
- [A first `HashSet`](../a_first_hashset/README.md) — `insert`, `contains`, `remove` and `take`, one printed step at a time
- [`BTreeMap` and `BTreeSet`](../sorted_collections/README.md) — the set that prints in order, and `Ord` instead of `Hash`
- [Every `HashMap` and `HashSet` error](../hashmap_errors/README.md) — the `f64` element and seventeen other refusals
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — why `union` costs nothing until you `collect`
- [Which duplicate survives ↗](https://masiarek.github.io/python-learning-library/04_Names_and_Objects/which_duplicate_survives/index.html) — the Python library: every de-duplication idiom by which copy it keeps and what order it returns, with this section's two answers as the Rust bridge
- [Operators are traits](../../12_Traits/operators_are_traits/README.md) — `BitOr`, `BitAnd`, `Sub`, `BitXor`, and why an impl on `&T` is a different impl
- [`std::collections::HashSet` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html) · [`std::collections::BTreeSet` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)

## Po polsku

Operacje na zbiorach mają w Ruscie dwie postaci i różnią się kosztem. Metody — `union` (suma, ∪), `intersection` (część wspólna, ∩), `difference` (różnica, ∖) i `symmetric_difference` (różnica symetryczna, △) — tylko pożyczają oba zbiory i zwracają leniwy iterator referencji; nic nie jest alokowane, dopóki nie wywołasz `collect`. Operatory `&a | &b`, `&a & &b`, `&a - &b`, `&a ^ &b` od razu budują nowy zbiór i klonują do niego elementy. Oba znaki `&` są obowiązkowe: dla dwóch zbiorów posiadanych (`a | b`) biblioteka standardowa nie ma implementacji i kompilator odmawia (`E0369`) — operator zabrałby oba argumenty na własność. Pytania „tak/nie” — `is_subset` (zawieranie ⊆, nie ostre ⊂), `is_superset`, `is_disjoint` (zbiory rozłączne), `contains` (należenie ∈) — niczego nie budują.

Zbiór potrafi przechować tylko typ, który umie porównać: `HashSet` wymaga `Eq + Hash`, `BTreeSet` wymaga `Ord`, więc `f64` (gdzie `NaN != NaN`) nie wejdzie do żadnego z nich. W Pythonie jest odwrotnie: `{1, 1.0, True}` to zbiór jednoelementowy, bo te trzy wartości są sobie równe. Kolejność przechodzenia po `HashSet` zmienia się przy każdym uruchomieniu programu, dlatego do wydruku i tam, gdzie porządek ma znaczenie, sięga się po `BTreeSet`. Łańcuch różnic symetrycznych zostawia elementy należące do **nieparzystej** liczby zbiorów — to XOR stosowany element po elemencie. A deduplikacja wektora ma dwie odpowiedzi: filtr `seen.insert` zachowuje kolejność pierwszego wystąpienia, `sort` + `dedup` daje kolejność posortowaną, a samo `dedup` usuwa tylko sąsiednie powtórzenia.

**Szukaj po polsku:** operacje na zbiorach w Ruscie · suma i część wspólna zbiorów · różnica symetryczna XOR · `rust HashSet union vs operator` · `rust HashSet<f64> nie kompiluje się`
