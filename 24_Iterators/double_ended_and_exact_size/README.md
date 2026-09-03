# `DoubleEndedIterator` and `ExactSizeIterator`

**Level:** 201 → 301 · deep dive

**One line:** `rev()` and `len()` are not `Iterator` methods you happen not to have written — they are two extra traits, each a **promise about the sequence** that `next` alone cannot make, and both are checked at compile time.

```rust
fn main() {
    let scores = [5, 3, 0, 4, 2, 1];
    println!("{:?}", scores.iter().rev().collect::<Vec<_>>());   // [1, 2, 4, 0, 3, 5]
    println!("{}", scores.iter().len());                         // 6
}
```

A slice iterator can do both because it holds two pointers and knows the distance between them. A hand-written iterator with only `next` can do neither, and asking is [an `E0277` on the `rev` line](../implementing_iterator/README.md), not a wrong answer at run time.

## `DoubleEndedIterator`: one more method, and one real obligation

```rust
impl DoubleEndedIterator for Countdown {
    fn next_back(&mut self) -> Option<u32> { … }
}
```

That unlocks `rev`, `rfind`, `rposition`, `rfold`, and a `last` that no longer walks the whole sequence.

The obligation is the part the compiler cannot check: **both ends consume from the same sequence, and they must meet in the middle.** Alternating calls on a six-item `Countdown`:

```text
next Some(6)  next_back Some(1)  next Some(5)  next_back Some(2)
remaining: [4, 3]
```

Four taken from two ends, two left, nothing handed out twice. An impl where `next` and `next_back` can both yield the same element — easy to write if each keeps its own cursor without checking the other — produces duplicates that no test of either method alone will find. Keep one pair of bounds, as above, and compare them in both methods.

## `ExactSizeIterator`: an empty impl block that is entirely a promise

```rust
impl ExactSizeIterator for Countdown {}
```

No method. `len()` is provided, and its body reads `size_hint()` and trusts it:

```text
size_hint()  (9, Some(9))
len()        9
```

So what you are signing for is that [`size_hint`](../implementing_iterator/README.md) returns two equal, correct numbers — and if it does not, `len()` reports a lie rather than panicking. The payoff is real: an exact hint lets `collect` make one allocation of exactly the right size instead of growing by doubling.

## Which adapters keep which

| adapter | `rev()` | `len()` |
|---|---|---|
| `map` | yes | yes |
| `skip`, `take`, `enumerate`, `zip` (both sides exact) | yes | yes |
| `filter`, `filter_map`, `flat_map` | yes | **no** |
| `str::chars` | yes | **no** |
| a hand-written iterator with only `next` | no | no |

Both columns are structural rather than arbitrary. `filter` can still walk backwards — testing a predicate from the far end is no harder — but it cannot say how many items will pass without running the predicate on all of them, so `len` would have to lie or to work. And `chars` cannot count without walking, because UTF-8 is variable width and the byte length is not the character count. The refusals are ordinary method-not-found errors:

```text title="Abridged — real rustc output, without the trailing suggestion"
error[E0599]: no method named `len` found for struct `Filter<I, P>` in the current scope
   |
 3 |     println!("{}", v.iter().filter(|n| **n % 2 == 0).len());
   |                    -                                 ^^^
   |                    |
   |                    method `len` is available on `&[u8]`
```

The *"method `len` is available on `&[u8]`"* note is worth reading twice: it is telling you the collection knows its length and the iterator over a filtered view of it does not. `.count()` is the honest answer, and it walks.

## The trap: `rev` and `enumerate` do not commute

```text
.enumerate().rev()  [(2, "Cara"), (1, "Ben"), (0, "Ada")]
.rev().enumerate()  [(0, "Cara"), (1, "Ben"), (2, "Ada")]
```

Same three names in the same order on screen, different numbers attached. `enumerate` numbers the items *it* hands out, so it takes its meaning from where it sits: first in the chain, the index is the position in the original data; last, it is the position in the reversed output.

If the number means *"row 3 of the file"* — an error message, a line reference, a ballot id — `enumerate` goes **first** and everything else after it. If it means *"third thing I am showing you"*, it goes last. Nothing about the types distinguishes these, both compile, and the bug surfaces as an off-by-everything in a message rather than as a crash.

## If you are coming from another language

