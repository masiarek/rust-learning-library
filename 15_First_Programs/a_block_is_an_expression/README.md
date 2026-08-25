# A block is an expression

**Level:** 101 → 201 · for newcomers

**One line:** `{ }` does two jobs — it opens a **scope** that ends at its closing brace, and it is an **expression** whose value is its last line written *without* a semicolon — and the second job is why a function body needs no `return`, why `if` can sit on the right of a `let`, and why adding one character can change a function's type.

Most people meet the first job on day one and the second one by accident, as an error message. They are worth learning together, because nearly every *"wait, Rust lets you write that?"* moment in a first month — a function body with no `return`, a `let` whose right-hand side is an `if`, a `match` used as a value — is the second job doing something a statement-oriented language cannot do at all.

---

## Job 1: it opens a scope, anywhere you like

A block does not need a `fn`, an `if`, or a loop to attach to. Braces on their own are a perfectly good statement:

```rust
let n = 5;
{
    let n = 10;
    println!("inner n is: {n}");   // inner n is: 10
}
println!("outer n is: {n}");       // outer n is: 5
```

The inner `n` is a second variable that exists for three lines. At the closing brace it is gone, and the name goes back to meaning the outer one — which was never touched.

Two things worth being precise about here, because this snippet circulates as a shadowing demo and it is the *least* interesting kind:

- **This is shadowing, but the boring kind.** An inner scope hiding an outer name is the textbook definition, and C, C++, Java, JavaScript and Python all do it. What is unusual about Rust is a second `let` of the same name in the *same* scope, which needs no braces at all — see [Shadowing and `unwrap`](../../17_Option_and_Result/shadowing_and_unwrap/README.md), or [SHADOWING.md](../../SHADOWING.md) for the whole set. This page is about the braces, not about the shadow.
- **A name declared inside really is gone afterwards.** Not hidden, not shadowed — absent:

  ```rust
  { let a = 1; }
  println!("{a}");   // error[E0425]: cannot find value `a` in this scope
  ```

## Job 2: it has a value

This is the half that is genuinely new if you are coming from C, Python, Java or ABAP. A block **evaluates to something**, and that something is its last line written without a semicolon:

```rust
let quorum = {
    let voters = 9;
    let half = voters / 2;
    half + 1                  // no semicolon: this is what the block is worth
};
println!("{quorum}");         // 5
```

`voters` and `half` are working names that never escape. What comes out is one value, bound to one immutable name. There is no helper function, no placeholder, and no `mut`.

That last line is called the **tail expression**, and everything else about Rust's syntax falls out of it.

## The semicolon is the switch

A semicolon turns an expression into a *statement*, and statements have no value — so the block's value becomes `()`, the unit type:

```rust
let with_tail = { tally() };    // 6
let with_semi = { tally(); };   // ()
```

Same block, one character apart, two different types. This is not a corner case you can avoid; it is the single most common first-week compile error in Rust, because a function body is just a block:

```rust
fn double(n: i32) -> i32 {
    n * 2;                      // <- the semicolon throws the value away
}
```

```text
error[E0308]: mismatched types
 --> p2.rs:1:22
  |
1 | fn double(n: i32) -> i32 {
  |    ------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     n * 2;
  |          - help: remove this semicolon to return this value
```

Read where the `^^^` points: at the **return type**, not at the semicolon. The compiler is saying "you promised `i32` and this body produces `()`", and the reason is four lines further down in the `help:`. Beginners routinely respond by changing the signature, which makes the error go away and the function useless.

The other direction has a lint of its own. `return n * 2;` in the tail position compiles fine, and `clippy::needless_return` — warn-by-default, so you do not have to opt in — will ask you to drop it:

```text
warning: unneeded `return` statement
help: remove `return`
  |
2 -     return n * 2;
2 +     n * 2
```

`return` still earns its keep for an **early** exit from the middle of a function. In the last line it is noise.

## Which is why `if`, `match` and `loop` are expressions too

None of these is a special form. They are built out of blocks, so they inherit blocks' value:

```rust
let verdict = if turnout >= 50 { "quorate" } else { "short" };

let label = match score {
    5 => "excellent",
    3..=4 => "good",
    _ => "weak",
};
```

