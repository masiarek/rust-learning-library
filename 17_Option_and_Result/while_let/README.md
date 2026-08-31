# `while let`: loop while the shape holds

**Level:** 201 · working knowledge

**One line:** The pattern is re-tested before every pass, so `None` becomes the loop's exit condition — and nothing in the language checks that your body is moving toward it.

[`if let`](../if_let/README.md) runs a block once when a pattern matches. `while let` runs it *for as long as* the pattern keeps matching. That one-word change buys a genuinely nice loop and introduces a bug the single-shot version cannot have, which is why it gets its own page.

---

## The exit condition is the pattern

```rust
while let Some(top) = stack.pop() {
    println!("popped {top}");
}
```

`pop()` is a [partial function](../partial_functions/README.md): it has no answer for an empty stack, so it returns an `Option`. Which means *running out of items* and *ending the loop* are the same event, expressed once. There is no length to read, no index to advance, and no off-by-one available to get wrong — the three places that loop would have gone wrong in C.

## The bug `if let` cannot have

Nothing about `while let` guarantees the body makes progress. Swap the consuming call for a looking one and the condition is true forever:

```rust
while let Some(top) = stack.last() {   // last() LOOKS; pop() REMOVES
    println!("top is still {top}");    // ...and this never stops printing
}
```

An `if let` runs once whatever you write in it, so this failure mode does not exist there. Here the compiler will not tell you: a `while let` whose scrutinee does not consume is a `loop {}` with extra steps, and it is your job to notice. **The question to ask of every `while let` is "what does this call take away?"** If the answer is nothing, you wanted `if let`.

## Where the borrow ends decides whether you may consume

Sometimes you have to look before you consume, and then the loop is really about the borrow checker. These two are the same statements in opposite order, and only one compiles:

```rust
while let Some(top) = stack.last() {
    let top = *top;         // finish with the borrow first
    stack.pop();            // ...then take the mutable one
    println!("read {top}");
}
```

```text
error[E0502]: cannot borrow `stack` as mutable because it is also borrowed as immutable
3 |     while let Some(top) = stack.last() {
  |                           ----- immutable borrow occurs here
4 |         stack.pop();
  |         ^^^^^^^^^^^ mutable borrow occurs here
5 |         println!("{top}");
  |                    --- immutable borrow later used here
```

The rule behind that pair — a borrow lives until its **last use**, not to the end of its block — belongs to [borrowing](../../18_Ownership/borrowing/README.md) rather than to loops, and is explained there. What is specific to `while let` is the collision it sets up: the head takes a shared borrow to *test* the pattern, and the body needs an exclusive one to make progress. Finish with the binding first (copy the value out, as above) and the two never overlap.

## `for` is this loop, already written for you

```rust
for n in names.iter() { … }

let mut it = names.iter();          // roughly what the above desugars to
while let Some(n) = it.next() { … }
```

So hand-writing `while let Some(x) = it.next()` is usually a downgrade — more code, same behaviour, and Clippy has a lint for it. The exception is the reason to know this at all: **`for` consumes the iterator and you never see it again**, so any loop that needs the iterator *between* passes has to be written the long way.

```rust
let mut it = marks.iter().peekable();
while let Some(&first) = it.next() {
    let mut run = 1;
    while it.peek() == Some(&&first) { it.next(); run += 1; }   // a variable bite per pass
    print!(" {first}×{run}");
}
```

Run-length grouping consumes a different number of items each pass, so there is nothing for a `for` header to bind. Same story with `by_ref().take(2)` — take a bite, keep the rest. When you see a hand-written `while let` over an iterator, that is what to look for; if it is not there, it is noise.

## Not just `Option`

Any pattern that eventually stops matching works, and the common one is a channel:

```rust
while let Ok(score) = rx.recv() { … }   // ends when the last sender is dropped
```

Convenient, and it carries `if let`'s trade into a loop: **`Err` ends it, so an orderly disconnect and a real error are now the same event.** That is usually what you want from `recv()`, and it is a bug the day it isn't. If those two deserve different handling, write the `match`.

## There is no `while let … else`

```text
error: `while...else` loops are not supported
```

Worth understanding rather than memorising: for `if let`, the pattern failing is the *other case*, so an `else` has something to mean. For `while let`, the pattern failing is how the loop **normally ends** — it happens on every single run — so an `else` would fire every time and say nothing. When you want an early exit on a *different* failure, put a [`let … else`](../if_let/README.md) inside the body, or check after the loop.

## If you are coming from another language

