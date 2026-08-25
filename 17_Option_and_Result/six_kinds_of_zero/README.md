# Six kinds of zero

**Level:** 201 · working knowledge

**One line:** When there are more than two reasons a value is missing, `Option` is the wrong type — and the enum you write instead turns a reporting *discipline* into something the compiler enforces.

A STAR ballot cell holds a digit 0–5. The [sibling star-voting-library ↗](https://github.com/masiarek/star-voting-library) also accepts five marker characters in that same cell, and **every one of them tabulates as zero**:

| Char | Means | Tallies as |
|---|---|---|
| `0` | the voter wrote a zero | 0 |
| `-` | blank — the box was left empty | 0 |
| `~` | race abstention — skipped the whole contest | 0 |
| `&` | candidate abstention — declined on this one candidate | 0 |
| `?` | spoiled — unreadable, or two bubbles in one row | 0 |
| `%` | spoiled, and a replacement ballot was issued | 0 |

Six different things happened. The arithmetic cannot tell them apart, and it does not need to. **The audit does**, and that is the whole problem.

---

## Where `Option` runs out

[Blank is not zero](../../16_Structs/representing_a_ballot/README.md#blank-is-not-zero) already made half this argument: `Vec<Option<Stars>>` remembers that a box was never marked, where `Vec<Stars>` cannot. That is the right type when there are exactly **two** cases.

Here there are six, and `Option` has one shape for five of them:

```rust
vec![Stars::new(0), None, None, None, None, None]
//   ^ wrote a 0     ^^^^ five different voters, one variant
```

The useful reframing is that **`Option` is not "the maybe type."** It is an ordinary sum type that happens to have two variants and live in the prelude:

```rust
enum Option<T> { Some(T), None }
```

Nothing about it is special except its ubiquity. Once your problem has six cases, reaching for `Option` is not being idiomatic — it is squeezing six things into a two-slot box because the box was already open.

## Two wrong turns worth taking first

**A sentinel.** Store `255` for blank, `254` for spoiled, and so on. It works until somebody sums the column — `255` is a *number*, and the compiler will happily add it to a tally. You spent a lesson narrowing a score to 0–5 and have now widened it to all 256 values, with five of them booby-trapped.

**A parallel flag.** `Vec<u8>` for scores plus `Vec<bool>` for "is a marker" — two vectors that must be edited together, which is exactly the desync bug from the ballot lesson, reintroduced. And it still cannot say *which* marker, so a third parallel vector is already on its way.

Both fail the same way: they put the meaning somewhere the type system cannot see it.

## The enum, and a projection that is lossy on purpose

```rust
enum Mark {
    Scored(Stars),       // '0'..='5'
    Blank,               // '-'
    RaceAbstention,      // '~'
    CandidateAbstention, // '&'
    Spoiled,             // '?'
    SpoiledReissued,     // '%'
}
```

One variant carries data; five do not. That mixture is normal and is most of what enums are for.

The tally then becomes a **projection** that deliberately throws information away:

```rust
fn tally_value(self) -> u8 {
    match self {
        Mark::Scored(s) => s.value(),
        Mark::Blank | Mark::RaceAbstention | Mark::CandidateAbstention
        | Mark::Spoiled | Mark::SpoiledReissued => 0,
    }
}
```

Five distinctions vanish, and that is *correct* — the arithmetic genuinely does not care. What matters is that they vanish **at the point of use rather than at the point of storage**. The type keeps what the projection discards, so a second function can still ask a different question of the same data.

## The payoff: a discipline becomes a type error

Here is the part that is hard to appreciate until it saves you.

Those markers are **paper vocabulary** — an online form prevents most of them at input, so a ballot cell can only be `?` or `%` if a human physically marked a sheet. Which means a hybrid election's marker counts are lopsided by channel, and a report that pools them says *"paper voters are sloppier"* when the truth is *"only paper voters **can** produce four of the six marks."* Same numbers, opposite conclusion.

In Python that is a rule someone has to remember. In Rust, `Channel` is a type, `Mark` is a type, and the function that reports them has to name every case it handles. Add a seventh marker next year and **every match that does not mention it stops compiling** — the report cannot silently keep its old shape while the data grows a new one.

That is the actual trade. You write six arms where a dynamic language writes one lookup, and in exchange the compiler audits every reader of that type, forever, including the ones written by people who have never heard of the marker you just added.

## What it costs: nothing, and the reason is a bit pattern

```text
size_of::<Stars>()          = 1
size_of::<Mark>()           = 1
size_of::<Option<Mark>>()   = 1
```

Eleven states — six scores plus five markers — and a byte holds 256, so `Mark` is one byte. `Option<Mark>` is *also* one byte, because the compiler parks `None` in one of the 245 patterns `Mark` never uses. This is the same [niche optimization](../option_as_collection/README.md) as before, now paying for something you actually wanted.

**One detail worth keeping**, because it was a genuine surprise while writing this page: that only works because `Stars` is an **enum**. Define it as `struct Stars(u8)` instead and `Mark` becomes **2 bytes** — every one of the 256 `u8` patterns is legal, so there is no spare pattern for a discriminant to hide in, and the compiler must add a tag byte. The newtype and the enum look interchangeable at the call site and are not: an enum tells the compiler which values are impossible, and impossible values are the raw material every niche optimization is made of.

## If you are coming from another language

- **Python** — you would write `IntEnum` or a set of string constants, and it works. What does not transfer is the checking: `match` in 3.10+ is a statement, not an exhaustiveness proof, and a `case _:` is mandatory-by-habit rather than a decision. Add a seventh marker and every `if/elif` chain and `dict` lookup in the codebase keeps running, quietly filing it under whatever the fallback was. The Rust version's six arms are not ceremony — they are the list of places that must be revisited, computed for you.
- **ABAP** — a domain with fixed values is the shape, and it is *documentation*: `CASE` without `WHEN OTHERS` does not fail activation, and adding a value to the domain does not flag a single `CASE` that ignores it. The nearest familiar pain is a status field that grows a code, after which every report silently buckets it wrong until a user notices a total. Rust's contribution is not that you can enumerate the states — you always could — it is that enumerating them creates an obligation the compiler will not let anyone drop.

## IOUs from this rung

- `Mark::try_from(char)` returns `Result<Mark, char>`, and the example `.expect()`s it. Real parsing wants the row and column in the error. → the error rungs.
- The election is hard-coded; nothing is read from a file.
- `Channel` is carried alongside the marks rather than in them, which is the right call here and would not be if channels multiplied.

---

## Practice

**The arm you didn't write.** The office adds a sixth mark, `%` — spoiled, and a replacement issued. Write two functions that bucket a `Mark` into an audit category: one with a `_ =>` catch-all, one exhaustive. Add the new variant, then run both over the same stack of ballots.

One of them changes its answer and does not tell you. Then read rustc's suggested fix for the other one carefully, and work out why taking it would be a mistake.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:six_kinds_of_zero_kata -->
*[`six_kinds_of_zero_kata.rs`](examples/six_kinds_of_zero_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: `_ =>` is a hole you punch in your own safety net.
//!
//! The election office adds a sixth mark — '%', spoiled and reissued. Two audit
//! functions were written before it existed. Only one of them tells you.
//!
//!   rustc --edition 2024 six_kinds_of_zero_kata.rs -o /tmp/skzk && /tmp/skzk

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mark {
    Scored(u8),
    Blank,
    RaceAbstention,
    CandidateAbstention,
    Spoiled,
    SpoiledReissued, // <- added today
}

/// Written when there were five marks. The `_` arm looked harmless: every
/// remaining case really was a blank at the time.
fn bucket_lazy(m: Mark) -> &'static str {
    match m {
        Mark::Scored(_) => "scored",
        Mark::RaceAbstention => "abstention",
        Mark::CandidateAbstention => "abstention",
        Mark::Spoiled => "spoiled",
        _ => "blank",
    }
}

/// Written the same day, without the `_`. It did not compile this morning
/// until somebody chose an arm for the new mark — and chose "spoiled".
fn bucket_strict(m: Mark) -> &'static str {
    match m {
        Mark::Scored(_) => "scored",
        Mark::Blank => "blank",
        Mark::RaceAbstention => "abstention",
        Mark::CandidateAbstention => "abstention",
        Mark::Spoiled => "spoiled",
        Mark::SpoiledReissued => "spoiled",
    }
}

fn tally<'a>(marks: &[Mark], bucket: fn(Mark) -> &'a str) -> Vec<(&'a str, usize)> {
    let mut out: Vec<(&str, usize)> = Vec::new();
    for m in marks {
        let b = bucket(*m);
        match out.iter_mut().find(|(name, _)| *name == b) {
            Some((_, n)) => *n += 1,
            None => out.push((b, 1)),
        }
    }
    out.sort_by_key(|(name, _)| *name);
    out
}

