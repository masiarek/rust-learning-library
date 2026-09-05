# Iterators are lazy

**Level:** 201 · working knowledge

**One line:** An adapter such as `map` or `filter` computes nothing — it builds a value describing the work — and a **consumer** such as `collect`, `sum` or `find` is what runs it, for as long as it needs an answer and no longer.

```rust
fn main() {
    let scores = [5, 3, 0, 4, 2, 1];
    let doubled: Vec<i32> = scores.iter().map(|s| s * 2).collect();
    println!("{doubled:?}");   // [10, 6, 0, 8, 4, 2]
}
```

Delete `.collect()` and the closure never runs at all. The compiler notices:

```text title="Abridged — real rustc output, without the second note and the help block"
warning: unused `Map` that must be used
 --> scratch.rs:3:5
  |
3 |     scores.iter().map(|s| println!("visiting {s}"));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
```

*"Iterators are lazy and do nothing unless consumed"* is the rule, in rustc's words, and it is why every adapter's *return type* carries `#[must_use]` — the attribute sits on `struct Map`, not on the `map` method, which is why the warning names `Map`. An adapter you throw away is always a mistake.

## Adapters and consumers

An **adapter** takes an iterator and returns a new one: `map`, `filter`, `take`, `skip`, `enumerate`, `zip`, `chain`, `rev`, `flat_map`, `inspect`. Nothing runs.

A **consumer** takes an iterator and returns something that is not an iterator: `collect`, `sum`, `count`, `fold`, `for_each`, `find`, `any`, `all`, `max`, `last`, `position`. Something runs.

The one-line test: **if it gives you back an iterator, it did nothing.**

## How far the consumer runs is the consumer's decision

The run below counts calls to one closure across four consumers on the same six-element array:

```text
.collect()  -> [10, 6, 0, 8, 4, 2]    closure calls: 6
.find(>=6)  -> Some(10)               closure calls: 1
.any(==0)   -> true                   closure calls: 3
.count()    -> 6                      closure calls: 6
```

`find` and `any` are **short-circuiting**: they stop at the first answer that settles the question. `collect` and `count` cannot — every element has to be visited to be counted or kept.

## The chain is one pass, element at a time

The mental model to *unlearn* is the shell pipeline, where each stage runs over the whole stream before the next begins. An iterator chain pulls **one element all the way through** before touching the next:

```text
result: [10, 6]
  see 5
  test 5
  double 5
  see 3
  test 3
  double 3
```

That is `.inspect().filter().map().take(2).collect()` over `[5, 3, 0, 4, 2, 1]`. Three things fall out of it, and all three are the reason iterator chains are fast:

- **No intermediate collections.** An eager `filter` would allocate a `Vec` for `map` to read. Nothing here allocates but the final `collect`.
- **`take(2)` stopped the source.** Scores `0, 4, 2, 1` were never read, not even by `inspect`. The stop propagates *backwards* through the chain.
- **Ordering the chain is a real decision.** Put the cheap, selective test first: in the run below, the same answer costs 3 calls of the expensive predicate one way round and 6 the other.

## And it makes an endless sequence usable

```rust
fn main() {
    let first_three: Vec<u32> = (1u32..)
        .map(|n| n * n)
        .filter(|sq| sq % 3 == 1)
        .take(3)
        .collect();
    println!("{first_three:?}");   // [1, 4, 16]
}
```

`1..` has no end. Nothing hangs, because nothing is computed until `collect` asks and it stops asking after three. In an eager language this program does not terminate.

## The trap: `map` for side effects

`map` is for producing values. Using it to *do* something per item is the single most common iterator mistake, and it fails silently unless you happen to also consume the result:

```rust
fn main() {
    let scores = [5, 3, 0, 4, 2, 1];
    // scores.iter().map(|s| println!("{s}"));      // prints nothing
    scores.iter().take(3).for_each(|s| println!("{s}"));   // 5 / 3 / 0
}
```

Rust gives you the second warning too — *"`Iterator::map` call that discard the iterator's values … this function returns `()`, which is likely not what you wanted"* — which is worth reading as the compiler noticing that your closure returned nothing to map *to*. Reach for `for_each`, or a plain `for` loop, which is a consumer by construction.

## The trap on the other side: observing an iterator spends it

Laziness means the work happens when a consumer asks. `next` is a consumer, so **asking in order to look is still asking** — and a `println!` that prints `it.next()` has permanently taken that item:

