# What *Rust: The Practical Guide* says about functions and code blocks, run

**Level:** 101 · a companion to [Functions](../functions/README.md) and [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md)

**One line:** The five listings of §2.3 "Functions" and §2.4 "Code Blocks" in *Rust: The Practical Guide* do what the book says: three compile and produce the stated values, one is the compile error the book describes, and one compiles with three warnings the book does not mention. Four sentences in the explanation around them do not hold on rustc 1.98.0, and six more are true but narrower than they read.

```rust
fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Computing multiplication");
    num1 * num2
}

fn main() {
    let answer = multiplication(10, 15);
    println!("{answer}"); // 150
}
```

## Where it comes from

*Rust: The Practical Guide* by Nouman Azam (Rheinwerk Computing, 2025). Chapter 2 covers the basics; §2.3 "Functions" introduces `fn`, parameters, the return type and tuples as return values through Listings 2.5 to 2.8, and §2.4 "Code Blocks" introduces the brace block and its value through Listing 2.9. Two sidebars, "Expressions versus Statements" and "Functions versus Code Blocks", are where most of the sentences checked here come from. The book's shelf page is [*Rust: The Practical Guide*, run](../../10_Resources/rust_the_practical_guide/README.md). The program above is Listing 2.5 with a `main` around it. Every listing was compiled with bare `rustc --edition 2024`, which is what [the check behind this library](../../CONTRIBUTING.md) uses.

## Four statements that do not hold

**1. An expression is a code line that evaluates to a value, a statement is a line that returns nothing, and `println!` is a statement.** An expression is not a line. `if`, `match` and a block are expressions, and each sits on the right of a `let` (block 3 below), while one `let` statement can run over three lines. And `println!` is not a statement: a macro call is an expression, and this one expands to a block whose value is `()`. That is why `let unit: () = println!(...)` compiles (block 2) and why `fn shout(s: &str) -> () { println!("{s}!") }` can end in it, with no semicolon:

```text title="Real run — RUSTC_BOOTSTRAP=1 rustc --edition 2024 -Zunpretty=expanded expand_println.rs, the function only"
fn shout(s: &str) -> () {

    { ::std::io::_print(format_args!("{0}!\n", s)); }
}
```