The `if` arms are blocks; `"quorate"` is a tail expression. This is also why an `if` used as a value **must** have an `else` — a missing branch would leave a path with no value — and why every `match` arm has to produce the same type. Both rules stop being arbitrary once you see the block underneath.

## What you actually reach for it for

Three jobs, in rough order of how often they come up.

**Scope the `mut` to the building.** Mutation is how you build a collection; it is not something the rest of the function needs to keep the right to do.

```rust
let ballots = {
    let mut v = Vec::new();
    v.push(5);
    v.push(3);
    v.push(4);
    v
};
// `ballots` is not `mut`. Nothing below this line can push to it.
```

**Give a shadow an end.** A shadow normally runs to the end of its scope; a block chooses that end deliberately — which is the standard fix for the one shadowing bug the compiler will not catch. See [When to shadow](../../18_Ownership/when_to_shadow/README.md), and [A name is not a place](../../18_Ownership/a_name_is_not_a_place/README.md) for this same idiom seen from the shadow's side — including the qualifier that cuts the other way, that a *same-scope* shadow does not end early at all.

**End a borrow early.** A borrow taken inside a block cannot outlive it, so the block is the tool when you need the original writable again. Note the caveat: since Rust 2018 a borrow ends at its **last use** rather than at the closing brace, so most of the `{ }` blocks you will see doing this in older code and older tutorials are no longer necessary. It is still the answer when the compiler disagrees with you about where the last use was. See [Borrowing](../../18_Ownership/borrowing/README.md) for the rule, and [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) for why a name's scope and a borrow's region are two different measurements that happen to share a word.

A fourth, rarer and worth knowing about: **releasing a lock**. A `MutexGuard` unlocks when it drops, and it drops at the end of its scope — so wrapping the critical section in a block is how you unlock before the function ends. [Lock poisoning](../../09_Advanced/mutex_poisoning/README.md) covers what happens when that goes wrong.

## The whole thing, running

