# Returning an iterator

**Level:** 201 → 301 · working knowledge

**One line:** `impl Iterator<Item = T>` is how a function hands back a chain it cannot name — and the two things that go wrong are a **lifetime** the opaque type has to capture and a **second branch** returning a different concrete type.

```rust
fn doubled_odds(scores: &[i32]) -> impl Iterator<Item = i32> {
    scores.iter().filter(|s| *s % 2 == 1).map(|s| s * 2)
}

fn main() {
    println!("{:?}", doubled_odds(&[5, 3, 0, 4, 2, 1]).collect::<Vec<_>>());   // [10, 6, 2]
    println!("{:?}", doubled_odds(&[5, 3, 0, 4, 2, 1]).take(1).collect::<Vec<_>>());   // [10]
}
```

Returning `Vec<i32>` instead would allocate, and do all the work, before the caller says it only wanted one. Returning the *chain* leaves that decision where it belongs.

## Why you cannot just name the type

The concrete return type above is

```text
Map<Filter<std::slice::Iter<'_, i32>, {closure}>, {closure}>
```

and there is no way to write that down: a closure has no name, so a type built out of two of them has no spelling. `impl Iterator<Item = i32>` is the promise without the spelling — still **one** concrete type, chosen by the body, resolved at compile time, no allocation and no virtual call.

Two consequences worth knowing. The caller cannot name it either, so it cannot go in a struct field without boxing or a generic parameter. And changing the body from `.filter().map()` to `.map().filter()` changes the hidden type — which is invisible and fine, until you also need it to be `Clone`, at which point the caller's requirement is on the hidden type rather than on your signature.

## The lifetime, and the edition that changed it

If the returned iterator borrows an argument, the opaque type has to capture that borrow. Whether you must say so depends on whether the lifetime is already visible:

```rust
// The borrow is in the item type. Fine in every edition.
fn names_over(rows: &[String], len: usize) -> impl Iterator<Item = &str> { … }

// The item type mentions no lifetime — but the iterator still holds `rows`.
fn lengths(rows: &[String]) -> impl Iterator<Item = usize> {
    rows.iter().map(|r| r.len())
}
```

The second one compiles here because this library builds with `--edition 2024`, where an opaque return type captures **every lifetime in scope**. Compiled with `--edition 2021`, the identical function is an error:

```text title="Real rustc output, --edition 2021"
error[E0700]: hidden type for `impl Iterator<Item = usize>` captures lifetime that does not appear in bounds
 --> scratch.rs:3:5
  |
2 | fn lengths(rows: &[String]) -> impl Iterator<Item = usize> {
  |                  ---------     --------------------------- opaque type defined here
  |                  |
  |                  hidden type `Map<std::slice::Iter<'_, String>, {closure}>` captures the anonymous lifetime defined here
3 |     rows.iter().map(|r| r.len())
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: add a `use<...>` bound to explicitly capture `'_`
  |
2 | fn lengths(rows: &[String]) -> impl Iterator<Item = usize> + use<'_> {
```

Both editions were checked. This matters when reading older code and older answers: a trailing `+ '_` on a returned `impl Iterator` is not decoration, it is the 2021 spelling of what 2024 now assumes — and the new `+ use<'a, T>` bound is how you *narrow* the capture again when the automatic one is too generous.

For a method handing out a view of its own data, the same rule applies to `&self`:

```rust
impl Roster {
    fn shouted(&self) -> impl Iterator<Item = String> + '_ {
        self.rows.iter().map(|r| r.to_uppercase())
    }
}
```

## The second branch, and the box

`impl Trait` means *one* hidden type. Two branches returning two chains is the same refusal as [two closures in one variable](../../23_Closures/what_a_closure_is/README.md):

```text title="Abridged — real rustc output, without the file-and-line header"
error[E0308]: `if` and `else` have incompatible types
  |
