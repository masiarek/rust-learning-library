# `continue`

**Level:** 101 · for newcomers

**One line:** `continue` abandons the rest of this pass and goes back to the loop's head — in a `for` that is the next item, in a `while` it is the condition again — so a `continue` placed above a `while` loop's `i += 1` never lets the counter move.

```rust
fn main() {
    let mut sum = 0;
    for n in 0..10 {
        if n % 2 == 0 {
            continue; // skip the rest of this pass; the range still hands out n + 1
        }
        sum += n;
    }
    println!("{sum}"); // 25
}
```

This is the common shape: a guard at the top of the body turns away the items you do not want, so the rest of the body can assume they are gone. Leaving the loop altogether is [`break`](../break_expressions/README.md); abandoning a pass of an *outer* loop is `continue 'outer`, on [loop labels](../loop_labels/README.md#continue-outer-abandons-the-rest-of-an-outer-pass).

## In a `for`, the iterator advances anyway

```text title="From the verified output below"
   visited [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
   sum of the odd ones = 25
```

Every `n` was visited, including the five even ones the body skipped. The head of a `for` is a call to the iterator's `next`, and a `continue` jumps straight to that call — the loop's position lives in the iterator, not in anything the body could skip.

## In a `while`, a `continue` above the increment loops forever

A `while` loop's head is its condition, and nothing else. If the counter moves in the body, a `continue` above that line jumps past it:

```rust
fn main() {
    let readings = [4, 7, -1, 5, 6]; // -1 is a sensor error, to be skipped
    let mut i = 0;
    let mut total = 0;
    let mut passes = 0;
    while i < readings.len() {
        passes += 1;
        if passes == 20 {
            break; // a safety cap, so this snippet ends; without it, it never does
        }
        if readings[i] < 0 {
            continue; // skips `i += 1` below, so i never moves again
        }
        total += readings[i];
        i += 1;
    }
    println!("i = {i}, total = {total}"); // i = 2, total = 11
}
```

```text title="From the verified output below"
   (pass, i): [(3, 2), (10, 2), (20, 2)]
   total = 11   <- 5 and 6 were never added
```

From the third pass on, `i` is 2 and stays 2: `readings[2]` is `-1`, the `continue` sends control back to `i < readings.len()`, which is still true, and the same reading is tested again. No warning, no panic — the program just stops making progress. Two fixes, and the second is the one to reach for:

```rust
fn main() {
    let readings = [4, 7, -1, 5, 6];

    let mut i = 0;
    let mut total = 0;
    while i < readings.len() {
        let r = readings[i];
        i += 1; // advance first, so nothing below can skip it
        if r < 0 {
            continue;
        }
        total += r;
    }
    println!("{total}"); // 22

    let mut total = 0;
    for &r in &readings {
        if r < 0 {
            continue;
        }
        total += r;
    }
    println!("{total}"); // 22
}
```

A `for` has no counter for the body to forget, which is [the case for a `for`](../for_loops/README.md) and for [loops without an index](../loops_without_an_index/README.md) generally; [`while` loops](../while_loops/README.md) keeps the `while` for when the number of passes really is unknown.

## In a `loop`, and with `let … else`

`continue` means the same thing in a `loop`: back to the top. With `let … else` it becomes a one-line guard that also unpacks the value you wanted:

```rust
fn main() {
    let lines = ["# settings", "", "width = 80", "oops", "height = 24"];
    let mut it = lines.iter();
    let mut settings = Vec::new();
    loop {
        let Some(line) = it.next() else { break };
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(" = ") else { continue };
        settings.push((key, value));
    }
    println!("{settings:?}"); // [("width", "80"), ("height", "24")]
}
```

`"oops"` has no ` = ` in it, so `split_once` returns `None` and the `else { continue }` drops the line. The `else` block of a `let … else` must not finish normally, and `continue` qualifies because, like `break`, it has type [`!`](../../15_First_Programs/the_never_type/README.md). The same type is what lets it sit in a `match` arm beside a value:

```rust
fn main() {
    let raw = ["12", "x", "30", "", "7"];
    let mut kept = Vec::new();
    for s in raw {
        let n: u32 = match s.parse() {
            Ok(n) => n,
            Err(_) => continue, // type !, so it fits where a u32 goes
        };
        kept.push(n);
    }
    println!("{kept:?}"); // [12, 30, 7]
}
```

## `filter` is the iterator form

An `if … { continue; }` guard at the top of a body is a filter, and a chain says so directly:

```text title="From the verified output below"
   (0..10).filter(odd).sum()          = 25
   readings.filter(r >= 0).sum()      = 22
   raw.filter_map(parse.ok()).collect = [12, 30, 7]
```

Same three answers as the loops above. `filter_map` is the `match … Err(_) => continue` loop in one call: the closure returns an `Option`, and `None` is the skip. [Adapters by job](../../24_Iterators/adapters_by_job/README.md) sorts `filter`, `filter_map` and `take_while` by what each one keeps — `take_while` being `break`, not `continue`.

## In a closure, `continue` is refused and `return` does its job

A closure is a function body of its own, so there is no loop inside it for `continue` to return to — not even when `for_each` calls it once per item:

```text title="Real rustc output for continue_in_closure.rs"
error[E0267]: `continue` inside of a closure
 --> continue_in_closure.rs:4:13
  |
2 |     (0..5).for_each(|n| {
  |                     --- enclosing closure
3 |         if n % 2 == 0 {
4 |             continue;
  |             ^^^^^^^^ cannot `continue` inside of a closure

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0267`.
```

Inside `for_each`, `return` ends this call of the closure, and `for_each` calls it again for the next item — which is exactly a `continue`. The same word in a `for` body means something much bigger:

```rust
fn odd_until_even(xs: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    for &n in xs {
        if n % 2 == 0 {
            return out; // leaves the whole function, not just this pass
        }
        out.push(n);
    }
    out
}

fn main() {
    let xs = [1, 3, 4, 5];
    let mut odd = Vec::new();
    xs.iter().for_each(|&n| {
        if n % 2 == 0 {
            return; // ends this call; for_each goes on to 5
        }
        odd.push(n);
    });
    println!("{odd:?} {:?}", odd_until_even(&xs)); // [1, 3, 5] [1, 3]
}
```

That is the trap in rewriting a loop as `for_each` or back: a `return` that meant *skip this one* in the closure means *stop everything* in the loop. A chain that needs to skip is usually a `filter` rather than a `for_each` with a `return` in it.

## `continue` outside a loop

```rust
fn main() {
    let n = 3;
    if n > 2 {
        // continue;   // error[E0268]: `continue` outside of a loop
    }
}
```

```text title="Real rustc output for continue_outside_a_loop.rs"
error[E0268]: `continue` outside of a loop
 --> continue_outside_a_loop.rs:4:9
  |
4 |         continue;
  |         ^^^^^^^^ cannot `continue` outside of a loop

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0268`.
```

Same error code as `break` outside a loop, with one word fewer: `break`'s message says *outside of a loop or labeled block*, because a labelled block is a `break` target, and this one does not — a block has no next pass, so there is nothing for `continue` to go back to. [Loop labels](../loop_labels/README.md#a-labelled-block-is-breakable-without-a-loop) has the error for trying (`E0696`).

## The verified output

<!-- output:continue_expressions -->
*Verified output of [`continue_expressions.rs`](examples/continue_expressions.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. continue skips the rest of this pass; for still advances
   visited [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
   sum of the odd ones = 25
   Every n was visited: the range hands out the next number
   whether or not the body finished.

2. while: a continue above the increment never reaches it
   stopped by a 20-pass safety cap, not by the loop condition
   (pass, i): [(3, 2), (10, 2), (20, 2)]
   total = 11   <- 5 and 6 were never added
   increment moved above the continue: total = 22
   the same loop as a for:            total = 22

3. continue in loop jumps back to the top; let-else makes the test
   [("width", "80"), ("height", "24")]

4. continue has type !, so it fits in a match arm
   ["12", "x", "30", "", "7"] -> [12, 30, 7]

5. filter is `if not wanted { continue; }` as an adapter
   (0..10).filter(odd).sum()          = 25
   readings.filter(r >= 0).sum()      = 22
   raw.filter_map(parse.ok()).collect = [12, 30, 7]

6. In for_each, return is the continue; in a for body it leaves the function
   for_each with return: [1, 3, 5]
   for with return:      [1, 3]
```
<!-- /output -->

## If you are coming from another language

**Python.** Same statement, same meaning, and the same `while` trap: `while i < len(xs): … if bad: continue … i += 1` spins forever in Python too. What transfers is the fix — a `for` over the items instead of an index — and Python's `for x in xs` is the same shape as Rust's `for &x in &xs`. Python has no `continue` to an outer loop ([PEP 3136 ↗](https://peps.python.org/pep-3136/), which would have added one, was rejected); Rust has `continue 'outer`.

**C.** Here the trap has a history. In C, `continue` inside `for (i = 0; i < n; i++)` jumps to the `i++` — the increment is part of the loop's head, so a `continue` cannot skip it. Rust has no three-part `for`, so a C loop translated into a Rust `while` moves `i++` into the body, where a `continue` above it now skips it. Measured in C: the same `continue`-before-increment in a `while` stays stuck at the skipped index exactly as the Rust version does, while the `for` version sums correctly. The Rust answer is to not translate it into a `while` at all: `for i in 0..n` puts the increment back inside the iterator.

**JavaScript and Java.** `continue` and `continue label` both exist and mean what they mean here, with the same `while` trap and the same C-style `for` that protects against it. Both languages refuse a labelled `continue` that names a block rather than a loop — measured on Node 20 (`SyntaxError: Illegal continue statement: 'blk' does not denote an iteration statement`) and on `javac` 25 (`not a loop label: blk`) — which is Rust's `E0696`.

**ABAP.** `CONTINUE` is `continue`: allowed only inside a loop, it ends the current pass and starts the next. `CHECK cond` inside a loop is the guard form of it — SAP documents it as equivalent to `IF NOT cond. CONTINUE. ENDIF.` — so `CHECK ls_item-qty > 0.` at the top of a `LOOP AT` is the ABAP spelling of `if qty <= 0 { continue; }`. The counter trap splits the same way as in Rust. `DO … ENDDO` maintains `sy-index` for you, so a `CONTINUE` cannot skip it — that is Rust's `for`. A `WHILE` with your own `lv_i = lv_i + 1` at the bottom spins forever on the first `CONTINUE` above it — that is Rust's `while`. What does not transfer: outside a loop, `CHECK` leaves the whole processing block (SAP's guideline is to use it only inside loops), whereas Rust's `continue` there is `E0268`, and nothing in Rust is a `continue` that turns into an early return depending on where it stands.

## See also

- [Flow control](../flow_control/README.md) — the chapter's overview of every jump
- [`break`](../break_expressions/README.md) — leave the loop instead of the pass, and the value it can carry out
- [Loop labels](../loop_labels/README.md) — `continue 'outer`, and why a labelled block cannot be `continue`d
- [`while` loops](../while_loops/README.md) — the loop whose counter a `continue` can strand
- [Loops without an index](../loops_without_an_index/README.md) — the loop with no counter to strand
- [The never type `!`](../../15_First_Programs/the_never_type/README.md) — why `Err(_) => continue` and `else { continue }` type-check
- [Adapters by job](../../24_Iterators/adapters_by_job/README.md) — `filter` and `filter_map`, the chain spelling of a guard with `continue`
- [*Rust in Action*, claims checked](../flow_control_claims_checked/README.md) — the book's §2.4 run claim by claim

## Sources

- [The Rust Reference: `continue` expressions ↗](https://doc.rust-lang.org/reference/expressions/loop-expr.html#continue-expressions) — control returns to the loop head, which for a `while` is the condition and for a `for` is the call to the iterator
- Error index: [E0268 ↗](https://doc.rust-lang.org/error_codes/E0268.html) · [E0267 ↗](https://doc.rust-lang.org/error_codes/E0267.html)
- *Rust in Action* (Tim McNamara, Manning 2021), §2.4.2 — the book's section on `continue`
- SAP ABAP Keyword Documentation: [`CONTINUE` ↗](https://help.sap.com/doc/abapdocu_752_index_htm/7.52/en-us/abapcontinue.htm) · [`CHECK` in loops ↗](https://help.sap.com/doc/abapdocu_756_index_htm/7.56/en-US/abapcheck_loop.htm)

## Po polsku

`continue` porzuca resztę **bieżącego obiegu** i wraca do nagłówka pętli. Co to znaczy, zależy od pętli. W `for` nagłówkiem jest wywołanie `next` na iteratorze, więc pętla i tak przechodzi do następnego elementu — `for n in 0..10` z `continue` dla parzystych odwiedza wszystkie dziesięć liczb i sumuje nieparzyste do 25. W `while` nagłówkiem jest wyłącznie warunek, i tu siedzi pułapka: jeśli licznik zwiększamy na dole ciała pętli, a `continue` stoi nad tym `i += 1`, licznik przestaje się ruszać. Zmierzone: przy odczytach `[4, 7, -1, 5, 6]` indeks utyka na 2 już od trzeciego obiegu, suma zatrzymuje się na 11, a bez sztucznego limitu pętla nie skończyłaby się nigdy. Naprawa to zwiększenie licznika **przed** `continue` — albo, lepiej, `for`, który nie ma licznika do zapomnienia.

`continue` jest wyrażeniem typu `!`, dlatego pasuje do ramienia `match` (`Err(_) => continue`) i do bloku `else` w `let … else`. Jego odpowiednikiem w łańcuchu iteratorów jest `filter` (a `filter_map` dla „sparsuj albo pomiń”). W domknięciu `continue` się nie skompiluje (`E0267`), a w `for_each` jego rolę pełni `return` — i to jest druga pułapka: to samo `return` w ciele zwykłej pętli `for` kończy **całą funkcję**, nie jeden obieg. Poza pętlą `continue` to `E0268`.

W ABAP-ie `CONTINUE` działa tak samo i też tylko w pętli, a `CHECK warunek` w pętli jest jego skrótem (`IF NOT warunek. CONTINUE. ENDIF.`). Podział pułapki jest identyczny: `DO … ENDDO` sam prowadzi `sy-index`, więc `CONTINUE` go nie ominie — to rustowy `for`; `WHILE` z własnym licznikiem na dole zawiesi się tak samo jak rustowy `while`.

**Szukaj po polsku:** instrukcja `continue` · pominięcie obiegu pętli · pętla nieskończona w `while` · `rust continue while infinite loop` · `rust continue inside closure E0267` · `rust for_each return continue`
