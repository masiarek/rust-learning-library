# Nothing checks a shadow

**Level:** 201 · working knowledge

**One line:** `rustc` has no lint for a shadowed variable — what gets mistaken for protection is the **type error** a wrong shadow trips on its way past, so when the shadow's type matches what it hides there is nothing between you and a wrong answer.

Three pages already cover shadowing here: [Shadowing and `unwrap`](../shadowing_and_unwrap/README.md) on what it is *for*, [A shadow does not drop](../shadowing_does_not_drop/README.md) on what it does to the value underneath, and [When to shadow](../when_to_shadow/README.md) on whether to reach for it at all — with [SHADOWING.md](../../SHADOWING.md) as the map of the set. The first two are about the **mechanism**; the third is a judgement call. This one asks something narrower: what **tooling** stands between you and a shadow that is simply wrong. The reassuring answer is in wide circulation and it is not true:

> "Most downsides of shadowing are negated by the borrow checker. If you got confused around variables, that code is probably not going to compile."

It compiles.

---

## The shadow that compiles clean

```rust
let scores = [5u32, 3, 0, 4];

let total = 0;
for s in scores {
    let total = total + s;
    println!("  running total: {total}");   // 5, then 3, then 0, then 4
}
println!("  final total:   {total}");       // 0
```

`let` inside the loop body makes a **new** `total` on every pass, built from the outer one — which is still `0`, because nothing ever writes to it. Each iteration computes one score, prints it, and drops it at the closing brace.

The running log is the part worth staring at. It is not obviously broken; it is four plausible numbers in a column, and they happen to be the scores rather than a sum. A four-element example makes that visible. A real one with two hundred ballots does not.

`rustc` compiles this with no output at all.

## Why nothing warned

There is no shadowing lint in the compiler. `rustc -W help` mentions "shadow" four times and every one is about trait items, `Deref` supertraits, or glob re-exports — none about a `let`.

Two things come close, and the important thing about both is that neither is a shadow check:

- **`unused_variables`**, but only if the shadow is never *read*. Delete the `println!` from that loop and the lint arrives. Keep any use of the value at all — a log line, a comparison, a push into a vector — and it has nothing to say. The bug survives precisely because the code around it looks like working code.
- **A type mismatch downstream**, if the shadow's type differs from what later lines expect. `E0308: mismatched types`.

The second one is the source of the folklore. Most shadows people write *do* change the type — that is what the feature is for — so most *mistaken* shadows change the type too, and get caught. The catch is a side effect of the type system doing its ordinary job, and it is unrelated to shadowing. Take the type change away and it goes with it.

Which gives the rule worth carrying: **a shadow whose type differs from what it hides is checked; a shadow whose type matches is not.** Same-type shadows are almost all of the dangerous ones, because a same-type shadow is nearly always a mistake — if you genuinely wanted a second `u32`, you wanted a second name.

## The three lints that do see it — and what the useful one costs

Clippy has three, and running each of them against the [kata file](examples/nothing_checks_a_shadow_kata.rs) — which contains one accumulator bug and one perfectly correct parse chain — separates them cleanly:

```text
$ cargo clippy -- -W clippy::shadow_same          # finds nothing
$ cargo clippy -- -W clippy::shadow_unrelated     # finds nothing

$ cargo clippy -- -W clippy::shadow_reuse
warning: `totals` is shadowed
  --> src/main.rs:36:13        <- the bug
warning: `raw` is shadowed
  --> src/main.rs:78:9         <- correct code
warning: `raw` is shadowed
  --> src/main.rs:79:9         <- correct code
```

| Lint | Fires on | On the bug? |
|---|---|---|
| `shadow_same` | `let x = x;`, `let x = &x;` | no — it only ever catches junk |
| `shadow_unrelated` | `let x = something_else;` | **no** — the accumulator reuses `total`, so this lint is silent |
| `shadow_reuse` | `let x = f(x);` | **yes** — and on `let x = x.trim().parse()?` too |

The lint that catches the bug is the one that also condemns the idiom. `shadow_reuse` cannot tell `let total = total + s` inside a loop from `let raw: u32 = raw.parse()?` at the top of a function, because syntactically they are the same move — and the second is what chapter 3.1 of the Book teaches on the page where it introduces the feature.

All three are **allow-by-default `restriction` lints**, which is clippy saying the same thing in its own vocabulary: `restriction` is the group for "you may want to forbid this, and we are not claiming it is wrong." So enabling `shadow_reuse` is a house-style commitment — plausible for a large team, or a codebase that is mostly loops — and not a bug filter you switch on and forget. Enabling it to catch one accumulator will cost you every honest parse chain in the crate.

