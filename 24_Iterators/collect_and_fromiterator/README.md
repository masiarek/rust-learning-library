# `collect` and `FromIterator`

**Level:** 201 · working knowledge

**One line:** `collect` has no behaviour of its own — it asks the **type you asked for** to build itself from your iterator, which is why the same call produces a `Vec`, a `String`, a deduplicated set, a map, or one `Result` covering every row.

```rust
fn main() {
    let words = ["Ada", "Ben", "Cara", "Ada"];
    let v: Vec<&str> = words.into_iter().collect();
    let s: String = words.into_iter().collect();
    println!("{v:?} {s:?}");   // ["Ada", "Ben", "Cara", "Ada"] "AdaBenCaraAda"
}
```

One iterator, two annotations, two different programs. The method is a one-liner:

```rust
fn collect<B: FromIterator<Self::Item>>(self) -> B {
    FromIterator::from_iter(self)
}
```

Everything interesting is in `B`.

## The type has to be *determined*, not *written*

Because the target does the work, the compiler cannot infer it from the iterator alone — `E0282, type annotations needed` is the most common `collect` error, and it means *"which collection?"*, not *"which element?"*.

But "annotate the collect" is the rule as usually taught, and it is a rule about the wrong thing. What `collect` needs is for **something** to pin `B`. Three things can:

```rust
let v: Vec<&str> = words.into_iter().collect();      // 1. the binding
let n = words.into_iter().collect::<Vec<_>>().len(); // 2. the turbofish
let report = Report { lines: words.into_iter().map(str::to_uppercase).collect() };
                                                     // 3. the USE SITE — nothing named here
```

The third is the one that surprises people, and it is not a special case: inference runs backward from wherever the value lands, so a **struct field**, a **function parameter**, or a **return type** determines `B` exactly as well as a binding does. In `fn shout(w: &[&str]) -> Vec<String> { w.iter().map(|s| s.to_uppercase()).collect() }` the signature is the annotation.

Use the turbofish when nothing downstream pins it — usually because you are about to call a method on the result. The `_` is inference doing the element type, which it *can* work out; only the container has to be named. That is also the cheapest way to explore an unfamiliar API: write `let x: Vec<_> = …` and let rustc or your editor tell you what the elements turned out to be.

**The one use site that does not pin it is a slice parameter** — which is the parameter type Rust otherwise tells you to prefer, and the one `clippy::ptr_arg` (warn by default: *"fn arguments of the type `&Vec<...>` or `&String`, suggesting to use `&[...]` or `&str` instead"*) will push you toward. Given `fn widest(lines: &[String]) -> usize`, this fails:

```rust
let lines = words.into_iter().map(str::to_uppercase).collect();  // ← does not compile
println!("{}", widest(&lines));
```

```text
error[E0277]: a slice of type `[String]` cannot be built since `[String]` has no definite size
   |
 7 |     let lines = words.into_iter().map(str::to_uppercase).collect();
   |                                                          ^^^^^^^ try explicitly collecting into a `Vec<String>`
   |
   = help: the trait `FromIterator<String>` is not implemented for `[String]`
```

`&Vec<String>` reaches `&[String]` by a **deref coercion**, and inference will not run a coercion backward — it takes `[String]` at face value and reports it unsized. Both `Vec<String>` and `&Vec<String>` do pin it, but the second is the exact signature `ptr_arg` just told you not to write, so the honest options are a by-value `Vec<String>` parameter or naming the type at the `collect` — which is what the compiler's own suggestion says. That is the small collision worth remembering: the idiomatic parameter type and use-site inference do not compose, and the lint wins.

## What you can collect into

| Target | What it does that a `Vec` does not |
|---|---|
| `String` | joins `char`s or `&str`s with no separator |
| `HashSet<T>` / `BTreeSet<T>` | drops duplicates; `BTreeSet` also sorts |
| `HashMap<K, V>` / `BTreeMap<K, V>` | from an iterator of **pairs**; a later key **overwrites** an earlier one |
| `VecDeque<T>`, `BinaryHeap<T>`, `LinkedList<T>` | the other std containers, same call |
| `Result<Vec<T>, E>` | one `Result` for the whole sequence — see below |
| `Option<Vec<T>>` | the same, for absence rather than failure |
| `(Vec<A>, Vec<B>)` | from an iterator of pairs, via `unzip` |
| `()` | discards the items; useful only in `Result<(), E>` |

