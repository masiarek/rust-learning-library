# Adapters by job

**Level:** 201 · working knowledge

**One line:** Twenty-odd adapters, and the choice between them is almost never about style — `filter` and `take_while` return different data, `flat_map` over a `Result` throws your errors away, and three of the ones people reach for are not iterator methods at all.

## The table

| The job you have | The adapter |
|---|---|
| keep the items matching a test | [`filter` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) |
| keep items **until** one fails the test | [`take_while` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take_while) (and [`skip_while` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip_while) for the tail) |
| transform every item | [`map` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) |
| transform **and** drop the ones that fail | [`filter_map` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map), closure returns `Option` |
| one item becomes several | [`flat_map` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map) |
| flatten an iterator of iterators you already have | [`flatten` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten) |
| number the items | [`enumerate` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate) |
| walk two sequences together | [`zip` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip) (stops at the shorter) |
| split a stream of pairs into two collections | [`unzip` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip) |
| split one stream into two by a test | [`partition` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition) |
| carry a running value and yield each step | [`scan` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan) |
| look at the next item without taking it | [`peekable` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable) + [`peek` ↗](https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.peek) |
| join two sequences end to end | `chain` |
| repeat forever | `cycle` (pair it with `take`) |
| every *n*th item | `step_by` |
| see each item without changing it (debugging) | `inspect` |

And the ones that are **not** on `Iterator`: `windows` and `chunks` are methods on a **slice**, and `dedup` is a method on `Vec`.

## The four that are traps

**`filter` versus `take_while`.** These read alike and answer different questions.

```text
scores = [5, 3, 0, 4, 2, 1]
.filter(< 5)      [3, 0, 4, 2, 1]
.take_while(< 5)  []
```

`filter` tests every item; `take_while` stops at the **first** failure and never looks again — here the leading `5` ends it immediately, and the result is empty rather than wrong-looking. On sorted data that early stop is exactly what you want and much cheaper. On unsorted data it silently returns a prefix that looks like a plausible answer, which is the worst kind of bug.

**`flat_map` over a `Result` deletes your errors.** A `Result` is an iterator of length 0 or 1, so this compiles and reads well:

```rust
let parsed: Vec<i32> = rows.iter().flat_map(|s| s.parse::<i32>()).collect();
```

Four rows in, three out. The `Err` was flattened away, with the same shape and the same type as the success case, and nothing anywhere says a row vanished. It is a legitimate tool when *"skip what does not parse"* is the requirement — and a silent data-loss bug when it is not. When the failure matters, [collect into a `Result`](../collect_and_fromiterator/README.md) instead, and let the first error stop the chain.

**`peek` needs `&mut`.** `peekable` looks like a read-only convenience, but `peek` must pull the item and hold it, so it takes `&mut self`. The iterator has to be `let mut`, and a `peek` whose borrow is still live where you also call `next` is a borrow error rather than a runtime problem.

**`windows` and `chunks` are not adapters.** Both need to look at several items at once, and an iterator that has handed you an item cannot go back for it — so they live on slices, where the data is still all there. `iter.windows(2)` is `E0599`, and the fix is to `collect` first or to keep the slice around. (`Iterator::map_windows` exists but is unstable.) `dedup` is the same story on `Vec`, and it removes only **consecutive** duplicates: deduplicating a whole sequence means sorting first, or collecting into a `HashSet`.

## `scan`, the one worth knowing about

`fold` gives you the final accumulator. `scan` gives you every intermediate:

```rust
let running: Vec<i32> = scores.iter().scan(0, |total, s| { *total += s; Some(*total) }).collect();
// [5, 8, 8, 12, 14, 15]
```

Two details do a lot of work. The state is `&mut` — you mutate through it rather than returning it, unlike `fold`. And the closure returns an `Option`, so returning `None` **ends the iterator**, which makes `scan` the way to write a `take_while` whose decision depends on everything seen so far.

## If you are coming from another language

- **Python.** `itertools` is the same catalogue with different names, and the mapping is worth having: `filter` → `filter`, `take_while` → `itertools.takewhile`, `skip_while` → `dropwhile`, `flat_map` → `itertools.chain.from_iterable(map(...))`, `zip` → `zip`, `chain` → `itertools.chain`, `cycle` → `itertools.cycle`, `step_by` → slicing or `itertools.islice`, `scan` → `itertools.accumulate`, `windows` → `itertools.pairwise` for the n=2 case. Two are absent from Python: `filter_map` (you write `filter(None, map(f, xs))` or a comprehension with an `if`), and `partition`, which the docs give as a recipe rather than a function. The trap that transfers is `takewhile` versus `filter` — identical in both languages, including the sorted/unsorted distinction. The one that does not: Python has no `Result`, so the silent-error-flattening trap above has no Python analogue at all.
- **ABAP.** The whole table is one `LOOP` with different statements inside it: `filter` is `CHECK` or a `WHERE` clause, `take_while` is `EXIT`, `skip_while` is a flag you set once, `map` is the assignment, `flat_map` is a nested `LOOP` with an inner `APPEND`, and `scan` is the running total everyone has written. That is worth naming because it explains the value of the catalogue: in ABAP each of those is the *same* construct with different punctuation, so the difference between "keep matching rows" and "stop at the first non-matching row" lives only in the reader's head. Here it lives in the method name and is visible at the call site. 7.40 table comprehensions (`FILTER`, `VALUE ... FOR ... WHERE`) cover the first two rows of the table and nothing further down it.
- **JavaScript.** `filter`, `map`, `flatMap`, `some`, `every` are the same, `reduce` is `fold`, and `Array.prototype` has no `takeWhile`, no `zip`, no `partition` — which is why every codebase has a `utils.js` reimplementing them. The structural difference is eagerness: each JS call allocates a new array, so a four-adapter chain allocates four times over the whole input, where the Rust chain allocates once at the `collect`. That is the argument for the fluent style being fine here and expensive there.

