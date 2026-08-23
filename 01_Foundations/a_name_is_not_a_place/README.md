# A name is not a place

**Level:** 201 · working knowledge

**One line:** `mut` writes into a place; a shadow builds a **second place** and moves the name onto it — and the way to prove that is not to print addresses but to take a reference, because the borrow checker accepts the shadow and rejects the `mut` spelling of the same four lines.

Four pages here already cover shadowing: [Shadowing and `unwrap`](../shadowing_and_unwrap/README.md) on what it is for, [A shadow does not drop](../shadowing_does_not_drop/README.md) on what happens to the value underneath, [When to shadow](../when_to_shadow/README.md) on whether to reach for it, and [Nothing checks a shadow](../nothing_checks_a_shadow/README.md) on what tooling will not save you. [SHADOWING.md](../../SHADOWING.md) is the map. This page answers the question that comes *before* all of them, and that every comparison table on the internet answers slightly wrong: **what, mechanically, is the difference from `mut`?**

The confusion is always the same one, and it has a name: conflating a **name** with a **place**. A name is a label you write in source code. A place is where a value lives. `mut` gives you one name and one place, and lets you write into the place. A shadow gives you one name and two places, and moves the name from the first to the second.

---

## The proof, in four lines

Take a reference before the shadow, and read it after:

```rust
let x = 5;
let y = &x;     // borrows the FIRST x
let x = 6;      // a second place; the name moves onto it
println!("y = {y}, x = {x}");   // y = 5, x = 6
```

Both values are alive at the same time, under one name. `y` still reads the first place, which nothing has touched — the shadow did not change a value, it changed what the *name* means. Now write the same idea with `mut`:

```rust
let mut x = 5;
let y = &x;
x = 6;                          // <- there is only one place, and y is watching it
println!("y = {y}, x = {x}");
```

That does not compile:

```text
error[E0506]: cannot assign to `x` because it is borrowed
 --> b_mut.rs:4:5
  |
3 |     let y = &x;
  |             -- `x` is borrowed here
4 |     x = 6;
  |     ^^^^^ `x` is assigned to here but it was already borrowed
5 |     println!("y = {y}, x = {x}");
  |                    - borrow later used here
```

This is the whole lesson. The two spellings are not two styles for one operation — one of them is a **write**, which the borrow checker polices, and the other is a **declaration**, which it has no reason to object to. That the compiler treats them differently is not a technicality; it is the evidence that they were never the same thing.

And it is not a `Copy` trick — a mistake this repo has [a whole page about](../shadowing_and_unwrap/README.md), because tutorials routinely credit shadowing for what `Copy` is quietly doing. The same shape works on a `String`, which is not `Copy` at all:

```rust
let name = String::from("Ada");
let seen = &name;                  // borrows the first String
let name = name.to_uppercase();    // a second String, in a second place
println!("{seen} {name}");         // Ada ADA
```

The first `String` is still allocated, still owned, and still borrowable through `seen`. Nothing was replaced. See [borrowing](../borrowing/README.md) for why `seen` stays valid, and [A shadow does not drop](../shadowing_does_not_drop/README.md) for when that first `String` is finally freed.

## Why the address demonstration is the weaker one

The popular way to show this is to print `&x` on both sides of a shadow and observe two different numbers. It is not wrong, but it proves less than it looks, and it is worth knowing why before you rely on it.

Addresses are an **implementation detail the demonstration itself creates**. Two bindings are two places in the language's model whether or not they get two stack slots, and the compiler is free to reuse one slot when nothing can tell. Taking `&x` is precisely what forces them apart — you observe two addresses because you asked for two addresses. Run the same program with no references in it and there is nothing left to measure.

Worse, the numbers can point the wrong way. Here is the reverse case, measured with `rustc` 1.97.1: a shadow that reuses one heap buffer, next to a `mut` assignment that abandons the one it had.

