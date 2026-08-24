# `Some` is a constructor, not a flag

**Level:** 101 → 201 · for newcomers

**One line:** `Some(x)` is a *function call* whose argument is the value itself — so `Some(None)` does not mean "present but empty", it means you handed an `Option` to something expecting a `u8`.

```rust
struct Person { first_name: String, age: Option<u8> }

let alfredo = Person {
    first_name: "Alfredo".to_string(),
    age: Some(None),          // <- "the age field is set, to nothing"
};
```

Reads fine in English — `Some` sounds like *set*, `None` like *nothing*.

```text title="Real rustc output — rustc 1.97.1, edition 2024"
error[E0308]: mismatched types
 12 |         age: Some(None),
    |              ---- ^^^^ expected `u8`, found `Option<_>`
    |              |
    |              arguments to this enum variant are incorrect
    |
note: tuple variant defined here   --> /…/core/src/option.rs:605:5
605 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
```

**"arguments to this enum variant are incorrect"** — not *"you nested an Option"*. And `Some` is shown as a **tuple variant**: a thing you call, called wrongly.

---

## Two shapes, no third

`Option<T>` has no shape meaning "present but empty":

| | what it is | what it holds |
|---|---|---|
| `None` | the whole absence | nothing — takes no argument |
| `Some(x)` | the whole presence | `x`, which must be a `T` |

`None` is not the payload of `Some` — it is its **sibling**. The clincher is that `Some` is a value you can hold:

```rust
let wrap: fn(u8) -> Option<u8> = Some;
wrap(31)                                  // Some(31)
[31u8, 44, 7].into_iter().map(Some)       // Some(31), Some(44), Some(7)
```

With the type written out, `Some(None)` is just a `fn(u8)` called with an `Option` — same error as `wrap(None)`.

## When `Some(None)` is right

It is a fine value, just not an `Option<u8>`. It is an **`Option<Option<u8>>`**, which earns its keep when there are *two* questions:

```rust
let rows: [(&str, Option<Option<u8>>); 3] = [
    ("never asked",     None),
    ("asked, declined", Some(None)),
    ("asked, answered", Some(Some(31))),
];
```

*Did we ask?* and *did they answer?* `.flatten()` collapses them once you no longer care which absence you had.

Same test [`Option` fields](../option_fields/README.md) applies to `Option<Vec<T>>`: keep the wrapper exactly when **absent and empty mean different things**. Usually they do not.

The two-layer type normally arrives *by accident*, from `.map()` with a closure that returns an `Option` — see [`Option` vs `Result`](../option_vs_result/README.md) and [what a monad is](../what_a_monad_is/README.md). Typing it by hand is the only route the compiler catches immediately.

## Elsewhere

- **Python** — `d.get("age")` returns `None` for a missing key *and* a present-`None`; telling them apart needs `"age" in d`, a separate expression the value does not carry. `Option<Option<u8>>` welds both answers into one value.
- **ABAP** — inexpressible on an elementary field: `lv_age TYPE i` is `0` for *unknown*, *declined*, and *a genuine zero*.

---

## Practice

**Three ways to make `Some(None)` compile, only one of them meant.** Type it into a `Person { name: String, age: Option<u8> }` and read `E0308` in full.

