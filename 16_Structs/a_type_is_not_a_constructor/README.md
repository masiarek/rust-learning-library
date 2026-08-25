# A type is not a constructor

**Level:** 101 → 201 · for newcomers

**One line:** Rust has no `Type()` call that makes a value — `Ballot { .. }` is a literal, `Ballot::new()` is a function somebody wrote, and `let b: Ballot;` names a *type* in the slot where the value was supposed to go.

```rust
struct Ballot { voter: String, score: u8 }
struct Precinct(u32);
#[derive(Debug)]
struct Sealed;

fn main() {
    let named = Ballot { voter: String::from("Ada"), score: 5 };
    let tuple = Precinct(7);
    let unit = Sealed;
    println!("{} {} {:?}", named.voter, tuple.0, unit);  // Ada 7 Sealed
}
```

Three flavors, three spellings, and **only the middle one is a call**.

---

## Where a value comes from

| Flavor | Declared | Built with | Is the bare name a value? |
|---|---|---|---|
| named-field | `struct Ballot { .. }` | `Ballot { voter, score }` — a literal | no |
| tuple struct | `struct Precinct(u32);` | `Precinct(7)` — a real function call | **yes**, a `fn(u32) -> Precinct` |
| unit struct | `struct Sealed;` | `Sealed` — the value itself | **yes**, and it takes no arguments |

Only the tuple struct gets a function out of its declaration. That is why "constructor" is literal for one flavor and a figure of speech for the other two, and why the same fact shows up twice — once as a refusal, once as a convenience:

```rust
let make: fn(u32) -> Precinct = Precinct;                                  // it is a function value
let ps: Vec<Precinct> = vec![3u32, 7, 12].into_iter().map(Precinct).collect();
println!("{:?} {:?}", make(12), ps);  // Precinct(12) [Precinct(3), Precinct(7), Precinct(12)]
```

`.map(Ballot)` is `E0423`. There is nothing to pass.

## `new` is a convention

Nothing in the language has heard of it. `Ballot::new` is an ordinary [associated function](../impl_blocks/README.md) that happens to return `Self`, and you can call it whatever the domain calls it:

```rust
impl Ballot {
    fn new(voter: &str, score: u8) -> Self { Ballot { voter: voter.to_string(), score } }
    fn blank() -> Self { Ballot { voter: String::from("(unmarked)"), score: 0 } }
}
```

Standard library, four names for the same job: `Vec::new()`, `String::from("Cara")`, `u8::default()`, `Ballot::blank()`. A type with no `new` is not missing anything.

## `:` supplies a type, `=` supplies a value

Only the second is required. This compiles, with no `mut`:

```rust
let turnout = 431;
let decided: Ballot;              // declared, not initialized
if turnout > 400 {
    decided = Ballot::new("Cara", 4);
} else {
    decided = Ballot::blank();
}
println!("{}", decided.score);    // 4
```

One assignment per path is not a mutation, so the binding stays immutable — and the compiler proves it was set before the read. Miss a path and you get `E0381`. The case where this is the *right* tool rather than an `Option` is [initial values](../../17_Option_and_Result/initial_values/README.md).

## The refusals

Four ways to reach for a value and miss, and they are four different error codes.

**`let b: Ballot();`** — both mistakes at once, which is why it prints two errors:

```text title="Real rustc output — rustc 1.97.1, edition 2024"
error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
 --> scratch.rs:9:12
  |
9 |     let b: Ballot();
  |            ^^^^^^^^ only `Fn` traits may use parentheses

error[E0381]: used binding `b` isn't initialized
  --> scratch.rs:10:16
   |
 9 |     let b: Ballot();
   |         - binding declared here but left uninitialized
10 |     println!("{b:?}");
   |                ^ `b` used here but it isn't initialized
   |
help: consider assigning a value
   |
 9 |     let b: Ballot() = /* value */;
   |                     +++++++++++++
```

After `:` rustc is reading a **type**, and `Name(A) -> B` in type position is the sugar for the `Fn` traits — `E0214` is the parser saying `Ballot` is not one of them. `E0381` is separate and downstream: no `=` ever appeared, so nothing was assigned.

**`let b = Ballot();`** — the value slot, so a different complaint, and it names the fields for you:

```text title="Real rustc output"
error[E0423]: expected function, tuple struct or tuple variant, found struct `Ballot`
 --> scratch.rs:9:13
  |
2 | struct Ballot { voter: String, score: u8 }
  | ------------------------------------------ `Ballot` defined here
...
9 |     let b = Ballot();
  |             ^^^^^^^^ help: use struct literal syntax instead: `Ballot { voter: val, score: val }`
```

**`let s = Sealed();`** — a unit struct is already a value, so the parentheses are a call on something that is not a function:

```text title="Real rustc output"
error[E0618]: expected function, found struct `Sealed`
 --> scratch.rs:9:13
  |
6 | struct Sealed;
  | ------------- struct `Sealed` defined here
...
9 |     let s = Sealed();
  |             ^^^^^^--
  |             |
  |             call expression requires function
  |
help: `Sealed` is a unit struct, and does not take parentheses to be constructed
  |
9 -     let s = Sealed();
9 +     let s = Sealed;
  |
```

**`let p = Precinct;`** — the one that **compiles**. `p` is the constructor function, not a `Precinct`, and you find out on the next line:

```text title="Real rustc output"
error[E0277]: `fn(u32) -> Precinct {Precinct}` doesn't implement `Debug`
  --> scratch.rs:10:15
   |
 4 | struct Precinct(u32);
   | --------------- consider calling the constructor for `Precinct`
...
10 |     println!("{p:?}");
   |               ^^^^^ `fn(u32) -> Precinct {Precinct}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: use parentheses to construct this tuple struct: `Precinct(/* u32 */)`
```

The eight refusals of *defining* a struct are a separate page: [when a struct refuses](../when_a_struct_refuses/README.md).

## If you are coming from another language

**Python.** `Person()` calls the class, and calling it is construction — `__call__` on the metaclass runs `__new__` then `__init__`. Rust splits those roles: the type name is a name in the *type* namespace, and construction is a literal (`Ballot { .. }`) or a function you wrote (`Ballot::new(..)`). Three transfers to make explicitly:

| | Python | Rust |
|---|---|---|
| build one | `Ballot(voter, score)` — call the class | `Ballot { voter, score }` — a literal |
| the "constructor" | `__init__`, known to the language | `new`, known to nobody; any name works |
| declare without a value | `b: Ballot` — an annotation, no object | `let b: Ballot;` — same, and the compiler tracks it |

The last row is the trap that looks harmless. In Python an un-assigned annotation is a comment for a type checker and a `NameError` at runtime if you use it; in Rust `let b: Ballot;` is a real declaration the borrow checker follows, and reading it early is a *compile* error rather than a runtime one. So the habit transfers — it is the `()` that does not.

And a genuine difference worth keeping: Python has no equivalent of `Precinct` *being* a function value. `Ballot` is not callable at all, while `Precinct` can be handed to `.map()` — same declaration keyword, two different namespaces populated.

**ABAP.** The two halves of `let p = Ballot { .. };` are two statements you already write separately:

```abap
DATA lo_ballot TYPE REF TO lcl_ballot.   " the ':' half — a name and a type, no object
CREATE OBJECT lo_ballot.                  " the '=' half — the value arrives
lo_ballot->say_hello( ).
```

`let b: Ballot();` is that first line, with a call bolted onto the type name, and then a method call on a reference that was never filled. ABAP lets you compile it and dumps with `CX_SY_REF_IS_INITIAL` when the third line runs; Rust refuses the build. Two more mappings:

| | ABAP | Rust |
|---|---|---|
| the constructor | `CONSTRUCTOR` method, known to the language | `new`, a convention with no special status |
| a structure with no object | `DATA ls TYPE ty_ballot` — initialised free | every field supplied, or it does not compile |
| deferred fill | reference stays `IS INITIAL` until `CREATE OBJECT` | `let b: Ballot;` — and reading it early is `E0381` |

The structural change is the second row. ABAP hands you an initial value for free and lets `IS INITIAL` stand in for "not set yet"; Rust has no such state for a struct, so "not built" and "built" are the only options, and the check moved from your `IF` to the compiler.

---

## Practice

**Four spellings, four error codes — and one of them compiles.**

```rust
struct Voter { name: String, seat: u8 }
struct Precinct(u32);
struct Sealed;
```

1. Predict the refusal for each before compiling — one of the four produces **two** errors, and one produces none:
   `let v: Voter();` · `let s = Sealed();` · `let w = Voter;` · `let p = Precinct;`
2. Write the shortest spelling of each that works.
3. The one that compiled: what type is `p`, and what does the *next* line say when you try to print it?
4. One of the three names can be passed straight to `.map(..)`. Which, and why not the other two?
5. Rewrite one binding as a deferred initialization (`let x: T;`, then assign). Why does it need no `mut`?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:a_type_is_not_a_constructor_kata -->
*[`a_type_is_not_a_constructor_kata.rs`](examples/a_type_is_not_a_constructor_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: four spellings, four error codes — and the one that compiles.
//!
//!   rustc --edition 2024 a_type_is_not_a_constructor_kata.rs -o /tmp/atinack && /tmp/atinack

#[derive(Debug)]
struct Voter {
    name: String,
    seat: u8,
}

#[derive(Debug)]
struct Precinct(u32);

#[derive(Debug)]
struct Sealed;

// The four bindings as they arrive:
//
//     let v: Voter();      // 1
//     let s = Sealed();    // 2
//     let w = Voter;       // 3
//     let p = Precinct;    // 4  <- this one COMPILES
//
// Written correctly below, with what each refusal was actually about.

fn main() {
    println!("Four spellings of 'make me one'. Three refuse, and they refuse differently.\n");

    println!("1. let v: Voter();      TWO errors, not one");
    println!("     error[E0214]: parenthesized type parameters may only be used with a `Fn` trait");
    println!("     error[E0381]: used binding `v` isn't initialized");
    println!("   After `:` rustc reads a TYPE. `Voter()` in type position is the");
    println!("   `Fn(A) -> B` sugar, and only the Fn traits may use it. Then, separately,");
    println!("   no `=` ever appeared, so nothing was assigned.");
    let v = Voter { name: String::from("Ada"), seat: 7 };
    println!("   fixed:  let v = Voter {{ .. }};        {v:?}\n");

    println!("2. let s = Sealed();    E0618 — expected function, found struct `Sealed`");
    println!("   help: `Sealed` is a unit struct, and does not take parentheses");
    println!("         to be constructed");
    println!("   A unit struct declares a type AND a value of that type sharing one name.");
    let s = Sealed;
    println!("   fixed:  let s = Sealed;               {s:?}\n");

    println!("3. let w = Voter;       E0423 — expected value, found struct `Voter`");
    println!("   help: use struct literal syntax instead: `Voter {{ name: val, seat: val }}`");
    println!("   A named-field struct declares ONLY the type. The bare name is not a value.");
    let w = Voter { name: String::from("Ben"), seat: 3 };
    println!("   fixed:  let w = Voter {{ .. }};        {w:?}");
    println!("   (fields read the ordinary way: w.name = {:?}, w.seat = {})\n", w.name, w.seat);

    println!("4. let p = Precinct;    COMPILES — and gives you the wrong thing");
    println!("   `p` is not a Precinct. It is the tuple struct's constructor function,");
    println!("   of type `fn(u32) -> Precinct {{Precinct}}`, and the next line is where");
    println!("   you find out:");
    println!("     error[E0277]: `fn(u32) -> Precinct {{Precinct}}` doesn't implement `Debug`");
    println!("     help: use parentheses to construct this tuple struct: `Precinct(/* u32 */)`");
    let p = Precinct(431);
    println!("   fixed:  let p = Precinct(431);        {p:?}   (p.0 = {})\n", p.0);

    println!("5. Which name can be passed straight to `.map(..)`?");
    let ps: Vec<Precinct> = vec![12u32, 40, 431].into_iter().map(Precinct).collect();
    println!("   .map(Precinct) works: {ps:?}");
    println!("   Only the tuple struct. `Precinct` IS a function value, which is exactly");
    println!("   what refusal 4 was complaining about — the same fact, once as a bug and");
    println!("   once as a feature. `Voter` is a type only, and `Sealed` is a type plus a");
    println!("   value that takes no arguments; neither is callable.");
    println!("   .map(Voter)  -> E0423 expected value, found struct `Voter`");
    println!("   .map(Sealed) -> E0618 expected function, found struct `Sealed`\n");

    println!("6. Deferred initialization needs no `mut`");
    let turnout = 512;
    let seat: u8;
    if turnout > 500 {
        seat = 1;
    } else {
        seat = 2;
    }
    println!("   let seat: u8;  then one assignment per path  ->  seat = {seat}");
    println!("   `mut` permits a SECOND write. There is no second write here, so the");
    println!("   binding stays immutable and the compiler still proves it was set");
    println!("   before use. That is what `let v: Voter();` was reaching for — it just");
    println!("   put a call in the type slot and never supplied the value.");
}
```
<!-- /source -->

