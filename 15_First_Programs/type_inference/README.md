# Type inference

**Level:** 101 → 201 · for newcomers

**One line:** An unannotated `let` does not hold "some type to be decided at runtime" — it holds one type, worked out from **how you use the value later in the function**, and compiled to exactly the same machine code as if you had written the type out.

```rust
fn takes_u32(x: u32) -> u32 { x }
fn takes_i8(y: i8) -> i8 { y }

let x = 10;
let y = 20;
takes_u32(x);   // x is u32
takes_i8(y);    // y is i8
```

Nothing on the two `let` lines differs. Delete the two calls and both become `i32`. The type came from the third and fourth lines — inference reads **forward**, which is the part that surprises people arriving from a language where a declaration is decided where it stands.

## It is not "any type"

The most expensive misreading of `let x = 10;` is that it is Python's `x = 10` — a name that will hold a tagged value the runtime inspects. It is not, and the difference is measurable rather than philosophical:

```rust
let x = 10;      // then used as a u32 -> 4 bytes
let y = 20;      // then used as an i8  -> 1 byte
```

Two identical-looking lines, two different amounts of memory, both fixed at compile time. A dynamically typed value carries a tag and costs the same whatever it holds.

The stronger version of the claim is checkable, so here it is checked. Two programs, one with `let x = 10;` and one with `let x: u32 = 10;`, compiled with the crate name and metadata pinned so that symbol mangling — which hashes the crate name and path — cannot differ for reasons that are not about the code:

```bash
rustc --edition 2024 --crate-name probe -C metadata=fixed -o a.bin inferred.rs
rustc --edition 2024 --crate-name probe -C metadata=fixed -o b.bin annotated.rs
cmp a.bin b.bin        # no output: byte-for-byte identical
```

The executables are identical. In the LLVM IR the two differ by exactly one line — a metadata node holding a hash of the source file — and in nothing else. The annotation is an *input to the compiler*, not an instruction that survives it.

## Inference reaches through a whole chain

```rust
let mut names = Vec::new();   // no element type yet
names.push("Ada");            // ...decided here
```

`Vec::new()` has nothing in it to inspect. The `let` line is not where the type is known; it is where the question is asked, and the answer can arrive several statements later. The same mechanism is what makes `collect()` and `parse()` work at all — see [what a type annotation does](../what_an_annotation_does/README.md), which is this page from the other side: what to do when you want to *drive* the decision rather than let it happen.

## The two fallbacks

When nothing in the function decides, Rust does not guess from the value:

```rust
let n = 1;     // i32, not u8 — "small" is not a reason
let f = 1.0;   // f64
```

Before it settles, an undecided number shows up in error messages under a placeholder name:

```rust
// let x = 3.14;
// let y = 20;
// assert_eq!(x, y);
```

```text title="Abridged — real rustc output for float_vs_integer.rs"
error[E0277]: can't compare `{float}` with `{integer}`
 --> float_vs_integer.rs:4:5
  |
4 |     assert_eq!(x, y);
  |     ^^^^^^^^^^^^^^^^ no implementation for `{float} == {integer}`
  |
  = help: the trait `PartialEq<{integer}>` is not implemented for `{float}`
```

`{float}` and `{integer}` are not types you can write. They are the compiler naming a variable it has not solved yet, and this line fails because no fallback ever makes two *different* fallbacks equal — `x` would settle on `f64` and `y` on `i32`, and there is no `PartialEq<i32> for f64`. (The program produces two errors, not one: the `E0277` above and an `E0308` for the same line.)

## Where inference stops: signatures

```rust
// fn double(n) { n * 2 }        // not Rust
fn double(n: i32) -> i32 { n * 2 }
```

Every parameter and every return type is written out, always. Inside a body the compiler solves; at a boundary you declare. That is a deliberate line and not an omission — it is what stops an edit inside one function from silently changing the type another function is required to pass it, and it is why a Rust compile error usually points at the line you changed rather than three modules away.

## When there is genuinely nothing to go on

```rust
// let v = Vec::new();       // error[E0282]: type annotations needed
// let x = "42".parse();     // error[E0284]
```

Two ways to answer, and they are the same answer written in two places:

```rust
let parsed: i32 = "42".parse().unwrap();   // on the binding
let turbo = "42".parse::<i32>().unwrap();  // on the call — the turbofish
```

Which to prefer is taste until the value is being returned or stored, at which point the annotation on the binding usually reads better. [When the compiler cannot infer](../../22_Generics/when_the_compiler_cannot_infer/README.md) works through the generic version of this, where the missing type is a parameter rather than a number.

## If you are coming from another language

**Python.** `x = 10` and `let x = 10;` look identical and are opposites in the one way that matters: Python decides at runtime, per value, and can decide differently on the next line; Rust decides once, at compile time, and the decision is a property of the program rather than of the execution. A type checker narrows the gap but does not close it — mypy *infers* `x: int` from `x = 10`, and mypy is not the interpreter, so `x = "text"` two lines later is an error in your editor and a working program at runtime.

