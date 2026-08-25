# `if let`: one arm, and move on

**Level:** 101 · newcomer

**One line:** `if let` is a `match` with the arm you did not care about deleted — which is exactly what you want, and exactly what stops the compiler from checking you covered everything.

---

## The empty arm

Once a function returns an [`Option`](../option_vs_result/README.md), the compiler wants you to say what happens in both cases. Very often the honest answer for one of them is *nothing at all*:

```rust
match first(&list) {
    Some(x) => println!("The first element is {x}"),
    None => {}                       // an arm written to satisfy the compiler
}
```

That empty arm is noise. It says nothing, and a reader still has to read it to find that out. `if let` is the same code with it removed:

```rust
if let Some(x) = first(&list) {
    println!("The first element is {x}")
}
// moving on...
```

Read it as *"if this value matches this pattern, bind the names in the pattern and run the block."* If it does not match, nothing happens and execution carries straight on. There is no runtime difference between the two versions — this is sugar, and it compiles to the same `match`.

## It really is a `match`, so every pattern works

`Some(x)` is the one you meet first, but nothing about `if let` is `Option`-specific. Anything you can write as a match arm can go on the left:

```rust
if let Some(&Ballot { voter, score }) = ballots.first() { … }   // destructure through the reference
if let Some(Ballot { voter, score: 5 }) = ballots.first() { … } // a literal is a condition, not a binding
if let Some((rank, label)) = pair { … }                          // tuples, nesting, `_`, all as usual
```

And when the other case *does* deserve a line, `else` is right there — at which point you have re-invented the two-arm `match` and either spelling is fine.

## The trade, stated plainly

A `match` over an enum is **exhaustive**: the compiler refuses to build until every variant is accounted for. That is not pedantry, it is the single most useful safety property in the language, and it pays out on the day someone adds a variant — every `match` in the codebase that now has a hole stops compiling and a human decides what to do about each one.

`if let` opts out of that check for one expression. Add a variant, and an `if let` that used to see everything now silently sees less:

```text
match  -> 3 scored, 1 blank, 1 spoiled = 5 of 5 marks
if let -> 3 scored, and 2 marks silently unaccounted for
```

Both of those are the same data. Neither is a bug in Rust; the second is a decision someone made when they wrote `if let` instead of `match`, possibly years before the variant that made it wrong existed.

So the rule of thumb: **`if let` when you genuinely mean "this one shape, and I do not care about the rest"** — a config value that is present or absent, a first element, a parse that worked. **`match` when the cases are a set you are dividing up**, especially over an enum you or your dependencies control. If you would want to be told about a new variant, do not use `if let`.

## Its three relatives

`if let` is one member of a family, and the other three are the answers to "yes, but what about…":

| You want | Write | Why |
|---|---|---|
| one shape, nothing otherwise | `if let Some(x) = v { … }` | the empty arm deleted |
| one shape, or leave this scope | `let Some(x) = v else { return; };` | the happy path stays unindented |
| repeat while the shape keeps matching | [`while let Some(x) = stack.pop() { … }`](../while_let/README.md) | the `None` *is* the loop's exit condition |
| just a `bool` | `matches!(v, Some(n) if n > 4)` | if the answer is a bool, ask for a bool |

[`let … else`](../option_vs_result/README.md) is the one worth internalising early. It inverts `if let`: instead of indenting the work you came to do, it deals with the failure and leaves the binding in scope at the left margin for the whole rest of the function. The `else` block must **diverge** — `return`, `break`, `continue`, `panic!` — which is what lets the compiler treat the binding as unconditionally present afterwards.

`matches!` deserves its own mention because the alternative is so tempting: a `mut` flag declared above an `if let` whose body sets it to `true`. That is four lines for a boolean expression.

## Two things that will bite you

**It binds by moving.** The pattern takes ownership unless you tell it not to, so this does not compile:

```text
error[E0382]: borrow of partially moved value: `name`
3 |     if let Some(n) = name {
  |                 - value partially moved here
6 |     println!("{name:?}");
  |                ^^^^ value borrowed here after partial move
help: borrow this binding in the pattern to avoid moving the value
3 |     if let Some(ref n) = name {
```

The compiler's suggestion works, but the form you will actually write is `if let Some(n) = &name` — match on a reference and the bindings become references too. `.as_ref()` and `.as_deref()` do the same job when you need to chain. Reaching for `.clone()` here is the common reflex and almost always the wrong one.

**Edition 2024 changed when the scrutinee's temporary dies.** If the value you match on builds a temporary, edition 2021 kept that temporary alive to the end of the whole `if let` — *including the `else` block*. Edition 2024 drops it before the `else` runs:

```text
2024:  dropped the temporary   →  else block
2021:  else block              →  dropped the temporary
```

Nobody cares until the temporary is a lock guard, at which point the 2021 order means an `else` block that takes the same lock deadlocks, and the 2024 order means it does not. This is one of the edition's few real behaviour changes, and it is the reason to prefer 2024 for new code rather than a style preference.

## Edition 2024: `if let` chains

Since Rust 1.88, and only in edition 2024, you can chain `let` bindings and conditions in one head, each binding visible to the next:

```rust
if let Some(a) = first_choice
    && let Some(b) = runner_up
    && a != b
{
    println!("runoff between {a} and {b}");
}
```

Under edition 2021 that is a hard error (*"let chains are only allowed in Rust 2024 or later"*) and the same logic has to be written as a staircase of nested `if let`s with the condition at the bottom. If you are reading older Rust and wondering why it is shaped like that, this is why.

## If you are coming from another language

- **Python** — the walrus in `if (x := d.get(k)) is not None:` is the same instinct, and 3.10's `match`/`case` is the same shape again. What changes: the binding exists *only* inside the branch where the pattern held, and inside it the value cannot be the missing one. There is no path on which `x` is `None` and you forgot to check.
- **ABAP** — this is `READ TABLE lt INTO ls ... IF sy-subrc = 0.` welded into a single statement. What changes: you cannot separate the read from the check, cannot forget the check, and cannot touch `ls` on the path where the read failed — the work area does not exist there. The cost is the flip side of the same weld: `if let` also drops the exhaustiveness the `CASE` it replaces would have given you, which is the trade above.

---

## Practice

**The arm you deleted.** Write an exhaustive `match` over a two-variant `Status` enum, and a one-arm `if let` that prints a banner only for the certified case. Then add a third variant to the enum and recompile.

