# Initial values: when `Option` is the wrong tool

**Level:** 201 · working knowledge

**One line:** For "a variable with no value yet", Rust usually does not want `Option` at all — it lets you declare without initializing and proves you assigned before use.

This is the one job on [`std::option`'s list](https://doc.rust-lang.org/core/option/) where the obvious reading leads most newcomers astray, so it is worth separating the case `Option` genuinely serves from the case it only appears to.

---

## The pattern you will write first

```rust
let mut initial_value: Option<i32> = None;
initial_value = Some(42);

match initial_value {
    Some(value) => println!("The initial value is: {value}"),
    None => println!("No initial value"),
}
```

This works. But count what it costs: a `mut`, a wrapper, and a `match` over a case that — by the time we look — cannot happen. The `None` arm is unreachable and the compiler cannot tell you so, because you asked for a type where absence is legal.

The habit comes from languages where a variable must be given *something* at declaration, so you reach for a "nothing yet" placeholder: `null`, `None`, `-1`, `""`. Rust does not have that constraint.

## What Rust actually offers

```rust
let settled: i32;          // declared, not initialized — and not `mut`
if flag {
    settled = 42;
} else {
    settled = 7;
}
println!("{settled}");     // fine: every path assigned it exactly once
```

**Rust does not require a value at declaration. It requires one before use, and it proves that at compile time.** Miss a branch and you get `error[E0381]: used binding is possibly-uninitialized`; assign twice and the missing `mut` catches you. So `settled` is a plain `i32` — no wrapper, no `mut`, no unreachable arm — and it is *more* strongly checked than the `Option` version, not less.

Most of the time you do not even need the statement form:

```rust
let same = if flag { 42 } else { 7 };
```

**The test:** if every path assigns before first use, the answer is deferred initialization. `Option` is for when that is *not* true.

## When `Option` is genuinely right

**The absence survives to the point of use.** A setting nobody configured is still missing when you read it — `None` there is not "not assigned yet", it is "the user did not say", which is a fact about the world and belongs in the type:

```rust
let configured: Option<u16> = lookup("port").and_then(|s| s.parse().ok());
let port = configured.unwrap_or(8080);
```

**There is no honest starting value.** A running "best so far" has nothing sensible to start at — `0` is a lie for an empty list and wrong the moment negatives are allowed:

```rust
let mut best: Option<u32> = None;
for s in scores {
    best = Some(best.map_or(s, |b| b.max(s)));
}
```

This is why `iter().max()` returns `Option` rather than picking a sentinel, and in real code you would just call it. The pattern still matters for folds that have no library equivalent.

**It is a struct field.** A field cannot be deferred — a struct is either fully built or does not exist — so "may not be set yet" has to be `Option` in the type. That case has [its own page](../option_fields/README.md).

**It is a `static`.** Same reason, and the modern tool is `OnceLock`:

```rust
static GREETING: OnceLock<String> = OnceLock::new();
GREETING.set("hello".to_string()).expect("first set always succeeds");
```

Prefer it over a `mut Option` in a global: it guarantees the value is written *exactly once*, and it is thread-safe, neither of which a bare `Option` promises.

---

## Practice

**Declare it, then prove it.** Write `quorum_for(voters: usize) -> usize` whose value is decided in three branches — no `Option`, and no `mut`. Let the compiler be the thing that guarantees the variable is set before it is read.

Write it with `let mut quorum: Option<usize> = None;` too, and read what the compiler says about the `None` you gave it. Then delete one branch from the plain version and read `E0381`. One of those two messages is a warning about a value you never needed; the other is a proof you cannot fake.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:initial_values_kata -->
*[`initial_values_kata.rs`](examples/initial_values_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a value that is not ready yet is not the same as a missing one.
//!
//!   rustc --edition 2024 initial_values_kata.rs -o /tmp/ivk && /tmp/ivk

/// Declared, not initialized. Every path assigns before the read, and the
/// compiler checks that — no `Option`, no `mut`, no unreachable arm.
fn quorum_for(voters: usize) -> usize {
    let quorum: usize;
    if voters == 0 {
        quorum = 0;
    } else if voters < 10 {
        quorum = voters; // a tiny board needs everyone
    } else {
        quorum = voters / 2 + 1;
    }
    quorum
}

/// The `Option` version of the same job, for contrast: one more state to read,
/// one more way to be wrong, and an arm that cannot happen.
///
/// The allow is part of the lesson — without it the compiler warns that the
/// `None` is "never read", which is it telling you the initial value was dead
/// on arrival.
#[allow(unused_assignments)]
fn quorum_for_awkward(voters: usize) -> usize {
    let mut quorum: Option<usize> = None;
    if voters == 0 {
        quorum = Some(0);
    } else if voters < 10 {
        quorum = Some(voters);
    } else {
        quorum = Some(voters / 2 + 1);
    }
    quorum.expect("every branch above assigns — but nothing checks that claim")
}

/// Where `Option` is genuinely right: the value may never arrive at all.
struct Election {
    name: &'static str,
    /// None until the count finishes. Not "not ready yet" — "may never happen".
    certified_winner: Option<&'static str>,
}

fn main() {
    println!("Declared without a value, proved assigned before use:");
    for voters in [0, 7, 461] {
        println!("  quorum_for({voters:>3}) -> {}", quorum_for(voters));
    }

    println!("\nThe Option version returns the same numbers…");
    for voters in [0, 7, 461] {
        println!("  quorum_for_awkward({voters:>3}) -> {}", quorum_for_awkward(voters));
    }
    println!("      …and pays for it with a state that cannot occur, an `expect`");
    println!("      whose claim nothing verifies, and a `mut` it did not need. The");
    println!("      compiler even warns that the initial None is never read — the");
    println!("      source has to #[allow] it to compile quietly.");

    println!("\nWhere Option earns its place — absence that outlives the function:");
    let running = Election { name: "Springfield 2026", certified_winner: None };
    let done = Election { name: "Shelbyville 2026", certified_winner: Some("Ada") };
    for e in [&running, &done] {
        match e.certified_winner {
            Some(w) => println!("  {:<18} certified: {w}", e.name),
            None => println!("  {:<18} not certified yet", e.name),
        }
    }
}
```
<!-- /source -->

<!-- output:initial_values_kata -->
*Verified output of [`initial_values_kata.rs`](examples/initial_values_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Declared without a value, proved assigned before use:
  quorum_for(  0) -> 0
  quorum_for(  7) -> 7
  quorum_for(461) -> 231

The Option version returns the same numbers…
  quorum_for_awkward(  0) -> 0
  quorum_for_awkward(  7) -> 7
  quorum_for_awkward(461) -> 231
      …and pays for it with a state that cannot occur, an `expect`
      whose claim nothing verifies, and a `mut` it did not need. The
      compiler even warns that the initial None is never read — the
      source has to #[allow] it to compile quietly.

Where Option earns its place — absence that outlives the function:
  Springfield 2026   not certified yet
  Shelbyville 2026   certified: Ada
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:initial_values -->
*Verified output of [`initial_values.rs`](examples/initial_values.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: The pattern you will see written first
  The initial value is: 42
      It works. But notice what it costs: a `mut`, a wrapper, and a `match`
      over a case that — by the time we look — cannot happen.
      rustc agrees, and says so: "value assigned to `initial_value` is
      never read" — the placeholder was dead the moment it was written.

──── Step 2: What Rust actually offers: declare now, assign later
  settled = 42
  same    = 42
      Rust does not require a value at DECLARATION — only before USE, and
      it proves that at compile time. So `settled` is never Option-shaped,
      never mut, and no branch can forget to set it.

──── Step 3: When Option IS right: absence survives to the point of use
  lookup("log_level")  -> Some("debug")
  lookup("port")       -> None
  port in use          -> 8080
      Here None is not 'not yet assigned', it is 'the user did not say'.
      That fact is still true when we read it — so it belongs in the type.

──── Step 4: …and for a running 'best so far', where there is no sensible start
  best of [3, 9, 4] -> Some(9)
  best of []        -> None
      Starting at 0 would be a LIE for an empty list, and wrong for negatives.
      (In real code: scores.iter().max() — which returns Option for this reason.)
  scores.iter().max() -> Some(9)

──── Step 5: Initialize once, later, globally: OnceLock
  before set: None
  after set:  Some("hello")
  second set: true
      A `static` cannot be deferred — so this is the case that genuinely
      needs an 'empty until later' box. OnceLock is that box, and unlike a
      mut Option it guarantees the value is written exactly once.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 01_Foundations/initial_values/examples/initial_values.rs -o /tmp/iv && /tmp/iv
```

## See also

- [`Option` fields](../option_fields/README.md) — the case where deferring is not available
- [Partial functions](../partial_functions/README.md) — why `iter().max()` returns `Option` in the first place
- [`OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html) — initialize-once, for statics and lazy globals
