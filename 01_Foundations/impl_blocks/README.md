# `impl` blocks

**Level:** 101 → 201 · for newcomers

**One line:** The functions do not go in the struct — they go in an `impl` block beside it, and the only thing that decides "associated function" from "method" is whether the first parameter is `self`.

Try putting a function in a struct and the compiler does not just refuse; it explains its own design:

```text title="Real rustc output"
error: functions are not allowed in struct definitions
  |
1 | struct Floor {
  |        ----- while parsing this struct
3 |     fn people(&self) -> u32 { 3 }
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: unlike in C++, Java, and C#, functions are declared in `impl` blocks
```

That help line is the whole page in one sentence. If you learned objects anywhere else, you learned that data and behaviour live in **one** box. Rust puts them in two:

```rust
struct Ballot { … }        // the box with the data

impl Ballot { … }          // the box with the functions
```

They are not nested and neither owns the other. `impl Ballot` says *"the following functions are associated with the type `Ballot`"*, and that is all it says.

---

## Why two boxes and not one

Because the second box can be opened again, by you, later, and — with traits — for types you did not define. One box would have to be closed when the type is defined. Three consequences follow, and each surprises somebody:

- **A type may have many `impl` blocks.** They add up. Nothing is being reopened or overridden.
- **`impl` is not struct-only.** Enums take methods identically — `Option`'s hundred methods are one `impl<T> Option<T>` in the standard library, no different from yours. So do type aliases of your own types, and unions.
- **Behaviour can be *shared* without inheritance,** by naming the shape in a `trait` and writing `impl Trait for Type`. That is Rust's answer to the thing inheritance was for.

## Associated function vs method — one difference only

| | first parameter | called as |
|---|---|---|
| **associated function** | none | `Ballot::new("Ada")` |
| **method** | `self`, `&self`, or `&mut self` | `ballot.total()` |

That is the entire distinction. A method is not a special kind of item — it is an associated function whose first parameter happens to be the value itself, and `ballot.total()` is **sugar**:

```rust
ballot.total()  ==  Ballot::total(&ballot)   // the same call
```

The dot inserts the `&` for you. The example below asserts those two are equal, so you can watch the sugar dissolve.

So when a reference says an associated function *"can be standalone, meaning it would be called like `Foo::bar()`"* — that is what it means. There is no instance to hang the call on, so you name the **type** instead of a value. `Ballot::new` is the obvious case: it is the function that *makes* a `Ballot`, so it cannot take one.

And Rust has no constructor syntax at all. `new` is an ordinary associated function returning `Self`; the name is a convention the standard library follows and nothing more. Nothing stops you writing `Ballot::blank()` or `Ballot::from_csv(..)`, and plenty of good APIs do.

## `Self` and `self` are different words

- **`Self`** (capital) is the **type** — inside `impl Ballot`, it is another spelling of `Ballot`. Useful in return position, and it keeps working if the type gets renamed.
- **`self`** (lowercase) is the **value** — the instance the method was called on.

`fn new(..) -> Self` returns a `Ballot`. `fn total(&self)` borrows one.

## The three receivers, and what each costs the caller

This is the choice you make on every method, and getting it wrong is what most early borrow-checker pain actually is:

| Receiver | The method may | The caller | Reach for it when |
|---|---|---|---|
| `&self` | read | keeps the value | the method asks a question |
| `&mut self` | read and change | keeps the value, changed | the method updates it in place |
| `self` | do anything, including destroy it | **loses the value** | the operation ends the value's life |

`self` is not an exotic case. `into_receipt`, `into_iter`, `certify` — anything whose name starts `into_` is usually taking `self`, and the value being unusable afterwards is the *feature*: a certified tally cannot be voted into, because it no longer exists.

Two errors come straight out of this table, and both name the fix:

```text
error[E0596]: cannot borrow `t` as mutable, as it is not declared as mutable
error[E0382]: borrow of moved value: `t`
```

The first is a `&mut self` method called through a binding that is not `mut` — the *method signature* is what demands it. The second is using a value after a method took `self`.

## Inherent impl vs trait impl

```rust
impl Ballot          { fn total(&self) -> u32 { … } }        // inherent
impl Summary for Ballot { fn one_line(&self) -> String { … } } // trait
```

Same syntax plus `for`. The difference is **who chose the signature**: an inherent impl is yours alone, while a trait impl fills in a shape someone else declared — which is what lets a function accept "any type that can summarise itself" rather than one concrete type.

A trait may ship a **default method** body, which every implementor gets free and any implementor may override. That is the closest thing Rust has to inheriting an implementation, and note what is missing: no base class, no `super`, and no way for the default to reach into a field.

One practical gotcha the example does not show: a trait's methods are only callable where the **trait is in scope**. A mysterious *"method not found"* on a type you know has the method is very often a missing `use`.

## Coming from another language

- **Python.** A `class` body holds fields *and* `def`s; Rust splits those into `struct` and `impl`. `self` is explicit in both — Python's `def total(self)` and Rust's `fn total(&self)` line up almost exactly, and `Ballot::new` is close to a `@classmethod` or `@staticmethod`. What has no equivalent is the receiver *choice*: Python has one `self`, Rust has three, and picking among them is the design work.
- **ABAP.** An `impl` block is doing the job `CLASS … IMPLEMENTATION` does, but attached to a plain structure type rather than to a class — so you get methods on data without needing an object at all. `Ballot::new` is a static method (`CLASS-METHODS`), `ballot.total()` an instance method, and a `trait` is close to an `INTERFACE` — with the large difference that you can implement one for a type you did not write.

---

## Practice

**Pick the right receiver four times, then break two of them on purpose.** Model a `Tally { contest: String, counts: Vec<u32> }` with four operations — create one, ask who is leading, record a vote, and certify the result — and give each the receiver it deserves before you write any bodies. One takes no `self`, one takes `&self`, one `&mut self`, and one takes `self`. Justify the last one in a sentence: what does consuming the tally *prevent*?

Then make each of these happen and read the error:

1. Call the recording method through a binding declared without `mut` — `E0596`. Note that nothing in the *call* is wrong; it is the method's signature reaching back out to the caller.
2. Use the tally again after certifying it — `E0382`.

Finally, make `leader()` answer `None` rather than `Some(0)` on a tally where nobody has voted, and say which of the two is the bug you would rather ship.

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

- [STRUCTS.md](../../STRUCTS.md) — the map: every struct lesson in reading order
- [What a struct is](../what_a_struct_is/README.md) — the data half, and the three flavors
- [Ownership and moves](../ownership_and_moves/README.md) — what `self` as a receiver actually does
- [Borrowing](../borrowing/README.md) — why `&self` and `&mut self` follow different rules
- [A score is not a number: the newtype](../newtype_score/README.md) — an `impl` block whose job is to guard one door
