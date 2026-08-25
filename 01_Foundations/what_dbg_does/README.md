# What `dbg!` does

**Level:** 101 → 201 · working knowledge

**One line:** `dbg!` is not a shorter `println!("{:?}")` — it hands your value back so you can wrap any expression in place, it captures the *source text* of what you asked about, and it writes to stderr, which is why half of what surprises people about it never appears in the output they are reading.

---

## It gives the value back

This is the property everything else follows from:

```rust
let doubled = dbg!(2 + 3) * 10;   // doubled == 50
```

`dbg!` evaluates its argument, prints it, and returns it. So you can drop it into the middle of an expression you already have — no temporary variable, no restructuring, and you delete it by deleting six characters. `println!` returns `()`, so the same edit would not compile.

## It captures the expression, not just the value

```text
[what_dbg_does.rs:19:19] 2 + 3 = 5
```

Three things: **file:line:col**, the **source text** of the expression, and the value. `2 + 3` is not a string you passed — the macro captured it. That is the real argument against hand-rolling `println!("x = {:?}", x)`: the label there is a string literal, so it silently goes stale the moment you rename `x` or paste the line somewhere else. A `dbg!` label cannot lie about what it printed.

`dbg!()` with no argument prints just the location, which is a decent "did we get here" probe.

## It writes to stderr

Not stdout. Three consequences, in increasing order of how long they take to work out:

- `cargo run > out.txt` keeps your program's real output clean and leaves the debugging on the terminal. That is the design.
- `2>/dev/null` makes it vanish, and a pipe **reorders** it — stdout is block-buffered when piped, stderr never is, so `dbg!` lines can appear ahead of `println!` lines that ran first.
- **In this repo it means `dbg!` output is unrecordable.** `tools/run_examples.py` captures stdout only, so a lesson that demonstrates `dbg!` has to *describe* it with `println!` to have an answer key at all. The example on this page does exactly that, and says so where it happens.

## It always formats with `{:#?}`

`dbg!` is hard-wired to the alternate (pretty) form. For a derived `Debug` that is a gift. For a **hand-written** one it is a trap:

```
Flat with {:?}   -> Boxy fake1 fake2
Flat with {:#?}  -> Boxy fake1 fake2
```

Identical — because that impl is a plain `write!` chain that never asks `f.alternate()`. The flag arrived and was ignored, so `dbg!` silently gets the flat version, and a trailing `writeln!` in the impl adds a blank line on top of the newline `dbg!` already prints. `f.debug_struct(…)` handles the flag for you, which is the first reason to reach for it; `f.alternate()` is the question to ask if you write the branch yourself.

## It moves a non-`Copy` argument

```rust
dbg!(ballot);           // moved in — and you did not catch the return
println!("{}", ballot); // error[E0382]: borrow of moved value
```

It hands the value back, so `let b = dbg!(b);` is fine. But a bare `dbg!(b);` on its own line drops the value at the end of the statement. **`dbg!(&b)` is the habit** — nothing moves, and the printed label reads `&b`, which is honest about what happened. This is why `dbg!` in real code is nearly always written with an `&`.

## It survives `--release`

Unlike `debug_assert!`, `dbg!` has no `cfg` gate. Compile with `-O` and it still prints. It is a thing you *delete*, not a logging macro you leave in — nothing but code review will catch one you forgot.

## The confusion worth naming: field vs whole value

It is easy to conclude from experiment that `dbg!` is lenient about some structs and strict about others. It is not. These four fail identically, with the same `E0277`:

```
dbg!(named_struct)        println!("{:?}", named_struct)
dbg!(unit_struct)         println!("{:?}", unit_struct)
```

`dbg!(b.score)` works with no derive because `u8` implements `Debug` — you named a **field**, so the field's type is what needs it. A unit struct only *looks* stricter: it has no field to name, so the lenient move is unavailable. There is no unit-struct rule and no `dbg!`-specific rule. **Whatever you name must implement `Debug`.**

## If you are coming from another language

**Python.** The closest thing is `print(f"{x=}")`, which also captures the expression text — and `icecream`'s `ic()` is a near-exact match, returning its argument the same way. What Python has no equivalent of is the *move*: `ic(obj)` never costs you the object.

**ABAP.** No equivalent. `WRITE` goes to the list, `BREAK-POINT` stops the program; there is nothing that prints a value inline and gives it back so the surrounding expression still works.

---

## Practice

**Find both traps, then fix them.** Take a struct with a hand-written `Debug` that is a plain `write!` chain ending in `writeln!`.

