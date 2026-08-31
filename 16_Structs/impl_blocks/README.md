# `impl` blocks

**Level:** 101 → 201 · for newcomers

**One line:** Functions go in an `impl` block beside the struct, not in it — and a method is an associated function that happens to take `self`.

```text title="Real rustc output"
error: functions are not allowed in struct definitions
  |
3 |     fn people(&self) -> u32 { 3 }
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: unlike in C++, Java, and C#, functions are declared in `impl` blocks
```

Two boxes, not one. Neither owns the other:

```rust
struct Ballot { … }        // the data
impl   Ballot { … }        // the functions
```

---

## Why two boxes

The second box can be reopened later, and via traits for types you did not define. Three consequences:

- **Many `impl` blocks per type.** They add up. Nothing is overridden.
- **Not struct-only.** Enums take methods identically — `Option`'s hundred methods are one `impl<T> Option<T>` in std.
- **Sharing without inheritance.** Name the shape in a `trait`, write `impl Trait for Type`.

## Associated function vs method

Everything in an `impl` block is an **associated function**. A **method** is one whose first parameter is a `self` receiver — a subset, not the other half.

| | first parameter | called as |
|---|---|---|
| **associated function** | none | `Ballot::new("Ada")` |
| **method** | `self` / `&self` / `&mut self` | `ballot.total()` |

```rust
ballot.total()  ==  Ballot::total(&ballot)   // the dot is sugar; both are Ballot:: items
```

