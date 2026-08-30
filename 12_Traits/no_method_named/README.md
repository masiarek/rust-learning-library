# "No method named …"

**Level:** 201 · working knowledge

**One line:** `E0599` is one error code over three unrelated mistakes — the method was never written, the trait is not imported, or the trait is not implemented — and the `help:` line underneath is what tells them apart.

```rust
use std::io::BufRead; // 2. IMPORTED: `.lines()` on a byte slice lives on this trait

#[derive(Clone)] // 3. IMPLEMENTED: `.clone()` is in the prelude; the impl is not
struct Ballot { star: u8 }

impl Ballot { // 1. WRITTEN: an inherent method, reachable with no import at all
    fn doubled(&self) -> u8 { self.star * 2 }
}

fn main() {
    let b = Ballot { star: 5 };
    println!("{}", b.doubled());                       // 10
    println!("{}", b.clone().star);                    // 5
    println!("{}", "a\nb".as_bytes().lines().count()); // 2
}
```

Delete any one of the three marked lines and the call under it stops compiling. Same code every time, different fix each time.

## Which one you have

| The `help:` line | What is wrong | Fix |
|---|---|---|
| *none — only "method not found for this struct"* | Nothing anywhere defines it | Write the method |
| items from traits can only be used if the trait **is in scope** | The impl exists; the trait is not imported | [`use` the trait](../trait_in_scope/README.md) |
| …if the trait **is implemented and** in scope | The trait is in scope; your type does not implement it | `impl` or `derive` it |
| the following trait **bounds were not satisfied** | The method comes from a blanket impl whose bound your type misses | Implement the *bound* |

Four words separate the middle two rows, and they point at opposite fixes: adding a `use` for a trait you never implemented does nothing, and writing an `impl` for a trait already implemented and merely unimported does nothing either.

## 1. The method was never written

```rust
struct Ballot {
    star: u8,
}

fn main() {
    let b = Ballot { star: 5 };
    // println!("{}", b.doubled());   // E0599 — nothing has ever defined `doubled`
}
```

```text
error[E0599]: no method named `doubled` found for struct `Ballot` in the current scope
 --> ballot.rs:7:22
  |
1 | struct Ballot {
  | ------------- method `doubled` not found for this struct
...
7 |     println!("{}", b.doubled());
  |                      ^^^^^^^ method not found in `Ballot`
```

**No `help:` line at all** — that absence is the diagnosis. rustc looked for something to suggest, found no trait anywhere in the crate graph offering a `doubled`, and had nothing to say beyond the fact. Write the `impl`, or fix the typo.

## 2. The trait is implemented but not imported

```rust
fn main() {
    let text = "first\nsecond".as_bytes();
    // for line in text.lines() { }   // E0599 until `std::io::BufRead` is imported
}
```

```text
error[E0599]: no method named `lines` found for reference `&[u8]` in the current scope
 --> reader.rs:3:22
  |
3 |     for line in text.lines() {
  |                      ^^^^^ method not found in `&[u8]`
  |
  = help: items from traits can only be used if the trait is in scope
help: trait `BufRead` which provides `lines` is implemented but not in scope; perhaps you want to import it
  |
1 + use std::io::BufRead;
  |
```

**"is implemented but not in scope"** names the trait and writes the line for you. `&[u8]` has been a `BufRead` the whole time; the method was unreachable, not absent. Why method resolution works this way, and the three spellings of the call once it does, are [a page of their own](../trait_in_scope/README.md).

One trap this hides: `"a\nb".lines()` compiles without any import, because `str` has an **inherent** `lines` of its own. The same call on `.as_bytes()` needs the trait. Two methods, one name, and the receiver type decides which of the two problems you have.

## 3. The trait is in scope but not implemented

```rust
#[derive(Debug)] // no Clone
struct Tally {
    counted: usize,
}

fn main() {
    let t = Tally { counted: 3 };
    // let copy = t.clone();   // E0599 — Clone is in the prelude, the impl is missing
}
```

```text
error[E0599]: no method named `clone` found for struct `Tally` in the current scope
 --> tally.rs:8:18
  |
2 | struct Tally {
  | ------------ method `clone` not found for this struct
...
8 |     let copy = t.clone();
  |                  ^^^^^ method not found in `Tally`
  |
  = help: items from traits can only be used if the trait is implemented and in scope
  = note: the following trait defines an item `clone`, perhaps you need to implement it:
          candidate #1: `Clone`
```