2 | /     if reverse {
3 | |         scores.iter().rev().copied()
  | |         ---------------------------- expected because of this
4 | |     } else {
5 | |         scores.iter().copied()
  | |         ^^^^^^^^^^^^^^^^^^^^^^ expected `Copied<Rev<Iter<'_, i32>>>`, found `Copied<Iter<'_, i32>>`
  |
help: you could change the return type to be a boxed trait object
  |
1 + fn maybe_reversed(scores: &[i32], reverse: bool) -> Box<dyn Iterator<Item = i32>> {
```

rustc names the fix. `Box<dyn Iterator<Item = i32> + '_>` erases both types into one, at the cost of an allocation and a virtual call per `next`. Three ways out, in the order to try them:

1. **Make the branches the same type** — often `.rev()` on both sides with a flag, or `either` of two `.filter()`s with the predicate doing the work.
2. **Box it.** Correct, obvious, and the cost is real but small next to any actual work per item.
3. **Return an enum implementing `Iterator`** — the `Either` pattern, which is what the `itertools` crate's `Either` is for. Static dispatch, no allocation, more code.

## On the way *in*, ask for `IntoIterator`

The mirror-image decision, and the one people get backwards:

```rust
fn total(rows: impl IntoIterator<Item = i32>) -> i32 {
    rows.into_iter().sum()
}

total(vec![1, 2, 3]);   // 6
total([1, 2, 3]);       // 6
total(1..=3);           // 6
```

A parameter of `impl Iterator` would refuse the first two and make every caller write `.into_iter()`. It is the same rule as [taking the loosest closure bound](../../23_Closures/three_closure_traits/README.md): a tighter bound buys the callee nothing and costs callers.

## If you are coming from another language

- **Python.** A generator function returns a generator, and its "type" is not something you write down either — so the instinct transfers directly, including the reason: the object's identity comes from the code, not from a nameable class. What Python does not make you decide is `impl` versus `Box<dyn>`, because every Python callable is already the boxed version — one heap object, dynamic dispatch, always. Rust's default is the opposite, and the box is what you reach for when you need Python's flexibility. And there is no lifetime question at all in Python: a generator holding `rows` keeps `rows` alive by reference-counting it, where Rust makes the borrow part of the signature so the caller cannot drop the data underneath it.
- **ABAP.** There is no way to return a lazy sequence, so the honest translation is *"return the internal table"* — which is the `Vec<i32>` version this page is arguing against, complete with the cost: the whole table is built before the caller sees any of it, and `LOOP ... EXIT` after one row has already paid for all of them. The nearest ABAP shape is a class with `has_next`/`get_next` returned as an interface reference, which is exactly `Box<dyn Iterator>` — object, virtual call, allocation. So an ABAP developer reading this page has the boxed version as their *only* option and can read `impl Iterator` as the free one that ABAP has no equivalent of.
- **C++.** `auto f() { return v | std::views::filter(...) | std::views::transform(...); }` is the same idea with `auto` doing what `impl Trait` does, and C++20 ranges have the identical dangling problem — a view over a temporary — with none of the compile-time protection. The E0700 above is precisely the diagnostic C++ does not give you.

---

## The verified output

<!-- output:returning_an_iterator -->
*Verified output of [`returning_an_iterator.rs`](examples/returning_an_iterator.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Return the chain, not the collection
   doubled_odds(&scores) -> [10, 6, 2]
   The caller decides what to do with it — and can stop early:
   .take(1)              -> [10]
   Returning Vec<i32> instead would have allocated, and done all
   the work, before the caller said it only wanted one.

2. Why the signature is `impl Iterator` and not the real type
   the real type is Map<Filter<Iter<'_, i32>, {closure}>, {closure}>
   and you cannot write that down: a closure has no name, so the
   type has no spelling. `impl Iterator<Item = i32>` is the promise
   without the spelling — one concrete type, chosen by the body.

3. Borrowing: the returned iterator holds the input
   names_over(&rows, 3)  -> ["Bernadette", "Cara"]
   Item = &str, so the borrow is visible in the signature and every
   edition accepts it.
   lengths(&rows)        -> [3, 10, 4]
   Item = usize, which mentions no lifetime at all — yet the iterator
   still holds `rows`. Edition 2024 captures every lifetime in scope,
   so this compiles as written. Under --edition 2021 the SAME function
   is E0700, "hidden type captures lifetime that does not appear in
   bounds", and wants `+ use<'_>` spelled out. Both were checked.

4. Two branches, two types — which `impl Trait` cannot express
   maybe_reversed(.., false) -> [5, 3, 0, 4, 2, 1]
   maybe_reversed(.., true)  -> [1, 2, 4, 0, 3, 5]
   `Rev<Copied<Iter<i32>>>` and `Copied<Iter<i32>>` are different
   types, so an `impl Iterator` return is E0308 — the same refusal
   as two closures in one variable. `Box<dyn Iterator>` erases both,
   for one allocation and a virtual call per `next`.

5. Owning, so no lifetime at all
   counted(5)            -> [1, 4, 9, 16, 25]
   `(1..=n)` owns its state, so nothing is borrowed and the
   signature stays bare. This is the version to prefer when you can.

6. A method handing out a view of its own rows
   roster.shouted()      -> ["ADA", "BERNADETTE", "CARA"]
   and the roster is still readable afterwards: 3 rows

7. On the way IN, take IntoIterator rather than Iterator
   total(vec![1, 2, 3])  -> 6
   total([1, 2, 3])      -> 6
   total(1..=3)          -> 6
   A parameter of `impl Iterator` would refuse the first two, and
   make every caller write `.into_iter()`. Loosest bound wins here
   exactly as it does for closures.
```
<!-- /output -->

---

## See also

- [Implementing `Iterator`](../implementing_iterator/README.md) — writing the concrete type this page is hiding
- [Iterators are lazy](../iterators_are_lazy/README.md) — why handing back the chain is worth the trouble
- [Returning a trait](../../12_Traits/returning_a_trait/README.md) — the same `impl Trait` / `Box<dyn Trait>` decision, in general
- [What a closure is](../../23_Closures/what_a_closure_is/README.md) — the unnameable type that makes the chain unnameable
- [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) — the `'_` and `use<'_>` above, and when to stop avoiding them
- [`iter`, `iter_mut`, `into_iter`](../iter_iter_mut_into_iter/README.md) — what a method like `shouted()` is a variant of

## Sources

[`impl Trait` in return position ↗](https://doc.rust-lang.org/reference/types/impl-trait.html) in the reference, and the edition guide on [RPIT lifetime capture rules ↗](https://doc.rust-lang.org/edition-guide/rust-2024/rpit-lifetime-capture.html) for the 2021/2024 difference measured above.

## Po polsku

Funkcja może oddać cały łańcuch iteratorów, ale nie potrafi nazwać jego typu: `Map<Filter<slice::Iter<'_, i32>, {closure}>, {closure}>` zawiera dwa domknięcia (*closures*), a domknięcie nie ma nazwy — więc typ z niego zbudowany nie ma zapisu. `impl Iterator<Item = i32>` jest właśnie obietnicą bez zapisu: wciąż **jeden** konkretny typ, wybrany przez ciało funkcji i rozstrzygnięty podczas kompilacji, bez alokacji i bez wywołania wirtualnego. Gdyby zamiast tego zwrócić wektor `Vec<i32>`, cała praca zostałaby wykonana i zaalokowana, zanim wywołujący zdąży powiedzieć `.take(1)`. Cena jest taka, że nazwy nie zna też wywołujący — bez `Box` albo parametru generycznego taki iterator nie wejdzie do pola struktury.

Pierwsza pułapka to czas życia (*lifetime*). Jeśli zwrócony iterator pożycza argument, ukryty typ musi to pożyczenie przechwycić, i tędy przebiega granica między edycjami. Pod `--edition 2024`, na której zbudowana jest ta biblioteka, `impl Iterator` przechwytuje **każdy** czas życia widoczny w sygnaturze, więc `fn lengths(rows: &[String]) -> impl Iterator<Item = usize>` kompiluje się bez żadnego dopisku — mimo że w typie elementu (`usize`) żadnej referencji nie widać. Ta sama funkcja pod `--edition 2021` to `E0700`, „hidden type … captures lifetime that does not appear in bounds”. Warto to wiedzieć, zanim zacznie się czytać starsze odpowiedzi na Stack Overflow, gdzie na końcu każdej takiej sygnatury stoi `+ '_`: to nie ozdobnik, tylko zapis z 2021 roku tego, co 2024 zakłada domyślnie. Nowsza forma `+ use<'a, T>` służy do rzeczy odwrotnej — do **zawężenia** przechwytywania, kiedy automatyczne okazuje się zbyt hojne.

Druga pułapka: `impl Trait` znaczy *jeden* ukryty typ, więc dwie gałęzie `if`/`else` zwracające dwa różne łańcuchy kończą się błędem `E0308` — „`if` and `else` have incompatible types”, bo `Copied<Rev<Iter<'_, i32>>>` i `Copied<Iter<'_, i32>>` to naprawdę różne typy. `rustc` sam podpowiada obiekt cechy (*trait object*): `Box<dyn Iterator<Item = i32> + '_>` skleja obie gałęzie w jedno, płacąc alokacją i wywołaniem wirtualnym przy każdym `next`. Kolejność prób jest taka: najpierw sprowadzić obie gałęzie do tego samego typu, potem `Box`, a dopiero na końcu własne wyliczenie implementujące `Iterator` (wzorzec `Either` znany z `itertools`). Na wejściu reguła jest odwrotna i to ją najczęściej się myli — parametr powinien mieć postać `impl IntoIterator<Item = i32>`, bo `impl Iterator` odrzuci i `vec![1, 2, 3]`, i `[1, 2, 3]`, zmuszając każdego wywołującego do dopisania `.into_iter()`.

**Szukaj po polsku:** typ zwracany `impl Trait` · obiekt cechy · domknięcie nie ma nazwy · `rust impl Iterator return type` · `rust E0700 capture lifetime` · `rust Box dyn Iterator two branches`
