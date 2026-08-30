# Lifetime annotations

**Level:** 201 · working knowledge

**One line:** `<'a>` does not make anything live longer — it names a relationship between lifetimes that already exist, so the compiler can refuse the arrangement where one outlives the other.

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let long = String::from("a long string");
    let short = String::from("short");
    println!("{}", longest(&long, &short));   // a long string
}
```

Read the signature as a claim about *sources*, not durations: **the answer is borrowed from `a` or from `b`, and it is only valid while both still are.** Nothing here allocates, extends, or reference-counts. `'a` is a name for a region of code that the compiler works out on its own; you are telling it which regions have to line up.

## The error that asks for one

Write the same function without the annotation and the compiler stops — and its `help:` line is the whole lesson:

```text
error[E0106]: missing lifetime specifier
 --> longest.rs:1:33
  |
1 | fn longest(a: &str, b: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
  |
1 | fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
  |           ++++     ++          ++          ++
```

*The signature does not say whether it is borrowed from `a` or `b`.* That is the missing information, and it is missing from the **signature**, not from the body — the compiler can see the body perfectly well. It refuses to look, because a signature is a contract with every caller and it will not let that contract be inferred from an implementation you might change tomorrow.

## It grants nothing

The single most expensive misreading is that `'a` gives a reference a longer life. Here is the annotated, correct signature above, called badly:

```rust
let long = String::from("a long string");
let result;
{
    let short = String::from("short");
    result = longest(&long, &short);   // `'a` is now the SHORTER of the two
}
// println!("{result}");               // E0597 — `short` is already gone
```

```text
error[E0597]: `short` does not live long enough
  --> longest.rs:10:33
   |
 9 |         let short = String::from("short");
   |             ----- binding `short` declared here
10 |         result = longest(&long, &short);
   |                                 ^^^^^^ borrowed value does not live long enough
11 |     }
   |     - `short` dropped here while still borrowed
12 |     println!("{result}");
   |                ------ borrow later used here
```

`'a` was written, spelled correctly, and accepted — and the code still does not compile. When one name covers two references, the compiler picks the region where **both** are valid, which is the shorter. An annotation is a constraint you have agreed to satisfy; it is never a duration you have been granted.

> **The test that settles it every time.** Ask *what would have to be true for this to be safe?* — then check whether it is. The annotation only ever states that requirement; the borrow checker is what verifies it.

## Most signatures need none

Three elision rules cover the common shapes, and they are why you can write hundreds of functions taking `&str` before meeting `E0106` once:

1. **Every elided lifetime in the parameters gets its own.** `fn f(a: &str, b: &str)` has two, not one.
2. **One input lifetime ⇒ it fills every elided output.** `fn first_word(s: &str) -> &str` needs nothing: there is only one thing the answer could be borrowed from.
3. **`&self` present ⇒ `self`'s lifetime fills every elided output.** A getter returning `&str` from a field needs no annotation, whatever else the method takes.

Rule 2 is why the beginner's instinct — *"references in signatures need lifetimes"* — is wrong, and it is the reason [the "don't use references" scaffold](../how_to_learn_lifetimes/README.md) can say that `&` in a *signature* is the one place references are free. `E0106` fires exactly when the rules run out, which for a function means: **more than one input reference, and an output that borrows.**

## On a struct it becomes part of the type

A field that borrows cannot elide, because there is no signature to elide from:

```text
error[E0106]: missing lifetime specifier
 --> excerpt.rs:2:11
  |
2 |     part: &str,
  |           ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct Excerpt<'a> {
2 ~     part: &'a str,
  |
```

