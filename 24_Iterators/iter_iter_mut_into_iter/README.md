# `iter`, `iter_mut`, `into_iter`

**Level:** 101 → 201 · working knowledge

**One line:** Three doors onto the same collection — `iter()` lends you each item, `iter_mut()` lends it mutably, `into_iter()` hands it over and keeps nothing — and a `for` loop is picking one of them for you based on whether you wrote `&`, `&mut`, or neither.

```rust
fn main() {
    let mut names = vec![String::from("Ada"), String::from("Ben")];

    for n in &names { println!("{n}"); }            // &String     — read
    for n in &mut names { n.push('!'); }            // &mut String — change in place
    for n in names { println!("{n}"); }             // String      — take it apart
    // `names` is gone here: the third loop consumed it.
}
```

| you write | the loop calls | each item is | the collection afterwards |
|---|---|---|---|
| `for n in &names` | [`names.iter()` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.iter) | `&T` | untouched |
| `for n in &mut names` | [`names.iter_mut()` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut) | `&mut T` | modified in place |
| `for n in names` | [`names.into_iter()` ↗](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter) | `T` | **consumed** |

## `for` wants `IntoIterator`, not `Iterator`

A `for` loop desugars to `IntoIterator::into_iter(expr)`, which is why `Vec` implements that trait three times over — once for `Vec<T>`, once for `&Vec<T>`, once for `&mut Vec<T>` — rather than implementing `Iterator` itself. The `&` in `for n in &names` is not decoration; it selects the impl, and with it the item type.

That indirection is also what keeps a collection re-iterable. An iterator is single-use by construction — `next` takes `&mut self` and there is no rewind — so if `Vec` *were* an `Iterator`, looping over one would exhaust it. [Implementing `Iterator`](../implementing_iterator/README.md) is where that distinction gets built by hand.

## The E0382 everybody meets

```text title="Abridged — real rustc output, without the file-and-line header or the std-source note"
error[E0382]: borrow of moved value: `v`
    |
  2 |     let v = vec![1, 2, 3];
    |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
  3 |     for x in v {
    |              - `v` moved due to this implicit call to `.into_iter()`
...
  6 |     println!("{:?}", v);
    |                      ^ value borrowed here after move
    |
help: consider iterating over a slice of the `Vec<i32>`'s content to avoid moving into the `for` loop
    |
  3 |     for x in &v {
    |              +
```

Nothing about loops is special here. `for x in v` is a call taking `self`, the same as any other, and *"moved due to this implicit call to `.into_iter()`"* is rustc pointing at the word that did it. The fix it offers is one character.

## Arrays are the exception worth memorising

```rust
fn main() {
    let scores = [5u8, 3, 0];
    let by_value: Vec<u8> = scores.into_iter().collect();   // items are u8
    let by_ref: Vec<&u8> = scores.iter().collect();         // items are &u8
    println!("{by_value:?} {by_ref:?} {scores:?}");           // [5, 3, 0] [5, 3, 0] [5, 3, 0]
}
```

