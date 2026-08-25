# The braces take a name, not an expression

**Level:** 101 → 201 · for newcomers

**One line:** `println!("n is {n}")` finds the variable called `n` — but what goes between the braces is an **identifier**, read by the macro at compile time, and nothing else: `{n + 1}`, `{v.len()}` and `{self.voter}` are each a compile error, each with a different message, and the fix is a `let` on the line above.

This is the feature that makes modern Rust look like it has f-strings. It does not. The resemblance is close enough to be misleading, and the moment you reach past a bare name the difference arrives as an error that talks about punctuation rather than about what you were trying to do.

---

## What it is, and when it arrived

Since **Rust 1.58** (January 2022) a format string can name a variable directly:

```rust
let n = 5;
println!("n is {n}");             // n is 5
println!("n is {}", n);           // the same thing, still fine
```

Both spellings work and neither is deprecated. Every format macro understands the capture — `format!`, `print!`, `eprintln!`, `write!`, `panic!`, `assert!`'s message — because they all go through the same parser.

If you learned Rust from anything written before 2022, or from an example someone wrote before then, you saw only the second form. That is the whole story of why both are everywhere.

## But only a name

The identifier is resolved by the macro, at compile time, using the ordinary name-lookup rules. There is no expression evaluation in there at all, and the parser stops at the first character that cannot continue an identifier:

```rust
println!("{n + 1}");
//  error: invalid format string: expected `}`, found `+`

println!("{scores.len()}");
//  error: invalid format string: expected `}`, found `.`

println!("{scores[0]}");
//  error: invalid format string: expected `}`, found `[`
```

Notice what kind of errors those are. Not one is a type error — the macro cannot even finish **reading the string**. It wanted a name and found punctuation, so it reports the punctuation. That is why the message never mentions the thing you were actually trying to do.

## Field access gets a message of its own

One case is common enough that rustc special-cases it:

```rust
impl Ballot {
    fn show(&self) {
        println!("ballot from {self.voter}");
    }
}
```

```text
error: invalid format string: field access isn't supported
 --> f2.rs:3:45
  |
3 |     fn show(&self) { println!("ballot from {self.voter}"); }
  |                                             ^^^^^^^^^^ not supported in format string
  |
  = note: consider moving this expression to a local variable and then using the local here instead
help: consider using a positional formatting argument instead
  |
3 -     fn show(&self) { println!("ballot from {self.voter}"); }
3 +     fn show(&self) { println!("ballot from {0}", self.voter); }
```

A compiler team wrote a bespoke diagnostic, a note **and** a machine-applicable fix for this one mistake, which tells you how often people make it inside a method.

The special case is narrow, though. It fires on a plain `ident.ident` and nothing more — `{ballot.scores.len()}` and `{ballot.scores[0]}` both fall back to `expected }, found .`, even though they open with a field access. Only the first brace in a method usually gets the good message.

## What the braces *will* take

Three things, and all of them are still names:

```rust
println!("{scores:?}");            // Debug instead of Display — still a name
println!("|{name:>width$}|");      // width, captured by name
println!("{ratio:.prec$}");        // precision, captured by name
```

The `$` is what marks `width` and `prec` as names rather than as literal numbers — `{name:>8}` is eight columns, `{name:>width$}` is however many `width` says. Everything after the `:` is the *format spec*, a separate small language from the capture; `{}` versus `{:?}` is [Debug and Display](../debug_vs_display/README.md), which is a different question and has its own page.

And a literal brace is doubled: `{{` prints `{`, which is what you want when a format string is producing JSON.

## The format string itself must be a literal

This follows from the same fact, and surprises people who reach for a translation table or a runtime template:

```rust
let s = "hello {n}";
println!(s);
//  error: format argument must be a string literal
//  help: you might be missing a string literal to format with
//    |  println!("{}", s);
```

The braces are read when your program is **compiled**. A string assembled at run time arrives long after the only thing that could have read them has finished. Runtime templating in Rust is a library job, not a macro one.

## It reads whatever the name means at that point

The capture is a name lookup, so anything that changes what a name means changes what the braces find — including a shadow:

```rust
let n = 5;
println!("{n}");        // 5
let n = "five";
println!("{n}");        // five
```

Same four characters in the string; two different variables, of two different types. Nothing special is happening — this is exactly what would happen to `n` anywhere else in the function. It is worth seeing once, because a format string does not *look* like a place where name resolution is going on.

## The whole thing, running

<!-- source:braces_take_a_name -->
*[`braces_take_a_name.rs`](examples/braces_take_a_name.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! The braces take a name, not an expression.
//!
//! Since Rust 1.58 you can write `println!("n is {n}")` and the macro will find
//! the variable called `n`. That looks like a Python f-string and it is not one:
//! what goes between the braces is an IDENTIFIER, resolved at compile time by
//! the macro, and nothing else. `{n + 1}`, `{v.len()}` and `{self.voter}` are
//! each a compile error with its own message. The escape hatch is a named
//! argument — or a `let` on the line above, which usually reads better anyway.
//!
//!   rustc --edition 2024 braces_take_a_name.rs -o /tmp/btan && /tmp/btan

fn banner(title: &str) {
    println!("\n──── {title}");
}

struct Ballot {
    voter: &'static str,
    scores: [u8; 3],
}

impl Ballot {
    /// `{self.voter}` is refused, so bind the field first and capture the name.
    fn show(&self) {
        let voter = self.voter;
        let scores = self.scores;
        println!("  {voter} scored {scores:?}");
    }
}

fn main() {
    banner("A name in the braces is looked up like any other name");

    let n = 5;
    let voters = 9;
    println!("  n is {n}, voters is {voters}");
    println!("  the old way still works: n is {}, voters is {}", n, voters);

    banner("But ONLY a name. These three are compile errors:");

    println!("  println!(\"{{n + 1}}\");");
    println!("      error: invalid format string: expected `}}`, found `+`");
    println!("  println!(\"{{scores.len()}}\");");
    println!("      error: invalid format string: expected `}}`, found `.`");
    println!("  println!(\"{{scores[0]}}\");");
    println!("      error: invalid format string: expected `}}`, found `[`");
    println!("  Not one of them is a type error. The macro cannot even finish");
    println!("  reading the string — it wanted a name and found punctuation.");

    banner("Field access gets a message of its very own");

    println!("  println!(\"ballot from {{self.voter}}\");");
    println!("      error: invalid format string: field access isn't supported");
    println!("      help: consider using a positional formatting argument instead");
    println!("  rustc wrote that error for this exact mistake, which tells you");
    println!("  how often people make it. The fix it suggests:");
    let ballot = Ballot { voter: "Ada", scores: [5, 3, 0] };
    ballot.show();

    banner("The two escape hatches");

    let scores = [5, 3, 0];
    println!("  {}", scores.len()); //          positional: the argument list
    println!("  {count}", count = scores.len()); // a named argument
    let count = scores.len(); //                or a `let`, and then a capture
    println!("  {count}");
    println!("  All three print the same 3. The third names the value, which is");
    println!("  the only one of them a reader can still follow at ten lines.");

    banner("Width and precision take a name too — with a trailing $");

    let name = "Ada";
    let width = 8;
    let ratio = 1.0_f64 / 3.0;
    let prec = 2;
    println!("  |{name:>width$}|   <- right-aligned in `width` columns");
    println!("  {ratio:.prec$}        <- `prec` decimal places");

    banner("It reads whatever the name means AT THAT POINT");

    let n = 5;
    println!("  {n}");
    let n = "five";
    println!("  {n}   <- same four characters in the string, different variable");
    println!("  The capture is a name lookup, so a shadow changes what it finds.");

    banner("The format string itself must be a literal");

    println!("  let s = \"hello {{n}}\";");
    println!("  println!(s);");
    println!("      error: format argument must be a string literal");
    println!("  The braces are read at COMPILE time, so a string assembled at");
    println!("  run time has nothing left to read them.");

    banner("And a real brace is doubled");

    println!("  To print {{}} you write {{{{}}}} — and {name} still captures alongside it.");
    let json = format!("{{\"voter\": \"{name}\"}}");
    println!("  {json}");
}
```
<!-- /source -->

<!-- output:braces_take_a_name -->
*Verified output of [`braces_take_a_name.rs`](examples/braces_take_a_name.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── A name in the braces is looked up like any other name
  n is 5, voters is 9
  the old way still works: n is 5, voters is 9

──── But ONLY a name. These three are compile errors:
  println!("{n + 1}");
      error: invalid format string: expected `}`, found `+`
  println!("{scores.len()}");
      error: invalid format string: expected `}`, found `.`
  println!("{scores[0]}");
      error: invalid format string: expected `}`, found `[`
  Not one of them is a type error. The macro cannot even finish
  reading the string — it wanted a name and found punctuation.

──── Field access gets a message of its very own
  println!("ballot from {self.voter}");
      error: invalid format string: field access isn't supported
      help: consider using a positional formatting argument instead
  rustc wrote that error for this exact mistake, which tells you
  how often people make it. The fix it suggests:
  Ada scored [5, 3, 0]

──── The two escape hatches
  3
  3
  3
  All three print the same 3. The third names the value, which is
  the only one of them a reader can still follow at ten lines.

──── Width and precision take a name too — with a trailing $
  |     Ada|   <- right-aligned in `width` columns
  0.33        <- `prec` decimal places

──── It reads whatever the name means AT THAT POINT
  5
  five   <- same four characters in the string, different variable
  The capture is a name lookup, so a shadow changes what it finds.

──── The format string itself must be a literal
  let s = "hello {n}";
  println!(s);
      error: format argument must be a string literal
  The braces are read at COMPILE time, so a string assembled at
  run time has nothing left to read them.

──── And a real brace is doubled
  To print {} you write {{}} — and Ada still captures alongside it.
  {"voter": "Ada"}
```
<!-- /output -->

## If you are coming from another language

- **Python.** This is the bridge that matters, and it is the one that misleads. An f-string takes a full **expression**: `f"{n + 1}"`, `f"{ballot.voter}"`, `f"{scores[0]}"` and `f"{total / count:.2f}"` all work. Rust's capture takes a name only. What transfers is the *look* and the `:spec` after the colon — `{x:>8}` and `{x:.2}` mean much what they do in Python. What changes is everything inside the braces before the colon. Expect to write a `let` above the line; a Python habit of computing in the string is the single most common way to meet this error.
- **JavaScript.** Template literals — `` `total: ${a + b}` `` — take expressions too, and are interpolated at run time rather than compiled into a call. Same divergence as Python, plus the literal rule: a JS template can be built from a variable, a Rust format string cannot.
- **C.** `printf("%d\n", n)` has no names at all, and no type checking either — a wrong specifier is undefined behaviour that compilers merely *warn* about. Rust's positional `{}` is the closest relative, with the type worked out by the compiler instead of spelled by you, and a mismatch between holes and arguments refused rather than warned about.
- **ABAP.** `|Total: { lv_total }|` string templates are the nearest thing, and they take expressions — including method calls with `{ lo_obj->get_total( ) }` — evaluated at run time. So the Rust restriction has no ABAP counterpart; what does transfer is the formatting-option syntax after the value, which does a similar job to Rust's `:spec`.

## Traps

- **Assuming it is an f-string.** It is a name, not an expression. If you are reaching for `.`, `[`, `(` or an operator inside the braces, stop and write a `let`.
- **Reading the punctuation error literally.** `expected }, found .` is true and unhelpful. It means "that is not a name."
- **Using `{self.field}` in a method.** Refused with a dedicated message. Bind the field first, or take rustc's suggestion and use a positional argument.
- **Trusting the good error message to keep appearing.** `{self.voter}` gets the bespoke diagnostic; `{self.scores[0]}` does not. Same mistake, different message.
- **Mixing capture and positional and losing count.** `println!("{a} {} {b}", x)` is legal — captures and arguments coexist — and it is exactly the line nobody can read in six months. Pick one style per call.
- **Forgetting `{{` in a format string that produces braces.** JSON and CSS built with `format!` need `{{` and `}}`, and the error if you forget is again about the format string, not about your data.
- **Expecting a variable format string to work.** It must be a literal. This is not a limitation you can argue with; it is what makes the compile-time checking possible.

## Practice

**The f-string that isn't.** Take a report line written the way a Python programmer would write it — the voter's name from a field, a count from `.len()`, the first score by index, and a mean from an arithmetic expression, all inside the braces — and compile it.

Read all four errors before fixing anything, and write down which of them told you what you actually did wrong.

Then make it compile three separate ways: with a `let` above the line and a capture, with named arguments in the call, and with positional arguments. All three print the same sentence.

Then choose one to ship, and be able to say why. Add a fifth value in the middle of the line for each version before you decide — one of the three makes you renumber, one makes you scroll, and one is a single new `let`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:braces_take_a_name_kata -->
*[`braces_take_a_name_kata.rs`](examples/braces_take_a_name_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the f-string that isn't.
//!
//! A report written the way a Python programmer would write it, then made to
//! compile. Four braces, four different refusals, and three fixes to choose
//! between — a `let` above the line, a named argument, or a positional one.
//! The last part is the judgement call: only one of the three scales.
//!
//!   rustc --edition 2024 braces_take_a_name_kata.rs -o /tmp/btank && /tmp/btank

fn banner(title: &str) {
    println!("\n──── {title}");
}

struct Ballot {
    voter: &'static str,
    scores: [u32; 3],
}

impl Ballot {
    fn total(&self) -> u32 {
        self.scores.iter().sum()
    }
}

fn main() {
    let ballot = Ballot { voter: "Ada", scores: [5, 3, 4] };

    banner("What a Python programmer writes, and what each brace gets back");

    println!("  println!(\"{{ballot.voter}}\");");
    println!("      error: invalid format string: field access isn't supported");
    println!("      help: consider using a positional formatting argument instead");
    println!();
    println!("  println!(\"{{ballot.scores.len()}} candidates\");");
    println!("      error: invalid format string: expected `}}`, found `.`");
    println!();
    println!("  println!(\"first: {{ballot.scores[0]}}\");");
    println!("      error: invalid format string: expected `}}`, found `.`");
    println!();
    println!("  println!(\"mean: {{ballot.total() / 3}}\");");
    println!("      error: invalid format string: expected `}}`, found `.`");
    println!();
    println!("  Only the first one is diagnosed as what it is. The other three");
    println!("  stop at the first character that cannot continue an identifier,");
    println!("  so the message names the punctuation and not your intent.");

    banner("Fix A: a `let` above the line, then capture the name");

    let voter = ballot.voter;
    let candidates = ballot.scores.len();
    let first = ballot.scores[0];
    let mean = ballot.total() / candidates as u32;
    println!("  {voter}: {candidates} candidates, first {first}, mean {mean}");

    banner("Fix B: a named argument — the name lives in the call");

    println!(
        "  {voter}: {candidates} candidates, first {first}, mean {mean}",
        voter = ballot.voter,
        candidates = ballot.scores.len(),
        first = ballot.scores[0],
        mean = ballot.total() / 3,
    );

    banner("Fix C: positional — no names at all");

    println!(
        "  {}: {} candidates, first {}, mean {}",
        ballot.voter,
        ballot.scores.len(),
        ballot.scores[0],
        ballot.total() / 3,
    );

    banner("Which one to ship");

    println!("  All three print the same line. They are not equally readable.");
    println!();
    println!("  C is the one that rots. With four holes you are already counting");
    println!("  arguments against braces by eye, and inserting a fifth in the");
    println!("  middle silently renumbers everything after it.");
    println!();
    println!("  B keeps the names but spends four lines inventing them at the");
    println!("  call site, where they exist only until the semicolon.");
    println!();
    println!("  A is the default. The names outlive the print, so the next line");
    println!("  can reuse them; the types are visible to the reader and to the");
    println!("  compiler's error messages; and the format string reads as the");
    println!("  sentence it is going to become. Reach for C only when there is");
    println!("  exactly one hole and the expression is short.");

    banner("The one C is genuinely for");

    println!("  {}", ballot.total()); //  one hole, one expression, no ceremony
    println!("  ...and inside a method, where `{{self.voter}}` is refused, a");
    println!("  positional argument is often lighter than binding every field.");
}
```
<!-- /source -->

<!-- output:braces_take_a_name_kata -->
*Verified output of [`braces_take_a_name_kata.rs`](examples/braces_take_a_name_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── What a Python programmer writes, and what each brace gets back
  println!("{ballot.voter}");
      error: invalid format string: field access isn't supported
      help: consider using a positional formatting argument instead

  println!("{ballot.scores.len()} candidates");
      error: invalid format string: expected `}`, found `.`

  println!("first: {ballot.scores[0]}");
      error: invalid format string: expected `}`, found `.`

  println!("mean: {ballot.total() / 3}");
      error: invalid format string: expected `}`, found `.`

  Only the first one is diagnosed as what it is. The other three
  stop at the first character that cannot continue an identifier,
  so the message names the punctuation and not your intent.

──── Fix A: a `let` above the line, then capture the name
  Ada: 3 candidates, first 5, mean 4

──── Fix B: a named argument — the name lives in the call
  Ada: 3 candidates, first 5, mean 4

──── Fix C: positional — no names at all
  Ada: 3 candidates, first 5, mean 4

──── Which one to ship
  All three print the same line. They are not equally readable.

  C is the one that rots. With four holes you are already counting
  arguments against braces by eye, and inserting a fifth in the
  middle silently renumbers everything after it.

  B keeps the names but spends four lines inventing them at the
  call site, where they exist only until the semicolon.

  A is the default. The names outlive the print, so the next line
  can reuse them; the types are visible to the reader and to the
  compiler's error messages; and the format string reads as the
  sentence it is going to become. Reach for C only when there is
  exactly one hole and the expression is short.

──── The one C is genuinely for
  12
  ...and inside a method, where `{self.voter}` is refused, a
  positional argument is often lighter than binding every field.
```
<!-- /output -->

</details>

## See also

- [Debug and Display](../debug_vs_display/README.md) — what `{}` and `{:?}` actually ask the type for, which is the other half of every format string
- [A block is an expression](../a_block_is_an_expression/README.md) — the `let` you write above the line, and why braces have a value at all
- [When to shadow](../../18_Ownership/when_to_shadow/README.md) — because a capture reads whatever the name means at that point
- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) — how far "that point" extends, and why the answer differs for the name, the value and a borrow of it
- [What a warning is asking](../what_a_warning_is_asking/README.md) — the `unused variable` you get when you bind a value for a print and then delete the print