What separates the two kinds is in [the Reference ↗](https://doc.rust-lang.org/reference/statements.html): a `let` and an item declaration are statements, and an *expression statement* is an expression followed by `;`. The `println!` line in Listing 2.5 is a statement because of its `;`, not because of the macro. [Flow control, run](../flow_control_claims_checked/README.md) has the transcript of `let` refused where a value is expected.

**2. A function may hold one and only one returning expression, and it must be the last expression.** Two arms of an `if` or a `match` are two expressions, and either one is the function's value (block 4). What the grammar refuses is two expressions in a row with no `;` between them, and the message says which character it wants:

```text title="Abridged — real rustc output for two_tails.rs"
error: expected `;`, found `num1`
 --> two_tails.rs:2:16
  |
2 |     num1 * num2
  |                ^ help: add `;` here
3 |     num1 + num2
  |     ---- unexpected token
```

So the rule is that a function body is a block, and a block has one tail. `return` is not a second returning expression but an exit: block 5 shows one leaving the whole function from inside a nested block.

**3. The `let full_name = { … }` line is an assignment statement, so a semicolon may be added after the block.** It is a `let` statement, not an assignment, and its `;` is required rather than permitted. Listing 2.9 without it stops in the parser:

```text title="Real rustc output for listing_2_9_no_semicolon.rs, a println! of full_name added after the block"
error: expected `;`, found `println`
 --> listing_2_9_no_semicolon.rs:6:6
  |
6 |     }
  |      ^ help: add `;` here
7 |     println!("{full_name}");
  |     ------- unexpected token
```

An assignment is `x = value` to a name that already exists, and it is an expression of type `()`, which [Flow control, run](../flow_control_claims_checked/README.md) shows with `let assigned = total = 5;`.

**4. A code block runs once; a function runs as often as it is called.** A block inside a loop runs on every pass (block 9): a `for` body is a block. What is true is that a block cannot be *called*. It has no name and no parameters, so using the same block twice means writing it twice.

## Listing 2.7 compiles, with three warnings the book does not mention

The book puts `return 45;` on the first line of `multiplication` to show an early return. It prints 45 as described, and rustc has three things to say about it:

```text title="Real rustc output for listing_2_7.rs"
warning: unreachable statement
 --> listing_2_7.rs:3:5
  |
2 |     return 45;
  |     --------- any code following this expression is unreachable
3 |     println!("Computing multiplication");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `num1`
 --> listing_2_7.rs:1:19
  |
1 | fn multiplication(num1: i32, num2: i32) -> i32 {
  |                   ^^^^ help: if this is intentional, prefix it with an underscore: `_num1`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `num2`
 --> listing_2_7.rs:1:30
  |
1 | fn multiplication(num1: i32, num2: i32) -> i32 {
  |                              ^^^^ help: if this is intentional, prefix it with an underscore: `_num2`

warning: 3 warnings emitted
```

The parameters count as unused because their only use, `num1 * num2`, can never run. [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) covers what the underscore in the help line would answer, and [Functions](../functions/README.md#return-is-for-leaving-early) shows the shape the listing was reaching for, a `return` inside an `if`.

## The claims, run

<!-- output:pg_fn_claims -->
*Verified output of [`pg_fn_claims.rs`](examples/pg_fn_claims.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Listing 2.5 prints its message, then the product
   Computing multiplication
   answer = 150

2. A println! call is an expression of type (), so it can be a tail
   this line is the value of a println! call
   the call evaluated to ()
   hello!
   shout("hello") is declared -> () and returned ()

3. An expression is not a line: if, match and a block have values
   if -> negative, match -> one digit, block -> -14
   and one let statement spread over three lines: total = -4

4. A tail can be an if or a match: either arm is the value
   sign(-3) = negative, sign(3) = not negative

5. return leaves the function, even from inside a block
   early(true) = 5, early(false) = 101

6. Listing 2.8 returns one value, a tuple, and a let pattern takes it apart
   basic_math(10, 15) = (150, 25, -5), type (i32, i32, i32)
   destructured: 150 25 -5

7. The argument's type may be decided by the parameter, or coerced to it
   let ten = 10; takes_i64(ten): ten is i64
   Function call with a variable
   my_fn(&owned) accepted a &String for a &str parameter

8. Listing 2.9: the block's value is its tail; the names inside are gone after it
   full_name = John Archer

9. A block inside a loop runs on every pass, and sees the loop variable
   pass 1: the block produced 1
   pass 2: the block produced 4
   pass 3: the block produced 9
   the block ran 3 times: [1, 4, 9]

10. A labelled block can leave early with a value
   'check on "John Archer" -> long
```
<!-- /output -->

Reading the blocks in order. Blocks 2 to 5 and 9 to 10 are behind the four statements above; the rest are the narrower ones.

1. **Listing 2.5.** As printed, with a `main`: `Computing multiplication`, then 150. The book's description holds, "the final expression becomes the return value" included. rustc's own word for it is *implicitly*, in the Listing 2.6 transcript below.
2. **`println!` is an expression.** Its value is `()`, bindable with a `let` and usable as a tail. The `-> ()` on `shout` is what a function with no arrow has anyway ([The unit type](../../15_First_Programs/the_unit_type/README.md)).
3. **An expression is not a line.** Three expressions on the right of three `let`s, and one `let` spread over three lines.
4. **Either arm is the value.** `sign` has two returning expressions in the book's sense, and neither is "the last expression of the function": the `if` is.
5. **`return` leaves the function.** `early(true)` is 5, not 105: the `return 5;` inside the block never let `v` be assigned. The book's word for a block's value, *returned*, is loose. A block *evaluates to* its tail, and `return` always means the enclosing function. Block 10 has the block-level early exit.
6. **Listing 2.8 returns one value.** The tuple `(150, 25, -5)` has type `(i32, i32, i32)`, and it is one thing: `let result = basic_math(10, 15)` holds it whole. "Multiple values" is a way of speaking, and the book's "destructed" is *destructured*: a `let` with a pattern, the same mechanism as a parameter ([Tuples](../../26_Collections/tuples/README.md)).
7. **The argument's type and the parameter's type must match.** They end up matching, and usually because the parameter decides. `let ten = 10;` is an `i64` because the call `takes_i64(ten)` needs one ([Type inference](../../15_First_Programs/type_inference/README.md)), and `my_fn(&owned)` hands a `&String` to a `&str` parameter through [deref coercion](../../15_First_Programs/what_an_annotation_does/README.md#a-borrow-the-annotation-is-a-coercion-site). Where neither applies, the mismatch is refused with the conversion named (transcript below). "Parameters are defined like variables" is narrower the other way: a parameter's type is mandatory and a `let`'s is optional. [Functions](../functions/README.md#the-signature-is-never-inferred) has the parse error for `fn add(a, b)`.
8. **Listing 2.9.** As printed it compiles with `unused variable: full_name`, since the listing never prints the name it built; printed, it is `John Archer`. `first_name` after the block is `E0425`, and rustc adds that the binding exists *in a different scope in the same function* (transcript below).
9. **A block runs on every pass.** Three passes, three values, and the block read the loop variable `i` without being handed it. The sidebar's "code blocks do not have explicit parameters; all variables in scope are visible" holds.
10. **A labelled block leaves early with a value.** `break 'check "long"` is the block-level counterpart of `return`: the shape of a helper with several `return`s, without the function ([Loop labels](../loop_labels/README.md#a-labelled-block-is-breakable-without-a-loop)).

One more sentence of the "Functions versus Code Blocks" sidebar has a page of its own. That a function "can only access variables that are either passed as parameters or locally defined" is true of `let` bindings and too narrow for items: a `fn` written inside another function sees every `const`, `static` and `fn` in scope and none of the enclosing function's locals, and asking it to is `E0434`. [Items inside a function](../../27_Modules/items_inside_a_function/README.md) runs that, beside the closure that does capture.

## Claims the compiler settles

**Listing 2.6.** The book quotes the error correctly. The label under the signature is worth reading too, since it is where the word *implicitly* comes from:

```text title="Abridged — real rustc output for listing_2_6.rs"
error[E0308]: mismatched types
 --> listing_2_6.rs:1:44
  |
1 | fn multiplication(num1: i32, num2: i32) -> i32 {
  |    --------------                          ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     println!("Computing multiplication");
3 |     num1 * num2;
  |                - help: remove this semicolon to return this value
```

**A real type mismatch.** With `let n: i32 = 10;` passed to `fn takes_i64(n: i64)`, inference has nothing left to decide and no coercion applies, and the help names the conversion:

```text title="Abridged — real rustc output for param_mismatch.rs"
error[E0308]: mismatched types
 --> param_mismatch.rs:7:30
  |
7 |     println!("{}", takes_i64(n));
  |                    --------- ^ expected `i64`, found `i32`
  |                    |
  |                    arguments to this function are incorrect
  |
help: you can convert an `i32` to an `i64`
  |
7 |     println!("{}", takes_i64(n.into()));
  |                               +++++++
```

**A block's names are gone after it.** Listing 2.9 with `first_name` printed after the block:

```text title="Abridged — real rustc output for first_name_after_block.rs"
error[E0425]: cannot find value `first_name` in this scope
 --> first_name_after_block.rs:7:28
  |
7 |     println!("{full_name} {first_name}");
  |                            ^^^^^^^^^^
  |
help: the binding `first_name` is available in a different scope in the same function
 --> first_name_after_block.rs:3:13
  |
3 |         let first_name = "John";
  |             ^^^^^^^^^^
```

**`return` on the last line.** The book's "add the `return` keyword" is for leaving early. On the last line it compiles, and clippy asks for it back:

```text title="Abridged — real clippy 0.1.98 output for needless_return.rs, via clippy-driver --edition 2024"
warning: unneeded `return` statement
 --> needless_return.rs:2:5
  |
2 |     return num1 * num2;
  |     ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(clippy::needless_return)]` on by default
help: remove `return`
  |
2 -     return num1 * num2;
2 +     num1 * num2
```

**snake_case is checked.** The book gives the convention, and rustc enforces it with a warning: `fn myFn` builds and runs, under `non_snake_case`.

```text title="Abridged — real rustc output for my_fn_camel.rs"
warning: function `myFn` should have a snake case name
 --> my_fn_camel.rs:1:4
  |
1 | fn myFn(s: &str) {
  |    ^^^^ help: convert the identifier to snake case: `my_fn`
```

## Practice

**Six functions and blocks: which compile, and what does each return?**

Predict, for each of the six, whether it compiles and what value it produces, then run the solution and compare. `triple` is `fn triple(n: i32) -> i32 { n * 3 }`.

```rust,ignore
fn a() -> i32 { 5; }
fn b(x: i32) -> i32 { if x > 0 { return x; } 0 }
fn c(x: i32) -> &'static str { if x % 2 == 0 { "even" } else { "odd" } }
fn d() { 5 }
fn e() -> (i32, i32) { (1, 2) }
let v = { let x = 2; triple(x) };   let w = { let x = 2; triple(x); };
```

For the two that do not compile, say which line of the error names the cause.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg_fn_kata -->
*[`pg_fn_kata.rs`](examples/pg_fn_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: six functions and blocks — which compile, and what each
//! one returns. The two that do not compile are described, not written;
//! their transcripts are on the page.

fn triple(n: i32) -> i32 {
    n * 3
}

fn b(x: i32) -> i32 {
    if x > 0 {
        return x;
    }
    0
}

fn c(x: i32) -> &'static str {
    if x % 2 == 0 { "even" } else { "odd" }
}

fn e() -> (i32, i32) {
    (1, 2)
}

fn main() {
    println!("1. fn a() -> i32 {{ 5; }}");
    println!("   does not compile: E0308, expected `i32`, found `()` -- the ; made the tail a statement");

    println!("2. fn b(x: i32) -> i32 {{ if x > 0 {{ return x; }} 0 }}");
    println!("   compiles: b(4) = {}, b(-4) = {}", b(4), b(-4));

    println!("3. fn c(x: i32) -> &'static str {{ if x % 2 == 0 {{ \"even\" }} else {{ \"odd\" }} }}");
    println!("   compiles: c(7) = {}, c(8) = {}", c(7), c(8));

    println!("4. fn d() {{ 5 }}");
    println!("   does not compile: E0308, expected `()`, found integer -- no -> means ()");

    println!("5. fn e() -> (i32, i32) {{ (1, 2) }}");
    let (p, q) = e();
    println!("   compiles: e() = {:?}, one value; p + q = {}", e(), p + q);

    println!("6. let v = {{ let x = 2; triple(x) }}; and let w = {{ let x = 2; triple(x); }};");
    let v = {
        let x = 2;
        triple(x)
    };
    let w = {
        let x = 2;
        triple(x);
    };
    println!("   both compile: v = {v}, w = {w:?}");
}
```
<!-- /source -->

<!-- output:pg_fn_kata -->
*Verified output of [`pg_fn_kata.rs`](examples/pg_fn_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. fn a() -> i32 { 5; }
   does not compile: E0308, expected `i32`, found `()` -- the ; made the tail a statement
2. fn b(x: i32) -> i32 { if x > 0 { return x; } 0 }
   compiles: b(4) = 4, b(-4) = 0
3. fn c(x: i32) -> &'static str { if x % 2 == 0 { "even" } else { "odd" } }
   compiles: c(7) = odd, c(8) = even
4. fn d() { 5 }
   does not compile: E0308, expected `()`, found integer -- no -> means ()
5. fn e() -> (i32, i32) { (1, 2) }
   compiles: e() = (1, 2), one value; p + q = 3
6. let v = { let x = 2; triple(x) }; and let w = { let x = 2; triple(x); };
   both compile: v = 6, w = ()
```
<!-- /output -->

The two refusals, from rustc 1.98.0. Both are `E0308`, and both point at the signature while the cause sits in the body: for `a` the `;`, for `d` the `-> i32` the help line suggests.

```text title="Abridged — real rustc output for kata_a.rs"
error[E0308]: mismatched types
 --> kata_a.rs:1:11
  |
1 | fn a() -> i32 {
  |    -      ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     5;
  |      - help: remove this semicolon to return this value
```

```text title="Abridged — real rustc output for kata_d.rs"
error[E0308]: mismatched types
 --> kata_d.rs:2:5
  |
1 | fn d() {
  |       - help: try adding a return type: `-> i32`
2 |     5
  |     ^ expected `()`, found integer
```

</details>

## If you are coming from another language

**Python.** A `def` body returns `None` unless a `return` runs, so "the last line is the value" is the habit to unlearn in each direction: a Python function whose last line is `a * b` returns `None`, and a Rust function whose last line is `a * b;` returns `()`, which is a compile error when the signature promised an `i32`, where Python would hand the `None` on. Python 3.14 has no block scope: a name bound inside an `if` or a `for` is still there after it, so Listing 2.9's `first_name` would survive its block. Multiple return values look the same, `return a * b, a + b`, and are the same thing, one tuple.

**C.** Block scope works as in Listing 2.9, but a block has no value: `int v = { int a = 2; a * 3 };` is *expected expression* under Apple clang 21 with `-std=c11`. GCC and clang accept `({ int a = 2; a * 3; })` as a *statement expression*, an extension that `-std=c11 -pedantic-errors` rejects; Rust's block value is the language, not an extension. A parameter's type is mandatory in C too. What Rust adds is the return type after the name instead of before it, and `()` in place of `void`.

**Ruby.** The nearest relative of the tail expression: a Ruby method returns the value of the last expression it evaluated, and `return` is for leaving early, as the Ruby library's [The last expression is the return value ↗](https://masiarek.github.io/ruby-learning-library/02_Methods_and_Arguments/the_last_expression_is_the_value/index.html) shows. The difference is the `;`. A Ruby method ending in `puts` returns `nil` because `puts` returns `nil`; a Rust function ending in `println!(...)` with no semicolon returns `()` for the same reason, and with a semicolon returns `()` because of the semicolon.

## See also

- [Functions](../functions/README.md) — the signature never inferred, `()` for no `->`, `return` for the early exit, and the transcripts for `fn add(a, b)` and `fn squareArea`
- [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) — the two jobs of `{ }`, and the semicolon as the switch
- [The unit type `()`](../../15_First_Programs/the_unit_type/README.md) — what `println!`, a block with a `;` and a function with no arrow all produce
- [Variables](../../15_First_Programs/variables/README.md) — a binding is scoped to its block
- [Type inference](../../15_First_Programs/type_inference/README.md) — how `let ten = 10` becomes an `i64`
- [What a type annotation does](../../15_First_Programs/what_an_annotation_does/README.md) — the coercion that lets a `&String` reach a `&str` parameter
- [Tuples](../../26_Collections/tuples/README.md) — Listing 2.8's one value
- [Loop labels](../loop_labels/README.md) — `break 'label value`, the block-level early exit
- [Items inside a function](../../27_Modules/items_inside_a_function/README.md) — what a function can see: items yes, an enclosing `let` no
- [What a closure is](../../23_Closures/what_a_closure_is/README.md) — the function that does see the variables around it
- [`const` and `static`](../../27_Modules/const_and_static/README.md) — the items a function body can hold, and every nested `fn` can read
- [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) — Listing 2.7's three warnings, and what `_num1` would answer
- [Returned by value](../../18_Ownership/returned_by_value/README.md) — what the type after `->` needs from the caller
- [`and`, `or` and a first program's explanation, run](../../15_First_Programs/and_or_claims_checked/README.md) — another book's functions section, checked the same way
- [Flow control, run](../flow_control_claims_checked/README.md) — *Rust in Action* §2.4, where `let` as a statement and `=` as an expression are run
- [*Rust: The Practical Guide*, run](../../10_Resources/rust_the_practical_guide/README.md) — the book's shelf page, with every section checked so far

## Sources

- Nouman Azam, *Rust: The Practical Guide* (Rheinwerk Computing, 2025), §2.3 "Functions" (Listings 2.5–2.8) and §2.4 "Code Blocks" (Listing 2.9), with the sidebars "Expressions versus Statements" and "Functions versus Code Blocks"
- The Rust Reference: [statements ↗](https://doc.rust-lang.org/reference/statements.html), [block expressions ↗](https://doc.rust-lang.org/reference/expressions/block-expr.html), [functions ↗](https://doc.rust-lang.org/reference/items/functions.html)
- Clippy: [`needless_return` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#needless_return)

## Po polsku

Sekcje §2.3 „Functions" i §2.4 „Code Blocks" książki *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) pokazują pięć listingów i wszystkie robią to, co obiecuje tekst — z jednym zastrzeżeniem: Listing 2.7 z `return 45;` w pierwszej linii daje trzy ostrzeżenia (*unreachable statement* i dwa *unused variable*, bo jedyne użycie parametrów nigdy się nie wykona), o których książka nie wspomina. Cztery zdania z objaśnień nie wytrzymują sprawdzenia na rustc 1.98.0. Wyrażenie (*expression*) nie jest „linią kodu": `if`, `match` i blok są wyrażeniami, a jedna instrukcja `let` może zajmować trzy linie; `println!` nie jest instrukcją (*statement*), tylko wywołaniem makra, które rozwija się do bloku o wartości `()`, więc `fn shout(s: &str) -> () { println!("{s}!") }` kompiluje się bez średnika. Zdanie „tylko jedno wyrażenie zwracające" jest mylące: ogon funkcji (*tail expression*) może być `if`-em z dwoma ramionami, a gramatyka odrzuca dopiero dwa wyrażenia pod rząd bez średnika (*expected `;`*). `let full_name = { ... };` to instrukcja `let`, a nie „przypisanie", i średnik jest tam wymagany, nie dozwolony. Blok w pętli wykonuje się w każdym przebiegu, więc „jednorazowe wykonanie" nie jest jego cechą — blok po prostu nie ma nazwy, więc nie da się go wywołać.

Sześć zdań jest prawdziwych, ale węższych, niż brzmią. Typ argumentu często ustala parametr: `let ten = 10;` staje się `i64`, bo tak chce wywołana funkcja, a `&String` trafia do parametru `&str` przez koercję (*deref coercion*); prawdziwa niezgodność to `E0308` z podpowiedzią `.into()`. Typ parametru jest obowiązkowy, a typ przy `let` nie. „Wiele wartości" z funkcji to jedna krotka (*tuple*) typu `(i32, i32, i32)`, rozkładana wzorcem w `let`. Blok nie „zwraca" wartości, tylko ją ma — `return` wewnątrz bloku opuszcza całą funkcję (`early(true)` daje 5, nie 105), a wyjściem z samego bloku jest `break 'etykieta wartość`. `return` w ostatniej linii kompiluje się, ale clippy prosi o jego usunięcie (`needless_return`). Funkcja widzi nie tylko parametry i zmienne lokalne, ale też wszystkie elementy (*items*) w zasięgu: `const`, `static`, inne funkcje — a nie widzi zmiennych `let` funkcji, w której jest zagnieżdżona (`E0434`).

**Szukaj po polsku:** wyrażenie a instrukcja w Ruscie · wartość bloku kodu · ostatnie wyrażenie bez średnika · zwracanie krotki z funkcji · `rust expected i32 found ()` · `rust unreachable statement return` · `rust clippy needless_return` · Rust: The Practical Guide errata