1. Print it with `{:?}` and with `{:#?}`. Why are they identical, and what did `dbg!` therefore get?
2. Rebuild the impl with `f.debug_struct(…)`. What changed, and what did you no longer have to write?
3. Write the same thing by hand a third time, honouring the flag yourself. Which function tells you it is set?
4. Now `dbg!(data);` on its own line, and use `data` afterwards. Read the error. Fix it two ways — one that rebinds and one that borrows — and say which you would put in real code, and why.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_dbg_does_kata -->
*[`what_dbg_does_kata.rs`](examples/what_dbg_does_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the two `dbg!` traps — the alternate flag, and the move.
//!
//!   rustc --edition 2024 what_dbg_does_kata.rs -o /tmp/wddk && /tmp/wddk

use std::fmt;

#[derive(Clone, Debug)] // Debug here is for TRAP 2; the wrappers below write their own
struct Data {
    name: String,
    bones: Vec<String>,
}

// (a) The hand-written Debug that ignores the alternate flag. This is the
//     common shape, and it is wrong in a way nothing warns about.
struct Flat(Data);
impl fmt::Debug for Flat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.name)?;
        for bone in &self.0.bones {
            write!(f, " {bone}")?;
        }
        writeln!(f) // and a trailing newline nobody asked for
    }
}

// (b) The same thing built with the Formatter's own helper, which asks.
struct Honest(Data);
impl fmt::Debug for Honest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Data")
            .field("name", &self.0.name)
            .field("bones", &self.0.bones)
            .finish()
    }
}

// (c) Hand-written, but honouring alternate() explicitly — what debug_struct
//     is doing for you underneath.
struct Manual(Data);
impl fmt::Debug for Manual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}\n  bones: {:?}", self.0.name, self.0.bones)
        } else {
            write!(f, "{} {:?}", self.0.name, self.0.bones)
        }
    }
}

fn main() {
    let d = Data { name: "Boxy".into(), bones: vec!["fake1".into(), "fake2".into()] };

    println!("TRAP 1 — a hand-written Debug that never asks f.alternate()\n");
    println!("  Flat with {{:?}}   -> {:?}", Flat(d.clone()));
    println!("  Flat with {{:#?}}  -> {:#?}", Flat(d.clone()));
    println!("  Identical. The alternate flag arrived and the impl ignored it.");
    println!("  dbg! is hard-wired to {{:#?}}, so dbg! gets the flat one too — and the");
    println!("  stray writeln! adds a blank line on top of the newline dbg! prints.\n");

    println!("  Honest with {{:?}}  -> {:?}", Honest(d.clone()));
    println!("  Honest with {{:#?}} ->");
    println!("{:#?}", Honest(d.clone()));
    println!("  f.debug_struct() checks the flag for you. Reach for it first.\n");

    println!("  Manual with {{:?}}  -> {:?}", Manual(d.clone()));
    println!("  Manual with {{:#?}} -> {:#?}", Manual(d.clone()));
    println!("  ...and if you must write it by hand, f.alternate() is the question.\n");

    println!("TRAP 2 — dbg! moves a non-Copy argument\n");
    println!("  dbg!(d);        // moves d in. It hands the value back...");
    println!("  println!(\"{{}}\", d.name);   // ...but you did not catch it: E0382");
    println!();
    println!("  Two fixes, and they mean different things:");
    let d = dbg!(d); //        catch the value back
    println!("    let d = dbg!(d);   rebind — d is alive because you took the return");
    dbg!(&d); //                borrow instead
    println!("    dbg!(&d);          borrow — nothing moved at all, and the printed");
    println!("                       label shows `&d` rather than `d`");
    println!("  Reach for the borrow. It reads as an inspection, which is what it is.");
    println!("  still own it: {} with {} bones", d.name, d.bones.len());

    println!("\n(Both dbg! calls above wrote to stderr, so they are absent from this");
    println!("page's recorded output — see section 3 of the lesson.)");
}
```
<!-- /source -->

<!-- output:what_dbg_does_kata -->
*Verified output of [`what_dbg_does_kata.rs`](examples/what_dbg_does_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
TRAP 1 — a hand-written Debug that never asks f.alternate()

  Flat with {:?}   -> Boxy fake1 fake2

  Flat with {:#?}  -> Boxy fake1 fake2

  Identical. The alternate flag arrived and the impl ignored it.
  dbg! is hard-wired to {:#?}, so dbg! gets the flat one too — and the
  stray writeln! adds a blank line on top of the newline dbg! prints.

  Honest with {:?}  -> Data { name: "Boxy", bones: ["fake1", "fake2"] }
  Honest with {:#?} ->
Data {
    name: "Boxy",
    bones: [
        "fake1",
        "fake2",
    ],
}
  f.debug_struct() checks the flag for you. Reach for it first.

  Manual with {:?}  -> Boxy ["fake1", "fake2"]
  Manual with {:#?} -> Boxy
  bones: ["fake1", "fake2"]
  ...and if you must write it by hand, f.alternate() is the question.

TRAP 2 — dbg! moves a non-Copy argument

  dbg!(d);        // moves d in. It hands the value back...
  println!("{}", d.name);   // ...but you did not catch it: E0382

  Two fixes, and they mean different things:
    let d = dbg!(d);   rebind — d is alive because you took the return
    dbg!(&d);          borrow — nothing moved at all, and the printed
                       label shows `&d` rather than `d`
  Reach for the borrow. It reads as an inspection, which is what it is.
  still own it: Boxy with 2 bones

(Both dbg! calls above wrote to stderr, so they are absent from this
page's recorded output — see section 3 of the lesson.)
```
<!-- /output -->

