# What a type annotation does

**Level:** 101 → 201 · for newcomers

**One line:** `let s = "a";` and `let s: &str = "a";` are the same program — but an annotation is not a comment. It is an *input* to inference, and on four other expression shapes it decides what the program is.

```rust
let s = "a";           // &'static str
let s: &str = "a";     // &'static str — identical type, identical machine code
```

A string literal has exactly one possible type, so there is nothing left for the annotation to decide. That is a fact about *literals*, not about annotations:

| you write | inferred | annotated | the annotation… |
|---|---|---|---|
| `"a"` | `&'static str` | `&str` | changes nothing |
| `1` | `i32` | `u8`, `u64`, `f64`… | **picks** the type |
| `&some_string` | `&String` | `&str` | **coerces** |
| `"42".parse()` | *nothing — `E0284`* | `i32` | is **required** |

---

## A literal decides its own type

```rust
let a = "a";
let b: &str = "a";
let c: &'static str = "a";   // the fully explicit spelling
```

All three are `&'static str`. The elided lifetime in `b` is a hole the compiler fills, and a literal only ever fills it with `'static` — worked through in [`&'static str`](../../14_Strings/static_str/README.md), which is also where the annotation starts *refusing* things once the text is not a literal.

So on `"a"`, write whichever reads better. The annotation is documentation.

## A numeric literal has a fallback, and the annotation overrides it

```rust
let n = 1;         // i32 — the fallback
let m: u8 = 1;     // u8,  one byte
let f: f64 = 1.0;  // f64, eight bytes
```

`i32` is not what `1` *means*; it is what Rust settles on when nothing else in the function decides. Pass the same binding to something expecting a `u64` and it becomes a `u64` with no annotation at all. The annotation is simply the loudest of the deciders.

That it is load-bearing shows up the moment the value does not fit:

```rust
// let big: u8 = 1_000_000;
```

```text
error: literal out of range for `u8`
 --> e1.rs:2:19
  |
2 |     let big: u8 = 1_000_000;
  |                   ^^^^^^^^^
  |
  = note: the literal `1_000_000` does not fit into the type `u8` whose range is `0..=255`
  = note: `#[deny(overflowing_literals)]` on by default
```

Widths, and the three bills each one comes with, are [Meet the byte](../../19_Numbers/meet_the_byte/README.md).

## A borrow: the annotation is a coercion site

```rust
let owned = String::from("hi");
let r = &owned;           // &String
let s: &str = &owned;     // &str — deref coercion, requested by the annotation
```

Same expression on the right, two different types on the left, and it is worth going slowly here because three separate things are happening.

**One.** `&owned` produces a `&String` and nothing else. Taking a reference to a `String` gives you a reference to a `String`; the expression has exactly one type, the same way `"a"` did.

**Two.** `&String` and `&str` are genuinely different types, not two names for one. A `String` is three words on the stack — pointer, length, capacity — with the bytes on the heap, so `&String` is a pointer *to that struct*. A `&str` is the pointer and the length **themselves**, travelling as a pair, with no capacity and no struct to point at. Ordinarily, handing one where the other is expected is `E0308`.

**Three.** It compiles anyway, because `String` carries an `impl Deref<Target = str>` — a trait with exactly one method, `fn deref(&self) -> &str`. That impl is the permission slip: it tells the compiler there is a canonical way to view a `String` as a `str`, so when a `&String` turns up where a `&str` is wanted, it may quietly insert the call rather than reject the program. That insertion is what "[deref coercion](../../14_Strings/six_kinds_of_string/README.md)" names.

So these four lines are the same line, written at four levels of explicitness:

```rust
let s: &str = &owned;                // the coercion — the compiler inserts the rest
let s: &str = &*owned;               // what it desugars to: deref, then re-borrow
let s: &str = Deref::deref(&owned);  // what that `*` calls
let s: &str = owned.as_str();        // the same thing, spelled as a method
```

All four hand back the **same pointer** — no allocation, no copy, not a single byte of `"hi"` moved. The only thing that changes is how much of the `String` you are still holding: 24 bytes on the stack become 16, and the capacity field is simply forgotten. Which is why the `&str` cannot grow the text and the `String` still can.

The word doing the work in "coercion" is *implicit*. Rust performs one only at a **coercion site** — a `let` with a stated type, a function argument, a struct literal field, a return position — and nowhere else. That is the whole reason the annotation is what triggers it: without `: &str` on the left there is no site, nothing to coerce *toward*, and you keep the `&String` you asked for.

A function argument is a coercion site too, which is why this is easy to overstate:

```rust
fn shout(s: &str) -> String { s.to_uppercase() }

