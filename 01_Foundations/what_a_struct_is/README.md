# What a struct is

**Level:** 101 → 201 · for newcomers

**One line:** A struct names a group of values and makes that name a *type* — and the behaviour does not live inside it, which is the part that catches everyone arriving from an object-oriented language.

Twenty pages in this library already use structs. None of them says what one is. This is that page.

```rust
struct Ballot {
    voter: String,
    scores: Vec<u8>,
}
```

Three things are true of that, and the third is the surprising one:

1. It groups values that belong together, and **names each one** — so you do not depend on position the way a bare tuple `(String, Vec<u8>)` does.
2. It creates a **new type**. Not a nickname for the pair — a type the compiler will keep apart from every other type, including an identical one under a different name.
3. It contains **no behaviour whatsoever.** There is no method in that block and there cannot be. Methods live in a separate `impl` block, and shared behaviour lives in traits.

---

## Data here, behaviour there

```rust
struct Ballot { … }        // the data

impl Ballot {              // the behaviour
    fn new(voter: &str) -> Self { … }   // associated function — no self
    fn total(&self) -> u32 { … }        // method — takes self
}
```

That split is not a stylistic quirk; it is why Rust has no classes and no inheritance. A struct is a **record**, an `impl` block attaches functions to it, and a `trait` describes behaviour that many types can share. Anything you would reach for inheritance to do, you do with traits instead.

The only difference between an **associated function** and a **method** is whether the first parameter is `self`:

| | signature | called as |
|---|---|---|
| associated function | `fn new(voter: &str) -> Self` | `Ballot::new("Ada")` |
| method | `fn total(&self) -> u32` | `ballot.total()` |

`ballot.total()` is *sugar* for `Ballot::total(&ballot)` — the example below proves it by asserting the two are equal. Rust has no special constructor syntax either: `new` is an ordinary associated function that happens to return `Self`, and the name is a convention, nothing more.

## Three flavors — and a fourth spelling

| Flavor | Written | Fields reached by |
|---|---|---|
| named-field (the classic C struct) | `struct Ballot { voter: String }` | `.voter` |
| tuple struct | `struct Precinct(u32);` | `.0` |
| unit struct | `struct Sealed;` | there are none |

A tuple struct really is a named-field struct whose field names happen to be numbers — `Precinct { 0: 7 }` compiles and is the same value as `Precinct(7)`.

The **fourth spelling** is the one that produces a confusing error. `struct AlsoEmpty {}` has no fields either, but it is not a unit struct: it must be constructed with braces. Use the bare name and you get an error that reads as though the type does not exist:

```text title="Real rustc output"
error[E0423]: expected value, found struct `AlsoEmpty`
   |
   |     println!("{:?}", AlsoEmpty);
   |                      ^^^^^^^^^ help: use struct literal syntax instead: `AlsoEmpty {}`
```

A unit struct is not a consolation prize. It holds nothing, so behaviour is the *only* thing it can hold — which makes it exactly the right type when you need something to `impl` a trait on and have no data to carry.

## What things are called

`UpperCamelCase` for the type, `snake_case` for fields and methods. This is not
taste — `rustc` has a lint for each (`non_camel_case_types`, `non_snake_case`) and
will warn on the way past. The struct's name should say what the group *means*
(`Ballot`, not `Data`), because that name is the type every downstream signature
will read.

## Privacy is per module, not per struct

Fields are **private by default**, and "private" means *"visible inside the module that defines the struct"* — not *"visible only to its own methods"*. Any code in the same module can read any field.

There is one consequence people meet as a puzzle rather than as a rule: for a **tuple struct**, a private field makes the *constructor* private too.

```text
error[E0603]: tuple struct constructor `Ballot` is private
   |  a constructor is private if any of the fields is private
```

That is not an obstacle, it is the mechanism — it is precisely what lets [the newtype](../newtype_score/README.md) guarantee that every value of a type went through one checked door.

## What the compiler will *not* let you do

- **Mark one field mutable.** `mut` belongs to the binding, not the field: the whole instance is mutable or none of it is.
- **Put a `&str` in a field without a lifetime.** `struct User { name: &str }` is `error[E0106]: missing lifetime specifier` — the struct must promise it will not outlive what it borrows. Owned `String` sidesteps the question, which is why beginner code uses it.
- **Print it with `{}`.** A struct has no `Display` unless you write one; `#[derive(Debug)]` and `{:?}` are the way in. See [Debug and Display](../debug_vs_display/README.md).

## Coming from another language

- **Python.** The closest thing is a `@dataclass` or a `NamedTuple` — a named record with typed fields. Two differences bite. Rust's is checked at compile time, so a misspelled field is an error rather than an `AttributeError` at 3am. And you **cannot bolt a new attribute on at runtime**: `rect.trash = "…"` has no equivalent, because the set of fields is part of the type.
- **ABAP.** A named-field struct is essentially a `TYPES: BEGIN OF … END OF` structure, and that analogy is very close. What has no ABAP counterpart is the `impl` block: there is no way to attach methods to a structure type, so behaviour ends up in a class or a function module *beside* the data. Rust splits them the same way — the difference is that `impl` keeps the association in the type system.
- **Not objects.** Rust has no classes and no inheritance. A struct with methods may get called an object in prose, but nothing is inherited and nothing is virtual unless you ask for it with `dyn`.

---

## Practice

**Three flavors, and the two things the compiler keeps apart.** Define `Color(i32, i32, i32)` and `Point(i32, i32, i32)` — identical shapes, different names — write a function taking one of them, pass it the other, and read `E0308`. Then note what a bare `(i32, i32, i32)` would have done with the same call, because that difference is the entire argument for tuple structs.