```text title="One machine's addresses — illustrative, not an answer key"
let mut s = String::from("alpha");   binding 0x…4770   heap 0x…61c0
s = String::from("bravo");           binding 0x…4770   heap 0x…61d0   <- new buffer, old one freed
s.push_str("!");                     binding 0x…4770   heap 0x…61d0   <- THIS is in-place mutation

let t = String::from("charlie");                       heap 0x…61c0
let t = t;                          // shadow by move  heap 0x…61c0   <- same buffer, nothing allocated
```

So "shadowing allocates, `mut` does not" is false in both directions on the same afternoon. The reference test has no such problem: it asks the compiler a question about *meaning*, and the compiler answers in a diagnostic rather than in a number that changes every run.

## `mut` does not "edit the value in place" either

The other half of the confusion. `x = 6` on an `i32` really does overwrite four bytes. But whole-value assignment on anything that owns something is not an edit — the old value is **dropped**, right there at the assignment, and a new one is moved in:

<!-- output:a_name_is_not_a_place -->
*Verified output of [`a_name_is_not_a_place.rs`](examples/a_name_is_not_a_place.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Two places, one name, both alive
  y = 5   <- the place y borrowed, untouched
  x = 6   <- the place the name means now

──── Not a `Copy` trick — the same thing with a String
  seen = Ada   <- still the first String, still owned
  name = ADA   <- the name now means the new one

──── The `mut` spelling of those four lines does not compile
  let mut x = 5;
  let y = &x;
  x = 6;        <- error[E0506]: cannot assign to `x`
                                 because it is borrowed
  One place, and `y` is watching it. There is no second place
  to put the 6 in, so the write has to be refused.

──── So `mut` does not 'edit the value in place' either
  mut:    slot holds first
    drop: first
  mut:    assigned; the drop above already happened
  mut:    slot holds second
    drop: second

  shadow: slot holds first
  shadow: shadowed; nothing has dropped
  shadow: slot holds second
    drop: second
    drop: first

──── Mutability belongs to the binding, not to the value
  s = Hello, world
  One String, three bindings, two answers to "is it mutable?"
  The value never had an opinion; only the binding does.

──── They are not opposites, so they combine
  weight = 61.5 kg

──── Where each one ends: a block
  shadow: inside the block,  quorum  = 100
  shadow: after it,          quorum  = 5    <- restored
  mut:    inside the block,  turnout = 100
  mut:    after it,          turnout = 100  <- kept
  block:  handed out of it,  quorum  = 101  <- carried
```
<!-- /output -->

Read the `mut:` block: `drop: first` prints *before* the line after the assignment. The old value did not get updated; it was destroyed on the spot. In the `shadow:` block nothing drops at all until the closing brace, and then both do, newest first.

In-place mutation is the third line of that first `String` example — `s.push_str("!")`, a method taking `&mut self` that reaches into the value and changes it. That is a different operation from `s = …`, and only the first one deserves the phrase.

## The comparison table that circulates, row by row

Some version of this table is on every blog and in every chat assistant's answer. Four of its five rows are right, so it is worth being exact about the one that is not.

| The row | Verdict |
|---|---|
| Shadowing creates a **new** variable; `mut` reuses the same one | **Correct**, and it is the only row that matters — everything else follows from it |
| Shadowing **allocates a new memory slot**; `mut` **modifies data in the same slot** | **The wrong one.** True of the *binding*, not of the *data*, and stated as though it were about the data. Whole-value assignment drops and replaces; a shadow by move reallocates nothing. Whether a shadow gets its own stack slot is unobservable unless you take a reference — at which point you are running the better test anyway |
| Shadowing **allows** a type change; `mut` **forbids** one | **Correct.** `let mut status = 1; status = "Active";` is `E0308` |
| Shadowing repeats `let`; `mut` needs the keyword once | **Correct** |
| After shadowing the binding **can stay immutable**; a `mut` binding stays mutable | **Correct**, and the "can" is doing real work — see below |

The last row hides the table's real flaw, which is the two-column layout itself. These are not opposites, and nothing stops you using both at once:

```rust
let weight = 60;                  // immutable i32
let mut weight = weight as f32;   // new place, new type, and now mutable
weight += 1.5;                    // weight = 61.5
```

That line is one shadow *and* one `mut`. A table with a column for each cannot say so.

## The quiz that circulates, option by option

The table has a companion in the wild: a short multiple-choice question that hands you a string and asks which line turns it into a number. A user's age arrives as `"30"`, the rest of the program wants a `u32`, and you pick one of three lines.

```rust
let age = "30";
println!("Age as string: {age}");

// A)  let mut age = age.parse::<u32>().unwrap();
// B)  let age     = age.parse::<u32>().unwrap();
// C)      age     = age.parse::<u32>().unwrap();

println!("Age as number: {age}");
```

The published answer is **B**, and B is right. Two of the three explanations that travel with it are right as well. The third is worth being careful about, because it is the same mistake the two-column table encourages.

| Option | What the compiler does with it | The explanation that comes with it |
|---|---|---|
| **A** `let mut age = …` | **Compiles** — `warning: variable does not need to be mutable` | *"Mixes `mut` with shadowing in a confusing way."* **The wrong one** — see below |
| **B** `let age = …` | **Compiles clean** | *"A new, immutable `age` of the right type."* Correct, and the reason to ship it |
| **C** `age = …` | `error[E0308]: mismatched types` — expected `&str`, found `u32` | *"`mut` changes a value, not a type."* Correct — and provable |

**C is worth proving rather than asserting**, because the claim is stronger than it looks. Put `mut` on the first line — `let mut age = "30";` — and C reports the *same* `E0308`. Mutability was never what stopped it. Assignment writes into a place, and that place is `&str`-shaped for as long as it exists; there is nowhere in it to put a `u32`. The error most people expect here is `E0384`, *cannot assign twice to immutable variable* — you only reach that one once the types already match, because the type error fires first and stops the compile.

**A is the row to fix.** It is not confusing and it is not a category error: it is one shadow and one `mut`, which is the `weight` line three paragraphs up and an ordinary thing to write. What is wrong with it *here* is narrower, and the compiler states it in one line — nothing below ever reassigns `age`, so the `mut` is unearned and `unused_mut` fires. Make the mutation real and the warning goes away:

```rust
let birthday = "30";
let mut birthday = birthday.parse::<u32>().unwrap();   // shadow AND mut
birthday += 1;                                         // birthday = 31, and no warning
```

At which point A's shape is the right answer to a slightly different question. Keep that distinction, because the two rejects are rejected by different machinery: **C is refused, A is merely unearned.** Filing them together under "wrong" is how *shadowing and `mut` are alternatives* gets learned, and taking that apart is what this page is for.

## The row the table is missing

Not one of the five rows mentions the difference you will actually trip over: **the two effects end at different times.** A shadow is a *declaration*, so it expires with the block that declared it and the outer name comes back. A write goes into a place declared somewhere else, so it outlives the block it happened in.

```rust
let quorum = 5;
{
    let quorum = quorum * 20;   // a declaration — it ends at the brace
}
// quorum is 5 again out here

let mut turnout = 5;
{
    turnout *= 20;              // a write into a place declared outside
}
// turnout is 100 out here
```

The `Where each one ends` group in the verified output above is those two blocks running. Neither one is surprising on its own; what makes the pair worth a row is that the source lines look equally local, and only one of them is.

This is the mechanical reason behind the worst shadowing bug in the set. A loop body is a block, so a shadowed accumulator is rebuilt from the outer value and thrown away at every brace — [the tally that never tallies](../when_to_shadow/README.md#1-the-accumulator-that-never-accumulates), which compiles, runs, and complains only that a `mut` it no longer uses could be dropped. **If the new value has to outlive the block that computes it, a shadow cannot do the job at all**, and no error will say so.

The escape hatch is that [a block is an *expression*](../a_block_is_an_expression/README.md), so the work can stay scoped while the result leaves:

```rust
let quorum = {
    let raw = quorum * 20;   // scratch names die at the brace
    raw + 1                  // the value is handed out
};                           // quorum is 101, and `raw` never existed outside
```

One qualifier, because it cuts the other way: a *same-scope* shadow does not end early either — it holds the name to the end of the enclosing block, and [the value it hid outlives it](../shadowing_does_not_drop/README.md), dropping afterwards rather than sooner. Scope is what ends a shadow; it is not a way to end a value.

## Mutability is a property of the binding, not of the value

The table's last row is worth pushing one step further, because the two-column framing encourages a belief that is flatly false: that `let mut` means *mutable data* and a bare `let` means *immutable data*. It does not. Values are never mutable or immutable; **bindings** are. Move the same value to a different binding and the answer changes:

```rust
let s = String::from("Hello");   // immutable binding
let mut s = s;                   // the SAME String, moved into a mutable one
s.push_str(", world");           // so the very same value is now mutable
let s = s;                       // and frozen again — still the same String
```

One `String`, three bindings, two different answers to "is it mutable?" — and no copy, no reallocation, nothing but the name changing hands. This is why "immutable variable" makes people uneasy on first meeting: it is not a promise about the data, it is a promise about *this* handle to it. The middle line is the freeze-after-building idiom running backwards, and it is a normal thing to write.

Which is also the sharpest way to state what a shadow is. `let mut x` says *"writes may happen to this place."* `let x = …` over an existing `x` says *"here is a different place; the name means this one now."* One is a claim about a place; the other replaces the place entirely.

## Does the choice cost anything?

A recurring claim says shadowing produces better code — the argument being that a linear chain of values is easier for the optimizer than a mutable slot it has to track through every branch. It is usually supported by two assembly listings from Compiler Explorer, in which the `mut` version has the larger stack frame.

The reasoning is plausible and the measurement is not, because those listings are from a **debug** build (the giveaway is the `seto al` / `jne` pair — an overflow check, which release builds do not emit). Compare the two functions the way you would ship them:

```rust
pub fn mutable_version() -> i32 {
    let mut x: i32 = 1;
    x = x + 4;
    let mut y: i32 = 0;
    y = y + 10;
    x + y
}

pub fn shadow_version() -> i32 {
    let x: i32 = 1;
    let x = x + 4;
    let mut y: i32 = 0;
    y = y + 10;
    x + y
}
```

```bash
rustc --crate-type=lib --emit asm -O shadow_vs_mut.rs
```

The optimizer does not merely produce equivalent code for the two. It produces **one function** and points the other name at it:

```text title="rustc 1.97.1, x86_64 — the whole of both functions"
_mutable_version:
	pushq	%rbp
	movq	%rsp, %rbp
	movl	$15, %eax        ; the entire body, folded to the constant 15
	popq	%rbp
	retq

_shadow_version = _mutable_version   ; ← same symbol. LLVM proved they are the same function.
```

That last line is the answer to the performance question and to this whole page at once: at the level where machine code exists, the distinction has been optimized away, because it was never a claim about machine code. **Choose between them on what you want the reader to know**, which is what [When to shadow](../when_to_shadow/README.md) is for. The codegen will not notice either way.

## If you are coming from another language

- **Python.** `x = 5` then `x = "five"` looks like shadowing and is rebinding — **one** name in the namespace dict, repointed. The old object simply loses a reference. You cannot hold both under one name, and there is no equivalent of `y = &x` pinning the first one alive, because Python has no borrows to check. The Rust question "did this write to a place, or make a new one?" does not arise there; every assignment is the second thing.
- **ABAP.** Neither feature transfers. `DATA(lv_x)` twice in one routine is a syntax error, so there is no shadowing; and a `DATA` name is a fixed-type storage location for the whole routine, so assignment is always the write that `mut` performs. What ABAP gives you for free is the guarantee this page is about — one name, one place, for the entire routine — and what it charges for it is `lv_input` / `lv_input_num`.
- **C.** Shadowing exists but only across a *nested* block, and it is exactly the address demonstration above: two `int sum` in two scopes at two addresses. What C cannot do is Rust's same-scope shadow, and what it has no analogue for is the borrow checker refusing the `mut` version — in C both spellings compile, and the pointer to the old `sum` is your problem.

## Practice

**Two places, or one?** Write the four-line reference test both ways — once with a shadow, once with `mut` — and compile both before running either. Predict which one fails and what the error will be called.

Then make it harder to dismiss: redo the shadow version with a `String` instead of an `i32`, so nobody can tell you `Copy` did the work. Print the borrowed name and the shadowed name on the same line.

Then settle the drop question. Give a type a `Drop` impl that prints, and build two blocks: one that assigns over a `mut` binding, one that shadows. Put a `println!` immediately after the assignment and after the shadow, and use the two outputs to say exactly when the first value died in each.

Finally, settle the performance question yourself rather than believing this page: put the two functions above in a file, run `rustc --crate-type=lib --emit asm -O` on it, and find out what your compiler does with them.

Last, grade the quiz. Compile all three of its options yourself, predicting each verdict before you run it — one compiles clean, one compiles with a warning, one does not compile at all. Then push on the two rejects, because they do not fail the same way: add `mut` to the first line and see whether that rescues C, then add a line that reassigns `age` and see whether that rescues A. Say which of the two you were stopped by a rule and which by a lint.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:a_name_is_not_a_place_kata -->
*[`a_name_is_not_a_place_kata.rs`](examples/a_name_is_not_a_place_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: two places, or one?
//!
//! Part 1 is the reference test, both ways — the shadow compiles, the `mut`
//! spelling does not, and the error is the point.
//! Part 2 repeats it on a `String`, so `Copy` cannot be blamed.
//! Part 3 times the drops, which is how you find out that whole-value
//! assignment destroys the old value rather than editing it.
//! Part 4 is the guessing-game shape where every Rust learner first meets
//! this, and the warning that sends them here.
//! Part 5 is the three-way multiple choice that circulates as a quiz, with
//! the distractor its answer key gets wrong.
//!
//!   rustc --edition 2024 a_name_is_not_a_place_kata.rs -o /tmp/anipk && /tmp/anipk

struct Tracked(&'static str);

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("      drop: {}", self.0);
    }
}

fn banner(title: &str) {
    println!("\n──── {title}");
}

/// Part 1 — the four-line test that settles it.
fn two_places() {
    banner("1. Two places, one name");

    let x = 5;
    let y = &x; //   borrows the first place
    let x = 6; //    declares a second place and moves the name onto it
    println!("      y = {y}, x = {x}   <- both alive, so there are two places");

    println!("      The `mut` spelling of the same four lines is rejected:");
    println!("        let mut x = 5;");
    println!("        let y = &x;");
    println!("        x = 6;   error[E0506]: cannot assign to `x`");
    println!("                              because it is borrowed");
    println!("      One place. `y` is reading it. The write cannot be allowed.");
}

/// Part 2 — the same shape on a type that is emphatically not `Copy`.
fn not_a_copy_trick() {
    banner("2. Not a `Copy` trick");

    let name = String::from("Ada");
    let seen = &name; //                 borrows the first String
    let name = name.to_uppercase(); //   allocates a second String
    println!("      seen = {seen}   <- the first String, still owned, still allocated");
    println!("      name = {name}   <- what the name means from here on");
}

/// Part 3 — when does the first value actually die?
fn when_does_it_die() {
    banner("3. Assignment drops; a shadow does not");

    println!("    mut:");
    {
        let mut slot = Tracked("first");
        println!("      holding {}", slot.0);
        slot = Tracked("second"); //   "first" dies on THIS line
        println!("      assigned — and the drop above has already run");
        println!("      holding {}", slot.0);
    }

    println!("    shadow:");
    {
        let slot = Tracked("first");
        println!("      holding {}", slot.0);
        let slot = Tracked("second"); //   nothing dies here
        println!("      shadowed — nothing has dropped");
        println!("      holding {}", slot.0);
    } //                                   both die here, newest first
}

/// Part 4 — the guessing game, with the keyboard simulated so the output is
/// an answer key rather than a transcript of whoever ran it.
fn the_guess_that_needs_no_mut() {
    banner("4. Why `let guess: u32` needs no `mut`");

    const SECRET: u32 = 42;
    const TYPED: [&str; 3] = ["  fifty\n", " 7 \n", "42\n"];

    for typed in TYPED {
        // `mut` here is real: `read_line` writes into this buffer.
        let mut guess = String::new();
        guess.push_str(typed); //   stands in for io::stdin().read_line(&mut guess)

        // A second variable, of a second type, in a second place. It is
        // written once at birth and only read afterwards — so `mut` on it
        // would earn `unused_mut`, which is the warning that starts the
        // confusion this page is about.
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("      {:?} -> not a number, next", typed.trim());
                continue;
            }
        };

        if guess == SECRET {
            println!("      {guess} -> correct");
        } else if guess < SECRET {
            println!("      {guess} -> too small");
        } else {
            println!("      {guess} -> too big");
        }
    }

    println!("      Two variables named `guess`, and neither is ever reassigned:");
    println!("      the String is mutated through `&mut`, the u32 is initialized once.");
}


/// Part 5 — three ways to write one line, and only one of them to ship.
///
/// The circulating quiz gives a string, asks for a number, and offers
/// `let mut age = …`, `let age = …` and a bare `age = …`. Its answer (the
/// middle one) is right; its reason for rejecting the first one is not.
fn three_ways_to_write_one_line() {
    banner("5. String in, number out — which line do you write?");

    let age = "30"; //   a &str, and the rest of the function wants a number
    println!("      A)  let mut age = age.parse::<u32>().unwrap();   compiles — with a warning");
    println!("      B)  let age     = age.parse::<u32>().unwrap();   compiles clean   <- ship this");
    println!("      C)      age     = age.parse::<u32>().unwrap();   error[E0308]");

    // B, for real. A second variable, of a second type, wearing the same name.
    let age = age.parse::<u32>().unwrap();
    println!("      B runs: age = {age}, and its type is now u32");

    println!("    C is not a style question — it does not compile:");
    println!("        error[E0308]: mismatched types");
    println!("          |     let age = \"30\";");
    println!("          |               ---- expected due to this value");
    println!("          |     age = age.parse::<u32>().unwrap();");
    println!("          |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `u32`");
    println!("      And `mut` does not rescue it. Write `let mut age = \"30\";` on the");
    println!("      first line and C reports the SAME E0308 — assignment writes into a");
    println!("      place, and that place is &str-shaped for as long as it exists.");
    println!("      (E0384, `cannot assign twice to immutable variable`, is the error");
    println!("      people expect here. You only reach it once the types match.)");

    println!("    A is the one the answer key gets wrong. It compiles:");
    println!("        warning: variable does not need to be mutable");
    println!("          |     let mut age = age.parse::<u32>().unwrap();");
    println!("          |         ----^^^");
    println!("          |         help: remove this `mut`");
    println!("      That is `unused_mut`, and it is a narrow, checkable complaint:");
    println!("      nothing below ever reassigns `age`. It is not a ruling against");
    println!("      shadowing and `mut` in one line — that combination is ordinary:");

    // The same shape as A, with the `mut` earned — so no warning fires.
    let birthday = "30";
    let mut birthday = birthday.parse::<u32>().unwrap(); //   shadow AND mut
    birthday += 1; //                                        this is what earns it
    println!("      let mut birthday = birthday.parse::<u32>().unwrap();");
    println!("      birthday += 1;   ->  {birthday}   <- one shadow, one mut, no warning");

    println!("      So: pick B because the compiler asked you to, not because the");
    println!("      two keywords cannot be spelled on one line.");
}

