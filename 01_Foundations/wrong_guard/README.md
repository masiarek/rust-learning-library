# Zero wins is not zero games

**Level:** 201 · working knowledge

**One line:** Returning `Result` proves the author knew the function could fail; it does not prove they guarded the input that actually has no answer — and the one they missed leaves through `Ok` as a plausible number.

Here is a function of the kind that turns up in every error-handling chapter, reconstructed. A global table holds each team's wins and losses; the caller wants a winning ratio:

```rust
fn winning_ratio(team: &String) -> Result<f32, &str> {
    if let Some(&(wins, losses)) = TEAMS.get(team.as_str()) {
        if wins == 0 {
            Ok(wins as f32)
        } else {
            Ok(((wins + losses) as f32) / wins as f32)
        }
    } else {
        Err("Invalid team")
    }
}
```

It compiles. It returns a `Result`. It handles the missing key. It even has a special case for the awkward input. And on a table holding **Bears 9-8**, **Lions 0-17**, and an expansion team at **0-0**, it prints this:

```text
     Bears -> Ok(1.8888888)
     Lions -> Ok(0.0)
 Expansion -> Ok(0.0)
    Sharks -> Err("Invalid team")
```

Two of those four lines are wrong, and the `Result` did not stop either one.

---

## The bug the type system was never going to catch

A team that goes 9-8 has won **nine of seventeen** — `0.529`. The listing computes `(wins + losses) / wins`, which is `17 / 9 = 1.889`: games *per win*, the ratio upside down. Both operands are `f32` and so is the return, so every version of this function type-checks equally well. Rust checks that you combined the right *kinds* of thing; it has no opinion about which way up you divided them.

Worth saying plainly because it is the honest limit of the whole toolkit: **`Result` protects the shape of an answer, never its arithmetic.** One line fixes it — and then the interesting bug is still there.

## The guard is real. It is on the wrong condition.

Look again at the special case. The author knew this function was [partial](../partial_functions/README.md) — undefined somewhere — because they wrote a branch for it. They just picked the wrong somewhere:

| Team | Record | `wins == 0`? | Is there an answer? |
|---|---|---|---|
| Lions | 0-17 | yes | **Yes** — `0 / 17` is `0.0`, and that is the true ratio |
| Expansion | 0-0 | yes | **No** — no games played, so no fraction exists |

Zero wins is *unusual*, and it caught the eye. Zero **games** is *undefined*, and it did not. Both land in the same branch, which returns `Ok(wins as f32)` — a hard-coded `0.0` dressed as a computed result. So the one input in the table with no answer is the one input that got a confident number, and it is indistinguishable downstream from a team that genuinely lost every game.

That is the shape worth memorising, because it long outlives this listing:

> **When a guard fires, ask what the branch returns.** If it returns a value rather than an error, the function has stopped being partial-but-honest and started fabricating. `0`, `0.0`, `-1` and `""` are the four values that fabrication usually wears.

The tell is that the guard and the failure are answering different questions. `wins == 0` is a statement about a *number being small*. "This team has no ratio" is a statement about a *fraction having no denominator*. Guard the denominator.

## The fix, at two sizes

Smallest version that stops it lying — right formula, guard moved to `played == 0`:

```rust
fn winning_ratio(team: &str) -> Result<f32, &'static str> {
    let &(wins, losses) = TEAMS.get(team).ok_or("no such team")?;
    let played = wins + losses;
    if played == 0 {
        return Err("that team has not played yet");
    }
    Ok(wins as f32 / played as f32)
}
```

Three smaller repairs rode along, and each is worth knowing on its own:

- **`&str`, not `&String`.** A `&String` accepts strictly less (every caller must own a `String` first) and buys nothing.
- **`&'static str`, not `&str`, in the error slot.** With bare `&str` the elided lifetime ties the error to the *argument*, so the message appears to borrow from the team name it is complaining about. The message is a literal; say so.
- **`.ok_or(…)?` instead of `if let … else`.** [Crossing from `Option` to `Result`](../option_vs_result/README.md) is one call, and `?` keeps the happy path at the left margin.

Then the version the surrounding chapter is really arguing for. There are **two** ways to fail here, and a caller would act differently on each:

```rust
#[derive(Debug, PartialEq)]
enum RatioError {
    NoSuchTeam(String),
    NoGamesPlayed,
}
```

An unknown team is the *caller's* mistake — ask them to check the spelling. An unplayed season is the *world's* state — print a dash in the table and move on. With `&str` errors the caller can only print them; with the enum it can `match` and do the right thing for each. That is the same test the [`Option` vs `Result`](../option_vs_result/README.md) page uses, applied one level down: it is not just *can the caller ask why?*, it is *would the caller do something different?*

