# The drop flag

**Level:** 301 · deep dive

**One line:** When the compiler cannot tell from the source whether a location still holds a value, it puts a hidden boolean in your stack frame and checks it at the closing brace — which is why a moved-out variable is *sometimes* a compile-time fiction and *sometimes* a byte of run-time state.

```rust
struct Named(&'static str);

impl Drop for Named {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn consume(n: Named) {
    println!("consumed {}", n.0);
}

fn maybe_move(flag: bool) {
    let n = Named("value");
    if flag {
        consume(n);
    }
    println!("end of maybe_move({flag})");
}
```

Call it twice. `maybe_move(true)` prints `consumed value`, `drop value`, `end of maybe_move(true)` — the drop happened inside `consume`. `maybe_move(false)` prints `end of maybe_move(false)`, `drop value` — the drop happened at the brace. One function, one `let`, one brace, and two different schedules chosen while the program runs.

## What the compiler cannot know

"A moved-out value is not dropped" is usually a claim it settles by reading the code. `let a = t;` empties `t` on every path, so the brace after it can simply not emit a drop for `t` — nothing survives to run time.

The `if` above breaks that. The brace is reached with `n` moved out on one path and intact on the other, and the same machine code has to serve both. So `rustc` allocates one more slot in the frame — a boolean, set when the value is initialised and cleared when it is moved out — and the brace becomes *"drop `n` if the flag says it is still there"*. The Rustonomicon calls these [drop flags ↗](https://doc.rust-lang.org/nomicon/drop-flags.html) and states the placement: they live on the stack, not inside the value.

You can see that second part without a debugger:

```rust
println!("{}", size_of::<Named>() == size_of::<&'static str>());  // true
```

A `Drop` impl adds nothing to a type's size. Before Rust 1.0 it did — the flag was a hidden field, so every droppable type paid for it in every location it was ever stored, including inside a `Vec`. [RFC 320 ↗](https://github.com/rust-lang/rfcs/blob/master/text/0320-nonzeroing-dynamic-drop.md) moved them to the frame of the one function that might branch, which is the arrangement you are compiling against today.

## Four things that follow

**Exactly one drop, on every path.** Never two, never none. That is the guarantee; the flag is just how it is kept when the source cannot answer.

**Fields are tracked separately.** A struct with no `Drop` impl of its own can be partly moved — take one field, and the brace drops the other and knows to leave the first alone.

**A type with its own `Drop` cannot be split.** Add a `Drop` impl to that same struct and moving one field out stops compiling:

```text title="Abridged — real rustc output, without the trailing Clone suggestion"
error[E0509]: cannot move out of type `Pair`, which implements the `Drop` trait
 --> e0509.rs:9:17
  |
9 |     let taken = p.left;
  |                 ^^^^^^
  |                 |
  |                 cannot move out of here
  |                 move occurs because `p.left` has type `Named`, which does not implement the `Copy` trait
```

`drop(&mut self)` takes the whole value, so it would be handed a `Pair` with a hole in it. There is no flag arrangement that fixes that, so the move is refused instead. This is the payoff of understanding the flag: [E0509](../../ERRORS.md#every-code-the-library-teaches) stops reading as an arbitrary restriction.

**It costs a byte and a branch, and optimization usually deletes it.** With `-O` the compiler routinely specialises the two paths and drops the flag along with them. Nothing about the *observable* behaviour depends on that, which is why the program below can have an answer key at all.

## The version where you can ask

The flag answers a question you cannot: there is no `is_still_there(n)`. When the emptiness is a thing your program needs to reason about, put it in the type — that is what [`Option::take` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.take) is for:

```rust
let mut slot = Some(Named("in the Option"));
if let Some(v) = slot.take() {     // slot is now None, and you can see it
    consume(v);
}
println!("{}", slot.is_some());    // false
```

Identical schedule, identical single drop. The difference is that `is_some()` exists, the state is `match`able, and it can be returned from the function — none of which a stack slot you cannot name will ever do for you. This is also the standard way out of "cannot move out of `self`" in a method: `mem::take` or `Option::take` on the field, rather than a `clone` that pretends the problem was the copy. ([Assignment drops the old value](../assignment_is_a_drop/README.md) has the rest of that family.)

## If you are coming from another language

- **C++.** This is the difference C++ solves by keeping the moved-from object alive. A `std::unique_ptr` that has been moved from is still destroyed at the closing brace; its destructor runs and finds a null pointer, which is why the type has to *have* a null state and why "valid but unspecified" is in the standard. Rust runs no destructor at all on a moved-out location, and the flag is the bookkeeping that makes that decidable at run time — so the cost lands in one stack byte in one function instead of in every instance of every movable type. If you have written a move constructor that nulls out the source, you have hand-written a drop flag inside the value, which is exactly the pre-1.0 Rust design that RFC 320 replaced.
- **Python / Java / C#.** No counterpart, and the absence is the point: an object's death is the collector's business, so no code you write ever has to decide whether a particular local still owns something. What transfers is the *shape* of the bug it prevents — the C# `IDisposable` field you conditionally hand to a caller, where both of you might call `Dispose()`, or neither. Rust's answer to that ambiguity is a flag rather than a convention.
- **ABAP.** Nothing here at all — references are copied, the garbage collector decides, and `CLEANUP` is the only deterministic hook. The nearest familiar shape is the `IF lo_x IS BOUND` guard before releasing a handle you may or may not still own: that check is what the drop flag does, generated for you, in the one place it is needed.

## The verified output

<!-- output:the_drop_flag -->
*Verified output of [`the_drop_flag.rs`](examples/the_drop_flag.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The same source line, two different answers
     consumed value (and dropped it here)
     drop value
     end of maybe_move(flag = true)
     ---
     end of maybe_move(flag = false)
     drop value
   Identical code. In the first call the value was dropped inside
   `consume`; in the second, at the closing brace. Something read at
   run time decided which — and it is not part of the value.

2. It is not in the type
     size_of::<Named>() == size_of::<&'static str>() : true
     a Drop impl adds nothing to the type's size, so the flag is
     somewhere else: a slot in the stack frame of the function that
     might or might not have moved the value out.

3. Dropped exactly once, whichever way the branch goes
     flag = true
     consumed counted (and dropped it here)
     drop counted
     flag = false
     drop counted
   Once per iteration, never twice, never zero times. That is the
   guarantee; the flag is how it is kept when the compiler cannot
   settle the question by reading the source.

4. Fields are tracked one at a time
     moved out `p.left` -> p.left, leaving p.right
     drop p.left
     end of block, and only `p.right` is left to drop:
     drop p.right
   `p` is partly moved: the compiler tracks the two fields
   separately, so the brace drops one of them and knows not to
   touch the other.

5. Which is why a type with its own Drop cannot be split
     Give `Pair` a `Drop` impl and `let taken = p.left;` stops
     compiling: error[E0509], cannot move out of type `Pair`,
     which implements the `Drop` trait. There is no half-value to
     hand `drop(&mut self)`, so the move is refused outright.

6. The version where the emptiness is a value you can see
     round 0: slot is_some = true
     consumed in the Option (and dropped it here)
     drop in the Option
     round 1: slot is_some = false
     round 2: slot is_some = false
     end of block, and there is nothing left to drop:
   `Option::take` puts the same decision in the type, where you can
   read it, test it, and return it. The flag does the identical job
   invisibly — which is fine, until you want to ASK.
```
<!-- /output -->

## Practice

**Count the drops before you run it.** Write a type that logs its own death, then build four functions: one that returns early while still owning the value, one that returns early after passing it to another function, one that moves it out on the second turn of a loop and then breaks, and a fourth that is the third rewritten with `Option::take`.

For each, say how many drops happen and *where the line for each one sits in the source*. Two of the four need a run-time decision at the closing brace; name them, and say what the fourth one bought by making the decision visible.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:the_drop_flag_kata -->
*[`the_drop_flag_kata.rs`](examples/the_drop_flag_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: count the drops on four paths, then remove the flag.
//!
//!   rustc --edition 2024 the_drop_flag_kata.rs -o /tmp/tdfk && /tmp/tdfk

struct Ballot(&'static str);

impl Drop for Ballot {
    fn drop(&mut self) {
        println!("       drop {}", self.0);
    }
}

fn file_it(b: Ballot) {
    println!("       filed {}", b.0);
}

/// Returns early while it still owns the value.
fn early_return_owning(short: bool) {
    let b = Ballot("A");
    if short {
        println!("       returning early, still owning it:");
        return;
    }
    file_it(b);
}

/// Returns early after handing the value away.
fn early_return_moved(short: bool) {
    let b = Ballot("B");
    if short {
        file_it(b);
        println!("       returning early, having moved it out:");
        return;
    }
    file_it(b);
}

/// Moves the value out on one iteration, then leaves the loop.
fn moved_in_a_loop() {
    let b = Ballot("C");
    for round in 0..3 {
        println!("       round {round}");
        if round == 1 {
            file_it(b);
            break;
        }
    }
    println!("       after the loop:");
}

/// The same shape with the emptiness written down.
fn moved_in_a_loop_visibly() {
    let mut slot = Some(Ballot("D"));
    for round in 0..3 {
        println!("       round {round}, holding = {}", slot.is_some());
        if round == 1 {
            if let Some(b) = slot.take() {
                file_it(b);
            }
        }
    }
    println!("       after the loop, holding = {}:", slot.is_some());
}

fn main() {
    println!("A. early return while still owning it        -> dropped AT the return");
    early_return_owning(true);

    println!("\nB. early return after moving it out          -> nothing at the return");
    early_return_moved(true);

    println!("\nC. moved out on round 1, then `break`        -> nothing at the brace");
    moved_in_a_loop();

    println!("\nD. the same, with the emptiness in the type  -> same schedule, askable");
    moved_in_a_loop_visibly();

    println!("\nFour paths, four values, four drops — one each, never two, never");
    println!("none. A and C are the ones a flag is for: the SAME closing brace");
    println!("has to drop in one execution and not in another, so the answer");
    println!("cannot be baked into the code at that brace. D is C with the");
    println!("question moved into the type, where `is_some()` can answer it.");
}
```
<!-- /source -->

<!-- output:the_drop_flag_kata -->
*Verified output of [`the_drop_flag_kata.rs`](examples/the_drop_flag_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
A. early return while still owning it        -> dropped AT the return
       returning early, still owning it:
       drop A

B. early return after moving it out          -> nothing at the return
       filed B
       drop B
       returning early, having moved it out:

C. moved out on round 1, then `break`        -> nothing at the brace
       round 0
       round 1
       filed C
       drop C
       after the loop:

D. the same, with the emptiness in the type  -> same schedule, askable
       round 0, holding = true
       round 1, holding = true
       filed D
       drop D
       round 2, holding = false
       after the loop, holding = false:

Four paths, four values, four drops — one each, never two, never
none. A and C are the ones a flag is for: the SAME closing brace
has to drop in one execution and not in another, so the answer
cannot be baked into the code at that brace. D is C with the
question moved into the type, where `is_some()` can answer it.
```
<!-- /output -->

</details>

## See also

- [Ownership and moves](../ownership_and_moves/README.md) — what a move is, and the rules the flag enforces
- [Assignment drops the old value](../assignment_is_a_drop/README.md) — the other place a location's fullness decides whether anything dies
- [Scope is about names, not values](../scope_is_about_names/README.md) — the schedule this page is the exception to
- [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) — the trait itself, and the three ways a value escapes being dropped at all
- [`Option` and `Result`](../../17_Option_and_Result/README.md) — where the emptiness goes when you want to read it

## Po polsku

Gdy kompilator potrafi odczytać ze źródła, czy dana zmienna nadal trzyma wartość, nie zostawia po tym śladu w programie: przeniesiona wartość po prostu nie jest wypuszczana na końcu bloku. Problem zaczyna się przy `if` — ten sam nawias klamrowy bywa osiągany raz z wartością na miejscu, a raz bez niej, i ten sam kod maszynowy musi obsłużyć oba przypadki.

Rozwiązaniem jest **flaga wypuszczenia** (*drop flag*): jeden ukryty bajt w ramce stosu, ustawiany przy inicjalizacji i zerowany przy przeniesieniu, a na końcu bloku sprawdzany. Wartość zostaje wypuszczona dokładnie raz na każdej ścieżce — nigdy dwa razy, nigdy zero.

Dwie rzeczy warto zapamiętać. Po pierwsze, **flaga nie jest częścią typu** — `size_of` typu z `Drop` jest taki sam jak bez niego. Przed wersją 1.0 było inaczej (flaga siedziała w wartości, więc płacił za nią każdy egzemplarz); RFC 320 przeniosło ją na stos. Po drugie, kompilator śledzi **pola osobno**, dlatego strukturę bez własnego `Drop` można przenieść „po kawałku” — ale strukturę z własnym `Drop` już nie, i to jest cała treść błędu `E0509`: metodzie `drop(&mut self)` nie da się podać wartości z dziurą.

Gdy pustka jest czymś, o co program musi *zapytać*, przenieś ją do typu: `Option::take()` daje ten sam harmonogram wypuszczania, ale stan da się sprawdzić przez `is_some()`, dopasować w `match` i zwrócić z funkcji. Flaga na stosie nie zrobi żadnej z tych rzeczy.

**Szukaj po polsku:** flaga wypuszczenia · `rust drop flag` · `E0509` przenoszenie pola · częściowe przeniesienie struktury · `Option::take`
