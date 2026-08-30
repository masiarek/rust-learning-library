# `Option` fields: modelling what may be absent

**Level:** 101 · for newcomers

**One line:** In a struct, every field is required — `Option<T>` is how the type says *"this one may legitimately be missing"*, and then the compiler holds every reader to it.

The other Option pages are about what a function *returns*. This one is about `Option` in a type definition, which is where you meet it when modelling data or deserializing someone else's JSON.

---

## Required is the default

```rust
#[derive(Debug)]
struct Person {
    first: String,
    last: String,
    age: Option<u8>,
}
```

Leave a field out of the initializer and it does not compile. Supply the wrong shape and the compiler tells you exactly what to write:

```text title="Abridged — real rustc output for `age: 57`"
error[E0308]: mismatched types
7 |     let p = Person { first: "Ada".to_string(), age: 57 };
  |                                                     ^^ expected `Option<u8>`, found integer
  |
  = note: expected enum `Option<u8>`
             found type `{integer}`
help: try wrapping the expression in `Some`
  |
7 |     let p = Person { first: "Ada".to_string(), age: Some(57) };
  |                                                     +++++  +
```

That error is the lesson in miniature. There is no "unset" state to fall into: you write `Some(57)` or you write `None`, and writing `None` is you *stating* that the value is absent rather than leaving the question open. Compare a language where a missing field is `null` by default and every reader downstream has to guess whether that was deliberate.

## Reading the field

```rust
match &b.isbn {
    Some(i) => format!("{} — ISBN {i}", b.title),
    None    => format!("{} — no ISBN on record", b.title),
}
```

Note `match &b.isbn`, not `match b.isbn`. The second tries to *move* the `String` out of a struct you only borrowed, and the borrow checker refuses. This is the single most common stumble with `Option` fields, and the fix is almost always one `&` or one `.as_ref()`.

The three one-liners that cover most days:

| Call | Gives you | Use when |
|---|---|---|
| [`.as_deref()` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref) | `Option<&str>` from `Option<String>` | passing it on without cloning |
| [`.map_or(default, f)` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or) | one value either way | rendering for display |
| [`.unwrap_or_default()` ↗](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default) | `T::default()` when absent | absent and empty mean the same thing |

Use `.as_ref()` when you want to inspect the inside and keep the original — `known.isbn.as_ref().map(|s| s.len())` leaves `known.isbn` owned and intact, where `known.isbn.map(…)` would consume it.

## The question that decides whether a field should be `Option` at all

`Option<Vec<T>>` is usually wrong, because `Vec` can already say "nothing here" by being empty. It is right exactly when **absent and empty mean different things**:

```rust
struct Survey {
    question: String,
    /// None = not collected yet. Some(vec![]) = collected, and nobody answered.
    responses: Option<Vec<u32>>,
}
```

A plain `Vec` collapses those two states into one, and no amount of downstream code can recover the difference. So ask it directly: *is "we never asked" a different fact from "we asked and got nothing"?* If yes, keep the `Option`. If no, delete it — you are carrying a wrapper that only adds a `match` arm.

The same question governs `Option<bool>` (is "unanswered" different from "no"?) and `Option<String>` (is a missing name different from an empty one?). Most of the time the answer is no, and the plain type is better.

## `Option` fields and `Default`

`Option<T>` implements `Default` as `None` **for every `T`**, whether or not `T` itself has a default:

```rust
#[derive(Debug, Default)]
struct Config {
    name: Option<String>,
    retries: Option<u32>,
    verbose: bool,
}

let cfg = Config { retries: Some(3), ..Default::default() };
```

This is why config structs are so often made of `Option`s: you get `Default` for free, the struct-update syntax lets a caller set one field, and each `None` honestly records "the user did not say", leaving you free to apply the real default at the point of use with `.unwrap_or(1)`.

Note `verbose` is a plain `bool`, not `Option<bool>` — for a flag, `false` *is* the answer, so wrapping it would only invent a third state nobody needs.

## What it costs

```
Person       56 bytes
String       24   (each)
Option<u8>    2
u8            1
```

`u8` uses all 256 of its bit patterns, so there is no spare one to mean `None`, and `Option<u8>` must carry a separate tag byte — doubling it. That is the *expensive* case, and it is still one byte. Where the inner type has an unused pattern the wrapper is free; [`Option` is a one-item collection](../option_as_collection/README.md) has the sizes side by side.

---

## Practice

**Which fields may legitimately be missing?** Model a `Voter` with a name, a middle name, a precinct, and the scores from their ballot. Make each field `Option` or not on purpose, and write a reader that tells *no ballot returned* apart from *ballot returned, nobody scored*.

