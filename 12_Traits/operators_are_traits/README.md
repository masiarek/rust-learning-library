# Operators are traits

**Level:** 201 · working knowledge

**One line:** `a + b` is `Add::add(a, b)`, and every operator in the language is a trait you can implement — which is why a newtype can read exactly like the number it wraps while still refusing to be added to a different unit.

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Points(i32);

impl Add for Points {
    type Output = Points;
    fn add(self, other: Points) -> Points { Points(self.0 + other.0) }
}

fn main() {
    println!("{:?}", Points(5) + Points(3));   // Points(8)
}
```

## `type Output` is why `+` can change the type

`Add`'s signature is `fn add(self, rhs: Rhs) -> Self::Output`, so **none of the three types has to match**. `Instant + Duration` is an `Instant`; `Instant - Instant` is a `Duration`; `String + &str` is a `String`. Picking `Output` is a design decision, not a formality.

## Each operator is one trait for one pair of types

| Want | Need |
|---|---|
| `p * 3` | `impl Mul<i32> for Points` |
| `3 * p` | `impl Mul<Points> for i32` — a **separate**, unrelated impl |
| `&p * 3` | `impl Mul<i32> for &Points` |
| `p += q` | `impl AddAssign for Points` — `Add` does not give it to you |
| `scores.sum()` | `impl Sum for Points` — `Add` does not give you that either |

That second row is legal even though it implements a foreign trait for a foreign type, because the orphan rule looks at the **whole impl**: a local type appears in it, as the parameter. It is the same rule that lets std write `impl Add<&str> for String`, and the same rule that refuses `impl Mul<i32> for f64` from your crate.

The verbosity is the feature. `Points + Seats` cannot compile by accident, because nobody wrote that impl.

## Indexing returns a reference

```rust
impl Index<usize> for Ballot {
    type Output = Points;
    fn index(&self, i: usize) -> &Points { &self.scores[i] }
}
```

`ballot[1]` is sugar for `*ballot.index(1)`, and `index` returns `&Self::Output`. That is why [`HashMap`](../../26_Collections/the_hashmap/README.md)'s `[]` panics on a missing key rather than returning `Option` — the trait has nowhere to put a `None`.

## The trap: the operator that hides a question

```rust
let morning = Turnout(0.80);
let evening = Turnout(0.40);
// morning + evening — but what is it?
```

Three plausible answers: `1.20` (nonsense), `0.60` (the mean), and `0.44` (weighted by how many voted in each session — the only correct one). The correct answer needs an argument the operator has nowhere to put.

So `Turnout` gets no `Add`. It gets `combine(other, voters)`, whose name and signature say what it does and force the caller to supply what is missing.

**The test:** *would every reader guess the same answer without reading the impl?*

| | |
|---|---|
| `Points + Points` | yes — it is a count |
| `PathBuf / &str` | yes, and std does exactly that |
| `Turnout + Turnout` | no — three readings |
| `Ballot + Ballot` | no — merge? concatenate? refuse? |

When the answer is no, a named method is not a worse API. It is the honest one.

## What cannot be overloaded

`&&` and `||` (they short-circuit, and a trait method cannot), `=`, `.`, and `?` — which does have traits (`Try`, `FromResidual`) but they are unstable. Everything else — `+ - * / % & | ^ << >> !`, `[]`, `()`, `== != < >` and the nine assigning forms — is a trait in [`std::ops` ↗](https://doc.rust-lang.org/std/ops/index.html) or [`std::cmp` ↗](https://doc.rust-lang.org/std/cmp/index.html).

## If you are coming from another language

- **Python.** `__add__`, `__mul__`, `__getitem__` — the same design, and the same advice about not surprising your reader. Two differences that matter in practice. Python's `__radd__` is the reflected fallback the interpreter tries when the left operand refuses; Rust has no fallback, so `3 * p` genuinely requires the second impl, and forgetting it is a compile error rather than a `TypeError` at run time. And Python's `NotImplemented` return value has no counterpart: an impl either exists or does not, decided when the crate compiles. `__iadd__` is `AddAssign`, with the same in-place-versus-rebind distinction and the same reason it exists.
- **ABAP.** Operators cannot be overloaded at all, so every one of these is a method call already — `lo_a->add( lo_b )` — and the question this page asks does not arise. What transfers is the diagnosis rather than the syntax: the reason ABAP's arithmetic on two `TYPE p` amounts is safe and on two "percentages held in a `p`" is not is exactly the `Turnout` trap, and the fix is the same one — a class with a named method that takes the weighting, instead of an amount you can add to anything. Rust just gives you the option of the operator once the meaning really is unambiguous.
- **C++.** `operator+` and friends, including the free-function form that is C++'s answer to the `3 * p` problem — `impl Mul<Points> for i32` is `Points operator*(int, Points)`. Rust's version cannot be a hidden conversion, since there are no implicit conversions to trip over.
- **Java.** No operator overloading, which is why `BigDecimal.add(other)` reads the way it does. If you have written that code, you have already met both sides of this page's argument.

---

## The verified output

<!-- output:operators_are_traits -->
*Verified output of [`operators_are_traits.rs`](examples/operators_are_traits.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The operator is the method
   a + b        = 8pt   is Add::add(a, b)
   Add::add(a, b) = 8pt   the same call, written out
   a - b        = 2pt
   -a           = -5pt
   a * 3        = 15pt   Mul<i32>, a DIFFERENT trait from Mul<Points>

2. `type Output` is why `+` can change the type
   Add's signature is `fn add(self, rhs: Rhs) -> Self::Output`, so
   none of the three types has to match. `Instant + Duration` is an
   Instant, `Instant - Instant` is a DURATION, and `String + &str` is
   a String. Here Output is Points because that is what made sense,
   not because the trait required it.

3. The assigning forms are separate traits
   after three `+=`: 12pt
   `+=` is AddAssign, and implementing Add does NOT give it to you.
   The difference is real: add_assign takes &mut self and can mutate
   in place, where add consumes and returns. For a Copy newtype that
   is a formality; for a String or a Vec it is an allocation saved.

4. Indexing, and what it returns
   ballot[1] = 3pt
   Index::index returns a REFERENCE — `&Self::Output` — and `[]` is
   sugar for `*ballot.index(1)`. That is why a HashMap's `[]` panics
   rather than returning Option: the trait has nowhere to put a None.

5. What the operators buy, and the trap
   fold with + : 12pt
   sorted (Ord): [Points(3), Points(4), Points(5)]
   A newtype with the right operators reads exactly like the number
   it wraps, while still refusing `Points + Seats`. That is the
   whole argument for the newtype: the type is a unit.
   The trap is the other direction — implementing Add on a type whose
   addition is not obvious. `Ballot + Ballot` could be a merge, a
   concatenation, or an error, and the operator hides the question.
   If the answer is not the one every reader would guess, write a
   method with a name.

6. What cannot be overloaded
   && and ||   they short-circuit; a trait method cannot
   =           assignment is not a trait
   .           field access and method calls (Deref is not this)
   ?           it has its own traits, Try and FromResidual, unstable
   Everything else — + - * / % & | ^ << >> ! [] () == < > and the
   nine assigning forms — is a trait in std::ops or std::cmp.
```
<!-- /output -->