Before you add the variant, also write the version that uses `if let … else` as a stand-in for `match`. Adding the variant makes the compiler come and find the `match`; the `if let` keeps compiling and starts telling a lie. That difference is the whole trade.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:if_let_kata -->
*[`if_let_kata.rs`](examples/if_let_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: what `if let` stops checking.
//!
//!   rustc --edition 2024 if_let_kata.rs -o /tmp/ilk && /tmp/ilk

#[derive(Debug)]
enum Status {
    Counting,
    Certified { winner: &'static str },
    // Added later — the whole point of the exercise. A `match` on Status stops
    // compiling the moment this line appears. Every `if let` keeps compiling.
    Contested { by: &'static str },
}

/// Exhaustive: the compiler made me come back and handle Contested.
fn announce(s: &Status) -> String {
    match s {
        Status::Counting => "still counting".to_string(),
        Status::Certified { winner } => format!("{winner} has won"),
        Status::Contested { by } => format!("result challenged by {by}"),
    }
}

/// One arm, and silence for everything else — which is right here, because
/// "print a banner only when we have a winner" genuinely has one case.
fn banner(s: &Status) {
    if let Status::Certified { winner } = s {
        println!("  ★ {winner} ★");
    }
}

/// The same shape, used wrongly: this one *looks* like it covers the states,
/// and quietly says nothing at all for Contested.
fn misleading(s: &Status) -> String {
    if let Status::Certified { winner } = s {
        format!("{winner} has won")
    } else {
        "still counting".to_string() // a lie for Contested
    }
}

fn main() {
    let states = [
        Status::Counting,
        Status::Certified { winner: "Ada" },
        Status::Contested { by: "Ben" },
    ];

    println!("match — the compiler made me handle the new variant:");
    for s in &states {
        println!("  {:<28} -> {}", format!("{s:?}"), announce(s));
    }

    println!("\nif let, used for what it is good at (one case, silence otherwise):");
    for s in &states {
        banner(s);
    }
    println!("  (only one banner printed, and that was the intent)");

    println!("\nif let/else, used as a substitute for match:");
    for s in &states {
        println!("  {:<28} -> {}", format!("{s:?}"), misleading(s));
    }
    println!("      The last line is wrong, and nothing warned. That is the");
    println!("      exhaustiveness you traded away for the deleted arm.");

    println!("\nlet-else keeps the happy path at the left margin:");
    for s in &states {
        println!("  {}", certified_or_bail(s));
    }
}

fn certified_or_bail(s: &Status) -> String {
    let Status::Certified { winner } = s else {
        return "  (not certified — nothing to print)".to_string();
    };
    format!("  certified: {winner}")
}
```
<!-- /source -->

<!-- output:if_let_kata -->
*Verified output of [`if_let_kata.rs`](examples/if_let_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
match — the compiler made me handle the new variant:
  Counting                     -> still counting
  Certified { winner: "Ada" }  -> Ada has won
  Contested { by: "Ben" }      -> result challenged by Ben

if let, used for what it is good at (one case, silence otherwise):
  ★ Ada ★
  (only one banner printed, and that was the intent)

if let/else, used as a substitute for match:
  Counting                     -> still counting
  Certified { winner: "Ada" }  -> Ada has won
  Contested { by: "Ben" }      -> still counting
      The last line is wrong, and nothing warned. That is the
      exhaustiveness you traded away for the deleted arm.

let-else keeps the happy path at the left margin:
    (not certified — nothing to print)
    certified: Ada
    (not certified — nothing to print)
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:if_let -->
*Verified output of [`if_let.rs`](examples/if_let.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: The empty arm you did not want to write
  match  -> The first element is 10
  if let -> The first element is 10
      Nothing printed for the empty list — `if let` just does not run.
      Same behaviour, same cost, one fewer arm to read.

──── Step 2: It is a `match`, so any pattern works — and it can take an `else`
  struct pattern  -> Ada scored 5
  literal in it   -> Ada gave a maximum score
  tuple pattern   -> rank 1 is the first
  else            -> nothing there, and this arm says so

──── Step 3: The trade: `if let` is not exhaustive
  match  -> 3 scored, 1 blank, 1 spoiled = 5 of 5 marks
  if let -> 3 scored, and 2 marks silently unaccounted for
      The day `Spoiled` was added to the enum, the `match` stopped
      compiling and someone had to decide what to do about it.
      The `if let` compiled fine and quietly went on under-counting.
      That is the price of the deleted arm — pay it deliberately.

──── Step 4: `let … else`: bind, or leave — the guard clause
  populated -> Ada leads with 12
  empty     -> no candidates at all
      `if let` indents the happy path; `let … else` keeps it flat and
      sends the failure out of the function. The `else` block must
      diverge — return, break, continue, or panic — so what follows it
      can rely on the binding existing.

──── Step 5: `while let`: keep going until the pattern stops matching
  popped 30, 2 left
  popped 20, 1 left
  popped 10, 0 left
      `pop()` is a partial function returning Option, so the loop's
      exit condition IS the None — no length check, no index, no
      off-by-one available to get wrong.

──── Step 6: When you only want a bool: `matches!` and `is_some_and`
  matches!(mark, Mark::Score(_))        -> true
  matches!(mark, Mark::Score(n) if n>4) -> false
  score.is_some_and(|n| n > 3)          -> true
      `if let` with an empty body and a flag set inside it is a smell.
      If the answer is a bool, ask for a bool.

──── Step 7: Edition 2024: `if let` chains
  chained -> runoff between Ada and Ben
  chained -> no runoff: one of the two is absent
      Two `if let`s and a condition in one head, each binding visible
      to the next. Stable since Rust 1.88, and only in edition 2024 —
      before that this was a staircase of nested `if let`s.

──── Step 8: It binds by MOVING, unless you ask otherwise
  &name        -> borrowed Ada
  name after   -> Some("Ada")   (still ours)
  as_deref()   -> Ada as a &str, no clone
  moved        -> took ownership of Ada
      Reaching for `.clone()` here is the usual reflex and the usual
      mistake: `&opt`, `.as_ref()`, or `.as_deref()` cost nothing.

──── Step 9: Edition 2024 moved when the scrutinee's temporary dies
  dropped the temporary the scrutinee built
  the else block runs — and the temporary is already gone
      In edition 2021 the drop line came AFTER the else block: the
      temporary lived to the end of the whole `if let`. If that
      temporary is a lock guard, the else block that tries to take
      the same lock deadlocks. Edition 2024 fixed it by dropping
      before the else — one of the edition's few behaviour changes.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/if_let/examples/if_let.rs -o /tmp/il && /tmp/il
```

## See also

- [`Option` vs `Result`](../option_vs_result/README.md) — where these patterns get used, and `let … else` in its natural habitat
- [`while let`](../while_let/README.md) — the same head in a loop, where the pattern failing is how it ends
- [Partial functions](../partial_functions/README.md) — why `pop()` and `first()` return an `Option` for `while let` and `if let` to consume
- [`Option` is a one-item collection](../option_as_collection/README.md) — the other way to handle one arm: `map`, `and_then`, `is_some_and`
- [The Rust Reference on `if let` ↗](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-let-expressions) and [`let` statements with an `else` ↗](https://doc.rust-lang.org/reference/statements.html#let-statements)