The map row is the one that bites: `collect` is not a merge. `[("Ada", 5), ("Ben", 3), ("Ada", 4)]` collects to `{"Ada": 4, "Ben": 3}` — the second `Ada` overwrote the first, silently. When the duplicates should combine, [`fold`](../fold_and_reduce/README.md) with `entry().or_insert()` is the tool.

## The one everybody needs: `Result<Vec<_>, _>`

An iterator **of** `Result`s collects into one `Result` **of** a collection:

```rust
fn main() {
    let rows = ["5", "3", "0"];
    let parsed: Result<Vec<i32>, _> = rows.into_iter().map(str::parse::<i32>).collect();
    println!("{parsed:?}");   // Ok([5, 3, 0])
}
```

Three things this buys, and one it costs:

- **The shape flips.** `Iterator<Item = Result<T, E>>` becomes `Result<Vec<T>, E>`, so the caller has one thing to check instead of one per row. This is the standard way to parse a file of numbers into a `Vec<i32>` with a `?` at the end.
- **It short-circuits.** On `["5", "no", "0", "also no"]` the closure runs **twice**, not four times. `collect` stops at the first `Err`.
- **`?` works on the result**, which is usually the whole point of the line.
- **Only the first error survives.** If you need all of them — a validation report, a row-by-row error list — `collect` is the wrong consumer. Use `partition(Result::is_ok)`, or collect into `Vec<Result<T, E>>` and sort it out afterwards. On the four rows above, partitioning keeps **two** errors where collecting kept one.

The un-flipped shape is **one annotation away**, on the same pipeline: ask for `Vec<Result<i32, _>>` instead and you get `[Ok(5), Err(..), Ok(0), Err(..)]` — all four rows run, every outcome kept, no short-circuit, because there is nothing to short-circuit *to*. Which of the two you want is the whole decision, and it is made in the type.

