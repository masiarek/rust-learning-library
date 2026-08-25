# Option

**Level:** reference · the map

**One line:** `Option<T>` is either `Some(T)` or `None`, and this page is the door to every lesson about it — in the order the questions actually come up.

There is a lot here, because `Option` is the type you meet on your first day and are still learning in your third month. The lessons are all one-idea pages that stand alone; what follows is a reading order rather than a syllabus, so start where your question is. **If you just want to know what `Option` *is*, read the next section** — the tables after it are the reading order, not the explanation.

---

## What it is, before the reading order

If you have landed here wanting the *idea* rather than the syllabus, it is this.

**Rust has no null.** A variable of type `i32` is a number — not "usually a number, sometimes null". Always. So how do you write a function that sometimes has no answer? `find_user()` when nobody has that id; `parse()` on garbage; *"what score did this ballot give Cara?"* when the ballot left her blank.

You change the **type**. Not `i32`, but `Option<i32>` — an ordinary enum with exactly two shapes:

```rust
enum Option<T> {
    Some(T),   // there is a value, and here it is
    None,      // there is no value
}
```

So an `Option<i32>` is **a box that either holds a number or is empty**. `Some(5)` is a full box; `None` is an empty one. And the part that makes it worth anything: **the box is not a number.**

```rust
fn find_score(name: &str) -> Option<i32> {
    if name == "Ada" { Some(5) } else { None }
}

let s = find_score("Ben");
println!("{}", s + 1);      // does not compile
```

```text
error[E0369]: cannot add `{integer}` to `Option<i32>`
 --> opt.rs:6:22
  |
6 |     println!("{}", s + 1);
  |                    - ^ - {integer}
  |                    |
  |                    Option<i32>
```

You cannot add to it, print it as a number, or pass it where an `i32` is wanted. To use the number you have to **open the box**, and opening it means saying, right there, what happens when it is empty:

```rust
match find_score("Ben") {
    Some(n) => println!("scored {n}"),
    None    => println!("left blank"),
}
```

Delete the `None` arm and the program does not build. That is the whole feature. Everything below on this page — `unwrap_or`, `if let`, `?`, `map` — is library convenience written on top of that `match`.

### The same idea in the languages you already have

- **Python.** You write this already, in the docstring: *"returns an int, or `None` if not found"*. Then `score = find_score("Ben")` followed by `score + 1` runs fine for months and raises `TypeError` at 3am. Python's `None` is a **value that can turn up anywhere**, and checking for it is your discipline; Rust's `None` is a **shape the type declares**, and checking for it is the compiler's job. Same idea, moved out of the comment and into the type — and out of runtime into build time.
- **ABAP.** After `SELECT SINGLE name … INTO lv_name`, `lv_name` is `''` — which means *both* "no row found" *and* "the row exists and the name is genuinely blank". Which one it was lives in a **separate variable**, `sy-subrc`, that you must remember to read and that the next statement overwrites. **`Option<T>` is `sy-subrc` welded onto the data.** It travels with the value everywhere the value goes, nothing overwrites it, and you cannot reach the data without going through it.

### The one trap that catches everybody

`Some(0)` is **not** `None`. Zero is an answer; `None` is the absence of one.

```rust
Some(0).unwrap_or(42)   // -> 0    the voter scored her zero
None.unwrap_or(42)      // -> 42   the voter did not score her at all
```

Python's `score or 42` answers `42` for **both**, because `0` is falsy. ABAP's `IS INITIAL` says the same, because "0" and "never set" are one bit pattern. Rust keeps them apart — the difference between a ballot that scored a candidate **0** and one that left them **blank**.

That is enough to read any of the pages below. [`Some` and `None`](01_Foundations/some_and_none/README.md) is the same ground taken slowly, with the compiler errors in full.

---

## Start here

Three pages, in this order, are enough to use `Option` for real:

| # | Lesson | Level | The question it answers |
|---|---|---|---|
| 1 | [`Some` and `None`: reading an `Option`](01_Foundations/some_and_none/README.md) | 101 | What the type *is*, why `match` has to cover both shapes, and why `Some(0)` is not `None` |
| 2 | [`if let`: one arm, and move on](01_Foundations/if_let/README.md) | 101 | Handling only the case you care about — and the exhaustiveness you trade away for it |
| 3 | [`Option` vs `Result`](01_Foundations/option_vs_result/README.md) | 101 | Absence versus failure, decided by one question: *could the caller ask why not?* |