shout(r);   // compiles — r is a &String, and the parameter type coerces it
```

Where the annotation actually earns its keep is anywhere no parameter is offering to do the work — inside a `Vec`, a tuple, a struct field, a return type:

```rust
let v = vec![&owned];              // Vec<&String>
let w: Vec<&str> = vec![&owned];   // Vec<&str>
```

Those two are not interchangeable, and the error says so plainly:

```text
error[E0308]: mismatched types
 --> errs.rs:6:24
  |
6 |     let w: Vec<&str> = v;
  |            ---------   ^ expected `Vec<&str>`, found `Vec<&String>`
  |            |
  |            expected due to this
  |
  = note: expected struct `Vec<&str>`
             found struct `Vec<&String>`
```

## Some expressions have no type without one

```rust
let parsed: i32 = "42".parse().unwrap();
let wide:   i64 = "42".parse().unwrap();
```

`parse` is generic over what it produces, so without a target there is no answer to compute:

```text
error[E0284]: type annotations needed
 --> e2.rs:2:9
  |
2 |     let x = "42".parse().unwrap();
  |         ^        ----- type must be known at this point
  |
help: consider giving `x` an explicit type
  |
2 |     let x: /* Type */ = "42".parse().unwrap();
  |          ++++++++++++
```

`collect` is the same shape, and shows the point at its sharpest — one expression, three programs:

```rust
let letters = ['R', 'u', 's', 't', 'a', 'c', 'e', 'a', 'n'];

let joined: String        = letters.iter().collect();          // "Rustacean"
let listed: Vec<char>     = letters.iter().copied().collect();  // 9 items
let unique: BTreeSet<char> = letters.iter().copied().collect(); // 8 items — 'a' twice
```

The alternative spelling puts the same information at the call instead of on the binding — `"42".parse::<i32>()`, `letters.iter().collect::<String>()`. That is the [turbofish](../../14_Strings/making_a_string/README.md), and it is the right tool when the value is not being bound to a name.

## So when do you write one?

**When the expression is ambiguous** — a numeric literal, `parse`, `collect`, `into`, `default` — or **when you want a coercion**. On `"a"` it is neither.

Beyond that it is taste, and the useful rule is that an annotation on a `let` is a *check* as much as a label: it fails the build if the expression ever stops producing what you thought. On a long function, or on a value that came back from something generic, that is worth the six characters. On `let s = "a";` it is not.

## If you are coming from another language

**Python.** `s: str = "a"` looks like the same edit, and it is not the same kind of thing at all. Python's annotation is inert — stored in `__annotations__`, read by mypy, and ignored entirely by the interpreter, which will happily let `s` hold an `int` a line later. Rust's participates in compilation: `let m: u8 = 1;` produces a genuinely different value, in a genuinely different amount of memory, and no Python annotation can do that.

The place the two languages *do* line up is more interesting, because Python solves the same problem with a different mechanism. Where Rust names the type and lets the compiler pick the function, Python names the function and lets the type follow:

| the question | Python | Rust |
|---|---|---|
| text → number | `int(s)` · `float(s)` | `let x: i32 = s.parse()?` · `let x: f64 = s.parse()?` |
| iterable → container | `list(g)` · `set(g)` · `"".join(g)` | `let v: Vec<_> = g.collect()` · `HashSet<_>` · `String` |
| pick it at the call | *(there is only the call)* | `s.parse::<i32>()` — the turbofish |

Three functions become one function plus three annotations. That is why `E0284` exists and Python has no equivalent error: the moment the type is the thing selecting the code, leaving it out leaves nothing to run.

What changes: in Python, forgetting to say `int` gets you a string that behaves like a string until `"42" + 1` blows up somewhere else. In Rust, forgetting to say `i32` does not compile, and the error points at the `let`.

**ABAP.** Inline declaration is inference and a `DATA` statement is the annotation, so the shape is familiar:

```abap
DATA(lv_x) = 'abc'.        " inferred
DATA lv_y TYPE string.     " annotated
lv_y = 'abc'.
```

But ABAP has the trap Rust does not, and it is worth knowing precisely because this page's answer for Rust is "no difference." In ABAP there *is* a difference, and it hides in the quotes:

| you write | you get |
|---|---|
| `DATA(lv) = 'abc'.` | `c LENGTH 3` — a fixed-width character field, trailing blanks truncated |
| ``DATA(lv) = `abc`.`` | `string` — variable length, blanks preserved |
| `DATA(lv) = 1.` | `i` |

So the ABAP habit of writing `DATA lv TYPE string.` explicitly is not redundancy, the way `let s: &str = "a";` is — it is defending against a literal whose type depends on which quote character you typed. Rust has one string literal syntax and one type for it, so the annotation has nothing to defend against.

The closest ABAP counterpart to *target-typed* inference is the `#` in the constructor expressions:

```abap
DATA(lt_a) = VALUE ty_tab( ( ... ) ).   " type stated
lt_b = VALUE #( ( ... ) ).              " # = take it from the target
```

`#` says "derive this from the context", which is exactly the job a Rust annotation does for `collect()` and `parse()`. What changes is reach: ABAP's `#` looks at the single operand position it sits in, while Rust solves the whole expression at once, so an annotation on the `let` reaches back through several chained calls to decide what `parse` returns.

---

## The verified output

<!-- output:what_an_annotation_does -->
*Verified output of [`what_an_annotation_does.rs`](examples/what_an_annotation_does.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A string literal — the annotation changes nothing
   let a = "a";                 &str
   let b: &str = "a";           &str
   let c: &'static str = "a";   &str
   same value in all three? true
   A literal has exactly one possible type, so there is nothing for the
   annotation to decide. (type_name erases lifetimes; all three are
   &'static str — the lifetime half is the static_str lesson.)

2. An integer literal — the annotation picks the type
   let n = 1;         i32  size 4   <- the i32 fallback
   let m: u8 = 1;     u8   size 1
   let f: f64 = 1.0;  f64  size 8
   One literal, three types. i32 is what Rust falls back to when nothing
   else decides — and an annotation is one of the things that can decide.
   Load-bearing, not decorative:
      let big: u8 = 1_000_000;   // error: literal out of range for `u8`

3. A borrow — the annotation performs a coercion
   let r = &owned;        &alloc::string::String
   let s: &str = &owned;  &str   <- deref coercion, asked for by the annotation
   The expression `&owned` only ever produces a &String. The annotation
   asks for a &str, which is a different type — and String's
   `impl Deref<Target = str>` is the permission slip that lets the
   compiler bridge them by inserting a call. Four spellings, one result:
      let s: &str = &owned;               the coercion (compiler-inserted)
      let s: &str = &*owned;              what it desugars to
      let s: &str = Deref::deref(&owned); what the `*` calls
      let s: &str = owned.as_str();       the same thing, written out
   same pointer in all four? true
   String is 24 bytes on the stack (ptr, len, cap); &str is 16 (ptr, len).
   The coercion copies no text and allocates nothing — it forgets the
   capacity field and keeps pointing at the same bytes.
   vec![&owned]                     alloc::vec::Vec<&alloc::string::String>
   let _: Vec<&str> = vec![&owned]; alloc::vec::Vec<&str>
   A call site coerces too — shout(r) compiles and gives "HI" — so
   the annotation only earns its keep where nothing else offers to coerce:
   inside a Vec, a tuple, a struct field, a return type.

4. No answer without one — the annotation drives inference
   let parsed: i32 = "42".parse().unwrap();  i32  42
   let wide:   i64 = "42".parse().unwrap();  i64  42
      let x = "42".parse().unwrap();   // error[E0284]: type annotations needed
   One expression, three results — only the annotation differs:
      let _: String        = letters.iter().collect();   "Rustacean"
      let _: Vec<char>     = ...collect();               9 items
      let _: BTreeSet<char> = ...collect();              8 items, {'R', 'a', 'c', 'e', 'n', 's', 't', 'u'}

The rule
   Annotate when the expression is ambiguous (a numeric literal, parse,
   collect, into) or when you want a coercion. On "a" it is neither, so
   `let s = "a";` and `let s: &str = "a";` are the same program.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 15_First_Programs/what_an_annotation_does/examples/what_an_annotation_does.rs -o /tmp/waad && /tmp/waad
```

## See also

- [`&'static str`](../../14_Strings/static_str/README.md) — the lifetime half of `let s: &str = "a";`, and where the annotation starts refusing things
- [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) — what the coercion in section three is coercing between
- [Meet the byte](../../19_Numbers/meet_the_byte/README.md) — why `u8` instead of `i32` is a decision and not a detail
- [A block is an expression](../a_block_is_an_expression/README.md) — the other place `E0308` catches a beginner, over one semicolon
- [Shadowing](../../SHADOWING.md) — `let` again with a new annotation is legal, and changes the type
- [Type inference ↗](https://doc.rust-lang.org/reference/type-inference.html) · [`E0282` ↗](https://doc.rust-lang.org/error_codes/E0282.html) · [Deref coercion ↗](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods)