---

## The verified output

<!-- output:adapters_by_job -->
*Verified output of [`adapters_by_job.rs`](examples/adapters_by_job.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Keep some — and the two ways to stop, which are not the same
   .filter(< 5)          [3, 0, 4, 2, 1]
   .take_while(< 5)      []
   .skip_while(< 5)      [5, 3, 0, 4, 2, 1]
   `filter` tests every item. `take_while` stops at the FIRST item
   that fails and never looks again — on a sorted sequence that is
   the point, and on an unsorted one it is a bug that returns a
   plausible prefix. The 5 at the front ends take_while immediately.

2. Transform-and-keep in one step
   .filter_map(parse.ok())  [5, 3, 4]
   `filter_map` is `map` whose closure returns Option: Some keeps and
   transforms, None drops. Writing it as .map(..).filter(..).map(..)
   parses twice or unwraps.

3. One item in, many out — and the silent-loss trap
   .flat_map(split)         ["Ada", "Lovelace", "Ben", "Carter"]
   .flat_map(parse)         [5, 3, 4]
   The second one is the trap. A Result IS an iterator of length 0 or
   1, so flat_map over it flattens the Ok values and DROPS the Err —
   quietly, with the same shape as the successful case. Three rows in,
   three out; four rows in, three out, and nothing says which vanished.
   When the failure matters, collect into Result instead.

4. Split into two, keeping both halves
   .partition(< 3)          small [0, 2, 1]  large [5, 3, 4]
   .unzip()                 ["Ada", "Ben"]  [5, 3]
   `partition` splits one stream by a predicate; `unzip` splits a
   stream of pairs by position. Neither is lazy: both consume.

5. Carry state along the chain
   .scan(0, running total)  [5, 8, 8, 12, 14, 15]
   `scan` is a fold that yields every intermediate rather than only
   the last, and its closure returns Option — returning None ends the
   iterator, which is how you write a stateful `take_while`.

6. Look ahead without consuming
   peek() saw 5, then take(2) got [5, 3]
   `peek` takes &mut self even though it consumes nothing, because it
   has to pull the item and hold it. That is why a peeked iterator
   needs `let mut`, and why peeking inside a `while let` over the same
   iterator needs care about where the borrow ends.

7. Join, repeat, and step
   .chain(other)            [5, 3, 9, 9]
   .cycle().take(8)         [1, 2, 3, 1, 2, 3, 1, 2]
   .step_by(2)              [5, 0, 2]
   `cycle` is endless, so it only makes sense with something that
   stops — laziness is what keeps it from hanging.

8. The three that are NOT iterator adapters
   slice::windows(2)        [[1, 2], [2, 3], [3, 4]]
   slice::chunks(3)         [[1, 2, 3], [4]]
   Vec::dedup()             [1, 2, 3]
   `windows` and `chunks` are methods on a SLICE, not on Iterator —
   they need to look at several items at once, which an iterator that
   has handed you an item can no longer do. `dedup` is a Vec method
   and only removes CONSECUTIVE duplicates; deduplicating a whole
   sequence is a collect into a HashSet, or a sort first.
```
<!-- /output -->

---

## See also

- [Iterators are lazy](../iterators_are_lazy/README.md) — why ordering the chain changes the work but not the answer
- [`fold` and `reduce`](../fold_and_reduce/README.md) — `scan` without the intermediates, and the consumer the rest are built on
- [`collect` and `FromIterator`](../collect_and_fromiterator/README.md) — where `partition` and `unzip` get their two collections, and the honest way to keep errors
- [When a `for` loop beats a chain](../when_a_loop_beats_a_chain/README.md) — the cases where none of these is the right answer
- [`Option` is a one-item collection](../../17_Option_and_Result/option_as_collection/README.md) — why `flat_map` over an `Option` or a `Result` type-checks at all
- [Walking a string](../../14_Strings/walking_a_string/README.md) — the `split` family, which is this table for text

## Sources

The [`Iterator` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html) page lists all of them with examples; [`slice::windows` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.windows), [`slice::chunks` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks) and [`Vec::dedup` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup) are the three that are not on it.