- **Python.** `reversed(x)` needs `__reversed__` or `__len__` + `__getitem__`, and `len(x)` needs `__len__` — so Python splits the same capability three ways and checks at run time: `reversed(iter([1,2,3]))` raises `TypeError: argument to reversed() must be a sequence`. Rust makes both static, which is the whole difference: the "generator is not reversible" mistake is a compile error here and an exception there. Note also that Python's `reversed` on a list returns a view that walks backwards, exactly like `rev()`, while `list(reversed(gen))` has to materialise — the same cost Rust makes you notice by refusing.
- **ABAP.** An internal table is always indexed, so both capabilities come free and invisibly: `LOOP AT lt_rows FROM 1 TO lines( lt_rows )` in either direction, `lines( )` as an O(1) read for a standard table. There is nothing to declare, and nothing that can be missing — which is why the Rust distinction reads as bureaucracy at first. The thing to carry across is what it buys: because these are separate traits, a Rust function taking `impl ExactSizeIterator` is *documented* to be handed something countable, where an ABAP `FORM` taking a table can be handed a hashed one and only discovers at run time that `READ ... INDEX` will not work on it.
- **C++.** This is the iterator-category hierarchy — input, forward, bidirectional, random-access — with `DoubleEndedIterator` as bidirectional and `ExactSizeIterator` as (roughly) the `sized_range` part of random-access. C++20 concepts made those checkable at compile time in the same way; before concepts they were tag structs and the errors were template noise. If you have read a `std::random_access_iterator_tag`, you have already met the idea, and Rust's version is the two useful cuts of it rather than five.

---

## Practice

**Five references where you meant five numbers.** Start from the program that reverses an array two ways: a `for` loop over `my_array.iter().rev()`, and the same iterator collected into `let reversed_vec: Vec<&i32> = my_array.iter().rev().collect();`. Both walk `[1, 2, 3, 4, 5]` backwards, both print the numbers in the order you wanted, and one of them handed back a `Vec` of pointers.

Four questions about that `Vec`. Delete the `: Vec<&i32>` annotation and read the error — then read its `help:` line and say where following the suggestion lands you. Sum the vec with `.iter().sum::<i32>()`, find the `&&i32` in the message, and get the total three other ways. Rewrite the collect three ways so the element type is `i32`, and say which of the three is doing something the other two are not. Then answer the question none of the output raises: `{:?}` printed `[5, 4, 3, 2, 1]` for both types, so what did the reference version cost?

