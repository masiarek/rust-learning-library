# `map_or` and `map_or_else`: transform, or fall back

**Level:** 201 · working knowledge

**One line:** `map` and a fallback fused into one call — with the default written *first* and run *last*, which is why clippy pushes you into these two from one side and straight back out of them from the other.

```rust
opt.map(f).unwrap_or(d)       ==  opt.map_or(d, f)
opt.map(f).unwrap_or_else(g)  ==  opt.map_or_else(g, f)
```

Four spellings, two behaviours. The whole page is about which of the four to write, and there is a real boundary — not a style preference — on both sides of `map_or`.

---

## The reason to reach for it: the type may change

`unwrap_or` can only ever hand you a `T`. `map_or` hands you a `U`:

```rust
let described: String = score.map_or("no score".to_string(), |v| format!("{v} stars"));
```

That is the case it exists for — when the fallback and the transformed value are the same *kind* of answer (a label, a percentage, a line of a report) but not the type you started with. If `U` and `T` are the same type and there is no transformation, you wanted `unwrap_or`.

## The default is written first and runs last

```rust
match quorum {                      quorum.map_or(100, |q| q * 2)
    Some(q) => q * 2,
    None => 100,
}
```

Same answer, opposite reading order: the `match` names the happy case first, `map_or` names the fallback first. Nothing will train that out of you, so the useful fact is the consolation prize — **swapping the two arguments is a type error, not a silent bug.** The cost of the surprise is one compile, not one wrong report.

## On a `Result`, the error closure comes first

```rust
r.map_or_else(|e| format!("skipped — {e}"), |v| format!("counted {v}"))
```

Read the signature rather than the name: `map_or_else(default, f)`, and on a `Result` the "default" is the closure taking `E`. So the **sad path is written on the left** — the reverse of every `match` you have written, where `Ok` comes first. This is the one member of the family worth re-reading at the call site every single time.

It is also the only shape here that sees the error at all: `map_or` on a `Result` discards it exactly as `unwrap_or` does.

## Eager and lazy, same rule as before

