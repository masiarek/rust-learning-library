# A score is not a number: the newtype

**Level:** 101 → 201 · working knowledge

**One line:** Wrap a primitive in a one-field type whose only constructor validates, and every function downstream can stop checking — because the invalid value no longer exists.

A STAR ballot cell holds an integer **0 to 5**. A `u8` holds **0 to 255**. That gap is where the bug lives, and no amount of careful coding closes it, because the type is telling every reader of your program that 200 is a perfectly good score.

---

## The gap

Here is a function that sums a ballot row. It is correct, and it is defenceless:

```rust
fn total_raw(row: &[u8]) -> u32 {
    row.iter().map(|&s| s as u32).sum()
}
```

A slipped keystroke turns `3` into `30`, and one ballot outvotes ten honest ones. The function has no way to object — it was handed `u8`s, and 30 is a `u8`. You can add a check inside it, but then you must add the same check inside every *other* function that touches a row, and remember to keep them in step forever. That is a convention, and conventions are enforced by attention.

## One door, checked once

The **newtype** is a struct with exactly one field and a private constructor:

```rust
pub struct Score(u8);

impl Score {
    pub const MAX: u8 = 5;

    pub fn new(raw: u8) -> Option<Score> {
        if raw <= Self::MAX { Some(Score(raw)) } else { None }
    }

    pub fn value(self) -> u8 { self.0 }
}
```

`Score::new` is the only way to obtain a `Score`, and it is a [partial function made total](../partial_functions/README.md): every `u8` gets an answer, and the out-of-range ones get `None`. The check happens **once, at the boundary** — where data enters the program — and never again.

A whole ballot row goes through the same door in one expression:

```rust
let parsed: Option<Vec<Score>> = row.iter().map(|&c| Score::new(c)).collect();
```

`collect()` into an `Option<Vec<_>>` stops at the first `None` and yields `None` for the whole row. You never hold a half-validated ballot — the row is accepted or rejected as a unit, which is exactly the rule a ballot needs.

## What the caller gains

```rust
fn total(row: &[Score]) -> u32 {
    row.iter().map(|s| s.value() as u32).sum()
}
```

Compare it to `total_raw`. Same arithmetic, no check — and **not because we skipped the check**, but because there is nothing left to check. A `&[Score]` is a proof, carried in the type, that somebody already looked. This is the whole return on the newtype: validation stops being a thing every function does and becomes a thing the *type* remembers.

The `derive` list matters more than it looks. `PartialOrd, Ord` make `Score` compare like the number it wraps, so `max()` and `sort()` work — which is precisely what a scoring round and a runoff are made of. `Copy` makes it behave like a primitive instead of moving out from under you. `PartialEq, Eq` let you compare cells. You get all of it for one line.

## The trap: privacy is per *module*, not per type

This is the mistake worth knowing, because the code looks identical when it is wrong:

```rust
let bad = Score(7);   // outside `mod ballot`
```

```text
error[E0423]: cannot initialize a tuple struct which contains private fields
note: constructor is not visible here due to private fields
```

That error only appears **outside** the module the struct is declared in. Inside `mod ballot`, `Score(7)` compiles fine. So a newtype written in one flat file with no module boundary buys you documentation, not enforcement — the door is there, and the wall around it is not. Put the type behind a `mod` (or its own file, which is the same thing), export `new` and `value`, and keep the field private.

## Two ways to say "0 to 5" — and the free byte

The struct says *"a `u8`, and trust me, I checked."* An enum says it in the type itself:

```rust
pub enum Stars { Zero, One, Two, Three, Four, Five }
```

There is no seventh variant to write, so the illegal value is not merely rejected at the door — it is **unrepresentable**. That has a consequence you can measure:

| Type | `size_of` | `size_of::<Option<T>>` |
|---|---|---|
| `Score(u8)` | 1 | **2** |
| `Stars` (6 variants) | 1 | **1** |

`Score` wraps a `u8` that uses all 256 bit patterns, so `Option` needs a second byte just to record which variant it is. `Stars` uses 6 patterns and leaves 250 spare, so `Option` hides `None` in one of them — the [niche optimization](../option_as_collection/README.md) from the `Option` page, earned here by a design choice rather than luck. The version that makes illegal states unrepresentable is also the smaller one.

Which to reach for: the enum when the legal values are few, named, and stable (a score, a suit, a weekday); the struct newtype when they are a *range* over a primitive you still want to do arithmetic with (a port number, a percentage, an age). Both are the same idea at different strengths.

