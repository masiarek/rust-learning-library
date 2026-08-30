# `Drop`, and what RAII buys

**Level:** 201 · working knowledge

**One line:** `Drop::drop` runs at a place you can point to in the source — the end of the owner's scope — which is what lets a value *be* the thing that releases a lock, closes a file, or writes a summary, with no `finally` and nothing for the caller to remember.

```rust
struct Tally { name: &'static str, total: u32 }

impl Drop for Tally {
    fn drop(&mut self) {
        println!("close {} at {}", self.name, self.total);
    }
}

fn main() {
    let mut t = Tally { name: "round 1", total: 0 };
    t.total += 8;
}   // close round 1 at 8
```

There is no `close()` to call. That is RAII — *resource acquisition is initialisation* — and in Rust it is not a pattern you apply but the way every value already works: `String` frees its buffer, `File` closes its handle, `MutexGuard` releases its lock, all in `drop`.

## Three drop orders, and two of them are opposites

| What | Order |
|---|---|
| local variables in a block | **reverse** declaration order |
| a struct's own `Drop` vs its fields | the struct **first**, then fields in declaration order |
| the elements of a `Vec` | front to back |

Both of the first two are forced. Locals unwind because a later value may borrow an earlier one. A struct goes first because your `drop` body may still need to read its fields — it takes `&mut self`, and the fields have to be alive for that.

## Ending a value early

```rust
fn main() {
    let t = Tally { name: "early", total: 0 };
    drop(t);
    println!("already closed");
}
```

`drop(x)` is a one-line function that takes `x` by value and does nothing. The value dies because it was **moved into a function that ended** — no special case in the language. Which is also why you cannot call it yourself:

```text
error[E0040]: explicit use of destructor method
 --> e40.rs:7:7
  |
7 |     l.drop();
  |       ^^^^ explicit destructor calls not allowed
  |
help: consider using `drop` function
  |
7 -     l.drop();
7 +     drop(l);
```

## The trap: `let _ =` does not bind

```rust
fn main() {
    let _ = Guard::acquire("lock");   // released immediately
    do_work();                        // ...with no lock held

    let _g = Guard::acquire("lock");  // held to the end of the scope
    do_work();
}
```

`_` is not a binding, it is a pattern that discards — so the guard is a temporary and drops at the **end of that statement**. The lock is released before the work starts, the program is correct-looking and wrong, and nothing warns.

The fix is one character: `let _g = …`. This is the most common RAII mistake in Rust, and the reason `MutexGuard` is `#[must_use]`.

## What `Drop` cannot do

- **Return anything.** It takes `&mut self`, so it cannot move a value out.
- **Fail.** There is no `Result`. A `close()` that can genuinely error still has to be called by hand, with `Drop` as the backstop that at least frees the handle. `std::fs::File` is exactly this: its `drop` ignores the close error, and `File::sync_all` is there for when you need to know.
- **Be guaranteed to run.** `std::mem::forget` skips it, an `Rc` cycle skips it, and a process that aborts rather than unwinds skips all of them. **Leaking is safe in Rust** — the language promises soundness, not the absence of garbage.

What it *does* survive is an early `return` and a panic, because both unwind the stack through the same machinery. That is what turns a guard from a convention into a guarantee.

## If you are coming from another language