### If you are coming from another language

- **Python.** The straight translation of the original raises nothing and returns `0.0` for a team with no games, exactly as here — the bug is not Rust's and Rust does not catch it. What changes is where you *put* the answer: Python's habit is `return None` or a bare `raise ValueError("bad team")`, and both collapse the two failures into one. `RatioError` is the `ValueError` subclass hierarchy you were unlikely to bother writing.
- **ABAP.** This is the "returns 0 and sets no `sy-subrc`" bug, which is the hardest kind to find in an ABAP report precisely because 0 is a legal amount. Moving the guard from `wins = 0` to `played = 0` is the same discipline as checking `IF lines( lt_games ) = 0` before dividing — except the compiler now forces the caller to open the result, so the zero cannot quietly reach a total.

---

## The verified output

Every line below came from compiling and running [`wrong_guard.rs`](examples/wrong_guard.rs) — including the broken first version, which is kept in the file precisely so the wrong numbers on this page are real ones.

<!-- output:wrong_guard -->
*Verified output of [`wrong_guard.rs`](examples/wrong_guard.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: It compiles, it returns Result, and it is wrong twice
       Bears -> Ok(1.8888888)
       Lions -> Ok(0.0)
   Expansion -> Ok(0.0)
      Sharks -> Err("Invalid team")
      Bears are 9-8. A winning ratio of 1.89 is not a ratio at all:
      (wins + losses) / wins is games PER WIN. Both sides are f32, so
      the compiler has nothing to object to. Arithmetic is on you.

──── Step 2: The guard is real — it is just on the wrong condition
  Lions     0-17  wins == 0, and 0/17 = 0.000 is a TRUE answer
  Expansion 0-0   no games played, so there is NO answer to give
      The author guarded `wins == 0`, which is unusual but answerable,
      and left `wins + losses == 0`, which is undefined, unguarded.
      So the one input with no answer is the one that got a number.

──── Step 3: Move the guard to the undefined case
       Bears -> Ok(0.5294118)
       Lions -> Ok(0.0)
   Expansion -> Err("that team has not played yet")
      Sharks -> Err("no such team")
      Lions keep their 0.0, because 0 wins in 17 games IS zero.
      Expansion now fails, because 0 wins in 0 games is not a number.

──── Step 4: Two failures the caller treats differently
       Bears -> 0.529
   Expansion -> print a dash, not a 0: the season has not started for that team
      Sharks -> typo, ask again: "Sharks" is not a team we track
      A bad lookup is the caller's mistake; an unplayed season is the
      world's. Only a named E lets the caller tell them apart.

──── Step 5: What would have caught it
       Bears expected 0.5294  got 0.5294  ok
       Lions expected 0.0000  got 0.0000  ok
  Expansion is_err -> true
      Not the compiler: every version above type-checks. What catches
      an inverted formula is one hand-computed expectation, and what
      catches a missing guard is a test at the boundary — 0 games.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/wrong_guard/examples/wrong_guard.rs -o /tmp/wg && /tmp/wg
```

---

## Traps

- **A guard whose branch returns a value.** The whole page. If the condition means *"there is no answer"*, the branch must return `Err` (or `None`), never a stand-in.
- **Guarding the numerator when the denominator is the problem.** `wins == 0` versus `wins + losses == 0`. Ask which quantity would make the arithmetic meaningless, and test *that*.
- **Trusting a signature because it says `Result`.** A `Result` return is evidence the author thought about failure once, not that they enumerated it. Read the `Ok` arm before you believe it.
- **`0` as a legal value and as a sentinel in the same function.** Once `0.0` can mean both "lost every game" and "no games", no caller can recover the difference. If a type must carry both, that is what `Option<f32>` inside the `Ok` is for.
- **A test suite with no boundary row.** Bears and Lions both pass against the *broken* guard. Only the 0-0 team distinguishes the two versions, and it is the row nobody thinks to write.

## See also

- [Partial functions](../partial_functions/README.md) — the general case: a function undefined over part of its input range, and what widening the return type buys
- [`Option` vs `Result`](../option_vs_result/README.md) — which of the two to reach for, and designing the `E`
- [Returning `None` on error](../none_on_error/README.md) — the neighbouring downgrade: a real reason thrown away rather than a fake answer invented
- [What a panic costs](../what_a_panic_costs/README.md) — the other end of the same branch: what it costs when the wrong path wins loudly instead of quietly, and why unwinding tidies your memory but not your half-done work