**"implemented and in scope"**, plus a `candidate #1` naming the trait to write. For `Clone` the import half is already done — it is one of the [prelude ↗](https://doc.rust-lang.org/std/prelude/index.html) traits — so cause 2 is ruled out before you start and `#[derive(Clone)]` is the whole fix.

`.clone()` on a type that does not implement it is the single most common way to meet `E0599`, which is why the derive is worth reaching for by reflex. What that derive costs, and when [`Copy`](../../16_Structs/copy_vs_clone/README.md) is the better answer, is the neighbouring lesson.

## 4. The bound behind the method is not satisfied

```rust
struct Report {
    winner: &'static str,
}

fn main() {
    let r = Report { winner: "Ada" };
    // println!("{}", r.to_string());   // E0599 — but the headline is not "no method named"
}
```

```text title="Abridged — real rustc output for report.rs, with the std source location dropped"
error[E0599]: `Report` doesn't implement `std::fmt::Display`
    --> report.rs:7:22
     |
   1 | struct Report {
     | ------------- method `to_string` not found for this struct because it doesn't satisfy `Report: ToString` or `Report: std::fmt::Display`
...
   7 |     println!("{}", r.to_string());
     |                      ^^^^^^^^^ method cannot be called on `Report` due to unsatisfied trait bounds
     |
     = note: the following trait bounds were not satisfied:
             `Report: std::fmt::Display`
             which is required by `Report: ToString`
```

The same code, and it does not say *no method named* anywhere in the headline. `to_string` is nobody's hand-written method: std has one blanket impl giving `ToString` to every `T: Display`, so the method you asked for exists for a whole family of types and yours is outside it. Implementing `ToString` directly is the wrong fix and the error never suggests it — write `Display` and the method appears.

This is the shape worth recognising, because the trait you must implement is **not the trait the method came from**. `which is required by` is the line that says so.

## Why one code covers all four

A method call resolves against a candidate list: the inherent methods of the receiver's type, then the methods of every trait **currently in scope** whose bounds the type satisfies. Anything not on that list does not exist as far as the dot is concerned. `E0599` is the single "the list was empty" outcome, and the four causes above are four different reasons a candidate never made it onto the list.

Two things follow. The error is about **the call site's imports**, not about your type — which is why the same struct compiles in one module and not in its neighbour. And `E0599` never means *found but forbidden*: a method that exists and is private is [`E0624` ↗](https://doc.rust-lang.org/error_codes/E0624.html), a separate code with a separate message.

`rustc --explain E0599` prints the same writeup as [the error index ↗](https://doc.rust-lang.org/error_codes/E0599.html), offline, from the toolchain you are compiling with. It covers cause 1 only.

## If you are coming from another language

**Python.** The same mistake is `AttributeError: 'Ballot' object has no attribute 'doubled'`, and the difference that matters is *when*: Python asks the question when the line runs, so a method missing on a branch nobody took is a bug that ships. Rust asks at compile time, for every branch.

Then the causes stop lining up. Python has no counterpart to cause 2 at all — there is no import that makes an existing method reachable, because behaviour arrives by inheritance and an inherited method is simply there. The nearest analogue is the mixin you forgot to put in the bases list, and that is really Rust's cause 3. Cause 4 has no Python form either: `str(obj)` always works, falling back to `<__main__.Report object at 0x…>` rather than refusing, so the missing `__str__` is a silent wrong answer instead of an error.

And a Python answer can change after the fact — `Ballot.doubled = lambda self: self.star * 2` patches the class at runtime, and the call three lines later succeeds. Rust's candidate list is fixed when the file is compiled; nothing can add to it later. What transfers is the triage habit. What changed is that you now get the verdict before the program runs, and one of the three questions is one Python never asks.

**ABAP.** The closest message is a syntax-check error at activation — `Method "DOUBLED" is unknown or PROTECTED or PRIVATE` — so like Rust, you hear about it before anything runs, which is already a better starting point than Python's.

Interfaces map onto cause 3 cleanly. `lo_ballot->if_shout~shout( )` needs the class to declare `INTERFACES if_shout` and implement the method; without that, the call does not compile. That is Rust's *implemented*. What ABAP has no rung for is *in scope*: a method the class implements is callable from anywhere the class is, and there is no import that can hide it. Rust's second cause is the genuinely new one, and it is the one that surprises — you can read the `impl` block ten lines above the failing call and still not be allowed to call it.

One more difference, in ABAP's favour and worth knowing so the messages do not mislead you: that ABAP sentence folds visibility into the same error — *unknown **or** PROTECTED or PRIVATE*. Rust splits them. `E0599` means not found; a method that exists and is out of reach is `E0624`, with `private method` written on the span. So in Rust, unlike ABAP, "no method named" never quietly means "there is one, you just may not have it".

## The verified output

<!-- output:no_method_named -->
*Verified output of [`no_method_named.rs`](examples/no_method_named.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The method EXISTS because an inherent impl wrote it
   b.doubled() = 10

2. The trait is IN SCOPE, so its method appears on a type std wrote
   text.lines().count() = 3
   collected            = ["first", "second", "third"]

3. The trait is IMPLEMENTED, so .clone() exists on our own type
   original = Ballot { voter: "Ada", star: 5 }
   clone    = Ballot { voter: "Ada", star: 0 }

4. The BOUND is satisfied, so the blanket impl hands us .to_string()
   b.to_string() = Ada gave 5 star(s)
   ^ nothing here implements ToString; implementing Display did it.
```
<!-- /output -->

## Practice

**Three calls, three causes.** Each line below fails with `E0599` and each needs a different fix — one of them is not fixed by an `impl` of the trait the method came from. Name the cause before you reach for the fix; the `help:` line is the whole exercise.

```rust
struct Precinct { ballots: u32, registered: u32 }
struct Station { id: u32, town: &'static str }

fn main() {
    let p = Precinct { ballots: 812, registered: 1_000 };
    // println!("{:.1}%", p.turnout());

    let mut log = String::new();
    // log.write_str("provisional").unwrap();

    let s = Station { id: 7, town: "Ada Falls" };
    // println!("{}", s.to_string());
}
```

Uncomment one at a time and read what rustc says before changing anything. Two of the three fixes are one line.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:no_method_named_kata -->
*[`no_method_named_kata.rs`](examples/no_method_named_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three calls that all failed with `E0599`, three different
//! fixes. The cause is named beside each one.
//!
//!   rustc --edition 2024 no_method_named_kata.rs -o /tmp/nmnk && /tmp/nmnk

use std::fmt;
use std::fmt::Write; // FIX 2 — trait not in scope. `String` already had
                     // `write_str`; nothing could reach it without this line.

struct Precinct {
    ballots: u32,
    registered: u32,
}

// FIX 1 — the method did not exist. No import could have helped: nothing had
// ever written it.
impl Precinct {
    fn turnout(&self) -> f64 {
        (self.ballots as f64 / self.registered as f64) * 100.0
    }
}

struct Station {
    id: u32,
    town: &'static str,
}

// FIX 3 — the bound was not satisfied. `to_string` is never implemented by
// hand; it arrives for any `T: Display`, so `Display` is the missing piece.
impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "station {} ({})", self.id, self.town)
    }
}

fn main() {
    let p = Precinct { ballots: 812, registered: 1_000 };
    println!("1. inherent method written  -> p.turnout() = {:.1}%", p.turnout());

    let mut log = String::new();
    write!(log, "{} of {} voted", p.ballots, p.registered).unwrap();
    log.write_str(" — provisional")
        .expect("writing to a String cannot fail");
    println!("2. std::fmt::Write imported -> log = {log:?}");

    let s = Station { id: 7, town: "Ada Falls" };
    println!("3. Display implemented      -> s.to_string() = {:?}", s.to_string());

    println!();
    println!("The three causes, in the order the compiler rules them out:");
    println!("  1. no such method anywhere               -> write it");
    println!("  2. trait implemented, but not in scope   -> import it");
    println!("  3. trait (or its bound) not implemented  -> implement it");
}
```
<!-- /source -->

<!-- output:no_method_named_kata -->
*Verified output of [`no_method_named_kata.rs`](examples/no_method_named_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. inherent method written  -> p.turnout() = 81.2%
2. std::fmt::Write imported -> log = "812 of 1000 voted — provisional"
3. Display implemented      -> s.to_string() = "station 7 (Ada Falls)"

The three causes, in the order the compiler rules them out:
  1. no such method anywhere               -> write it
  2. trait implemented, but not in scope   -> import it
  3. trait (or its bound) not implemented  -> implement it
```
<!-- /output -->

</details>

## See also

- [A trait must be in scope](../trait_in_scope/README.md) — cause 2 in full: why resolution works this way, and the fully-qualified spelling that reaches past an inherent method of the same name
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — cause 3's most common instance, and when the derive is the wrong answer
- [What a trait is](../what_a_trait_is/README.md) — the declaration all of this is reaching into
- [Making a `String`](../../14_Strings/making_a_string/README.md) — `.to_string()` from the other side, once `Display` is in place
- [Reading a compilation failure](../../20_Compilers/reading_a_compilation_failure/README.md) — which of four programs produced the message you are holding