Two more on `rev()` itself. The page's `enumerate` trap has siblings: predict `.skip(1).rev()`, `.rev().skip(1)`, `.take(2).rev()` and `.rev().take(2)` before running any of them, and say which one means *"the last three, newest first"*. Finally, swap the array for a `HashSet` and watch the same `.rev()` stop compiling — then say why the refusal is the honest answer rather than a missing feature.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:double_ended_and_exact_size_kata -->
*[`double_ended_and_exact_size_kata.rs`](examples/double_ended_and_exact_size_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: five references where you meant five numbers.
//!
//!   rustc --edition 2024 double_ended_and_exact_size_kata.rs -o /tmp/deesk && /tmp/deesk
//!
//! Three refusals belong to this kata and cannot live in a file that compiles.
//! Each was produced by a scratch file named the way its transcript says.
//!
//! 1. Drop the annotation and `collect` has nothing to aim at (`no_annotation.rs`):
//!
//!        let reversed_vec = my_array.iter().rev().collect();
//!
//!    error[E0283]: type annotations needed
//!      = note: the type must implement `FromIterator<&i32>`
//!      help: consider giving `reversed_vec` an explicit type
//!        4 |     let reversed_vec: Vec<_> = my_array.iter().rev().collect();
//!
//!    Note what the suggestion would give you: `Vec<_>` infers `Vec<&i32>`,
//!    so following the compiler's advice lands on the reference vec too.
//!
//! 2. Summing it the obvious way is one `&` too many (`summing_refs.rs`):
//!
//!        let total: i32 = reversed_vec.iter().sum();
//!
//!    error[E0277]: a value of type `i32` cannot be made by summing an
//!                  iterator over elements of type `&&i32`
//!      = help: the trait `Sum<&&i32>` is not implemented for `i32`
//!
//! 3. `rev()` on a set is a compile error, not an ordering (`reversing_a_set.rs`):
//!
//!        let backwards: Vec<&i32> = seen.iter().rev().collect();
//!
//!    error[E0277]: the trait bound `std::collections::hash_set::Iter<'_, i32>:
//!                  DoubleEndedIterator` is not satisfied

fn main() {
    let my_array = [1, 2, 3, 4, 5];

    // 1 -----------------------------------------------------------------
    println!("1. The program as written prints the same thing twice");
    print!("   for num in my_array.iter().rev()   ");
    for num in my_array.iter().rev() {
        print!("{num} ");
    }
    println!();

    let reversed_vec: Vec<&i32> = my_array.iter().rev().collect();
    let reversed_val: Vec<i32> = my_array.iter().rev().copied().collect();
    println!("   Vec<&i32>                         {reversed_vec:?}");
    println!("   Vec<i32>                          {reversed_val:?}");
    println!("   Debug prints a &i32 by printing what it points at, so the");
    println!("   two lines are identical and the type is invisible on screen.");
    println!();

    // 2 -----------------------------------------------------------------
    println!("2. Three ways to the sum, once the items are references");
    let by_copied: i32 = reversed_vec.iter().copied().sum();
    let by_deref: i32 = reversed_vec.iter().map(|n| **n).sum();
    let by_into: i32 = reversed_vec.clone().into_iter().sum();
    println!("   .iter().copied().sum()            {by_copied}   &&i32 -> &i32");
    println!("   .iter().map(|n| **n).sum()        {by_deref}   &&i32 -> i32");
    println!("   .into_iter().sum()                {by_into}   items are &i32, vec consumed");
    println!("   `.iter().sum::<i32>()` is the one that does not compile: `iter`");
    println!("   on a Vec<&i32> hands out &&i32, and i32 sums &i32 but not &&i32.");
    println!();

    // 3 -----------------------------------------------------------------
    println!("3. Three routes to a Vec<i32>, and what the reference vec cost");
    let copied: Vec<i32> = my_array.iter().rev().copied().collect();
    let cloned: Vec<i32> = my_array.iter().rev().cloned().collect();
    let owned: Vec<i32> = my_array.into_iter().rev().collect();
    println!("   .iter().rev().copied()            {copied:?}   Copy items, free");
    println!("   .iter().rev().cloned()            {cloned:?}   same here, the general one");
    println!("   .into_iter().rev()                {owned:?}   arrays yield values (2021+)");
    println!(
        "   a &i32 is a pointer, and a pointer is usize-wide: {}",
        size_of::<&i32>() == size_of::<usize>()
    );
    println!("   so Vec<&i32> stores one pointer per number where Vec<i32>");
    println!("   stores the number — wider on any 64-bit target, and one");
    println!("   indirection on every read. The borrow is not the cheap option.");
    println!();

    // 4 -----------------------------------------------------------------
    println!("4. rev() does not commute with skip or take either");
    let a: Vec<i32> = my_array.iter().copied().skip(1).rev().collect();
    let b: Vec<i32> = my_array.iter().copied().rev().skip(1).collect();
    let c: Vec<i32> = my_array.iter().copied().take(2).rev().collect();
    let d: Vec<i32> = my_array.iter().copied().rev().take(2).collect();
    println!("   .skip(1).rev()                    {a:?}   dropped 1, then reversed");
    println!("   .rev().skip(1)                    {b:?}   reversed, then dropped 5");
    println!("   .take(2).rev()                    {c:?}   kept the first two");
    println!("   .rev().take(2)                    {d:?}   kept the last two");
    println!("   All four compile and none is wrong. \"The last three, newest");
    println!("   first\" is `.rev().take(3)`; `.take(3).rev()` is the first three");
    println!("   read backwards, which is a different set of rows.");
    println!();

    // 5 -----------------------------------------------------------------
    println!("5. Why a HashSet refuses rev() rather than picking an order");
    println!("   `seen.iter().rev()` is E0277: hash_set::Iter is not");
    println!("   DoubleEndedIterator. A hash set has no last element to walk");
    println!("   back from — its iteration order is unspecified — so there is");
    println!("   nothing for next_back to return. The refusal is the honest");
    println!("   answer, and it arrives at compile time.");
}
```
<!-- /source -->

<!-- output:double_ended_and_exact_size_kata -->
*Verified output of [`double_ended_and_exact_size_kata.rs`](examples/double_ended_and_exact_size_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The program as written prints the same thing twice
   for num in my_array.iter().rev()   5 4 3 2 1 
   Vec<&i32>                         [5, 4, 3, 2, 1]
   Vec<i32>                          [5, 4, 3, 2, 1]
   Debug prints a &i32 by printing what it points at, so the
   two lines are identical and the type is invisible on screen.

2. Three ways to the sum, once the items are references
   .iter().copied().sum()            15   &&i32 -> &i32
   .iter().map(|n| **n).sum()        15   &&i32 -> i32
   .into_iter().sum()                15   items are &i32, vec consumed
   `.iter().sum::<i32>()` is the one that does not compile: `iter`
   on a Vec<&i32> hands out &&i32, and i32 sums &i32 but not &&i32.

3. Three routes to a Vec<i32>, and what the reference vec cost
   .iter().rev().copied()            [5, 4, 3, 2, 1]   Copy items, free
   .iter().rev().cloned()            [5, 4, 3, 2, 1]   same here, the general one
   .into_iter().rev()                [5, 4, 3, 2, 1]   arrays yield values (2021+)
   a &i32 is a pointer, and a pointer is usize-wide: true
   so Vec<&i32> stores one pointer per number where Vec<i32>
   stores the number — wider on any 64-bit target, and one
   indirection on every read. The borrow is not the cheap option.

4. rev() does not commute with skip or take either
   .skip(1).rev()                    [5, 4, 3, 2]   dropped 1, then reversed
   .rev().skip(1)                    [4, 3, 2, 1]   reversed, then dropped 5
   .take(2).rev()                    [2, 1]   kept the first two
   .rev().take(2)                    [5, 4]   kept the last two
   All four compile and none is wrong. "The last three, newest
   first" is `.rev().take(3)`; `.take(3).rev()` is the first three
   read backwards, which is a different set of rows.

5. Why a HashSet refuses rev() rather than picking an order
   `seen.iter().rev()` is E0277: hash_set::Iter is not
   DoubleEndedIterator. A hash set has no last element to walk
   back from — its iteration order is unspecified — so there is
   nothing for next_back to return. The refusal is the honest
   answer, and it arrives at compile time.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:double_ended_and_exact_size -->
*Verified output of [`double_ended_and_exact_size.rs`](examples/double_ended_and_exact_size.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. `rev()` is not a method on Iterator's items — it needs the other end
   forwards  [5, 3, 0, 4, 2, 1]
   .rev()    [1, 2, 4, 0, 3, 5]
   A slice iterator knows where it ends, so it can walk backwards.
   An iterator that only has `next` cannot, and `.rev()` on one is
   E0277: the trait bound `Countdown: DoubleEndedIterator` is not
   satisfied — a compile error, never a run-time surprise.

2. Writing `next_back` is what buys it
   Countdown::new(5)          [5, 4, 3, 2, 1]
   .rev()                     [1, 2, 3, 4, 5]
   .rfind(|n| n % 2 == 0)     Some(2)

3. The contract: both ends eat from ONE sequence, and meet in the middle
   next Some(6)  next_back Some(1)  next Some(5)  next_back Some(2)
   remaining: [4, 3]
   Six items, four taken from alternating ends, two left. An impl
   where the two ends can hand out the same element twice is a bug
   the compiler cannot catch — this is the part you must get right.

4. ExactSizeIterator: len() without walking anything
   size_hint()  (9, Some(9))
   len()        9
   `len` is not a count — it reads `size_hint` and trusts it. That
   is why implementing the trait is a promise rather than a method:
   the impl block is empty, and what you are signing for is that
   size_hint's two numbers are equal and correct.
   and an exact hint pays: collect capacity 9

5. Which adapters keep which trait
   map      rev [8, 6, 4, 2]  len 4
   filter   rev [4, 2]  len: no — filter cannot know how many pass
   skip     rev [4, 3, 2]  len 3
   chars    rev "cba"  len: no — a char is 1 to 4 bytes, so the count
            is not known without walking the string
   Both are structural: an adapter keeps a trait only if it can
   still honour the contract. `filter` keeps rev and loses len.

6. The trap: rev() and enumerate() do not commute
   .enumerate().rev()  [(2, "Cara"), (1, "Ben"), (0, "Ada")]
   .rev().enumerate()  [(0, "Cara"), (1, "Ben"), (2, "Ada")]
   Same three names, same order on screen, DIFFERENT numbers.
   `enumerate` counts the position it hands out, so putting it first
   preserves the original index and putting it last renumbers from
   the new front. If the index means "row in the file", it goes first.
```
<!-- /output -->

---

## See also

- [Implementing `Iterator`](../implementing_iterator/README.md) — the required half, and where `size_hint` comes from
- [Iterators are lazy](../iterators_are_lazy/README.md) — why `len()` cannot just count, and what `count()` costs instead
- [`iter`, `iter_mut`, `into_iter`](../iter_iter_mut_into_iter/README.md) — the slice iterators that have both traits for free
- [Meet the `char`](../../14_Strings/meet_the_char/README.md) — why `chars()` cannot know its own length
- [Walking a string](../../14_Strings/walking_a_string/README.md) — where the `chars` iterator's other limits show up
- [What a trait is](../../12_Traits/what_a_trait_is/README.md) — an empty impl block as a promise rather than as code

## Sources

[`DoubleEndedIterator` ↗](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html) and [`ExactSizeIterator` ↗](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html) — the second page states the `size_hint` obligation, and that a wrong impl means a wrong `len()` rather than a panic.

## Po polsku

Do `Iterator` dochodzą tu dwie osobne cechy (*traits*) i najważniejsze jest to, że **żadna z nich nie jest „metodą, której ktoś zapomniał dopisać”**. `DoubleEndedIterator` wymaga `next_back` i dopiero to odblokowuje `rev`, `rfind`, `rposition`, `rfold` oraz `last`, które nie musi już przejść całej sekwencji. `ExactSizeIterator` daje `len()`. Ręcznie napisany iterator mający wyłącznie `next` nie ma ani jednego, ani drugiego — i dowiadujesz się o tym od kompilatora: `rev()` kończy się `E0277` (*the trait bound `Countdown: DoubleEndedIterator` is not satisfied*), a `len()` zwyczajnym „nie ma takiej metody”, czyli `E0599` — na tej stronie widać go w wersji dla `filter`: *no method named `len` found for struct `Filter<I, P>`*. To nigdy nie jest zła odpowiedź w czasie działania programu.

Po polsku „długość” i „liczba elementów” brzmią jak to samo pojęcie, a tu dzieli je koszt. `len()` niczego **nie liczy** — czyta `size_hint()` i mu wierzy. Dlatego `impl ExactSizeIterator for Countdown {}` jest pustym blokiem bez jednej linii kodu: cały jego sens to obietnica, że `size_hint()` zwraca dwie równe i prawdziwe liczby, tutaj `(9, Some(9))`. Jeśli nie zwraca, `len()` **skłamie**, zamiast wywołać panikę — nie ma tu żadnej kontroli w czasie działania. Uczciwą odpowiedzią jest wtedy `count()`, które faktycznie przechodzi całą sekwencję. Nagroda za dobrą obietnicę jest konkretna: `collect` rezerwuje pamięć raz, dokładnie na tyle elementów, zamiast rosnąć przez podwajanie.

Przy `DoubleEndedIterator` kompilator również nie sprawdzi rzeczy najistotniejszej: **oba końce jedzą z tej samej sekwencji i muszą spotkać się w środku**. Na sześcioelementowym `Countdown` naprzemienne wywołania dają `next Some(6)`, `next_back Some(1)`, `next Some(5)`, `next_back Some(2)` i zostaje `[4, 3]` — cztery wydane, dwa czekają, żaden dwa razy. Implementacja, w której każdy koniec trzyma własny kursor i nie zagląda do drugiego, wyda ten sam element dwukrotnie, a test samego `next` ani samego `next_back` tego nie wykryje. Trzymaj jedną parę granic i porównuj ją w obu metodach.

Tabela adapterów nie jest arbitralna. `filter` zachowuje `rev` (sprawdzanie predykatu od drugiego końca nie jest trudniejsze), ale traci `len`, bo nie wie, ile elementów przejdzie, dopóki predykat nie zadziała na wszystkich. `chars()` traci `len` z powodu, który dla polskiego czytelnika jest wyjątkowo namacalny: `"żółw"` to cztery znaki (*chars*), ale **siedem bajtów**, bo ż, ó i ł zajmują w UTF-8 po dwa — długość w bajtach nie jest liczbą znaków, więc iterator po znakach nie poda swojej długości bez przejścia łańcucha. I na koniec pułapka, która kompiluje się w obu wersjach: `.enumerate().rev()` daje `[(2, "Cara"), (1, "Ben"), (0, "Ada")]`, a `.rev().enumerate()` — `[(0, "Cara"), (1, "Ben"), (2, "Ada")]`. Te same nazwy w tej samej kolejności, inne numery. `enumerate` numeruje to, co samo wydaje, więc gdy liczba ma znaczyć „wiersz nr 3 w pliku”, `enumerate` idzie na **początek** łańcucha wywołań; gdy ma znaczyć „trzecia rzecz, którą ci pokazuję” — na koniec.

**Szukaj po polsku:** iterator dwukierunkowy · długość iteratora · liczba znaków a liczba bajtów · `rust DoubleEndedIterator next_back` · `rust ExactSizeIterator size_hint` · `rust enumerate rev order`