- **Python.** `__del__` is the false friend: it exists, it is not deterministic, and everyone is told not to use it. The real counterpart is the **context manager** — `__enter__`/`__exit__` and the `with` statement — and the mapping is exact except for one thing: Python makes the scope explicit at every call site (`with open(p) as f:`) while Rust attaches it to the *value*, so a Rust `File` cannot be used outside its lifetime by construction. That is the trade: no `with` to forget, and no way to say "release this early" except `drop()`. Reference counting means CPython usually frees at the same moment Rust would, so the *timing* will feel familiar; what changes is that Rust's timing is a rule rather than an implementation detail, and PyPy will show you the difference.
- **ABAP.** There is nothing deterministic. An instance is released by the garbage collector at an unspecified time, and `CLASS_DESTRUCTOR`/`~DESTRUCTOR` fires whenever that happens — so the ABAP habit is the opposite of RAII: acquire, then release explicitly in a `CLEANUP` section or at the end of the method, and hope no early `RETURN` skips it. `TRY … CLEANUP … ENDTRY` is the closest mechanism, and it is exactly what `Drop` replaces. Two things transfer usefully. The discipline of pairing `OPEN CURSOR` with `CLOSE CURSOR`, or `ENQUEUE` with `DEQUEUE`, is the pattern RAII automates — in Rust the lock object *is* the `DEQUEUE`. And the ABAP bug where an early `RETURN` skips the `DEQUEUE` is the bug `Drop` makes unrepresentable, which is worth knowing as the concrete payoff rather than as theory.
- **C++.** This is C++'s RAII, borrowed name and all, with three differences: destructors run on **move-out** in C++ only after the moved-from object is destroyed (Rust simply does not run drop on a moved-out value at all), Rust has no destructor for a partially-moved value to worry about, and `Drop` cannot be called explicitly. If you already think in `std::lock_guard`, you already think in this.
- **Java / C#.** `try-with-resources` and `using`, which are the `with` statement's cousins — scope-based, explicit at the call site, and forgettable. `finalize()`/`Finalize()` is `__del__`: non-deterministic, deprecated, not this.

---

## The verified output

