# `break`

**Level:** 101 → 201 · for newcomers

**One line:** `break` ends the innermost `loop`, `while` or `for` around it, on the spot and no further out — which is the trap in a nested loop — and from a `loop` or a labelled block it can carry a value out with it.

```rust
fn main() {
    let temps = [12, 15, 31, 18, 33];
    let mut first_hot = None;
    for (i, &t) in temps.iter().enumerate() {
        if t > 30 {
            first_hot = Some(i);
            break; // 18 and 33 are never looked at
        }
    }
    println!("{first_hot:?}"); // Some(2)
}
```

All three loops accept a bare `break`, and control lands on the first statement after the loop. `continue` is its sibling on [its own page](../continue_expressions/README.md); choosing *which* loop to leave is [loop labels](../loop_labels/README.md).

## `break` leaves the innermost loop, and only that one

```rust
fn main() {
    let grid = [[1, -2, 3], [4, 5, 6], [-7, 8, 9]];
    let mut found = None;
    for (r, row) in grid.iter().enumerate() {
        for (c, &n) in row.iter().enumerate() {
            if n < 0 {
                found = Some((r, c));
                break; // leaves the column loop; the row loop carries on
            }
        }
    }
    println!("{found:?}"); // Some((2, 0)), not the first negative at (0, 1)
}
```

```text title="From the verified output below"
   rows visited: [0, 1, 2]
   found = Some((2, 0)), assigned 2 times
```

The `break` ended the column loop for row 0 and nothing else. The row loop went on to row 1 and row 2, met `-7`, and overwrote the answer. Nothing warns, because leaving only the inner loop is a legitimate thing to want, and the code reads correctly at a glance.

