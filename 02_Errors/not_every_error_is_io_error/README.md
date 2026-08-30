# Not every error is an `io::Error`

**Level:** 201 · working knowledge

**One line:** A function that reads a file *and* parses a number has two error types and one return type — and the two ways to make them one are to **erase** the difference or to **name** it, which is the same decision as who your caller is.

```rust
use std::io;

fn read_sheet(_name: &str) -> Result<String, io::Error> {
    Ok("Ada=5".to_string())   // stands in for a file read
}

fn tally(name: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let sheet = read_sheet(name)?;                  // io::Error
    let (_who, count) = sheet.split_once('=').ok_or("sheet has no '='")?;
    let n: u32 = count.parse()?;                    // ParseIntError
    Ok(n)
}

fn main() {
    println!("{:?}", tally("ballots.txt").map_err(|e| e.to_string()));   // Ok(5)
}
```

## The error you actually get

Declare that function `-> Result<u32, ParseIntError>` — the type of the last thing that can fail, which is the natural first guess — and the read stops compiling:

```text title="Abridged — real rustc output for two_errors.rs, without the file-and-line header"
error[E0277]: `?` couldn't convert the error to `ParseIntError`
   |
12 | fn tally(name: &str) -> Result<u32, ParseIntError> {
   |                         -------------------------- expected `ParseIntError` because of this
13 |     let sheet = read_sheet(name)?;
   |                 ----------------^ the trait `From<std::io::Error>` is not implemented for `ParseIntError`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
```

Read where the error lands. Not at the call — `read_sheet` returned exactly what it promised — but at the **`?`**, which is [the operator that converts through `From`](../../17_Option_and_Result/the_question_mark_operator/README.md). The message is telling you that a conversion you never wrote is missing, and both answers below are ways of supplying one.

## Answer 1 — erase it: `Box<dyn Error>`

Every type implementing [`Error`](../the_error_trait/README.md) converts into `Box<dyn Error>` through a blanket impl, so this compiles the moment you write it and keeps compiling as you add new failure sources:

```text
ballots.txt  Ok(5)
torn.txt     Err("invalid digit found in string")
gone.txt     Err("no such file or directory")
```

Three different failures, one type. What a caller can now do with the value is print it. That is the whole list — and at the top of a program, where the caller is a person, it is also the whole requirement.

The escape hatch shows the cost rather than removing it:

```rust
use std::error::Error;
use std::io;

fn main() {
    let boxed: Box<dyn Error> =
        Box::new(io::Error::new(io::ErrorKind::NotFound, "no such file"));

    println!("{:?}", boxed.downcast_ref::<io::Error>().map(|e| e.kind()));   // Some(NotFound)
    println!("{:?}", boxed.downcast_ref::<std::num::ParseIntError>());       // None
}
```

`downcast_ref` works, and notice what it is: a run-time type test, one per candidate you can think of, that the compiler cannot check you finished. Add a new failure upstream and every caller silently falls through to its fallback — no error, no warning, no arm to fill in.

## Answer 2 — name it: an enum

One variant per thing a caller might do **differently**. Not one per underlying error type — that distinction is where most hand-written error enums go wrong:

```rust
use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum TallyError {
    Unreadable(io::Error),
    Malformed(String),
    NotANumber(ParseIntError),
}

impl fmt::Display for TallyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TallyError::Unreadable(_) => write!(f, "could not read the tally sheet"),
            TallyError::Malformed(row) => write!(f, "row has no '=': {row:?}"),
            TallyError::NotANumber(_) => write!(f, "the count is not a number"),
        }
    }
}

impl Error for TallyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TallyError::Unreadable(e) => Some(e),
            TallyError::NotANumber(e) => Some(e),
            TallyError::Malformed(_) => None,   // nothing underneath: we made this one
        }
    }
}

impl From<io::Error> for TallyError {
    fn from(e: io::Error) -> Self { TallyError::Unreadable(e) }
}

impl From<ParseIntError> for TallyError {
    fn from(e: ParseIntError) -> Self { TallyError::NotANumber(e) }
}

fn main() {
    let e = TallyError::from("x".parse::<u32>().unwrap_err());
    println!("{e}");                      // the count is not a number
    println!("{}", e.source().unwrap());  // invalid digit found in string

    let m = TallyError::Malformed("Dev".to_string());
    println!("{m}");                      // row has no '=': "Dev"
    println!("{:?}", m.source().is_none());   // true - we made this one
}
```