fn main() {
    two_places();
    not_a_copy_trick();
    when_does_it_die();
    the_guess_that_needs_no_mut();
    three_ways_to_write_one_line();
}
```
<!-- /source -->

<!-- output:a_name_is_not_a_place_kata -->
*Verified output of [`a_name_is_not_a_place_kata.rs`](examples/a_name_is_not_a_place_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── 1. Two places, one name
      y = 5, x = 6   <- both alive, so there are two places
      The `mut` spelling of the same four lines is rejected:
        let mut x = 5;
        let y = &x;
        x = 6;   error[E0506]: cannot assign to `x`
                              because it is borrowed
      One place. `y` is reading it. The write cannot be allowed.

──── 2. Not a `Copy` trick
      seen = Ada   <- the first String, still owned, still allocated
      name = ADA   <- what the name means from here on

──── 3. Assignment drops; a shadow does not
    mut:
      holding first
      drop: first
      assigned — and the drop above has already run
      holding second
      drop: second
    shadow:
      holding first
      shadowed — nothing has dropped
      holding second
      drop: second
      drop: first

──── 4. Why `let guess: u32` needs no `mut`
      "fifty" -> not a number, next
      7 -> too small
      42 -> correct
      Two variables named `guess`, and neither is ever reassigned:
      the String is mutated through `&mut`, the u32 is initialized once.

──── 5. String in, number out — which line do you write?
      A)  let mut age = age.parse::<u32>().unwrap();   compiles — with a warning
      B)  let age     = age.parse::<u32>().unwrap();   compiles clean   <- ship this
      C)      age     = age.parse::<u32>().unwrap();   error[E0308]
      B runs: age = 30, and its type is now u32
    C is not a style question — it does not compile:
        error[E0308]: mismatched types
          |     let age = "30";
          |               ---- expected due to this value
          |     age = age.parse::<u32>().unwrap();
          |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `u32`
      And `mut` does not rescue it. Write `let mut age = "30";` on the
      first line and C reports the SAME E0308 — assignment writes into a
      place, and that place is &str-shaped for as long as it exists.
      (E0384, `cannot assign twice to immutable variable`, is the error
      people expect here. You only reach it once the types match.)
    A is the one the answer key gets wrong. It compiles:
        warning: variable does not need to be mutable
          |     let mut age = age.parse::<u32>().unwrap();
          |         ----^^^
          |         help: remove this `mut`
      That is `unused_mut`, and it is a narrow, checkable complaint:
      nothing below ever reassigns `age`. It is not a ruling against
      shadowing and `mut` in one line — that combination is ordinary:
      let mut birthday = birthday.parse::<u32>().unwrap();
      birthday += 1;   ->  31   <- one shadow, one mut, no warning
      So: pick B because the compiler asked you to, not because the
      two keywords cannot be spelled on one line.
```
<!-- /output -->

