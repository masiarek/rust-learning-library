# Variables

**Level:** 101 · for newcomers

**One line:** `let` binds a name to a value, and the binding is **immutable by default** — `mut` is not a convenience you sprinkle on, it is a promise to the next reader that this name changes.

```rust
let x: i32 = 10;
println!("{x}");  // 10
```

That is the whole of the syntax. The annotation is optional — `let x = 10;` is the same program — and [what a type annotation does](../what_an_annotation_does/README.md) is its own page.

## The second line is the lesson

```rust
let x: i32 = 10;
// x = 20;        // error[E0384]: cannot assign twice to immutable variable
```

```text title="Abridged — real rustc output for immutable.rs"
error[E0384]: cannot assign twice to immutable variable `x`
 --> immutable.rs:3:5
  |
2 |     let x: i32 = 10;
  |         - first assignment to `x`
3 |     x = 20;
  |     ^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x: i32 = 10;
  |         +++
```

`x` is not a box holding 10 that you may refill. It is a name for a value, and by default the name is spoken for. Note what the compiler offers: not "use a different name" but "make this binding mutable" — the fix is one word, and the word is a decision.

## `mut` is the permission assignment needs

```rust
let mut count = 0;
count = 1;
count += 1;
println!("{count}");  // 2
```

Nothing else changes. `mut` does not make the value special, it makes the *name* writable.

And the promise is enforced in both directions — a `mut` nothing ever writes to is a warning:

```text title="Abridged — real rustc output for unused_mut.rs"
warning: variable does not need to be mutable
 --> unused_mut.rs:2:9
  |
2 |     let mut total = 0;
  |         ----^^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default
```

So `mut` in Rust source means *this changes below*, reliably, because the compiler will not let it mean anything weaker. That is worth more than the mutation it permits: scanning a function for `mut` tells you which of its names are moving parts. [What a warning is asking](../what_a_warning_is_asking/README.md) reads this one and its neighbours in full.

## Mutability belongs to the binding, not to the value

```rust
let owned = String::from("Ada");   // not mut
let mut moved = owned;             // now it is
moved.push_str(" Lovelace");
println!("{moved}");  // Ada Lovelace
```

The same heap bytes were immutable under one name and writable under the next, and nothing about the `String` changed. This is the sentence that stops `mut` being read as "this data is mutable", and [a name is not a place](../../18_Ownership/a_name_is_not_a_place/README.md) proves it properly, with the borrow checker rather than with addresses.

## A binding is scoped to its block

```rust
let outer = "visible to the end of main";
{
    let inner = "visible only inside these braces";
    println!("{inner}");
}
println!("{outer}");
// println!("{inner}");   // error[E0425]: cannot find value `inner` in this scope
```

The compiler's help line on that error is unusually good — it says the binding *exists in a different scope in the same function* and points at it, rather than assuming a typo. Braces are the unit: [a block is an expression](../a_block_is_an_expression/README.md) is the same braces doing their other job, and [scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) is what "goes out of scope" actually means once a value can be moved out of one.

## `let` again is a new variable, not an assignment

```rust
let spaces = "   ";
let spaces = spaces.len();
println!("{spaces}");  // 3
```

No `mut`, and no error — because nothing was assigned. A *second* variable was declared and took over the name, and it may have a different type. That is **shadowing**, it is a large enough topic to have its own map, and the distinction from `mut` is not stylistic: a shadow cannot write to anything that outlives its block. Start at [the shadowing map](../../SHADOWING.md).

| you write | what happens | needs `mut`? | may change the type? |
|---|---|---|---|
| `x = 20;` | writes into the existing place | **yes** | no |
| `let x = 20;` | new variable, same name | no | **yes** |

## If you are coming from another language

**Python.** `x = 10` then `x = 20` is legal and unremarkable, and the closest thing to `let` is nothing at all — assignment declares. Two habits transfer badly. First, Python has no immutable *binding*: `CONSTANT = 3` is a naming convention the interpreter never checks, and `Final[int]` is a hint for a type checker, not for CPython. Rust's default is the enforced version of what `UPPER_CASE` was always trying to say. Second, Python's `x = 20` after `x = "text"` is fine, so the mental model is *a name points at whatever you last put there*; in Rust that line is a type error before it is anything else, and the rebinding you actually want is a second `let`.

The reverse direction is the one that stings: because Python names are freely rebindable, Python programmers reach for reassignment where Rust wants a fresh binding — accumulating into a variable declared far above, rather than computing a value and naming it once. The idiomatic Rust of a `for` loop that builds a total is often not a `mut` at all but an iterator chain, so a Python-shaped Rust function tends to carry `mut`s that a Rust reviewer reads as "something changes here" and then cannot find.