```rust
fn main() {
    let mut it = vec!["a".to_string(), "b".to_string()].into_iter();
    println!("{it:?}");             // IntoIter(["a", "b"]) - free, takes nothing
    let first = it.next();
    println!("{:?}", it.next());    // Some("b") - and "b" is now gone
    // assert_eq!(it.next(), Some("b".to_string()));   // would panic: left None
    println!("{first:?}");          // Some("a") - a stored value, still there
}
```

The two prints look alike and are not. `println!("{it:?}")` borrows the iterator and shows what is left in it, costing nothing. `println!("{:?}", it.next())` calls a method that takes `&mut self` and hands back the item by value — the printing is a side effect of a consumption that has already happened.

Which is why the assert fails where it does. `first` holds up, because it is a value that was saved rather than a call that was repeated; every later `next()` returns `None`, because [`vec::IntoIter` ↗](https://doc.rust-lang.org/std/vec/struct.IntoIter.html) is a [`FusedIterator` ↗](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) and an exhausted iterator goes on saying so. **The debugging print is what broke the test**, and deleting the print makes the test pass — which is close to the worst shape a bug can have.

The habit that avoids it: **collect once, then look at the collection.** `let items: Vec<_> = it.collect();` gives you something that can be printed, indexed and asserted against as often as you like, because a `Vec` is not spent by being read. When you do want to take part of an iterator and keep the rest, `by_ref()` is the method that says so.

## If you are coming from another language

- **Python.** This is the Python 2 → 3 change, and if you lived through it you already own the model: `map` and `filter` used to return lists and now return lazy iterators, `range` stopped materialising, and `itertools.islice` is `take`. So the mistake Python teaches you to avoid — `map(print, xs)` printing nothing at the REPL in Python 3 — is exactly the trap above, and Rust's version comes with a compiler warning instead of a silent no-op. Two differences that matter. A Python generator is *one-shot* and Rust's iterator is too, but Python re-running `for x in gen` silently yields nothing while Rust's borrow checker refuses the second use outright. And Python's laziness stops at the syntax: a list comprehension is eager, a generator expression is lazy, and the only difference is a bracket — `[f(x) for x in xs]` versus `(f(x) for x in xs)`. In Rust the split is visible in the type: `Map<...>` is a plan, `Vec<...>` is a result.
- **ABAP.** There is nothing lazy in the language, so the honest translation is the internal table you build in the first `LOOP` for the second `LOOP` to read. That is precisely the intermediate collection this page says iterators avoid, and the ABAP habit that transfers is the good one: everybody who has tuned a nested `LOOP` already knows to filter early — a `WHERE` on the `LOOP` beats a `CHECK` inside it, for the same reason the cheap test goes first here. What does not exist in ABAP is short-circuiting over a table: `LOOP ... WHERE` still walks the whole table unless you `EXIT`, so `find` and `any` correspond to a `LOOP` with an explicit `EXIT` after the first hit, written by hand every time. Rust's consumers have that `EXIT` built into their contract. The nearest thing ABAP has to a lazy source is a cursor — `OPEN CURSOR` / `FETCH NEXT PACKAGE` — which is the same idea (produce on demand, stop when the caller stops asking) at the database boundary rather than in the language.
- **Java / C#.** `Stream` and LINQ are the same design: intermediate operations are lazy, terminal operations run the pipeline, and both make the split part of the vocabulary. Java's *"streams are single-use"* rule matches Rust's move semantics on iterators, and C#'s deferred execution surprise — `IEnumerable` re-running the query every time you enumerate it — has no Rust counterpart, because consuming a Rust iterator consumes it once and the compiler will not let you do it twice.

---

## The verified output

<!-- output:iterators_are_lazy -->
*Verified output of [`iterators_are_lazy.rs`](examples/iterators_are_lazy.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Building the chain runs nothing at all
   SCORES.iter().map(doubled)        closure calls: 0
   `_plan` is a value of type Map<Iter<i32>, {closure}>. It holds the
   source and the closure, and has not looked at a single score.

2. A consumer is what runs it — and how far it runs is the consumer's call
   .collect()  -> [10, 6, 0, 8, 4, 2]    closure calls: 6
   .find(>=6)  -> Some(10)               closure calls: 1
   .any(==0)   -> true                   closure calls: 3
   .count()    -> 6                      closure calls: 6
   find and any stop at the first answer. collect and count cannot.

3. The chain is one pass, element at a time — not one pass per adapter
   result: [10, 6]
     see 5
     test 5
     double 5
     see 3
     test 3
     double 3
   Each score is carried all the way through before the next one starts,
   and `take(2)` stopped the whole chain: scores 0, 4, 2 and 1 were never
   read. An eager version would have built two intermediate Vecs and
   visited all six.

4. Which is what makes an endless sequence usable
   (1..).map(square).filter(%3==1).take(3) = [1, 4, 16]
   `1..` has no end. Nothing hangs, because nothing is computed until
   `collect` asks, and it stops asking after three.

5. The mistake the compiler warns about
   SCORES.iter().map(|s| println!("{s}"));   <- prints NOTHING
   warning: unused `Map` that must be used
   note: iterators are lazy and do nothing unless consumed
   `map` is for producing values. To run a side effect per item, use
   `for_each` or a plain `for` loop, both of which consume:
   .for_each(...)  seen = [5, 3, 0]

6. Laziness is per-adapter, so where you put an expensive step matters
   cheap test first:  1 kept, expensive test ran 3 times
   expensive first:   1 kept, expensive test ran 6 times
   same answer, different amount of work. Order the chain so the
   cheapest, most selective test runs first.

7. Observing an iterator SPENDS it
   IntoIter(["a", "b"])                <- Debug on the iterator: free, takes nothing
   let first = it.next();       first = Some("a")
   println!("{:?}", it.next())  prints Some("b")
   println!("{:?}", it.next())  prints None
   first is STILL Some("a"): a stored value, not a repeated call.
   But assert_eq!(it.next(), Some("b")) would panic here — left None,
   right Some("b") — because the two prints above already took both.
   Printing an iterator is free; printing what next() returns is not.
```
<!-- /output -->

## Practice

**Predict the closure-call count, then count it.** Take one array of six numbers and one closure that increments a counter each time it runs. Build seven chains over them — `collect`, `count`, `find`, `position`, `any`, `all`, and a `zip` against a three-element array — and write down, before running anything, how many times you expect the counted closure to be called in each.

Three of the seven stop early, and the reason differs each time: one stops because the answer is settled, one because the *other* iterator ran out, and one because a `take` upstream said so. Then answer the question the counts raise: `.filter(cheap).filter(expensive)` and `.filter(expensive).filter(cheap)` return the same rows — say which you would write, and what you would need to know about the data to be sure.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:iterators_are_lazy_kata -->
*[`iterators_are_lazy_kata.rs`](examples/iterators_are_lazy_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: how many times does the counted closure actually run?
//!
//!   rustc --edition 2024 iterators_are_lazy_kata.rs -o /tmp/ialk && /tmp/ialk

use std::cell::Cell;

const SCORES: [i32; 6] = [5, 3, 0, 4, 2, 1];
const NAMES: [&str; 3] = ["Ada", "Ben", "Cara"];

fn main() {
    let calls = Cell::new(0);
    // The counted closure. It is deliberately trivial: what is being measured
    // is how often the chain asks for it, not what it does.
    let seen = |s: &i32| {
        calls.set(calls.get() + 1);
        *s
    };
    let mut report = |label: &str, answer: String| {
        println!("   {label:<44} {:<18} calls: {}", answer, calls.get());
        calls.set(0);
    };

    println!("1. Seven chains over the same six scores");

    let out: Vec<i32> = SCORES.iter().map(seen).collect();
    report(".map(seen).collect()", format!("{out:?}"));

    let n = SCORES.iter().map(seen).count();
    report(".map(seen).count()", n.to_string());

    let f = SCORES.iter().map(seen).find(|s| *s == 0);
    report(".map(seen).find(== 0)", format!("{f:?}"));

    let p = SCORES.iter().map(seen).position(|s| s == 4);
    report(".map(seen).position(== 4)", format!("{p:?}"));

    let a = SCORES.iter().map(seen).any(|s| s < 4);
    report(".map(seen).any(< 4)", a.to_string());

    let all = SCORES.iter().map(seen).all(|s| s < 4);
    report(".map(seen).all(< 4)", all.to_string());

    let z: Vec<(i32, &str)> = SCORES.iter().map(seen).zip(NAMES).collect();
    report(".map(seen).zip(3 names).collect()", format!("{}", z.len()));

    println!();
    println!("2. Why each of the short ones stopped");
    println!("   find(== 0)     3 calls — the zero sits at index 2, so the third");
    println!("                  element settled the question.");
    println!("   position(== 4) 4 calls — same reason, one element further along.");
    println!("   any(< 4)       2 calls — the first `true` ends it. `all` is the");
    println!("                  mirror image: it runs until the first `false`,");
    println!("                  which here is the very first score, so 1 call.");
    println!("   zip(3 names)   3 calls — the SHORTER side ends the pair. Nothing");
    println!("                  about the scores stopped it; the names ran out.");
    println!("   collect/count  6 calls — neither can answer without every element.");

    println!();
    println!("3. And the trap: zip pulls the LEFT side first");
    let left = Cell::new(0);
    let right = Cell::new(0);
    let pairs: Vec<(i32, i32)> = SCORES
        .iter()
        .inspect(|_| left.set(left.get() + 1))
        .zip(SCORES.iter().inspect(|_| right.set(right.get() + 1)).take(2))
        .map(|(a, b)| (*a, *b))
        .collect();
    println!("   pairs = {pairs:?}");
    println!("   left side pulled {} times, right side {} times", left.get(), right.get());
    println!("   zip asks the left iterator for an item BEFORE it discovers the");
    println!("   right one is empty — so the longer side is always pulled one");
    println!("   extra time. If that pull has a side effect, this is where it bites.");

    println!();
    println!("4. Which order to write .filter(cheap).filter(expensive) in");
    let expensive_calls = Cell::new(0);
    let expensive = |s: &&i32| {
        expensive_calls.set(expensive_calls.get() + 1);
        **s > 3
    };
    let cheap_first = SCORES.iter().filter(|s| **s % 2 == 1).filter(expensive).count();
    let cheap_first_cost = expensive_calls.get();
    expensive_calls.set(0);
    let expensive_first = SCORES.iter().filter(expensive).filter(|s| **s % 2 == 1).count();
    println!("   cheap first:     {cheap_first} row(s), expensive test ran {cheap_first_cost} times");
    println!("   expensive first: {expensive_first} row(s), expensive test ran {} times",
             expensive_calls.get());
    println!("   Same rows either way, so this is a cost question only — and the");
    println!("   answer is not simply \"cheap first\". Put first whichever test");
    println!("   REJECTS the most rows per unit of work: a cheap filter that keeps");
    println!("   everything saves nothing. You need the data's selectivity, which");
    println!("   is exactly what a query planner spends its life estimating.");
}
```
<!-- /source -->

<!-- output:iterators_are_lazy_kata -->
*Verified output of [`iterators_are_lazy_kata.rs`](examples/iterators_are_lazy_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Seven chains over the same six scores
   .map(seen).collect()                         [5, 3, 0, 4, 2, 1] calls: 6
   .map(seen).count()                           6                  calls: 6
   .map(seen).find(== 0)                        Some(0)            calls: 3
   .map(seen).position(== 4)                    Some(3)            calls: 4
   .map(seen).any(< 4)                          true               calls: 2
   .map(seen).all(< 4)                          false              calls: 1
   .map(seen).zip(3 names).collect()            3                  calls: 3

2. Why each of the short ones stopped
   find(== 0)     3 calls — the zero sits at index 2, so the third
                  element settled the question.
   position(== 4) 4 calls — same reason, one element further along.
   any(< 4)       2 calls — the first `true` ends it. `all` is the
                  mirror image: it runs until the first `false`,
                  which here is the very first score, so 1 call.
   zip(3 names)   3 calls — the SHORTER side ends the pair. Nothing
                  about the scores stopped it; the names ran out.
   collect/count  6 calls — neither can answer without every element.

3. And the trap: zip pulls the LEFT side first
   pairs = [(5, 5), (3, 3)]
   left side pulled 3 times, right side 2 times
   zip asks the left iterator for an item BEFORE it discovers the
   right one is empty — so the longer side is always pulled one
   extra time. If that pull has a side effect, this is where it bites.

4. Which order to write .filter(cheap).filter(expensive) in
   cheap first:     1 row(s), expensive test ran 3 times
   expensive first: 1 row(s), expensive test ran 6 times
   Same rows either way, so this is a cost question only — and the
   answer is not simply "cheap first". Put first whichever test
   REJECTS the most rows per unit of work: a cheap filter that keeps
   everything saves nothing. You need the data's selectivity, which
   is exactly what a query planner spends its life estimating.
```
<!-- /output -->

</details>

---

## See also

- [`iter`, `iter_mut`, `into_iter`](../iter_iter_mut_into_iter/README.md) — where the chain starts, and what it hands you
- [Implementing `Iterator`](../implementing_iterator/README.md) — the `next` method all of this laziness is made of
- [The three closure traits](../../23_Closures/three_closure_traits/README.md) — why `map` takes an `FnMut` and `unwrap_or_else` takes an `FnOnce`
- [`Option` is a one-item collection](../../17_Option_and_Result/option_as_collection/README.md) — the same adapters, on an iterator of length 0 or 1
- [Walking a string](../../14_Strings/walking_a_string/README.md) — `chars`, `bytes` and the split family, which are these iterators over text
- [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) — why a chain of adapters compiles to the loop you would have written

## Sources

[`std::iter` ↗](https://doc.rust-lang.org/std/iter/index.html) — its own *Laziness* section is the source of the rule, and the [`Iterator` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html) page, whose adapter return types (`Map`, `Filter`, `Take`…) each carry the `#[must_use]` quoted above.

## Po polsku

Polskie określenie na to zjawisko istnieje i jest dobre: **leniwe wartościowanie**, w opozycji do **gorliwego**. `map` czy `filter` niczego nie liczą — budują wartość opisującą pracę do wykonania, i to wartość o widocznym typie: `Map<Iter<i32>, {closure}>` trzyma źródło i domknięcie (*closure*), nie zaglądając do żadnego elementu. Skasuj `.collect()` z pierwszego przykładu, a domknięcie nie wykona się ani razu. Kompilator to zauważy i wypowie regułę wprost: *„iterators are lazy and do nothing unless consumed”* — zdanie warte zapamiętania po angielsku, bo w takiej postaci zobaczysz je w terminalu. Zwróć uwagę na drobiazg, który tłumaczy treść ostrzeżenia: `#[must_use]` siedzi na **strukturze `Map`**, a nie na metodzie `map`, i dlatego rustc pisze *unused `Map` that must be used* — czyli o strukturze, nie o metodzie. Test na jedną linijkę: **jeśli coś oddaje ci iterator, to nic nie zrobiło.**

Kto zatem wykonuje pracę i ile jej wykona? Konsument — i to on decyduje, jak daleko zajdzie. Osoba znająca strumienie z Javy ma tu gotową parę pojęć: adapter to **operacja pośrednia**, a konsument — **operacja kończąca**. Na tej samej sześcioelementowej tablicy licznik wywołań domknięcia wygląda tak: `.collect()` — 6, `.find(>=6)` — 1, `.any(==0)` — 3, `.count()` — 6. `find` i `any` **przerywają** w chwili, gdy odpowiedź jest już przesądzona; `collect` i `count` nie mają takiej możliwości, bo żeby coś policzyć albo zachować, trzeba to najpierw zobaczyć.

Model do **oduczenia się** to potok powłoki, w którym każdy etap przerabia cały strumień, zanim zacznie następny. Łańcuch iteratorów działa odwrotnie: przeciąga **jeden element przez całą trasę**, zanim dotknie kolejnego — stąd ślad `see 5`, `test 5`, `double 5`, `see 3`, `test 3`, `double 3`, a nie sześć „see”, potem sześć „test”. Wynikają z tego trzy rzeczy i wszystkie trzy są powodem, dla którego takie łańcuchy są szybkie:

- **Brak kolekcji pośrednich.** Gorliwy `filter` musiałby zaalokować wektor, żeby `map` miał co czytać. Tu alokuje wyłącznie końcowy `collect`.
- **`take(2)` zatrzymał źródło.** Elementy `0, 4, 2, 1` nie zostały przeczytane w ogóle, nawet przez `inspect` — zatrzymanie propaguje się **wstecz** przez łańcuch.
- **Dzięki temu nieskończona sekwencja jest użyteczna.** `(1u32..).map(|n| n * n).filter(|sq| sq % 3 == 1).take(3).collect()` daje `[1, 4, 16]` i nic się nie zawiesza, choć `1..` nie ma końca. W języku gorliwym ten program nigdy by się nie skończył.

Najczęstszy błąd to `map` użyty **dla efektu ubocznego** — i jest cichy, bo bez konsumenta po prostu nic się nie dzieje: `scores.iter().map(|s| println!("{s}"))` nie wypisze niczego. Sięgnij po `for_each` albo po zwykłą pętlę `for`, która jest konsumentem z natury. Na koniec rzecz, którą łatwo przyswoić za mocno: kolejność filtrów naprawdę ma znaczenie (ten sam wynik kosztuje 3 albo 6 wywołań drogiego predykatu), ale zasadą **nie** jest „najtańszy najpierw”. Pierwszy ma iść ten test, który **odrzuca najwięcej wierszy na jednostkę pracy** — tani filtr przepuszczający wszystko nie oszczędza niczego. To dokładnie pojęcie **selektywności** z optymalizacji zapytań, a wiedza o niej pochodzi z danych, nie z kodu.

**Szukaj po polsku:** leniwe wartościowanie · wartościowanie gorliwe · operacje pośrednie i kończące · selektywność predykatu · `rust iterators are lazy` · `rust unused Map that must be used`
