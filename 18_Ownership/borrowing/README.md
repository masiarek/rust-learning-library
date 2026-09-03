# Borrowing: `&T`, `&mut T`, and where a borrow ends

**Level:** 101 → 201 · for newcomers

**One line:** A reference lets you use a value without owning it, under one rule — **many readers, or one writer, never both at once** — and the part that decides whether your code compiles is where the compiler thinks each borrow *ended*.

[Ownership](../ownership_and_moves/README.md) answers *who frees this?*. That answer alone would make Rust unusable: every function that wanted to look at your `String` would have to take it and hand it back. Borrowing is the other half.

---

## The alternative, so you can see what borrowing buys

```rust
fn take_and_return(s: String) -> (usize, String) { (s.len(), s) }   // give it back
fn just_look(s: &str) -> usize { s.len() }                          // just look
```

Both answer the same question. The first makes every caller thread the value back out through a tuple and rebind it; the second says *"I need to read this, I will not keep it."* That is all a reference is — **access without responsibility**. The owner still owes the free, and the borrow is guaranteed to be over before the owner is.

## The one rule

At any moment a value may have:

- **any number of shared references** `&T` — everyone may read, nobody may write; or
- **exactly one exclusive reference** `&mut T` — that one may write, and nobody else may even read.

That is the whole borrow checker. Both halves show up as errors you will meet on day one:

```text
error[E0499]: cannot borrow `v` as mutable more than once at a time
3 |     let a = &mut v;
  |             ------ first mutable borrow occurs here
4 |     let b = &mut v;
  |             ^^^^^^ second mutable borrow occurs here
```

```text
error[E0502]: cannot borrow `v` as immutable because it is also borrowed as mutable
3 |     let m = &mut v;
  |             ------ mutable borrow occurs here
4 |     println!("{v:?}");
  |                ^ immutable borrow occurs here
```

Note what the second one says: while an exclusive borrow is live, the **owner** cannot read its own value. Exclusive means exclusive against everybody.

## Where a borrow ends — the part that decides everything

The rule above is easy. What people actually trip over is *when* a borrow stops counting, because that is what makes ordinary code legal or illegal.

**A borrow lives until its last use, not to the end of its block.** This is what "non-lexical lifetimes" names, and it says exactly what it sounds like: the borrow's extent is decided by where you last *mention* the binding, not by any brace.

```rust
let first = &scores[0];        // shared borrow starts
println!("{first}");           // ...and ends HERE, at its last use
scores.push(9);                // so this exclusive borrow is fine
```

Swap those last two lines and the same program is `E0502`, with the error naming the rule out loud:

```text
error[E0502]: cannot borrow `scores` as mutable because it is also borrowed as immutable
  |     let first = &scores[0];
  |                  ------ immutable borrow occurs here
  |     scores.push(9);
  |     ^^^^^^^^^^^^^^ mutable borrow occurs here
  |     println!("{first}");
  |                ----- immutable borrow later used here
```

*"immutable borrow later used here"* is the sentence to internalise. When a borrow error looks wrong to you, that line tells you which later mention is holding the borrow open — and the fix is usually to move that use earlier, or to copy what you needed out of it (`let first = *first;`) so the binding is no longer a reference at all.

Before this rule existed — pre-2018, when borrows really did run to the closing brace — you had to introduce an inner `{ }` block to end one early. You will still see those blocks in older code and in older explanations. They are no longer needed for this.

## The bug the rule exists to prevent

It is not pedantry, and this is the example that shows why:

```rust
for x in &scores {
    if *x == 0 { scores.push(1); }     // error[E0502]
}
```

`push` may **reallocate** the vector — move its buffer to a new address and free the old one. The loop is holding a pointer into the old buffer. Every language has to deal with this: C reads the freed memory and behaves unpredictably, Python raises `RuntimeError: dictionary changed size during iteration` *if the timing exposes it*, Java throws `ConcurrentModificationException` at runtime. Rust declines to build the program. The fix is to decide first and mutate after — build the additions, then `extend`; or use `retain`, which is written to take the exclusive borrow for the whole job.

## `&` means shared, not immutable

The common gloss is "immutable reference" and "mutable reference". It is a useful first approximation and it will mislead you the day you meet `Cell`, `RefCell`, `Mutex`, or an atomic — all of which mutate through a `&`:

```rust
fn bump(counter: &Cell<i32>) { counter.set(counter.get() + 1); }   // through a shared ref
```

That is not a loophole. The rule was never about writing; it is about **aliasing**. `&T` means *others may be holding this too*, and a type designed for that case may take responsibility for making the mutation safe — at runtime, with a flag or a lock. The accurate pair of words is **shared** and **exclusive**, which is also why `&mut` excludes readers rather than merely other writers.

## The `&` you never type

```rust
scores.len()          // = Vec::len(&scores)
owned.push('!')       // = String::push(&mut owned, '!')
```

The dot operator inserts whichever borrow the method asked for. That is convenient, and it is why a borrow error can arrive pointing at a line with no `&` anywhere in it — the reference is real, you just did not write it.

