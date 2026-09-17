# `for` loops

**Level:** 101 · for newcomers

**One line:** `for x in 1..5` walks a range and `for elem in [2, 4, 8]` walks a collection — and both are the same mechanism, because `for` does not know about ranges or arrays at all, only about **iterators**.

```rust
fn main() {
    for n in 1..5 {
        println!("{n}");                            // 1, 2, 3, 4 — never 5
    }

    let prices = vec![250, 1200, 99];
    let mut total = 0;
    for price in &prices {
        total += price;
    }
    println!("{total} from {} prices", prices.len()); // 1549 from 3 prices
}
```

Whatever follows `in` is handed to [`IntoIterator::into_iter` ↗](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html), and the loop calls `next()` on the result until it returns `None`. A range, an array, a `Vec` and a `&Vec` all implement `IntoIterator`, which is the only reason all of them can follow `in`. [Printing the HIR](../../20_Compilers/printing_the_hir/README.md) shows the rewrite rustc actually performs: a `loop` inside two `match`es.

## `1..5` stops at 4, and `1..=5` does not

`n..m` is **exclusive** of `m`: four turns, the last one 4. `n..=m` is the inclusive form, and the `=` is the whole difference. When the loop needs no value at all, `_` takes its place — `for _ in 0..3` runs three times and binds nothing.

A range whose start is past its end is **empty**, not backwards. `for _ in 5..1` runs zero times, and rustc says nothing about it. Clippy does, at deny level:

```text title="Abridged — real clippy-driver output for backwards_range.rs, without the help URL and the closing summary"
error: this range is empty so it will yield no values
 --> backwards_range.rs:3:14
  |
3 |     for _ in 5..1 {
  |              ^^^^
  |
  = note: `#[deny(clippy::reversed_empty_ranges)]` on by default
help: consider using the following if you are attempting to iterate over this range in reverse
  |
