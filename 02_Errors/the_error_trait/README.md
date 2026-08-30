# What makes a type an error

**Level:** 201 · working knowledge

**One line:** `std::error::Error` asks for almost nothing — `Debug`, `Display`, and one optional method — and the whole quality of your error handling comes down to whether you bothered with that optional method.

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct QuorumNotMet { cast: u32, needed: u32 }

impl fmt::Display for QuorumNotMet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "quorum not met: {} ballots cast, {} needed", self.cast, self.needed)
    }
}

impl Error for QuorumNotMet {}

fn main() {
    let e = QuorumNotMet { cast: 41, needed: 60 };
    println!("{e}");     // quorum not met: 41 ballots cast, 60 needed
    println!("{e:?}");   // QuorumNotMet { cast: 41, needed: 60 }
}
```

`impl Error for QuorumNotMet {}` with an empty body is a real implementation, not a placeholder. Everything the trait needs was already there.

## The trait, in full

This is the whole stable surface, read from the standard library's own source:

```rust title="From the standard library's own source — core/src/error.rs"
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    // ...plus `provide`, which is nightly-only, and two deprecated methods
}
```

Two supertraits you must satisfy, one method with a default. `description` and `cause` are the deprecated pair — if you find them in an old answer online, that answer predates 2018.

## The two supertraits are two audiences

Not a formality. They are read by different people at different times:

| | who reads it | where it shows up |
|---|---|---|
| `Display` | a user of your program | your own messages, and every layer of an error chain |
| `Debug` | you | `unwrap`, `expect`, `{:?}`, and what the runtime prints for an `Err` out of `main` |

That last one surprises people: [`main` returning a `Result`](../main_returns_result/README.md) prints the **`Debug`** form, so the careful `Display` sentence you wrote is not the one your user sees unless you print it yourself. Which of the two you get where is [Debug and Display](../../15_First_Programs/debug_vs_display/README.md).

Writing only one of them is not a shortcut — it is the bound failing:

```text title="Abridged — real rustc output for no_display.rs, without the file-and-line headers"
error[E0277]: `TallyError` doesn't implement `std::fmt::Display`
  |
6 | impl Error for TallyError {}
  |                ^^^^^^^^^^ unsatisfied trait bound
  |
help: the trait `std::fmt::Display` is not implemented for `TallyError`
note: required by a bound in `std::error::Error`
  |
59 | pub trait Error: Debug + Display {
  |                          ^^^^^^^ required by this bound in `Error`
```

The note quotes the trait definition back at you, which is as clear as a missing-bound error gets.

## `source()` is the one that matters

The default is `None`, which means *"nothing caused this"*. Leave it there when that is true. Override it whenever your error wrapped another one, because it is the only thing that makes an error **chain**:

```rust
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
struct SheetUnreadable { path: String, cause: io::Error }

impl fmt::Display for SheetUnreadable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not read the tally sheet {}", self.path)
    }
}