## Dangling is not a thing you can do

```text
error[E0106]: missing lifetime specifier
1 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  = help: this function's return type contains a borrowed value,
          but there is no value for it to be borrowed from
```

Returning a reference to a local is the classic use-after-free, and it is rejected as a *signature* problem: a returned reference has to borrow **from something the caller already has**, which in practice means from an argument. That is where lifetimes come in, and why you almost never write one — when the answer is unambiguous the compiler fills it in.

## If you are coming from another language

- **Python** — `b = a` gives you a second name for the same object, and mutating through either is visible from both. That is a *shared mutable* alias, exactly the combination Rust's rule forbids, and the reason `dict changed size during iteration` exists as a runtime error. Rust moves that class of bug to compile time; the price is that "just pass the list around" now requires you to say whether the callee reads it or writes it.
- **ABAP** — a field-symbol is a borrow: `LOOP AT lt ASSIGNING FIELD-SYMBOL(<ls>)` hands you write access to a row you do not own, and the dump you get from touching `<ls>` after the table was refreshed *is* a dangling reference. Rust checks the same hazard before the program runs, and gives it a name — the loop holds the borrow, so the refresh is what gets rejected.

---

## Practice

**Many readers, or one writer.** Write `total(&[u8])` and `cap_at(&mut Vec<u8>, u8)`, then call them on the same data — two shared borrows first, the mutable one after.

