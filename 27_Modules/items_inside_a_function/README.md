# Items inside a function

**Level:** 201 · working knowledge

**One line:** A `fn`, `struct`, `impl`, `use`, `const` or `static` written inside a function body is an ordinary item that only that block can name — not a closure — so it is in scope for the whole block, the lines above it included, and a nested `fn` sees every item around it and none of the enclosing function's variables: asking it to is `E0434`.

```rust
fn main() {
    let bonus = 10;
    fn double(n: i32) -> i32 {
        n * 2 // an item: it cannot read `bonus`
    }
    let with_bonus = |n: i32| n + bonus; // a closure: it can
    println!("{} {}", double(1), with_bonus(1)); // 2 11
}
```

## The items you can nest

Anything declared at module level can be declared inside a block. [The Reference ↗](https://doc.rust-lang.org/reference/statements.html#item-declarations) lists an *item declaration* as one kind of statement, beside `let` and the expression statement. The ones people nest most:

| Item | Why inside a function | In the output |
|---|---|---|
| [`fn`](../../25_Control_Flow/functions/README.md) | a helper only this function calls | blocks 1, 2, 5, 6 |
| [`struct`](../../16_Structs/what_a_struct_is/README.md) and its [`impl`](../../16_Structs/impl_blocks/README.md) | a shape only this function builds | block 3 |
| [`use`](../the_use_declaration/README.md) | a shorter name for one body | block 4 |
| [`const`](../const_and_static/README.md#a-const-inside-a-function) | a named value, pasted in at every use | block 2 |
| [`static`](../const_and_static/README.md) | one address for the whole program, named locally | block 2 |

## What a nested `fn` can see

Items, not locals. *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025, §2.4) says a function can only access variables that were passed as parameters or defined locally, and that is exactly right for `let` bindings and too narrow for everything else. Block 2 has `next` reading a `const` and a `static` declared in `main`, and it could call any other `fn` in scope. What it cannot read is `bonus`, and the message says why in five words:

```text title="Real rustc output for e0434.rs, the unused-variable warning dropped"
error[E0434]: can't capture dynamic environment in a fn item
 --> e0434.rs:4:13
  |
4 |         n + bonus
  |             ^^^^^
  |
  = help: use the `|| { ... }` closure form instead
```

A `fn` item is compiled once and carries nothing per call; a closure is a struct holding what it captured, which is [what a closure is](../../23_Closures/what_a_closure_is/README.md#the-capture-is-the-whole-difference). The help line is the fix, and block 2 shows the two constructs that do see `bonus`: a block, because it is part of the same body, and a closure, because it captures.

The same wall stands between a nested `fn` and the outer function's type parameters. The note is the rule in one line:

```text title="Abridged — real rustc output for e0401_fn.rs"
error[E0401]: can't use generic parameters from outer item
 --> e0401_fn.rs:3:19
  |
1 | fn width<T>() -> usize {
  |          - type parameter from outer item
2 |     fn inner() -> usize {
  |        ----- generic parameter used in this inner function
3 |         size_of::<T>()
  |                   ^ use of generic parameter from outer item
  |
  = note: nested items are independent from their parent item for everything except for privacy and name resolution
help: try introducing a local generic parameter here
  |
2 |     fn inner<T>() -> usize {
  |             +++
```

Block 6 takes the help: `inner<U>` declares its own parameter, and `width` passes its `T` in.

## Order does not matter

Block 1 calls `helper(2)` three lines above `fn helper`. An item's name covers the whole block it is declared in; a `let` is in scope from its own line down, and reading it earlier is `E0425`:

```text title="Abridged — real rustc output for let_before_its_line.rs"
error[E0425]: cannot find value `later` in this scope
 --> let_before_its_line.rs:3:20
  |
3 |     println!("{}", later);
  |                    ^^^^^ not found in this scope
```

The same file calls `helper(2)` on the line above that one, before its `fn`, and the error does not mention it. An item declared under the statements that use it compiles and reads like a bug; clippy's [`items_after_statements` ↗](https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#items_after_statements) says so, from `clippy::pedantic`, so it is off by default. Items first is the usual layout.

## The name stays inside, the value can leave

A nested `struct` can be built, given an `impl`, and returned, as long as the signature does not have to spell its name. Block 3 returns a `Point` through `-> impl Display`; `Box<dyn Display>` would do the same. Naming it in the signature fails, because the name is scoped to the body and the signature sits outside it:

```text title="Abridged — real rustc output for struct_name_escapes.rs"
error[E0425]: cannot find type `Point` in this scope
 --> struct_name_escapes.rs:1:14
  |
1 | fn make() -> Point {
  |              ^^^^^ not found in this scope
```

A `use` is scoped the same way. Block 4's `HashSet` is imported inside `count_distinct`, and the same name in `main` is unresolved, with rustc suggesting the import it cannot see from there:

```text title="Abridged — real rustc output for use_scoped_to_fn.rs, the second error (E0433, for HashMap::new()) dropped"
error[E0425]: cannot find type `HashMap` in this scope
 --> use_scoped_to_fn.rs:8:12
  |
8 |     let m: HashMap<&str, i32> = HashMap::new();
  |            ^^^^^^^ not found in this scope
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |
```

## Nested `fn` versus closure

| | nested `fn` | closure |
|---|---|---|
| captures the locals | no, `E0434` | yes |
| type | a fn item type, printed as `main::double` | an anonymous type, printed as `main::{{closure}}` |
| size | 0 bytes | what it captured: 0 bytes for nothing, 4 for one `i32` |
| coerces to `fn(i32) -> i32` | always | only when it captured nothing |
| in scope | the whole block | from its `let` down |

Block 5 prints the type names and sizes, and [Function pointers](../../23_Closures/function_pointers/README.md) is where the fourth row leads. A capturing closure is refused with the reason spelled out:

```text title="Abridged — real rustc output for capturing_closure_to_fn_ptr.rs"
error[E0308]: mismatched types
 --> capturing_closure_to_fn_ptr.rs:8:26
  |
7 |     let add_bonus = |n: i32| n + bonus;
  |                     -------- the found closure
8 |     println!("{}", apply(add_bonus, 1));
  |                    ----- ^^^^^^^^^ expected fn pointer, found closure
  |                    |
  |                    arguments to this function are incorrect
  |
  = note: expected fn pointer `fn(i32) -> i32`
                found closure `{closure@capturing_closure_to_fn_ptr.rs:7:21: 7:29}`
note: closures can only be coerced to `fn` types if they do not capture any variables
 --> capturing_closure_to_fn_ptr.rs:7:34
  |
7 |     let add_bonus = |n: i32| n + bonus;
  |                                  ^^^^^ `bonus` captured here
```

## When a nested helper reads better, and what it costs

A helper nested in the only function that calls it says so by its position: nobody else can call it, and a reader does not have to search the module for other uses. The cost is that nobody else can test it either. A `#[test]` on a nested function is not an error; it is a warning and a test that never runs:

```text title="Abridged — real rustc --edition 2024 --test output for test_in_fn.rs, then the test binary's run"
warning: cannot test inner items
 --> test_in_fn.rs:5:5
  |
5 |     #[test]
  |     ^^^^^^^
  |
  = note: `#[warn(unnameable_test_items)]` on by default

warning: function `helper_doubles` is never used
 --> test_in_fn.rs:6:8
  |
6 |     fn helper_doubles() {
  |        ^^^^^^^^^^^^^^

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The lint's name says why: the harness refers to every test by path, and an item inside a function body has none. A helper worth a test of its own goes to module level, private, where [Modules and visibility](../modules_and_visibility/README.md) keeps it out of everyone else's way.

## The verified output

<!-- output:items_in_fn -->
*Verified output of [`items_in_fn.rs`](examples/items_in_fn.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A nested fn is callable above its own line; a let is not
   helper(2) = 20

2. What a nested fn sees: const, static and other items, never a let
   next(2) = 3 (reads STEP and LIMIT)
   a block sees the let: 101
   a closure captures it: 101

3. A nested struct: the value leaves through impl Display, the name stays inside
   origin_report() = (3, 4)

4. A use inside a function shortens names for that block only
   count_distinct("a b a c") = 3

5. Nested fn versus closure: type, size, and coercion to a fn pointer
   double:     items_in_fn::main::double (0 bytes)
   triple:     items_in_fn::main::{{closure}} (0 bytes)
   add_offset: items_in_fn::main::{{closure}} (4 bytes)
   apply(double, 1) = 2, apply(triple, 1) = 3
   both coerce to fn(i32) -> i32; add_offset captured offset and cannot (E0308)

6. A nested fn cannot use the outer fn's type parameter; it declares its own
   width::<u32>() = 4, width::<u8>() = 1
```
<!-- /output -->

## The trap it exists for

Moving a closure body into a nested `fn` "to give it a name" and meeting `E0434`, because the closure was capturing a local. Two fixes: pass the captured value as a parameter, as the kata does, or keep the closure and let the `let` be its name.

## Practice

**From a closure to a nested `fn`, and a type that never leaves.**

Start from `let factor = 3; let scale = |x: i32| x * factor;` applied to `[1, 2, 3]`. Rewrite `scale` as a nested `fn` and say what had to change in its signature, and why the closure needed no such change. Then show one call to a nested `fn` that sits above the `fn` itself, and say why the same cannot be done with the closure.

Next, write `fn celsius_report(c: f64) -> impl Display` whose `struct Celsius(f64)` and its `impl Display` live inside the function, and print `celsius_report(21.5)`. Say why `let t: Celsius = Celsius(21.5);` cannot be written in `main`.

Finish with a nested `fn` that reads a `const` declared *below* it and a `static` beside it, and name the one kind of binding it can never read.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:items_in_fn_kata -->
*[`items_in_fn_kata.rs`](examples/items_in_fn_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: from a closure to a nested fn, and a type that never
//! leaves its function.

use std::fmt;

/// `Celsius` and its `impl` live here. The value leaves through
/// `impl Display`; the name does not.
fn celsius_report(c: f64) -> impl fmt::Display {
    struct Celsius(f64);
    impl fmt::Display for Celsius {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:.1} C", self.0)
        }
    }
    Celsius(c)
}

fn main() {
    println!("=== part 1: the captured factor becomes a parameter ===");
    let factor = 3;
    let scale = |x: i32| x * factor;
    fn scale_by(x: i32, factor: i32) -> i32 {
        x * factor
    }
    let by_closure: Vec<i32> = [1, 2, 3].iter().map(|&x| scale(x)).collect();
    let by_fn: Vec<i32> = [1, 2, 3].iter().map(|&x| scale_by(x, factor)).collect();
    println!("  closure:                    {by_closure:?}");
    println!("  nested fn, factor passed in: {by_fn:?}");
    println!("  fn scale(x: i32) -> i32 {{ x * factor }} is E0434: a fn item captures nothing");

    println!("\n=== part 1b: an item is in scope for the whole block, a let from its line down ===");
    println!("  first_word(\"John Archer\") = {:?}, called above its fn", first_word("John Archer"));
    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }
    println!("  a closure is bound by a let, so using it above that line is E0425");

    println!("\n=== part 2: the value leaves, the name does not ===");
    println!("  celsius_report(21.5) = {}", celsius_report(21.5));
    println!("  naming Celsius in main does not compile: the name is scoped to the block");

    println!("\n=== part 3: what a nested fn sees ===");
    fn bump(n: i32) -> i32 {
        (n + BASE).min(CAP)
    }
    const BASE: i32 = 10;
    static CAP: i32 = 25;
    println!("  bump(20) = {} -- reads BASE (declared below it) and CAP", bump(20));
    println!("  a let is the one thing it cannot read; a block or a closure can");
}
```
<!-- /source -->

<!-- output:items_in_fn_kata -->
*Verified output of [`items_in_fn_kata.rs`](examples/items_in_fn_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== part 1: the captured factor becomes a parameter ===
  closure:                    [3, 6, 9]
  nested fn, factor passed in: [3, 6, 9]
  fn scale(x: i32) -> i32 { x * factor } is E0434: a fn item captures nothing

=== part 1b: an item is in scope for the whole block, a let from its line down ===
  first_word("John Archer") = "John", called above its fn
  a closure is bound by a let, so using it above that line is E0425

=== part 2: the value leaves, the name does not ===
  celsius_report(21.5) = 21.5 C
  naming Celsius in main does not compile: the name is scoped to the block

=== part 3: what a nested fn sees ===
  bump(20) = 25 -- reads BASE (declared below it) and CAP
  a let is the one thing it cannot read; a block or a closure can
```
<!-- /output -->

The refusal the solution describes in part 2, from rustc 1.98.0. Two errors, because `Celsius` is both a type and a constructor and the `let` names it twice:

```text title="Abridged — real rustc output for celsius_outside.rs"
error[E0425]: cannot find type `Celsius` in this scope
  --> celsius_outside.rs:14:12
   |
14 |     let t: Celsius = Celsius(21.5);
   |            ^^^^^^^ not found in this scope

error[E0425]: cannot find function, tuple struct or tuple variant `Celsius` in this scope
  --> celsius_outside.rs:14:22
   |
14 |     let t: Celsius = Celsius(21.5);
   |                      ^^^^^^^ not found in this scope
```

</details>

## If you are coming from another language

**Python.** A nested `def` *is* a closure: it reads the enclosing function's variables, and on Python 3.14 an `inner` reading `bonus` prints 11 with nothing extra. Only assignment needs a declaration. `count += 1` inside the nested function is `UnboundLocalError: cannot access local variable 'count' where it is not associated with a value`, until `nonlocal count` makes it the outer variable. Rust splits reading from capturing: a nested `fn` reads nothing from outside, a closure captures, and [`move`](../../23_Closures/the_move_keyword/README.md) decides whether it holds the value or a borrow. A `def` is also a statement that runs, so the name exists from the line the `def` executes, like a Rust `let`; a Rust `fn` item exists for the whole block.

**JavaScript.** A function declaration is hoisted with its body, so calling it above its line works, and it is a closure over the enclosing scope. Rust keeps the first half, an item in scope for the whole block, and drops the second, since a `fn` captures nothing. The JavaScript library's [Hoisting and the temporal dead zone ↗](https://masiarek.github.io/javascript-typescript-learning-library/04_Variables_and_Scope/hoisting_and_the_tdz/index.html) and [Closures ↗](https://masiarek.github.io/javascript-typescript-learning-library/04_Variables_and_Scope/closures/index.html) are the two halves.

**Ruby.** `def` is a scope gate: a `def` inside a method does not see the method's locals, and reading one is `NameError: undefined local variable or method 'bonus' for main` on Ruby 4.0. That is Rust's split, with Rust's fix: a lambda `->(n) { n + bonus }` sees `bonus` and prints 11. [Closures capture variables, not values ↗](https://masiarek.github.io/ruby-learning-library/03_Blocks_Procs_and_Lambdas/closures_capture_variables/index.html) in the Ruby library is the capturing half.

**C.** Standard C has no nested functions: Apple clang 21 with `-std=c11` stops at the inner definition with *function definition is not allowed here* (GCC accepts one, as an extension). A `static` function at file scope is the usual substitute, and a private module-level `fn` is its Rust counterpart.

**Java.** A local class can read a local variable of the enclosing method only if that variable is final or effectively final: on javac 25 a local `Adder` reading `bonus` compiles, and a local `Reader` reading a `count` that was assigned twice is *local variables referenced from an inner class must be final or effectively final*. A Rust local `struct` reads no local at all; its methods are `fn` items, and the data they work on comes in as fields or parameters.

## See also

- [`const` and `static`](../const_and_static/README.md) — a `const` inside a function, and what it cannot see: a `let` (`E0435`) and the outer `T` (`E0401`)
- [What a closure is](../../23_Closures/what_a_closure_is/README.md) — the nested function that can capture
- [Function pointers](../../23_Closures/function_pointers/README.md) — what both coerce to when they capture nothing
- [Functions](../../25_Control_Flow/functions/README.md) — the top-level `fn` this nests
- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) — a name visible only inside a block
- [Modules and visibility](../modules_and_visibility/README.md) — the module-level alternative, and where a helper worth its own test goes
- [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) — the block that does see the locals, and its value
- [What *Rust: The Practical Guide* says about functions and code blocks, run](../../25_Control_Flow/functions_claims_checked/README.md) — the book sentence this page answers, beside the rest of its §2.3 and §2.4

## Po polsku

`fn`, `struct` z `impl`, `use`, `const` czy `static` zapisane wewnątrz ciała funkcji to zwykłe elementy (*items*) widoczne w całym bloku — także powyżej linii, w której je zadeklarowano — a nie domknięcia (*closures*). Zagnieżdżona funkcja widzi wszystkie elementy w zasięgu (stałe, `static`, inne funkcje), ale żadnej zmiennej `let` funkcji zewnętrznej: próba to `E0434` *can't capture dynamic environment in a fn item*, a podpowiedź kompilatora wskazuje domknięcie `|| { ... }`. Nie widzi też parametrów generycznych funkcji zewnętrznej (`E0401`) i musi zadeklarować własne. Blok i domknięcie widzą zmienne lokalne. Nazwa zagnieżdżonej struktury nie wychodzi poza blok (`E0425` w sygnaturze), ale jej wartość może wyjść przez `impl Display`. `use` wewnątrz funkcji skraca nazwy tylko w tym bloku. Zagnieżdżona funkcja i domknięcie, które nic nie przechwyciło, mają 0 bajtów i oba rzutują się na wskaźnik `fn(i32) -> i32`; domknięcie, które coś przechwyciło, nie (`E0308`). `#[test]` na funkcji zagnieżdżonej to ostrzeżenie `unnameable_test_items` i test, który nigdy się nie uruchomi — pomocnik wart testu idzie na poziom modułu. Zdanie z *Rust: The Practical Guide* (§2.4), że funkcja widzi tylko parametry i zmienne lokalne, jest prawdziwe dla `let`, a za wąskie dla elementów. Kto przychodzi z Pythona, gdzie zagnieżdżony `def` jest domknięciem, trafia na `E0434` najczęściej; w Ruby `def` jest bramą zasięgu (*scope gate*) dokładnie jak `fn` w Ruscie.

**Szukaj po polsku:** funkcja zagnieżdżona · element wewnątrz funkcji · struktura lokalna w funkcji · `rust nested function E0434` · `rust fn inside fn vs closure` · `rust cannot test inner items` · `rust E0401 generic parameters from outer item`
