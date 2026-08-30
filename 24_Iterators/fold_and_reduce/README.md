# `fold` and `reduce`

**Level:** 201 · working knowledge

**One line:** `fold` carries an accumulator through the sequence and hands it back — which is what `sum`, `count`, `all` and `collect` are each made of, and the one to reach for when the answer is not the same type as the items.

```rust
fn main() {
    let scores = [5, 3, 0, 4, 2, 1];
    let total = scores.iter().fold(0, |acc, s| acc + s);
    println!("{total}");   // 15
}
```

Two arguments: a starting value, and a closure taking `(accumulator, item)` that returns the next accumulator. Nothing else happens — no allocation, one pass, and the accumulator is moved through rather than copied.

## The named consumers are folds

This is not an analogy. `Sum for i32` in std is:

```rust
fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    iter.fold(0, |a, b| a + b)
}
```

and `count` is `self.fold(0, |accum, _elem| accum + 1)`, and `all` is a `try_fold` that breaks on the first `false`. So:

| you write | what it is |
|---|---|
| `.sum()` | `fold(0, +)` |
| `.count()` | a fold from `0` that adds one per item |
| `.max()` | `max_by(Ord::cmp)`, a reducing fold |
| `.collect::<Vec<_>>()` | `fold(Vec::new(), push)`, roughly |
| `.all(p)` | `try_fold` that breaks on the first `false` |

**Keep writing the named one.** Knowing they are folds is worth having because it tells you what `fold` is *for* — not because hand-rolling `sum` is ever an improvement. The named version is clearer, and several of them are specialized for speed in ways your fold will not be.

## What it is actually for: an accumulator of a different type

`sum` can only build a number. `fold` can build anything, and that is the whole point:

```rust
use std::collections::HashMap;

fn main() {
    let rows = [("Ada", 5), ("Ben", 3), ("Ada", 4)];
    let tally: HashMap<&str, i32> = rows.into_iter().fold(HashMap::new(), |mut acc, (name, score)| {
        *acc.entry(name).or_insert(0) += score;
        acc
    });
    println!("{}", tally["Ada"]);   // 9
}
```

Two rules of thumb come out of the shape. The accumulator is taken **by value and returned**, so it is `|mut acc, item| { …; acc }`, not `|acc, item|` with a mutation you hoped would stick. And a tuple accumulator answers two questions in one pass:

```rust
let (lo, hi) = scores
    .iter()
    .fold((i32::MAX, i32::MIN), |(lo, hi), s| (lo.min(*s), hi.max(*s)));   // (0, 5)
```

`.min()` then `.max()` walks the data twice for the same answer.

## `reduce`: fold with the first item as the starting value

```rust
fn reduce<F>(mut self, f: F) -> Option<Self::Item> {
    let first = self.next()?;
    Some(self.fold(first, f))
}
```

That is std's whole implementation, and it explains both differences. The accumulator **must** be the item type, since the first item is the seed. And the answer is an `Option`, because an empty sequence has no first item and `reduce` will not invent one:

```text
on an empty iterator:   reduce -> None
                        fold   -> 0
```

Which you want depends on whether an identity exists. For a sum it does — the total of nothing is genuinely `0`, so `fold(0, +)` is honest. For a maximum it does not: `fold(i32::MIN, max)` on an empty list returns `-2147483648`, a number that was never in your data and reads as a real answer. `reduce` returns `None` there, and `Option::max` is `None` for the same reason.

## `try_fold` is the one that can stop

`fold` has no way to say *"stop, I am done"* — it will visit every item, so an error has to be carried in the accumulator to the end. `try_fold` short-circuits:

```rust
let parsed: Result<i32, _> = ["5", "3", "no", "4"]
    .into_iter()
    .try_fold(0, |acc, s| Ok::<_, std::num::ParseIntError>(acc + s.parse::<i32>()?));
```

The closure runs **three** times, not four. This is the machinery under the short-circuiting consumers: `all` is `try_fold` breaking on `false`, `any` on `true`, `find` on a match. When a fold of yours needs to give up early, this is the method — and if the accumulator is a `Result`, [`collect`](../collect_and_fromiterator/README.md) may already do what you want with less ceremony.

## The trap: an accumulator you rebuild instead of carry

```rust
// Carried — one Vec, six pushes.
scores.iter().fold(Vec::new(), |mut acc, s| { acc.push(*s); acc });

// Rebuilt — six Vecs to produce one.
scores.iter().fold(Vec::new(), |acc, s| { let mut next = acc.clone(); next.push(*s); next });
```

Identical output, quadratic cost. The clone is there because `acc` was not declared `mut` and the first version looks like it should not compile — it does, because the accumulator is *moved* into the closure and moved back out. **A `.clone()` inside a fold is nearly always this mistake.** The same shape with a `String` accumulator is the usual way an innocent-looking line join becomes the slow part of a program.

One more, further off: **folding floats is order-dependent**, so the associativity your maths assumes is not the compiler's to use. That is [letting the compiler reorder a float sum](../../19_Numbers/letting_the_compiler_reorder/README.md).

## If you are coming from another language