Take the fix and `'a` is now part of the type's name, which is where lifetimes become infectious — the same shape as [a bound written on a struct](../../22_Generics/where_the_bound_goes/README.md), one concept over:

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {          // declared once, then used — as `impl<T> Container<T>` does
    fn new(part: &'a str) -> Self { Excerpt { part } }
    fn shout(&self) -> &str { self.part }   // rule 3: no annotation needed
}
```

The struct costs nothing at run time — `size_of::<Excerpt>()` is 16, exactly the `&str` it holds — and costs an annotation in every signature that names `Excerpt<'a>` from here on. That is the trade `how_to_learn_lifetimes` is warning about when it says *don't put references in struct fields*: not that the type is wrong, but that one field has put a lifetime parameter into everything downstream of it.

## Two lifetimes say more than one

Reach for a second name whenever the answer is borrowed from one argument and not the other:

```rust
fn trimmed_to<'a>(text: &'a str, delimiter: &str) -> &'a str {
    text.split(delimiter).next().unwrap_or(text)
}
```

The answer is cut out of `text`; `delimiter` is only read. Writing both as `'a` compiles, and quietly demands something no caller wants to give:

```text
error[E0716]: temporary value dropped while borrowed
 --> trimmed_to.rs:7:35
  |
7 |     let head = trimmed_to(&kept, &String::from("."));
  |                                   ^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
  |                                   |
  |                                   creates a temporary value which is freed while still in use
8 |     println!("{head}");
  |                ---- borrow later used here
  |
help: consider using a `let` binding to create a longer lived value
```

With two lifetimes that call compiles and the temporary is dropped on schedule. This is the good news buried in the feature: **an annotation is information, and more of it permits more programs.** The instinct to reuse `'a` everywhere is what makes lifetimes feel like an obstacle course, and it is the same instinct as putting `T: Clone` on a struct.

## `'static` is just the longest one

`'static` is not a different kind of thing — it is the region that lasts the whole program, so a `&'static str` is accepted wherever any `&'a str` is wanted and never the reverse. The full treatment, including the trap that `T: 'static` the **bound** means *"contains no borrow that could expire"* rather than *"lives forever"*, is [`&'static str`](../../14_Strings/static_str/README.md).

## If you are coming from another language

- **Python** — there is no counterpart and there is nothing to port, because the garbage collector answers this question at run time by keeping the object alive as long as anything refers to it. The transferable half is the *bug* that ends up on the other side of the wall. `def head(text, delim): return text.split(delim)[0]` returns a new `str`, so nothing can dangle; but `memoryview(buf)` handed back from a function whose `buf` was local is the same shape, and Python's answer is that the `memoryview` keeps `buf` alive. Rust's is that the compiler proves you did not need it to. The cost you are paying for that proof is exactly the annotation — Python spends a reference count at run time, Rust spends `<'a>` at compile time, and neither is free.
- **ABAP** — closer than Python, and worth being exact about. A `DATA ref TYPE REF TO zcl_thing` is a managed reference and ABAP's garbage collector will not free the object while it lives, so the dangling case cannot arise. Where you *have* met this is field symbols: `ASSIGN itab[ 1 ] TO FIELD-SYMBOL(<fs>)` gives a genuine view into somebody else's memory and keeps nothing alive, so `FREE itab` or deleting that row leaves `<fs>` pointing at nothing — a runtime dump, not a syntax error, and the exact arrangement `E0597` refuses at compile time. `<'a>` is the type system's way of writing down "this field symbol may not outlive the table it points into", which ABAP leaves to your discipline.
- **C++** — the same problem, and the version most people have already been bitten by. `std::string_view` returned from a function whose `std::string` was a local is undefined behaviour that usually *works* in testing and fails in production. A lifetime annotation is the machinery that turns that into a compile error, and the price is that you have to say which argument the view came from. C++ has no way to express that in the type, which is why its answer is a coding standard and Rust's is `E0106`.

## The verified output

<!-- output:lifetime_annotations -->
*Verified output of [`lifetime_annotations.rs`](examples/lifetime_annotations.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. `'a` ties the answer to both arguments
   longest(&long, &short) = a long string
   `'a` is the SHORTER of the two lifetimes, not a duration the
   annotation hands out — move `short` into an inner scope and
   the identical signature gives E0597.

2. Most signatures need no annotation — elision fills the hole
   first_word("Call me Ishmael") = Call
   first_word(&novel)            = Call
   One input reference, one output: there is only one source it
   could borrow from, so the compiler writes `'a` for you.

3. On a struct the parameter becomes part of the type
   excerpt.part  = Call me Ishmael
   excerpt.shout() = Call me Ishmael
   size_of::<Excerpt>() = 16   the same as the &str it holds (16)
   `impl<'a> Excerpt<'a>` declares it and then uses it — the same
   say-it-twice shape as `impl<T> Container<T>`.

4. Two lifetimes say more than one
   trimmed_to(&kept, &temporary) = the part we keep
   The delimiter was a temporary, freed at the end of that very
   statement, and the answer outlives it. Write both parameters as
   `'a` and the same line is E0716: the temporary would have to
   live as long as the borrow it has nothing to do with.

5. `'static` is not a different kind of thing, just the longest
   longest(&long, literal) = lives as long as the program
   A `&'static str` is accepted wherever `&'a str` is wanted,
   because 'static outlives every 'a. The reverse never holds.
```
<!-- /output -->

## See also

- [Borrowing](../borrowing/README.md) — the `&` these annotations are about, and the rule that lets one writer or many readers
- [How to learn lifetimes](../how_to_learn_lifetimes/README.md) — the "clone everything" scaffold, and this is the page for when it comes down
- [`&'static str`](../../14_Strings/static_str/README.md) — the longest lifetime, and why `T: 'static` is not what it sounds like
- [Scope is about names](../scope_is_about_names/README.md) — the three things "out of scope" is asked to mean, one of which is the borrow region `'a` names
- [Where the bound goes](../../22_Generics/where_the_bound_goes/README.md) — the same infectiousness one concept over: a constraint on the struct that every later `impl` repeats
- [`Cow`](../clone_on_write/README.md) — the type that refuses to choose between owning and borrowing, and the annotation it needs to do it