3 -     for _ in 5..1 {
3 +     for _ in (1..5).rev() {
```

## A range is a value, and `for` uses it up

`let r = 1..5;` is an ordinary binding of type `Range<i32>`, with methods of its own (`r.contains(&5)` is `false`). It is also **not `Copy`**, so the first `for i in r` moves it and a second one is refused:

```text title="Abridged — real rustc output for range_reused.rs, without the closing summary"
error[E0382]: use of moved value: `r`
 --> range_reused.rs:6:14
  |
2 |     let r = 1..5;
  |         - move occurs because `r` has type `std::ops::Range<i32>`, which does not implement the `Copy` trait
3 |     for i in r {
  |              - `r` moved due to this implicit call to `.into_iter()`
...
6 |     for i in r {
  |              ^ value used here after move
```

`r.clone()` walks a copy. `&mut r` walks the range itself and leaves behind whatever the loop did not reach: break out of `for i in &mut rest` at 3, and `rest` is `4..10` afterwards. A `Range` is its own iterator, so its current position lives in `start`, which the loop advanced.

## What follows `in` decides what each item is

| you write | each item is | afterwards |
|---|---|---|
| `for p in &prices` | `&i32` | `prices` untouched |
| `for p in &mut prices` | `&mut i32` — write with `*p += 1` | `prices` changed in place |
| `for name in names` | `String`, owned | `names` moved |

Those are the three doors of [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md), chosen by one `&`. An array, a `Vec` and a slice (`&vector[1..]`) all go through them the same way. Section 4 of the output below prints each item's type.

## `for x in v` moves `v` when the loop starts

The move is not something that happens when the loop finishes. It happens on the `for` line, before the first turn — so even a use *inside* the body is too late:

```rust
let container = vec![1, 2, 3];
for item in &container {
    println!("{item} of {}", container.len());   // 1 of 3, 2 of 3, 3 of 3
}
// for item in container { container.len(); }   // error[E0382]: borrow of moved value
```

```text title="Abridged — real rustc output for moved_on_the_first_line.rs, without the closing summary"
error[E0382]: borrow of moved value: `container`
 --> moved_on_the_first_line.rs:4:34
  |
2 |     let container = vec![1, 2, 3];
  |         --------- move occurs because `container` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     for item in container {
  |                 --------- `container` moved due to this implicit call to `.into_iter()`
4 |         println!("{item} of {}", container.len());
  |                                  ^^^^^^^^^ value borrowed here after move
  |
note: `into_iter` takes ownership of the receiver `self`, which moves `container`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/iter/traits/collect.rs:312:17
help: consider iterating over a slice of the `Vec<i32>`'s content to avoid moving into the `for` loop
  |
3 |     for item in &container {
  |                 +
```

The caret lands on the innocent use, but the `for` line carries a label naming the implicit `.into_iter()` call, and the help is the one-character fix. Move the use after the loop and you get the same three spans with `...` between them.

Two things the error does not mean:

- **Not every collection moves.** `[i32; 3]` is `Copy`, so `for x in small` walks a copy and `small` is still usable afterwards.
- **The name is not dead.** A moved `Vec` binding can be assigned again: `v = vec![9];` after `for _ in v` compiles and prints `[9]`. The *value* left; the variable is waiting for a new one.

## `&collection` means whatever that type's `IntoIterator` says

`for x in &words` is not a rule that means `words.iter()`. It calls `IntoIterator::into_iter(&words)`, and it is `Vec`'s implementation that happens to call `iter()` — the output below prints the same type, `core::slice::iter::Iter<'_, &str>`, for both. Arrays, slices, `HashMap` and `BTreeMap` implement it for references too (`for (fruit, n) in &stock` hands out `(&&str, &i32)`).

Types without that implementation refuse, and the message says *iterator* rather than *borrow*:

```text title="Abridged — real rustc output for borrow_a_string.rs, without the closing summary"
error[E0277]: `&String` is not an iterator
 --> borrow_a_string.rs:3:14
  |
3 |     for c in &s {
  |              ^^ `&String` is not an iterator
  |
  = help: the trait `Iterator` is not implemented for `&String`
  = note: required for `&String` to implement `IntoIterator`
```

A `String` has no single answer to *what are its items* — bytes or `char`s — so it makes you say which: `s.chars()` or `s.bytes()` (`"héllo"` is 5 `char`s and 6 bytes). A `&Range` is refused the same way — `` `&std::ops::Range<{integer}>` is not an iterator `` — and there the help is to remove the `&`, because a range is already an iterator.

## A `for` loop evaluates to `()`

`let unit = for _ in 0..0 {};` compiles, and `unit` is `()`. There is no way to hand a value out: `break x` inside a `for` is `E0571`, "`break` with value from a `for` loop". Finding the first match is `iter().find(..)`, which returns an `Option`; a loop that produces a value is [`loop`](../the_loop_keyword/README.md), and leaving early is [`break`](../break_expressions/README.md).

## The loop variable is the item, not the loop's counter

`i` is a new immutable binding on every turn, so `i += 2` in the body is `E0384`:

```text title="Abridged — real rustc output for skip_ahead.rs, without the closing summary"
error[E0384]: cannot assign twice to immutable variable `i`
 --> skip_ahead.rs:4:13
  |
2 |     for i in 0..5 {
  |         - first assignment to `i`
3 |         if i == 1 {
4 |             i += 2;
  |             ^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     for mut i in 0..5 {
  |         +++
```

Take the help and it compiles — and does not do what the C loop did. `for mut i in 0..5` with `i += 2` at 1 produces `[0, 3, 2, 3, 4]`: the change lands on this turn's copy, and the range hands out 2 next anyway. Skipping is something you tell the iterator (`filter`), and a fixed stride is `step_by` — `(0..10).step_by(3)` is `[0, 3, 6, 9]`. The binding also ends with the body: `i` after the closing `}` is `E0425`, "cannot find value `i` in this scope".

## Position, and loops that are really chains

A counter kept by hand beside the loop is `.enumerate()`, and an index into the collection is usually a sign the loop wanted the item instead — both are [Loops without an index](../loops_without_an_index/README.md), and pairing two sequences is [`zip` and `enumerate`](../../24_Iterators/zip_and_enumerate/README.md).

A loop that only filters, transforms and totals is a chain with the plumbing showing:

```rust
let mut even_squares = 0;
for n in 1..=10 {
    if n % 2 == 0 {
        even_squares += n * n;
    }
}
let chained: i32 = (1..=10).filter(|n| n % 2 == 0).map(|n| n * n).sum();
assert_eq!(even_squares, chained);                 // both 220
```

Which to write is not a matter of taste in every case: [When a `for` loop beats a chain](../../24_Iterators/when_a_loop_beats_a_chain/README.md) lists the ones where the loop wins, and [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) explains why a chain with no `sum` or `collect` at the end does nothing.

## If you are coming from another language

**Python.** The protocol is the one you know. Python's `for` calls `iter()` and then `next()` until `StopIteration`; Rust's calls `into_iter()` and then `next()` until `None`. `range(1, 5)` is exclusive exactly as `1..5` is, and `range(5, 1)` is empty exactly as `5..1` is. Python has no inclusive range, so `1..=5` is new. Assigning to the loop variable does not skip ahead in Python either — `i += 2` inside `for i in range(5)` prints `0 3 2 3 4` in both languages.

Three things change. Python's `for x in xs` never moves `xs`; the Rust spelling of that loop is `for x in &xs`, and plain `for x in xs` is the one that gives the list away. Python's loop variable outlives the loop — after `for i in range(3): pass`, `i` is 2 — and Rust's is gone at the closing brace. And Python lets you change the list you are walking: `for x in queue: queue.append(...)` visits the appended items too (on Python 3.14.7, a three-item list with two appends makes five turns). Rust refuses to compile the same loop over `&queue`, with `E0502`; [iterator invalidation](../../31_C_and_Cpp/iterator_invalidation/README.md) is what that rule prevents.

**C.** The three-part `for (init; cond; step)` has no Rust spelling. The common case, `for (int i = 0; i < n; i++)`, is `for i in 0..n`, and the `<` is the `..` — write `i <= n` and you want `0..=n`. The rest map onto iterator methods rather than onto the header: a stride is `step_by`, a countdown is `(0..n).rev()` (which has no spelling for the `size_t i; i >= 0` loop that never ends), and a loop that changes `i` in its body is a `while` in Rust, because a `for` variable is not the counter — the `mut i` section above is what happens when you try.

C++'s range-based `for (auto x : v)` is the closer relative. `const auto& x` is `&v` and `auto& x` is `&mut v`; plain `auto x` copies each element, which Rust spells `v.iter().cloned()`. The third door has no C++ counterpart: a range-`for` never takes the container, so no later line can find it gone.

**ABAP.** `DO n TIMES. … ENDDO.` is `for _ in 0..n`, and `sy-index` is the counter you did not have to declare — with two differences. `sy-index` starts at 1, where a Rust range starts wherever you say (`1..=n` if you want ABAP's numbering). And `sy-index` is a system field, while a Rust loop variable is a binding that nothing else can overwrite.

`LOOP AT itab` is the collection loop, and two of its forms are Rust doors: `ASSIGNING <fs>` writes into the table like `&mut v`, and `REFERENCE INTO` hands out a pointer like `&v`. The everyday form, `INTO wa`, copies each row and leaves the table alone — the nearest Rust is `for row in rows.iter().cloned()`, a copy per turn that you now have to ask for. The door ABAP does not have is the consuming one: nothing in `LOOP AT` ends the table, so `for row in rows` making `rows` unusable is new. So is the compile-time refusal: `APPEND` or `DELETE` on the table you are looping over is legal ABAP, and in Rust it is `E0502` before the program runs. `EXIT`, `CONTINUE` and `CHECK` are [`break`](../break_expressions/README.md), [`continue`](../continue_expressions/README.md), and `if !condition { continue; }`.

## Practice

**Countdown, then three doors.** Print a countdown from 10 to 1. Write the version a C habit produces first — `for n in 10..1` — and predict how many lines it prints before you run it. Then write it correctly two ways.

Then take `let mut scores = vec![72, 85, 90];` and write three loops: print the total and still be able to use `scores`; add 5 to every score in place; and turn the scores into a `Vec<String>` of `"score: 77"` labels. One of the three has to come last. Say which, and what rustc reports — the error code, and which line it points at — if you move it first.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:for_loops_kata -->
*[`for_loops_kata.rs`](examples/for_loops_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a countdown that prints nothing, then three loops over one
//! Vec in the only order that compiles.
//!
//!   rustc --edition 2024 for_loops_kata.rs -o /tmp/flk && /tmp/flk

fn main() {
    println!("=== part 1: the countdown ===");
    let mut lines = 0;
    for _ in 10..1 {
        lines += 1;
    }
    println!("  for n in 10..1       -> {lines} lines   <- start > end is an EMPTY range, not a backwards one");

    let mut out = Vec::new();
    for n in (1..=10).rev() {
        out.push(n.to_string());
    }
    println!("  for n in (1..=10).rev() -> {}", out.join(" "));

    out.clear();
    for n in (1..11).rev() {
        out.push(n.to_string());
    }
    println!("  for n in (1..11).rev()  -> {}   <- same, but the 11 makes you do arithmetic", out.join(" "));
    println!("  rustc compiles 10..1 without a word; clippy's reversed_empty_ranges is deny-by-default");

    println!();
    println!("=== part 2: three doors, one Vec ===");
    let mut scores = vec![72, 85, 90];

    // Door 1: &scores lends each item as &i32, so scores survives.
    let mut total = 0;
    for s in &scores {
        total += s;
    }
    println!("  for s in &scores     -> total {total}, scores still {scores:?}");

    // Door 2: &mut scores lends each item as &mut i32; write through it with *.
    for s in &mut scores {
        *s += 5;
    }
    println!("  for s in &mut scores -> *s += 5, scores now {scores:?}");

    // Door 3: scores itself is moved into the loop. This one has to come last.
    let mut labels: Vec<String> = Vec::new();
    for s in scores {
        labels.push(format!("score: {s}"));
    }
    println!("  for s in scores      -> labels {labels:?}");

    println!();
    println!("=== why door 3 comes last ===");
    println!("  `for s in scores` moves the Vec when that loop STARTS. Put it first and the");
    println!("  next loop is refused: error[E0382]: borrow of moved value: `scores`.");
    println!("  The caret sits on `&scores` in `for s in &scores`, the first use after the move;");
    println!("  the consuming `for` line is labelled");
    println!("  \"`scores` moved due to this implicit call to `.into_iter()`\".");
    println!("  One error, not two: the `&mut scores` loop after it is not reported separately.");
    println!("  The help is a one-character fix on the consuming loop: `for s in &scores`.");
}
```
<!-- /source -->

<!-- output:for_loops_kata -->
*Verified output of [`for_loops_kata.rs`](examples/for_loops_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== part 1: the countdown ===
  for n in 10..1       -> 0 lines   <- start > end is an EMPTY range, not a backwards one
  for n in (1..=10).rev() -> 10 9 8 7 6 5 4 3 2 1
  for n in (1..11).rev()  -> 10 9 8 7 6 5 4 3 2 1   <- same, but the 11 makes you do arithmetic
  rustc compiles 10..1 without a word; clippy's reversed_empty_ranges is deny-by-default

=== part 2: three doors, one Vec ===
  for s in &scores     -> total 247, scores still [72, 85, 90]
  for s in &mut scores -> *s += 5, scores now [77, 90, 95]
  for s in scores      -> labels ["score: 77", "score: 90", "score: 95"]

=== why door 3 comes last ===
  `for s in scores` moves the Vec when that loop STARTS. Put it first and the
  next loop is refused: error[E0382]: borrow of moved value: `scores`.
  The caret sits on `&scores` in `for s in &scores`, the first use after the move;
  the consuming `for` line is labelled
  "`scores` moved due to this implicit call to `.into_iter()`".
  One error, not two: the `&mut scores` loop after it is not reported separately.
  The help is a one-character fix on the consuming loop: `for s in &scores`.
```
<!-- /output -->

</details>

## The verified output

<!-- output:for_loops -->
*Verified output of [`for_loops.rs`](examples/for_loops.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== 1. n..m stops before m; n..=m includes it ===
  1..5               -> [1, 2, 3, 4]      4 turns, the last one is 4
  1..=5              -> [1, 2, 3, 4, 5]   5 turns
  for _ in 0..3      -> 3 turns, and no loop variable to name
  for _ in 5..1      -> 0 turns  <- empty, not reversed; rustc says nothing
  (1..5).rev()       -> [4, 3, 2, 1]

=== 2. a range is a value, and for consumes it ===
  let r = 1..5;      type core::ops::range::Range<i32>
  r.contains(&5)     = false
  for i in r.clone() -> [1, 2, 3, 4]   (Range is not Copy; a second `for i in r` is E0382)
  for i in &mut rest, break at 3 -> rest is now 4..10

=== 3. arrays, Vecs and slices go through the same door ===
  for elem in array        -> [2, 4, 8]
  for elem in &vector      -> [2, 4, 8]
  for elem in &vector[1..] -> [4, 8]

=== 4. what you hand to for decides what each item is ===
  for p in &prices      item: &i32                    read  -> total = 1549, prices still [250, 1200, 99]
  for p in &mut prices  item: &mut i32                write -> *p += 1, prices now [251, 1201, 100]
  for name in names     item: alloc::string::String   owned -> kept = ["Ada", "Ben"], names is gone

=== 5. the move happens as the loop starts, and the name survives it ===
  [i32; 3] is Copy: after `for x in small`, sum = 6, small = [1, 2, 3]
  Vec is not Copy: `for _ in v` ran 3 turns, then `v = vec![9]` -> v = [9]

=== 6. `for x in &c` calls IntoIterator on the REFERENCE, whatever that does ===
  (&words).into_iter()  core::slice::iter::Iter<'_, &str>
  words.iter()          core::slice::iter::Iter<'_, &str>
  for (fruit, n) in &stock -> apples = 3   fruit: &&str, n: &i32
  for (fruit, n) in &stock -> pears = 0   fruit: &&str, n: &i32
  for c in &s is E0277 (&String is not an iterator); s.chars() -> ['h', 'é', 'l', 'l', 'o']
  s.bytes().count() = 6, s.chars().count() = 5

=== 7. enumerate, not a counter kept by hand ===
  0: alpha
  1: beta

=== 8. a for loop evaluates to (), so it cannot hand a value out ===
  let unit = for _ in 0..0 {};  unit = ()
  first element > 3, found without a loop: Some(4)

=== 9. many for loops are a chain in disguise ===
  loop:  sum of even squares in 1..=10 = 220
  chain: (1..=10).filter(even).map(square).sum() = 220

=== 10. the loop variable is the item, not the loop's counter ===
  for mut i in 0..5, i += 2 when i == 1 -> [0, 3, 2, 3, 4]   <- 2 still arrives
  to skip, say so to the iterator: filter  -> [0, 3, 4]
  a fixed stride (C's i += 3) is step_by: (0..10).step_by(3) -> [0, 3, 6, 9]
```
<!-- /output -->

## See also

- [`while` loops](../while_loops/README.md) — the loop with a condition instead of a sequence
- [`loop`](../the_loop_keyword/README.md) — the one with neither, and the only one that hands out a value
- [`break` expressions](../break_expressions/README.md) and [`continue` expressions](../continue_expressions/README.md) — leaving a `for` early, or skipping a turn
- [Loop labels](../loop_labels/README.md) — leaving a nested `for` from the inside
- [Loops without an index](../loops_without_an_index/README.md) — why `for i in 0..v.len()` is rarely the loop you want
- [Flow control](../flow_control/README.md) — every construct in this chapter, what it evaluates to, and how you leave it
- [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) — the three questions `for` is really asking
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — why the chain you write instead of a loop does no work until you ask
- [Printing the HIR](../../20_Compilers/printing_the_hir/README.md) — the `loop` and `match` a `for` becomes
- [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md) — the macro to reach for when you want to see each turn
- [Comprehensive Rust: `for` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/loops/for.html)

## Sources

*Rust in Action* (Tim McNamara, Manning 2021), §2.4.1 "For: The central pillar of iteration" — the claims about ranges, anonymous loops, the three reference forms, and what happens to the container, each re-run above; [What *Rust in Action* says about flow control, run](../flow_control_claims_checked/README.md) has the verdicts in one table. The Reference on [iterator loops ↗](https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops), [`IntoIterator` ↗](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) in std, and Clippy's [`reversed_empty_ranges` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#reversed_empty_ranges).

## Po polsku

`for` nie wie nic o zakresach ani o tablicach — zna wyłącznie iteratory. To, co stoi po `in`, trafia do `IntoIterator::into_iter`, a pętla woła `next()` aż do `None`; dlatego `for x in 1..5` i `for elem in [2, 4, 8]` to ten sam mechanizm, a sam zakres jest zwyczajną wartością, którą można przypisać (`let r = 1..5;`). Po polsku najłatwiej zapamiętać `1..5` jako przedział prawostronnie otwarty: cztery obiegi, ostatni dla 4 — wersję domkniętą pisze się `1..=5` i cała różnica siedzi w tym jednym znaku `=`. Zakres „od tyłu”, `5..1`, jest po prostu **pusty**: pętla nie wykona się ani razu, `rustc` milczy, a dopiero Clippy zgłasza błąd `reversed_empty_ranges` i podpowiada `(1..5).rev()`. Zakres nie jest też `Copy`, więc drugie `for i in r` to `E0382`.

Pułapka jest gdzie indziej: `for x in v` **przenosi własność** wektora, i to w chwili startu pętli, a nie po jej zakończeniu — już `v.len()` w ciele tej samej pętli to `E0382`. Kompilator stawia znacznik przy niewinnym użyciu, ale linię `for` opisuje jako „niejawne wywołanie `.into_iter()`” i podpowiada lekarstwo jednym znakiem: `for x in &v`. Wybór między `&v`, `&mut v` a samym `v` to wybór między `iter`, `iter_mut` a `into_iter` — pożyczyć do odczytu, pożyczyć do zapisu albo skonsumować. Tablica typów `Copy` przeżywa pętlę, a przeniesioną zmienną wolno ponownie przypisać — umiera wartość, nie nazwa. Zapis `&kolekcja` nie jest ogólną regułą „to znaczy `.iter()`”: działa tylko tam, gdzie typ ma odpowiednią implementację, stąd `for c in &s` dla `String` kończy się `E0277` i trzeba wybrać `s.chars()` albo `s.bytes()`.

Czytelnikowi z C przyda się jeszcze jedno: zmienna pętli to kolejny element, a nie licznik. `i += 2` w ciele to `E0384`, a po dodaniu podpowiadanego `mut` program się kompiluje, ale niczego nie przeskakuje (`[0, 3, 2, 3, 4]`) — do przeskakiwania służą `filter` i `step_by`. W ABAP-ie odpowiednikiem jest `DO n TIMES` z `sy-index` liczonym od 1 oraz `LOOP AT` z wariantami `INTO`, `ASSIGNING` i `REFERENCE INTO`; nowością jest wariant konsumujący i to, że modyfikacja tabeli w trakcie pętli jest odrzucana już przy kompilacji. Sama pętla `for` ma wartość `()`, więc wynik wynosi się przez `find` albo przez `loop` z `break wartość`, a indeks bierze się z `.enumerate()`, nie z ręcznie prowadzonego licznika.

**Szukaj po polsku:** pętla for w Ruscie · przedział prawostronnie otwarty · przeniesienie własności w pętli · zmienna sterująca pętli · `rust for loop moves vector E0382` · `rust range inclusive 1..=5` · `rust for mut i skip`
