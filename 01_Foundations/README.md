# Foundations

The ideas you meet in your first week of Rust and keep using forever. Each page is one idea, with a program you can compile and run beside it.

| Lesson | Level | What it teaches |
|---|---|---|
| [Running a scratch program](rustc_without_cargo/README.md) | 101 | `rustc` alone, `cargo new`, and `src/bin/` — rustc's edition-2015 default, and what Cargo was quietly doing for you |
| [`Some` and `None`](some_and_none/README.md) | 101 | The enum itself: two shapes, one exhaustive `match`, and why `Some(0)` is not `None` |
| [`Option` vs `Result`](option_vs_result/README.md) | 101 | Absence versus failure — and the single question ("could the caller ask *why not?*") that decides which type you want |
| [Partial functions](partial_functions/README.md) | 201 | Why `Option` exists at all: it turns a function that is undefined somewhere into one that always answers |
| [Returning `None` on error](none_on_error/README.md) | 201 | Why `input.parse().ok()` is usually a downgrade: four distinct causes arriving as one indistinguishable `None` |
| [Zero wins is not zero games](wrong_guard/README.md) | 201 | A guard on the wrong condition: the input with no answer is the one that gets a number, and `Result` does not stop it |
| [Initial values](initial_values/README.md) | 201 | The job where `Option` is usually the *wrong* tool — Rust lets you declare without initializing and proves you assigned |
| [Optional function arguments](optional_arguments/README.md) | 201 | No default parameters, no overloading — the five shapes that replace them, and why `Option<&T>` beats `&Option<T>` |
| [`if let`](if_let/README.md) | 101 | A `match` with one arm — the family (`if let` / `let … else` / `while let` / `matches!`), and the exhaustiveness you trade away |
| [`unwrap_or`](unwrap_or/README.md) | 201 | The default you already have — eager, consuming, and it erases the very difference it stood in for |
| [`unwrap_or_else`](unwrap_or_else/README.md) | 201 | The lazy fallback — built only if needed, allowed to consume what it captures, and on a `Result` the only one handed the error |
| [`unwrap_or_default`](unwrap_or_default/README.md) | 201 | The fallback the *type* chose — a derived `Default` is the type's zero, not your domain's, and a missing impl is a guard rail |
| [`map_or` and `map_or_else`](map_or/README.md) | 201 | Transform *and* fall back in one call — the default written first and run last, and the clippy lint on each side of it |
| [`expect`](expect/README.md) | 201 | The message is a claim about why this cannot fail — and being unable to write it is the finding |
| [What a panic costs](what_a_panic_costs/README.md) | 201 | The other half of `unwrap`: where the panic points, what unwinding gives back (memory) and what it does not (your work), and why the exit code is 101 |
| [`while let`](while_let/README.md) | 201 | The loop whose exit condition is a pattern — and the one bug `if let` cannot have: a body that never makes progress |
| [Borrowing](borrowing/README.md) | 101 → 201 | `&T` and `&mut T`, the many-readers-or-one-writer rule, and the last-use rule that decides which order compiles |
| [`Option` fields](option_fields/README.md) | 101 | `Option` in a type definition: required-by-default fields, and when `Option<Vec<T>>` is right |
| [`Option` is a one-item collection](option_as_collection/README.md) | 201 | It iterates; `None` really is variant 0; and why the tag is often free |
| [Nullable pointers](nullable_pointers/README.md) | 201 | Rust has no null, so an absent pointer is a different type — `Option<Box<T>>`, free, and what makes a recursive type possible |
| [The `Result` you are reading is probably an alias](result_aliases/README.md) | 201 | `io::Result<T>` is `Result<T, io::Error>` — how to expand an alias and read what can actually go wrong, plus `Ok(())` and `Infallible` |
| [Shadowing and `unwrap`](shadowing_and_unwrap/README.md) | 201 | Two unrelated ideas the tutorials tie together: what keeps the original `Option` alive is `Copy`, not the shadow — and what shadowing is really for |
| [Ownership and moves](ownership_and_moves/README.md) | 101 | A move transfers *responsibility*, not bytes — the three rules, made visible by a value that announces its own death |
| [A score is not a number](newtype_score/README.md) | 101 → 201 | The newtype: one private field, one validating door, and why privacy is per *module* |
| [What is a ballot, in memory?](representing_a_ballot/README.md) | 201 | Array vs `Vec` vs tuple vs struct vs map vs flat matrix — and which bugs each one makes writeable |
| [Six kinds of zero](six_kinds_of_zero/README.md) | 201 | Where `Option` runs out: six reasons a cell is empty, one enum, and a report the compiler audits |
| [Meet the byte](meet_the_byte/README.md) | 101 → 201 | `u8` is one byte and the unit `size_of` counts in — plus the three bills a width comes with: overflow that differs by build, the shift the type picks, and a `.len()` measured in bytes |
| [Why hexadecimal](why_hexadecimal/README.md) | 101 → 201 | Why a byte is two hex digits and always will be — plus the three traps that follow: unpadded `{:x}` losing the byte boundary, `from_str_radix` refusing the `0x` it just printed, and hex of a negative showing two's complement |
| [What a float actually stores](what_a_float_stores/README.md) | 201 | The one division that ends exactness — why `0.1` is not 0.1, why the error goes both ways, and why Rust withholds `Eq` and `Ord` from `f64` |