<!-- source:a_block_is_an_expression -->
*[`a_block_is_an_expression.rs`](examples/a_block_is_an_expression.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! A block is an expression.
//!
//! `{ }` does two jobs, and most people only ever meet the first. It opens a
//! SCOPE — a name declared inside stops existing at the closing brace. It is
//! also an EXPRESSION — its value is its last line written without a semicolon.
//! The second job is why a function body needs no `return`, why `if` can sit on
//! the right-hand side of a `let`, and why adding one semicolon changes what a
//! block is worth from an `i32` to `()`.
//!
//!   rustc --edition 2024 a_block_is_an_expression.rs -o /tmp/abie && /tmp/abie

fn banner(title: &str) {
    println!("\n──── {title}");
}

/// A function body IS a block, and `n * 2` is its tail expression.
fn double(n: i32) -> i32 {
    n * 2
}

/// Something to call when the point is the semicolon, not the arithmetic.
fn tally() -> i32 {
    6
}

fn main() {
    banner("Job 1: it opens a scope, anywhere you like");

    let n = 5;
    println!("  outer n is: {n}");
    {
        let n = 10; //      a second variable; the brace below ends it
        println!("    inner n is: {n}");
    }
    println!("  outer n is: {n}   <- the outer one was never touched");

    banner("...and a name declared inside is gone at the brace");

    println!("  {{ let a = 1; }}");
    println!("  println!(\"{{a}}\");   <- error[E0425]: cannot find value `a`");
    println!("  No subtlety: past the brace, the name does not exist.");

    banner("Job 2: it has a VALUE — its last line, with no semicolon");

    let quorum = {
        let voters = 9;
        let half = voters / 2;
        half + 1 //         no semicolon: this is what the block is worth
    };
    println!("  quorum = {quorum}");

    banner("The semicolon is the switch");

    let with_tail = { tally() };
    let with_semi = { tally(); };
    println!("  {{ tally() }}    is {with_tail}");
    println!("  {{ tally(); }}   is {with_semi:?}      <- the unit value");
    println!("  Same block, one character apart, two different types.");

    banner("So a function body was a block all along");

    println!("  fn double(n: i32) -> i32 {{ n * 2 }}");
    println!("  double(4) = {}   <- a tail expression, not a `return`", double(4));

    banner("What it is FOR (1): scoping the `mut` to the building");

    let ballots = {
        let mut v = Vec::new();
        v.push(5);
        v.push(3);
        v.push(4);
        v //            hand the finished Vec out; the `mut` stays behind
    };
    println!("  ballots = {ballots:?}");
    println!("  `ballots` is not `mut`, and no line below here can grow it.");

    banner("What it is FOR (2): giving a shadow an end");

    let name = String::from("ada");
    {
        let name = name.to_uppercase();
        println!("    inside:  {name}");
    }
    println!("  outside: {name}   <- the shadow ended at the brace");

    banner("What it is FOR (3): the branch that decides a value");

    let turnout = 61;
    let verdict = if turnout >= 50 { "quorate" } else { "short" };
    println!("  turnout {turnout}% -> {verdict}");
    println!("  `if` is an expression because its arms are blocks.");
}
```
<!-- /source -->

<!-- output:a_block_is_an_expression -->
*Verified output of [`a_block_is_an_expression.rs`](examples/a_block_is_an_expression.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Job 1: it opens a scope, anywhere you like
  outer n is: 5
    inner n is: 10
  outer n is: 5   <- the outer one was never touched

──── ...and a name declared inside is gone at the brace
  { let a = 1; }
  println!("{a}");   <- error[E0425]: cannot find value `a`
  No subtlety: past the brace, the name does not exist.

──── Job 2: it has a VALUE — its last line, with no semicolon
  quorum = 5

──── The semicolon is the switch
  { tally() }    is 6
  { tally(); }   is ()      <- the unit value
  Same block, one character apart, two different types.

──── So a function body was a block all along
  fn double(n: i32) -> i32 { n * 2 }
  double(4) = 8   <- a tail expression, not a `return`

──── What it is FOR (1): scoping the `mut` to the building
  ballots = [5, 3, 4]
  `ballots` is not `mut`, and no line below here can grow it.

──── What it is FOR (2): giving a shadow an end
    inside:  ADA
  outside: ada   <- the shadow ended at the brace

──── What it is FOR (3): the branch that decides a value
  turnout 61% -> quorate
  `if` is an expression because its arms are blocks.
```
<!-- /output -->

## If you are coming from another language

- **Python.** Neither job transfers. An `if` or `for` body is **not** a scope — a name bound inside it is still there afterwards, which is why Python needs `nonlocal` and `global` and Rust does not. And a block has no value: `a if c else b` and the walrus `:=` exist precisely because there is no way to give a multi-statement block one, so anything needing several steps becomes a helper function. In Rust the braces do that job, and the working names stay inside.
- **ABAP.** Also neither. `DATA` is routine-wide, so `IF … ENDIF` introduces no scope at all and a name declared inside a loop is visible after it — the `lv_` / `lt_` prefixes exist partly because every local in a routine shares one namespace. There is no block expression either: a value built in several steps needs a routine-level variable or its own `FORM`/method. What you gain is that a working name genuinely stops existing; what you give up is ABAP's free guarantee that one name in a routine means one thing.
- **C and C++.** Job 1 transfers exactly — block scope, nesting, the lot. Job 2 does not: a C block has no value, which is why GCC and Clang ship `({ … })` **statement expressions** as a non-standard extension, and why macros that need several steps depend on it. Rust's version is the ordinary language, not an extension.
- **JavaScript.** Job 1 transfers if you use `let`/`const` (`var` is function-scoped and ignores blocks). Job 2 does not, and the workaround is the IIFE — `(() => { … })()` — which is exactly what a Rust block expression replaces, minus the function call and the closure.
- **Java.** Block scope yes, block value no. A `switch` *expression* arrived in Java 14 with `yield` for the multi-statement case, which is Java retrofitting the tail expression onto one construct; Rust has it on all of them because it never had the split.

## Traps

- **Changing the signature to silence `E0308`.** The error points at the return type because that is what was promised. The cause is the `help:` line — a semicolon. Fix the body.
- **Deleting the tail line's neighbour and taking the tail with it.** You get caught two different ways, which is lucky: `{ let x = 5; }` compiles as `()` and warns `unused variable`, while `{ let x = 5 }` is a **syntax** error — `expected one of ., ;, ?, else, or an operator, found }` — because a `let` is a statement and needs its semicolon before the brace.
- **Writing `return` in the tail.** Legal, and `clippy::needless_return` will ask you not to. Keep `return` for genuine early exits.
- **Reaching for a block to end a borrow that already ended.** Non-lexical lifetimes made most of those blocks unnecessary in 2018. Try deleting it; the compiler will tell you if it was load-bearing.
- **Reading the nested-block snippet as the shadowing lesson.** It is the version every language has. The Rust-specific one is a second `let` in the same scope, and it needs no braces — [SHADOWING.md](../../SHADOWING.md) is the map.
- **Expecting `if` without `else` to have a value.** It has one, and the compiler says which: `let x = if c { 5 };` is `error[E0317]: \`if\` may be missing an \`else\` clause`, with the note *"`if` expressions without `else` evaluate to `()`"*. Every path has to produce the type you asked for, and a missing branch is a path.

## Practice

**The semicolon that changed the type.** Four parts, and the first is a compile error worth causing on purpose.

Write a `mean(scores: &[u32]) -> u32` whose body ends in `total / n;` — with the semicolon — and read the error before fixing it. Say out loud which line the `^^^` is under and which line actually caused it.

Then take a function that builds a `Vec` with a `mut` accumulator and reshape it so that the `mut` binding never escapes the braces it was needed in.

Then take an `if`/`else if`/`else` chain that assigns into a `mut` placeholder in every branch, and make the `if` itself the value. Delete the final `else` and predict the error before you read it.

Finally, take a borrow that the compiler complains outlives its welcome, wrap it in a block, and then check whether the block was needed at all by deleting it again.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:a_block_is_an_expression_kata -->
*[`a_block_is_an_expression_kata.rs`](examples/a_block_is_an_expression_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the semicolon that changed the type.
//!
//! Four parts, and the first one is a compile error worth causing on purpose.
//! (1) A tail expression grows a semicolon and the function's return type stops
//! matching. (2) A `mut` builder is sealed behind a block expression, so what
//! escapes is an immutable binding. (3) An assign-in-every-branch `mut` becomes
//! an `if` expression, because `if` is built from blocks and therefore has a
//! value. (4) A borrow is given an end by putting it in a block.
//!
//!   rustc --edition 2024 a_block_is_an_expression_kata.rs -o /tmp/abiek && /tmp/abiek

fn banner(title: &str) {
    println!("\n──── {title}");
}

/// The fixed version. With `;` after `total / n` this is `E0308`.
fn mean(scores: &[u32]) -> u32 {
    let total: u32 = scores.iter().sum();
    let n = scores.len() as u32;
    total / n
}

fn main() {
    banner("Part 1: the semicolon that changed the type");

    println!("  fn mean(scores: &[u32]) -> u32 {{");
    println!("      let total: u32 = scores.iter().sum();");
    println!("      let n = scores.len() as u32;");
    println!("      total / n;          <- one character, and the body is ()");
    println!("  }}");
    println!();
    println!("  error[E0308]: mismatched types");
    println!("     |    ------            ^^^ expected `u32`, found `()`");
    println!("     |    |");
    println!("     |    implicitly returns `()` as its body has no tail");
    println!("     |    or `return` expression");
    println!("     |     total / n;");
    println!("     |              - help: remove this semicolon to return this value");
    println!();
    println!("  Read the help line: rustc is not asking for a `return`. The body");
    println!("  is a block, the block's value is its tail, and a semicolon threw");
    println!("  the tail away. Without it:");
    println!("      mean(&[5, 3, 4]) = {}", mean(&[5, 3, 4]));

    banner("Part 2: the builder that hands out something immutable");

    let raw = [("Cara", 5), ("Ada", 4), ("Ben", 2), ("Dev", 4)];
    let cutoff = 4;

    //  Everything mutable happens inside the braces; an immutable Vec comes out.
    let through = {
        let mut v = Vec::new();
        for (name, score) in &raw {
            if *score >= cutoff {
                v.push(*name);
            }
        }
        v.sort_unstable();
        v
    };

    println!("  cutoff {cutoff} -> {through:?}");
    println!("  `mut` lived for six lines inside the block. The binding that");
    println!("  escaped is plain, so nothing below can push to it.");

    banner("Part 3: the branch that IS the value");

    for turnout in [61, 50, 12] {
        //  Not: let mut verdict = ""; if … { verdict = … } else { … }
        let verdict = if turnout >= 50 {
            "quorate"
        } else if turnout >= 25 {
            "advisory only"
        } else {
            "void"
        };
        println!("  turnout {turnout:>3}% -> {verdict}");
    }
    println!("  No `mut`, no placeholder value, and the compiler checks that");
    println!("  every arm produced one — a missing `else` would not compile.");

    banner("Part 4: the borrow that ends where you say");

    let mut ballots = vec![5, 3, 4];
    let top = {
        let view = &ballots; //     the borrow starts here...
        *view.iter().max().unwrap()
    }; //                           ...and cannot outlive this brace
    ballots.push(9); //             so the Vec is writable again immediately
    println!("  top before the push: {top}");
    println!("  ballots now: {ballots:?}");
    println!();
    println!("  Since 2018 a borrow usually ends at its last USE, so most code");
    println!("  no longer needs this. It is still the tool when the compiler");
    println!("  disagrees with you about where the last use was.");
}
```
<!-- /source -->

<!-- output:a_block_is_an_expression_kata -->
*Verified output of [`a_block_is_an_expression_kata.rs`](examples/a_block_is_an_expression_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Part 1: the semicolon that changed the type
  fn mean(scores: &[u32]) -> u32 {
      let total: u32 = scores.iter().sum();
      let n = scores.len() as u32;
      total / n;          <- one character, and the body is ()
  }

  error[E0308]: mismatched types
     |    ------            ^^^ expected `u32`, found `()`
     |    |
     |    implicitly returns `()` as its body has no tail
     |    or `return` expression
     |     total / n;
     |              - help: remove this semicolon to return this value

  Read the help line: rustc is not asking for a `return`. The body
  is a block, the block's value is its tail, and a semicolon threw
  the tail away. Without it:
      mean(&[5, 3, 4]) = 4

──── Part 2: the builder that hands out something immutable
  cutoff 4 -> ["Ada", "Cara", "Dev"]
  `mut` lived for six lines inside the block. The binding that
  escaped is plain, so nothing below can push to it.

──── Part 3: the branch that IS the value
  turnout  61% -> quorate
  turnout  50% -> quorate
  turnout  12% -> void
  No `mut`, no placeholder value, and the compiler checks that
  every arm produced one — a missing `else` would not compile.

──── Part 4: the borrow that ends where you say
  top before the push: 5
  ballots now: [5, 3, 4, 9]

  Since 2018 a borrow usually ends at its last USE, so most code
  no longer needs this. It is still the tool when the compiler
  disagrees with you about where the last use was.
```
<!-- /output -->

</details>

## See also

- [The braces take a name, not an expression](../braces_take_a_name/README.md) — the other thing every line above is quietly using: `{n}` in a format string
- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) — the word Job 1 uses, and the two other things it is asked to mean: when a value dies, and when a borrow stops mattering
- [What a warning is asking](../what_a_warning_is_asking/README.md) — `unused variable` on a block's working name, and what the two answers to it mean
- [When to shadow](../../18_Ownership/when_to_shadow/README.md) — where "give the shadow an end" is the fix
- [A shadow does not drop](../../18_Ownership/shadowing_does_not_drop/README.md) — why a block is the way to free something early, when nothing else will
- [Borrowing](../../18_Ownership/borrowing/README.md) — the last-use rule, and when a block is still the answer
- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) — what job 1 actually ends, and the two other things "out of scope" is asked to mean
- [Initial values](../../17_Option_and_Result/initial_values/README.md) — the other route away from `mut`: declare without initializing and let the compiler prove you assigned
- [SHADOWING.md](../../SHADOWING.md) — the map, if the nested-`n` snippet is what brought you here