## If you are coming from another language

- **Python** — the shape you already write is `@dataclass(frozen=True)` with a `__post_init__` that raises, and it works. `typing.NewType` does **not**: it is erased at runtime and checked only by a type-checker you may not be running. The real difference is not the check, it is what happens after it — in Python, `total(scores)` still cannot know whether anyone validated, so defensive re-checking (or trust) is the only option. In Rust, the parameter type *is* the answer.
- **ABAP** — a domain with a value range is the closest thing, and the honest comparison is unflattering: DDIC fixed values are enforced on **dynpro input and by `VALUE CHECK`**, not on a plain `lv_score = 30` in your code. The data element documents the range; ABAP does not stop you leaving it. `Score::new` is the check you would have written by hand at the top of every FORM, hoisted into the type so you cannot forget one.

Both bridges land on the same sentence: everyone already validates at the boundary. What Rust adds is that the *result* of validating is a value you can carry, so the boundary only has to be crossed once.

## Where this lands in a STAR count

The [star-voting-library](https://github.com/masiarek/star-voting-library)'s Python engine does exactly the right thing for Python: one function, [`validate_star_rows(ballots_text, max_score=5)`](https://github.com/masiarek/star-voting-library/blob/master/STARVote_LH_tabulation_engine/starvote_larry_hastings.py), called once on the way in. Everything downstream then handles plain `int`s and trusts that call. The trust is well placed — and it is *trust*, held by the author's memory of the pipeline. Rewriting that in Rust, the newtype is where the trust turns into something the compiler carries for you. It is the first thing a STAR engine needs and the smallest, which is why it is the first rung.

---

## The verified output

<!-- output:newtype_score -->
*Verified output of [`newtype_score.rs`](examples/newtype_score.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: A bare u8 has no opinion about STAR
  total_raw([5, 3, 0])  = 8
  total_raw([5, 30, 0]) = 35
      Both compile, both run, both return a number. The second
      ballot just outvoted ten honest ones, and no type objected.

──── Step 2: Score::new is the only door, and it is checked once
  Score::new(0  ) -> Some(0)
  Score::new(3  ) -> Some(3)
  Score::new(5  ) -> Some(5)
  Score::new(6  ) -> None
  Score::new(30 ) -> None
  Score::new(255) -> None
      Same shape as any partial function: a u8 that is out of
      range has no Score, so the answer is None rather than a lie.

──── Step 3: A whole ballot row, accepted or rejected as a unit
  [5, 3, 0] -> accepted, 3 cells
  [5, 30, 0] -> rejected (one bad cell voids the row)
      collect() into Option<Vec<_>> stops at the first None. One
      illegal cell means you never hold a half-valid ballot.

──── Step 4: Downstream code has nothing left to check
  total(&row)          = 8
  row.iter().max()     = Some(5)
      `total` does no validation because an invalid Score does not
      exist. Ord comes free from the derive, so max/sort behave
      like the numbers they wrap — which is what a runoff needs.

──── Step 5: What actually closes the door is the module boundary
  Inside `mod ballot`, `Score(7)` still compiles — privacy is per
  module, not per type. Outside it, the same expression is a
  compile error: E0423, "cannot initialize a tuple struct which
  contains private fields".
      So the newtype is only as strong as the boundary you put it
      behind. One file with no `mod` is a convention, not a wall.

──── Step 6: The stronger design is also the smaller one
  size_of::<Score>()          = 1
  size_of::<Option<Score>>()  = 2
  size_of::<Stars>()          = 1
  size_of::<Option<Stars>>()  = 1
  Stars::new(4) -> Some(Four) (value Some(4))
  Stars::new(9) -> None
      Score wraps a u8 that uses all 256 patterns, so Option needs
      a second byte for the tag. Stars uses 6 of 256, and Option
      hides None in one of the 250 spare patterns — free.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/newtype_score/examples/newtype_score.rs -o /tmp/ns && /tmp/ns
```

## See also

- [Partial functions](../partial_functions/README.md) — why `Score::new` returns `Option` rather than panicking
- [`Option` is a one-item collection](../option_as_collection/README.md) — the niche that makes `Option<Stars>` free
- [`Option` fields](../option_fields/README.md) — the same "required by default" instinct applied to structs
- [The long way round](../../ROADMAP.md) — where this rung sits, and what comes next
