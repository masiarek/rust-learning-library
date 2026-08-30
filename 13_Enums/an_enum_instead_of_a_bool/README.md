# An enum instead of a bool

**Level:** 201 · working knowledge

**One line:** A `bool` parameter is two unnamed states, so the compiler cannot tell a correct call from a backwards one — two named variants cost the same byte and turn the backwards call into a build error.

```rust
enum Sides { Both, Single }
enum Output { BlackAndWhite, Color }

fn print_page(sides: Sides, output: Output) { /* ... */ }

print_page(Sides::Both, Output::BlackAndWhite);
```

The call says what it does without anyone opening the function, and the arguments cannot be swapped — two separate jobs, both now the compiler's.

The example is [David Drysdale's, from *Effective Rust* Item 1 ↗](https://www.lurklurk.org/effective-rust/use-types.html) (pp. 4–5).

---

## What two bools cost

The signature everybody writes first:

```rust
fn print_page(both_sides: bool, color: bool) { /* ... */ }

print_page(/* both_sides= */ true, /* color= */ false);  // both sides / black and white
print_page(false, true);                                 // single side / colour -- also fine
```

Both calls compile. Only one of them is the job you meant, and nothing in the program knows which. The `/* both_sides= */` comment is the giveaway: it is documentation standing exactly where a type should be, and it is checked by nobody — delete it, or move it, or leave it behind when the parameters are reordered, and the compiler is equally content.

| | two bools | two enums |
|---|---|---|
| reading the call site | `true, false` — go and look at the signature | `Sides::Both, Output::BlackAndWhite` |
| arguments swapped | compiles, prints the wrong job | `E0308`, with a fix-it |
| a third alternative arrives | a second bool, or a rewrite | a third variant, and every `match` reports in |
| what it costs | 1 byte | 1 byte |

## The refusal

The whole file, with the backwards call left as a comment so the page stays paste-safe:

```rust
pub enum Sides {
    Both,
    Single,
}

pub enum Output {
    BlackAndWhite,
    Color,
}

pub fn print_page(sides: Sides, output: Output) {
    let _ = (sides, output);
}

fn main() {
    // print_page(Output::BlackAndWhite, Sides::Single);   // <- E0308
}
```

```text title="rustc 1.98.0 on print_page.rs — the file above with line 16 uncommented"
error[E0308]: arguments to this function are incorrect
  --> print_page.rs:16:5
   |
16 |     print_page(Output::BlackAndWhite, Sides::Single);
   |     ^^^^^^^^^^ ---------------------  ------------- expected `Output`, found `Sides`
   |                |
   |                expected `Sides`, found `Output`
   |
note: function defined here
  --> print_page.rs:11:8
   |
11 | pub fn print_page(sides: Sides, output: Output) {
   |        ^^^^^^^^^^
help: swap these arguments
   |
16 -     print_page(Output::BlackAndWhite, Sides::Single);
16 +     print_page(Sides::Single, Output::BlackAndWhite);
   |
```

`help: swap these arguments` is the compiler naming the mistake and writing the fix. Against `bool, bool` it has nothing to say, because there is no mistake to see: two `bool`s in either order are two `bool`s.

## Newtype or enum?

A [newtype](../../16_Structs/newtype_score/README.md) — a one-field struct wrapping the `bool` — buys the same refusal, verified the same way:

```rust
struct DoubleSided(pub bool);
struct ColorOutput(pub bool);

fn print_page(sides: DoubleSided, color: ColorOutput) { /* ... */ }

print_page(DoubleSided(true), ColorOutput(false));
// print_page(ColorOutput(false), DoubleSided(true));   // <- E0308, "swap these arguments" again
```

What it does not buy is a name for either state. `DoubleSided(true)` still asks the reader to know which way round `true` runs, and `!flag` still flips it silently. The enum has no `true` in it to get backwards.

| | `struct DoubleSided(bool)` | `enum Sides { Both, Single }` |
|---|---|---|
| rejects the swapped call | yes | yes |
| names the two states | no — still `true` / `false` | yes |
| can grow a third state | no, `bool` has two | yes, and every `match` becomes a build error |
| `#[repr(transparent)]` for FFI | yes | not the same thing |

Drysdale's rule, and it is a good one: **newtype if the semantics will always be Boolean; enum if a third alternative could arise** — `Sides::BothAlternateOrientation` is his example, and it is exactly the change that is free in one column and a rewrite in the other. Adding it makes every `match` on `Sides` fail to compile until it is handled, which is [the whole payoff of the enum](../what_an_enum_is/README.md).

## The same mistake inside a struct

A comment explaining when a *field* is valid is the same giveaway as `/* both_sides= */`, one level down:

```rust
struct DisplayProps {
    x: u32,
    y: u32,
    monochrome: bool,
    // `fg_color` must be (0, 0, 0) if `monochrome` is true.
    fg_color: RgbColor,
}
```

The comment is describing states the type permits and the program must not produce. Put the bool and the field it governs into one enum and there is nothing left to describe:

```rust
enum Color {
    Monochrome,
    Foreground(RgbColor),
}

struct DisplayProps {
    x: u32,
    y: u32,
    color: Color,
}
```

The example program counts both, with a three-value stand-in for `RgbColor` so the states can be enumerated rather than asserted:

| shape | states | that break the comment |
|---|---|---|
| `monochrome: bool` + `fg: Shade` | 3 × 2 = **6** | 2 |
| `Color::Monochrome` \| `Color::Foreground(Shade)` | 1 + 3 = **4** | none can be written |

With a real 24-bit `RgbColor` the same arithmetic gives 33,554,432 states of which 16,777,215 contradict the comment, against 16,777,217 that cannot. This is *make invalid states inexpressible* (Item 1, p. 7), and it is [the product-versus-sum choice](../variants_that_carry_data/README.md) with a `bool` as the product's second factor.

It is also smaller. Measured on this target: the struct holding a `bool` and a `Shade` is **2 bytes**; the one holding the `Color` enum is **1**. A three-variant `Shade` uses 3 of the 256 values in its byte, so the extra `Monochrome` tag moves into one of the 253 that are spare — the same niche that makes [`Option<Box<T>>` free](../../17_Option_and_Result/nullable_pointers/README.md).

## The lints that sound like they cover this

Two clippy lints name this exact refactor. Neither will mention the code above.

```text title="Abridged — clippy 1.98.0 with the lint switched on by hand"
warning: more than 3 bools in function parameters
  = help: consider refactoring bools into two-variant enums
```

Read from the toolchain rather than from memory (`clippy-driver -Whelp`):

| lint | group | default | fires at |
|---|---|---|---|
| [`fn_params_excessive_bools` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#fn_params_excessive_bools) | `pedantic` | **allow** | more than 3 bools in a signature |
| [`struct_excessive_bools` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#struct_excessive_bools) | `pedantic` | **allow** | more than 3 bools in a struct |

So two things have to be true before a lint says anything: you turned on `pedantic` (as [the strict-clippy config](../../05_Tooling/strict_lints/README.md) does), and you already have four. `print_page(bool, bool)` is under the threshold, and `DisplayProps` has one `bool` in it. The help text is right about the fix and will never be shown to the person who needs it most, which is the argument for treating this as a design habit rather than something tooling catches — the same shape as [a typo becoming a binding](../a_typo_becomes_a_binding/README.md), where the two obvious lints are also silent.

## If you are coming from another language

**Python.** The `bool` parameter is the same trap, and Python fixes half of it with a keyword-only marker: `def print_page(*, both_sides: bool, color: bool)` makes `print_page(True, False)` a `TypeError` and forces every caller to write the names. That is the readability half and the swap half, at the call site, at runtime. It does not name the states — `both_sides=True` still leaves `True` meaning something the reader has to look up — and it is not enforced anywhere at all if the parameters are positional. `enum.Enum` names the states, `Literal["both", "single"]` names them for a type checker, and neither is checked by the interpreter. What Rust adds is that all three checks are the same check, and it runs before the program does.

**ABAP.** Named arguments are compulsory the moment a method takes two parameters — `print_page( both_sides = abap_true color = abap_false )` — so ABAP fixes the swap by syntax rather than by types, and fixes it better than an enum does at the call site. The gap is what `abap_bool` *is*: `c LENGTH 1`, so `'Y'` and `'0'` and `'?'` all fit, and every one of them behaves as false against the standard `IF flag = abap_true` test. A wrong value passes silently where a wrong *order* cannot. Since 7.51 an enumerated type closes that half — `TYPES: BEGIN OF ENUM t_sides, both, single, END OF ENUM t_sides.` — and the variable then holds only declared values. What still does not follow is the exhaustiveness: a `CASE` over the enumerated type compiles with a `WHEN` missing, so the third alternative arriving is a grep, not a build error.

**C.** A C enum is an `int` in a hat, and the swap survives it. `print_page(BW, SINGLE)` with `enum Sides` and `enum Output` parameters **compiles** — Apple clang 21 with `-Wall -Wextra` emits two `-Wimplicit-enum-enum-cast` warnings and produces the object file. C++'s `enum class` is the version that behaves like Rust's: the same call is a hard error, *no known conversion from `Output` to `Sides`*.

**Java, C#, Go.** All three have real enum types and all three let you pass `true` positionally, so the trap and the fix are both available; the difference is only that Rust's enum can carry the payload the bool was gating, which is what collapses `DisplayProps` from six states to four.

## The verified output

[`examples/an_enum_instead_of_a_bool.rs`](examples/an_enum_instead_of_a_bool.rs) compiled and run:

<!-- output:an_enum_instead_of_a_bool -->
*Verified output of [`an_enum_instead_of_a_bool.rs`](examples/an_enum_instead_of_a_bool.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
two bools, called as intended:
  print_page(true, false)            -> both sides / black and white
two bools, arguments swapped -- compiles, prints the wrong job:
  print_page(false, true)            -> single side / colour

two enums, called as intended:
  print_page(Sides::Both, Output::BlackAndWhite)
                                     -> both sides / black and white
two enums, arguments swapped -- does not compile:
  print_page(Output::BlackAndWhite, Sides::Single)
                                     -> error[E0308], with a fix-it

states the type permits, with 3 shades:
  bool + shade   (AND)  6 states, 2 of them break the comment
  Color          (OR )  4 states, every one meaningful
    monochrome: true  fg: Black  ok
    monochrome: true  fg: Red    contradicts the comment
    monochrome: true  fg: White  contradicts the comment
    monochrome: false fg: Black  ok
    monochrome: false fg: Red    ok
    monochrome: false fg: White  ok

and with a real RgbColor (16777216 colours):
  bool + RgbColor  33554432 states, 16777215 of them break the comment
  Color            16777217 states

sizes:
  bool        1
  Sides       1
  Output      1
  PropsBool   2
  PropsEnum   1
```
<!-- /output -->

---

## See also

- [What an enum is](../what_an_enum_is/README.md) — the declaration, and the `E0004` that arrives with the third variant
- [Variants that carry data](../variants_that_carry_data/README.md) — product versus sum, and the niche that made the enum a byte smaller
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the newtype, in full, where the wrapped value has more than two states
- [An enum as a state machine](../an_enum_as_a_state_machine/README.md) — the same argument with a `match (state, event)` table behind it
- [A typo becomes a binding](../a_typo_becomes_a_binding/README.md) — the other enum trap the obvious lints do not catch
- [Strict clippy](../../05_Tooling/strict_lints/README.md) — what turning on `pedantic` actually costs