Now move the `println!` that reads through a shared borrow to *below* a `push`, and read `E0502`. Nothing about the code moved except the last *use* of the borrow, and that is what defines where it ends. Take `&[u8]` rather than `&Vec<u8>` while you are there, and notice which callers that buys you.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:borrowing_kata -->
*[`borrowing_kata.rs`](examples/borrowing_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: many readers, or one writer — and where the borrow ended.
//!
//!   rustc --edition 2024 borrowing_kata.rs -o /tmp/bwk && /tmp/bwk

/// Reads. Takes `&[u8]`, not `&Vec<u8>` — every caller with something
/// slice-shaped can call it, and it promises to change nothing.
fn total(scores: &[u8]) -> u32 {
    scores.iter().map(|s| *s as u32).sum()
}

/// Writes. One of these may exist at a time, and no reader alongside it.
fn cap_at(scores: &mut Vec<u8>, cap: u8) {
    for s in scores.iter_mut() {
        if *s > cap {
            *s = cap;
        }
    }
}

fn main() {
    let mut scores = vec![5u8, 9, 3, 7];

    println!("Many readers at once — fine:");
    let a = &scores;
    let b = &scores;
    println!("  a.len() = {}, b.len() = {}, total = {}", a.len(), b.len(), total(b));

    // Both shared borrows are last used above, so they are over by this line.
    // That is what lets the mutable borrow below exist at all.
    println!("\nOne writer, once the readers are finished:");
    cap_at(&mut scores, 5);
    println!("  capped -> {scores:?}");

    println!("\nWhere the borrow ends is the whole game:");
    let first = &scores[0];
    println!("  read through the borrow -> {first}");
    // `first` is not used again, so its borrow has ended here...
    scores.push(4);
    println!("  ...so pushing is allowed now -> {scores:?}");
    println!("      Move that println! below the push and it stops compiling:");
    println!("      E0502, cannot borrow `scores` as mutable because it is also");
    println!("      borrowed as immutable. Nothing about the code moved — only");
    println!("      the last USE of the borrow, which is what defines its end.");

    println!("\nThe bug the rule exists to prevent:");
    let mut v = vec![1u8, 2, 3];
    let len = v.len(); // read it OUT, do not hold a borrow across the push
    v.push(4);
    println!("  len read before the push -> {len}, now {} ", v.len());
    println!("      Holding `&v[0]` across that push would be a dangling pointer");
    println!("      in a language that allowed it: push can reallocate, and the");
    println!("      old buffer is freed. Rust rejects it at compile time instead.");

    println!("\n`&` means shared, not immutable — the interior-mutability escape:");
    use std::cell::Cell;
    let counter = Cell::new(0u32);
    let bump = |c: &Cell<u32>| c.set(c.get() + 1);
    bump(&counter);
    bump(&counter);
    println!("  counter behind a & -> {}", counter.get());
}
```
<!-- /source -->

<!-- output:borrowing_kata -->
*Verified output of [`borrowing_kata.rs`](examples/borrowing_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Many readers at once — fine:
  a.len() = 4, b.len() = 4, total = 24

One writer, once the readers are finished:
  capped -> [5, 5, 3, 5]

Where the borrow ends is the whole game:
  read through the borrow -> 5
  ...so pushing is allowed now -> [5, 5, 3, 5, 4]
      Move that println! below the push and it stops compiling:
      E0502, cannot borrow `scores` as mutable because it is also
      borrowed as immutable. Nothing about the code moved — only
      the last USE of the borrow, which is what defines its end.

The bug the rule exists to prevent:
  len read before the push -> 3, now 4 
      Holding `&v[0]` across that push would be a dangling pointer
      in a language that allowed it: push can reallocate, and the
      old buffer is freed. Rust rejects it at compile time instead.

`&` means shared, not immutable — the interior-mutability escape:
  counter behind a & -> 2
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:borrowing -->
*Verified output of [`borrowing.rs`](examples/borrowing.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: The alternative to borrowing is giving it back
  moved in and out -> len 5, and we have "hello" again
  borrowed         -> len 5, and we never lost it
      Same answer. The first signature makes every caller thread the
      value back through a tuple; the second one just asks to look.

──── Step 2: Many readers at once
  a.len()=3 b.first()=Some(5) c=5
  owner still readable: [5, 3, 0]
      Any number of `&T` may coexist, and the owner can still read.
      Nothing can change underneath them, so nothing can be surprised.

──── Step 3: One writer, and only one
  after add_one    -> [6, 4, 1]
      While that `&mut` was out, `scores` was unusable even for
      READING — an exclusive borrow is exclusive against everyone,
      including its owner. A second `&mut v` alongside it is E0499.

──── Step 4: A borrow ends at its LAST USE, not at the end of the block
  read through the shared borrow: 5
  pushed, now [5, 3, 0, 9]
      Move that `println!` of `first` below the `push` and this stops
      compiling with E0502. Same two statements, opposite order: what
      extends a borrow is the last USE of the binding, not the call
      that created it and not the closing brace.

──── Step 5: The bug the rule exists to prevent
  grown safely     -> [5, 3, 0, 1]
  retain           -> [5, 3, 1]
      Pushing to a Vec can REALLOCATE it, which would leave the loop's
      pointer aimed at freed memory. Python raises at runtime if you
      are lucky; C just reads the old buffer. Here it does not build.

──── Step 6: `&` is shared, not immutable
  mutated through &Cell<i32> -> 2
      The rule is about ALIASING, not about writing: `&T` means
      'others may hold this too', and a type built for it (Cell,
      RefCell, Mutex, atomics) may still change inside. Calling `&`
      'immutable' is the shorthand that makes those look like cheats.

──── Step 7: Method calls borrow for you
  scores.len()      -> 3
  Vec::len(&scores) -> 3
  owned.push('!')   -> "ada!"
      The `&` and `&mut` in everyday code are mostly invisible: the
      dot operator inserts whichever the method asked for. That is
      why a borrow error can arrive from a line with no & in it.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 18_Ownership/borrowing/examples/borrowing.rs -o /tmp/br && /tmp/br
```

## See also

- [Ownership and moves](../ownership_and_moves/README.md) — the half this one continues: who owes the free, and what a move actually transfers
- [A shadow does not drop](../shadowing_does_not_drop/README.md) — a borrow that outlives the *name* it borrowed from, and the `E0505` you get for freeing underneath it
- [`while let`](../../17_Option_and_Result/while_let/README.md) — a loop that has to look before it consumes, which turns out to be a borrow question
- [`Option` is a one-item collection](../../17_Option_and_Result/option_as_collection/README.md) — `take()`, the standard way to get a value *out* of something you only borrowed
- [The Rust Book, ch. 4.2 — References and Borrowing ↗](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Po polsku

Pożyczanie (*borrowing*) to sposób na skorzystanie z danych bez przejmowania ich na własność. Tour of Rust tłumaczy to jako „Pożyczanie Zasobu Przy Pomocy Referencji” i to dobra intuicja: referencja `&T` jest jak wypożyczenie książki z biblioteki — możesz czytać, ale właściciel się nie zmienia i książkę trzeba oddać, zanim biblioteka ją zutylizuje.

Reguła jest jedna i zawsze warto ją cytować w tej formie: **wielu czytających albo jeden piszący, nigdy jedno i drugie naraz.** `&T` to referencja współdzielona, `&mut T` to referencja mutowalna (wyłączna). Uwaga na częsty błąd tłumaczenia: `&T` bywa nazywane „referencją niemutowalną”, co jest mylące — `&` znaczy **współdzielona**, a nie „niezmienna”. Przez `&Cell<T>` czy `&Mutex<T>` da się pisać jak najbardziej.

Najtrudniejsza część nie jest jednak w regule, tylko w pytaniu **kiedy pożyczenie się kończy**. Od czasu NLL (*non-lexical lifetimes*) pożyczenie kończy się w miejscu **ostatniego użycia** referencji, a nie na zamykającym nawiasie klamrowym. To dlatego kod, który „powinien” się nie kompilować według starszych polskich poradników, kompiluje się bez problemu — sporo materiałów po polsku opisuje jeszcze stan sprzed 2018 roku.

Zwisająca referencja (*dangling reference*) nie jest w bezpiecznym Ruscie czymś, co można popełnić — kompilator odrzuca taki program, zamiast pozwolić mu paść w czasie działania. To jest właśnie ta gwarancja, za którą płaci się nauką kontrolera pożyczeń.

**Szukaj po polsku:** pożyczanie w Ruscie · referencje mutowalne · kontroler pożyczeń · `rust borrow checker` · `rust NLL non-lexical lifetimes`