## Getting the value out

Eight ways, and the last two are decisions rather than accesses:

| Lesson | Level | Reach for it when |
|---|---|---|
| [`unwrap_or`: the default you already have](01_Foundations/unwrap_or/README.md) | 201 | The fallback is a value you are holding — and the trap that it is computed either way |
| [`unwrap_or_else`: the fallback that is built only if it is needed](01_Foundations/unwrap_or_else/README.md) | 201 | The fallback costs something, or depends on the error |
| [`unwrap_or_default`: the fallback the type chose for you](01_Foundations/unwrap_or_default/README.md) | 201 | `0` / `""` / empty is genuinely the right answer — and when the type's zero is not your domain's |
| [`map_or` and `map_or_else`: transform, or fall back](01_Foundations/map_or/README.md) | 201 | You want to *change* the value on the way out, with a default for the gap |
| [`Option` is a one-item collection](01_Foundations/option_as_collection/README.md) | 201 | Iterating it, `take()`/`replace()`, and asking a question without unwrapping |
| [`while let`: loop while the shape holds](01_Foundations/while_let/README.md) | 201 | Draining something until it hands back `None` |
| [`expect`: writing down the proof](01_Foundations/expect/README.md) | 201 | Absence is a bug, and you can write the sentence saying why it cannot happen |
| [What a panic costs](01_Foundations/what_a_panic_costs/README.md) | 201 | Before you write `unwrap`: what the crash does to a running program |

And one page about a claim you will read elsewhere: [Shadowing and `unwrap`](01_Foundations/shadowing_and_unwrap/README.md) (201) — they are unrelated, and the popular explanation credits shadowing for something `Copy` is doing.

## Why the type exists, and where it belongs

| Lesson | Level | What it teaches |
|---|---|---|
| [Partial functions: why `Option` exists](01_Foundations/partial_functions/README.md) | 201 | The justification: returning `Option<T>` makes a function that is undefined somewhere *total* |
| [`Option` fields: modelling what may be absent](01_Foundations/option_fields/README.md) | 101 | `Option` in a type definition — required-by-default fields, and when `Option<Vec<T>>` is right |
| [Optional function arguments](01_Foundations/optional_arguments/README.md) | 201 | No default parameters and no overloading: the five shapes that replace them |
| [Nullable pointers](01_Foundations/nullable_pointers/README.md) | 201 | `Option<Box<T>>` — free, and what makes a recursive type possible |
| [Initial values: when `Option` is the wrong tool](01_Foundations/initial_values/README.md) | 201 | The job it looks made for and isn't: Rust lets you declare without initializing and proves you assigned |

## When `Option` is not enough

`None` says *"no answer"* and nothing more. Once the caller could act on *why*, you owe them a `Result`:

| Lesson | Level | What it teaches |
|---|---|---|
| [Returning `None` on error](01_Foundations/none_on_error/README.md) | 201 | Why `input.parse().ok()` is usually a downgrade: four distinct causes arriving as one `None` |
| [Zero wins is not zero games](01_Foundations/wrong_guard/README.md) | 201 | Returning `Result` does not mean you guarded the input that actually has no answer |
| [The `Result` you are reading is probably an alias](01_Foundations/result_aliases/README.md) | 201 | `io::Result<T>` is `Result<T, io::Error>` — how to expand one and read what can fail |

## The eight jobs the standard library says it does

The [`std::option` module docs ↗](https://doc.rust-lang.org/core/option/) list what `Option` is *for*, which is a better map of the topic than any tutorial ordering. That list is kept in [`01_Foundations`](01_Foundations/README.md) with the page covering each job — including the ones a first reading of the docs makes sound alike.

## Practising it

Most of the pages above end in a `## Practice` exercise with a compiled solution. The order to attempt them in is in [KATAS.md](KATAS.md); every kata is a row there, and the numbers live only in that table.

## Looking a term up

[GLOSSARY.md](GLOSSARY.md) defines the vocabulary these pages use — `and_then`, discriminant, exhaustiveness, `let … else`, niche, the null-pointer optimization, `#[must_use]`, `?` — and every entry links to the page that explains it properly.
