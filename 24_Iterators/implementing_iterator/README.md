# Implementing `Iterator`

**Level:** 201 → 301 · working knowledge

**One line:** Write `next` and an `Item` type, and seventy-five other methods arrive — but implement `Iterator` on a *collection* and you have built something that empties itself the first time anybody reads it.

```rust
struct Countdown { n: u32 }

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.n == 0 { return None; }
        self.n -= 1;
        Some(self.n + 1)
    }
}

fn main() {
    println!("{:?}", Countdown { n: 5 }.collect::<Vec<_>>());              // [5, 4, 3, 2, 1]
    println!("{}", Countdown { n: 5 }.sum::<u32>());                       // 15
    println!("{:?}", Countdown { n: 9 }.filter(|n| n % 2 == 1).take(3).collect::<Vec<_>>());  // [9, 7, 5]
}
```

`sum`, `filter`, `take`, `zip`, `max`, `last` and the `for` loop were not written for `Countdown`. They are default methods on the trait, all of them built out of the one method above.

## What the trait requires

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ...and seventy-five methods with default bodies
}
```

In 1.98.0's source, `Iterator` declares **76 methods** and exactly one of them — `next` — has no body. `None` means finished; after that a well-behaved iterator keeps returning `None`, though nothing enforces it (that promise is what the separate `FusedIterator` marker is for).

## Three things that do not arrive for free

**`rev()` needs `DoubleEndedIterator`.** Reversing means pulling from the far end, and `next` cannot do that — you must also write `next_back`:

```text title="Abridged — real rustc output, without the file-and-line headers"
error[E0277]: the trait bound `Countdown: DoubleEndedIterator` is not satisfied
   |
11 |     let v: Vec<u32> = Countdown { n: 3 }.rev().collect();
   |                                          ^^^ unsatisfied trait bound
   |
note: required by a bound in `rev`
     |
3445 |         Self: Sized + DoubleEndedIterator,
```

**`len()` needs `ExactSizeIterator`**, for the same reason: `next` never claims to know how many are left.

**`size_hint` is optional and costs real allocations.** Its default is `(0, None)` — *"somewhere between zero and unknown"* — and `collect` believes it, so the `Vec` grows by doubling:

```text
default size_hint()  (0, None)    collect capacity 16
size_hint written    (9, Some(9)) collect capacity 9
```

Nine items either way. One of them reallocated its way to a capacity of 16; the other made a single allocation of exactly nine. `size_hint` is three lines and it is the first thing to add once a custom iterator is used in anger.

## The mistake: implementing `Iterator` on a collection

It is a tempting shortcut — the type already knows its rows, so give it a `next` and a cursor field, and `for row in collection` starts working. Then:

```text
first pass over the collection:  [5, 3, 0]
second pass over the SAME value: []
```

`next` takes `&mut self` and there is no rewind, so **an iterator is single-use by construction**. Make your collection an iterator and you have inherited that: it can be read once, `for row in collection` consumes it, and the cursor is now a field somebody has to reset. Nobody wants a `Vec` that empties itself when you look at it.

Note where the flaw actually is. It is not that the cursor is hard to reset; it is that *iterating* and *being a sequence* are different jobs, and one value doing both can only do the second once.

## What std does instead

`Vec` does not implement `Iterator`. It implements `IntoIterator` three times, and hands out a separate iterator type per kind of access. Copy that shape:

```rust
struct Roster { rows: Vec<String> }

impl Roster {
    fn iter(&self) -> std::slice::Iter<'_, String> { self.rows.iter() }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, String> { self.rows.iter_mut() }
}

impl IntoIterator for Roster {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter { self.rows.into_iter() }
}

impl<'a> IntoIterator for &'a Roster {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter { self.rows.iter() }
}

