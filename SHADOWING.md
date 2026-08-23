# Shadowing

**Level:** reference · the map

**One line:** Writing `let x` when `x` already exists makes a **second variable** wearing the first one's name — and this page is the door to every lesson about what that does, what it is for, and where it goes wrong.

Three pages, and they answer three different questions: *what is it*, *what happens to the value it hid*, and *should I use it here*. Start wherever your question is; none of them assumes the others.

---

## What it is, before the reading order

If you landed here wanting the idea rather than the syllabus, it is this.

```rust
let spaces = "   ";
let spaces = spaces.len();
println!("{spaces}");  // 3
```

That is legal, and the second line is **not** an assignment. `spaces` is not being changed from a string into a number — a number cannot be put in a string's storage. A *new variable* is declared, of a different type, and it takes over the name for the rest of the scope. The old one is untouched.

Three consequences follow, and everything on the three pages below is one of them.

**It can change the type, so a name can survive a conversion.** This is the whole point. `input` stays `input` through `&str` → trimmed `&str` → `u32`, instead of becoming `input_raw`, `input_trimmed`, `input_num` — names that re-encode in text what the compiler already knows.

**It is not mutation, and that distinction is load-bearing.** `let mut x` promises the reader *"this may change anywhere below"*. A shadow promises *"this changed here, and the result is final"*. The second is a much stronger claim, and it is why so much idiomatic Rust manages without `mut` at all. The two are not interchangeable, either: a shadow cannot write to something that outlives the block it is in, which is the most common way shadowing is misused.

**It takes away a name, not a value.** This is the one that surprises people. The shadowed value is still alive, still owned, still borrowable through any reference taken earlier — and because values drop in reverse declaration order, it dies *after* the shadow that hid it. So shadowing is not a way to release something early; it is the opposite, since it removes the handle you would have needed.

### The same idea in the languages you already have

- **Python.** `x = int(x)` looks like the same move and is not. Python **rebinds one variable**: the old object loses a reference and may be collected, so you can never have both alive under one name. Rust makes a **second variable** and keeps the first — which is why shadowing a lock in Rust leaves the lock held, a bug Python cannot have.
- **ABAP.** There is no analogue. A `DATA` name is one typed variable for the whole routine, and declaring `DATA(lv_x)` twice is a syntax error. So ABAP has to use `lv_input` / `lv_input_num`, and its `lv_` / `lt_` prefixes exist to carry the type in the name — precisely the job shadowing removes. What you give up in exchange is ABAP's free guarantee that one name in a routine means one thing.
- **C / C++.** Both shadow, but only in a *nested* block; a redefinition in the same block is an error, which is why C code opens braces to get the effect and why `-Wshadow` exists. Neither gives you the type change, because the declaration carries the type.

## The three lessons

| # | Lesson | Level | The question it answers |
|---|---|---|---|
| 1 | [Shadowing and `unwrap`](01_Foundations/shadowing_and_unwrap/README.md) | 201 | What shadowing *is* — and why the popular explanation that ties it to `unwrap` is crediting it for something `Copy` is doing |
| 2 | [A shadow does not drop](01_Foundations/shadowing_does_not_drop/README.md) | 201 | What happens to the value underneath: nothing. It outlives the shadow that hid it, and no name is left to free it early |
| 3 | [When to shadow](01_Foundations/when_to_shadow/README.md) | 201 | The judgement call — the design trade against `mut`, the five idioms, and the three bugs that compile |

Read in that order they build: the first defines it, the second explains the mechanism the third page's worst bug rests on, and the third is the one you will come back to.

## Where it turns up elsewhere

Shadowing is not a topic you finish; it is a habit that shows up inside other lessons. These are the places it does real work, or does real damage:

| Lesson | What shadowing is doing there |
|---|---|
| [What a warning is asking](01_Foundations/what_a_warning_is_asking/README.md) | `unused variable` is the compiler's only genuine net against an accidental shadow — and `unused_mut` on an accumulator is a shadowing bug reported in words that never say so |
| [Ownership and moves](01_Foundations/ownership_and_moves/README.md) | Drop order in reverse declaration order, which is what makes a shadowed value outlive its shadow |
| [Borrowing](01_Foundations/borrowing/README.md) | Why a reference taken before a shadow keeps working after it |
| [`if let`](01_Foundations/if_let/README.md) | Pattern bindings introduce a fresh name; `let … else` is the guard clause the unwrap-and-narrow idiom opens with |
| [Initial values](01_Foundations/initial_values/README.md) | The other route away from `mut` — declare without initializing and let the compiler prove you assigned |
| [A score is not a number](01_Foundations/newtype_score/README.md) | `let id = BallotId(id);` — narrowing into a newtype so the loose form becomes unreachable |
| [Lock poisoning](09_Advanced/mutex_poisoning/README.md) | The guards that must never be shadowed, and what a held one costs a second thread |

## The one rule, if you only keep one

**Shadow when the new binding is the same concept in a new form, and keep it close to the one it replaces.** Reach for a second name when it is a different thing, and never shadow a value that holds a resource. [When to shadow](01_Foundations/when_to_shadow/README.md) is that sentence with the evidence attached.

## Practising it

Each of the three pages ends in a `## Practice` exercise with a solution CI compiles and runs. The order to attempt them in is in [KATAS.md](KATAS.md); every kata is a row there, and the numbers live only in that table.

## Looking a term up

[GLOSSARY.md](GLOSSARY.md) defines the vocabulary these pages use — shadowing, pattern binding, drop order, `Copy`, dangling reference, `clippy::shadow_unrelated` — and every entry links to the page that explains it properly.
