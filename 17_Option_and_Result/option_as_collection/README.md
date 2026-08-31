# `Option` is a one-item collection

**Level:** 201 · working knowledge

**One line:** An `Option` is a container that holds either zero items or one — so it is iterable, and the entire `Iterator` toolbox already works on it.

This page answers two questions that always arrive together: *is `Option` iterable?* and *are there numbers behind `Some` and `None`?* Both answers are yes, and the second comes with a catch worth understanding.

---

## Yes, it iterates

```rust
for i in Some(5) {
    println!("{i}");   // prints 5
}

for i in None::<i32> {
    println!("{i}");   // never runs
}
```

`Option<T>` implements `IntoIterator`, yielding exactly one item for `Some` and none for `None`. Once you see it as "a `Vec` that can never hold more than one thing", a lot of library design stops looking arbitrary.

**But don't actually write that loop.** rustc has a lint for it, and the lint is right:

```text
warning: for loop over an `Option`. This is more readably written as an `if let` statement
```

A `for` over an `Option` runs at most once — it is an [`if let`](../if_let/README.md) wearing a loop's clothing, and a reader has to look twice to be sure no iteration is happening. The example keeps the loops (behind an `allow`) because they *prove* the impl exists. Where that impl actually earns its keep is everything below, where an `Option` is handed to an adapter rather than looped over directly.

That is why all of this already works, with no conversion step:

| You write | Because |
|---|---|
| [`iter.flatten()` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten) over `Option`s | each `Option` is itself iterable, so flattening drops the `None`s |
| [`vec.extend(Some(3))` ↗](https://doc.rust-lang.org/std/iter/trait.Extend.html#tymethod.extend) | `extend` takes any `IntoIterator` |
| [`iter.chain(Some(9))` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain) | same reason |
| [`iter.filter_map(f)` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) | it is literally `map` followed by `flatten` |

And the reverse trick, which is the one people are most pleased to discover: **a collection of `Option`s collects into a single `Option` of a collection.**

```rust
let all: Option<Vec<i32>> = vec![Some(1), Some(2), Some(3)].into_iter().collect();  // Some([1, 2, 3])
let any: Option<Vec<i32>> = vec![Some(1), None,    Some(3)].into_iter().collect();  // None
```

All-or-nothing, and it short-circuits — nothing after the first `None` is even visited. `Result` collects the same way, which is how you validate a whole batch and get back either every parsed value or the first error.

## Yes, there are numbers — and no, you may not have them

`None` is variant 0 and `Some` is variant 1. That is not folklore; you can print it:

```rust
std::mem::discriminant(&None::<i32>)   // Discriminant(0)
std::mem::discriminant(&Some(1))       // Discriminant(1)
```

The ordering that follows is guaranteed and useful: `None < Some(anything)`, because the derived `Ord` follows declaration order and `None` is declared first. Sorting a `Vec<Option<T>>` puts the absent ones first, and `.max()` over `Option`s prefers a present value.

But you cannot extract those numbers. `None as i32` does not compile — a cast to an integer is only allowed for a *fieldless* enum, and `Some(T)` carries data. The `Discriminant(0)` text above is a `Debug` courtesy, not an API; `discriminant` values can only be compared to each other.

**And the deeper reason it is not part of the contract:** the tag is often not stored at all.

```
i32                        4 bytes
Option<i32>                8
Box<i32>                   8
Option<Box<i32>>           8      <- free
Option<Option<Box<i32>>>  16      <- the free lunch runs out
bool                       1
Option<bool>               1      <- also free
```

Where the inner type has a bit pattern it can never legally hold — a *niche* — `None` takes that pattern and costs nothing. A `Box` is never null, so null means `None`. A `bool` uses 2 of its 256 byte values, so one of the spare 254 means `None`. Wrap it twice and the niche is spent, so `Option<Option<Box<i32>>>` grows to 16 bytes and pays for a real tag.

That is the whole reason `Option` is free where a hand-rolled "-1 means missing" sentinel is not: the compiler finds the impossible value for you, and proves nothing else can produce it.

## Asking without unwrapping

`is_some_and` is the one worth adding to your vocabulary — it replaces the `x.is_some() && x.unwrap() > 1` you were about to write:

```rust
pub fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool
```

Note it takes `self`, not `&self`: it *consumes* the option. Fine for a `Copy` inner type, but for an owned `String` you will want `.as_ref().is_some_and(…)` so the original survives. Its mirror `is_none_or` completes the pair, and `map_or(default, f)` is the same shape when you want a value rather than a `bool`.

## `take()` — moving out of something you only borrow

```rust
let mut slot = Some(String::from("first"));
let got = slot.take();        // got = Some("first"), slot = None
let old = slot.replace(…);    // swaps, handing back what was there
```

This is the answer to a borrow-checker problem you will certainly hit: you have `&mut self` and need to move a non-`Copy` value *out* of a field. You cannot — that would leave the struct half-empty. `take()` resolves it by leaving a valid `None` behind, which is why the standard library reaches for it constantly in linked structures and state machines.

It is also the one place `Option` earns its keep purely as a *mechanism* rather than as a description of your data.

---

## Practice

**No `match` allowed.** Given `Vec<Option<u8>>` — one entry per registered voter, `None` where no ballot came back — report how many ballots were cast, their total, and their mean, using only iterator methods. Then move a `String` out of an `Option` field you have only a `&mut` to.

Reach for `flatten` before `filter_map`, and notice that neither one needed you to say what `None` means — the empty container simply contributes nothing. For the second half, try a plain move out of the borrowed field first and read what the borrow checker wants instead.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:option_as_collection_kata -->
*[`option_as_collection_kata.rs`](examples/option_as_collection_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: count the ballots that exist, without writing a match.
//!
//!   rustc --edition 2024 option_as_collection_kata.rs -o /tmp/oack && /tmp/oack

fn main() {
    // One entry per registered voter; None means the ballot never came back.
    let returned: Vec<Option<u8>> = vec![Some(5), None, Some(3), Some(0), None];

    // An Option is a container of zero or one item, so `flatten` drops the
    // empty ones exactly as it would for nested Vecs.
    let total: u32 = returned.iter().flatten().map(|s| *s as u32).sum();
    let counted = returned.iter().flatten().count();

    println!("Iterating an Option like the one-item collection it is:");
    println!("  entries       -> {}", returned.len());
    println!("  ballots cast  -> {counted}");
    println!("  score total   -> {total}");
    println!("  mean of cast  -> {:.2}", total as f64 / counted as f64);

    // filter_map is the same move with the transform folded in.
    let doubled: Vec<u32> = returned.iter().filter_map(|s| s.map(|n| n as u32 * 2)).collect();
    println!("  doubled       -> {doubled:?}");

    println!("\nThe zero-or-one shape shows up directly, too:");
    println!("  Some(5).iter().count() -> {}", Some(5).iter().count());
    println!("  None::<u8>.iter().count() -> {}", None::<u8>.iter().count());
    println!("  Some(5).into_iter().chain(None).collect::<Vec<_>>() -> {:?}",
        Some(5).into_iter().chain(None).collect::<Vec<_>>());

    // take(): move the value out of a field you only have a &mut to, leaving
    // None behind. The borrow checker would refuse a plain move.
    let mut pending: Option<String> = Some("Cara's late ballot".to_string());
    let collected = collect_ballot(&mut pending);
    println!("\ntake() — moving out of something you only borrow:");
    println!("  collected -> {collected:?}");
    println!("  pending   -> {pending:?}   (left as None, not a copy)");
    println!("  again     -> {:?}", collect_ballot(&mut pending));
}

fn collect_ballot(slot: &mut Option<String>) -> Option<String> {
    slot.take()
}
```
<!-- /source -->

<!-- output:option_as_collection_kata -->
*Verified output of [`option_as_collection_kata.rs`](examples/option_as_collection_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Iterating an Option like the one-item collection it is:
  entries       -> 5
  ballots cast  -> 3
  score total   -> 8
  mean of cast  -> 2.67
  doubled       -> [10, 6, 0]

The zero-or-one shape shows up directly, too:
  Some(5).iter().count() -> 1
  None::<u8>.iter().count() -> 0
  Some(5).into_iter().chain(None).collect::<Vec<_>>() -> [5]

take() — moving out of something you only borrow:
  collected -> Some("Cara's late ballot")
  pending   -> None   (left as None, not a copy)
  again     -> None
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:option_as_collection -->
*Verified output of [`option_as_collection.rs`](examples/option_as_collection.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: Yes — you can `for` over an Option (and still shouldn't)
  for i in Some(5)       -> body ran, i = 5
  for i in None::<i32>   -> body never ran
  Some(5).iter().count()     = 1
  None::<i32>.iter().count() = 0
      An Option is a Vec that can never hold more than one thing.
      rustc warns on the loops above — 'more readably written as an
      if let' — and it is right. The impl earns its keep in Step 2.

──── Step 2: So every iterator adapter already works on it
  flatten     [Some(1), None, Some(3)] -> [1, 3]
  extend      [1,2] + Some(3) + None   -> [1, 2, 3]
  filter_map  ["1", "x", "3"] -> [1, 3]
  chain       [1,2].chain(Some(9))     -> [1, 2, 9]
      filter_map is just map + flatten: the None rows drop out on their own.

──── Step 3: The reverse: collect a pile of Options into ONE Option
  every Some -> Some([1, 2, 3])
  one None   -> None
      All-or-nothing, and it short-circuits: nothing after the first None is visited.
      Result collects the same way, which is how you validate a whole batch at once.

──── Step 4: Are there numbers behind Some and None?
  None::<i32> < Some(0)  = true
  Some(1) < Some(2)      = true
  discriminant(&None) == discriminant(&Some(1)) = false
  discriminant(&None::<i32>) prints as Discriminant(0)
  discriminant(&Some(1))     prints as Discriminant(1)
      There ARE numbers: None is variant 0, Some is variant 1 — declaration order.
      But you cannot get at them. `None as i32` does NOT compile: a cast is only
      allowed for a fieldless enum, and Some(T) carries data. The Debug text above
      is a courtesy, not an API — compare the sizes in the next step.

──── Step 5: …and the tag is not always even stored
  i32                        4 bytes
  Option<i32>                8
  Box<i32>                   8
  Option<Box<i32>>           8
  Option<Option<Box<i32>>>  16
  bool                       1
  Option<bool>               1
      Where the inner type has an unused bit pattern (a 'niche'), None takes it
      and the tag costs nothing. That is why the number is not part of the API.

──── Step 6: Asking a question without unwrapping
  Some(2).is_some()              = true
  Some(2).is_none()              = false
  Some(2).is_some_and(|n| n > 1) = true
  Some(0).is_some_and(|n| n > 1) = false
  None.is_some_and(|n| n > 1)    = false
  Some(2).map_or(0, |n| n * 10)  = 20
  None.map_or(0, |n| n * 10)     = 0
      is_some_and replaces the `x.is_some() && x.unwrap() > 1` you were about to write.

──── Step 7: take() and replace(): moving out of a field you only borrow
  after take()     got = Some("first"), slot = None
  after replace()  old = None, slot = Some("second")
      take() swaps in None and hands you the value. It is the standard way to
      move a non-Copy value out of a &mut, which the borrow checker otherwise refuses.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 17_Option_and_Result/option_as_collection/examples/option_as_collection.rs -o /tmp/oac && /tmp/oac
```

## See also

- [`Option` vs `Result`](../option_vs_result/README.md) — which of the two you should be reaching for in the first place
- [`Option` fields](../option_fields/README.md) — `Option` in a type definition rather than a return type
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — the same adapters over a sequence longer than one, and when they actually run
- [`std::mem::discriminant` ↗](https://doc.rust-lang.org/std/mem/fn.discriminant.html) and [`Option::is_some_and` ↗](https://doc.rust-lang.org/core/option/enum.Option.html#method.is_some_and)

## Po polsku

Polskie materiały przedstawiają `Option` (*Opcja*) przede wszystkim jako bezpieczny zamiennik `null`-a — to prawda, ale gubi sedno tej strony. `Option<T>` jest **pojemnikiem na zero albo jeden element**: to wyliczenie implementujące `IntoIterator`, więc cały zestaw adapterów z `Iterator` działa na nim bez żadnej konwersji. `flatten` wyrzuca `None`-y dokładnie tak, jak spłaszcza zagnieżdżone wektory, `extend(Some(3))` i `chain(Some(9))` przyjmują `Option` wprost, a `filter_map` to po prostu `map` plus `flatten`. Sama pętla `for i in Some(5)` też się kompiluje i to jest mała pułapka — rustc ostrzega *„for loop over an `Option`. This is more readably written as an `if let` statement”* i ma rację: taka pętla wykona się najwyżej raz, czyli jest `if let`em w przebraniu. Implementacja zarabia na siebie tam, gdzie `Option` **podajesz** adapterowi, a nie tam, gdzie iterujesz po nim ręcznie.

W drugą stronę działa sztuczka, która cieszy najbardziej: kolekcja `Option`ów zbiera się w jeden `Option` kolekcji. `vec![Some(1), Some(2), Some(3)]` zebrane przez `collect()` daje `Some([1, 2, 3])`, a jeden `None` w środku daje `None` dla całości — wszystko albo nic, w dodatku z przerwaniem na pierwszym `None`, bo nic za nim nie jest już odwiedzane. `Result` zbiera się identycznie i to jest w Ruscie standardowy sposób na sprawdzenie całej paczki danych naraz: albo wszystkie sparsowane wartości, albo pierwszy błąd.

Czy pod `Some` i `None` są liczby? Są: `None` to wariant 0, `Some` to wariant 1, według kolejności deklaracji — dlatego wyprowadzone `Ord` daje `None < Some(cokolwiek)` i sortowanie wypycha braki na początek. Wyciągnąć ich jednak nie można: `None as i32` się nie kompiluje, bo rzutowanie na liczbę wolno zrobić tylko dla wyliczenia bez pól, a `Some(T)` niesie dane. Głębszy powód jest ciekawszy — znacznik wariantu (*discriminant*) często w ogóle nie jest przechowywany. Jeśli typ wewnętrzny ma wzorzec bitowy, którego nigdy legalnie nie przyjmie (*niche*, nisza), to `None` zajmuje właśnie ten wzorzec i nie kosztuje nic: `Box<i32>` nigdy nie jest zerowy, więc `Option<Box<i32>>` ma te same 8 bajtów co goły `Box`, a `Option<bool>` mieści się w jednym bajcie. Zawiń dwa razy i nisza się kończy — `Option<Option<Box<i32>>>` puchnie do 16 bajtów i płaci za prawdziwy znacznik. Właśnie dlatego `Option` bywa darmowy tam, gdzie ręcznie wpisany wartownik w stylu „−1 znaczy brak” darmowy nie jest.

Na koniec jedna metoda czysto mechaniczna: `take()`. Gdy masz `&mut self` i chcesz **wyjąć** z pola wartość, która nie jest `Copy`, kompilator odmawia — `error[E0507]: cannot move out of ... which is behind a mutable reference` — bo struktura zostałaby w połowie pusta. `slot.take()` zostawia w polu poprawne `None` i oddaje ci zawartość, a `replace()` robi to samo, podmieniając na nową wartość; biblioteka standardowa sięga po to stale w maszynach stanów i strukturach wiązanych. Przy okazji uwaga na `is_some_and`: bierze `self`, a nie `&self`, więc dla `Option<String>` napisz `.as_ref().is_some_and(…)`, żeby oryginał przeżył wywołanie.

**Szukaj po polsku:** Option jako kolekcja zero lub jeden · wyliczenie Option w Ruscie · optymalizacja niszy · `rust option iterator flatten` · `rust collect vec of options into option of vec` · `rust option take cannot move out of borrowed content`