`map_or(expensive(), f)` builds the fallback on every call, needed or not; `map_or_else(expensive, f)` builds it only on the sad path. The difference in the source is a pair of parentheses — `expensive()` versus `expensive` — which is a very small visual signal for a real cost. [The same rule, with the counter that proves it.](../unwrap_or/README.md#the-eager-trap)

## Where clippy pushes you in, and where it pushes you out

Both lints are worth knowing by name, because together they mark the boundary of what `map_or` is for.

**In** — `clippy::map_unwrap_or` (pedantic):

```text
warning: called `map(<f>).unwrap_or(<a>)` on an `Option` value
help: use `map_or(<a>, <f>)` instead
3 -     let a = s.map(|v| v * 2).unwrap_or(0);
3 +     let a = s.map_or(0, |v| v * 2);
```

**Out** — `clippy::unnecessary_map_or`, which is on **by default**:

```text
warning: this `map_or` can be simplified
help: use `is_some_and` instead
5 -     let c = s.map_or(false, |v| v > 5);
5 +     let c = s.is_some_and(|v| v > 5);
```

The two are not in conflict. When the fallback is a **value**, `map_or` beats writing `map` and `unwrap_or` separately. When the fallback is `false` or `true`, you were never defaulting at all — you were asking a yes/no question, and the named methods say so in words:

| You wrote | Mean | Write instead |
|---|---|---|
| `opt.map_or(false, pred)` | "present **and** passes" | `opt.is_some_and(pred)` |
| `opt.map_or(true, pred)` | "absent **or** passes" | `opt.is_none_or(pred)` |

`is_some_and` landed in Rust 1.70, `is_none_or` in 1.82 — recent enough that plenty of live code still spells them the long way, and old enough that new code has no excuse.

## Where a `match` still wins

`map_or` is for **two short expressions producing one value**:

```rust
b.score.map_or("—".to_string(), |s| format!("{s}/5"))
```

It is the wrong tool the moment either branch does more than that. A closure that has to mutate its environment to be useful is a `match` wearing a disguise:

```rust
match b.score {
    Some(s) => { counted += 1; total += u32::from(s); }
    None => abstained += 1,
}
```

And neither closure can `?` or `return` out of the enclosing function — a closure's `return` returns from the closure. So anything with an early exit is a `match` or an `if let`, and that one is not a matter of taste.

## If you are coming from another language

- **Python** — `f(x) if x is not None else d` is `map_or` with the arguments in the order you actually want: transform first, fallback last, condition in the middle. Rust's version is shorter and reads backwards; that is the trade.
- **JavaScript** — `x?.f() ?? d` is the closest thing, and it reads happy-path-first too. Worth noticing that both of these languages put the fallback where you would say it out loud, and Rust does not; the argument order is a consequence of `map_or` being a method on the wrapper rather than a piece of syntax.
- **ABAP** — `COND #( WHEN lv_score IS NOT INITIAL THEN |{ lv_score }/5| ELSE '—' )` is the same expression, and it has the same trap in a different place: `IS NOT INITIAL` conflates "no score" with "scored zero", which is exactly what an `Option<u8>` keeps apart. The transformation is the easy half; what travels badly is the *test*.

---

## Practice

**Transform, or fall back.** Turn an `Option<Winner>` into a `String` label in one call: the winner's name and margin when there is one, `"no result yet"` when there is not. Then find a case on the same data where a `match` is still the better tool.

Try `unwrap_or` first and watch it refuse: its fallback must already be the type of the value inside, and yours is not. Then read your own `map_or` line back and note where the default sits — written first, run last — which is the whole reason clippy will push you back out of it once a third case appears.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:map_or_kata -->
*[`map_or_kata.rs`](examples/map_or_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: transform when present, fall back when not — in one call.
//!
//!   rustc --edition 2024 map_or_kata.rs -o /tmp/mork && /tmp/mork

#[derive(Debug)]
struct Winner {
    name: &'static str,
    margin: u32,
}

fn label(w: &Option<Winner>) -> String {
    // The default is written FIRST and used LAST. Read it as:
    // "…or this, when there is nothing to transform".
    w.as_ref().map_or("no result yet".to_string(), |w| {
        format!("{} by {} votes", w.name, w.margin)
    })
}

fn main() {
    let counted = Some(Winner { name: "Ada", margin: 17 });
    let pending: Option<Winner> = None;

    println!("The type changes, which is the reason to reach for map_or:");
    println!("  {}", label(&counted));
    println!("  {}", label(&pending));
    println!("      Option<Winner> in, String out. unwrap_or cannot do that —");
    println!("      its fallback must already be the same type as the value.");

    println!("\nEager and lazy, the same rule as the unwrap family:");
    let seats: Option<u32> = Some(3);
    println!("  map_or       -> {}", seats.map_or(expensive_default(), |s| s * 2));
    println!("  map_or_else  -> {}", seats.map_or_else(expensive_default_lazy, |s| s * 2));

    println!("\nWhere a match still wins — when both arms have something to say:");
    match &counted {
        Some(w) if w.margin < 10 => println!("  {} won, but call it a recount", w.name),
        Some(w) => println!("  {} won comfortably ({} votes)", w.name, w.margin),
        None => println!("  still counting"),
    }
    println!("      Three outcomes and a guard. Squeezing that into map_or would");
    println!("      cost more than the line it saves.");
}

fn expensive_default() -> u32 {
    println!("      …building the default (ran even though seats was Some)");
    0
}

fn expensive_default_lazy() -> u32 {
    println!("      …building the default (you will not see this line)");
    0
}
```
<!-- /source -->

<!-- output:map_or_kata -->
*Verified output of [`map_or_kata.rs`](examples/map_or_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The type changes, which is the reason to reach for map_or:
  Ada by 17 votes
  no result yet
      Option<Winner> in, String out. unwrap_or cannot do that —
      its fallback must already be the same type as the value.

Eager and lazy, the same rule as the unwrap family:
      …building the default (ran even though seats was Some)
  map_or       -> 6
  map_or_else  -> 6

Where a match still wins — when both arms have something to say:
  Ada won comfortably (17 votes)
      Three outcomes and a guard. Squeezing that into map_or would
      cost more than the line it saves.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:map_or -->
*Verified output of [`map_or.rs`](examples/map_or.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: One call instead of two — and the type may change
  Some(4)  map(|v| v*25).unwrap_or(0) = 100 map_or(0, |v| v*25) = 100
  None     map(|v| v*25).unwrap_or(0) = 0   map_or(0, |v| v*25) = 0
  Option<u8> -> String: "4 stars" / "no score"
      unwrap_or can only give you a T. map_or gives you a U, so it is the
      one to reach for when the fallback and the transformed value are the
      same KIND of answer — a label, a percentage, a row of a report — but
      not the type you started with.

──── Step 2: The default is written first and runs last
  match: Some(q) => q*2, None => 100   -> 80
  quorum.map_or(100, |q| q * 2)        -> 80
      Same answer, opposite reading order: the match names the happy case
      first, map_or names the fallback first. Nothing enforces the habit —
      but swapping the two arguments is a type error, not a silent bug, so
      the cost of the surprise is one compile, not one wrong report.

──── Step 3: On a Result, the ERROR closure comes first
  Ok(4)    map_or_else(|e| .., |v| ..) -> counted 4
  Err(..)  map_or_else(|e| .., |v| ..) -> skipped — row 7: '4x' is not a number
      Read the signature, not the name: map_or_else(default, f), and on a
      Result the 'default' is the closure taking E. So the sad path is
      written on the left, which is the reverse of every match you have
      written, where Ok comes first. This is the one member of the family
      worth double-checking at the call site every single time.

──── Step 4: map_or is eager, map_or_else is lazy — same rule as before
  scored.map_or(expensive_label(), |v| format!("{v} stars"))
      (building the fallback label...)
  -> "4 stars"   (the fallback was built and thrown away)
  scored.map_or_else(expensive_label, |v| format!("{v} stars"))
  -> "4 stars"   (nothing printed above: the closure never ran)

──── Step 5: Where clippy pushes you in, and where it pushes you back out
  IN  — map(f).unwrap_or(d) is clippy::map_unwrap_or (pedantic):
        "use map_or(<a>, <f>) instead"
  OUT — map_or over a PREDICATE is clippy::unnecessary_map_or (on by default):
        Some(4)  map_or(false, |v| v > 3) = true  is_some_and(|v| v > 3) = true
        Some(4)  map_or(true,  |v| v > 3) = true  is_none_or(|v| v > 3)  = true
        None     map_or(false, |v| v > 3) = false is_some_and(|v| v > 3) = false
        None     map_or(true,  |v| v > 3) = true  is_none_or(|v| v > 3)  = true
      The two nudges are not in conflict: they mark a boundary. When the
      fallback is a VALUE, map_or beats writing map and unwrap_or. When it
      is `false` or `true`, you were never defaulting at all — you were
      asking a yes/no question, and is_some_and / is_none_or say that in
      words. (is_none_or is the newer of the two: Rust 1.82.)

──── Step 6: Where a match still wins
  Ada   5/5
  Ben   —
  Cara  0/5
  counted 2, abstained 1, total 5
      The first loop is exactly what map_or is for: two short expressions
      producing one value. The second is not — the branches update three
      counters, and a closure that mutates its environment to be useful is
      a match wearing a disguise. Neither closure can `?` or `return` out
      of the enclosing function either, so anything with an early exit is a
      match or an `if let` and not a debate.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/map_or/examples/map_or.rs -o /tmp/mo && /tmp/mo
```

## See also

- [`unwrap_or`](../unwrap_or/README.md) — the fallback with no transformation, and the eager trap in full
- [`unwrap_or_else`](../unwrap_or_else/README.md) — the lazy one, and `or_else`, its other confusable neighbour
- [`unwrap_or_default`](../unwrap_or_default/README.md) — when the fallback comes from the type instead
- [`if let`](../if_let/README.md) — the shape to reach for when the branches stop being expressions
- [`Option::map_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or) · [`map_or_else`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or_else) · [`is_some_and`](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some_and) · [`is_none_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none_or)