Those row labels are the everyday shorthand, and it reads the two as disjoint. The Reference does not: *"there are associated functions (including methods)"*, and *"associated functions whose first parameter is named `self` are called methods"* — [Associated Items ↗](https://doc.rust-lang.org/reference/items/associated-items.html). Worth knowing before an error message or a std page says "an associated function that is a method" and it looks like a contradiction.

"Standalone, called like `Foo::bar()`" = no instance to hang the call on, so you name the **type**.

No constructor syntax exists. `new` is an ordinary associated function returning `Self`; the name is a std convention.

**`Self`** (capital) is the *type*; **`self`** (lowercase) is the *value*.

## The three receivers

| Receiver | May | The caller | Use when |
|---|---|---|---|
| `&self` | read | keeps it | the method asks a question |
| `&mut self` | read and change | keeps it, changed | it updates in place |
| `self` | anything, incl. destroy it | **loses it** | the operation ends the value's life |

Anything named `into_*` takes `self`. The value being unusable afterwards is the point — a certified tally cannot be voted into.

```text
error[E0596]: cannot borrow `t` as mutable, as it is not declared as mutable
error[E0382]: borrow of moved value: `t`
```

First: a `&mut self` method called through a non-`mut` binding. The *signature* demands it, not the call. Second: using a value after a method took `self`.

## Inherent vs trait impl

```rust
impl Ballot             { fn total(&self) -> u32 { … } }        // inherent — your signature
impl Summary for Ballot { fn one_line(&self) -> String { … } }  // trait — someone else's
```

A trait may ship a **default method** body, overridable by any implementor. Closest thing to inherited implementation. Missing: no base class, no `super`, no reaching into a field.

Trait methods are only callable where the **trait is in scope**. An unexplained *"method not found"* is usually a missing `use`.

## If you are coming from another language

**Python.** A `class` body holds fields and `def`s; Rust splits them. `def total(self)` and `fn total(&self)` line up; `Ballot::new` is a `@classmethod`.

Two differences:

- `impl` is fixed at compile time. No monkeypatching, and you cannot add a method to another crate's type except through a trait.
- Python has one `self`, Rust has three. A Python method never declares whether the caller still owns the object afterwards.

**ABAP.** The closest bridge here — ABAP already splits the two boxes:

```abap
CLASS lcl_ballot DEFINITION.      " the shape
CLASS lcl_ballot IMPLEMENTATION.  " the bodies
```

| ABAP | Rust |
|---|---|
| `CLASS-METHODS` | associated function — `Ballot::new()` |
| `METHODS` | method — `ballot.total()` |
| `INTERFACE` | `trait` (but implementable for types you did not write) |
| `CLASS … IMPLEMENTATION` | `impl Ballot` |

Rust attaches the implementation to a plain structure type, so you get methods without an object.

No ABAP equivalent: `IMPORTING` / `CHANGING` / `EXPORTING` describe parameters, never the receiver. There is no way to say "this method consumes the object it was called on".

---

## Practice

**Pick the right receiver four times, then break two.** Model `Tally { contest: String, counts: Vec<u32> }` with create / who-leads / record-a-vote / certify. Choose each receiver *before* writing any bodies: one takes no `self`, one `&self`, one `&mut self`, one `self`.

1. Justify the last: what does consuming the tally *prevent*?
2. Call the recording method through a non-`mut` binding — `E0596`. Nothing in the call is wrong.
3. Use the tally after certifying — `E0382`.
4. Make `leader()` return `None`, not `Some(0)`, on an empty tally. Which is the bug you would rather ship?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:impl_blocks_kata -->
*[`impl_blocks_kata.rs`](examples/impl_blocks_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: pick the right receiver four times, then break two of them.
//!
//!   rustc --edition 2024 impl_blocks_kata.rs -o /tmp/ibk && /tmp/ibk

#[derive(Debug)]
struct Tally {
    contest: String,
    counts: Vec<u32>,
}

impl Tally {
    // 1. NO self. There is no Tally yet — this is the thing that makes one.
    fn new(contest: &str, candidates: usize) -> Self {
        Self { contest: contest.to_string(), counts: vec![0; candidates] }
    }

    // 2. &self. Asks a question and changes nothing. The caller keeps the tally,
    //    and two of these can run at once because shared borrows stack.
    fn leader(&self) -> Option<usize> {
        let best = *self.counts.iter().max()?;
        if best == 0 {
            return None;
        }
        self.counts.iter().position(|&c| c == best)
    }

    // 3. &mut self. Changes it, caller keeps it. Needs a `mut` binding to call.
    fn record(&mut self, candidate: usize) {
        self.counts[candidate] += 1;
    }

    // 4. self. Consumes it. Certifying ENDS the tally's life on purpose — you
    //    should not be able to record another vote into a certified result.
    fn certify(self) -> String {
        match self.leader() {
            Some(i) => format!("{}: candidate {} wins with {}", self.contest, i, self.counts[i]),
            None => format!("{}: no votes cast", self.contest),
        }
    }
}

fn main() {
    println!("Four operations, four different receivers:");
    println!("  new      no self     there is no value yet");
    println!("  leader   &self       asks, changes nothing");
    println!("  record   &mut self   changes it, you keep it");
    println!("  certify  self        ends it — that is the point\n");

    let mut t = Tally::new("Mayor", 3);
    println!("  fresh:  leader() = {:?}   (None, not Some(0) — nobody has voted)", t.leader());

    t.record(2);
    t.record(0);
    t.record(2);
    println!("  after 3 votes: counts {:?}, leader {:?}", t.counts, t.leader());

    println!("\nBreak 1 — call a &mut self method through a non-mut binding:");
    println!("    let t = Tally::new(..);  t.record(0);");
    println!("    error[E0596]: cannot borrow `t` as mutable, as it is not declared as mutable");
    println!("  The method signature is what demands it. `mut` on the BINDING is the answer.");

    println!("\nBreak 2 — use the value after a method that took `self`:");
    println!("    let receipt = t.certify();  t.record(1);");
    println!("    error[E0382]: borrow of moved value: `t`");
    println!("  Not a restriction to work around — it is the guarantee `self` buys:");
    println!("  a certified tally cannot be voted into, because it no longer exists.");

    println!("\n  {}", t.certify()); // t is consumed here, deliberately last
}
```
<!-- /source -->

<!-- output:impl_blocks_kata -->
*Verified output of [`impl_blocks_kata.rs`](examples/impl_blocks_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Four operations, four different receivers:
  new      no self     there is no value yet
  leader   &self       asks, changes nothing
  record   &mut self   changes it, you keep it
  certify  self        ends it — that is the point

  fresh:  leader() = None   (None, not Some(0) — nobody has voted)
  after 3 votes: counts [1, 0, 2], leader Some(2)

Break 1 — call a &mut self method through a non-mut binding:
    let t = Tally::new(..);  t.record(0);
    error[E0596]: cannot borrow `t` as mutable, as it is not declared as mutable
  The method signature is what demands it. `mut` on the BINDING is the answer.

Break 2 — use the value after a method that took `self`:
    let receipt = t.certify();  t.record(1);
    error[E0382]: borrow of moved value: `t`
  Not a restriction to work around — it is the guarantee `self` buys:
  a certified tally cannot be voted into, because it no longer exists.

  Mayor: candidate 2 wins with 2
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:impl_blocks -->
*Verified output of [`impl_blocks.rs`](examples/impl_blocks.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Associated function vs method — the only difference is `self`
   Ballot::new("Ada")  associated function, no instance existed yet
   b.add(5)            method, called on the value
   b.total() = 7

2. `b.total()` is sugar. This is the same call, spelled out:
   Ballot::total(&b) = 7   equal: true
   The dot inserts the `&` for you. That is the whole trick.

3. The three kinds of self, and what each costs the caller
   &self      total()        -> 7   caller keeps it
   &mut self  add(4)         -> caller keeps it, changed
              [5, 2, 0, 4]
   self       into_receipt() -> caller LOSES it
              Ada cast 4 scores
              `c` cannot be used again: E0382, borrow of moved value

4. Several impl blocks are fine — they add up
   b.is_blank() = false  (from the second impl Ballot block)

5. `impl` is not struct-only — enums take methods identically
   Elected("Ada")           -> Ada wins
   Tied(3)                  -> 3-way tie
   NoContest                -> no contest

6. Inherent impl vs trait impl
   inherent: you choose the signature
     b.total()      -> 7
   trait:    the trait chose it, so many types can answer
     b.one_line()   -> Ada scored 3 candidates, total 7
     Verdict.one_line() -> 3-way tie
   default method, inherited free by Ballot:
     b.shout()      -> ADA SCORED 3 CANDIDATES, TOTAL 7
   ...and overridden by Verdict:
     Verdict.shout()    -> *** 3-way tie ***
```
<!-- /output -->

## See also

- [STRUCTS.md](../../STRUCTS.md) · [What a struct is](../what_a_struct_is/README.md) · [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) · [Borrowing](../../18_Ownership/borrowing/README.md) · [the newtype](../newtype_score/README.md)

## Po polsku

Rust rozdziela to, co w Javie, C# czy ABAP-ie mieści się w jednym ciele klasy: struktura trzyma **dane**, a blok `impl` — **funkcje**. Kompilator mówi to wprost, i to jednym z niewielu komunikatów `rustc`, które wymieniają inne języki z nazwy: „functions are not allowed in struct definitions … unlike in C++, Java, and C#, functions are declared in `impl` blocks”. Żadne z tych dwóch pudełek nie należy do drugiego — bloków `impl` dla jednego typu może być wiele i one się **sumują** (nic się nie nadpisuje), a `impl` nie jest zarezerwowany dla struktur: wyliczenia (*enums*) dostają metody dokładnie tak samo, bo sto metod `Option` to w std jeden `impl<T> Option<T>`.

Wszystko, co stoi w bloku `impl`, jest **funkcją powiązaną** (*associated function*; po polsku spotyka się też „funkcję stowarzyszoną”, więc szukaj raczej po angielsku — polskie nazewnictwo się tu rozjeżdża). **Metoda** to taka funkcja powiązana, której pierwszym parametrem jest odbiornik `self`; to podzbiór, a nie druga kategoria obok. `ballot.total()` znaczy dokładnie `Ballot::total(&ballot)` — kropka sama dokłada `&`. Rust nie ma też słowa kluczowego na konstruktor: `new` to zwyczajna funkcja powiązana zwracająca `Self`, sama konwencja biblioteki standardowej. I pułapka czysto pisarska, o której warto pamiętać, notując po polsku: `Self` (typ) i `self` (wartość) różnią się **tylko wielkością litery**, więc nigdy nie zaczynaj zdania od `self` — po kropce wyjdzie z tego coś zupełnie innego.

Trzy odbiorniki to trzy różne umowy z wywołującym i tak najlepiej je czytać: `&self` tylko pyta (wywołujący zachowuje wartość), `&mut self` zmienia ją w miejscu (zachowuje, ale zmienioną), a `self` **przejmuje ją na własność**, więc wywołujący ją traci. Stąd dwa błędy z tej lekcji. `E0596` („cannot borrow `t` as mutable”) pojawia się przy wywołaniu metody `&mut self` przez niemutowalne wiązanie — wadliwe jest wiązanie, nie wywołanie, więc lekarstwem jest `mut` przy `let`, a nie zmiana w sygnaturze. `E0382` po metodzie biorącej `self` (`certify`, każde `into_*`) to z kolei nie ograniczenie do obejścia, tylko cała gwarancja, którą kupuje `self`: do certyfikowanego wyniku nie da się dopisać głosu, bo ten wynik już nie istnieje. Osobno warto zapamiętać, że metody z cechy (*trait*) są widoczne dopiero wtedy, gdy sama cecha jest w zasięgu — niespodziewane „method not found” to zwykle brakujący `use`.

**Szukaj po polsku:** bloki impl · funkcje powiązane · metody i struktury w Ruscie · `rust impl block associated function` · `rust E0596 cannot borrow as mutable`