<!-- output:a_type_is_not_a_constructor_kata -->
*Verified output of [`a_type_is_not_a_constructor_kata.rs`](examples/a_type_is_not_a_constructor_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Four spellings of 'make me one'. Three refuse, and they refuse differently.

1. let v: Voter();      TWO errors, not one
     error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
     error[E0381]: used binding `v` isn't initialized
   After `:` rustc reads a TYPE. `Voter()` in type position is the
   `Fn(A) -> B` sugar, and only the Fn traits may use it. Then, separately,
   no `=` ever appeared, so nothing was assigned.
   fixed:  let v = Voter { .. };        Voter { name: "Ada", seat: 7 }

2. let s = Sealed();    E0618 — expected function, found struct `Sealed`
   help: `Sealed` is a unit struct, and does not take parentheses
         to be constructed
   A unit struct declares a type AND a value of that type sharing one name.
   fixed:  let s = Sealed;               Sealed

3. let w = Voter;       E0423 — expected value, found struct `Voter`
   help: use struct literal syntax instead: `Voter { name: val, seat: val }`
   A named-field struct declares ONLY the type. The bare name is not a value.
   fixed:  let w = Voter { .. };        Voter { name: "Ben", seat: 3 }
   (fields read the ordinary way: w.name = "Ben", w.seat = 3)

4. let p = Precinct;    COMPILES — and gives you the wrong thing
   `p` is not a Precinct. It is the tuple struct's constructor function,
   of type `fn(u32) -> Precinct {Precinct}`, and the next line is where
   you find out:
     error[E0277]: `fn(u32) -> Precinct {Precinct}` doesn't implement `Debug`
     help: use parentheses to construct this tuple struct: `Precinct(/* u32 */)`
   fixed:  let p = Precinct(431);        Precinct(431)   (p.0 = 431)

5. Which name can be passed straight to `.map(..)`?
   .map(Precinct) works: [Precinct(12), Precinct(40), Precinct(431)]
   Only the tuple struct. `Precinct` IS a function value, which is exactly
   what refusal 4 was complaining about — the same fact, once as a bug and
   once as a feature. `Voter` is a type only, and `Sealed` is a type plus a
   value that takes no arguments; neither is callable.
   .map(Voter)  -> E0423 expected value, found struct `Voter`
   .map(Sealed) -> E0618 expected function, found struct `Sealed`

6. Deferred initialization needs no `mut`
   let seat: u8;  then one assignment per path  ->  seat = 1
   `mut` permits a SECOND write. There is no second write here, so the
   binding stays immutable and the compiler still proves it was set
   before use. That is what `let v: Voter();` was reaching for — it just
   put a call in the type slot and never supplied the value.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:a_type_is_not_a_constructor -->
*Verified output of [`a_type_is_not_a_constructor.rs`](examples/a_type_is_not_a_constructor.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three flavors, three spellings — and only ONE of them is a call
   named-field   Ballot { voter, score }  ->  Ballot { voter: "Ada", score: 5 }
   tuple struct  Precinct(7)              ->  Precinct(7)
   unit struct   Sealed                   ->  Sealed
   ...ordinary values, with ordinary fields: named.voter = "Ada", named.score = 5, tuple.0 = 7
   The braces are a literal; the bare name is a value. `Ballot()` is
   not a shorter spelling of either one:
     error[E0423]: expected function, tuple struct or tuple variant,
                   found struct `Ballot`
     help: use struct literal syntax instead: `Ballot { voter: val, score: val }`

2. The tuple struct's name really IS a function — the other two are not
   let make: fn(u32) -> Precinct = Precinct;   make(12) = Precinct(12)
   [3, 7, 12].map(Precinct) = [Precinct(3), Precinct(7), Precinct(12)]
   `.map(Ballot)` is E0423: `expected value, found struct `Ballot``.
   So 'constructor' is literal for one flavor and a figure of speech for the rest.

3. `Sealed` is two declarations sharing a name
   let s: Sealed = Sealed;   Sealed
   Left of the `=` is the TYPE namespace; right of it is the VALUE namespace.
   `struct AlsoEmpty {}` declares only the type, so it needs `AlsoEmpty {}`.

4. `new` is a convention, and std does not even keep to it
   Ballot::new("Ben", 3)   Ballot { voter: "Ben", score: 3 }
   Ballot::blank()         Ballot { voter: "(unmarked)", score: 0 }
   Vec::<u8>::new()        []
   String::from("Cara")    "Cara"
   u8::default()           0
   Four names for 'make me one'. None of them is a keyword.

5. `:` supplies a TYPE, `=` supplies a VALUE — and only the second is required
   let decided: Ballot;  assigned on every path, exactly once
   turnout = 431  ->  Ballot { voter: "Cara", score: 4 }
   No `mut`: one assignment is not a mutation. Read it before assigning
   and the compiler stops you:
     error[E0381]: used binding `decided` isn't initialized

6. So `let b: Ballot();` is both mistakes in eight characters
     error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
       after `:` rustc is reading a TYPE, and `Name(..)` in type position is
       the `Fn(A) -> B` sugar, which only the Fn traits may use.
     error[E0381]: used binding `b` isn't initialized
       and no value was ever supplied, because `=` never appeared.
   The fix is one character and one pair of braces: `let b = Ballot { .. };`
```
<!-- /output -->

## See also

- [What a struct is](../what_a_struct_is/README.md) — the three flavors in full, and the privacy that makes a tuple struct's constructor private
- [When a struct refuses](../when_a_struct_refuses/README.md) — the eight errors of *defining* one
- [`impl` blocks](../impl_blocks/README.md) — where `new` lives, and why it is an associated function
- [Initial values](../../17_Option_and_Result/initial_values/README.md) — deferred initialization as the deliberate tool
- [STRUCTS.md](../../STRUCTS.md) — the map