## Practice

**Four impls for one `*`, and an operator that should not exist.** Give a `Points` newtype four impls so that `p * 3`, `3 * p`, `&p * 3` and `p + q` all compile. One of the four implements a foreign trait for a foreign type — say why the compiler allows it.

Then take a `Turnout(f64)` and write down three different plausible meanings of `morning + evening`. Compute all three. Implement none of them, write the method that takes what the operator could not, and state the one-line test you would apply next time.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:operators_are_traits_kata -->
*[`operators_are_traits_kata.rs`](examples/operators_are_traits_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: four impls for one `*`, and an operator that should not exist.
//!
//!   rustc --edition 2024 operators_are_traits_kata.rs -o /tmp/opk && /tmp/opk

use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Points(i32);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Turnout(f64);

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}pt", self.0)
    }
}

// 1. Points * i32
impl Mul<i32> for Points {
    type Output = Points;
    fn mul(self, n: i32) -> Points {
        Points(self.0 * n)
    }
}

// 2. i32 * Points — a SEPARATE impl, and legal only because Points is local.
impl Mul<Points> for i32 {
    type Output = Points;
    fn mul(self, p: Points) -> Points {
        Points(self * p.0)
    }
}

// 3. &Points * i32, so a borrowed value works without a deref at the call site.
impl Mul<i32> for &Points {
    type Output = Points;
    fn mul(self, n: i32) -> Points {
        Points(self.0 * n)
    }
}

// 4. Points + Points, the ordinary one.
impl Add for Points {
    type Output = Points;
    fn add(self, other: Points) -> Points {
        Points(self.0 + other.0)
    }
}

/// Deliberately NOT `impl Add for Turnout` — see section 3.
impl Turnout {
    fn combine(self, other: Turnout, voters: (f64, f64)) -> Turnout {
        let (a, b) = voters;
        Turnout((self.0 * a + other.0 * b) / (a + b))
    }
}