fn main() {
    let ballots = [
        Mark::Scored(5),
        Mark::Scored(3),
        Mark::Blank,
        Mark::RaceAbstention,
        Mark::Spoiled,
        Mark::SpoiledReissued,
        Mark::SpoiledReissued,
        Mark::CandidateAbstention,
    ];

    println!("Eight cells, two audit reports of the same stack:\n");
    println!("  with `_ => \"blank\"`   {:?}", tally(&ballots, bucket_lazy));
    println!("  exhaustive            {:?}", tally(&ballots, bucket_strict));

    println!("\n  The two reissued ballots landed in \"blank\" on the first report.");
    println!("  Nothing failed. It compiled, it ran, and it filed two spoiled");
    println!("  ballots as voters who declined to mark the box — which is the");
    println!("  difference between a printing fault and an electorate's opinion.");

    println!("\nWhat the second function did this morning, before it was fixed —");
    println!("real rustc output, not a paraphrase:\n");
    for line in [
        "error[E0004]: non-exhaustive patterns: `Mark::SpoiledReissued` not covered",
        "   |",
        "   |     match m {",
        "   |           ^ pattern `Mark::SpoiledReissued` not covered",
        "   |",
        "help: ensure that all possible cases are being handled by adding a match",
        "      arm with a wildcard pattern or an explicit pattern as shown",
        "   |",
        "   ~         Mark::Spoiled => \"spoiled\",",
        "   ~         Mark::SpoiledReissued => todo!(),",
    ] {
        println!("    {line}");
    }

    println!("\n  Read the help text closely: rustc offers you the wildcard as a fix.");
    println!("  Taking it turns this compile error into the silent miscount above,");
    println!("  for every marker anyone adds afterwards. `_` is not a shortcut —");
    println!("  it is a standing promise that every FUTURE variant belongs in that");
    println!("  bucket, made on behalf of people who have not written them yet.");

    println!("\n  Where a wildcard IS right: matching on something genuinely open,");
    println!("  like a char from a file. `_ => return Err(other)` is a real answer");
    println!("  to 'any other character', because there is no finite list to name.");
}
```
<!-- /source -->

<!-- output:six_kinds_of_zero_kata -->
*Verified output of [`six_kinds_of_zero_kata.rs`](examples/six_kinds_of_zero_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Eight cells, two audit reports of the same stack:

  with `_ => "blank"`   [("abstention", 2), ("blank", 3), ("scored", 2), ("spoiled", 1)]
  exhaustive            [("abstention", 2), ("blank", 1), ("scored", 2), ("spoiled", 3)]

  The two reissued ballots landed in "blank" on the first report.
  Nothing failed. It compiled, it ran, and it filed two spoiled
  ballots as voters who declined to mark the box — which is the
  difference between a printing fault and an electorate's opinion.

What the second function did this morning, before it was fixed —
real rustc output, not a paraphrase:

    error[E0004]: non-exhaustive patterns: `Mark::SpoiledReissued` not covered
       |
       |     match m {
       |           ^ pattern `Mark::SpoiledReissued` not covered
       |
    help: ensure that all possible cases are being handled by adding a match
          arm with a wildcard pattern or an explicit pattern as shown
       |
       ~         Mark::Spoiled => "spoiled",
       ~         Mark::SpoiledReissued => todo!(),

  Read the help text closely: rustc offers you the wildcard as a fix.
  Taking it turns this compile error into the silent miscount above,
  for every marker anyone adds afterwards. `_` is not a shortcut —
  it is a standing promise that every FUTURE variant belongs in that
  bucket, made on behalf of people who have not written them yet.

  Where a wildcard IS right: matching on something genuinely open,
  like a char from a file. `_ => return Err(other)` is a real answer
  to 'any other character', because there is no finite list to name.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:six_kinds_of_zero -->
*Verified output of [`six_kinds_of_zero.rs`](examples/six_kinds_of_zero.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: The six kinds of zero
  char   variant                          tallies as
   0     scored                           0
   -     blank                            0
   ~     race abstention                  0
   &     candidate abstention             0
   ?     spoiled                          0
   %     spoiled + reissued               0
      Every row adds zero to the total. Six different things
      happened, and a report that says only "0" has lost five of them.

──── Step 2: Option can tell two of them apart, and that is all
  Option<Stars>  [Some(Zero), None, None, None, None, None]
      Some(Stars(0)) is the voter who wrote a zero. The five Nones
      are five different voters, and Option has one shape for all
      of them. `Option` is not "the maybe type" — it is a sum type
      with exactly two variants, and you have six cases.

──── Step 3: Two tempting wrong turns
  sentinel u8    [0, 255, 254, 253, 252, 251]
      Works until somebody sums the column. 255 is a NUMBER; the
      compiler will happily add it to a tally, and 0..=5 no longer
      describes the type. You have widened the thing you narrowed.
  parallel flags scores [0, 0, 0, 0, 0, 0] is_marker [false, true, true, true, true, true]
      Two Vecs that must be edited together — the desync bug from
      the ballot lesson, reintroduced. And it still cannot say WHICH
      marker, so a third parallel Vec is coming.

──── Step 4: The enum, and a projection that is lossy on purpose
  ballot  5,-,3,~,?,%
    cell 0  scored               -> tallies 5   paper-only: false
    cell 1  blank                -> tallies 0   paper-only: false
    cell 2  scored               -> tallies 3   paper-only: false
    cell 3  race abstention      -> tallies 0   paper-only: true
    cell 4  spoiled              -> tallies 0   paper-only: true
    cell 5  spoiled + reissued   -> tallies 0   paper-only: true
  total = 8
      tally_value() throws five distinctions away, and that is
      correct — the arithmetic genuinely does not care. The type
      keeps what the projection discards, so the audit still can.

──── Step 5: The report the type makes possible
  Paper: 12 cells, 6 non-scores, 4 of them paper-only
  Online: 12 cells, 3 non-scores, 0 of them paper-only
      Pooled, that reads "paper voters are sloppier." Split, it reads
      "only paper voters CAN produce four of the six marks." Same
      numbers, opposite conclusion — which is why the channel has to
      be in the type too, not in a comment.

──── Step 6: What it costs
  size_of::<Stars>()          = 1
  size_of::<Mark>()           = 1
  size_of::<Option<Mark>>()   = 1
      Six score values plus five markers is eleven states, and a byte
      holds 256. Option<Mark> is still one byte, because the compiler
      parks None in a bit pattern Mark never uses. Remembering all six
      distinctions costs exactly nothing over storing the digit.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 17_Option_and_Result/six_kinds_of_zero/examples/six_kinds_of_zero.rs -o /tmp/skz && /tmp/skz
```

## See also

- [What is a ballot, in memory?](../../16_Structs/representing_a_ballot/README.md) — the rung below; where "blank is not zero" stops at two cases
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the `Stars` enum this page wraps, and why it is an enum
- [`Option` as a collection](../option_as_collection/README.md) — the niche optimization that makes `Option<Mark>` free