If you want one of them on, `shadow_unrelated` is the cheapest, because `let config = other_thing;` under an existing `config` is hard to justify. It just will not catch this.

## Items are shadowed too

`fn` and `let` share the **value namespace**, so shadowing is not only about variables — and the two directions get opposite treatment.

A `fn` inside a block shadows one outside it, in total silence:

```rust
fn threshold() -> u32 { 50 }

fn main() {
    println!("{}", threshold());        // 50
    {
        fn threshold() -> u32 { 5 }
        println!("{}", threshold());    // 5   — a different function, no warning
    }
    println!("{}", threshold());        // 50
}
```

A `let` shadowing a `fn` is caught, but only because calling a `u32` is a type error rather than a naming one:

```text
error[E0618]: expected function, found `u32`
 --> fnshadow.rs:6:20
  |
1 | fn seats() -> u32 { 5 }
  | ----------------- this function of the same name is available here, but it's shadowed by the local binding
...
4 |     let seats = 3u32;
  |         ----- `seats` has type `u32`
6 |     println!("{}", seats());
  |                    ^^^^^-- call expression requires function
```

Note rustc's own word for it in that note: *shadowed*. Which settles a question people argue about, below.

## The same shape, where the compiler does stop you

Put an owned value through the identical shadow-in-a-loop and it becomes a hard error:

```rust
let name = String::from("Ada");
let mut jobs: Vec<Box<dyn Fn()>> = Vec::new();
for i in 1..=3 {
    let name = name.clone();   // delete this line and it stops compiling
    jobs.push(Box::new(move || println!("job {i}: {name}")));
}
```

```text
error[E0382]: borrow of moved value: `name`
  | value moved into closure here, in previous iteration of loop
help: consider cloning the value before moving it into the closure
```

That `let name = name.clone();` before a `move` closure is the most common shadow in real Rust — it is in every codebase that spawns tasks — and it is one of the few where forgetting it is caught.

The contrast is the whole page in miniature. Structurally these are the same construct: a `let` in a loop body, same name as something outside. One is a wrong answer with no diagnostic; the other will not build. **The difference is not the shadow, it is what got lost.** Losing a number breaks no rule the compiler knows. Losing a `String` twice over breaks the only rule it cares about most.