</details>

---

## The verified output

Every `dbg!` line this program prints goes to **stderr**, and the recorded key below is stdout only — which is section 3 demonstrating itself. Run it to see the other half.

<!-- output:what_dbg_does -->
*Verified output of [`what_dbg_does.rs`](examples/what_dbg_does.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. `dbg!` returns its argument, and that is the whole point
   let doubled = dbg!(2 + 3) * 10;   ->  50
   It evaluated to 5 and handed it straight back, so you can wrap any
   sub-expression without restructuring the code around it.
   println! returns (), so the same move would not compile.

2. It prints three things, not one
   file:line:col, the EXPRESSION SOURCE TEXT, and the value:
       [what_dbg_does.rs:19:19] 2 + 3 = 5
   `2 + 3` is not a string you passed — the macro captured the source.
   That is why `dbg!(x)` beats `println!("x = {:?}", x)`: the label
   cannot go stale when you rename x.

3. It writes to STDERR
   A dbg! line just fired above and you may not see it here — `2>/dev/null`
   hides it, and a pipe reorders it, because stdout is block-buffered when
   piped while stderr never is.
   Practical consequence: `cargo run > out.txt` keeps your program's real
   output clean and leaves the debugging on the terminal. In THIS repo it
   means run_examples.py cannot record dbg! output at all — it captures
   stdout only, so a lesson must print with println! to have an answer key.

4. It formats with `{:#?}`, always
   dbg! is hard-wired to the alternate (pretty) form, one field per line.
   For a derived Debug that is a gift. For a HAND-WRITTEN one it is a trap:
   if your impl never asks f.alternate(), `{:?}` and `{:#?}` print the
   same thing, and dbg! silently gets the flat version.

5. It MOVES a non-Copy argument
   dbg!(owned) took ownership. It gives the value back, so `let x = dbg!(x)`
   is fine — but a bare `dbg!(owned);` on its own line drops it, and the
   next use is E0382. `dbg!(&owned)` is the habit: borrow, print, move on.
   still here: Ben scored 3

6. It is NOT removed in release builds
   Unlike debug_assert!, dbg! has no cfg gate. Compile with -O and it still
   prints. It is a thing you delete, not a logging macro you leave in.
   (`cargo build --release` will not save you; a code review has to.)

7. The confusion worth naming: field vs whole value
   dbg!(b.score) works with no derive, because u8 implements Debug.
   dbg!(b) needs Ballot to implement it. These four are the SAME error:
       dbg!(named_struct)        println!("{:?}", named_struct)
       dbg!(unit_struct)         println!("{:?}", unit_struct)
   A unit struct only LOOKS stricter — it has no field to name, so the
   lenient move is not available. There is no unit-struct rule, and no
   dbg!-specific rule: whatever you NAME must implement Debug.
   (that dbg! fired on stderr too — a u8, no derive needed)
```
<!-- /output -->

## See also

- [Debug and Display](../debug_vs_display/README.md) — which trait `dbg!` needs, why `{}` refuses your struct, and why the language generates one and not the other
- [When a struct refuses](../when_a_struct_refuses/README.md) — the `E0277` above, alongside the seven other struct refusals
- [What a warning is asking](../what_a_warning_is_asking/README.md) — including why `let _ = value;` drops immediately and `let _x = value;` does not
- [`dbg!` ↗](https://doc.rust-lang.org/std/macro.dbg.html) · [`Formatter::alternate` ↗](https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.alternate) · [`Formatter::debug_struct` ↗](https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.debug_struct)