<!-- output:drop_and_raii -->
*Verified output of [`drop_and_raii.rs`](examples/drop_and_raii.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Last declared, first dropped
     end of block
     drop c
     drop b
     drop a
   Reverse declaration order, always — because a later value may
   borrow an earlier one, and never the other way round.

2. Moving moves the drop with it
     consume got moved
     drop moved
     back in main, and it is already gone
   The drop happened at the end of `consume`, not here. Whoever owns
   the value owns the free — which is what makes ownership readable:
   find the last owner and you have found the deallocation.

3. Ending a value early
     drop early
     still in the block, and it is gone
   `drop(x)` is a one-line function that takes x by value and does
   nothing. The value dies because it was MOVED into a function that
   ended — no magic, and it is why you cannot call `l.drop()`
   yourself: that is E0040, "explicit use of destructor method".

4. RAII: the release you cannot forget
     open round 1
     ...counting...
     close round 1 at 8
   There is no close() to call and no `finally` to write. The
   compiler inserted the call at the end of the scope, and it runs
   on the early-return path and on the panic path too.

5. Fields drop after the struct, in declaration order
     built from field first and field second
     end of block
     drop Pair itself
     drop field first
     drop field second
   The struct's own Drop::drop runs FIRST, then its fields in
   declaration order — the opposite of the local-variable rule, and
   the one that surprises people. It has to be that way: your drop
   body may still need the fields.

6. What Drop is not for
   Not for returning a value: drop takes &mut self, so it cannot
   move anything out, and it cannot fail — there is no Result.
   A close() that can error still needs to be called by hand, with
   Drop as the backstop that at least frees the handle.
   And a value can leak: std::mem::forget, an Rc cycle, or a process
   that aborts all skip it. Leaking is SAFE in Rust; unsoundness is
   what the language promises to prevent, not garbage.
```
<!-- /output -->

## Practice

**Three drop orders, and the guard released one line early.** Build a type that logs when it is dropped and use it to observe three sequences: three locals in a block, a struct with two fields and its own `Drop`, and a `Vec` of three. Two of the three orders are opposites — say why each has to be the way it is.

Then write a `Guard` that logs on acquire and on release, and use it three ways: bound to a name for the whole block, explicitly `drop`ped halfway, and bound to a bare `_`. One of the three releases the lock before the work starts. Find it in the log, then say what the one-character fix is.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:drop_and_raii_kata -->
*[`drop_and_raii_kata.rs`](examples/drop_and_raii_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three drop orders, and the guard released one line early.
//!
//!   rustc --edition 2024 drop_and_raii_kata.rs -o /tmp/drk && /tmp/drk

use std::cell::RefCell;

// A shared log, so the whole run can be printed in order at the end.
thread_local! {
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

fn note(s: String) {
    LOG.with(|l| l.borrow_mut().push(s));
}

fn take_log() -> Vec<String> {
    LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

struct Named(&'static str);

impl Drop for Named {
    fn drop(&mut self) {
        note(format!("drop {}", self.0));
    }
}

struct Outer {
    first: Named,
    second: Named,
}

impl Drop for Outer {
    fn drop(&mut self) {
        note("drop Outer itself".to_string());
    }
}

/// The RAII guard: it holds the lock for exactly as long as it is alive.
struct Guard(&'static str);

impl Guard {
    fn acquire(name: &'static str) -> Self {
        note(format!("acquire {name}"));
        Guard(name)
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        note(format!("release {}", self.0));
    }
}

fn show(title: &str) {
    println!("   {title}");
    for line in take_log() {
        println!("     {line}");
    }
}

fn main() {
    println!("1. Locals: reverse declaration order");
    {
        let _a = Named("a");
        let _b = Named("b");
        let _c = Named("c");
        note("end of block".to_string());
    }
    show("");

    println!();
    println!("2. A struct: itself first, then its fields IN declaration order");
    {
        let o = Outer { first: Named("field first"), second: Named("field second") };
        note(format!("built from {} and {}", o.first.0, o.second.0));
        note("end of block".to_string());
    }
    show("");
    println!("   Opposite rules, and both are forced. A later local may borrow an");
    println!("   earlier one, so locals unwind. A struct's own drop body may still");
    println!("   read its fields, so the struct goes first and the fields follow in");
    println!("   the order they were written.");

    println!();
    println!("3. A Vec drops front to back");
    {
        let _v = vec![Named("v0"), Named("v1"), Named("v2")];
        note("end of block".to_string());
    }
    show("");

    println!();
    println!("4. The guard released one line early");
    {
        let _g = Guard::acquire("whole block");
        note("work with the lock held".to_string());
    }
    show("held to the end of the block:");
    {
        let g = Guard::acquire("dropped early");
        note("work with the lock held".to_string());
        drop(g);
        note("work with the lock NOT held".to_string());
    }
    show("explicitly released, then more work:");
    {
        let _ = Guard::acquire("bound to underscore");
        note("work — but is the lock held?".to_string());
    }
    show("bound to a bare `_`:");
    println!("   The third is the bug. `let _ = Guard::acquire(..)` does not bind");
    println!("   anything, so the guard is a temporary that drops at the END OF");
    println!("   THAT STATEMENT — the lock is released before the work starts. The");
    println!("   fix is one character: `let _g = ...`, which is a binding and lives");
    println!("   to the end of the scope. This is the most common RAII mistake in");
    println!("   Rust, and it compiles without a warning.");

    println!();
    println!("5. Drop still runs when the work does not finish");
    fn early_return(fail: bool) -> &'static str {
        let _g = Guard::acquire("across an early return");
        if fail {
            return "returned early";
        }
        "ran to the end"
    }
    let r = early_return(true);
    show(&format!("early_return(true) -> {r}"));
    println!("   And on a panic too, while the stack unwinds — which is what makes");
    println!("   a guard a guarantee rather than a convention. The exceptions are");
    println!("   std::mem::forget, a reference cycle, and a process that aborts");
    println!("   instead of unwinding.");
}
```
<!-- /source -->

<!-- output:drop_and_raii_kata -->
*Verified output of [`drop_and_raii_kata.rs`](examples/drop_and_raii_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Locals: reverse declaration order
   
     end of block
     drop c
     drop b
     drop a

2. A struct: itself first, then its fields IN declaration order
   
     built from field first and field second
     end of block
     drop Outer itself
     drop field first
     drop field second
   Opposite rules, and both are forced. A later local may borrow an
   earlier one, so locals unwind. A struct's own drop body may still
   read its fields, so the struct goes first and the fields follow in
   the order they were written.

3. A Vec drops front to back
   
     end of block
     drop v0
     drop v1
     drop v2

4. The guard released one line early
   held to the end of the block:
     acquire whole block
     work with the lock held
     release whole block
   explicitly released, then more work:
     acquire dropped early
     work with the lock held
     release dropped early
     work with the lock NOT held
   bound to a bare `_`:
     acquire bound to underscore
     release bound to underscore
     work — but is the lock held?
   The third is the bug. `let _ = Guard::acquire(..)` does not bind
   anything, so the guard is a temporary that drops at the END OF
   THAT STATEMENT — the lock is released before the work starts. The
   fix is one character: `let _g = ...`, which is a binding and lives
   to the end of the scope. This is the most common RAII mistake in
   Rust, and it compiles without a warning.

5. Drop still runs when the work does not finish
   early_return(true) -> returned early
     acquire across an early return
     release across an early return
   And on a panic too, while the stack unwinds — which is what makes
   a guard a guarantee rather than a convention. The exceptions are
   std::mem::forget, a reference cycle, and a process that aborts
   instead of unwinding.
```
<!-- /output -->

</details>

---

## See also

- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — whose scope the drop happens in
- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) — the timing this page depends on, from the other side
- [Lock poisoning](../../09_Advanced/mutex_poisoning/README.md) — what a guard does when the thread holding it panics
- [`Box`](../../26_Collections/the_box/README.md) — the recursive drop that can overflow the stack
- [Marker traits](../marker_traits/README.md) — why a type that implements `Drop` cannot also be `Copy`
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — the unwind that still runs every `drop` on the way out

## Sources

[RAII ↗](https://doc.rust-lang.org/rust-by-example/scope/raii.html) and [Drop ↗](https://doc.rust-lang.org/rust-by-example/trait/drop.html) in Rust by Example; [`std::ops::Drop` ↗](https://doc.rust-lang.org/std/ops/trait.Drop.html) and [`std::mem::drop` ↗](https://doc.rust-lang.org/std/mem/fn.drop.html), whose one-line body is quoted above. The `E0040` transcript is a real compile of the two-line program it describes.

## Po polsku

Wypuszczanie zasobu (*drop*) w Ruscie zdarza się w miejscu, które da się wskazać palcem w kodzie — na końcu zasięgu właściciela. Tour of Rust nazywa ten rozdział „Wypuszczanie Zasobów” i to trafne słowo, bo `Drop::drop` nie jest odpowiednikiem Pythonowego `__del__` ani javowego finalizatora: nic tu nie czeka na odśmiecacz, kompilator po prostu wstawia wywołanie w konkretnym wierszu. Samo RAII zostaw po angielsku — skrót jest międzynarodowy i tak samo zapisywany w polskich książkach.

I właśnie stamtąd, z polskiej literatury o C++, większość czytelników zna „destruktor” i RAII — co jest ogromną przewagą, ale w trzech miejscach myli. Po pierwsze, destruktora nie wywołasz ręcznie: `l.drop()` to `error[E0040]`, a `drop(l)` to zwyczajna jednolinijkowa funkcja, która przyjmuje wartość przez przeniesienie własności i nie robi nic — wartość ginie dlatego, że trafiła do funkcji, która się skończyła. Po drugie, po przeniesieniu własności stara zmienna nie jest już wypuszczana w ogóle (nie ma czegoś takiego jak „pusty obiekt po przeniesieniu”, który i tak trzeba zniszczyć). Po trzecie, kolejności są dwie i są przeciwstawne: zmienne lokalne giną w **odwrotnej** kolejności deklaracji, ale struktura ginie **najpierw sama**, a dopiero potem jej pola, w kolejności zapisania — inaczej się nie da, bo ciało `drop` dostaje `&mut self` i musi jeszcze widzieć te pola.

Pułapka tej strony ma szerokość jednego znaku: `_` **nie jest wiązaniem**, tylko wzorcem, który odrzuca. Po `let _ = Guard::acquire("lock")` strażnik jest wartością tymczasową i ginie na końcu tej instrukcji — blokada zostaje zwolniona, zanim praca się zacznie, program wygląda poprawnie i nic nie ostrzega. Poprawka to `let _g = …`. Warto też zapamiętać, czego `Drop` nie potrafi: niczego nie zwraca, nie umie zawieść (nie ma `Result`, więc `close()`, które naprawdę może się nie udać, i tak trzeba wywołać ręcznie) i nie ma gwarancji, że w ogóle się wykona — `std::mem::forget`, cykl `Rc` czy `abort` pomijają go. Wyciek pamięci jest w Ruscie **bezpieczny**: język obiecuje brak niezdefiniowanego zachowania, a nie brak śmieci, i to zdanie warto mieć pod ręką, bo polskie streszczenia w stylu „Rust gwarantuje bezpieczeństwo pamięci” bywają czytane jako „Rust nie przecieka”.

**Szukaj po polsku:** wypuszczanie zasobów · RAII destruktor · `rust Drop trait` · `rust E0040 explicit use of destructor method` · `rust let _ drops immediately`