(Worth noticing in that `help:` — rustc suggests `let value = name.clone()`, a *different* name. The compiler's own advice is the non-shadowing form; humans write the shadow because it saves inventing `name2`.)

## Why people disagree about whether this is even shadowing

A surprising amount of argument about shadowing is two definitions passing each other, and it matters here because one of them makes the dangerous case invisible.

**The narrow definition** — a binding in an *inner* scope hiding one in an outer scope — is the textbook one, and under it nearly every language shadows: C, C++, Java, Python, JavaScript, Lisp, Perl all let an inner scope reuse a name. Nothing about it is Rust-specific, and nothing about it is interesting.

**The wide definition** — including a second `let` of the same name in the *same* scope — is the one Rust uses. The Book's chapter is titled "Shadowing" and its examples are same-scope. Clippy names its lints `shadow_*` for exactly those cases, as the transcript above shows. rustc's `E0618` note says "shadowed" about a local hiding an item.

The unusual part of Rust is the second one, and it is also where the silent bug lives — the accumulator above is a same-scope shadow inside a loop body. So "shadowing is fine, every language has it" is true of the narrow definition and irrelevant to the risky one, while "that isn't shadowing, that's rebinding" gets the mechanism backwards: `let total = total + s` creates a genuinely *new* variable, and [the old one is still alive](../shadowing_does_not_drop/README.md), not overwritten.

## If you are coming from another language

- **Python.** `total = total + s` inside a loop simply works — one name, rebound, no new variable. Python cannot have this bug in this shape, and its nearest equivalent runs the other way: assign to a name you meant to read from an enclosing scope and you get `UnboundLocalError` at run time, which at least *is* an error. What transfers is the habit of a loop accumulator being a single mutable name; what changes is that Rust offers you a second way to write that line which looks identical and silently does not accumulate.
- **ABAP.** There is no shadowing at all — a `DATA` name is one typed variable for the whole routine, so the compiler resolves `lv_total` to the same storage everywhere and this bug is unwriteable. The familiar shape is a misplaced `CLEAR lv_total` inside the `LOOP` rather than before it: syntax check clean, ATC quiet, and the total comes out as the last row. Same lesson, different mechanism — an accumulator reset once per pass, and no tool that considers it its business.
- **C and C++.** Both refuse a same-block redeclaration outright, so the risky form is unwriteable, and both offer `-Wshadow` (off by default) for the nested-scope kind. That is the more interesting comparison: C treats shadowing as a *suspected mistake* and gives you one switch, Rust treats it as an idiom and gives you three, all off. The sibling page walks through [what C and C++ do with the same program](../shadowing_does_not_drop/README.md#what-c-and-c-do-with-the-same-program).
- **Java.** Often said not to allow shadowing; the claim is about *locals* only. Java forbids redeclaring a local inside a nested block of the same method, but JLS §6.4.1 is literally titled "Shadowing" and a parameter shadowing a field is the most common pattern in the language — it is why constructors say `this.x = x`. What Java lacks is the same-scope form, which is the one this page is about.

---

## Practice

**The tally that never tallied.** Write a small score count: three or four ballots of 0–5 scores over three candidates, an accumulator outside the loop, a per-ballot log line inside it, and a `winner` function that returns the highest-scoring candidate. Introduce the bug from the top of this page — `let totals = …` inside the loop — and run it.

Read the output before reading the code. Decide whether you would have caught it in review.

Then go looking for a tool that catches it: build it, read every warning, and run each of clippy's three shadow lints in turn. One of them finds it. Work out what *else* that one flags in the same file, and decide whether you would turn it on.

Then fix it three ways and pick one to ship.

Worth getting wrong on purpose: delete the log line from inside the loop, rebuild, and watch a warning appear that was not there a moment ago — then put the line back and watch it leave. That is the whole margin you were relying on.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:nothing_checks_a_shadow_kata -->
*[`nothing_checks_a_shadow_kata.rs`](examples/nothing_checks_a_shadow_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the tally that never tallied.
//!
//! A shadowed accumulator inside a loop. The compiler says nothing, the
//! per-ballot log looks fine, the report prints a real candidate's name — and
//! the number behind that name is zero. Then: which of clippy's three shadow
//! lints finds it, and what that one costs you elsewhere in this same file.
//!
//!   rustc --edition 2024 nothing_checks_a_shadow_kata.rs -o /tmp/ncask && /tmp/ncask

const CANDIDATES: [&str; 3] = ["Ada", "Ben", "Cara"];

/// Three ballots, 0–5 scores. Ben is the honest winner: 3 + 5 + 2 = 10,
/// against Ada's 9 and Cara's 6.
const BALLOTS: [[u32; 3]; 3] = [[5, 3, 0], [4, 5, 1], [0, 2, 5]];

fn banner(title: &str) {
    println!("\n──── {title}");
}

fn winner(totals: [u32; 3]) -> &'static str {
    let mut best = 0;
    for i in 1..totals.len() {
        if totals[i] > totals[best] {
            best = i;
        }
    }
    CANDIDATES[best]
}

// ─────────────────────────────────────────────────────────────── the bug
fn tally_buggy() -> [u32; 3] {
    let totals = [0u32; 3];
    for ballot in BALLOTS {
        // Reads the OUTER `totals` — which is still [0, 0, 0] — adds one
        // ballot, and drops the result at the closing brace.
        let totals = [
            totals[0] + ballot[0],
            totals[1] + ballot[1],
            totals[2] + ballot[2],
        ];
        println!("  counted {ballot:?}  running total {totals:?}");
    }
    totals
}

// ─────────────────────────────────────────────────────────────── the fixes
fn tally_mut() -> [u32; 3] {
    let mut totals = [0u32; 3];
    for ballot in BALLOTS {
        for i in 0..totals.len() {
            totals[i] += ballot[i];
        }
    }
    totals
}

fn tally_fold() -> [u32; 3] {
    BALLOTS.iter().fold([0u32; 3], |mut acc, ballot| {
        for i in 0..acc.len() {
            acc[i] += ballot[i];
        }
        acc
    })
}

fn tally_distinct_names() -> [u32; 3] {
    let mut running = [0u32; 3];
    for ballot in BALLOTS {
        for i in 0..running.len() {
            running[i] += ballot[i];
        }
    }
    running
}

// A CORRECT shadow, in the same file, for the lint to have an opinion about.
fn parse_quorum(raw: &str) -> u32 {
    let raw = raw.trim(); // &str -> &str
    let raw: u32 = raw.parse().unwrap_or(0); // &str -> u32, the Book's own idiom
    raw
}

fn main() {
    // ────────────────────────────────────────────────────────── 1
    banner("As shipped: a log that looks fine and a winner that is not");
    let totals = tally_buggy();
    println!("  Winner: {}", winner(totals));
    println!("      Nothing above looks alarming. Every ballot was counted, each");
    println!("      running total is a real number, and Ada is a real candidate.");
    println!("      Two things give it away, and only in hindsight:");
    println!("  totals = {totals:?}");
    println!("      Every 'running total' was just that ballot echoed back, and");
    println!("      the accumulator never left zero. `winner` broke the all-zero");
    println!("      tie by index, so the report named whoever was first in the");
    println!("      candidate list. The honest winner is Ben, with 10.");

    // ────────────────────────────────────────────────────────── 2
    banner("What the compiler had to say about it: nothing");
    println!("  $ rustc --edition 2024 nothing_checks_a_shadow_kata.rs");
    println!("  $                                    <- no output, exit 0");
    println!("      `unused_variables` cannot fire: the shadow is read on the");
    println!("      next line, by the log. There is no type error: both are");
    println!("      [u32; 3]. And there is no shadowing lint in rustc to fire in");
    println!("      the first place. Three near-misses, all accidents of shape —");
    println!("      drop the log line and the first one would have caught it.");

    // ────────────────────────────────────────────────────────── 3
    banner("Which clippy lint finds it (recorded from a real run on this file)");
    println!("  (clippy needs a cargo crate, so this file is src/main.rs there)");
    println!("  $ cargo clippy -- -W clippy::shadow_same");
    println!("  $                                    <- finds NOTHING");
    println!();
    println!("  $ cargo clippy -- -W clippy::shadow_unrelated");
    println!("  $                                    <- finds NOTHING");
    println!();
    println!("  $ cargo clippy -- -W clippy::shadow_reuse");
    println!("  warning: `totals` is shadowed");
    println!("    --> src/main.rs:36:13        <- the bug");
    println!("  warning: `raw` is shadowed");
    println!("    --> src/main.rs:78:9         <- parse_quorum, line 1");
    println!("  warning: `raw` is shadowed");
    println!("    --> src/main.rs:79:9         <- parse_quorum, line 2");
    println!("      parse_quorum(\"  42 \") = {}", parse_quorum("  42 "));
    println!("      ...which is correct code, flagged twice. It is the idiom");
    println!("      chapter 3.1 of the Book teaches.");

    // ────────────────────────────────────────────────────────── 4
    banner("The trade, stated plainly");
    println!("  shadow_same       `let x = x;`             junk always, the bug never");
    println!("  shadow_unrelated  `let x = something_else;` silent here");
    println!("  shadow_reuse      `let x = f(x);`          catches it — and the idiom");
    println!("      The only lint that would have caught this bug is the one that");
    println!("      also condemns the good use of the feature. All three are");
    println!("      allow-by-default `restriction` lints, which is clippy saying");
    println!("      the same thing in its own vocabulary: a style commitment, not");
    println!("      a bug filter. Turn `shadow_reuse` on and you have banned");
    println!("      `let x = x.trim().parse()?` across the crate. That can be a");
    println!("      trade worth making — a large team, a codebase full of loops —");
    println!("      but make it deliberately, not hoping to catch one accumulator.");

    // ────────────────────────────────────────────────────────── 5
    banner("Three fixes, and the one to ship");
    let (a, b, c) = (tally_mut(), tally_fold(), tally_distinct_names());
    println!("  1. `mut`, no shadow      -> {a:?}  winner {}", winner(a));
    println!("  2. fold, no accumulator  -> {b:?}  winner {}", winner(b));
    println!("  3. no name reused        -> {c:?}  winner {}", winner(c));
    println!("      Fix 2 is the one to ship, and the reason generalises past this");
    println!("      bug: it removes the BINDING, so the mistake has nowhere to");
    println!("      live. Fix 1 works, and is the honest counter-example to");
    println!("      'shadowing lets you keep everything immutable' — an");
    println!("      accumulator is supposed to survive the iteration, so a fresh");
    println!("      binding per pass is precisely the wrong tool. Fix 3 works and");
    println!("      relies on you not reusing a name, which is the discipline that");
    println!("      just failed.");

    println!("\n      The through-line: the compiler polices ownership, types and");
    println!("      exhaustiveness, and a shadow can be wrong in none of those");
    println!("      ways. When a shadow's type matches what it hides, you are the");
    println!("      only check there is.");
}
```
<!-- /source -->

<!-- output:nothing_checks_a_shadow_kata -->
*Verified output of [`nothing_checks_a_shadow_kata.rs`](examples/nothing_checks_a_shadow_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── As shipped: a log that looks fine and a winner that is not
  counted [5, 3, 0]  running total [5, 3, 0]
  counted [4, 5, 1]  running total [4, 5, 1]
  counted [0, 2, 5]  running total [0, 2, 5]
  Winner: Ada
      Nothing above looks alarming. Every ballot was counted, each
      running total is a real number, and Ada is a real candidate.
      Two things give it away, and only in hindsight:
  totals = [0, 0, 0]
      Every 'running total' was just that ballot echoed back, and
      the accumulator never left zero. `winner` broke the all-zero
      tie by index, so the report named whoever was first in the
      candidate list. The honest winner is Ben, with 10.

──── What the compiler had to say about it: nothing
  $ rustc --edition 2024 nothing_checks_a_shadow_kata.rs
  $                                    <- no output, exit 0
      `unused_variables` cannot fire: the shadow is read on the
      next line, by the log. There is no type error: both are
      [u32; 3]. And there is no shadowing lint in rustc to fire in
      the first place. Three near-misses, all accidents of shape —
      drop the log line and the first one would have caught it.

──── Which clippy lint finds it (recorded from a real run on this file)
  (clippy needs a cargo crate, so this file is src/main.rs there)
  $ cargo clippy -- -W clippy::shadow_same
  $                                    <- finds NOTHING

  $ cargo clippy -- -W clippy::shadow_unrelated
  $                                    <- finds NOTHING

  $ cargo clippy -- -W clippy::shadow_reuse
  warning: `totals` is shadowed
    --> src/main.rs:36:13        <- the bug
  warning: `raw` is shadowed
    --> src/main.rs:78:9         <- parse_quorum, line 1
  warning: `raw` is shadowed
    --> src/main.rs:79:9         <- parse_quorum, line 2
      parse_quorum("  42 ") = 42
      ...which is correct code, flagged twice. It is the idiom
      chapter 3.1 of the Book teaches.

──── The trade, stated plainly
  shadow_same       `let x = x;`             junk always, the bug never
  shadow_unrelated  `let x = something_else;` silent here
  shadow_reuse      `let x = f(x);`          catches it — and the idiom
      The only lint that would have caught this bug is the one that
      also condemns the good use of the feature. All three are
      allow-by-default `restriction` lints, which is clippy saying
      the same thing in its own vocabulary: a style commitment, not
      a bug filter. Turn `shadow_reuse` on and you have banned
      `let x = x.trim().parse()?` across the crate. That can be a
      trade worth making — a large team, a codebase full of loops —
      but make it deliberately, not hoping to catch one accumulator.

──── Three fixes, and the one to ship
  1. `mut`, no shadow      -> [9, 10, 6]  winner Ben
  2. fold, no accumulator  -> [9, 10, 6]  winner Ben
  3. no name reused        -> [9, 10, 6]  winner Ben
      Fix 2 is the one to ship, and the reason generalises past this
      bug: it removes the BINDING, so the mistake has nowhere to
      live. Fix 1 works, and is the honest counter-example to
      'shadowing lets you keep everything immutable' — an
      accumulator is supposed to survive the iteration, so a fresh
      binding per pass is precisely the wrong tool. Fix 3 works and
      relies on you not reusing a name, which is the discipline that
      just failed.

      The through-line: the compiler polices ownership, types and
      exhaustiveness, and a shadow can be wrong in none of those
      ways. When a shadow's type matches what it hides, you are the
      only check there is.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:nothing_checks_a_shadow -->
*Verified output of [`nothing_checks_a_shadow.rs`](examples/nothing_checks_a_shadow.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: The shadow that compiles clean
  running total: 5
  running total: 3
  running total: 0
  running total: 4
  final total:   0
      The running totals are 5, 3, 0, 4 — the scores themselves,
      not a sum. Each iteration read the outer `total` (still 0),
      added one score, and threw the result away at the brace.
      Zero warnings. `rustc` compiled this without a word.

──── Step 2: Why nothing warned
  There is no shadowing lint in rustc. `rustc -W help` mentions
  'shadow' four times and every one is about trait items or glob
  re-exports — none about a `let`.
      Two things nearly catch it, and neither is a shadow check:
      * `unused_variables`, if the shadow is never READ. Read it
        once inside the loop — as the println above does — and
        that lint has nothing to say.
      * a TYPE MISMATCH downstream, if the shadow's type differs
        from what later code expects: `E0308: mismatched types`.
      Both are accidents. Same type, read once: total silence.

──── Step 3: Items get shadowed too, just as quietly
  outer threshold() -> 50
  inner threshold() -> 5   <- a different function
  after the block   -> 50
      A `fn` inside a block shadows one outside it, with no
      diagnostic at all. `fn` and `let` share the VALUE namespace,
      so a variable can hide a function too — and that one rustc
      does catch, because calling a u32 is a type error:
        error[E0618]: expected function, found `u32`
          | this function of the same name is available here,
          | but it's shadowed by the local binding
      Note rustc's own word for it there: shadowed.

──── Step 4: The same shape, where the compiler DOES stop you
  job 1: Ada
  job 2: Ada
  job 3: Ada
  original still here: Ada
      Delete that `let name = name.clone();` and it does not
      compile: E0382, 'value moved into closure here, in previous
      iteration of loop'. Structurally the SAME shadow-in-a-loop as
      step 1 — and this one is a hard error.
      The difference is not the shadow. It is that ownership is
      checked and arithmetic is not. Step 1 lost a number, which
      no rule forbids; step 4 would have lost a String twice over,
      which every rule forbids.

──── Step 5: Three ways to write step 1 so it cannot go quiet
  1. `mut`, no shadow          -> 12
  2. no accumulator at all     -> 12
  3. a name that cannot collide-> 12
      This is the one place `mut` genuinely beats shadowing, and
      it is the opposite of the usual advice: an accumulator is
      supposed to survive the iteration, so the thing shadowing
      gives you — a fresh binding each time — is exactly the bug.
      Option 2 is the one to ship: with no binding to shadow,
      the mistake has nowhere to live.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/nothing_checks_a_shadow/examples/nothing_checks_a_shadow.rs -o /tmp/ncas && /tmp/ncas
```

## Traps

- **Believing the borrow checker is watching.** It watches ownership. A shadow that loses a number violates nothing it knows about, and the identical shadow that would lose a `String` is a hard error — same construct, opposite outcome, and the shadow is not what decided it.
- **`let` inside a loop body, when you meant to accumulate.** The single most common way this bites. An accumulator has to outlive the iteration, and a fresh binding per pass is the exact opposite of that.
- **Reading a clean build as a clean bill of health.** Same type plus one use of the value equals silence. The absence of a warning here carries no information.
- **Turning on `shadow_reuse` to catch one bug.** It also flags `let x = x.trim().parse()?`, which is the idiom the Book teaches. Enable it as a style decision for a whole codebase, or not at all.
- **Assuming `shadow_unrelated` is the strict one.** Its name suggests it catches the careless cases, but an accumulator *reuses* the shadowed value, so it stays silent on exactly the bug you want.
- **Shadowing a name that is also a function.** In an inner block a `fn` shadows an outer `fn` with no diagnostic; the reverse is caught only as `E0618`, and only when you actually call it.

## See also

- [Shadowing and `unwrap`](../shadowing_and_unwrap/README.md) — what shadowing is *for*, and the type change that makes it worth having
- [A shadow does not drop](../shadowing_does_not_drop/README.md) — what happens to the value the shadow hid: nothing, until the end of the scope
- [When to shadow](../when_to_shadow/README.md) — the judgement call this page leaves open: what the feature buys that `mut` cannot, the idioms worth copying, and the three bugs that compile
- [SHADOWING.md](../../SHADOWING.md) — the map of all four shadowing lessons, in reading order
- [What a warning is asking](../what_a_warning_is_asking/README.md) — `unused_variables`, the near-miss above, and what `_name` actually answers
- [Ownership and moves](../ownership_and_moves/README.md) — the rule that *is* enforced, and why it turns the same shape into `E0382`
- [`shadow_reuse`](https://rust-lang.github.io/rust-clippy/master/index.html#shadow_reuse), [`shadow_same`](https://rust-lang.github.io/rust-clippy/master/index.html#shadow_same), [`shadow_unrelated`](https://rust-lang.github.io/rust-clippy/master/index.html#shadow_unrelated) — the three lints, and the `restriction` group they sit in
- [The Rust Book, ch. 3.1 — Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)