impl<'a> IntoIterator for &'a mut Roster {
    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;
    fn into_iter(self) -> Self::IntoIter { self.rows.iter_mut() }
}
```

That is the whole pattern, and for a type wrapping a `Vec` every body is a one-liner delegating to the slice iterators. What it buys: all three `for` spellings work, [`iter`/`iter_mut`/`into_iter`](../iter_iter_mut_into_iter/README.md) mean what they mean everywhere else, every adapter is available on the borrow as well as on the value — and the roster is still there afterwards.

Writing the iterator struct *by hand* — a `next` that walks your own nodes — only becomes necessary when there is no slice underneath. That is the case for a tree, a graph, or a linked list, and it is where a hand-rolled iterator earns the lifetime annotations the delegating version got for free.

## The lifetime in the signature is load-bearing

`fn iter(&self) -> std::slice::Iter<'_, String>` — the `'_` ties the returned iterator to the borrow of `self`, which is what stops the iterator outliving the data it reads:

```text title="Real rustc output"
error[E0515]: cannot return value referencing local variable `v`
 --> scratch.rs:3:5
  |
3 |     v.iter()
  |     -^^^^^^^
  |     |
  |     returns a value referencing data owned by the current function
  |     `v` is borrowed here
  |
  = help: use `.collect()` to allocate the iterator
```

The help line is the whole decision, stated in six words: return a **borrow** tied to data the caller owns, or return **owned** data. A function that builds a `Vec` locally and wants to hand back its contents has to choose the second — `collect()` into a `Vec<String>`, or return the `Vec` itself and let the caller call `iter()`.

This is also the reason a borrowing iterator over your own type needs a lifetime parameter (`struct Iter<'a, T>`) while the by-value one does not: only the borrowing one contains a reference. If the annotations feel arbitrary, [how to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) is the scaffold — and this is one of the few places where the scaffold's advice, *don't put references in structs*, has to be set aside, because a borrowing iterator is exactly a struct holding a reference.

## If you are coming from another language

- **Python.** The protocol is the same shape and the vocabulary maps one-for-one: `__next__` is `next`, `StopIteration` is `None`, and `__iter__` is `IntoIterator::into_iter`. Python even has the same two-role split — *iterable* versus *iterator* — and the classic Python bug is the one this page is about: a class whose `__iter__` does `return self` is a one-shot iterable, which is why a file object cannot be looped over twice and a list can. Rust makes the same distinction structurally rather than by convention, and gives you a compile error instead of a silently empty second loop. Two things Python has no counterpart for: `iter_mut()`, since Python iteration never lends you a writable slot, and `size_hint`, though `__length_hint__` exists for exactly the same optimization and is just as optional. And a Rust iterator is not a generator — there is no `yield` in stable Rust, so the state that a Python generator keeps on its frame becomes fields on your struct, which is precisely what `Countdown { n }` is.
- **ABAP.** There is no iterator protocol in the language, so `LOOP AT` is not built on anything you can implement — you cannot make your own class loopable, and the standard workaround is an iterator *object* in the Gang-of-Four sense: a class with `has_next` and `get_next`, usually reached through an interface. That is `next` split into two calls, and the split is the source of its problems: two calls means two chances to get the order wrong, and the "no more rows" answer is separate from the row itself. Rust folds both into one `Option<Item>`, which is why the `while` loop cannot be written incorrectly — there is no state to check between the test and the read. If you have used `cl_object_collection`'s iterator or written a `zif_iterator`, you have written the awkward version of this trait; what Rust adds is that implementing it once makes 75 further methods work on your type, where in ABAP each of `map`, `filter` and `sum` remains a `LOOP` somebody writes again.
- **Java / C#.** `Iterable`/`Iterator` and `IEnumerable`/`IEnumerator` are the same two-role split, for the same reason, and both languages make the same recommendation about not conflating them. The difference is what you get for implementing it: Java hands you the enhanced `for` and nothing else until you call `.stream()`, C# hands you LINQ through extension methods, and Rust puts all 75 methods on the trait itself — so there is no second interface to opt into and no `.stream()` step.

---

## The verified output

