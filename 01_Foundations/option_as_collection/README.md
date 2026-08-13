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
| `iter.flatten()` over `Option`s | each `Option` is itself iterable, so flattening drops the `None`s |
| `vec.extend(Some(3))` | `extend` takes any `IntoIterator` |
| `iter.chain(Some(9))` | same reason |
| `iter.filter_map(f)` | it is literally `map` followed by `flatten` |

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
rustc --edition 2024 01_Foundations/option_as_collection/examples/option_as_collection.rs -o /tmp/oac && /tmp/oac
```

## See also

- [`Option` vs `Result`](../option_vs_result/README.md) — which of the two you should be reaching for in the first place
- [`Option` fields](../option_fields/README.md) — `Option` in a type definition rather than a return type
- [`std::mem::discriminant`](https://doc.rust-lang.org/std/mem/fn.discriminant.html) and [`Option::is_some_and`](https://doc.rust-lang.org/core/option/enum.Option.html#method.is_some_and)