impl Error for SheetUnreadable {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

fn main() {
    let e = SheetUnreadable {
        path: "ballots.txt".into(),
        cause: io::Error::new(io::ErrorKind::NotFound, "no such file or directory"),
    };
    println!("{e}");                      // could not read the tally sheet ballots.txt
    println!("{}", e.source().unwrap());  // no such file or directory
}
```

Delete those four lines and the struct still compiles, still holds the `io::Error`, and still prints the same first sentence. What changes is that the cause is now unreachable — the reader is told the *what* and never the *why*, and the information is sitting right there in a field. That is the single most common defect in hand-written Rust errors.

## Which is why a `Display` names one layer only

Each `Display` should say what **that** layer knew, and nothing about its cause. The cause will print itself:

```text
could not read the tally sheet ballots.txt
  caused by: no such file or directory
```

If the outer message had read *"could not read ballots.txt: no such file or directory"*, the chain would print that whole sentence and then repeat its second half. The rule falls out of composition, not style.

And the same reasoning gives the rest of the convention, which std follows throughout: **lowercase, no trailing period, no `Error:` prefix**. Every one of those is added by whoever prints the chain, and a message carrying its own turns into `error: Error: Quorum not met.: no such file...` the first time it is nested.

## Walking the chain is a loop you write

There is no `{}` that prints the whole chain, and no method that returns it:

```rust
use std::error::Error;

fn report(e: &dyn Error) -> String {
    let mut out = e.to_string();
    let mut cursor = e.source();
    while let Some(inner) = cursor {
        out.push_str(&format!("\n  caused by: {inner}"));
        cursor = inner.source();
    }
    out
}

fn main() {
    let e = std::io::Error::new(std::io::ErrorKind::NotFound, "no such file");
    println!("{}", report(&e));   // no such file
}
```

`Error::sources()` — an iterator that would replace that loop — has been unstable since 2018 ([`error_iter`, issue 58520 ↗](https://github.com/rust-lang/rust/issues/58520)), so on stable every program writes this by hand or takes a crate that has. It is four lines, and worth having in whatever your program uses to print a failure.

## What implementing `Error` buys, and what it does not

**Buys you** the blanket `impl From<E: Error> for Box<dyn Error>`, so [`?`](../../17_Option_and_Result/the_question_mark_operator/README.md) accepts your type in any function returning `Box<dyn Error>` — and `downcast_ref::<YourType>()` to get it back out. It also makes your type usable by every crate whose API is written in terms of `dyn Error`, which is most of them.

**Does not buy you** a backtrace (nothing on stable attaches one), any context about *where* it happened, the `From` impls that make `?` convert into your own type, or the chain printing above. Those four gaps are exactly the shape of [`thiserror` and `anyhow`](../thiserror_vs_anyhow/README.md): the first generates the impls, the second adds the context and the printing.

## If you are coming from another language

**Python.** `class TallyError(Exception)` is `#[derive(Debug)] struct` + `impl Error`, and `str(e)` is `Display`. Two differences worth holding onto.

The first is that Python's chain is automatic and Rust's is declared. `raise TallyError("...") from exc` sets `__cause__`, and even a plain `raise` inside an `except` block sets `__context__`, so the traceback shows *"During handling of the above exception, another exception occurred"* whether you asked for it or not. Rust's `source()` is that link, written by hand, and defaulting to `None` — which is why forgetting it is a real bug in Rust and impossible in Python.

The second is who the base class is for. `Exception` gives you `raise`/`except` machinery you genuinely need; Rust's `Error` gives you almost nothing, because the machinery is in `Result` and `?` instead. So `impl Error` is closer to `__str__` plus a marker: it says *this type is meant to travel as a failure*, and it is what lets `Box<dyn Error>` hold it.

**ABAP.** `CX_ROOT` and its subclasses map closely, and this is one of the places ABAP is ahead of the languages most Rust newcomers arrive from:

| ABAP | Rust |
|---|---|
| `CLASS cx_tally_error DEFINITION INHERITING FROM cx_static_check` | `#[derive(Debug)] enum TallyError` + `impl Error` |
| `GET_TEXT( )` / the `TEXTID` message | `impl Display` |
| `PREVIOUS` | `source()` |
| `CATCH cx_root INTO DATA(lx)` | `Box<dyn Error>` |

`PREVIOUS` is the closest thing in any of these languages to `source()` — it is the same idea, a pointer to the exception underneath, and ABAP even fills it for you if you pass `PREVIOUS = lx` when raising. The lesson transfers exactly: an exception raised without `PREVIOUS` loses the cause, and that is the same defect as a `source()` you did not override.

Where they part company is the text. ABAP's message comes from a `T100` message class — an ID and a number, translatable, maintained outside the code — and Rust has no equivalent at all: `Display` is a hard-coded English sentence in the source file, and a translated program needs a crate and a lookup you write yourself. If you are used to `MESSAGE e001(zvote) WITH lv_path`, the absence is real and worth planning for rather than discovering.

## The verified output

<!-- output:the_error_trait -->
*Verified output of [`the_error_trait.rs`](examples/the_error_trait.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The trait, in full
   pub trait Error: Debug + Display {
       fn source(&self) -> Option<&(dyn Error + 'static)> { None }
   }
   That is all of the stable surface. Two supertraits you must satisfy,
   one method with a default. `impl Error for T {}` is a real impl.

2. The two supertraits are two audiences
   Display  {}    quorum not met: 41 ballots cast, 60 needed
   Debug    {:?}  QuorumNotMet { cast: 41, needed: 60 }
   Display is the sentence a person reads. Debug is what `unwrap`
   prints, and what the runtime prints for an `Err` out of `main`.
   Writing only one of them is not a shortcut — it is E0277.

3. `source()` is what makes an error chain
   with source():    could not read the tally sheet ballots.txt
     caused by: no such file or directory
   without source(): could not read the tally sheet ballots.txt
   Same struct, same fields, same Display. The second one is holding
   the cause and not handing it on, so the reader is told the WHAT
   and never the WHY. Overriding source() is four lines.

4. Which is why Display names one layer only
   each layer's Display: could not read the tally sheet ballots.txt
   the cause's Display:  no such file or directory
   If the outer Display had written "could not read ballots.txt: no
   such file or directory", the chain would print that sentence and
   then repeat its second half. Say what only THIS layer knew.

5. The house style for a Display, and why it is not taste
   lowercase, no trailing period, no "error:" prefix:
     good  "quorum not met: 41 ballots cast, 60 needed"
     bad   "Error: Quorum not met."
   The bad one composes into: "error: Error: Quorum not met.: ..."
   std follows the same rule: no such file or directory

6. What implementing Error buys you
   Box<dyn Error> from any Error, via a blanket From impl:
     could not read the tally sheet ballots.txt
   so `?` accepts your type in any fn returning Box<dyn Error>.
   and downcast_ref gets the type back: Some("ballots.txt")
   downcast to the wrong type: false

7. What it does NOT buy you
   No automatic chain printing: `{}` on an error prints ONE layer.
   The loop in `report()` above is what every program writes, because
   Error::sources() is still unstable (error_iter, issue 58520).
   No backtrace on stable, no automatic context, and no `From` impls
   — those are the three gaps thiserror and anyhow exist to fill.
```
<!-- /output -->

## See also

- [Not every error is an `io::Error`](../not_every_error_is_io_error/README.md) — the next step: two error types in one function, and the enum that names them
- [`thiserror` vs `anyhow`](../thiserror_vs_anyhow/README.md) — the two crates that fill the four gaps above
- [Debug and Display](../../15_First_Programs/debug_vs_display/README.md) — the two traits this one requires, in their own right
- [`main` can return a `Result`](../main_returns_result/README.md) — where the `Debug` form is what your user reads
- [The `?` operator](../../17_Option_and_Result/the_question_mark_operator/README.md) — what `impl Error` makes possible at a call site
- [What a trait is](../../12_Traits/what_a_trait_is/README.md) — supertraits and default methods, which is all this one is
- [`std::error::Error` ↗](https://doc.rust-lang.org/std/error/trait.Error.html) · [Effective Rust, Item 4 ↗](https://effective-rust.com/errors.html)