1. Build it three ways: delete the `Some`; pass a real `u8`; change the field's type so the original line is legal.
2. For the third, write a reader telling *not asked* from *asked and declined* from *answered*, and produce a count the singly-optional type could not.
3. Defend one as the fix you would ship for "we do not know Alfredo's age". One of the others is not wrong so much as expensive.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:some_is_a_constructor_kata -->
*[`some_is_a_constructor_kata.rs`](examples/some_is_a_constructor_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three ways to make `Some(None)` compile, and the one to ship.
//!
//!   rustc --edition 2024 some_is_a_constructor_kata.rs -o /tmp/sick && /tmp/sick

/// Fixes 1 and 3 both live here. The field is singly optional, so there are
/// exactly two things you can write: `None`, or `Some(<a u8>)`.
#[derive(Debug)]
struct Person {
    name: String,
    age: Option<u8>,
}

/// Fix 2 changes the FIELD instead of the value. Now `Some(None)` is legal —
/// and it means something the other type could not say.
#[derive(Debug)]
struct Respondent {
    name: String,
    age: Option<Option<u8>>,
}

fn person_age(p: &Person) -> String {
    match p.age {
        Some(a) => format!("{a}"),
        None => "unknown".to_string(),
    }
}

fn respondent_age(r: &Respondent) -> String {
    match r.age {
        None => "not asked".to_string(),
        Some(None) => "asked, declined".to_string(),
        Some(Some(a)) => format!("{a}"),
    }
}

fn main() {
    println!("Fix 1 — delete the `Some`. The absence of a u8 is spelled `None`.");
    println!("Fix 3 — or pass a u8, which is what `Some` was asking for all along.");
    let people = [
        Person { name: "Alfredo".to_string(), age: None },
        Person { name: "Bianca".to_string(), age: Some(31) },
    ];
    for p in &people {
        println!("   {:<9} {}", p.name, person_age(p));
    }
    println!("   Two states, and no way to write a third. That is the point.");

    println!("\nFix 2 — widen the field, and `Some(None)` starts carrying a fact.");
    let respondents = [
        Respondent { name: "Alfredo".to_string(), age: None },
        Respondent { name: "Bianca".to_string(), age: Some(None) },
        Respondent { name: "Chidi".to_string(), age: Some(Some(31)) },
    ];
    for r in &respondents {
        println!("   {:<9} {}", r.name, respondent_age(r));
    }

    // The report the singly-optional type cannot produce.
    let asked = respondents.iter().filter(|r| r.age.is_some()).count();
    let answered = respondents.iter().filter(|r| r.age.flatten().is_some()).count();
    println!("   asked {asked} of {}, answered {answered}", respondents.len());
    println!("   An Option<u8> field folds 'not asked' and 'declined' into one");
    println!("   None, and no reader downstream can prise them apart again.");

    println!("\nWhich would you ship for \"we do not know Alfredo's age\"? Fix 1.");
    println!("   Fix 2 buys a distinction you were not making. It puts a third");
    println!("   state in front of every reader, and each one now owes a");
    println!("   `Some(None)` arm for a fact nobody collected. Widen the type");
    println!("   when the second question is real — not to make one line build.");
}
```
<!-- /source -->

<!-- output:some_is_a_constructor_kata -->
*Verified output of [`some_is_a_constructor_kata.rs`](examples/some_is_a_constructor_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Fix 1 — delete the `Some`. The absence of a u8 is spelled `None`.
Fix 3 — or pass a u8, which is what `Some` was asking for all along.
   Alfredo   unknown
   Bianca    31
   Two states, and no way to write a third. That is the point.

Fix 2 — widen the field, and `Some(None)` starts carrying a fact.
   Alfredo   not asked
   Bianca    asked, declined
   Chidi     31
   asked 2 of 3, answered 1
   An Option<u8> field folds 'not asked' and 'declined' into one
   None, and no reader downstream can prise them apart again.

Which would you ship for "we do not know Alfredo's age"? Fix 1.
   Fix 2 buys a distinction you were not making. It puts a third
   state in front of every reader, and each one now owes a
   `Some(None)` arm for a fact nobody collected. Widen the type
   when the second question is real — not to make one line build.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:some_is_a_constructor -->
*Verified output of [`some_is_a_constructor.rs`](examples/some_is_a_constructor.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Two shapes, and neither of them is 'present but empty'
   Alfredo Sanchez, age not on file
   Bianca Rossi, 31 years old
   the field itself:  None  and  Some(31)

2. `Some` is a function, so it has a type you can write down
   let wrap: fn(u8) -> Option<u8> = Some;
   wrap(31)                  -> Some(31)
   [31, 44, 7].map(Some)     -> [Some(31), Some(44), Some(7)]
   So `Some(None)` is a call with the wrong argument type. The
   compiler says `expected u8, found Option<_>` because that is
   literally what happened: you passed an Option to fn(u8).

3. Where `Some(None)` is exactly right: two questions, not one
   never asked      None            flatten -> None
   asked, declined  Some(None)      flatten -> None
   asked, answered  Some(Some(31))  flatten -> Some(31)
   `.flatten()` collapses the two absences back into one, which is
   the right move only once you no longer need to tell them apart.
```
<!-- /output -->