Those two `From` impls are the whole trick: `?` finds them by type, so the body of `tally` is unchanged — `read_sheet(name)?` now produces a `TallyError::Unreadable` and nobody wrote a conversion at the call site. And because each variant keeps the error it came from and hands it back through `source()`, the chain survives the renaming:

```text
torn.txt     the count is not a number <- invalid digit found in string
gone.txt     could not read the tally sheet <- no such file or directory
```

## Which is the point: the caller can decide

```rust title="Assuming the `TallyError` and `tally` from above"
fn recover(name: &str) -> String {
    match tally(name) {
        Ok(n) => format!("counted {n}"),
        Err(TallyError::Unreadable(e)) if e.kind() == io::ErrorKind::NotFound =>
            "no sheet yet — starting a fresh tally at 0".to_string(),
        Err(TallyError::Unreadable(_)) => "sheet unreadable — retrying later".to_string(),
        Err(TallyError::NotANumber(_)) => "one bad count — skipping the row".to_string(),
        Err(TallyError::Malformed(_)) => "sheet is not a tally sheet — aborting".to_string(),
    }
}
```

Four failures, four different recoveries — start fresh, retry, skip, abort. None of them is available through a `Box<dyn Error>`, because the value no longer knows which one it is. And the `match` is **exhaustive**: add a variant, and the compiler walks you round every caller that has a decision to revisit. That is the guarantee `downcast_ref` cannot give.

## What it cost to write

Counted from the example behind this page:

| | lines | |
|---|---|---|
| the enum | 5 | you write this either way |
| `impl Display` | 9 | derived from attributes by `thiserror` |
| `impl Error` (`source`) | 9 | " |
| `impl From<io::Error>` | 5 | " |
| `impl From<ParseIntError>` | 5 | " |
| **total** | **33** | **28 of them mechanical** |