The `newtype_score`, `representing_a_ballot` and `six_kinds_of_zero` rows are the first rungs of [the long way round](../ROADMAP.md) — a slow walk toward a working STAR count, where each step exists because it is the next thing Rust wants to teach. They stand alone as Rust lessons; the election is the excuse.

## The eight jobs `Option` does

The [`std::option` module docs](https://doc.rust-lang.org/core/option/) list what `Option` is actually *for*. It is a better map of the topic than any tutorial ordering, so here it is with the page that covers each job:

| Job | Covered by |
|---|---|
| Initial values | [Initial values](initial_values/README.md) — and why deferred initialization usually beats it |
| Return values for **partial functions** (undefined over part of their input range) | [Partial functions](partial_functions/README.md) |
| Reporting **simple errors**, returning `None` on failure | [Returning `None` on error](none_on_error/README.md) — where "simple" is the load-bearing word |
| Optional **struct fields** | [`Option` fields](option_fields/README.md) |
| Fields that can be **loaned or taken** | [one-item collection](option_as_collection/README.md) — `take()` / `replace()` |
| Optional **function arguments** | [Optional function arguments](optional_arguments/README.md) — and the four alternatives that usually beat it |
| **Nullable pointers** | [Nullable pointers](nullable_pointers/README.md) — `Option<Box<T>>`, the niche that makes it free, and the recursive type it unlocks |
| **Swapping** things out of difficult situations | [one-item collection](option_as_collection/README.md) — `take()` against the borrow checker |

"Partial function" is the phrase worth keeping, and it now has [its own page](partial_functions/README.md). A function undefined for some of its inputs — `first()` on an empty list, square root of a negative — is exactly the shape `Option` exists to make total: returning `Option<T>` turns a function that *sometimes has no answer* into one that *always* has an answer, where "no answer" is one of the answers.

**Every row in that table now has a page.**

## Planned

Rough order, not a promise. Each becomes a page once it has a runnable example worth reading:

- **Lifetimes** — what `'a` is actually annotating, and why most of the time you write none
- **`String` vs `&str`** — the same split as `Vec<T>` vs `&[T]`, and why it exists
- **Traits** — shared behaviour without inheritance; `impl Trait` versus `dyn Trait`
- **Error handling in the large** — `thiserror` for libraries, `anyhow` for applications, building on [`Option` vs `Result`](option_vs_result/README.md)
- **Iterators** — laziness, and why the loop you would write by hand is usually slower
- **`Rc`, `RefCell`, and friends** — what to reach for when the borrow checker says no and you are sure you are right