| | Python | Rust |
|---|---|---|
| declare | `x = 10` | `let x = 10;` |
| change the value | `x = 20` | `let mut x = 10; x = 20;` |
| reuse the name for a new type | `x = "text"` | `let x = "text";` (a new variable) |
| a name that must not change | `X = 10` by convention | the default |

**ABAP.** `DATA lv_x TYPE i.` then `lv_x = 10.` is the two-step Rust collapses into one, and everything declared with `DATA` is writable forever — the only immutable thing in the language is `CONSTANTS`, which must be given its value at declaration and can never be assigned again. So the mapping is clean but inverted:

| | ABAP | Rust |
|---|---|---|
| ordinary variable | `DATA lv_x TYPE i.` | `let mut x: i32;` |
| never changes | `CONSTANTS lc_x TYPE i VALUE 10.` | `let x = 10;` — the **default** |
| inferred declaration | `DATA(lv_x) = 10.` | `let x = 10;` |

What changes is which one you have to type. ABAP's default is mutable and immutability costs a different keyword and a compile-time-only value, so `CONSTANTS` is reserved for genuine constants; Rust's default is immutable and *mutability* costs the extra word, so a Rust function routinely has a dozen names that never change and no ceremony announcing it. Reading Rust, the useful reflex is the opposite of the ABAP one: do not scan for what is constant, scan for the three `mut`s.

Worth knowing for the `DATA(lv_x) = 10.` row: ABAP's inline declaration takes its type from the right-hand side of *that statement*, and nothing later can influence it. Rust's inference reads the whole function — see [type inference](../type_inference/README.md), where that difference is the point.

---

## The verified output

<!-- output:variables -->
*Verified output of [`variables.rs`](examples/variables.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. `let` binds a name to a value
   let x: i32 = 10;   x = 10   i32
   let y = 10;        y = 10   i32
   Same program. The annotation is a check, not a requirement —
   nothing here needed it, because 10 has a fallback type.

2. The binding is immutable by default
   Adding a second line does not work:
      let x: i32 = 10;
      x = 20;   // error[E0384]: cannot assign twice to immutable variable
   `x` is not a box holding 10 that you may refill. It is a name for
   a value, and the name is spoken for.

3. `mut` is the permission that assignment needs
   let mut count = 0;   count = 0
   count = 1;           count = 1
   count += 1;          count = 2
   One word, and every later line may write to it.

4. `mut` you did not use is a warning, not a shrug
      let mut total = 0;      // never assigned again
      println!("{total}");
   warning: variable does not need to be mutable
   The compiler holds you to the promise: `mut` says *this changes*,
   so a `mut` nothing writes to is a claim the code does not keep.

5. Mutability belongs to the binding, not to the value
   let owned = String::from("Ada");   // not mut
   let mut moved = owned;             // now it is
   moved.push_str(" Lovelace");       moved = "Ada Lovelace"
   The same bytes were immutable under one name and writable under
   the next. Nothing about the String changed — only the binding.

6. A binding is scoped to its block
   inner: visible only inside these braces
   outer: visible to the end of main
      println!("{inner}");   // error[E0425]: cannot find value `inner`

7. `let` again is a new variable, not an assignment
   let spaces = "   ";        &str
   let spaces = spaces.len();  usize   spaces = 3
   That is shadowing, and it needed no `mut` — because nothing was
   assigned. A second variable took over the name.

The rule
   `let` names a value; `mut` lets a later line write through the name;
   `let` again replaces the name and may change its type.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 15_First_Programs/variables/examples/variables.rs -o /tmp/variables && /tmp/variables
```

## See also

- [Values](../values/README.md) — what you can put on the right of a `let` before defining a type of your own
- [Type inference](../type_inference/README.md) — how the compiler decides what `let x = 10;` holds
- [What a type annotation does](../what_an_annotation_does/README.md) — the `: i32`, and the four shapes where it stops being decoration
- [The shadowing map](../../SHADOWING.md) — `let` twice, and the five questions it raises
- [A name is not a place](../../18_Ownership/a_name_is_not_a_place/README.md) — the proof that `mut` and a shadow are different mechanisms
- [What a warning is asking](../what_a_warning_is_asking/README.md) — `unused_mut` and its neighbours, read properly
- [Variables and mutability ↗](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) · [`E0384` ↗](https://doc.rust-lang.org/error_codes/E0384.html) · [Comprehensive Rust: Variables ↗](https://google.github.io/comprehensive-rust/types-and-values/variables.html)
