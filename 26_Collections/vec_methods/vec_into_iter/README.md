# `Vec::into_iter` — and the three `IntoIterator` impls

[`Vec` methods](../README.md) · [Collections](../../README.md)

**Level:** 201 · working knowledge

**One line:** `Vec` implements `IntoIterator` three times — for `Vec<T>`, `&Vec<T>` and `&mut Vec<T>` — and which one you get, along with whether the vector survives, is decided by the receiver rather than by the method name.

```rust
fn main() {
    let mut names = vec![String::from("Ada"), String::from("Ben")];

    for n in &names { println!("{n}"); }        // &String     — read
    for n in &mut names { n.push('!'); }        // &mut String — change in place
    for n in names { println!("{n}"); }         // String      — take it apart
    // `names` is gone here: the third loop consumed it.
}
```

`into_iter` is not an inherent method on `Vec` at all. It is the single method of the [`IntoIterator` ↗](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) trait, and `Vec` implements that trait three times over.

## The three impls, as std writes them

Quoted from `library/alloc/src/vec/mod.rs` in the pinned 1.98.0 source, bodies elided. A `text` fence rather than a `rust` one because these are impls std already provides — pasted into a file they would collide with it.

```text
impl<T, A: Allocator> IntoIterator for Vec<T, A> {
    type Item = T;
    type IntoIter = IntoIter<T, A>;
    fn into_iter(self) -> Self::IntoIter { /* ... */ }
}

impl<'a, T, A: Allocator> IntoIterator for &'a Vec<T, A> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a, T, A: Allocator> IntoIterator for &'a mut Vec<T, A> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}
```

Three things are worth reading off that directly.

**The `Item` type is what differs**, and it is what the loop body sees: `T`, `&T`, `&mut T`.

**Two of the three take a reference, despite the `self`.** `fn into_iter(self)` in the second and third impls has `Self = &'a Vec<T, A>` and `Self = &'a mut Vec<T, A>` — so `self` *is* the reference. Reading `self` as "consumes the vector" is right for the first impl and wrong for the other two. Only the by-value impl ends the vector.

**The last two are literally `self.iter()` and `self.iter_mut()`.** Not "equivalent to" — the same call. That is the proof of the section below.

## `iter` vs `into_iter`

`into_iter` is **generic**: it obtains *an* iterator, and whether that yields values, shared references or mutable references depends on the type it is called on. That is context-dependent and occasionally surprising.

`iter` and `iter_mut` are **ad-hoc**: their return type does not depend on context. They conventionally yield shared and mutable references respectively, and they always do.

| you write | resolves to | `Item` | the vector afterwards |
|---|---|---|---|
| `v.iter()` | `slice::Iter<'_, T>` | `&T` | untouched |
| `v.iter_mut()` | `slice::IterMut<'_, T>` | `&mut T` | modified in place |
| `v.into_iter()` | `vec::IntoIter<T>` | `T` | **consumed** |
| `(&v).into_iter()` | `slice::Iter<'_, T>` | `&T` | untouched |
| `(&mut v).into_iter()` | `slice::IterMut<'_, T>` | `&mut T` | modified in place |

A detail the table hides: `iter` and `iter_mut` are **slice** methods, not `Vec` methods. `Vec<T>` derefs to `[T]`, and that is where they live — which is why the two right-hand columns match the reference impls exactly.

## `for` picks one for you

A `for` loop desugars to `IntoIterator::into_iter(expr)`. So the `&` in `for n in &names` is not decoration; it selects the impl, and with it the item type and the fate of the vector.

| you write | the loop calls | each item is |
|---|---|---|
| `for n in &names` | `names.iter()` | `&T` |
| `for n in &mut names` | `names.iter_mut()` | `&mut T` |
| `for n in names` | `names.into_iter()` | `T` |

That indirection is also what keeps a collection re-iterable. An iterator is single-use by construction — `next` takes `&mut self` and there is no rewind — so if `Vec` *were* an `Iterator`, looping over one would exhaust it.

## The `E0382` this explains

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

Nothing about loops is special here. `for x in v` is a call taking `self`, and *"moved due to this implicit call to `.into_iter()`"* is rustc naming the thing that did it. The fix it offers is one character.

## Arrays: the surprise that used to be here

Older writing about `into_iter` — including the Rust by Example post this page grew out of — uses arrays to show how surprising the context-dependence can be. That example no longer works, and it is worth knowing why, because the code in question still compiles and quietly means something different.

Before Rust 1.53, `IntoIterator` was implemented for `&[T; N]` and `&mut [T; N]` but **not** for `[T; N]`. When a method is not found on a value, resolution automatically retries on references to it — so `array.into_iter()` silently resolved through the slice and yielded `&T`. Every other type implements the trait all three ways, which is exactly what made arrays the odd one out.

