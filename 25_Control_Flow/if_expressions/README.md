# `if` expressions

**Level:** 101 · for newcomers

**One line:** `if` in Rust is used exactly like `if` in every other language — and then it does one more thing, which is **have a value**, so `let size = if n < 10 { "small" } else { "large" };` needs no ternary operator and no assignment in each branch.

```rust
let n = 7;
let size = if n < 10 { "small" } else { "large" };
println!("{size}");   // small
```

No `?:`, no `let mut size;` above it, no `size = …` in each branch. The branch that runs hands back its last expression, and that becomes the value of the whole `if`.

## No parentheses, and the braces are not optional

```rust
let n = 7;
if n < 10 {
    println!("{n} is a single digit");   // 7 is a single digit
}
// if (n < 10) { … }           compiles, and warns: unnecessary parentheses
// if n < 10 println!("…");    error: expected `{`, found `println`
```

Parentheses around the condition are legal and linted: `unused_parens` is on by default, and its `help:` deletes them. The braces are grammar — the condition ends where the `{` starts — so there is no one-statement form, and no second line that looks indented under the `if` but runs regardless.

```text title="Abridged — real rustc output for if_without_braces.rs, summary lines dropped"
error: expected `{`, found `println`
 --> if_without_braces.rs:3:15
  |
3 |     if n < 10 println!("small");
  |               ^^^^^^^ expected `{`
  |
note: the `if` expression is missing a block after this condition
 --> if_without_braces.rs:3:8
  |
3 |     if n < 10 println!("small");
  |        ^^^^^^
help: you might have meant to write this as part of a block
  |
3 |     if n < 10 { println!("small") };
  |               +                   +
```

```text title="Abridged — real rustc output for if_with_parens.rs, summary line dropped"
warning: unnecessary parentheses around `if` condition
 --> if_with_parens.rs:3:8
  |
3 |     if (n < 10) {
  |        ^      ^
  |
  = note: `#[warn(unused_parens)]` (part of `#[warn(unused)]`) on by default
help: remove these parentheses
  |
3 -     if (n < 10) {
3 +     if n < 10 {
  |
```

A warning is a question rather than a refusal — [what a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) — and this one's answer is always *delete them*.

## The condition is a `bool`, and nothing else is

*Rust in Action* §2.4.6 says `if` accepts any expression that evaluates to a Boolean. That is the whole list: an integer, an empty `Vec`, a `None` are not Booleans, and nothing converts them.

```rust
let n = 1;
// if n { }         // error[E0308]: mismatched types — expected `bool`, found integer
if n != 0 { }       // say which comparison you meant
```

```text title="Abridged — real rustc output for if_on_an_integer.rs, summary lines dropped"
error[E0308]: mismatched types
 --> if_on_an_integer.rs:3:8
  |
3 |     if n {
  |        ^ expected `bool`, found integer
```

[Meet the `bool`](../../15_First_Programs/meet_the_bool/README.md) has the rest of that story — the same rule in `while`, guards and `filter`, and `n != 0` as the conversion rustc suggests. The book's other claims around this section are run on [the claims page](../flow_control_claims_checked/README.md).

C's `if (x = 5)` meets the same rule from the other side. Assignment is an expression in Rust too, but its type is `()`, so the typo is a type error rather than a condition that is always true:

```text title="Abridged — real rustc output for assign_in_condition.rs, summary lines dropped"
error[E0308]: mismatched types
 --> assign_in_condition.rs:3:8
  |
3 |     if x = 5 {
  |        ^^^^^ expected `bool`, found `()`
  |
help: you might have meant to compare for equality
  |
3 |     if x == 5 {
  |           +
```

One more thing may stand where the condition goes: a `let` pattern, as in `if let Some(p) = port && p >= 1024 { … }`. That is [`if let`](../../17_Option_and_Result/if_let/README.md), and the `&&` between a `let` and a `bool` is edition 2024 only.

## `if` has a value: the last expression of the branch that ran

```rust
let label = if item == 42 {
    "forty-two"
} else if item == 132 {
    "one hundred thirty-two"
} else {
    "neither"
};
```

That is the book's `else if` chain given something to produce. `else if` is not its own keyword: it is `else` followed by another `if` expression, so the whole ladder is one expression with one value.

Each branch is a [block](../../15_First_Programs/a_block_is_an_expression/README.md), so it can do work first and hand back only its tail:

```rust
let price = if cents >= 100 {
    let dollars = cents / 100;
    let rest = cents % 100;
    format!("${dollars}.{rest:02}")   // $19.99 for 1999
} else {
    format!("{cents} cents")          // 45 cents for 45
};
```

`dollars` and `rest` never escape the branch. Only the branch that is taken is evaluated — section 5 of the output below calls a function that prints, and the `else` side prints nothing — and an `if` goes anywhere a value goes: a function's tail with no `return`, an argument to `println!`, the right of a `let`.

## Both branches must have the same type

The whole `if` has one type, so both branches have to agree on what it is:

```rust
let size = if n < 10 { "small" } else { "large" };   // both &str
// let size = if n < 10 { "small" } else { 10 };     // E0308
```

```text title="Abridged — real rustc output for branches_disagree.rs, summary lines dropped"
error[E0308]: `if` and `else` have incompatible types
 --> branches_disagree.rs:3:45
  |
3 |     let size = if n < 10 { "small" } else { 10 };
  |                            -------          ^^ expected `&str`, found integer
  |                            |
  |                            expected because of this
```

The same rule works for you in the other direction: a type from outside reaches into **both** branches. `let b: u8 = if ready { 1 } else { 2 };` makes both literals `u8`, where without the annotation both are `i32` — section 6 of the output.

## The stray `;`, and which branch rustc blames

A semicolon after `"small"` turns that branch into a statement, so its value becomes `()` — [the unit type](../../15_First_Programs/the_unit_type/README.md) — and the branches disagree again. Where the error lands depends on whether anything **outside** the `if` expected a type.

Nothing did here:

```text title="Abridged — real rustc output for stray_semicolon.rs, summary lines dropped"
error[E0308]: `if` and `else` have incompatible types
 --> stray_semicolon.rs:3:46
  |
3 |     let size = if n < 10 { "small"; } else { "large" };
  |                            --------          ^^^^^^^ expected `()`, found `&str`
  |                            |      |
  |                            |      help: consider removing this semicolon
  |                            expected because of this
```

The `^^^^^^^` is under `"large"`, the branch you did **not** touch. With no annotation the first branch sets the expectation, so the second is reported as the wrong one, and the only mark on the line you changed is the `help:` under the `;`. Put the `;` in the `else` branch instead and the labels swap: `expected &str, found ()`, still under the second branch.

Annotate the `let` and the same mistake is reported on the other side, under a different title:

```text title="Abridged — real rustc output for stray_semicolon_annotated.rs, summary lines dropped"
error[E0308]: mismatched types
 --> stray_semicolon_annotated.rs:3:32
  |
3 |     let size: &str = if n < 10 { "small"; } else { "large" };
  |                                ^^^^^^^^^-^^
  |                                |        |
  |                                |        help: remove this semicolon to return this value
  |                                expected `&str`, found `()`
```

Now `&str` is expected from outside, and the branch with the `;` is the one that fails to supply it. A function's return type does the same job as the annotation. Either way the `help:` line is under the character to delete, so read it before the `^`. [The kata](#practice) asks you to predict both placements before you compile.

## No `else` means the value is `()`

An `if` without `else` still has a value — `()`, whichever way the condition goes, because the missing branch has nothing to hand back. Ask it for anything else and the error has a code of its own:

```rust
let c = true;
// let x = if c { 1 };        // error[E0317]: `if` may be missing an `else` clause
let x = if c { 1 } else { 0 };
println!("{x}");              // 1
```

```text title="Abridged — real rustc output for if_without_else.rs, summary lines dropped"
error[E0317]: `if` may be missing an `else` clause
 --> if_without_else.rs:3:13
  |
3 |     let x = if c { 1 };
  |             ^^^^^^^-^^
  |             |      |
  |             |      found here
  |             expected integer, found `()`
  |
  = note: `if` expressions without `else` evaluate to `()`
  = help: consider adding an `else` block that evaluates to the expected type
```

With no `let` to receive it, the same `if` is `E0308` instead — the lone branch has to be `()` as well, and rustc says so in its label:

```text title="Abridged — real rustc output for statement_if_with_value.rs, summary lines dropped"
error[E0308]: mismatched types
 --> statement_if_with_value.rs:3:12
  |
3 |     if c { 1 }
  |     -------^--
  |     |      |
  |     |      expected `()`, found integer
  |     `if` expressions without `else` arms expect their inner expression to be `()`
```

## The `;` after the whole `let`

`let size = if … { … } else { … };` ends in `;` because it is a `let` statement, and every `let` needs one; the `}` just before it closes the `else` block. Leave it out and the complaint arrives on the next line:

```text title="Abridged — real rustc output for let_without_semicolon.rs, summary lines dropped"
error: expected `;`, found `println`
 --> let_without_semicolon.rs:3:54
  |
3 |     let size = if n < 10 { "small" } else { "large" }
  |                                                      ^ help: add `;` here
4 |     println!("{size}");
  |     ------- unexpected token
```

Two semicolons, opposite jobs: the one inside a branch decides its value, the one after the `}` ends the statement. [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md#both-jobs-at-once) has the same pair on a bare block.

## A branch that never finishes fits any type

```rust
for (total, count) in [(10, 2), (7, 0), (9, 3)] {
    let mean = if count == 0 {
        continue;              // type `!`: it never produces a value
    } else {
        total / count
    };
    println!("{mean}");        // 5, then 3
}
```

`continue`, `break`, `return` and `panic!` never hand anything back, so their type is [the never type `!`](../../15_First_Programs/the_never_type/README.md), which fits a slot of any type — the two branches still agree. The skip and the computation sit in one `if`, and `mean` needs no `mut` and no default. [`continue`](../continue_expressions/README.md) and [`break`](../break_expressions/README.md) have their own pages.

## When to reach for `match` instead

- **Several outcomes decided by one value.** `match` lays the cases out as a table, and [checks the table has no gaps](../match_expressions/README.md). An `if` ladder ends in an `else` that silently takes whatever the rungs above it missed.
- **Conditions on different things** — `if cart.is_empty() { … } else if user.is_guest() { … }` — stay an `if`. `match` analyses one value; there is no single value here to analyse.
- **A plain `bool`**: either compiles. `match is_even(n) { true => "even", false => "odd" }` is exhaustive with two arms, and the book shows it beside the `if`; the `if` is the one most readers take in faster.

## Run it

<!-- output:if_expressions -->
*Verified output of [`if_expressions.rs`](examples/if_expressions.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── 1. The statement form: no parentheses, braces required
  7 is a single digit

──── 2. if has a value: the last expression of the branch that ran
  n =  3 -> size = "small"
  n = 10 -> size = "large"
  n = 99 -> size = "large"

──── 3. An else-if chain is still ONE expression
   42 -> "forty-two"   (same as describe: true)
  132 -> "one hundred thirty-two"   (same as describe: true)
    7 -> "neither"   (same as describe: true)

──── 4. A branch can do work before it hands back its value
  1999 -> $19.99
    45 -> 45 cents
  `dollars` and `rest` never escape the branch; only the tail does.

──── 5. Only the branch that is taken is evaluated
  evaluating the then branch
  x = 1; nothing was printed for the else branch

──── 6. One type for the whole if: the type both branches agree on
  let a     = if ready { 1 } else { 2 };         -> i32
  let b: u8 = if ready { 1 } else { 2 };         -> u8
  let s     = if ready { "yes" } else { "no" }; -> &str
  The annotation on b reaches into BOTH branches: neither literal is an i32.

──── 7. No else means the value is (), whichever way it goes
  quiet = true: the block ran
  quiet = true: unit = ()
  quiet = false: unit = ()

──── 8. A branch that never finishes fits any type
  10 / 2 = 5
  7 / 0: skipped
  9 / 3 = 3

──── 9. if goes anywhere a value goes: a tail, an argument
  -5 is negative and odd
   0 is zero and even
   6 is positive and even
```
<!-- /output -->

## If you are coming from another language

**Python.** The statement transfers with its punctuation swapped: `if n < 10:` and an indented body become `if n < 10 {` and braces, and `elif` is spelled `else if`. Python's conditional expression, `"small" if n < 10 else "large"`, is the real counterpart of Rust's `if` as a value — it too must have an `else`. Three things change. Truthiness is gone: `if items:` is idiomatic Python and a compile error here, where you write `if !items.is_empty()`. The two sides must be one type: Python accepts `1 if c else "large"` and hands you an `int` or a `str` depending on `c`; Rust stops at `E0308`. And a Rust branch is a whole block, so it can hold `let`s, a loop or a `continue` — work that Python's one-line form has no room for, and that ends up in a helper function or a walrus.

**C.** C requires the parentheses because it does not require the braces; Rust inverts both. A one-statement `if` body is how Apple's 2014 `goto fail;` bug shipped: a duplicated line sat indented under an `if` and ran unconditionally, and in Rust that line cannot be written without braces that show where the body ends. `if (n)` and `if (ptr)` rely on non-zero meaning true; here the condition is a `bool` or nothing. `c ? a : b` becomes `if c { a } else { b }`, and one of C's rules is dropped on the way — C converts the two operands to a common type (`c ? 1 : 2.5` is a `double`), where Rust converts nothing and requires them to match. And `if (x = 5)` is no longer a bug that compiles, because an assignment's type is `()`.

**ABAP.** `IF … ELSEIF … ELSE … ENDIF.` transfers exactly — top to bottom, the first true condition's block runs, at most one block runs. It is a statement, though, so producing a value from it means assigning a variable in every branch, which is the shape `let x = if …` removes. The true counterpart is the constructor expression `COND #( WHEN … THEN … ELSE … )`, which has a value, and two of its rules differ from Rust's. Leave out `ELSE` and `COND` yields the type's **initial value** when no `WHEN` holds; Rust refuses that `if` outright (`E0317`), so no blank or zero appears unasked. And every `THEN` result need only be *convertible* to the result type, where Rust requires the *same* type (`E0308`) and converts nothing. `ELSE THROW cx_…( )` is the diverging branch — Rust's `panic!` or `return` in an `else`. Conditions are the other place to watch: a predicative method call, `IF lo_cart->is_empty( ).`, is true whenever the result is **not initial**, whatever type the method returns, which is the one piece of truthiness in the language. In Rust the method has to return `bool`.

## Practice

**A receipt with no placeholders.** A cart is a list of `(item, cents, quantity)` rows. Print one line per row and a total, and write every decision as an `if` that *is* the value — never a `let mut label;` assigned in each branch:

- `price(cents: u32) -> String` formats `1250` as `$12.50` and `45` as `45c`, in one `if`
- a row with quantity 0 is skipped from inside the `if` that computes the line total
- a line of $20 or more gets a marker, and shipping ($5.99) is free from $50 up

Then three predictions, each written down before you compile. **(a)** Put a `;` after the first `format!` in `price`: which branch does the `^` land under, and what does rustc say it expected? **(b)** Put the same `;` after the marker string instead: same questions. **(c)** Change `qty == 0` to `qty`: what does the error call the type it found?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:if_expressions_kata -->
*[`if_expressions_kata.rs`](examples/if_expressions_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a receipt with no placeholder variables.
//!
//! Every decision on the receipt is an `if` that IS the value, so no line reads
//! `let mut label;` followed by an assignment in each branch.
//!
//!   rustc --edition 2024 if_expressions_kata.rs -o /tmp/ifk && /tmp/ifk

/// One `if`, two formats. Both branches are a `String`, so the `if` is one.
fn price(cents: u32) -> String {
    if cents >= 100 {
        format!("${}.{:02}", cents / 100, cents % 100)
    } else {
        format!("{cents}c")
    }
}

fn main() {
    let cart = [
        ("coffee", 1_250u32, 2u32),
        ("napkins", 99, 0),
        ("mug", 899, 1),
        ("stickers", 45, 3),
        ("grinder", 2_499, 1),
    ];

    println!("1. One line per row; a quantity of 0 leaves from inside the if");
    let mut subtotal = 0;
    for (item, cents, qty) in cart {
        let line = if qty == 0 {
            println!("   {item:<9} skipped: quantity 0");
            continue; // type `!`: fits where the else branch has a u32
        } else {
            cents * qty
        };
        let marker = if line >= 2_000 { "   <- $20 or more" } else { "" };
        subtotal += line;
        println!("   {item:<9} {qty} x {:>6} = {:>7}{marker}", price(cents), price(line));
    }

    println!();
    println!("2. Shipping is free from $50, and each total is one more if away");
    for (label, goods) in [("this cart", subtotal), ("mug + stickers", 899 + 135)] {
        let shipping = if goods >= 5_000 { 0 } else { 599 };
        println!(
            "   {label:<15} goods {:>7}   shipping {:>5}   total {:>7}",
            price(goods),
            if shipping == 0 { "free".to_string() } else { price(shipping) },
            price(goods + shipping)
        );
    }

    println!();
    println!("3. The three predictions");
    println!("   (a) `;` after the first format! in price. rustc says `mismatched");
    println!("       types`, expected `String`, found `()`, and the ^ is under the");
    println!("       branch you changed: the return type set the expectation.");
    println!("   (b) The same `;` after the marker string. Now it says `if` and");
    println!("       `else` have incompatible types, expected `()`, found `&str`,");
    println!("       and the ^ is under \"\", the branch you did NOT touch: nothing");
    println!("       outside the if expected a type, so the first branch set it.");
    println!("   (c) `if qty {{`: expected `bool`, found `u32`. No truthiness; and");
    println!("       `u32` rather than `integer`, because the literal said 2u32.");
}
```
<!-- /source -->

<!-- output:if_expressions_kata -->
*Verified output of [`if_expressions_kata.rs`](examples/if_expressions_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One line per row; a quantity of 0 leaves from inside the if
   coffee    2 x $12.50 =  $25.00   <- $20 or more
   napkins   skipped: quantity 0
   mug       1 x  $8.99 =   $8.99
   stickers  3 x    45c =   $1.35
   grinder   1 x $24.99 =  $24.99   <- $20 or more

2. Shipping is free from $50, and each total is one more if away
   this cart       goods  $60.33   shipping  free   total  $60.33
   mug + stickers  goods  $10.34   shipping $5.99   total  $16.33

3. The three predictions
   (a) `;` after the first format! in price. rustc says `mismatched
       types`, expected `String`, found `()`, and the ^ is under the
       branch you changed: the return type set the expectation.
   (b) The same `;` after the marker string. Now it says `if` and
       `else` have incompatible types, expected `()`, found `&str`,
       and the ^ is under "", the branch you did NOT touch: nothing
       outside the if expected a type, so the first branch set it.
   (c) `if qty {`: expected `bool`, found `u32`. No truthiness; and
       `u32` rather than `integer`, because the literal said 2u32.
```
<!-- /output -->

</details>

## See also

- [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) — why an `if` branch has a value at all
- [`match` expressions](../match_expressions/README.md) — the same idea for more than two cases, and the exhaustiveness the compiler checks
- [Flow control](../flow_control/README.md) — every construct in this chapter, what it evaluates to, and how you leave it
- [`if let`](../../17_Option_and_Result/if_let/README.md) — `if` against a *pattern*, once `Option` is in play
- [Meet the `bool`](../../15_First_Programs/meet_the_bool/README.md) — the only type a condition can have, and `then_some` for a value without an `if`
- [The never type `!`](../../15_First_Programs/the_never_type/README.md) — why a `continue` branch fits beside an `i32` one
- [`if` expressions ↗](https://doc.rust-lang.org/reference/expressions/if-expr.html) · [Comprehensive Rust: `if` ↗](https://google.github.io/comprehensive-rust/control-flow-basics/if.html)

## Sources

Tim McNamara, *Rust in Action* (Manning, 2021), §2.4.6 — the `else if` chain and the claim that `if` accepts any expression that evaluates to a Boolean. [The Rust Reference: `if` expressions ↗](https://doc.rust-lang.org/reference/expressions/if-expr.html) and [`E0317` ↗](https://doc.rust-lang.org/error_codes/E0317.html). Every rustc transcript above was produced by rustc 1.98.0 with `--edition 2024` from a scratch file of the name its header shows. Python's side: [conditional expressions ↗](https://docs.python.org/3/reference/expressions.html#conditional-expressions). ABAP's side, from the 7.58 keyword documentation: [`IF` ↗](https://help.sap.com/doc/abapdocu_758_index_htm/7.58/en-US/abapif.htm), [`COND` ↗](https://help.sap.com/doc/abapdocu_758_index_htm/7.58/en-US/abenconditional_expression_cond.htm) and [predicative method calls ↗](https://help.sap.com/doc/abapdocu_758_index_htm/7.58/en-US/abenpredicative_method_calls.htm).

## Po polsku

Warunek nie stoi tu w nawiasach, za to klamry są obowiązkowe nawet dla jednej instrukcji — co z definicji likwiduje klasyczny błąd z dopisaniem drugiej linii pod `if`-em bez klamer. Nawiasy wokół warunku są dozwolone, ale kompilator ostrzega (`unused_parens`) i sam podpowiada, żeby je usunąć. Warunek musi być typu `bool`: `if n` dla liczby całkowitej się nie skompiluje, bo w Ruscie nigdzie nie ma „prawdziwościowości” (*truthiness*) znanej z Pythona czy C — zero nie jest fałszem, a pusty łańcuch znaków niczym szczególnym. Z tego samego powodu literówka `if x = 5` nie przechodzi: przypisanie ma typ `()`, a nie `bool`.

Do tego `if` jest wyrażeniem i ma wartość, więc zamiast operatora warunkowego `?:` i zamiast przypisania w każdej gałęzi pisze się `let size = if n < 10 { "small" } else { "large" };` — pod jednym warunkiem: **obie gałęzie muszą mieć ten sam typ**. Stąd typowy `E0308` początkującego, którego zwykłą przyczyną jest jeden zabłąkany średnik, bo `"small";` zmienia wartość gałęzi z `&str` na `()`. Warto wiedzieć, gdzie kompilator postawi znaczek `^`: bez adnotacji typu oczekiwanie ustala pierwsza gałąź, więc błąd („`if` and `else` have incompatible types”) wskazuje gałąź, **której nie ruszaliśmy**; z adnotacją (`let size: &str = …`) albo z typem zwracanym funkcji błąd („mismatched types”) wskazuje gałąź ze średnikiem. W obu przypadkach linia `help:` stoi dokładnie pod średnikiem do usunięcia. Uwaga — średnik kończący całe `let x = if … { … };` to zupełnie inny średnik i on ma tam być.

`if` bez `else` też ma wartość: zawsze `()`. Dlatego `let x = if c { 1 };` to osobny błąd, `E0317` („`if` may be missing an `else` clause”), a nie cicha wartość domyślna. Tu leży najważniejsza różnica wobec ABAP-owego `COND #( WHEN … THEN … )`: bez `ELSE` ABAP zwraca wartość początkową typu (zero, spację), a Rust odmawia kompilacji; ABAP konwertuje też wyniki `THEN` do typu wyniku, a Rust wymaga identycznego typu. Gałąź, która nigdy się nie kończy — `continue`, `return`, `panic!` — ma typ `!` i pasuje wszędzie, więc `let mean = if count == 0 { continue; } else { total / count };` kompiluje się bez `mut` i bez wartości zastępczej, tak jak ABAP-owe `ELSE THROW`. Gdy przypadków jest więcej niż dwa i wszystkie dotyczą jednej wartości, lepszy jest `match`, bo kompilator sprawdzi, że drabinka nie ma dziur.

**Szukaj po polsku:** instrukcja warunkowa a wyrażenie · warunek musi być typu `bool` · operator warunkowy w Ruscie · `rust if expression both branches same type` · `rust E0308 if and else have incompatible types` · `rust E0317 if may be missing an else clause` · `abap COND a rust if`