- **Python.** `functools.reduce(f, xs, init)` is `fold`, and `functools.reduce(f, xs)` — no initializer — is `reduce`, right down to the failure: Python raises `TypeError: reduce() of empty iterable with no initial value` where Rust returns `None`. That is the difference worth carrying, since it is the same design decision made twice, once as an exception at run time and once as a type you have to open. Note also that Python's `reduce` was moved out of builtins in Python 3 with Guido arguing most uses are clearer as a loop or a `sum`/`max`/`any` — advice that transfers exactly: reach for the named consumer, and let `fold` be the one that builds a `dict`. The `mut acc` shape has no Python counterpart because `dict` is a reference; `acc[k] = v; return acc` in a `reduce` mutates the caller's dict, which is a bug Rust's ownership makes unwriteable.
- **ABAP.** There is no fold, and the translation is the loop everybody writes: `DATA(lv_total) = 0. LOOP AT lt_scores INTO DATA(lv_s). lv_total += lv_s. ENDLOOP.` — the accumulator declared before the loop, the closure body inside it. Two things transfer. `REDUCE` in ABAP 7.40+ **is** this exact operator: `REDUCE i( INIT x = 0 FOR wa IN lt_scores NEXT x = x + wa )`, and it is worth reading the `INIT`/`NEXT` keywords against Rust's two arguments — `INIT` is the starting value, `NEXT` is the closure. What ABAP has no counterpart for is `try_fold`: leaving a `LOOP` early is `EXIT`, a statement that jumps, where Rust's early stop is a *value* the accumulator returns, which is why the error survives the exit rather than being left in a variable you have to remember to check.
- **JavaScript.** `arr.reduce(f, init)` is `fold` and `arr.reduce(f)` is `reduce` — same pair, same empty-array trap (`TypeError: Reduce of empty array with no initial value`). The differences are cost and mutation: JS hands the callback `(acc, item, index, array)` and nothing stops you from mutating `array` mid-reduce, and the idiomatic `{...acc, [k]: v}` spread accumulator is exactly the rebuild-instead-of-carry trap above, made the default by the language's own style guides.

---

## The verified output

<!-- output:fold_and_reduce -->
*Verified output of [`fold_and_reduce.rs`](examples/fold_and_reduce.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. fold takes a starting value and a way to absorb one more item
   .fold(0, |acc, s| acc + s)   = 15
   .sum::<i32>()                = 15
   same answer, and `sum` IS this fold — std writes it as one.

2. Which is why the others are folds too
   count -> 6   (std: 6)
   max   -> 5   (std: Some(5))
   collect -> [5, 3, 0, 4, 2, 1]
   Reach for the named one every time — this is what it is made of,
   not a suggestion to write it yourself.

3. The accumulator does NOT have to be the item type
   six i32 -> one String   "5-3-0-4-2-1"
   three pairs -> a tally  [("Ada", 9), ("Ben", 3)]
   That is the whole reason fold exists: `sum` can only build a number.

4. Two answers in one pass, with a tuple accumulator
   min and max together    (0, 5)
   .min() then .max() walks the data twice; this walks it once.

5. reduce is fold with the first item as the starting value
   .reduce(|a, b| a + b)   -> Some(15)
   Note the Option: with no items there is no starting value to
   return, so the answer is None rather than a made-up zero.
   on an empty iterator:   reduce -> None
                           fold   -> 0
   fold's answer is the identity you supplied. reduce refuses to
   invent one, which is the right answer for max, and the wrong
   shape for sum — where 0 really is the answer.

6. try_fold stops at the first failure, and that is how any/all/find work
   try_fold over [5, 3, "no", 4] -> Err(invalid digit found in string)
   the closure ran 3 times, not 4 — it stopped at the bad row.
   A plain fold cannot do that: it has no way to say "stop", so it
   would have to carry the error to the end in the accumulator.

7. The trap: an accumulator that is rebuilt rather than carried
   carried:  [5, 3, 0, 4, 2, 1]
   cloned:   [5, 3, 0, 4, 2, 1]
   Identical output. The second built six Vecs to produce one, which
   is quadratic in the length. `mut acc` and returning it is the
   shape to write; a `.clone()` inside a fold is nearly always this.
```
<!-- /output -->

---

## See also

- [Iterators are lazy](../iterators_are_lazy/README.md) — why `fold` is a consumer and `map` is not
- [`collect` and `FromIterator`](../collect_and_fromiterator/README.md) — the fold that builds a collection, with the target type doing the work
- [`iter`, `iter_mut`, `into_iter`](../iter_iter_mut_into_iter/README.md) — which door to fold over, and why `*s` appears in half the closures above
- [The three closure traits](../../23_Closures/three_closure_traits/README.md) — `fold` takes an `FnMut`, which is what lets the accumulator change
- [Letting the compiler reorder a float sum](../../19_Numbers/letting_the_compiler_reorder/README.md) — the one type where the order of a fold is part of the answer
- [What a monad is](../../17_Option_and_Result/what_a_monad_is/README.md) — `try_fold`'s return type, and why it can short-circuit at all

## Sources

[`Iterator::fold` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold), [`reduce` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce) and [`try_fold` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold). The `Sum`, `count`, `reduce` and `all` bodies quoted above are from the pinned 1.98.0 toolchain's own source — `library/core/src/iter/traits/accum.rs` and `library/core/src/iter/traits/iterator.rs`.