Arrays now implement `IntoIterator` by value in the ordinary way, so `[u8; 3].into_iter()` yields `u8` and there is no surprise left. Two things follow: the historical example does not reproduce on a modern compiler, and older answers on the internet still assume the old behaviour — the code compiles either way and the item type changes underneath you. The [edition guide's `IntoIterator` for arrays ↗](https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html) has the migration story.

## One more reason `iter()` and `into_iter()` are not interchangeable

These two lines look like a free choice between item types:

```rust
fn main() {
    let refs: Vec<&i32> = [1, 2].iter().collect();
    let vals: Vec<i32> = [1, 2].into_iter().collect();
    println!("{refs:?} {vals:?}");   // [1, 2] [1, 2]
}
```

Both compile. Swap the literals for run-time values and only the second still does:

```text title="Abridged — real rustc output, without the file-and-line header or the trailing summary"
error[E0716]: temporary value dropped while borrowed
  |
3 |     let refs: Vec<&i32> = [x, y].iter().collect();
  |                           ^^^^^^                 - temporary value is freed at the end of this statement
  |                           |
  |                           creates a temporary value which is freed while still in use
4 |     println!("{refs:?}");
  |                ---- borrow later used here
  |
help: consider consuming the `[i32; 2]` when turning it into an `Iterator`
  |
3 |     let refs: Vec<&i32> = [x, y].into_iter().collect();
  |                                  +++++
help: consider using a `let` binding to create a longer lived value
  |
3 ~     let binding = [x, y];
4 ~     let refs: Vec<&i32> = binding.iter().collect();
  |
```

The first version worked because of **rvalue static promotion**: an array literal of constants is lifted to a `'static` constant, so references into it outlive the statement. Once the values come from variables there is nothing to promote, the array is an ordinary temporary, and the references die at the semicolon. Nothing about `Vec` changed — the collection is the same in both.

**rustc offers two fixes, and they do not produce the same thing.** `into_iter()` stops borrowing and gives a `Vec<i32>`; binding the array to a variable keeps the borrow alive long enough and gives the `Vec<&i32>` you asked for. Which one you want depends on whether the references were the point. Reach for the first unless they were.

Worth knowing when you check this yourself: `{:?}` prints `[1, 2]` for both `Vec<&i32>` and `Vec<i32>`, because `Debug for &T` forwards to `T`. The output cannot tell you which you built; only the type annotation can.

## Choosing

- **`iter()`** — you are reading. The default.
- **`iter_mut()`** — you are changing elements in place.
- **`into_iter()`** — you do not need the collection afterwards, which is more often than it feels.

The rule of thumb worth carrying: **a `.clone()` added to make a borrow compile is usually an `into_iter()` one line earlier.**

## Example

<!-- source:vec_into_iter -->
*[`vec_into_iter.rs`](examples/vec_into_iter.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
use std::slice;
use std::vec::IntoIter;

fn main() {
    // 1. Vec implements IntoIterator three times, and the receiver picks one.
    println!("1. Three impls, three item types");
    let mut v = vec![String::from("Ada"), String::from("Ben")];

    let by_ref: slice::Iter<'_, String> = (&v).into_iter();
    println!("   &Vec<T>     -> Item = &T       first {:?}", by_ref.clone().next());

    let by_mut: slice::IterMut<'_, String> = (&mut v).into_iter();
    for s in by_mut { s.push('!'); }
    println!("   &mut Vec<T> -> Item = &mut T   now {v:?}");

    let by_val: IntoIter<String> = v.clone().into_iter();
    println!("   Vec<T>      -> Item = T        collected {:?}", by_val.collect::<Vec<_>>());

    // 2. The associated types are the whole difference. Named out loud:
    println!("\n2. The named types");
    let borrowed = vec![1, 2, 3];
    let a: slice::Iter<'_, i32> = (&borrowed).into_iter();
    println!("   (&Vec<i32>).into_iter()      is slice::Iter<i32>,    count {}", a.count());
    let mut owned = vec![1, 2, 3];
    let b: slice::IterMut<'_, i32> = (&mut owned).into_iter();
    println!("   (&mut Vec<i32>).into_iter()  is slice::IterMut<i32>, count {}", b.count());
    let c: IntoIter<i32> = vec![1, 2, 3].into_iter();
    println!("   Vec<i32>.into_iter()         is vec::IntoIter<i32>,  count {}", c.count());

    // 3. A `for` loop is one of those three, chosen by what you wrote.
    println!("\n3. What `for` desugars to");
    let mut v = vec![1, 2, 3];
    let mut sum = 0;
    for n in &v { sum += *n; }                 // (&v).into_iter()
    for n in &mut v { *n *= 10; }              // (&mut v).into_iter()
    let owned: Vec<i32> = v.into_iter().collect();   // v.into_iter() — v is gone
    println!("   &v sum {sum}, &mut v then owned {owned:?}");

    // 4. into_iter() on a REFERENCE does not consume anything, despite the
    //    `self` in its signature: self IS the reference there.
    println!("\n4. `self` is the reference in two of the three");
    let v = vec![1, 2, 3];
    let n = (&v).into_iter().count();
    println!("   counted {n} through &v, and v is still here: {v:?}");

    // 5. iter() and iter_mut() are the unambiguous spellings. They are
    //    slice methods, reached through Deref — Vec does not define them.
    println!("\n5. iter() vs into_iter()");
    let v = vec![1, 2, 3];
    let same_type: bool = {
        let _a: slice::Iter<'_, i32> = v.iter();
        let _b: slice::Iter<'_, i32> = (&v).into_iter();
        true
    };
    println!("   v.iter() and (&v).into_iter() are the same type: {same_type}");
    println!("   iter() always yields &T; into_iter() depends on the receiver.");

    // 6. The pre-1.53 array surprise, and what it looks like today.
    println!("\n6. Arrays, before and after Rust 1.53");
    let arr = [1u8, 2, 3];
    let values: Vec<u8> = arr.into_iter().collect();     // Item = u8
    let refs: Vec<&u8> = arr.iter().collect();           // Item = &u8
    println!("   [u8; 3].into_iter() -> {values:?} (u8)");
    println!("   [u8; 3].iter()      -> {refs:?} (&u8)");
    println!("   Before 1.53 the first line yielded &u8: arrays had no by-value");
    println!("   impl, so method resolution fell through to the slice.");

    // 7. Why `[1, 2].iter().collect()` compiles but `[x, y].iter().collect()`
    //    does not: the literal array is promoted to a 'static constant, so the
    //    references outlive the statement. Nothing about Vec is involved.
    println!("\n7. Promotion, and why the same line can stop compiling");
    let from_literal: Vec<&i32> = [1, 2].iter().collect();
    let x = 1; let y = 2;
    let from_runtime: Vec<i32> = [x, y].into_iter().collect();   // note: into_iter
    println!("   [1, 2].iter().collect()          -> Vec<&i32>, len {}", from_literal.len());
    println!("   [x, y].into_iter().collect()     -> Vec<i32>,  len {}", from_runtime.len());
    println!("   [x, y].iter().collect() would be error[E0716] — the array is a");
    println!("   temporary. rustc offers two fixes: add `into_` (giving Vec<i32>),");
    println!("   or bind the array first (keeping the Vec<&i32> you asked for).");
    println!("   Debug prints both as {from_literal:?}, so the output cannot tell");
    println!("   you which is which — only the type can.");

    // 8. The error this all exists to explain.
    println!("\n8. Why the borrow checker cares");
    let names = vec![String::from("Ada")];
    let consumed: Vec<String> = names.into_iter().collect();
    // println!("{names:?}");   // error[E0382]: borrow of moved value: `names`
    println!("   after into_iter(), `names` is moved; only {consumed:?} remains");
}
```
<!-- /source -->

<!-- output:vec_into_iter -->
*Verified output of [`vec_into_iter.rs`](examples/vec_into_iter.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three impls, three item types
   &Vec<T>     -> Item = &T       first Some("Ada")
   &mut Vec<T> -> Item = &mut T   now ["Ada!", "Ben!"]
   Vec<T>      -> Item = T        collected ["Ada!", "Ben!"]

2. The named types
   (&Vec<i32>).into_iter()      is slice::Iter<i32>,    count 3
   (&mut Vec<i32>).into_iter()  is slice::IterMut<i32>, count 3
   Vec<i32>.into_iter()         is vec::IntoIter<i32>,  count 3

3. What `for` desugars to
   &v sum 6, &mut v then owned [10, 20, 30]

4. `self` is the reference in two of the three
   counted 3 through &v, and v is still here: [1, 2, 3]

5. iter() vs into_iter()
   v.iter() and (&v).into_iter() are the same type: true
   iter() always yields &T; into_iter() depends on the receiver.

6. Arrays, before and after Rust 1.53
   [u8; 3].into_iter() -> [1, 2, 3] (u8)
   [u8; 3].iter()      -> [1, 2, 3] (&u8)
   Before 1.53 the first line yielded &u8: arrays had no by-value
   impl, so method resolution fell through to the slice.

7. Promotion, and why the same line can stop compiling
   [1, 2].iter().collect()          -> Vec<&i32>, len 2
   [x, y].into_iter().collect()     -> Vec<i32>,  len 2
   [x, y].iter().collect() would be error[E0716] — the array is a
   temporary. rustc offers two fixes: add `into_` (giving Vec<i32>),
   or bind the array first (keeping the Vec<&i32> you asked for).
   Debug prints both as [1, 2], so the output cannot tell
   you which is which — only the type can.

8. Why the borrow checker cares
   after into_iter(), `names` is moved; only ["Ada"] remains
```
<!-- /output -->

## See also

- [`iter`, `iter_mut`, `into_iter`](../../../24_Iterators/iter_iter_mut_into_iter/README.md) — the same three doors as a lesson, with the Python and ABAP bridges
- [`Vec::drain`](../vec_drain/README.md) — moving elements out while keeping the vector
- [`Vec::as_slice`](../vec_as_slice/README.md) — the deref that makes `iter` a slice method
- [Iterators are lazy](../../../24_Iterators/iterators_are_lazy/README.md) — what the chain after `iter()` does, and when
- [Collect into a `Vec`](../../../24_Iterators/collect_into_a_vec/README.md) — the other direction

[`IntoIterator` ↗](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) and [`vec::IntoIter` ↗](https://doc.rust-lang.org/std/vec/struct.IntoIter.html) in the standard library.
