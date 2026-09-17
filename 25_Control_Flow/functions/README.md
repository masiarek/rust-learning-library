# Functions

**Level:** 101 · for newcomers

**One line:** `fn name(param: Type) -> Type { }` — every parameter and the return type are written out, always, and the body's **last expression without a semicolon** is what comes back, so most Rust functions contain no `return` at all.

```rust
fn percent(n: i32) -> i32 {
    if n < 0 {
        return 0; // early exit
    }
    n.min(100) // the tail expression is the return value
}

fn main() {
    println!("{} {}", percent(-5), percent(250)); // 0 100
}
```

## The signature is never inferred

Inside a body, [type inference](../../15_First_Programs/type_inference/README.md) works out nearly every type, sometimes from a line further down. At the signature it stops. Ask for the return type to be inferred and rustc refuses, although its help line shows it already knows the answer:

```text title="Abridged — real rustc output for placeholder.rs"
error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
 --> placeholder.rs:1:27
  |
1 | fn add(a: i32, b: i32) -> _ {
  |                           ^ not allowed in type signatures
  |
help: replace with the correct return type
  |
1 - fn add(a: i32, b: i32) -> _ {
1 + fn add(a: i32, b: i32) -> i32 {
  |
```

A signature is what every caller compiles against. If it were inferred, an edit inside the body could change the type every caller sees, and the error would turn up in their code instead of yours.

A parameter with no type stops the parser:

```text title="Abridged — real rustc output for untyped.rs, the second error (for b) dropped"
error: expected one of `:`, `@`, or `|`, found `,`
 --> untyped.rs:1:9
  |
1 | fn add(a, b) -> i32 {
  |         ^ expected one of `:`, `@`, or `|`
  |
help: if this is a parameter name, give it a type
  |
1 | fn add(a: TypeName, b) -> i32 {
  |         ++++++++++
```

`@` and `|` are pattern syntax, and they are in that list because **a parameter is a [pattern](../../30_Pattern_Matching/irrefutable_patterns/README.md)**, the same as the left side of a `let`. `fn swap((a, b): (i32, i32))` takes a tuple apart in the signature, and `fn first_of(n: i32, _: bool)` accepts an argument it never binds. Both run in the output below.

## No `->` means `()`, and so does a final `;`

A function that names no return type returns [the unit type `()`](../../15_First_Programs/the_unit_type/README.md). `log_line` below has no `->`, and the value it hands back prints as `()`.

The two ways of getting this wrong give two different messages. A value where `()` was declared:

```text title="Abridged — real rustc output for no_arrow.rs"
error[E0308]: mismatched types
 --> no_arrow.rs:2:5
  |
1 | fn add(a: i32, b: i32) {
  |                       - help: try adding a return type: `-> i32`
2 |     a + b
  |     ^^^^^ expected `()`, found `i32`
```

And a semicolon after the value that was meant to be returned:

```text title="Abridged — real rustc output for semicolon.rs"
error[E0308]: mismatched types
 --> semicolon.rs:1:23
  |
1 | fn percent(n: i32) -> i32 {
  |    -------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     n.min(100);
  |               - help: remove this semicolon to return this value
```

The arrow points at `-> i32`, which is correct. The mistake is the `-` under the `;` a line lower. [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) explains why that one character does it.

rustc's own wording gives you the vocabulary: the tail expression is the **implicit** return, and `return` is the explicit one.

## `return` is for leaving early

Both are correct, and idiomatic Rust uses each for one job. `percent` leaves early with `return 0;` for the case that needs no more work, and the value at the bottom is the normal way out. A `return x;` on the last line compiles, and clippy 1.98.0 warns *unneeded `return` statement* (`needless_return`, on by default), since the last line already returns its value.

## Order does not matter

`main` sits at the top of the example and calls every function defined below it. Items in a module can be used before they are defined, so Rust has no prototypes and no forward declarations. C needs a declaration before the call: C99 removed implicit declarations, and Apple clang 21 rejects a call to a function defined further down with *call to undeclared function*.

## Arguments are evaluated first, left to right