</details>

## Traps

- **Printing addresses to prove two bindings differ.** It works, and it teaches the wrong reason — it makes a semantic distinction look like a fact about stack layout, which is the belief the wrong table row is built on. Take a reference instead.
- **Reading `s = String::from("new")` as an edit.** It drops the old `String`, frees its buffer, and moves a new one in. If you have a `Drop` impl with side effects, the assignment is where they fire.
- **Assuming a shadow is the cheap option, or the expensive one.** At `-O` there is nothing to choose between; on this machine the compiler emitted a single function for both.
- **Thinking you must pick one.** `let mut x = x as f32;` is both, and it is a normal line of Rust.

## See also

- [SHADOWING.md](../../SHADOWING.md) — the map of the whole set
- [When to shadow](../when_to_shadow/README.md) — the judgement call, once you know the mechanism
- [A shadow does not drop](../shadowing_does_not_drop/README.md) — how long the first place stays alive, and why you cannot free it early
- [Shadowing and `unwrap`](../shadowing_and_unwrap/README.md) — the `Copy` confusion this page's `String` example is guarding against
- [Borrowing](../borrowing/README.md) — why `y` stays valid across the shadow, and what `E0506` is protecting
- [Ownership and moves](../ownership_and_moves/README.md) — drop order, which is what the two `Drop` blocks above are showing