The place the analogy genuinely helps is generics: `def f(xs)` accepting any iterable is not far from `fn f<T>(xs: &[T])`, and both defer the decision to the caller. What changes is when the decision is checked, and therefore who finds the mistake.

| | Python | Rust |
|---|---|---|
| `x = 10` | a name bound to an `int` object; rebindable to anything | one type, fixed at compile time |
| annotation | `x: int = 10`, inert at runtime | participates in compilation |
| function signature | optional, and unchecked without a tool | mandatory, and the compiler's contract |
| "what type is this?" | `type(x)` at runtime | it has no runtime existence to ask about |

**ABAP.** Inline declaration is the closest counterpart, and its limit is the useful lesson:

```abap
DATA(lv_x) = 10.          " lv_x is i — decided by the right-hand side, here
DATA(lv_y) = '10'.        " lv_y is c LENGTH 2 — the quotes decide, not the usage
```

ABAP infers from the **right-hand side of that one statement**. Nothing later in the method can influence it, and nothing needs to: the value is fully determined where it stands. Rust solves the whole function body at once, so `let x = 10;` genuinely has no answer until the compiler has read what you do with `x` — which is why the Rust error for an under-determined type points at the `let` and talks about something that happens twenty lines down.

The other half of the bridge is `FIELD-SYMBOLS` and generic types: `DATA lv TYPE REF TO data` plus `ASSIGN` is ABAP's *runtime* polymorphism, checked when the statement executes, and its Rust counterpart (`dyn Trait`) is a deliberate opt-in with a pointer's worth of cost — see [static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md). Inference is the opposite thing: it costs nothing at runtime because there is nothing left of it at runtime.

---

## The verified output

<!-- output:type_inference -->
*Verified output of [`type_inference.rs`](examples/type_inference.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The same literal, two types — decided by a LATER line
   let x = 10;   ...then takes_u32(x)   x is u32
   let y = 20;   ...then takes_i8(y)    y is i8
   Nothing on the `let` lines differs. Delete the two calls and both
   become i32. The type came from how the value is used.

2. This is not "any type"
   x occupies 4 byte(s), y occupies 1
   A dynamically typed value carries a tag at runtime and costs the
   same whatever it holds. These are 4 bytes and 1 byte, fixed at
   compile time — the identical machine code the annotated spelling
   would have produced.

3. Inference reaches forward through a whole chain
   let mut names = Vec::new();   // no element type yet
   names.push("Ada");            // ...decided here
   names is alloc::vec::Vec<&str>
   The `let` line is not where the type is known; it is only where
   the question is asked.

4. The two fallbacks, for when nothing decides
   let n = 1;     -> i32
   let f = 1.0;   -> f64
   Rust never picks a *width* for you by guessing at the value: 1
   does not become u8 because it is small. It becomes i32 because
   that is the written-down fallback.

5. Before it settles, the type has a placeholder name
      let x = 3.14;
      let y = 20;
      assert_eq!(x, y);
   error[E0277]: can't compare `{float}` with `{integer}`
   {float} and {integer} are not types you can write. They are the
   compiler saying "a number whose width is still undecided" — and
   the reason this line fails is that no fallback ever makes two
   different fallbacks equal.

6. Where inference stops: a signature is never inferred
      fn double(n) { n * 2 }       // not Rust
      fn double(n: i32) -> i32     // every parameter, every return
   Inside a body the compiler solves; at a boundary you declare. That
   is why changing a function body cannot silently change its callers.

7. When there is genuinely nothing to go on
      let v = Vec::new();          // error[E0282]: type annotations needed
      let x = "42".parse();        // error[E0284]
   Two ways to answer, and they are the same answer written twice:
      let parsed: i32 = "42".parse().unwrap();   42
      let turbo = "42".parse::<i32>().unwrap();  42
   equal? true

The rule
   An unannotated `let` is not an unknown type — it is a type the
   compiler works out from everything you do with the value, falling
   back to i32 or f64 only when nothing in the function decides.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 15_First_Programs/type_inference/examples/type_inference.rs -o /tmp/ti && /tmp/ti
```

## See also

- [Variables](../variables/README.md) — the `let` this page is about the right-hand side of
- [Values](../values/README.md) — where `i32` and `f64` sit among the built-in types, and how wide each one is
- [What a type annotation does](../what_an_annotation_does/README.md) — the same subject from the driver's seat: the four shapes where the annotation decides the program
- [When the compiler cannot infer](../../22_Generics/when_the_compiler_cannot_infer/README.md) — `E0282` and `E0283` once generics are involved
- [`str::parse`](../../14_Strings/str_methods/str_parse/README.md) — the method that has no return type until you supply one
- [Type inference ↗](https://doc.rust-lang.org/reference/type-inference.html) · [`E0282` ↗](https://doc.rust-lang.org/error_codes/E0282.html) · [Comprehensive Rust: Type Inference ↗](https://google.github.io/comprehensive-rust/types-and-values/inference.html)