`add(traced("first", 2), traced("second", 3))` prints `evaluating first`, then `evaluating second`, then the sum — see the third block of [the verified output](#the-verified-output). Every argument is evaluated before the function runs, left to right. That order is a guarantee, not an accident of this compiler. [The Reference ↗](https://doc.rust-lang.org/reference/expressions.html#evaluation-order-of-operands) lists call expressions among those whose operands are *evaluated left to right as written in the source code*.

This is why a function cannot short-circuit. `fn and(a: bool, b: bool) -> bool { a && b }` returns the same value as `&&`, but by the time it runs, both arguments have already been computed. The output shows `and(left, right)` evaluating both, and `left && right` stopping at a `false` left. When the right side is an index that may be out of bounds, or a network fetch, that is the whole difference. It is the same trap as [`&` beside `&&`](../../15_First_Programs/meet_the_bool/README.md#the-trap-compiles-on-bools-and-does-not-short-circuit), and [a book's `and`/`or` functions](../../15_First_Programs/and_or_claims_checked/README.md) are where it usually shows up first.

## No overloading, no default arguments, no named arguments

A name means one function per module:

```text title="Abridged — real rustc output for overload.rs"
error[E0428]: the name `area` is defined multiple times
 --> overload.rs:5:1
  |
1 | fn area(side: u32) -> u32 {
  | ------------------------- previous definition of the value `area` here
...
5 | fn area(width: u32, height: u32) -> u32 {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `area` redefined here
```

Four things take the place of those features:

| Instead of | Write | In the output |
|---|---|---|
| overloading on arity | two names, as std does with `Vec::new` and `Vec::with_capacity` | `square_area(3)`, `rect_area(3, 4)` |
| a default argument | an `Option` parameter, and `unwrap_or` inside | `greeting(None)` |
| named arguments | a struct of options, or a builder | [Optional function arguments](../../17_Option_and_Result/optional_arguments/README.md), section 5 |
| overloading on type | a [generic](../../22_Generics/what_a_generic_is/README.md) parameter with a trait bound | — |

[Optional function arguments](../../17_Option_and_Result/optional_arguments/README.md) weighs five ways of making an argument optional and says when `Option` is the wrong one.

## A function is a value

`and` and `or` both have type `fn(bool, bool) -> bool`, so they can sit in an array and be called through a variable. The last block of the output prints both truth tables that way. A [closure](../../23_Closures/what_a_closure_is/README.md) is the anonymous version, and it can also capture variables from around it, which a `fn` cannot.

## Names are snake_case, and rustc checks

```text title="Abridged — real rustc output for camel.rs"
warning: function `squareArea` should have a snake case name
 --> camel.rs:1:4
  |
1 | fn squareArea(side: u32) -> u32 {
  |    ^^^^^^^^^^ help: convert the identifier to snake case: `square_area`
  |
  = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default
```

A warning, not an error, so the program still builds. Variables get the same check. Types and traits are `CamelCase` and constants `SCREAMING_SNAKE_CASE`, checked by `non_camel_case_types` and `non_upper_case_globals`, the other two lints in [`nonstandard_style` ↗](https://doc.rust-lang.org/rustc/lints/groups.html).

## Methods are functions in an `impl` block

A `fn` inside an `impl` block whose first parameter is `self`, `&self` or `&mut self` is a method, called as `value.name()`. Everything on this page still applies to it. [`impl` blocks](../../16_Structs/impl_blocks/README.md) covers the receiver.

## The verified output

<!-- output:functions -->
*Verified output of [`functions.rs`](examples/functions.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== the last expression is the return value ===
  percent( -5) = 0
  percent( 42) = 42
  percent(250) = 100

=== no -> means () ===
  log: saved
  log_line returned ()

=== arguments are evaluated before the call, left to right ===
  evaluating first
  evaluating second
  add = 5
  so and(left, right) evaluates both:
    evaluating left
    evaluating right
  and left && right stops at left:
    evaluating left

=== no overloading, no default arguments ===
  square_area(3)        = 9
  rect_area(3, 4)       = 12
  greeting(None)        = Hello, world!
  greeting(Some("Ada")) = Hello, Ada!

=== a parameter is a pattern ===
  swap((1, 2))      = (2, 1)
  first_of(7, true) = 7

=== a function is a value ===
  and over TT TF FT FF = [true, false, false, false]
  or  over TT TF FT FF = [true, true, true, false]
```
<!-- /output -->

## If you are coming from another language

**Python.** `def` → `fn`, and the three differences that matter all come from Rust writing the types down. First, type hints are optional in Python and ignored when the code runs; Rust's parameter types are required and checked. Second, a Python function with no `return` returns `None`, however its last line ends, while a Rust function returns its last expression, so forgetting `return` is how you get `None` in Python and adding a `;` is how you get `()` in Rust. Third, default and keyword arguments do not exist, and neither does their best-known trap: `def g(x=[])` builds the list once, when the `def` runs, and every call without `x` shares it (on Python 3.14, `print(g(), g())` with `x.append(1)` inside prints `[1, 1] [1, 1]`). The order rule is similar in effect but not in mechanism. Python looks a name up when the call runs, so calling a function defined further down works as long as its `def` has already run. Rust resolves names at compile time, so there is no such condition to get wrong.

**C.** A signature is a prototype that cannot get out of step with the definition, since it *is* the definition, and `()` plays the part of `void`. The difference worth remembering is argument order. C leaves the order in which arguments are evaluated unspecified, and compilers really do differ. The same `add(traced("first", 2), traced("second", 3))` in C prints `first` then `second` under Apple clang 21, and `second` then `first` under GCC 14.4 on x86-64 Linux, with or without `-O2`. Rust's left-to-right order is part of the language, so no compiler can change it.

**ABAP.** A method's `IMPORTING`, `EXPORTING` and `RETURNING` parameters are typed in the definition too, so the idea that a signature is written out is familiar. ABAP has two things Rust leaves out: `OPTIONAL` and `DEFAULT` on importing parameters, and named parameters at the call site. In Rust, both become one of the rows in the table above. Rust also has nothing like several `EXPORTING` parameters. A function returns one value, and when it needs to hand back several, that value is a tuple or a struct.

## See also

- [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) — why the last line is the return value, and the `;` that changes it
- [The unit type `()`](../../15_First_Programs/the_unit_type/README.md) — what a function with no `->` returns
- [Type inference](../../15_First_Programs/type_inference/README.md) — worked out inside a body, stated at the boundary
- [`and`, `or` and a first program's explanation, run](../../15_First_Programs/and_or_claims_checked/README.md) — a book chapter's two-line functions, and the claims around them checked
- [Meet the `bool`](../../15_First_Programs/meet_the_bool/README.md) — `&&` short-circuits, and a function taking both sides cannot
- [`impl` blocks](../../16_Structs/impl_blocks/README.md) — the same `fn`, with a receiver
- [What a closure is](../../23_Closures/what_a_closure_is/README.md) — the anonymous version, and what it can capture that a `fn` cannot
- [Optional function arguments](../../17_Option_and_Result/optional_arguments/README.md) — what to do without default arguments
- [The Reference: functions ↗](https://doc.rust-lang.org/reference/items/functions.html) — the grammar, including parameters as patterns
- [Comprehensive Rust: Functions ↗](https://google.github.io/comprehensive-rust/control-flow-basics/functions.html)
- [Listing 2.2, run](../../10_Resources/rust_in_action/first_steps/README.md) — parameter types required, the return type optional, and the semicolon that breaks `-> i32`, each shown by rustc
- [Returned by value](../../18_Ownership/returned_by_value/README.md) — what the type after `->` needs: a size the caller can reserve, and what to return instead of `str`

## Po polsku

Sygnatura funkcji nigdy nie jest wnioskowana: każdy parametr ma nazwę **i** typ, a wynik ma `->`. Wnioskowanie typów (*type inference*) działa w ciele funkcji, ale zatrzymuje się na granicy — `-> _` to `E0121`, choć podpowiedź kompilatora pokazuje, że zna właściwy typ. To celowa decyzja: sygnatura jest kontraktem, względem którego kompilują się wszyscy wywołujący, więc zmiana w ciele nie może jej po cichu przestawić. Parametr bez typu kończy się komunikatem *expected one of `:`, `@`, or `|`* — `@` i `|` pojawiają się tam, bo **parametr jest wzorcem** (*pattern*), tak jak lewa strona `let`: `fn swap((a, b): (i32, i32))` rozkłada krotkę już w nagłówku, a `_: bool` przyjmuje argument bez nadawania mu nazwy.

Wynikiem jest ostatnie wyrażenie **bez średnika** (*tail expression*) — to zwrot **niejawny** (*implicit*), a `return` to zwrot jawny, używany do wcześniejszego wyjścia. Brak `->` oznacza typ `()`; dopisany średnik na końcu też daje `()`, a `E0308` wskazuje poprawny nagłówek, podczas gdy błąd siedzi linijkę niżej pod znakiem `-`.

Kolejność definicji nie ma znaczenia: `main` może wołać funkcję zdefiniowaną niżej, bez prototypów. **Argumenty są obliczane przed wywołaniem, od lewej do prawej** — i to gwarancja języka zapisana w Reference, a nie cecha jednego kompilatora. W C kolejność jest nieokreślona: ten sam program wypisuje `first`, `second` pod clangiem i `second`, `first` pod GCC 14. Z tej reguły wynika pułapka: funkcja `and(a, b)` zwraca to samo co `a && b`, ale **nie skraca obliczeń** (*short-circuit*), bo oba argumenty są już policzone, zanim ruszy jej ciało.

Przeciążania (*overloading*), argumentów domyślnych ani nazwanych nie ma — druga definicja tej samej nazwy to `E0428`. W ich miejsce: dwie różne nazwy, parametr `Option`, struktura opcji albo *builder*, oraz typ generyczny z ograniczeniem cechą. Nazwy funkcji i zmiennych piszemy w `snake_case` i `rustc` to sprawdza ostrzeżeniem `non_snake_case`. Dla znających ABAP: typowane parametry `IMPORTING`/`EXPORTING`/`RETURNING` wyglądają znajomo, ale `OPTIONAL`, `DEFAULT` i nazwane parametry przy wywołaniu nie mają w Ruscie odpowiednika, a kilka wartości wynikowych zwraca się jako krotkę lub strukturę.

**Szukaj po polsku:** sygnatura funkcji · wartość zwracana bez `return` · kolejność obliczania argumentów · brak przeciążania funkcji w Ruscie · `rust missing return value semicolon E0308` · `rust E0121 placeholder return type` · `rust default arguments alternative`
