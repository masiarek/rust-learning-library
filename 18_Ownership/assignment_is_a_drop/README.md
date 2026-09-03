# Assignment drops the old value

**Level:** 201 · working knowledge

**One line:** `x = value` is not an edit — it frees whatever `x` was holding, so a value can die in the middle of a function, on a line with no closing brace anywhere near it.

```rust
struct Tally(&'static str);

impl Drop for Tally {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let mut round = Tally("round 1");
    round = Tally("round 2");     // drop round 1
    println!("now {}", round.0);  // now round 2
}                                 // drop round 2
```

Two values, two drops. The second one is the familiar rule — a value dies when its owner's scope ends. The first is the one nothing on the page announces: `round 1` was freed by an assignment statement that reads like a change of contents.

## The order is the half people get wrong

The assignment does three things, and it does them in this order:

1. evaluate the right-hand side,
2. drop what the place currently holds,
3. move the new value in.

So the new value already exists when the old one dies — which is why `v = v.into_iter().rev().collect()` works, and why a `Drop` impl that logs cannot log the two events the other way round. The [Rust Reference ↗](https://doc.rust-lang.org/reference/expressions/operator-expr.html#assignment-expressions) states it as a rule, and the run below shows it: `built round 2` prints *before* `drop round 1`.

## Two assignments that drop nothing

An assignment drops what was **there**. Twice in ordinary Rust, nothing is:

```rust
let mut owner = Tally("owner");
let elsewhere = owner;          // moved out — the location is now empty
owner = Tally("replacement");   // legal, and drops nothing

let later;                      // declared, holding nothing
later = Tally("first value");   // also drops nothing
```

Re-initialising a moved-out binding surprises people twice over — first that it compiles at all, then that it is silent. Both follow from the same rule, and the compiler tracks which of the two situations a location is in; when it cannot tell statically, it keeps [a hidden boolean on the stack](../the_drop_flag/README.md) to decide at run time.

The third case is the one to hold beside these, because it looks identical and is not:

```rust
let e = Tally("eight");
let e = Tally("nine");    // a shadow. Nothing is dropped, and `eight` is still alive.
```

`let` **declares**; `=` **writes**. A shadow adds a second variable and hides the first name, so both values live to the end of the block and [the shadow drops first](../shadowing_does_not_drop/README.md). One character of difference in what you type, and a completely different schedule.

## Through a `&mut`, and why `mem::replace` exists

```rust
let r = &mut slot;
*r = Tally("new");     // drop old
```

`*r = value` is an assignment, so it drops too. And it has to: you cannot move a value *out* through a `&mut`, because that would leave the borrowed location empty while somebody else still has a reference to it. Dropping is the only thing the write can do with the old value.

Which is exactly the gap [`std::mem::replace` ↗](https://doc.rust-lang.org/std/mem/fn.replace.html) fills — it does the same write and hands the old value back instead of destroying it:

| You want | Write | Needs |
|---|---|---|
| the old value gone | `*r = new` | nothing |
| the old value back | [`mem::replace(r, new)` ↗](https://doc.rust-lang.org/std/mem/fn.replace.html) | nothing |
| the old value back, leaving a default | [`mem::take(r)` ↗](https://doc.rust-lang.org/std/mem/fn.take.html) | `T: Default` |
| two values exchanged | [`mem::swap(a, b)` ↗](https://doc.rust-lang.org/std/mem/fn.swap.html) | nothing |

None of the three clones anything; all of them are the borrow checker's answer to "I need to get this value out of a place I only have a `&mut` to". `take` is the one you reach for most, usually on a struct field: `let items = mem::take(&mut self.items);` leaves an empty `Vec` behind and gives you the full one, with no allocation and no `clone`.

## Where it bites

```rust
let mut buffer = String::new();
for line in lines {
    buffer = String::from(line);   // allocates, and frees last round's buffer
}
```

Every `=` here is a free plus the next allocation. `buffer.clear()` then `push_str(line)` [keeps the buffer](../../14_Strings/string_methods/string_clear/README.md) and allocates once for the whole loop. The same applies to `v = Vec::new()` against [`v.clear()`](../../26_Collections/vec_methods/vec_clear/README.md).

The louder version is a type whose `Drop` has side effects — a lock, a transaction, a file, a span. Assigning a new one *releases the old one first*, and the release happens at the assignment rather than at the end of the block:

```rust
let mut guard = lock_for("round 1");
guard = lock_for("round 2");   // round 1 is released here, before round 2 is stored
```

Read that as an edit and the release is invisible. `rustc` will tell you when the old value was never even read — `warning: value assigned to 'round' is never read` — but that is a lint about the *read*, not about the drop, and it stays quiet the moment you use the value once.

## If you are coming from another language

- **C++.** This is the destructor-plus-move-assignment story you already know, with the operator taken away. `a = b` in C++ calls `operator=`, which is a function the type author wrote and which typically reuses the destination's storage — `std::string::operator=` keeps the existing capacity when it can. Rust has no assignment operator to overload: `=` always drops the old value and moves the new one in, and the reuse you get for free in C++ is opt-in here as [`clone_from`/`clone_into`](../../12_Traits/clone_into/README.md). The other half transfers exactly: a moved-from value in C++ is left in a valid-but-unspecified state and *is* still destroyed, while Rust simply does not run the destructor at all — which is why C++ move constructors have to null out the source and Rust's do not exist.
- **Python.** `x = value` rebinds a name; the object formerly bound is freed only if its refcount hits zero, and *when* is CPython's business. Rust's `=` is a write to a fixed location with a deterministic free attached, so the two languages differ in both halves — Rust does not rebind (that is `let`, and it makes a new variable) and does not defer. The habit worth unlearning is reaching for reassignment where you want a new binding: in Rust that choice decides whether anything is dropped at all.
- **ABAP.** `lv_x = lo_obj` copies a reference and the old instance is collected whenever the garbage collector gets to it. There is no moment in the source you can point to. The Rust rule gives you back the thing `CLEANUP` was for: the release is at the assignment, in the code, every time. If you have written `IF lo_lock IS BOUND. lo_lock->release( ). ENDIF.` before overwriting a handle, that `IF` is what Rust does automatically — and `mem::replace` is the version that hands you the old handle so you can do something with it first.
- **Java / C#.** As Python, minus the refcounting: assignment drops a reference and the object leaves when the collector says so. `IDisposable`/`AutoCloseable` is the manual approximation, and reassigning a variable holding one is the classic leak — nothing calls `Dispose()` for you. That leak is not expressible here.

## The verified output

<!-- output:assignment_is_a_drop -->
*Verified output of [`assignment_is_a_drop.rs`](examples/assignment_is_a_drop.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Assignment drops the old value
     built round 1
     round 1 counted 12 ballots
     built round 2
     drop  round 1 (12)
     still inside the block, one line to go
     round is now round 2 (30)
     drop  round 2 (30)
   `round 1` died on the assignment line. No brace closed, nothing
   went out of scope, and the statement reads like an edit.
   Read the order too: BUILT round 2 comes before DROP round 1.
   The right-hand side is evaluated first, then the old value is
   dropped, then the new one is stored.

2. Two assignments that drop nothing
     built owner
     moved out of `owner`; now assigning to it again:
     built replacement
     no drop line appeared — the location held nothing to drop
     both are alive: owner and replacement
     drop  owner (1)
     drop  replacement (2)
     `let later;` declares a name over an empty location
     built first value
     no drop line here either: first value (3)
     drop  first value (3)
   An assignment drops what was there. Twice above, nothing was.

3. Writing through &mut T drops too
     built old
     built new
     drop  old (7)
     the write through `r` dropped what `slot` held
     slot is now new (8)
     drop  new (8)
   `*r = value` is an assignment, so the same rule applies — and
   you cannot move the old value out through `&mut`, only drop it.

4. Keeping the old value instead of dropping it
     built outgoing
     built incoming
     replace handed the old value back: outgoing (5)
     no drop line — it is alive, and it is mine now
     slot holds incoming (6)
     end of block:
     drop  outgoing (5)
     drop  incoming (6)
   That is the whole difference between `*r = v` and
   `mem::replace(r, v)`: one drops the old value, one returns it.

5. The other two, on types that need no Drop impl to show it
     take:  taken = "Ada", name = ""   (name got Default)
     swap:  a = "second", b = "first"   (nothing dropped, nothing cloned)
   `take` needs Default, `swap` needs nothing, `replace` needs neither.

6. Why it is worth knowing
     the loop assigned 3 times, so 3 buffers were freed: "third"
     clear + push_str reuses one buffer:              "third"
   Same result, and the first version does an allocation and a free
   per round because every `=` freed what was there.
```
<!-- /output -->

## Practice

**Six statements, and only two of them drop anything.** Write a type that prints when it is dropped, then run these six situations in separate blocks and predict the log before you look: a plain reassignment; the first write to a deferred `let x;`; a write to a binding you have already moved out of; a write through a `&mut`; a shadow (`let e = …; let e = …;`); and a `mem::replace`.

For each one say *where* the old value went — dropped on that line, dropped at the brace, or handed back to you. Two of the six drop something where the statement sits. The interesting disagreement is between the shadow and the reassignment, which differ by one keyword and by everything else.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:assignment_is_a_drop_kata -->
*[`assignment_is_a_drop_kata.rs`](examples/assignment_is_a_drop_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: six statements, and only three of them drop anything.
//!
//!   rustc --edition 2024 assignment_is_a_drop_kata.rs -o /tmp/aiadk && /tmp/aiadk

use std::mem;

struct T(&'static str);

impl Drop for T {
    fn drop(&mut self) {
        println!("       drop {}", self.0);
    }
}

fn main() {
    println!("A. reassignment                     let mut a = T(1); a = T(2);");
    {
        let mut a = T("one");
        println!("       a holds {}", a.0);
        a = T("two");
        println!("       <- one drop, above, on the assignment line");
        println!("       a now holds {}", a.0);
        println!("       end of block:");
    }

    println!("\nB. first write to a deferred `let`  let b; b = T(3);");
    {
        let b;
        b = T("three");
        println!("       b holds {}   <- nothing dropped: the location was empty", b.0);
        println!("       end of block:");
    }

    println!("\nC. write to a moved-out binding     let m = c; c = T(5);");
    {
        let mut c = T("four");
        let m = c;
        c = T("five");
        println!("       m = {}, c = {}   <- nothing dropped: `c` had been emptied", m.0, c.0);
        println!("       end of block:");
    }

    println!("\nD. write through &mut               *r = T(7);");
    {
        let mut d = T("six");
        let r = &mut d;
        *r = T("seven");
        println!("       <- one drop, above: `*r = v` is an assignment");
        println!("       d = {}", d.0);
        println!("       end of block:");
    }

    println!("\nE. shadowing                        let e = T(8); let e = T(9);");
    {
        let e = T("eight");
        println!("       e = {}", e.0);
        let e = T("nine");
        println!("       e = {}   <- nothing dropped: `let` declares, it does not write", e.0);
        println!("       end of block, and BOTH are still alive:");
    }
    println!("       ^ reverse declaration order, so the shadow dies first and");
    println!("         `eight` outlives the name that hid it.");

    println!("\nF. mem::replace                     let old = replace(&mut f, T(11));");
    {
        let mut f = T("ten");
        let old = mem::replace(&mut f, T("eleven"));
        println!("       old = {}, f = {}   <- nothing dropped: it was returned", old.0, f.0);
        println!("       end of block:");
    }

    println!("\nTally: two of the six dropped anything where the statement sits,");
    println!("       A and D. B and C did not, because an assignment drops what");
    println!("       was THERE and both locations were empty; E did not, because");
    println!("       `let` declares rather than writes; F did not, because the old");
    println!("       value was handed back instead.");
}
```
<!-- /source -->

<!-- output:assignment_is_a_drop_kata -->
*Verified output of [`assignment_is_a_drop_kata.rs`](examples/assignment_is_a_drop_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
A. reassignment                     let mut a = T(1); a = T(2);
       a holds one
       drop one
       <- one drop, above, on the assignment line
       a now holds two
       end of block:
       drop two

B. first write to a deferred `let`  let b; b = T(3);
       b holds three   <- nothing dropped: the location was empty
       end of block:
       drop three

C. write to a moved-out binding     let m = c; c = T(5);
       m = four, c = five   <- nothing dropped: `c` had been emptied
       end of block:
       drop four
       drop five

D. write through &mut               *r = T(7);
       drop six
       <- one drop, above: `*r = v` is an assignment
       d = seven
       end of block:
       drop seven

E. shadowing                        let e = T(8); let e = T(9);
       e = eight
       e = nine   <- nothing dropped: `let` declares, it does not write
       end of block, and BOTH are still alive:
       drop nine
       drop eight
       ^ reverse declaration order, so the shadow dies first and
         `eight` outlives the name that hid it.

F. mem::replace                     let old = replace(&mut f, T(11));
       old = ten, f = eleven   <- nothing dropped: it was returned
       end of block:
       drop ten
       drop eleven

Tally: two of the six dropped anything where the statement sits,
       A and D. B and C did not, because an assignment drops what
       was THERE and both locations were empty; E did not, because
       `let` declares rather than writes; F did not, because the old
       value was handed back instead.
```
<!-- /output -->

</details>

## See also

- [Scope is about names, not values](../scope_is_about_names/README.md) — the five *other* things that move a value's death, and the one `_` the compiler refuses
- [A shadow does not drop](../shadowing_does_not_drop/README.md) — the contrast case, in full
- [The drop flag](../the_drop_flag/README.md) — what the compiler does when it cannot tell statically whether a location is full
- [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) — the trait, its three orders, and the three ways a value escapes being dropped at all
- [`clone_into`](../../12_Traits/clone_into/README.md) — reusing the destination's buffer instead of replacing it

## Po polsku

Przypisanie `x = wartość` **nie jest edycją**: najpierw powstaje nowa wartość, potem zostaje wypuszczona (*dropped*) ta, którą zmienna trzymała, a dopiero na końcu nowa trafia na jej miejsce. Wartość może więc umrzeć w środku funkcji, w linijce, przy której nie ma żadnego nawiasu klamrowego — a to jest moment, którego uczy się jako „koniec zasięgu”.

Dwa przypisania nie wypuszczają niczego, bo nie mają czego: przypisanie do zmiennej, z której wartość została wcześniej **przeniesiona**, oraz pierwsze przypisanie do zadeklarowanej, ale niezainicjalizowanej zmiennej (`let x;`). Trzeci przypadek wygląda tak samo, a jest czymś zupełnie innym — **przesłanianie** (`let e = …; let e = …;`) niczego nie wypuszcza, bo `let` *deklaruje*, a nie *zapisuje*: powstaje druga zmienna, obie wartości żyją do końca bloku, a przesłaniająca ginie pierwsza.

Zapis przez referencję mutowalną (`*r = wartość`) też wypuszcza starą wartość — i musi, bo przez `&mut` nie wolno wartości *wyprowadzić*, zostawiłoby to pożyczone miejsce puste. Stąd biorą się trzy funkcje ze `std::mem`: `replace` oddaje starą wartość zamiast ją niszczyć, `take` zostawia w to miejsce wartość domyślną (wymaga `Default`), a `swap` zamienia dwie wartości miejscami. Żadna z nich niczego nie kopiuje — to jest odpowiedź *borrow checkera* na pytanie „jak wyjąć wartość z miejsca, do którego mam tylko `&mut`”.

Praktyczna konsekwencja: `bufor = String::from(linia)` w pętli zwalnia poprzedni bufor i alokuje nowy w każdym obrocie, podczas gdy `bufor.clear()` plus `push_str(linia)` alokuje raz. A gdy typ ma `Drop` z efektem ubocznym — blokada, transakcja, plik — przypisanie **zwalnia poprzedni zasób w tej właśnie linijce**, nie na końcu bloku.

**Szukaj po polsku:** przypisanie a wypuszczanie wartości · `rust drop on assignment` · `std::mem::replace` · `rust mem take swap` · przenoszenie własności a przesłanianie