Since edition 2021, `array.into_iter()` yields values. Before it, the method resolved through the slice and yielded **references** — arrays had no by-value `IntoIterator` impl at all, so `.into_iter()` silently meant `.iter()`. That is a live trap when reading older answers: the code compiles either way and the item type changes underneath you. The [edition guide's `IntoIterator` for arrays ↗](https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html) has the migration story.

Note also that the array above is still usable after `into_iter()`, because `u8` is `Copy` — the array was copied, not moved. On `[String; 3]` the same line consumes it.

## Getting from `&T` to `T`, and what it costs

`iter()` gives you `&T`, and half of the clones in a beginner's Rust program are there to get out of that. The run below counts them, using a `Name` type whose `Clone` impl increments a counter:

```text
.iter().collect::<Vec<&Name>>()   0 clone(s)
.iter().cloned().collect()        3 clone(s)
.into_iter().collect()            0 clone(s)
```

All three produce three items. The middle one paid for it, and it paid because `iter()` was the wrong door — not because collecting is expensive. So:

- **`.copied()`** — for `Copy` items. `scores.iter().copied().sum::<i32>()`. Free.
- **`.cloned()`** — for everything else. Honest, visible, and exactly as expensive as `T::clone`.
- **`.into_iter()`** — when you did not need the original afterwards, which is more often than it feels. No clone at all.

The rule of thumb worth carrying: **a `.clone()` added to make a borrow compile is usually an `into_iter()` one line earlier**. The exception is when you genuinely need both copies, and then [`Cow`](../../18_Ownership/clone_on_write/README.md) is often the better answer again.

## If you are coming from another language

- **Python.** `for x in xs` never moves `xs`, and there is exactly one door: iteration always hands you a reference to the object, and whether you can mutate it depends on the object, not on the loop. So the two Rust doors that have no Python counterpart are the ones that matter — `iter_mut()`, which lets you write through the item, and `into_iter()`, which ends the collection. The Python idioms they replace are `for i, x in enumerate(xs): xs[i] = f(x)` for the first and `while xs: x = xs.pop()` for the second, and both are the shapes that produce the *"list changed size during iteration"* class of bug. Rust's borrow checker refuses that outright: while `iter()` is alive you cannot push, and the compiler names the line. What transfers cleanly is laziness — a Python iterator and a Rust one are both one-shot — and the `list(...)` habit is `collect()`.
- **ABAP.** This is the closest correspondence in the library, and it is worth learning through: `LOOP AT itab INTO wa` copies each row into a work area, `LOOP AT itab ASSIGNING <fs>` binds a field symbol to the row itself, and `LOOP AT itab REFERENCE INTO dref` hands you a pointer. Those are `into_iter()` (a copy per row, and by far the most common ABAP performance finding), `iter_mut()` (write straight into the table), and `iter()` respectively — the same three choices, made per loop, with the same costs. Two differences, both in Rust's favour and both worth naming. `INTO wa` copies but does *not* consume the table, so ABAP's default is a per-row copy that nobody is forced to notice, while Rust's `into_iter()` empties the collection and makes the transfer explicit — you cannot pay for a copy by accident, and you cannot forget you spent the table. And modifying a table while looping over it is legal ABAP with famously undefined-feeling consequences, where the equivalent in Rust is a borrow error at compile time. If you have ever changed a `LOOP ... INTO` to `ASSIGNING` and watched a program get twice as fast, you already know why `iter()` is the default door here.
- **JavaScript.** `for (const x of xs)` is `iter()` and nothing else; `xs.map(f)` allocates a new array eagerly. The nearest thing to `into_iter()` is `xs.splice(0)`, and nobody writes that. The one habit to drop is treating `map` as the way to loop with a side effect: in Rust it will not even run — see [iterators are lazy](../iterators_are_lazy/README.md).

---

## The verified output

<!-- output:iter_iter_mut_into_iter -->
*Verified output of [`iter_iter_mut_into_iter.rs`](examples/iter_iter_mut_into_iter.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The same Vec, three ways
   iter()      yields &Name     Name("Ada")
   iter_mut()  yields &mut Name  and the change landed: Name("Ada!")
   into_iter() yields Name       taken apart into: ["Ada!", "Ben!", "Cara!"]
   `names` is gone after into_iter — it was consumed, not borrowed.

2. A `for` loop is one of those three, chosen by what you wrote
   for n in &names        =>  names.iter()       item: &Name
     "Ada"     "Ben"     "Cara"
   for n in &mut names    =>  names.iter_mut()   item: &mut Name
     [Name("ADA"), Name("BEN"), Name("CARA")]
   for n in names         =>  names.into_iter()  item: Name  (moves it)
     "ADA"     "BEN"     "CARA"
   `for` needs IntoIterator, not Iterator, and Vec implements it three
   times: for Vec<T>, for &Vec<T>, and for &mut Vec<T>.

3. Which is why the beginner's E0382 happens where it does
   for n in names { .. }   then using `names` afterwards is:
     error[E0382]: borrow of moved value: `names`
     `names` moved due to this implicit call to `.into_iter()`
     help: consider iterating over a slice of the `Vec<Name>`'s content
   The fix rustc suggests is one character: `for n in &names`.

4. Arrays are the exception worth memorising
   [u8; 3].into_iter()  -> items are u8   [5, 3, 0]
   [u8; 3].iter()       -> items are &u8  [5, 3, 0]
   ...and `scores` is still usable: [5, 3, 0]  (u8 is Copy)
   Before edition 2021, array.into_iter() yielded REFERENCES — the
   method resolved through the slice, because arrays had no by-value
   IntoIterator. Old answers on the internet still assume that.

5. Getting from &T to T: copied, cloned, and what each costs
   .iter().collect::<Vec<&Name>>()   0 clone(s)  3
   .iter().cloned().collect()        3 clone(s)  3
   .into_iter().collect()            0 clone(s)  3
   All three produce a collection of three. Only the middle one paid
   for it — and it paid because `iter()` was the wrong door to start
   from, not because collecting is expensive.
   .copied() is the same adapter for Copy types: sum = 8

6. Choosing, in one line each
   read it            -> iter()       &T
   change it in place -> iter_mut()   &mut T
   take it apart      -> into_iter()  T   (the collection is spent)
   Reaching for .clone() to make a borrow compile usually means the
   answer was into_iter() one line earlier.
```
<!-- /output -->

---

## See also

- [`Vec::into_iter`](../../26_Collections/vec_methods/vec_into_iter/README.md) — the three `IntoIterator` impls written out, and why `self` is a reference in two of them
- [Iterators are lazy](../iterators_are_lazy/README.md) — what the chain after `iter()` does, and when
- [Implementing `Iterator`](../implementing_iterator/README.md) — writing the three doors for a collection of your own
- [Borrowing](../../18_Ownership/borrowing/README.md) — the rule that makes `iter()` and `iter_mut()` two different things
- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — what `for n in names` did to `names`
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — why the array survives `into_iter()` and a `Vec<String>` does not
- [`clone_into`](../../12_Traits/clone_into/README.md) — when the clone is unavoidable, the cheaper way to spell it
- [Walking a string](../../14_Strings/walking_a_string/README.md) — the same three-door question for text, where the answer is different

## Sources

[`IntoIterator` ↗](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) in std, and the edition guide on [`IntoIterator` for arrays ↗](https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html).