Make `precinct` an `Option<u32>` first and count how many readers now have to handle a state that cannot occur. Then try storing the scores as a plain `Vec<u8>` and see which of the two facts above you can no longer report.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:option_fields_kata -->
*[`option_fields_kata.rs`](examples/option_fields_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: which fields may legitimately be missing?
//!
//!   rustc --edition 2024 option_fields_kata.rs -o /tmp/ofk && /tmp/ofk

#[derive(Debug, Default)]
struct Voter {
    /// Required. A voter with no name is not a voter with an empty name.
    name: String,
    /// Optional, and the type says so: plenty of people have no middle name.
    middle_name: Option<String>,
    /// NOT an Option. "Registered in no precinct" is not a state that exists;
    /// making it optional would push a check into every reader for nothing.
    precinct: u32,
    /// Optional for a real reason: the ballot may not be back yet. An empty Vec
    /// would say "returned, and scored nobody", which is a different fact.
    scores: Option<Vec<u8>>,
}

fn describe(v: &Voter) {
    let full = match &v.middle_name {
        Some(m) => format!("{} {} ", v.name, m),
        None => format!("{} ", v.name),
    };
    // Two different questions, and the type makes you ask them separately.
    let ballot = match &v.scores {
        None => "no ballot returned".to_string(),
        Some(s) if s.is_empty() => "ballot returned, nobody scored".to_string(),
        Some(s) => format!("ballot returned, {} scores, total {}", s.len(), s.iter().map(|n| *n as u32).sum::<u32>()),
    };
    println!("  {full:<18} precinct {:<4} {ballot}", v.precinct);
}

fn main() {
    let voters = [
        Voter { name: "Ada".into(), middle_name: Some("Q".into()), precinct: 7, scores: Some(vec![5, 2, 0]) },
        Voter { name: "Ben".into(), middle_name: None, precinct: 7, scores: Some(vec![]) },
        Voter { name: "Cara".into(), middle_name: None, precinct: 12, scores: None },
    ];

    println!("Three voters, and the two absences the type keeps apart:");
    for v in &voters {
        describe(v);
    }

    let returned = voters.iter().filter(|v| v.scores.is_some()).count();
    println!("\n  ballots returned: {returned} of {}", voters.len());
    println!("      A Vec<u8> field with no Option could not have told you that.");

    println!("\nDefault gives None for every optional field, for free:");
    println!("  {:?}", Voter::default());
}
```
<!-- /source -->

<!-- output:option_fields_kata -->
*Verified output of [`option_fields_kata.rs`](examples/option_fields_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Three voters, and the two absences the type keeps apart:
  Ada Q              precinct 7    ballot returned, 3 scores, total 7
  Ben                precinct 7    ballot returned, nobody scored
  Cara               precinct 12   no ballot returned

  ballots returned: 2 of 3
      A Vec<u8> field with no Option could not have told you that.

Default gives None for every optional field, for free:
  Voter { name: "", middle_name: None, precinct: 0, scores: None }
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:option_fields -->
*Verified output of [`option_fields.rs`](examples/option_fields.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: Required by default; Option marks the exceptions
  known.isbn = Some("1-123-456")   tags: rust, reference
  draft.isbn = None   tags: 0 (none)
      You cannot forget a field: leaving `isbn` out is a compile error, and
      writing `None` is you SAYING it is absent rather than leaving it undefined.

──── Step 2: Reading the field: match on a REFERENCE to it
  Great book — ISBN 1-123-456
  Untitled draft — no ISBN on record
      `match &b.isbn`, not `match b.isbn` — the latter tries to move a String
      out of a borrowed struct, which the borrow checker refuses.

──── Step 3: The three one-liners you will actually use
  as_deref()          Some("1-123-456")
  map_or (known)      320 pages
  map_or (draft)      unknown
  unwrap_or_default   ""
  as_ref().map(len)   Some(9)   (known.isbn still owned: Some("1-123-456"))

──── Step 4: Option<Vec<T>> — usually wrong, occasionally exactly right
  question: "Best method?"
  not collected yet
  collected — nobody answered
  collected — 2 responses
      A plain Vec cannot tell those first two apart. If they mean the same thing
      in your domain, use Vec and delete the Option; if they differ, keep it.

──── Step 5: Option<T> defaults to None for every T
  Config::default() = Config { name: None, retries: None, verbose: false }
  one field set     = Config { name: None, retries: Some(3), verbose: false }
      Option<T>: Default is None regardless of T — so a config struct of Options
      derives Default even when the inner types cannot.
  retries.unwrap_or(1)      = 3
  name.unwrap_or("anon")    = anon
  verbose (a plain bool)   = false   <- not optional: false IS the answer

──── Step 6: What an optional field costs
  Ada Lovelace — age None
  Person                  56 bytes
  String (each)           24
  Option<u8>               2
  u8                       1
      u8 uses all 256 of its bit patterns, so there is no spare one for None
      and Option<u8> must carry a separate tag byte. Compare Option<Box<T>>,
      which is free because a null pointer was never a legal Box.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 17_Option_and_Result/option_fields/examples/option_fields.rs -o /tmp/of && /tmp/of
```

## See also

- [`Option` vs `Result`](../option_vs_result/README.md) — absence versus failure
- [`Option` is a one-item collection](../option_as_collection/README.md) — iteration, `take()`, and what the wrapper costs
- [`std::option` module docs ↗](https://doc.rust-lang.org/core/option/) — the canonical list of what `Option` is for