<!-- output:implementing_iterator -->
*Verified output of [`implementing_iterator.rs`](examples/implementing_iterator.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Write `next`, and seventy-five more methods arrive
   Countdown { n: 5 }.collect()      [5, 4, 3, 2, 1]
   .sum()                            15
   .filter(odd).take(3).collect()    [9, 7, 5]
   .zip([a, b, c]).collect()         [(3, "a"), (2, "b"), (1, "c")]
   .max()  .last()                   Some(4) Some(1)
   for n in Countdown { n: 3 }        <- works too: every Iterator
     3     2     1

2. What the trait actually asked for
   type Item = u32;                     the element type
   fn next(&mut self) -> Option<Item>;  advance, or say you are done
   That is all. In 1.98.0's source, `Iterator` declares 76 methods
   (a handful still unstable) and exactly one of them — `next` — has
   no default body. Everything in section 1 is written in terms of it.

3. Three things that are NOT free
   .rev()   needs DoubleEndedIterator  (you must also write next_back)
   .len()   needs ExactSizeIterator
   Both are compile errors on Countdown, not runtime surprises.
   And the default size_hint costs allocations:
     default size_hint()  (0, None)    collect capacity 16
     size_hint written    (9, Some(9)) collect capacity 9
   Same nine items. Without the hint, collect grew the Vec by doubling
   and overshot to 16; with it, one allocation of exactly nine.

4. The mistake: implementing Iterator ON a collection
   first pass over the collection:  [5, 3, 0]
   second pass over the SAME value: []
   An iterator is single-use by construction — `next` takes &mut self
   and there is no rewind. So a collection that IS an iterator can be
   read once, and `for row in collection` consumes it. Nobody wants a
   Vec that empties itself when you look at it.

5. What std does instead: hand out an iterator per kind of access
   roster.iter().count()          3
   roster.iter().count() again    3
   after iter_mut:                ["Ada!", "Ben!", "Cara!"]
   for row in &roster             Ada! Ben! Cara!
   adapters work on the borrow:   longest = Some("Cara!")
   for row in roster (by value)   ["Ada!", "Ben!", "Cara!"]
   Three IntoIterator impls — for Roster, &Roster and &mut Roster —
   are what make all three `for` spellings work, and the collection
   itself stays re-readable because it never became the iterator.

6. The borrowing iterators carry a lifetime, and it is load-bearing
   fn iter(&self) -> std::slice::Iter<'_, String>
   The '_ ties the iterator to the borrow of the Roster, so the
   compiler refuses an iterator that outlives what it reads:
     error[E0515]: cannot return value referencing local variable `v`
     help: use `.collect()` to allocate the iterator
   That help line is the whole decision: hand back a borrow tied to
   data the caller owns, or hand back owned data.
```
<!-- /output -->

---

## See also

- [`iter`, `iter_mut`, `into_iter`](../iter_iter_mut_into_iter/README.md) — the three doors this page teaches a type of your own to open
- [Iterators are lazy](../iterators_are_lazy/README.md) — what the seventy-five free methods do with your `next`
- [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) — the `'a` on a borrowing iterator, and the one rule this page has to break
- [What a trait is](../../12_Traits/what_a_trait_is/README.md) — required versus provided methods, which is the mechanism behind the 76
- [`impl` blocks](../../16_Structs/impl_blocks/README.md) — where `iter()` and `iter_mut()` live on a type of your own
- [Returning a trait](../../12_Traits/returning_a_trait/README.md) — `impl Iterator<Item = T>` as a return type, once the concrete iterator gets ugly to name

## Sources

[`Iterator` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html) and [`IntoIterator` ↗](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) in std; the [`std::iter` ↗](https://doc.rust-lang.org/std/iter/index.html) module page has the *Implementing Iterator* walkthrough this one is a shorter, measured version of. The method count above is from the trait's own source, `library/core/src/iter/traits/iterator.rs`, in the pinned 1.98.0 toolchain.

## Po polsku

Wymagania cechy (*trait*) `Iterator` mieszczą się w dwóch linijkach: typ powiązany `type Item` i metoda `fn next(&mut self) -> Option<Self::Item>`. W źródłach wersji 1.98.0 `Iterator` deklaruje **76 metod** i dokładnie jedna z nich — `next` — nie ma ciała; pozostałe siedemdziesiąt pięć to metody domyślne, napisane w kategoriach tej jednej. Dlatego `sum`, `filter`, `take`, `zip`, `max`, `last` i pętla `for` działają na `Countdown` bez jednej dodatkowej linijki. `None` znaczy „koniec”, a to, że porządny iterator zwraca `None` już zawsze, jest zwyczajem, którego nikt nie egzekwuje — od deklarowania tego jest osobny znacznik `FusedIterator`.

Trzy rzeczy nie przychodzą za darmo i warto wiedzieć o nich, zanim się zdziwisz. `rev()` wymaga `DoubleEndedIterator`, czyli dopisania `next_back` — bez tego dostajesz `E0277` (*the trait bound `Countdown: DoubleEndedIterator` is not satisfied*). `len()` wymaga `ExactSizeIterator`, bo `next` nigdy nie twierdził, że wie, ile elementów zostało. A `size_hint` jest opcjonalne i **kosztuje realne alokacje**: domyślnie zwraca `(0, None)`, czyli „gdzieś od zera do nie wiadomo ile”, `collect` w to wierzy i powiększa wektor przez podwajanie — dziewięć elementów kończy z pojemnością **16**, podczas gdy przy napisanym `size_hint` zwracającym `(9, Some(9))` jest jedna alokacja na dokładnie dziewięć. To trzy linijki kodu i pierwsza rzecz do dopisania, gdy własny iterator trafia do prawdziwego użycia.

Główny błąd, przed którym ostrzega ta strona, ma w polskich materiałach gotową nazwę — tyle że przy innym języku: kursy Pythona wbijają do głowy różnicę między **obiektem iterowalnym** a **iteratorem**, i to jest dokładnie ta sama różnica. Iterowanie i bycie sekwencją to dwie różne role, a jedna wartość może pełnić drugą z nich tylko raz — `next` bierze `&mut self` i nie ma przewijania, więc **iterator jest z definicji jednorazowy**. Gdy zaimplementujesz `Iterator` na własnej kolekcji, dziedziczysz tę jednorazowość: pierwsze przejście daje `[5, 3, 0]`, drugie po tej samej wartości `[]`, a `for row in collection` konsumuje kolekcję. Nikt nie chce wektora, który opróżnia się od patrzenia. Zwróć uwagę, gdzie leży wada — nie w tym, że kursor trudno wyzerować, tylko w tym, że te dwie role zostały sklejone.

Wzorzec do skopiowania jest ten, który stosuje std: **`Vec` nie implementuje `Iterator`**, tylko `IntoIterator` — trzy razy, dla `Roster`, `&Roster` i `&mut Roster` — plus metody `iter()` i `iter_mut()`. Dla typu opakowującego wektor każde z tych ciał jest jednolinijkowym przekazaniem do iteratorów wycinka, a kolekcja zostaje na miejscu i można ją czytać dalej. Ręczne pisanie struktury iteratora ma sens dopiero wtedy, gdy pod spodem nie ma wycinka: drzewo, graf, lista wiązana. Wtedy pojawia się adnotacja czasu życia i nie jest ona ozdobnikiem — `'_` w `fn iter(&self) -> std::slice::Iter<'_, String>` przywiązuje iterator do pożyczenia `self`, dlatego kompilator odrzuca iterator, który przeżyłby dane (`E0515`, *cannot return value referencing local variable `v`*). Podpowiedź *„use `.collect()` to allocate the iterator”* to cała decyzja w sześciu słowach: albo oddajesz referencję związaną z danymi, które ma wywołujący, albo oddajesz dane na własność. Przy okazji: w stabilnym Ruscie nie ma `yield`, więc stan, który generator w Pythonie trzyma na swojej ramce, u ciebie staje się polami struktury — `Countdown { n }` to właśnie ten stan.

**Szukaj po polsku:** obiekt iterowalny a iterator · metody domyślne cechy · własny iterator w Ruscie · `rust implement Iterator next size_hint` · `rust IntoIterator for &T` · `rust E0515 returns value referencing local variable`
