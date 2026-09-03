# When to shadow

**Level:** 201 · working knowledge

**One line:** Shadow when the new binding is **the same concept in a new form**; reach for a second name when it is a different thing — and never shadow something holding a resource.

[Shadowing and `unwrap`](../../17_Option_and_Result/shadowing_and_unwrap/README.md) covers what shadowing *is*, and [a shadow does not drop](../shadowing_does_not_drop/README.md) covers what it does to the value underneath. Both leave the question you actually face at the keyboard: *should I write `let x` again here, or think of another name?* This page is that decision, and the four ways it goes wrong — three of which compile.

The whole answer fits in the one-line summary above. Everything below is why each clause is in it.

---

## What the feature actually buys

The usual defence of shadowing is that it saves you from `input_raw`, `input_trimmed`, `input_num`. True, and undersold — the real argument is that **the alternative is not a longer name, it is `mut`**, and `mut` tells the reader far less.

```rust
let raw = "  42  ";
let raw = raw.trim();
let raw: u32 = raw.parse().expect("a number");
println!("{raw}");  // 42
```

Three bindings, zero `mut`, and the type changed twice. Now try it with mutation instead:

```rust
let mut text = "  42  ";
text = text.trim();
let number: u32 = text.parse().expect("a number");  // a second name arrives anyway
```

Assignment must preserve the type, so `mut` could not carry `&str` to `u32` — the second name showed up regardless. And you paid for it: `text` is mutable for the rest of the scope, which promises the reader almost nothing.

That is the trade, and it is worth saying in one line: **shadowing says "this changed *here*"; `mut` says "this may change *anywhere below*".** A language without shadowing pushes people toward the weaker of those two claims. Rust having it is why so much idiomatic Rust is immutable.

## The idioms

Every one of these is the same concept arriving in a better form, which is the test.

**Generic parameter to the one concrete type the body wants.** The canonical opening line of any `AsRef` API:

```rust
fn read_header(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    format!("{}", path.display())
}
```

**Unwrap and narrow.** `Option<String>` to `String` to `&str` and back, with no second name to keep straight:

```rust
let Some(name) = name else { return "nobody".to_string() };
let name = name.trim();
name.to_uppercase()
```

**Freeze after building.** The `mut` was scoped to the *building*, not to the variable's life:

```rust
let mut totals = Vec::new();
totals.push(5);
let totals = totals;   // cannot be pushed to from here on
```

**Narrow into a [newtype](../../16_Structs/newtype_score/README.md).** The loose form is still alive but nothing can reach it, so no later line can pass a raw number where an id is due:

```rust
let id = 42_u32;
let id = OrderId(id);
```

**Clone for a move, inside a block.** The shadow lives and dies in the braces, and the outer name survives — the cheapest way to hand a thread its own handle:

```rust
let counted = {
    let rows = Arc::clone(&rows);
    thread::spawn(move || rows.len())
};
println!("{} {}", counted.join().unwrap(), rows.len());  // 3 3
```

And the small ones you will write without thinking: `let s = s.as_str();`, `let line = line.trim();`, `let x = *x;` inside a loop, `let result = dbg!(result);` for an instrumentation line you can delete later without touching anything else.

## Three bugs that compile

### 1. The accumulator that never accumulates

```rust
let mut total = 0;
for score in [5, 3, 4] {
    let total = total + score;   // shadows; the outer `total` is untouched
    println!("{total}");         // 5, then 3, then 4
}
println!("{total}");             // 0
```

Each iteration builds a fresh `total` from the outer zero and throws it away at the brace. The only warning is *"variable does not need to be mutable"*, which never says "shadow" — so read it as the bug report it is: **an accumulator that does not need `mut` is not accumulating.**

This one has a page of its own. [Nothing checks a shadow](../nothing_checks_a_shadow/README.md) takes it apart as its central specimen — why no lint fires, and how narrow the margin is that makes it silent.

### 2. The guard that was never released