fn main() {
    println!("1. Four impls, and what each one is for");
    let p = Points(5);
    println!("   Points * i32   : {}", p * 3);
    println!("   i32 * Points   : {}", 3 * p);
    println!("   &Points * i32  : {}", &p * 3);
    println!("   Points + Points: {}", p + Points(3));
    println!("   `3 * p` is NOT free once you have written `p * 3`. Mul is generic");
    println!("   over its right-hand type, not symmetric: `impl Mul<i32> for Points`");
    println!("   and `impl Mul<Points> for i32` are two unrelated impls.");

    println!();
    println!("2. Why the second one is allowed at all");
    println!("   `impl Mul<Points> for i32` implements a foreign trait for a");
    println!("   FOREIGN type — and it compiles, because the orphan rule looks at");
    println!("   the whole impl: a local type appears in it, as the parameter.");
    println!("   That is why std can write `impl Add<&str> for String` and why you");
    println!("   can write this one, and also why `impl Mul<i32> for f64` is");
    println!("   refused: nothing in it is yours.");

    println!();
    println!("3. The operator that should not exist");
    let morning = Turnout(0.80);
    let evening = Turnout(0.40);
    println!("   morning {:.0}%, evening {:.0}%", morning.0 * 100.0, evening.0 * 100.0);
    println!("   a naive `+`      would give {:.0}%", (morning.0 + evening.0) * 100.0);
    println!("   the mean         would give {:.0}%", (morning.0 + evening.0) / 2.0 * 100.0);
    let combined = morning.combine(evening, (100.0, 900.0));
    println!("   weighted by size : {:.0}%   <- the only right answer", combined.0 * 100.0);
    println!("   Three plausible readings of `morning + evening`, and the correct");
    println!("   one needs an argument the operator has nowhere to put. So Turnout");
    println!("   has no Add: it has `combine(other, voters)`, whose name and");
    println!("   signature say what it does. An operator hides the question, and");
    println!("   the reader never learns there was one.");

    println!();
    println!("4. The test for whether to implement an operator");
    println!("   Would every reader guess the same answer without reading the impl?");
    println!("   Points + Points     yes — it is a count");
    println!("   Turnout + Turnout   no  — three readings, above");
    println!("   Ballot + Ballot     no  — merge? concatenate? refuse?");
    println!("   Path / &str         yes, and std does exactly that for PathBuf");
    println!("   When the answer is no, a named method is not a worse API. It is");
    println!("   the honest one.");

    println!();
    println!("5. What you still get for free, and what you do not");
    let scores = [Points(5), Points(3), Points(4)];
    let total: Points = scores.iter().copied().fold(Points(0), |a, b| a + b);
    println!("   fold with `+`  : {total}");
    println!("   `.sum()`       : needs `impl Sum for Points` — Iterator::sum is a");
    println!("                    trait method with its own bound, and Add alone");
    println!("                    does not satisfy it.");
    println!("   `+=`           : needs AddAssign, separately.");
    println!("   `Points * 3.0` : needs Mul<f64>, separately.");
    println!("   Each operator is one trait for one pair of types. That is verbose,");
    println!("   and it is why `Points + Seats` cannot compile by accident.");
}
```
<!-- /source -->

<!-- output:operators_are_traits_kata -->
*Verified output of [`operators_are_traits_kata.rs`](examples/operators_are_traits_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Four impls, and what each one is for
   Points * i32   : 15pt
   i32 * Points   : 15pt
   &Points * i32  : 15pt
   Points + Points: 8pt
   `3 * p` is NOT free once you have written `p * 3`. Mul is generic
   over its right-hand type, not symmetric: `impl Mul<i32> for Points`
   and `impl Mul<Points> for i32` are two unrelated impls.

2. Why the second one is allowed at all
   `impl Mul<Points> for i32` implements a foreign trait for a
   FOREIGN type — and it compiles, because the orphan rule looks at
   the whole impl: a local type appears in it, as the parameter.
   That is why std can write `impl Add<&str> for String` and why you
   can write this one, and also why `impl Mul<i32> for f64` is
   refused: nothing in it is yours.

3. The operator that should not exist
   morning 80%, evening 40%
   a naive `+`      would give 120%
   the mean         would give 60%
   weighted by size : 44%   <- the only right answer
   Three plausible readings of `morning + evening`, and the correct
   one needs an argument the operator has nowhere to put. So Turnout
   has no Add: it has `combine(other, voters)`, whose name and
   signature say what it does. An operator hides the question, and
   the reader never learns there was one.

4. The test for whether to implement an operator
   Would every reader guess the same answer without reading the impl?
   Points + Points     yes — it is a count
   Turnout + Turnout   no  — three readings, above
   Ballot + Ballot     no  — merge? concatenate? refuse?
   Path / &str         yes, and std does exactly that for PathBuf
   When the answer is no, a named method is not a worse API. It is
   the honest one.

5. What you still get for free, and what you do not
   fold with `+`  : 12pt
   `.sum()`       : needs `impl Sum for Points` — Iterator::sum is a
                    trait method with its own bound, and Add alone
                    does not satisfy it.
   `+=`           : needs AddAssign, separately.
   `Points * 3.0` : needs Mul<f64>, separately.
   Each operator is one trait for one pair of types. That is verbose,
   and it is why `Points + Seats` cannot compile by accident.
```
<!-- /output -->

</details>

---

## See also

- [What a trait is](../what_a_trait_is/README.md) — the declaration these are all instances of
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the newtype that earns these impls
- [`From` and `Into`](../../29_Conversion/from_and_into/README.md) — the orphan rule, stated in full
- [`HashMap`](../../26_Collections/the_hashmap/README.md) — where `Index` returning a reference becomes visible
- [Debug and Display](../../15_First_Programs/debug_vs_display/README.md) — `{}` is a trait call too
- [Where the bound goes](../../22_Generics/where_the_bound_goes/README.md) — writing a function generic over `T: Add<Output = T>`

## Sources

[Operator Overloading ↗](https://doc.rust-lang.org/rust-by-example/trait/ops.html) in Rust by Example, and [`std::ops` ↗](https://doc.rust-lang.org/std/ops/index.html), whose module page lists every operator and the trait behind it.
