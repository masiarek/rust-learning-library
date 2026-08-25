# What a warning is asking

**Level:** 101 → 201 · for newcomers

**One line:** A warning is `rustc` asking whether you meant what you wrote — it comes from the compiler itself rather than a linter you install, `_name` and `_` are two different answers to it, and only one of those two leaves your program behaving the same way.

The first Rust warning almost everybody meets is `unused variable`, and it arrives with a fix attached: *"if this is intentional, prefix it with an underscore."* Read as an instruction, that sentence is a way to make the compiler stop talking. Read as what it is — a question, with *if* at the front — it is the compiler declining to guess. You wrote a name and never read it. Either you meant to, and the warning just found a bug, or you did not, and it wants you to say so in the code where the next reader will see it.

This page is about answering that question properly, because the wrong answer is cheap to type and two of the wrong answers are silent.

---

## The warning is inside the compiler

Nothing was configured to produce this. `rustc` ships the lints, turns most of them on, and names them in its own output:

```text
warning: unused variable: `audit`
   |
55 |     let audit = AuditEntry::open("approval tally");
   |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_audit`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
```

That last line is the useful one, and it is easy to skim past. It gives the lint's **name** (`unused_variables`), the **group** it belongs to (`unused`), and the **level** it currently sits at (`warn`) — which is everything you need to change your mind about it later, at whatever scope you want. A warning in Rust is not a diagnostic the compiler improvised; it is a named thing with a level you can set.

That placement is the difference from most languages. There is no step where you go and run the checker.

## Four warnings, four different right answers

Here is a tally program that compiles, runs, and exits `0`. It is also wrong, and the compiler says so four times:

```text
warning: unused variable: `audit`
warning: unused variable: `abstentions`
warning: unused variable: `winner`
warning: function `old_margin` is never used
warning: 4 warnings emitted
```

Four warnings, and **`_` is the right answer to exactly one of them**:

| Warning | What was actually going on | The fix |
|---|---|---|
| `winner` | computed, never printed — the program announced nothing | **use it**: the warning found a bug |
| `abstentions` | this tally genuinely does not report them | **`_abstentions`**: says "on purpose" in the code |
| `audit` | a value whose *lifetime* is the point | **`_audit`** — and never bare `_`, see below |
| `old_margin` | a helper nothing calls any more | **delete it**: git remembers it |

The one worth dwelling on is `winner`. The compiler had found a real defect — the whole tally ran and printed no result — and the fix it *suggested* was `_winner`, which would have silenced the report and kept the bug. Every rustc suggestion is conditional on that `if this is intentional`, and only you know. This is also the reason to read `cargo fix --bin <name>` output rather than run it and move on: it applies the suggestions, and the suggestion here was wrong.

`old_margin` earns its own note. `#[allow(dead_code)]` would also have made the message go away, and it is the wrong tool: an `allow` preserves the function for ever and hands the next reader a puzzle about whether it still matters. Silencing is for code you are keeping.

## The two underscores are not the same answer

This is the part that is not about tidiness. `let _name = value` and `let _ = value` differ at **runtime**, and the difference is *when the value is destroyed*.

- `let _name = value` **binds**. The value lives until the end of the scope, like any other binding — the leading underscore changes nothing except the lint.
- `let _ = value` **binds nothing**. `_` is a wildcard pattern, not a name; there is no place for the value to live, so it is dropped immediately, on that line.

A value that announces its own destruction makes this visible rather than arguable:

```rust
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("      DROP {}", self.0);
    }
}

fn main() {
    {
        let _named = Guard("a: _named");
        println!("      still inside the block");   // prints BEFORE the drop
    }
    {
        let _ = Guard("b: bare _");
        println!("      still inside the block");   // prints AFTER the drop
    }
}
```