- **Python** — `while (line := f.readline()):` is the same instinct, and the same trap: it ends on a value the loop happens to treat as false, so a legitimately falsy item ends it early. Rust's version tests the *shape*, so a `Some(0)` and an empty string keep the loop running; only `None` stops it.
- **ABAP** — the `DO … READ TABLE … IF sy-subrc <> 0. EXIT. ENDIF. ENDDO.` shape, with the read, the check, and the exit fused into the loop header. What changes: you cannot forget the `EXIT`, and the work area does not exist on the pass where the read failed. What does *not* change: you still have to make sure the body advances, exactly as you do with a manual index.

---

## Practice

**A loop with no counter.** Drain a stack with `while let Some(x) = stack.pop()`, then write the same loop over an iterator's `next()` and compare it with the `for` you would actually write.

Then write the body that peeks instead of advancing — `queue.last()` where you meant `pop()`. Bound it with a counter so it terminates, and note what the compiler said about it: nothing. `while let` re-tests the pattern and is perfectly happy to re-test it forever. (A tell: that queue never needed to be `mut`.)

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:while_let_kata -->
*[`while_let_kata.rs`](examples/while_let_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the pattern is the exit condition — and nothing checks it.
//!
//!   rustc --edition 2024 while_let_kata.rs -o /tmp/wlk && /tmp/wlk

fn main() {
    // Drain: each pass shortens the stack, so the pattern eventually fails.
    let mut stack = vec![5u8, 3, 0, 4];
    print!("Draining with `while let Some(x) = stack.pop()`:\n ");
    while let Some(x) = stack.pop() {
        print!(" {x}");
    }
    println!("\n  stack is now {stack:?} — the None ended the loop");

    // The same loop written over an iterator, and then the way you would
    // actually write it.
    let scores = [5u8, 3, 0];
    let mut it = scores.iter();
    print!("\n`while let Some(s) = it.next()`:\n ");
    while let Some(s) = it.next() {
        print!(" {s}");
    }
    print!("\n`for s in scores`               :\n ");
    for s in scores {
        print!(" {s}");
    }
    println!("\n  Identical. `for` is this loop with the advance built in, which");
    println!("  is exactly the line the next demo forgets.");

    // The bug: a body that does not move toward the failing pattern. Bounded
    // here with a counter so the example terminates and can be verified — in
    // real code there is no counter, and the program simply hangs.
    // Not `mut` — and that is the tell: this loop never changes the queue.
    let queue = vec!["late ballot"];
    let mut passes = 0;
    println!("\nA body that never shortens the queue:");
    while let Some(item) = queue.last() {
        passes += 1;
        println!("  pass {passes}: still holding {item:?}");
        if passes == 3 {
            println!("  (stopped by a counter that only exists in this demo)");
            break;
        }
        // The missing line is `queue.pop();`.
    }
    println!("  `last()` peeks; `pop()` advances. Nothing in the language knows");
    println!("  which one your loop needed — `while let` re-tests the pattern and");
    println!("  is perfectly happy to re-test it forever.");
}
```
<!-- /source -->

<!-- output:while_let_kata -->
*Verified output of [`while_let_kata.rs`](examples/while_let_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Draining with `while let Some(x) = stack.pop()`:
  4 0 3 5
  stack is now [] — the None ended the loop

`while let Some(s) = it.next()`:
  5 3 0
`for s in scores`               :
  5 3 0
  Identical. `for` is this loop with the advance built in, which
  is exactly the line the next demo forgets.

A body that never shortens the queue:
  pass 1: still holding "late ballot"
  pass 2: still holding "late ballot"
  pass 3: still holding "late ballot"
  (stopped by a counter that only exists in this demo)
  `last()` peeks; `pop()` advances. Nothing in the language knows
  which one your loop needed — `while let` re-tests the pattern and
  is perfectly happy to re-test it forever.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:while_let -->
*Verified output of [`while_let.rs`](examples/while_let.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: The exit condition IS the pattern
  popped 30, 2 left
  popped 20, 1 left
  popped 10, 0 left
      `pop()` is a partial function returning Option, so running out
      of items and ending the loop are the same event. Nothing here
      counts, indexes, or checks a length.

──── Step 2: The bug `if let` cannot have: a body that makes no progress
  pass 1: top is still 30
  pass 2: top is still 30
  pass 3: top is still 30
  pass 4: top is still 30
  ...stopped by hand at 4 — nothing in the loop was ever going to stop it
      An `if let` runs once whatever you write in it. A `while let`
      re-tests the pattern, and the compiler will not check that the
      body changed anything. The scrutinee has to CONSUME.

──── Step 3: Where the borrow ends decides whether you may consume
  read 30, then popped it
  read 20, then popped it
  read 10, then popped it
      Move the `println!` of `top` below the `pop()` and this stops
      compiling with E0502 — the immutable borrow from `last()` is
      still live at that point. Same two statements, opposite order,
      and the difference is the last USE of the binding, not the pop.

──── Step 4: `for` is this loop, already written for you
  for            -> Ada Ben Cara
  while let      -> Ada Ben Cara
      Identical, because `for` desugars to roughly the second one.
      Hand-writing it is usually a downgrade — unless you need the
      iterator itself between passes, which `for` has moved away.

──── Step 5: When you DO need the iterator: peeking and taking
  runs           -> 5×3 3×2 0×1
  first two [5, 5], then the rest -> 5 3 3 0
      Both halves need the iterator to survive between passes, which
      is the one thing `for` takes away. That — not style — is when
      the hand-written loop is the right call.

──── Step 6: Not just Option: any pattern that eventually stops matching
  received 5, running total 5
  received 3, running total 8
  received 0, running total 8
  channel closed, final total 8
      Convenient, and it carries `if let`'s trade into a loop: `Err`
      ends it, and a disconnect and a real error are now the same
      event. If those deserve different handling, write the `match`.

──── Step 7: There is no `while let … else`
  first_two(&[5, 3, 0]) -> Some((5, 3))
  first_two(&[5])       -> None
      `while … else` is a hard error: "`while...else` loops are not
      supported". A loop's pattern failing is its NORMAL ending, so
      there is nothing for an else to mean. Put the escape inside the
      body with `let … else`, or check after the loop.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 17_Option_and_Result/while_let/examples/while_let.rs -o /tmp/wl && /tmp/wl
```

## See also

- [`if let`](../if_let/README.md) — the single-shot version, and the rest of the family
- [Partial functions](../partial_functions/README.md) — why `pop()`, `next()`, and `recv()` hand back something that can say "no more"
- [Borrowing](../../18_Ownership/borrowing/README.md) — why copying `*top` out ends the borrow, and the last-use rule that decides which order compiles
- [`Option` is a one-item collection](../option_as_collection/README.md) — the iterator toolbox this loop is competing with
- [The Rust Reference on `while let` ↗](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-pattern-loops)

## Po polsku

W `while let` warunkiem zakończenia pętli nie jest wartość logiczna, tylko **kształt** — dopasowanie wzorców (*pattern matching*) wykonywane od nowa przed każdym przebiegiem. `stack.pop()` nie ma odpowiedzi dla pustego stosu, więc zwraca `Option`, a przez to „skończyły się elementy” i „koniec pętli” stają się jednym zdarzeniem zapisanym raz. Nie ma długości do odczytania, nie ma indeksu do zwiększania i nie ma gdzie popełnić błędu o jeden. Warto od razu zauważyć różnicę wobec pythonowego `while (line := f.readline()):`, które kończy się na wartości uznanej za fałszywą: tutaj `Some(0)` czy `Some("")` pętli nie kończą, bo sprawdzany jest wariant wyliczenia, a nie prawdziwość.

Za to `while let` ma błąd, którego `if let` mieć nie może: **nic w języku nie sprawdza, czy ciało pętli w ogóle zbliża się do `None`**. `stack.last()` podgląda, `stack.pop()` zabiera — te dwie linijki wyglądają niemal identycznie, a druga wersja drukuje w nieskończoność i kompilator nie powie ani słowa. Pytanie, które trzeba zadać każdemu `while let`, brzmi więc: **co to wywołanie zabiera?** Jeśli odpowiedź brzmi „nic”, chodziło o `if let`. Dobrym sygnałem ostrzegawczym jest sama deklaracja: jeśli kolekcja nie musiała być `mut`, to znaczy, że pętla jej nie zmienia.

Bywa też, że trzeba najpierw zajrzeć, a dopiero potem zabrać, i wtedy pętla jest tak naprawdę zadaniem dla borrow checkera. Nagłówek bierze referencję współdzieloną, żeby sprawdzić wzorzec, a ciało potrzebuje mutowalnej, żeby posunąć pętlę do przodu — te dwa pożyczenia nie mogą się nakładać. Rozwiązanie to jedna linijka: skopiuj wartość (`let top = *top;`) i dopiero wtedy wywołaj `pop()`. W odwrotnej kolejności dostaniesz `error[E0502]` z linią `immutable borrow later used here`, bo pożyczenie żyje do **ostatniego użycia** wiązania, a nie do klamry. Na koniec dwie rzeczy do zapamiętania: `for` to dokładnie ta pętla, tylko już napisana (`while let Some(x) = it.next()` ma nawet lint w Clippy), a ręczne pisanie ma sens tylko wtedy, gdy iterator jest potrzebny **pomiędzy** przebiegami — przy `peekable()` albo `by_ref().take(2)`, bo `for` zabiera iterator na własność i już go nie zobaczysz.

**Szukaj po polsku:** pętla while let · dopasowanie wzorców · pętla nieskończona · `rust while let vs for` · `rust E0502 immutable borrow later used here` · `clippy while_let_on_iterator`