This is the dangerous one, and it follows directly from shadowing taking away a name rather than a value:

```rust
let guard = counter.lock().unwrap();
println!("{}", *guard);
let mut guard = register.lock().unwrap();   // is `counter` unlocked now?
*guard += 1;

println!("{}", counter.try_lock().is_err());    // true  — still locked
println!("{}", register.try_lock().is_err());   // true
```

Both locks are held to the end of the scope, and there is no longer a name that could release the first one early. In one thread that only wastes a lock. Add a second thread that wants `counter` and it is a deadlock whose cause is a line that looks like a rename.

**Nothing warns**, because the first guard was read before the shadow — which is what real code looks like. The rule is blunt and worth keeping blunt: *never shadow a value that holds a resource.* Locks, spans, file handles, transaction guards. Give it a different name, or put it in its own block.

### 3. One name, two concepts

```rust
let threshold: usize = 3;        // the minimum score that counts
println!("minimum: {threshold}");

// …a later edit needs the row count, and reaches for the nearest good word
let threshold = scores.len();    // a DIFFERENT quantity
let counted = scores.iter().filter(|&&s| s >= threshold).count();
```

Over `[5, 2, 4, 0, 3]` that counts **1** where it should count 3. Both bindings are `usize`, both are read, and the compiler has no way to know that the first `threshold` was a *score* and the second a *quantity*. Nothing warns, ever.

This is the case for a second name — and note *why*. Not because shadowing is unsafe: because the two values are not the same thing. The rule at the top of this page is doing all the work here.

**Distance is the aggravating factor.** A shadow two lines from its original reads as one thought; a shadow forty lines away is a redefinition wearing a familiar word. If you cannot see both `let`s at once, you are not shadowing, you are colliding.

## The one the compiler does catch

Functions live in the same namespace as values, so a `let` hides one:

```rust
fn rows_read() -> usize { 461 }

let rows_read = rows_read();
let again = rows_read();   // does not compile
```

```text
error[E0618]: expected function, found `usize`
  |
1 | fn rows_read() -> usize { 461 }
  | -------------------------- this function of the same name is available here,
  |                            but it's shadowed by the local binding
```

Worth meeting once, because it is the only shadowing mistake rustc names out loud. The three above got a misleading warning, a correct-looking program, and nothing at all.

## What would have caught this?

Almost nothing, and that is the uncomfortable part of the rule at the top of this page. The compiler's one genuine net is `unused variable`, which fires only when a shadowed binding is **never read** — and in all three bugs above the first binding *was* read, because that is what real code looks like. `error[E0618]` is the honourable exception, and it is the only one of the four mistakes on this page that rustc names out loud.

Clippy has three lints for it, all allow-by-default, and choosing between them is not the coin flip it looks like. Run over this page's own kata file — which holds a shadowed accumulator, a name reused for a second concept, and four correct parse-and-narrow shadows:

| Lint | On the accumulator (bug 1) | On the reused name (bug 3) | On the four correct shadows |
|---|---|---|---|
| [`shadow_same` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#shadow_same) | no | no | no — it found nothing in the file at all |
| [`shadow_unrelated` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#shadow_unrelated) | **no** — the accumulator *reuses* `total`, so it is not "unrelated" | yes | no |
| [`shadow_reuse` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#shadow_reuse) | **yes** | no | **yes — all four of them** |

So the only lint that catches the worst bug is the one that also condemns the idiom the feature exists for. `shadow_reuse` cannot tell `let total = total + s` in a loop from `let raw: u32 = raw.parse()?` at the top of a function, because syntactically they are the same move. If you want one of them on, `shadow_unrelated` is the cheapest and the most defensible — it just will not catch bug 1.

[Nothing checks a shadow](../nothing_checks_a_shadow/README.md) is the full account: the lint output in situ, what the `restriction` group means about all three, and the one-line margin that decides whether the compiler says anything at all.

## If you are coming from another language

- **Python.** `x = int(x)` is the closest shape, and the difference is the one that bites. Python **rebinds one variable** — the old object loses a reference and may be collected, so you cannot have both alive under one name. Rust makes a **second variable**, and the first is still there, still owned, still dropping at the end of the scope. That is exactly bug 2: Python has no equivalent of the doubled lock, because in Python there is nothing left holding it.
- **ABAP.** No analogue at all: a `DATA` name is one typed variable for the whole routine, and a second `DATA(lv_x)` for a name already declared is a syntax error. So ABAP forces the `lv_input` / `lv_input_num` pair that shadowing exists to avoid — and its `lv_` / `lt_` prefixes exist to carry the type in the name. Shadowing is the feature that lets you stop doing that, because the type is in the compiler rather than the identifier. What you give up is the guarantee ABAP hands you for free: that one name in a routine means one thing. Bug 3 is that guarantee being spent.
- **C / C++.** Both shadow, but only across a new block — a redefinition in the *same* block is an error, which is why `-Wshadow` exists and why C code nests braces to get the effect. Neither gives you the type change, since the declaration carries the type.

---

## Practice

**Three shadows, one of them earned.** Write a function that reads a handful of survey cards — a label and a rating as untrimmed text — and reports two numbers: the total rating, and how many cards scored at or above some minimum. Put three shadows in it on purpose:

1. one that parses a card's raw text into a number,
2. one inside the loop that computes `total + this_card`,
3. one that reuses the name of the minimum score for the count of cards.

Predict what it prints before you run it. Then answer three questions: which of the three bindings is the compiler's single warning about, does that warning contain the word "shadow", and which of the two wrong numbers would you have noticed in a code review?

Then fix it — and the point of the exercise is that **the two fixes are different kinds of fix.** One shadow has to be deleted; one has to be renamed; one is correct and must survive both passes untouched. Say which is which before you look.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:when_to_shadow_kata -->
*[`when_to_shadow_kata.rs`](examples/when_to_shadow_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three shadows in one function, and only one of them is earned.
//!
//! The broken version below compiles, runs, prints a plausible report, and is
//! wrong twice. It gets exactly ONE warning, which points at neither bug in
//! terms you would recognise.
//!
//! The two fixes are deliberately different, and that difference is the answer
//! to "when should I shadow?":
//!
//!   * the lost sum is fixed by DELETING a shadow — the loop wanted to write to
//!     the outer variable, so it wanted `mut`, not a new binding;
//!   * the wrong filter is fixed by RENAMING — both values are legitimate, they
//!     are simply not the same concept and should never have shared a word.
//!
//! The third shadow is left exactly as it is. It is the idiom.
//!
//!   rustc --edition 2024 when_to_shadow_kata.rs -o /tmp/wtsk && /tmp/wtsk

/// One survey card as it arrives: a label and its ratings as raw text.
const CARDS: [(&str, &str); 5] = [
    ("Ada", " 5 "),
    ("Ben", "2"),
    ("Cara", " 4"),
    ("Dan", "0 "),
    ("Eve", " 3 "),
];

/// The minimum score that counts as support, for the report's second line.
const SUPPORT_AT_LEAST: usize = 3;

// ─────────────────────────────────────────────────────────────── the bug
//
// rustc's only complaint about this function is:
//
//   warning: variable does not need to be mutable
//    --> when_to_shadow_kata.rs
//     |     let mut total = 0;
//
// which is the accumulator bug reported in a vocabulary that does not contain
// the word "shadow". Nothing at all is said about `threshold`.
#[allow(unused_mut)]
fn summarize_broken() -> (usize, usize) {
    let mut total = 0;

    for (_who, raw) in CARDS {
        // Shadow #1 — EARNED. Same concept (this card's rating), better form:
        // &str with spaces -> &str trimmed -> usize. No second name needed.
        let raw = raw.trim();
        let raw: usize = raw.parse().expect("every card above is a number");

        // Shadow #2 — the lost sum. This builds a fresh `total` from the outer
        // one and drops it at the closing brace, three times over.
        let total = total + raw;
        let _ = total;
    }

    let threshold = SUPPORT_AT_LEAST;
    println!("  counting support at {threshold} or above");

    // Shadow #3 — one name, two concepts. `threshold` was a SCORE; now it is a
    // COUNT of cards. Both are usize, both are read, so nothing warns.
    let threshold = CARDS.len();
    let strong = CARDS
        .iter()
        .filter(|(_who, raw)| raw.trim().parse::<usize>().unwrap_or(0) >= threshold)
        .count();

    (total, strong)
}

// ─────────────────────────────────────────────────────────── the two fixes
fn summarize_fixed() -> (usize, usize) {
    let mut total = 0;

    for (_who, raw) in CARDS {
        // Shadow #1 stays. It was never the problem.
        let raw = raw.trim();
        let raw: usize = raw.parse().expect("every card above is a number");

        // Fix A: DELETE the shadow. The loop wanted to write to the variable
        // that outlives it, and `mut` is exactly the promise that allows.
        total += raw;
    }

    // Fix B: RENAME. Both values are wanted; they are simply different things,
    // and the moment they have honest names the bug is unwriteable.
    let min_score = SUPPORT_AT_LEAST;
    let card_count = CARDS.len();
    let strong = CARDS
        .iter()
        .filter(|(_who, raw)| raw.trim().parse::<usize>().unwrap_or(0) >= min_score)
        .count();

    println!("  counting support at {min_score} or above, over {card_count} cards");

    (total, strong)
}

fn main() {
    println!("──── The broken version");
    let (total, strong) = summarize_broken();
    println!("  total score = {total}      (five cards scoring 5, 2, 4, 0, 3)");
    println!("  strong  = {strong}      (should be the scores of 3 or more)");
    println!("      Both numbers are wrong, and both look like they could be");
    println!("      right — which is the whole hazard. A total of 0 reads as a");
    println!("      blank form, and 1 supporter as a weak field. Neither");
    println!("      number looks like a bug; they look like a finding.");

    println!("\n──── The fixed version");
    let (total, strong) = summarize_fixed();
    println!("  total score = {total}     (5 + 2 + 4 + 0 + 3)");
    println!("  strong  = {strong}      (5, 4 and 3 clear the bar of 3)");

    println!("\n──── Why the two fixes differ");
    println!("  The lost sum was a shadow standing where `mut` belonged: the");
    println!("  loop needed to CHANGE something that outlives it, and that is");
    println!("  the one job shadowing cannot do. Deleting the `let` fixed it.");
    println!();
    println!("  The wrong filter was a shadow standing where a SECOND NAME");
    println!("  belonged: a minimum score and a card count are different");
    println!("  quantities that happen to share a type. Renaming fixed it,");
    println!("  and no shadow was involved in the repair at all.");
    println!();
    println!("  Shadow #1 survived both passes untouched. Same concept, new");
    println!("  form, two lines apart — that is what an earned shadow reads");
    println!("  like, and it is why the answer is never \"stop shadowing\".");
}
```
<!-- /source -->

<!-- output:when_to_shadow_kata -->
*Verified output of [`when_to_shadow_kata.rs`](examples/when_to_shadow_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── The broken version
  counting support at 3 or above
  total score = 0      (five cards scoring 5, 2, 4, 0, 3)
  strong  = 1      (should be the scores of 3 or more)
      Both numbers are wrong, and both look like they could be
      right — which is the whole hazard. A total of 0 reads as a
      blank form, and 1 supporter as a weak field. Neither
      number looks like a bug; they look like a finding.

──── The fixed version
  counting support at 3 or above, over 5 cards
  total score = 14     (5 + 2 + 4 + 0 + 3)
  strong  = 3      (5, 4 and 3 clear the bar of 3)

──── Why the two fixes differ
  The lost sum was a shadow standing where `mut` belonged: the
  loop needed to CHANGE something that outlives it, and that is
  the one job shadowing cannot do. Deleting the `let` fixed it.

  The wrong filter was a shadow standing where a SECOND NAME
  belonged: a minimum score and a card count are different
  quantities that happen to share a type. Renaming fixed it,
  and no shadow was involved in the repair at all.

  Shadow #1 survived both passes untouched. Same concept, new
  form, two lines apart — that is what an earned shadow reads
  like, and it is why the answer is never "stop shadowing".
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:when_to_shadow -->
*Verified output of [`when_to_shadow.rs`](examples/when_to_shadow.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: What the feature buys: it keeps `mut` meaning something
  shadowed: 42
      Three `let`s, zero `mut`. Every line's value is final, and
      the type changed twice without inventing `raw_trimmed`.
  with mut: 42
      `mut` could not carry &str -> u32, so a second name arrived
      anyway — and `text` is now mutable for the rest of the
      scope, which promises the reader far less than the
      shadowed version did.
      THAT is the trade: shadowing says "changed HERE";
      `mut` says "may change ANYWHERE below".

──── Step 2: The idioms: same concept, better form
  generic -> concrete:  /etc/hosts
  unwrap-and-narrow:    ADA
  frozen after build:   [5, 3]
      From this line on, `totals` cannot be pushed to. The `mut`
      was scoped to the building, not to the variable's life.
  narrowed to newtype:  OrderId(42)
      The bare u32 is still alive, but nothing can reach it —
      so no later line can pass a raw number where an id is due.
  clone-for-move:       thread counted 3, outer still has 3
      The shadow lived and died inside the braces. Shadowing in an
      inner block is the cheapest way to keep a name you still need.

──── Step 3: The accumulator that never accumulates
  inside the loop, total = 5
  inside the loop, total = 3
  inside the loop, total = 4
  after the loop,  total = 0
      Every iteration built a fresh `total` from the outer 0 and
      threw it away at the closing brace. The sum is lost.
      The tell is a warning that never mentions shadowing:
        warning: variable does not need to be mutable
      An accumulator that does not need `mut` is not accumulating.
      Read that warning as the bug report it is.

──── Step 4: The guard that was never released
  read the counter through its guard: 0
  holding the register guard, value = 1
  counter still locked?  true
  register still locked? true
      BOTH. The first guard did not go anywhere when its name
      was taken — it is alive and holding the lock until the
      brace, with nothing left that could release it early.
  after the brace, counter free? true
      In one thread this only wastes a lock. Add a second thread
      that wants `counter` and it is a deadlock whose cause is a
      line that looks like a rename.

──── Step 5: One name, two concepts — the failure with no warning
  scores      = [5, 2, 4, 0, 3]
  minimum score to count: 3
  threshold   = 5   (the row count, not the minimum score)
  counted     = 1   — expected 3, the scores that are >= 3
      Both bindings are `usize` and both are read, so there is no
      warning to catch it. The compiler cannot know that the first
      `threshold` was a score and the second one a quantity.
      This is the case for a second name. Not because shadowing is
      unsafe — because the two values are not the same thing.

──── Step 6: A shadow hides a function just as happily as a value
  rows_read = 461
      The name is now a usize, so the function is unreachable:
        let again = rows_read();
        error[E0618]: expected function, found `usize`
          | this function of the same name is available here,
          | but it's shadowed by the local binding
      Worth reading once, because it is the only shadowing
      mistake rustc names out loud. Steps 3 to 5 got a misleading
      warning, a correct-looking program, and nothing at all.

──── The rule
  Shadow when the new binding is the SAME CONCEPT in a new form,
  and keep it close to the one it replaces. Reach for a second
  name when it is a different thing (step 5), and never shadow
  something that holds a resource (step 4).
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 18_Ownership/when_to_shadow/examples/when_to_shadow.rs -o /tmp/wts && /tmp/wts
```

## Traps

- **Reaching for a shadow when you wanted `mut`.** If the new value has to outlive the block it is computed in — an accumulator, a running best, anything a loop updates — shadowing cannot do that job at all, and the failure is silent arithmetic rather than an error.
- **Shadowing anything with a `Drop` that matters.** The old value is still alive and still holding whatever it holds, with no name left to release it. Locks are the sharp end; a large buffer held to the end of a long function is the quiet one.
- **Shadowing at distance.** Two lines apart is a refinement a reader follows. Forty lines apart is a redefinition, and the second `let` will read as a new variable to everyone including you.
- **Assuming the compiler is watching.** It warns only when the shadowed binding was *never read*. Read it once — which is normal — and every check in the table above goes quiet.
- **Treating "shadowing is fine" as "shadowing is free".** It is a naming decision, and naming decisions are the ones that survive into every later reading of the code.
- **Reaching for a clippy lint after getting burned once.** The lint that would have caught the bug that burned you is probably `shadow_reuse`, and it fires on every honest parse chain you own. `shadow_unrelated` is the affordable one and it is silent on the accumulator. There is no setting that buys you bug 1 for free — the rule at the top of this page is the mitigation.

## See also

- [Shadowing](../../SHADOWING.md) — the map: all three shadowing lessons, and the pages that touch it
- [Shadowing and `unwrap`](../../17_Option_and_Result/shadowing_and_unwrap/README.md) — what shadowing is, and the folklore that credits it for `Copy`'s work
- [A shadow does not drop](../shadowing_does_not_drop/README.md) — the mechanism behind bug 2, in full, with a value that announces its own death
- [Nothing checks a shadow](../nothing_checks_a_shadow/README.md) — the companion to this page: how little the compiler and clippy will do about any of it
- [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) — how to read the `unused_mut` that is really a bug report
- [Initial values](../../17_Option_and_Result/initial_values/README.md) — the other way to avoid `mut`: declare without initializing and let the compiler prove you assigned
- [`if let`](../../17_Option_and_Result/if_let/README.md) — `let … else`, the guard clause the unwrap-and-narrow idiom opens with
- [The Rust Book, ch. 3.1 — Shadowing ↗](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)

## Po polsku

Reguła w jednym zdaniu: **przesłaniaj, gdy nowe wiązanie to to samo pojęcie w nowej postaci; sięgnij po drugą nazwę, gdy to inna rzecz.** I nigdy nie przesłaniaj czegoś, co trzyma zasób.

Idiomatyczne zastosowania są trzy i wszystkie mieszczą się w regule: konwersja typu przy zachowaniu znaczenia (`let age = age.parse::<u32>()?;` — dalej wiek, tylko już liczba), zawężenie wartości (`let input = input.trim();`), oraz „rozpakowanie” (`let cfg = cfg.unwrap_or_default();`).

Trzy błędy, które się kompilują, i żaden nie zostanie zgłoszony:

- przesłonięcie zmiennej **inną rzeczą** o tej samej nazwie, przez co dalszy kod czyta co innego, niż autor sądzi;
- przesłonięcie w gałęzi `if`, przez co poza gałęzią wraca stara wartość, choć wygląda na zaktualizowaną;
- przesłonięcie strażnika (`MutexGuard`, `File`), które przedłuża trzymanie zasobu do końca bloku, zamiast go zwolnić.

Kompilator łapie tylko jeden przypadek: gdy typy się nie zgadzają. Kiedy się zgadzają — a przy przesłanianiu „tej samej rzeczy w nowej postaci” bardzo często się zgadzają — nie dostaniesz nic. Tym, co by to złapało, jest przegląd kodu albo test, nie `rustc`.

**Szukaj po polsku:** kiedy stosować przesłanianie · idiomatyczny Rust przesłanianie · `rust shadowing best practices` · `clippy::shadow_reuse`