`Option<Vec<T>>` behaves identically, with `None` in place of `Err`. If you have met this shape in a functional language, it is [`traverse`/`sequence` ↗](https://hackage.haskell.org/package/base/docs/Data-Traversable.html) — Rust gets it from one `FromIterator` impl rather than a separate combinator.

## `Result<(), E>`: the degenerate case that is actually useful

There is a `FromIterator` impl for `()`, so a sequence of fallible steps whose successes carry no value collects into a single "did it all work":

```rust
let checked: Result<(), _> = rows.into_iter().map(|s| s.parse::<i32>().map(|_| ())).collect();
```

`Ok(())` or the first `Err`, nothing else kept. That is the right shape for a run of side effects — writes, validations, sends — and it is also the reason a stray `let _: () = it.collect();` compiles and silently does nothing.

## Making your own type collectable

One trait, one method:

```rust
impl FromIterator<String> for Roster {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut rows = Vec::new();
        let mut longest = 0;
        for row in iter {
            longest = longest.max(row.len());
            rows.push(row);
        }
        Roster { rows, longest }
    }
}
```

and `let roster: Roster = names.into_iter().map(String::from).collect();` now works. Note the parameter is `IntoIterator`, not `Iterator` — the same reason [a collection is never itself an iterator](../implementing_iterator/README.md).

## What it costs

`collect` **allocates a new collection every time**. Two ways to pay less:

- **`extend`** pours into a collection you already have — the same work without a fresh allocation, and the natural form inside a loop that accumulates across iterations.
- **An honest [`size_hint`](../implementing_iterator/README.md)** lets either one make a single allocation instead of growing by doubling. For a slice-backed iterator you get that free; for a hand-written one it is three lines you have to write.

And a collection you collect into is a claim about your element type, checked at the call: `HashSet` needs `Eq + Hash`, `BTreeSet` needs `Ord`. The error arrives on the `collect` line rather than on the type definition, which is confusing exactly once.

## If you are coming from another language

- **Python.** `list(gen)`, `set(gen)`, `dict(pairs)`, `"".join(gen)` — Python names the target as a *constructor call* and Rust names it as a *type*, but it is the same decision made in the same place, and the dict case has the same last-key-wins behaviour. What Python has no counterpart for is `Result<Vec<_>, _>`: gathering per-row failures into one answer is a `try`/`except` around the whole loop, which is coarser (you lose which row) or a manual accumulator (which is `partition`). And Python's `list()` cannot be extended to your own class in the way `FromIterator` extends `collect` — the nearest thing is writing `MyType(gen)` and accepting the iterable in `__init__`, which works but is not the same generic call site.
- **ABAP.** There is no equivalent, and that is worth saying plainly: building a result collection is always a `LOOP` with an `APPEND`, or the 7.40+ table comprehension `VALUE ty_tab( FOR wa IN lt_src ( CORRESPONDING #( wa ) ) )`, which is the closest ABAP gets — and note it names the target type on the left exactly as Rust does. The pieces with no ABAP counterpart are the interesting ones: `HashSet` deduplication is `DELETE ADJACENT DUPLICATES` after a `SORT`, a separate statement rather than a choice of target; and `Result<Vec<_>, _>` has no analogue at all, since ABAP's failure channel is `sy-subrc` or an exception, neither of which composes over a table. The habit that transfers badly is checking `sy-subrc` per row inside the loop; the Rust shape hoists that decision to the collect.
- **Java / C#.** `.collect(Collectors.toList())` and `.ToList()` are the same idea, and Java's `Collector` is `FromIterator` with more moving parts. The difference is where the type comes from: Java passes a collector *value*, Rust infers the impl from the annotation, which is why Rust's failure mode is "type annotations needed" and Java's is picking the wrong collector. The map case is where the two designs diverge most. Java's two-argument `Collectors.toMap` **throws** — *"an `IllegalStateException` is thrown when the collection operation is performed"* if the mapped keys contain duplicates — so handling them means a three-argument overload with a merge function, and choosing the map implementation means a four-argument one with a supplier. C#'s `ToDictionary` throws the same way. Rust's `collect` does none of that: it takes the target from the annotation and **silently overwrites** on a duplicate key. Fewer knobs, but the duplicate-key behaviour you get by default is the one Java made you ask for explicitly — worth knowing in both directions when porting.

---

## The verified output

<!-- output:collect_and_fromiterator -->
*Verified output of [`collect_and_fromiterator.rs`](examples/collect_and_fromiterator.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One call, and the TYPE decides what gets built
   Vec<&str>      ["Ada", "Ben", "Cara", "Ada"]
   String         "AdaBenCaraAda"
   BTreeSet<&str> {"Ada", "Ben", "Cara"}   (sorted, and the duplicate Ada is gone)
   VecDeque<&str> ["Ada", "Ben", "Cara", "Ada"]
   Same iterator four times. `collect` did not decide any of this;
   the annotation did, by picking whose FromIterator impl runs.

2. Pairs collect into a map
   BTreeMap  {"Ada": 4, "Ben": 3}
   Ada appears twice in the input and once here: for a map, a later
   key overwrites an earlier one. `collect` is not a merge — reach
   for `fold` with `entry().or_insert()` when the totals matter.

3. Turbofish, when there is no variable to annotate
   .collect::<Vec<_>>().len()   = 4
   the `_` is inference doing the element type; only the container
   has to be named, and often only in one of the two places.

4. The one everybody needs: Result<Vec<_>, _> from fallible rows
   good rows -> Ok([5, 3, 0])
   bad rows  -> Err(invalid digit found in string)
   and it stopped after 2 of 4 rows — collect into a Result is
   short-circuiting. Note the shape it flipped: an iterator OF
   Results became one Result OF a Vec, so the caller has one thing
   to check instead of one per row.
   Same pipeline, one annotation apart:
   Vec<Result<_, _>> -> [Ok(5), Err(..), Ok(0), Err(..)]
   all 4 rows ran and every outcome is kept — the un-flipped shape
   has nothing to short-circuit TO, so it does not.
   Only the FIRST error survives. To keep them all, collect into a
   (Vec<_>, Vec<_>) with partition, or Vec<Result<_, _>> and sort it out.
   partitioned -> 2 ok, 2 err (every error kept)

5. Option collects the same way, and so does anything else you write
   all even -> Some([2, 4, 6])
   one odd  -> None   (one None and the whole answer is None)
   our own  -> Roster with 3 rows, longest 10
   one `impl FromIterator<String> for Roster` and `.collect()` works
   on it — the trait is the whole extension point.

6. What collect costs, and the two ways to pay less
   collect  -> [3, 3, 4, 3]  capacity 4
   extend   -> [3, 3, 4, 3]  capacity 4
   `collect` allocates a new collection every time. `extend` pours
   into one you already have, which is the loop-friendly form; and
   an exact `size_hint` is what lets either make one allocation.

7. The useful degenerate case: Result<(), E>
   all rows parse            -> Ok(())
   one row does not          -> Err(invalid digit found in string)
   There is a FromIterator impl for `()`, so collecting an iterator
   of `Result<(), E>` gives one `Result<(), E>`: did every step work,
   with the first failure and nothing else kept. That is the shape
   for a run of fallible side effects whose successes carry no value.
   Same trick, same warning: it stops at the first Err.

8. Deduplicating is a collect, and it costs a trait or two
   three names -> HashSet<Voter> -> ["Ada", "Ben"]
   `Voter` had to derive Eq and Hash to land in a HashSet, and Ord
   to land in a BTreeSet. Which collection you collect into is a
   claim about your type, checked at the collect call.

9. You do not have to WRITE the type — it has to be KNOWABLE
   struct field    -> ["ADA", "BEN", "CARA", "ADA"]
   return position -> ["ADA", "BEN", "CARA", "ADA"]
   by-value arg    -> widest = 4
   Not one of those three `collect` calls names a type, and all
   three compile: inference runs BACKWARD from where the value
   lands. An annotation is not the requirement — a DETERMINED type
   is, and a struct field, a return type or a parameter determines
   one just as well as a binding does.
   The use site that does NOT work is a `&[String]` parameter, which
   is the one Rust otherwise tells you to prefer: `&Vec<String>` ->
   `&[String]` is a deref coercion, and inference will not run a
   coercion backward. It takes `[String]` literally and rejects it
   as unsized — so that call is where you go back to a turbofish.
```
<!-- /output -->

---

## See also

- [Collect the iterator into a `Vec`](../collect_into_a_vec/README.md) — the prior question: whether to materialize at all, and what the `Vec` is bought with
- [`fold` and `reduce`](../fold_and_reduce/README.md) — what to use when the duplicates have to combine rather than overwrite
- [Iterators are lazy](../iterators_are_lazy/README.md) — `collect` is the consumer that makes the chain run
- [Implementing `Iterator`](../implementing_iterator/README.md) — `size_hint`, and why `from_iter` takes an `IntoIterator`
- [`Option` is a one-item collection](../../17_Option_and_Result/option_as_collection/README.md) — the same flip, on a sequence of length 0 or 1
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — which of the two collects you want
- [Making a `String`](../../14_Strings/making_a_string/README.md) — `collect::<String>()` beside the four other spellings

## Sources

[`Iterator::collect` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) and [`FromIterator` ↗](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) — the std page for the trait lists every impl, which is the real answer to "what can I collect into".

Sam Van Overmeire, [*The Many Neat Tricks of Rust's `collect`* ↗](https://medium.com/@sam.van.overmeire/the-many-neat-tricks-of-rusts-collect-ab7e185f6fee) (Feb 2026) — a blog post, and the source of the use-site-inference point above; its closing example drops the annotation entirely and lets a struct field supply it. The slice-parameter limit and the `clippy::ptr_arg` collision are this page's, measured on the pinned toolchain. Its Java comparison is where the `Collectors.toMap` escalation comes from; the throwing behaviour is quoted from the [`Collectors` javadoc ↗](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/stream/Collectors.html) rather than the post.
