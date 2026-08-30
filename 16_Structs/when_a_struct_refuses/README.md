# When a struct refuses

**Level:** 101 → 201 · working knowledge

**One line:** Eight refusals you will meet in your first week of writing structs — and the useful surprise is that four of them are the *same* error code meaning four unrelated things, so the code is never the diagnosis.

---

## Read the `note:`, not the first line

Every error below prints its own fix. Seven of the eight include the exact edit; the eighth names both conflicting sites. What people do instead is read the red line, recognise the code, and guess — which works right up until `E0277` shows up, because **`E0277` is just "a trait bound was not satisfied"** and for structs that covers at least four different mistakes.

| Code | Reads as | Actually about |
|---|---|---|
| [`E0063` ↗](https://doc.rust-lang.org/error_codes/E0063.html) | missing field | there is no partly-built struct |
| [`E0277` ↗](https://doc.rust-lang.org/error_codes/E0277.html) | trait bound not satisfied | **no `Display`** — you have to write it |
| `E0277` | trait bound not satisfied | **no `Debug`** — but this one it will generate |
| `E0277` | trait bound not satisfied | **a `str` field** — a missing *size*, not a missing impl |
| `E0277` | trait bound not satisfied | **`Eq` without `PartialEq`** |
| [`E0119` ↗](https://doc.rust-lang.org/error_codes/E0119.html) | conflicting implementations | you both derived and hand-wrote a trait |
| [`E0594` ↗](https://doc.rust-lang.org/error_codes/E0594.html) | not declared as mutable | `mut` belongs to the binding, and you made a new one |
| [`E0282` ↗](https://doc.rust-lang.org/error_codes/E0282.html) | type annotations needed | a function in `impl<T>` that never mentions `T` |

## E0063 — there is no partly-built struct

```text title="Real rustc output"
error[E0063]: missing field `voter` in initializer of `Ballot`
 --> e1.rs:2:22
  |
2 | fn main() { let _b = Ballot { score: 5 }; }
  |                      ^^^^^^ missing `voter`
```

Rust has no uninitialized state for a struct — the moment one exists, every field has a value. So name them all, or hand the job to something that can: `..Default::default()` fills the rest. Watch what that gives you, though: a derived `Default` is **the type's zero, not your domain's**, so a missing name becomes `""` rather than an error.

## E0277, four ways

**No `Display`, from `{}`.** Rust will not guess how a human should read your type.

```text title="Real rustc output"
error[E0277]: `Ballot` doesn't implement `std::fmt::Display`
  |                       --   ^^^^^^^^^^^^^^^^^^^ `Ballot` cannot be formatted with the default formatter
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

**No `Debug`, from `{:?}`.** Same code, opposite advice — this one it offers to generate:

```text title="Real rustc output"
error[E0277]: `Ballot` doesn't implement `Debug`
  = note: add `#[derive(Debug)]` to `Ballot` or manually `impl Debug for Ballot`
```

Which of the two you want, and why the language refuses to guess the first, is [Debug and Display](../../15_First_Programs/debug_vs_display/README.md).

**A `str` field.** This one is misfiled by the code: it is not a missing impl at all, it is a missing *size*.

```text title="Real rustc output"
error[E0277]: the size for values of type `str` cannot be known at compilation time
  |                        ^^^ doesn't have a size known at compile-time
  = help: the trait `Sized` is not implemented for `str`
  = note: only the last field of a struct may have a dynamically sized type
help: borrowed types always have a statically known size
  |                        &str
help: the `Box` type always has a statically known size and allocates its contents in the heap
  |                        Box<str>
```

`str` is the text itself, of unknown length; `&str` is a *reference* to it and `String`/`Box<str>` own it on the heap. All three of those have a size. The `note:` is the part worth keeping: a dynamically sized field is legal, but **only in last position**.

**`#[derive(Eq)]` on its own.**

```text title="Real rustc output"
error[E0277]: can't compare `Ballot` with `Ballot`
    |        ^^^^^^ no implementation for `Ballot == Ballot`
    = help: the trait `PartialEq` is not implemented for `Ballot`
note: required by a bound in `Eq`
help: consider annotating `Ballot` with `#[derive(PartialEq)]`
```

`Eq` adds no methods. It is a *promise about* `PartialEq` — that `==` is reflexive, so `a == a` always holds. That promise needs the `==` it is about, which is why the pair is always derived together. (Floats are the reason the split exists: `f64` has `PartialEq` and not `Eq`, because `NaN != NaN`.)

## E0119 — derived and hand-written are two impls

```text title="Real rustc output"
error[E0119]: conflicting implementations of trait `Default` for type `Ballot`
1 | #[derive(Default)]
  |          ^^^^^^^ conflicting implementation for `Ballot`
3 | impl Default for Ballot { … }
  | ----------------------- first implementation here
```

A derive *writes an impl*. Two impls of one trait for one type is the coherence rule saying no. Keep whichever knows more — usually the hand-written one, since a derived `Default` can only ever produce the type's zero, while yours can encode a real default like a quorum of ten.

## E0594 — `mut` belongs to the binding

```text title="Real rustc output"
error[E0594]: cannot assign to `b.score`, as `b` is not declared as mutable
6 |     b.score = 6;
  |     ^^^^^^^^^^^ cannot assign
help: consider changing this to be mutable
5 |     let mut b = b;
```

Note where the `help:` points — **line 5, not line 6**. The assignment is fine; the rebinding on the line before dropped the `mut`. `let b = b;` is a deliberate idiom for freezing a value after its setup phase, and this error is that idiom working. There is no such thing as a mutable *field*: [a name is not a place](../../18_Ownership/a_name_is_not_a_place/README.md).

## E0282 — nothing to infer `T` from

```text title="Real rustc output"
error[E0282]: type annotations needed
4 |     fn check() { let _q = Tally::quorum(); }
  |                           ^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the struct `Tally`
help: consider specifying a concrete type for the type parameter `T`
4 |     fn check() { let _q = Tally::</* Type */>::quorum(); }
```

`quorum()` takes no `T`, returns no `T`, and mentions no `T` — but it lives inside `impl<T> Tally<T>`, so calling it still requires choosing one, and there is nothing in the call to choose from. Three fixes, in order of preference: give it a receiver (`self.quorum()`), name the type (`Tally::<i32>::quorum()`), or — usually the right answer — **move it out of the generic impl**, because a function with no `T` in it never belonged there.

---

## Practice

**Seven errors, five root causes, three edits.** This arrives from a colleague and refuses seven times:

```rust
#[derive(Debug, Default, Eq)]
struct Voter { name: str, seat: u8 }
impl Default for Voter { fn default() -> Self { unimplemented!() } }
fn main() { let v = Voter { seat: 1 }; println!("{}", v); }
```

1. Read all seven before changing anything. Group them by *cause*, not by line.
2. Change `name: str` to `name: String` and recompile. How many errors went away? Why that many?
3. Resolve the `Default` conflict. Which of the two impls should survive, and what does the other one know that it doesn't?
4. Fix the last two. Which of them could rustc not have written for you, and why not?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:when_a_struct_refuses_kata -->
*[`when_a_struct_refuses_kata.rs`](examples/when_a_struct_refuses_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: seven errors, five root causes, three edits.
//!
//!   rustc --edition 2024 when_a_struct_refuses_kata.rs -o /tmp/wsrk && /tmp/wsrk

use std::fmt;

// The struct as it arrives is four lines and refuses seven times:
//
//     #[derive(Debug, Default, Eq)]
//     struct Voter { name: str, seat: u8 }
//     impl Default for Voter { fn default() -> Self { unimplemented!() } }
//     fn main() { let v = Voter { seat: 1 }; println!("{}", v); }
//
// Fixed, with a note on each edit:

#[derive(Debug, PartialEq, Eq)] // edit 2: Default dropped (it clashes), PartialEq added
struct Voter {
    name: String, // edit 1: str -> String
    seat: u8,
}

impl Default for Voter {
    fn default() -> Self {
        Voter { name: String::from("unregistered"), seat: 0 }
    }
}

impl fmt::Display for Voter {
    // edit 3: `{}` needs this written by hand
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "seat {} — {}", self.seat, self.name)
    }
}

fn main() {
    println!("The error COUNT is not the work count.\n");

    println!("  7 errors   as it arrives");
    println!("  4 errors   after edit 1: `name: str` -> `name: String`");
    println!("  2 errors   after edit 2: drop derived Default, add PartialEq");
    println!("  0 errors   after edit 3: impl Display, and name every field\n");

    println!("Edit 1 removed THREE of the seven, because one root cause produced them:");
    println!("  E0277  str doesn't have a size known at compile-time   (x2)");
    println!("  E0277  the trait bound `str: Default` is not satisfied");
    println!("An unsized field poisons every derive that has to touch it. Chasing the");
    println!("three separately would have been three investigations of one mistake.\n");

    println!("Edit 2 removed two more, and they were unrelated to each other:");
    println!("  E0119  conflicting implementations of `Default`  — derive AND impl");
    println!("  E0277  can't compare `Voter` with `Voter`        — Eq without PartialEq");
    println!("Keeping the hand-written Default is the right call: it knows a domain");
    println!("default the derive could never guess.");
    println!("  Voter::default() = {}", Voter::default());
    println!("  ...where the derive would have said name: \"\", seat: 0\n");

    println!("Edit 3 was the only one rustc could not write for you:");
    println!("  E0063  missing field `name`   — it named the field");
    println!("  E0277  doesn't implement Display — it suggested {{:?}} instead");
    println!("The suggestion is a real option, and often the right one. Choosing to");
    println!("write Display means you decided a human reads this type.");
    let v = Voter { name: String::from("Ada"), seat: 7 };
    println!("  Display: {v}");
    println!("  Debug:   {v:?}");
    println!("  and the pair now compares: {}", v == Voter { name: "Ada".into(), seat: 7 });

    println!("\nThe habit: read all seven before editing any of them, and group them by");
    println!("root cause. rustc reports every error it can reach in one pass — it is not");
    println!("a queue to be worked one at a time.");
}
```
<!-- /source -->

<!-- output:when_a_struct_refuses_kata -->
*Verified output of [`when_a_struct_refuses_kata.rs`](examples/when_a_struct_refuses_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The error COUNT is not the work count.

  7 errors   as it arrives
  4 errors   after edit 1: `name: str` -> `name: String`
  2 errors   after edit 2: drop derived Default, add PartialEq
  0 errors   after edit 3: impl Display, and name every field

Edit 1 removed THREE of the seven, because one root cause produced them:
  E0277  str doesn't have a size known at compile-time   (x2)
  E0277  the trait bound `str: Default` is not satisfied
An unsized field poisons every derive that has to touch it. Chasing the
three separately would have been three investigations of one mistake.

Edit 2 removed two more, and they were unrelated to each other:
  E0119  conflicting implementations of `Default`  — derive AND impl
  E0277  can't compare `Voter` with `Voter`        — Eq without PartialEq
Keeping the hand-written Default is the right call: it knows a domain
default the derive could never guess.
  Voter::default() = seat 0 — unregistered
  ...where the derive would have said name: "", seat: 0

Edit 3 was the only one rustc could not write for you:
  E0063  missing field `name`   — it named the field
  E0277  doesn't implement Display — it suggested {:?} instead
The suggestion is a real option, and often the right one. Choosing to
write Display means you decided a human reads this type.
  Display: seat 7 — Ada
  Debug:   Voter { name: "Ada", seat: 7 }
  and the pair now compares: true

The habit: read all seven before editing any of them, and group them by
root cause. rustc reports every error it can reach in one pass — it is not
a queue to be worked one at a time.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:when_a_struct_refuses -->
*Verified output of [`when_a_struct_refuses.rs`](examples/when_a_struct_refuses.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. E0063 — missing field in initializer
   There is no partly-built struct in Rust. Name every field, or let
   something else supply the rest:
   named all:            Ballot { voter: "Ada", score: 5 }   (voter "Ada", score 5)
   ..Default::default(): Ballot { voter: "", score: 5 }   <- and this needs Default to exist
   note the derived Default gave voter an EMPTY string, not a missing one:
   the type's zero, which is rarely your domain's. b.voter.is_empty() = true

2. E0277 — one code, four different problems
   E0277 is 'a trait bound was not satisfied'. For structs it shows up as:

   2a. no Display, from `{}`
       Rust will not guess how you want a human to read your type.
       Ada scored 5

   2b. no Debug, from `{:?}`
       Same code, opposite advice: this one it WILL generate.
       Seat(3)

   2c. `str` as a field type — not a missing impl, a missing SIZE
       str is unsized, so it cannot be a field. Borrow it or box it:
       &str  field: Ada   Box<str> field: Ada
       The note is the part to read: 'only the LAST field of a struct
       may have a dynamically sized type'.

   2d. `#[derive(Eq)]` alone — Eq is a promise about PartialEq
       Eq adds no methods. It says == is reflexive, so it needs the
       PartialEq that defines ==. Always derive the pair:
       Seat(3) == Seat(3) is true

3. E0119 — conflicting implementations
   #[derive(Default)] AND `impl Default` is two impls of one trait.
   Keep the one that knows something the other cannot:
   derived would be Quorum { needed: 0 }; ours says needed = 10

4. E0594 — cannot assign, not declared as mutable
   `let b = b;` rebinds. mut belongs to the BINDING, so the new one
   does not inherit it — and rustc's help points at line 5, not line 6.
   The value never changed; the name did.

5. E0282 — type annotations needed
   `Tally::quorum()` names no T anywhere, but it lives in impl<T>, so
   there is nothing to infer T from. Three fixes, in order of preference:
     Tally::<i32>::quorum() = 10   name it at the call
     t.rows()               = 3   or have a receiver to infer from
     ...or move it out of impl<T> entirely, if it truly has no T in it.

The pattern across all eight: rustc is not withholding a fix.
Seven of these print the exact edit, and the eighth (E0119) names both
conflicting sites. The skill is reading the `note:` line, not the first one.
```
<!-- /output -->

## See also

- [Debug and Display](../../15_First_Programs/debug_vs_display/README.md) — the two `E0277`s above, argued properly
- [What a struct is](../what_a_struct_is/README.md) · [STRUCTS.md](../../STRUCTS.md)
- [A name is not a place](../../18_Ownership/a_name_is_not_a_place/README.md) — why `E0594` points at the binding
- [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) — the same habit, applied to the messages that do *not* stop the build