Read the order in [the verified output](#the-verified-output) below: in (a) the drop comes after the block's last line, in (b) it comes before it. Same value, same scope, one character of difference.

Which turns a lint answer into a real bug class. Anything whose *existence* is the work — a `MutexGuard`, a file lock, a span or timer, a transaction handle, an audit entry — bound to bare `_` is released before the code it was supposed to protect ever runs. It compiles, the warning is gone, and the guard is not guarding. Rust's own docs call this out for exactly one type; it is true of all of them. The rule is short: **if the value has a `Drop` you care about, it needs a name — `_something`, never `_`.**

Related, and the mirror image: [a shadow does not drop](../../18_Ownership/shadowing_does_not_drop/README.md) — rebinding a name does not destroy the value underneath, which is the other half of knowing when your values die.

## The four levels, and where you set them

Every lint sits at one of four levels, and you can move it:

| Level | Effect |
|---|---|
| `allow` | say nothing at all |
| `warn` | print it; compile anyway; exit `0` |
| `deny` | print it as an error; the compile fails |
| `forbid` | `deny`, and refuse any later `allow` of this lint |

You set one at whatever scope the decision belongs to:

```rust
#![deny(unused)]                    // crate-wide, top of main.rs / lib.rs

#[allow(dead_code)]                 // this item only
fn kept_for_now() {}

fn main() {
    #[allow(unused_variables)]      // this block only
    let scratch = 1;                // no warning, and no underscore needed
}
```

…or from outside the source entirely, which is how CI does it without editing anything:

```bash
RUSTFLAGS="-D warnings" cargo build
```

`forbid` is the one to use sparingly — it exists for lints where a local `allow` should not be somebody's option, and it costs you the escape hatch in the one place that legitimately needs it.

## A warning is not an error, and that is the trap

Look again at the tally above: four warnings, and it still **exited `0`**. Nothing fails until somebody decides it should. That is the whole reason `-D warnings` exists in CI, and the reason to add it early — a codebase with forty standing warnings is one where warning forty-one is invisible, and warning forty-one is the one that found a bug. The value of the mechanism is not the individual message; it is that the count is normally zero, so a new one is *noticeable*.

Two more things worth knowing about the boundary:

- **Some things are unconditional errors**, not lints at all — the borrow checker, type mismatches, `unsafe` misuse. No level to set, no `allow`. If you can `#[allow]` it, it was a lint.
- **`clippy` picks up where `rustc` stops.** `rustc`'s lints are mostly about code that is *wrong or dead*; clippy's ~700 are about code that works but is not how Rust is written (`if x { true } else { false }`, a manual loop that wanted `.sum()`). Same level system, same `#[allow(clippy::name)]` syntax, and `cargo clippy` runs it. Reach for it once `cargo build` is quiet.

## If you are coming from another language

- **Python** — the mechanism you know for this is a separate program. `ruff`, `flake8` and `pylint` report unused locals; CPython itself never mentions them, and nothing stops the file running. So the transfer is not the *idea* of a lint but its **location**: in Rust it is in the compiler you already ran, on by default, with no config file and no step to forget. The underscore convention transfers almost exactly — `_`, `_unused` and `for _ in range(n)` mean the same "deliberately ignored" in both — but the runtime consequence does not. Python's `_` is an ordinary name that holds a reference; Rust's bare `_` is a pattern that binds nothing, so the `with lock:` you would reach for to scope a resource is `let _guard = ...` here, and writing `let _ = ...` is the bug this page is about.
- **ABAP** — the closest counterpart is the extended program check: `SLIN` reports unused variables, and the syntax check alone does not. That is the same split Python has, and the same one Rust closes — here it is the ordinary compile that tells you, every time, with no separate transaction to remember. What does *not* transfer is the silencing idiom: ABAP's pseudo-comments (`"#EC NEEDED`) and pragmas (`##NEEDED`) are the nearest thing to `#[allow]`, and like `#[allow]` they mark an exception rather than fix anything. The genuinely new part is the `Drop` half — ABAP objects are collected when nothing references them, at a time nobody promises, so "this value must live until the end of this block" is not a thing you can state. In Rust it is, it is exact, and `_` versus `_name` is how you state it.

## Practice

**Four warnings, four different right answers — and only one of them is an underscore.**

Take the approval tally below. It compiles, runs, exits `0`, and emits the four warnings shown earlier. For **each** one, decide which of these it wants — *use the value*, *rename to `_name`*, *replace with bare `_`*, or *delete the code* — and say why before you touch anything. Two are genuinely deliberate and two are defects. One of them will still compile and still be broken if you pick the wrong kind of underscore; find that one first, and predict what its output will look like before you run it.

Then set `#![deny(unused)]` at the top and confirm the file that used to build now refuses to.

<!-- source:what_a_warning_is_asking_before -->
*[`what_a_warning_is_asking_before.rs`](examples/what_a_warning_is_asking_before.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata starting point: an approval tally that compiles, runs, exits 0 —
//! and is wrong. Four warnings say so. Each one wants a different answer.
//!
//! Compile it and read the four warnings before you change anything:
//!
//!   rustc --edition 2024 what_a_warning_is_asking_before.rs -o /tmp/before

const NAMES: [&str; 4] = ["Ada", "Ben", "Cara", "Dev"];

/// Closes an audit entry when it is dropped.
struct AuditEntry(&'static str);

impl Drop for AuditEntry {
    fn drop(&mut self) {
        println!("  [audit] closed: {}", self.0);
    }
}

impl AuditEntry {
    fn open(what: &'static str) -> Self {
        println!("  [audit] opened: {what}");
        AuditEntry(what)
    }
}

/// Split a column of approval marks into (approvals, abstentions).
fn split(marks: &[Option<bool>]) -> (u32, u32) {
    let mut approvals = 0;
    let mut abstentions = 0;
    for m in marks {
        match m {
            Some(true) => approvals += 1,
            Some(false) => {}
            None => abstentions += 1,
        }
    }
    (approvals, abstentions)
}

/// The margin between the top two — from an earlier version of this program.
fn old_margin(totals: &[u32; 4]) -> u32 {
    let mut sorted = *totals;
    sorted.sort_unstable();
    sorted[3] - sorted[2]
}

fn main() {
    let ballots: [[Option<bool>; 4]; 4] = [
        [Some(true), Some(false), Some(true), None],
        [Some(true), Some(true), Some(false), Some(false)],
        [None, Some(true), Some(true), Some(false)],
        [Some(false), Some(false), Some(true), Some(true)],
    ];

    let audit = AuditEntry::open("approval tally");

    let mut totals = [0u32; 4];
    for seat in 0..4 {
        let column: Vec<Option<bool>> = ballots.iter().map(|b| b[seat]).collect();
        let (approvals, abstentions) = split(&column);
        totals[seat] = approvals;
    }

    for (name, total) in NAMES.iter().zip(totals.iter()) {
        println!("  {name:<5} {total}");
    }

    let winner = NAMES
        .iter()
        .zip(totals.iter())
        .max_by_key(|(_, total)| **total)
        .map(|(name, _)| *name)
        .unwrap_or("nobody");
}
```
<!-- /source -->

<!-- output:what_a_warning_is_asking_before -->
*Verified output of [`what_a_warning_is_asking_before.rs`](examples/what_a_warning_is_asking_before.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
  [audit] opened: approval tally
  Ada   2
  Ben   2
  Cara  3
  Dev   1
  [audit] closed: approval tally
```
<!-- /output -->

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_a_warning_is_asking_kata -->
*[`what_a_warning_is_asking_kata.rs`](examples/what_a_warning_is_asking_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: four warnings, four different right answers.
//!
//! The point of the exercise is that "add an underscore" is the correct fix
//! for exactly one of these. One was a real bug the compiler found, one is a
//! deliberate discard, one would BREAK if you used a bare `_`, and one wants
//! deleting rather than silencing.

const NAMES: [&str; 4] = ["Ada", "Ben", "Cara", "Dev"];

/// Closes an audit entry when it is dropped — so *when* it drops is visible.
struct AuditEntry(&'static str);

impl Drop for AuditEntry {
    fn drop(&mut self) {
        println!("  [audit] closed: {}", self.0);
    }
}

impl AuditEntry {
    fn open(what: &'static str) -> Self {
        println!("  [audit] opened: {what}");
        AuditEntry(what)
    }
}

/// Split a column of approval marks into (approvals, abstentions).
fn split(marks: &[Option<bool>]) -> (u32, u32) {
    let mut approvals = 0;
    let mut abstentions = 0;
    for m in marks {
        match m {
            Some(true) => approvals += 1,
            Some(false) => {}
            None => abstentions += 1,
        }
    }
    (approvals, abstentions)
}

// Warning 4 was `fn old_margin(...)`, a helper nothing calls any more.
// The fix is deletion, not `#[allow(dead_code)]`: an allow would preserve it
// for ever, and the next reader would have to work out whether it still
// matters. Version control already remembers it. It is simply gone.

fn main() {
    // Four voters, four candidates. `None` is a blank — nobody marked it.
    let ballots: [[Option<bool>; 4]; 4] = [
        [Some(true), Some(false), Some(true), None],
        [Some(true), Some(true), Some(false), Some(false)],
        [None, Some(true), Some(true), Some(false)],
        [Some(false), Some(false), Some(true), Some(true)],
    ];

    // Warning 3: an audit entry must OUTLIVE the work it records.
    // `let _ = AuditEntry::open(..)` compiles, silences the warning, and
    // closes the entry before the tally even starts. The name is load-bearing.
    let _audit = AuditEntry::open("approval tally");

    let mut totals = [0u32; 4];
    for seat in 0..4 {
        let column: Vec<Option<bool>> = ballots.iter().map(|b| b[seat]).collect();

        // Warning 2: this tally does not report abstentions, and that is a
        // decision, not an oversight. `_abstentions` says so out loud.
        let (approvals, _abstentions) = split(&column);
        totals[seat] = approvals;
    }

    for (name, total) in NAMES.iter().zip(totals.iter()) {
        println!("  {name:<5} {total}");
    }

    // Warning 1: this was the real bug. `winner` was computed and never read,
    // so the program did all the work and announced nothing. The compiler was
    // not complaining about style; it had found the missing line.
    let winner = NAMES
        .iter()
        .zip(totals.iter())
        .max_by_key(|(_, total)| **total)
        .map(|(name, _)| *name)
        .unwrap_or("nobody");

    println!("  winner: {winner}");
}
```
<!-- /source -->

<!-- output:what_a_warning_is_asking_kata -->
*Verified output of [`what_a_warning_is_asking_kata.rs`](examples/what_a_warning_is_asking_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
  [audit] opened: approval tally
  Ada   2
  Ben   2
  Cara  3
  Dev   1
  winner: Cara
  [audit] closed: approval tally
```
<!-- /output -->

The two output blocks are worth diffing rather than reading. The broken version prints no `winner:` line at all — that was `unused_variables` reporting a real defect — and everything else about it looks fine, which is exactly why the warning had to be the thing that caught it.

</details>

## The proof

<!-- source:what_a_warning_is_asking -->
*[`what_a_warning_is_asking.rs`](examples/what_a_warning_is_asking.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! A warning is the compiler asking whether you meant it.
//!
//! `unused_variables` is the first one almost everybody meets, and the
//! suggested fix — "prefix it with an underscore" — is usually read as a way
//! to make the compiler shut up. It is not. `_name` and `_` are two different
//! answers to the question, and one of them changes when your value is
//! destroyed. Step 2 makes that visible.
//!
//!   rustc --edition 2024 what_a_warning_is_asking.rs -o /tmp/wawia && /tmp/wawia

fn banner(n: u8, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ── Step 2's instrument ──────────────────────────────────────────────────────
// A value that announces its own destruction, so "when does this drop?" stops
// being a claim on a page and becomes a line of output.

struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("      DROP {}", self.0);
    }
}

// ── Step 3's instrument ──────────────────────────────────────────────────────
// `dead_code` is a different lint from `unused_variables`, and it fires on
// items rather than bindings. The `#[allow]` is the point of the step, not an
// apology for it: the attribute is how you answer a lint for one item.

#[allow(dead_code)]
fn never_called_but_allowed() -> u8 {
    42
}

fn main() {
    println!("What a warning is asking");

    // ── Step 1 ───────────────────────────────────────────────────────────────
    banner(1, "Three ways to not use a binding");

    let used = 10u8;
    println!("      let used     -> {used}, and no warning: it was read");

    let _computed = used * 2;
    println!("      let _computed -> computed on purpose, unused on purpose");

    let _ = used * 3;
    println!("      let _         -> discarded outright; there is no name to read");

    println!("      All three compile with zero warnings. They are not the same.");

    // ── Step 2 ───────────────────────────────────────────────────────────────
    banner(2, "The two underscores disagree about WHEN");

    println!("    (a) let _named = Guard(..)  — binds, lives to the end of scope");
    {
        let _named = Guard("a: _named");
        println!("      still inside the block");
    }

    println!("    (b) let _ = Guard(..)       — binds nothing, drops immediately");
    {
        let _ = Guard("b: bare _");
        println!("      still inside the block");
    }

    println!("      Read the order above: (a) dropped after its line, (b) before.");
    println!("      A mutex guard bound to `_` is unlocked before you use it.");

    // ── Step 3 ───────────────────────────────────────────────────────────────
    banner(3, "The four lint levels, and where you set them");

    for (level, effect) in [
        ("allow", "say nothing at all"),
        ("warn", "print it; compile anyway; exit 0"),
        ("deny", "print it as an error; compile fails"),
        ("forbid", "deny, and refuse any later allow of this lint"),
    ] {
        println!("      #[{level:<6}] {effect}");
    }
    println!("      Set on an item, a block, or the crate (`#![deny(...)]`),");
    println!("      or from outside with `-D warnings` / RUSTFLAGS in CI.");
    println!("      never_called_but_allowed() = {}", never_called_but_allowed());

    // ── Step 4 ───────────────────────────────────────────────────────────────
    banner(4, "A warning is not an error, and that is the trap");

    println!("      Every warning in this file's history still exited 0.");
    println!("      Nothing fails until someone chooses to make it fail —");
    println!("      `-D warnings` in CI is that choice, made once.");
    println!("      `cargo fix` applies the compiler's own suggestions, so");
    println!("      read them first: it will happily add an underscore where");
    println!("      the honest fix was to USE the variable.");
}
```
<!-- /source -->

## The verified output

<!-- output:what_a_warning_is_asking -->
*Verified output of [`what_a_warning_is_asking.rs`](examples/what_a_warning_is_asking.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
What a warning is asking

──── Step 1: Three ways to not use a binding
      let used     -> 10, and no warning: it was read
      let _computed -> computed on purpose, unused on purpose
      let _         -> discarded outright; there is no name to read
      All three compile with zero warnings. They are not the same.

──── Step 2: The two underscores disagree about WHEN
    (a) let _named = Guard(..)  — binds, lives to the end of scope
      still inside the block
      DROP a: _named
    (b) let _ = Guard(..)       — binds nothing, drops immediately
      DROP b: bare _
      still inside the block
      Read the order above: (a) dropped after its line, (b) before.
      A mutex guard bound to `_` is unlocked before you use it.

──── Step 3: The four lint levels, and where you set them
      #[allow ] say nothing at all
      #[warn  ] print it; compile anyway; exit 0
      #[deny  ] print it as an error; compile fails
      #[forbid] deny, and refuse any later allow of this lint
      Set on an item, a block, or the crate (`#![deny(...)]`),
      or from outside with `-D warnings` / RUSTFLAGS in CI.
      never_called_but_allowed() = 42

──── Step 4: A warning is not an error, and that is the trap
      Every warning in this file's history still exited 0.
      Nothing fails until someone chooses to make it fail —
      `-D warnings` in CI is that choice, made once.
      `cargo fix` applies the compiler's own suggestions, so
      read them first: it will happily add an underscore where
      the honest fix was to USE the variable.
```
<!-- /output -->

## See also

- [Comments that compile](../comments_that_compile/README.md) — a lint you meet the same way: a misplaced `///` is `unused_doc_comments`, a warning rather than an error, and documentation nobody will ever read
- [A shadow does not drop](../../18_Ownership/shadowing_does_not_drop/README.md) — the other half of "when does this value die", from the naming side
- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — why a value has exactly one owner and one end, which is what makes the `_` versus `_name` difference observable at all
- [What a panic costs](../../17_Option_and_Result/what_a_panic_costs/README.md) — the other thing that runs on the way out of a scope, and what happens when it runs during unwinding
- [Running a scratch program](../rustc_without_cargo/README.md) — where `RUSTFLAGS` and `-D warnings` go when there is no Cargo
- [Formatting](../../05_Tooling/formatting/README.md) — the other thing that is not your problem to argue about, settled by a tool