Two fixes, and neither is a flag variable: put a label on the row loop and write `break 'rows;` — see [loop labels](../loop_labels/README.md) — or make the search a value with `find_map`, as [When a `for` loop beats a chain](../../24_Iterators/when_a_loop_beats_a_chain/README.md#3-when-you-need-out-of-two-levels-at-once) compares.

## The book's endless `zip` stops at (51, 51)

*Rust in Action* §2.4.5 introduces `break` with `for (x, y) in (0..).zip(0..) { if x + y > 100 { break; } }`. Both ranges are unbounded, so the `break` is the only way that loop ever ends:

```text title="From the verified output below"
   stopped at (x, y) = Some((51, 51)) on pass 52
```

`50 + 50` is 100, which is not `> 100`, so the pair that trips the test is (51, 51) on the 52nd pass.

## In a `match` arm, `break` leaves the loop

```rust
fn main() {
    let tokens = ["3", "4", "x", "5"];
    let mut sum = 0;
    for t in tokens {
        let n: i32 = match t.parse() {
            Ok(n) => n,
            Err(_) => break, // leaves the for; "5" is never read
        };
        sum += n;
    }
    println!("{sum}"); // 7
}
```

A `match` is not something you can break out of, so a `break` in an arm always belongs to the loop around the `match`. That is the opposite of C, where `break` inside a `switch` leaves the `switch`.

The `Err(_) => break` arm type-checks beside `Ok(n) => n` because `break` is an expression of type [`!`](../../15_First_Programs/the_never_type/README.md): control never comes back from it, so it fits wherever an `i32` is expected.

## `break` with a value: `loop` and labelled blocks only

```rust
fn main() {
    let mut n = 0;
    let first_square_over_20 = loop {
        n += 1;
        if n * n > 20 {
            break n;
        }
    };
    let from_range = (1..).find(|n| n * n > 20);
    println!("{first_square_over_20} {from_range:?}"); // 5 Some(5)
}
```

A `loop` has no condition to fail and no iterator to run out, so the only way it finishes with a value is a `break` that brings one. A `for` or `while` can also end by running out, and then there would be no value — so rustc refuses `break n` there:

```text title="Real rustc output for break_value_in_for.rs"
error[E0571]: `break` with value from a `for` loop
 --> break_value_in_for.rs:4:13
  |
2 |     let found = for n in 1..10 {
  |                 -------------- you can't `break` with a value in a `for` loop
3 |         if n * n > 20 {
4 |             break n;
  |             ^^^^^^^ can only break with a value inside `loop` or breakable block
  |
help: use `break` on its own without a value inside this `for` loop
  |
4 -             break n;
4 +             break;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0571`.
```

The `while` version is the same error code, worded `` `break` with value from a `while` loop ``, and a label on the `for` does not help: `break 'search n` is still `E0571`. The *breakable block* in the message is a [labelled block](../loop_labels/README.md#a-labelled-block-is-breakable-without-a-loop), `'name: { … break 'name value; … }`. For a `for`-shaped search, the value you wanted is usually an `Option`, which is what `find` returns. [The `loop` keyword](../the_loop_keyword/README.md) covers `break value` in full; a bare `break` in a `loop` gives `()`.

## `break` outside a loop, or inside a closure

```rust
fn main() {
    let n = 3;
    if n > 2 {
        // break;   // error[E0268]: `break` outside of a loop or labeled block
    }
}
```

```text title="Real rustc output for break_outside_a_loop.rs"
error[E0268]: `break` outside of a loop or labeled block
 --> break_outside_a_loop.rs:4:9
  |
4 |         break;
  |         ^^^^^ cannot `break` outside of a loop or labeled block

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0268`.
```

A closure is a function body of its own, so a `break` inside one cannot reach a loop outside it — not even the loop that `for_each` runs for you:

```text title="Real rustc output for break_in_closure.rs"
error[E0267]: `break` inside of a closure
 --> break_in_closure.rs:5:13
  |
3 |     words.iter().for_each(|w| {
  |                           --- enclosing closure
4 |         if *w == "stop" {
5 |             break;
  |             ^^^^^ cannot `break` inside of a closure

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0267`.
```

The closure's way of saying *stop* is a return value the iterator understands. [`try_for_each` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_for_each) stops at the first [`ControlFlow::Break` ↗](https://doc.rust-lang.org/std/ops/enum.ControlFlow.html) and hands it back:

```rust
use std::ops::ControlFlow;

fn main() {
    let words = ["alpha", "beta", "stop", "gamma"];
    let result = words.iter().try_for_each(|&w| {
        if w == "stop" {
            return ControlFlow::Break(w);
        }
        println!("{w}"); // alpha, then beta
        ControlFlow::Continue(())
    });
    println!("{result:?}"); // Break("stop")
}
```

## When "stop at the first match" is the whole loop, it is a value

The opening loop exists to compute one `Option<usize>`. Iterator consumers compute the same thing and stop at the same place:

```text title="From the verified output below"
   for + break            -> Some(2) after 3 passes
   position(t > 30)       -> Some(2) after 3 calls
   any(t > 30)            -> true after 3 calls
   find(t > 30)           -> Some(31) after 3 calls
   filter(t > 30).count() -> 2 after 5 calls   <- counting has to see all five
```

[`position` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position), [`any` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) and [`find` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find) each return on the first `true`, so each ran its closure three times — the same three passes the `break` loop made. The difference is what you are left holding: no `mut` binding declared above the loop, and nothing to forget to assign. [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) shows why a consumer stops pulling once it has its answer, and [Adapters by job](../../24_Iterators/adapters_by_job/README.md) has `take_while`, which is `break` in the middle of a chain.

Keep the loop when the body does more than search — two accumulators, a `?` that should name its row — which is [When a `for` loop beats a chain](../../24_Iterators/when_a_loop_beats_a_chain/README.md).

## The verified output

<!-- output:break_expressions -->
*Verified output of [`break_expressions.rs`](examples/break_expressions.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. break leaves the innermost loop only
   rows visited: [0, 1, 2]
   found = Some((2, 0)), assigned 2 times
   The first negative is at (0, 1). The row loop kept going after
   the break, met -7 on row 2, and overwrote the answer.
   with break 'rows: found = Some((0, 1))

2. The book's endless zip, ended by break
   stopped at (x, y) = Some((51, 51)) on pass 52
   50 + 50 = 100 is not > 100, so (51, 51) is the first pair that is.

3. break in a match arm leaves the loop, and has type !
   read ["3", "4", "x"], sum = 7
   "5" was never read: the break left the for loop, not the match.

4. break with a value: loop and labelled blocks only
   loop { .. break n; }              -> 5
   'size: { .. break 'size "big"; }  -> "big"
   loop { break; }                   -> ()   <- a bare break is break ()
   (1..).find(|n| n * n > 20)        -> Some(5)   <- the for-shaped answer is an Option

5. Stop at the first match as a value: position, any, find
   for + break            -> Some(2) after 3 passes
   position(t > 30)       -> Some(2) after 3 calls
   any(t > 30)            -> true after 3 calls
   find(t > 30)           -> Some(31) after 3 calls
   filter(t > 30).count() -> 2 after 5 calls   <- counting has to see all five

6. A closure cannot break; try_for_each stops on ControlFlow::Break
   seen = ["alpha", "beta"]
   result = Break("stop")
```
<!-- /output -->

## If you are coming from another language

**Python.** `break` is the same statement and leaves the innermost loop only, just as here. Python has no labelled `break` — [PEP 3136 ↗](https://peps.python.org/pep-3136/) proposed one in 2007 and Guido van Rossum rejected it, judging code that needs it rare and the feature more likely to be misused — so the Python ways out of two loops are a flag, moving the loops into a function and using `return`, or raising an exception. Rust has the label, and `return` from a helper function works the same in both. Python's `for … else`, whose `else` runs only when no `break` happened, has no Rust counterpart; a `find` returning `None` or a labelled block with a fallback value at its end says the same thing as a value.

**C.** Two differences, both about which construct `break` belongs to. In C, `break` inside a `switch` leaves the `switch` and the loop around it keeps going; in Rust a `match` is never a `break` target, so `break` in an arm always leaves the loop. And C's `break` cannot carry a value or pick an outer loop, which is why C code leaves nested loops with a flag or a `goto` — see [loop labels](../loop_labels/README.md#no-goto-and-the-cleanup-it-was-for-is-drop). What the compiler now enforces: a `break` that could end a loop without a value is refused if you give it one (`E0571`), and a `break` outside a loop is `E0268` rather than something to discover at run time.

**JavaScript and Java.** `break` behaves the same, including leaving only the innermost loop, and both languages have labels (`outer: for (…)`) with the same job as Rust's. Rust's differences are the quote in `'outer` and that `break` can carry a value.

**ABAP.** `EXIT` inside `DO`, `WHILE`, `LOOP AT` or `SELECT … ENDSELECT` is `break`: it ends the innermost loop only and processing resumes after its closing statement — the same nested-loop trap, since leaving an outer `DO` from an inner one needs a flag set before the inner `EXIT` and checked after the inner `ENDDO`. Rust gives that case a label instead. Two habits do not transfer. Outside a loop, ABAP's `EXIT` leaves the whole processing block (SAP's guideline is to use it only in loops), where Rust's `break` there is `E0268`. And an ABAP loop has no value, so the result is always a variable declared before `DO`; in Rust the `loop` or labelled block can *be* the value.

## See also

- [Flow control](../flow_control/README.md) — the chapter's overview, and where `break` sits among the other jumps
- [`continue`](../continue_expressions/README.md) — the other half: skip the rest of this pass instead of the rest of the loop
- [Loop labels](../loop_labels/README.md) — `break 'outer`, labelled blocks, and why Rust has no `goto`
- [The `loop` keyword](../the_loop_keyword/README.md) — the loop whose `break value` becomes the loop's value
- [`for` loops](../for_loops/README.md) · [`while` loops](../while_loops/README.md) — the two loops `break` can leave but cannot take a value out of
- [The never type `!`](../../15_First_Programs/the_never_type/README.md) — why `Err(_) => break` fits in an arm that must be an `i32`
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — `find`, `any` and `position`, and exactly when each one stops pulling
- [*Rust in Action*, claims checked](../flow_control_claims_checked/README.md) — the book's §2.4 run claim by claim

## Sources

- [The Rust Reference: `break` expressions ↗](https://doc.rust-lang.org/reference/expressions/loop-expr.html#break-expressions) and [`break` and loop values ↗](https://doc.rust-lang.org/reference/expressions/loop-expr.html#break-and-loop-values) — innermost-loop rule, type `!`, values only from `loop` and labelled blocks
- Error index: [E0268 ↗](https://doc.rust-lang.org/error_codes/E0268.html) · [E0571 ↗](https://doc.rust-lang.org/error_codes/E0571.html) · [E0267 ↗](https://doc.rust-lang.org/error_codes/E0267.html)
- *Rust in Action* (Tim McNamara, Manning 2021), §2.4.5 — the `(0..).zip(0..)` example measured above
- [PEP 3136 ↗](https://peps.python.org/pep-3136/) (rejected) and [SAP ABAP Keyword Documentation: `EXIT` in loops ↗](https://help.sap.com/doc/abapdocu_752_index_htm/7.52/en-US/abapexit_loop.htm)

## Po polsku

`break` kończy pętlę — i tylko tę **najbliższą**, w której stoi. Pułapka siedzi w zagnieżdżeniu: w siatce `[[1, -2, 3], [4, 5, 6], [-7, 8, 9]]` szukanie pierwszej liczby ujemnej z `break` w pętli wewnętrznej zwraca `(2, 0)` zamiast `(0, 1)`, bo `break` zakończył tylko przegląd kolumn, a pętla po wierszach poszła dalej, trafiła na `-7` i nadpisała wynik. Ostrzeżenia nie będzie, bo wyjście z samej pętli wewnętrznej bywa całkiem sensownym życzeniem. Naprawa to etykieta (`break 'rows`) albo wyszukiwanie zapisane jako wartość (`find_map`) — nie zmienna-flaga.

Dwie różnice wobec C. `break` w ramieniu `match` opuszcza **pętlę**, a nie `match` (w C `break` w `switch` wychodzi tylko ze `switch`). A sam `break` jest wyrażeniem typu `!`, dlatego `Err(_) => break` pasuje obok ramienia zwracającego `i32`. `break` z wartością (`break n`) działa wyłącznie w `loop` i w bloku z etykietą; w `for` i `while` to `E0571`, bo te pętle mogą się skończyć same, bez żadnego `break`, i wtedy wartości by nie było. Poza pętlą `break` to `E0268`, a w domknięciu (np. w `for_each`) — `E0267`; tam rolę `break` przejmuje `try_for_each` z `ControlFlow::Break`. Najczęściej zaś najlepszą odpowiedzią nie jest ani `break`, ani flaga, tylko `.find()`, `.any()` czy `.position()`, które wyrażają „zatrzymaj się na pierwszym trafieniu” jako wartość — i wywołują domknięcie dokładnie tyle razy, ile obiegów zrobiłaby pętla z `break`.

W ABAP-ie odpowiednikiem jest `EXIT` w `DO`, `WHILE` czy `LOOP AT`: też kończy tylko najbliższą pętlę, więc wyjście z dwóch poziomów wymaga flagi. Poza pętlą `EXIT` opuszcza cały blok przetwarzania, a w Ruscie `break` w tym miejscu po prostu się nie skompiluje.

**Szukaj po polsku:** przerwanie pętli · przerwanie pętli zagnieżdżonej · `break` z wartością · `rust break with value` · `rust break inside closure E0267` · `rust break outside of a loop E0268`