Next, put a tuple struct in a module with `pub` on the struct but **not** on its field, and try to construct it from outside. The error is `E0603`, and the sentence to read twice is *"a constructor is private if any of the fields is private."* Work out why that follows rather than being an extra rule.

Finish with a unit struct, and give it a `Display` impl — a type with no data whose entire reason to exist is the behaviour hung on it.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_a_struct_is_kata -->
*[`what_a_struct_is_kata.rs`](examples/what_a_struct_is_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three flavors, and the two things the compiler keeps apart.
//!
//!   rustc --edition 2024 what_a_struct_is_kata.rs -o /tmp/wasik && /tmp/wasik

use std::fmt;

// Same three fields, same types, two different TYPES. This is the whole
// point of a tuple struct over a bare tuple: the name is load-bearing.
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn paint(c: &Color) -> String {
    format!("#{:02x}{:02x}{:02x}", c.0, c.1, c.2)
}

mod sealed {
    // `pub` on the STRUCT is not `pub` on the FIELD, and for a tuple struct
    // that also makes the CONSTRUCTOR private — you cannot write Ballot(9)
    // from outside this module, because doing so would set a private field.
    #[derive(Debug)]
    pub struct Ballot(u32);

    impl Ballot {
        pub fn new(n: u32) -> Self {
            Ballot(n)
        }
        pub fn get(&self) -> u32 {
            self.0
        }
    }
}

// A unit struct carries no data, so the only thing it can be is a place to
// hang behaviour. That is not a consolation prize — it is the use case.
struct Blank;

impl fmt::Display for Blank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(no ballot returned)")
    }
}

fn main() {
    println!("1. Identical shapes, different types");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("   Color(0,0,0) -> {}", paint(&black));
    println!("   {origin:?} is the same THREE i32s and paint(&origin) will not compile:");
    println!("     error[E0308]: mismatched types — expected `&Color`, found `&Point`");
    println!("   A bare (i32, i32, i32) would have accepted either. The name is the guard.");

    println!("\n2. Destructuring — the tuple struct comes apart like a tuple");
    let Color(r, g, b) = black;
    println!("   let Color(r, g, b) = black;  ->  r={r} g={g} b={b}");
    let Point(x, y, z) = origin;
    println!("   let Point(x, y, z) = origin; ->  x={x} y={y} z={z}");
    println!("   Same shape, same destructuring, and still not interchangeable.");

    println!("\n3. A private field makes a tuple struct's CONSTRUCTOR private");
    let ballot = sealed::Ballot::new(431);
    println!("   sealed::Ballot::new(431) -> {ballot:?}, get() = {}", ballot.get());
    println!("   sealed::Ballot(431) from out here is E0603:");
    println!("     `a constructor is private if any of the fields is private`");
    println!("   That is the newtype's whole guarantee: one door, and the module owns it.");

    println!("\n4. A unit struct holds nothing, so behaviour is all it can hold");
    println!("   {Blank}   <- via its Display impl; the value carries no data at all");
}
```
<!-- /source -->

<!-- output:what_a_struct_is_kata -->
*Verified output of [`what_a_struct_is_kata.rs`](examples/what_a_struct_is_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Identical shapes, different types
   Color(0,0,0) -> #000000
   Point(0, 0, 0) is the same THREE i32s and paint(&origin) will not compile:
     error[E0308]: mismatched types — expected `&Color`, found `&Point`
   A bare (i32, i32, i32) would have accepted either. The name is the guard.

2. Destructuring — the tuple struct comes apart like a tuple
   let Color(r, g, b) = black;  ->  r=0 g=0 b=0
   let Point(x, y, z) = origin; ->  x=0 y=0 z=0
   Same shape, same destructuring, and still not interchangeable.

3. A private field makes a tuple struct's CONSTRUCTOR private
   sealed::Ballot::new(431) -> Ballot(431), get() = 431
   sealed::Ballot(431) from out here is E0603:
     `a constructor is private if any of the fields is private`
   That is the newtype's whole guarantee: one door, and the module owns it.

4. A unit struct holds nothing, so behaviour is all it can hold
   (no ballot returned)   <- via its Display impl; the value carries no data at all
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:what_a_struct_is -->
*Verified output of [`what_a_struct_is.rs`](examples/what_a_struct_is.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three flavors, and a fourth spelling
   named   Ballot { voter: "Ada", scores: [] }
   tuple   Precinct(7)   field reached by index: 7
   unit    Sealed          — the type has exactly one value
   braces  AlsoEmpty      — `AlsoEmpty {}`, NOT the same as a unit struct

2. A tuple struct is a named-field struct whose fields are numbers
   Precinct { 0: 7 } == Precinct(7)  ->  true

3. Field ORDER in the expression is free; the value is the same
   Ballot { voter: "Ben", scores: [5, 2] }
   Ballot { voter: "Ben", scores: [5, 2] }
   same value: true

4. Data in the struct, behaviour in the impl block
   Cara scored [5, 2, 0], total 7
   Ballot::total(&ballot) == ballot.total()  ->  true

5. Privacy is per module — `count` is private, and that is why
   the module has to hand you a method to read it
   id (pub)        12
   count (private) 431   <- via count(), not s.count
   s.count would be E0616: field `count` of struct `Sealed` is private
```
<!-- /output -->

## See also

- [STRUCTS.md](../../STRUCTS.md) — the map: every struct lesson in the library, in reading order
- [A score is not a number: the newtype](../newtype_score/README.md) — the tuple struct with a job
- [`Option` fields](../option_fields/README.md) — the field that may legitimately be missing
- [Debug and Display](../debug_vs_display/README.md) — why `{}` refuses your struct
- [Ownership and moves](../ownership_and_moves/README.md) — what happens when a struct owns its fields