That is the honest case for [`thiserror` ↗](https://docs.rs/thiserror): it generates those 28 lines from attributes and changes the model above by nothing at all.

```rust title="Not compiled here — thiserror is a dependency, and examples in this library build with no crates"
#[derive(Debug, thiserror::Error)]
enum TallyError {
    #[error("could not read the tally sheet")]
    Unreadable(#[from] std::io::Error),
    #[error("row has no '=': {0:?}")]
    Malformed(String),
    #[error("the count is not a number")]
    NotANumber(#[from] std::num::ParseIntError),
}
```

`#[from]` writes the `From` impl *and* the `source()` arm; `#[error("…")]` writes `Display`. Same enum, same variants, same behaviour — which is why it is worth writing the long form once, as above, before reaching for the derive.

## So which one

**A library names them.** Its callers have decisions to make, and the enum *is* part of the API. Which means it is semver-visible: adding a variant breaks every exhaustive `match` downstream unless you mark it `#[non_exhaustive]`, which forces callers to include a `_` arm from the start. (Within a single crate the attribute does nothing — its effect is entirely on code in *other* crates, which is exactly the code you cannot see.)

**A binary erases them.** The caller is a person, the only remaining decision is what to print, and `Box<dyn Error>` — or [`anyhow`](../anyhow_and_context/README.md), which is that plus context — says so honestly.

The common shape is both: a library half that names its failures, and a `main` that erases them on the way to the screen. Picking by habit is how a library ends up returning one opaque type to callers who needed to tell *file missing* from *file malformed*. [`thiserror` vs `anyhow`](../thiserror_vs_anyhow/README.md) is that decision in full.

## If you are coming from another language

**Python.** An exception hierarchy is the enum, spread across classes: `except FileNotFoundError` / `except ValueError` are the variants, and the `except` ladder is the `match`. The differences are the familiar two — the ladder is not exhaustive, so a failure nobody wrote a clause for escapes to the top; and the function's signature says nothing about which exceptions can arrive, so the ladder is written from documentation and experience rather than from the type.

The Rust move with no Python equivalent is `#[from]` and `?`: in Python you re-raise by hand (`raise TallyError(...) from exc`) at every layer, while `?` performs the conversion at every `?` in the module from one `From` impl written once.

**ABAP.** The enum is a small exception hierarchy under a common superclass, and `RAISING cx_tally_error` in the signature is the `E` in `Result<T, E>` — declared, checked, part of the interface. Three things line up and one bites:

- `CATCH cx_unreadable. CATCH cx_not_a_number.` is the `match`, and like Python's ladder it is **not** exhaustive — `CATCH cx_root` is the `_` arm you add to be safe, and `Box<dyn Error>` is what you get when you use it everywhere.
- `PREVIOUS` is `source()`, and passing it is the same discipline.
- `CX_STATIC_CHECK` versus `CX_DYNAMIC_CHECK` is roughly the "must the caller deal with this" question that Rust answers with `#[must_use]` on `Result` — except Rust has only the strict setting.

The one that bites: adding a subclass to an ABAP exception hierarchy is safe, because callers catch by superclass. Adding a variant to a public Rust enum is a **breaking change**, because callers match exhaustively. `#[non_exhaustive]` is how you buy back the ABAP behaviour, and it has to be there from the first release.

## The verified output

<!-- output:not_every_error_is_io_error -->
*Verified output of [`not_every_error_is_io_error.rs`](examples/not_every_error_is_io_error.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One function, two error types
   read_sheet -> io::Error       parse -> ParseIntError
   Declaring the function `-> Result<u32, ParseIntError>` and using `?`
   on the read is E0277: "`?` couldn't convert the error", because the
   conversion `?` performs is `From`, and nobody wrote that impl.

2. Answer 1 — erase it: Box<dyn Error>
   ballots.txt  Ok(5)
   torn.txt     Err("invalid digit found in string")
   gone.txt     Err("no such file or directory")
   Compiles the moment you write it, and every error converts. What
   the caller can now do is print. That is the whole list.

3. ...and the escape hatch that proves the cost
   downcast_ref::<io::Error>()     = Some(NotFound)
   downcast_ref::<ParseIntError>() = None
   It works, and notice what it is: a run-time type test, one per
   candidate, that the compiler cannot check you finished. Add a new
   failure upstream and every caller silently falls through.

4. Answer 2 — name it: an enum with one variant per decision
   ballots.txt  Ok(5)
   torn.txt     the count is not a number <- invalid digit found in string
   gone.txt     could not read the tally sheet <- no such file or directory
   Same three inputs. Now the failures have names, and each one still
   carries the error it came from, so the chain survives.

5. Which is the point: the caller can DECIDE
   ballots.txt  counted 5
   torn.txt     one bad count — skipping the row
   gone.txt     no sheet yet — starting a fresh tally at 0
   Four arms, four different recoveries — and the match is exhaustive,
   so adding a variant makes the compiler visit every caller.

6. What it cost to write
   the enum                  5 lines   you write this either way
   impl Display              9 lines   \
   impl Error (source)       9 lines    | derived from attributes
   impl From<io::Error>      5 lines    | by `thiserror`
   impl From<ParseIntError>  5 lines   /
   -------------------------------
   33 lines, 28 of them mechanical. That is what the crate removes —
   and it does not change the model above by one byte.

7. So which one
   A library: name them. Its callers have decisions to make, and the
   enum IS the API — mark it #[non_exhaustive] or adding a variant is
   a breaking change.
   A binary: erase them. The caller is a person, the decision is what
   to print, and Box<dyn Error> (or anyhow) says exactly that.
   The common shape is both: a library half that names, a main that
   erases. Picking by habit is how a library ends up unable to tell
   its callers 'file missing' from 'file malformed'.
```
<!-- /output -->

## See also

- [What makes a type an error](../the_error_trait/README.md) — the trait both answers rest on, and the `source()` that keeps the chain
- [`thiserror` vs `anyhow`](../thiserror_vs_anyhow/README.md) — the same decision, with the two crates that own each side of it
- [`anyhow` and context](../anyhow_and_context/README.md) — the erasing side, done properly
- [The `?` operator](../../17_Option_and_Result/the_question_mark_operator/README.md) — the `From` hop this whole page is about
- [`From` and `Into`](../../29_Conversion/from_and_into/README.md) — the conversion in its own right
- [Variants that carry data](../../13_Enums/variants_that_carry_data/README.md) — what the enum above is doing
- [The `Result` you are reading is probably an alias](../../17_Option_and_Result/result_aliases/README.md) — expanding `io::Result` to see the `E` you are converting
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — Step 8 designs the `E`, Step 9 is when you would rather not
