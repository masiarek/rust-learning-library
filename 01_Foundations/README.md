# Foundations

The ideas you meet in your first week of Rust and keep using forever. Each page is one idea, with a program you can compile and run beside it.

| Lesson | Level | What it teaches |
|---|---|---|
| [`Some` and `None`](some_and_none/README.md) | 101 | The enum itself: two shapes, one exhaustive `match`, and why `Some(0)` is not `None` |
| [`Option` vs `Result`](option_vs_result/README.md) | 101 | Absence versus failure — and the single question ("could the caller ask *why not?*") that decides which type you want |
| [Partial functions](partial_functions/README.md) | 201 | Why `Option` exists at all: it turns a function that is undefined somewhere into one that always answers |
| [Returning `None` on error](none_on_error/README.md) | 201 | Why `input.parse().ok()` is usually a downgrade: four distinct causes arriving as one indistinguishable `None` |
| [Initial values](initial_values/README.md) | 201 | The job where `Option` is usually the *wrong* tool — Rust lets you declare without initializing and proves you assigned |
| [`Option` fields](option_fields/README.md) | 101 | `Option` in a type definition: required-by-default fields, and when `Option<Vec<T>>` is right |
| [`Option` is a one-item collection](option_as_collection/README.md) | 201 | It iterates; `None` really is variant 0; and why the tag is often free |
| [Nullable pointers](nullable_pointers/README.md) | 201 | Rust has no null, so an absent pointer is a different type — `Option<Box<T>>`, free, and what makes a recursive type possible |
| [A score is not a number](newtype_score/README.md) | 101 → 201 | The newtype: one private field, one validating door, and why privacy is per *module* |
| [What is a ballot, in memory?](representing_a_ballot/README.md) | 201 | Array vs `Vec` vs tuple vs struct vs map vs flat matrix — and which bugs each one makes writeable |

The last two rows are the first rungs of [the long way round](../ROADMAP.md) — a slow walk toward a working STAR count, where each step exists because it is the next thing Rust wants to teach. They stand alone as Rust lessons; the election is the excuse.

## The eight jobs `Option` does

The [`std::option` module docs](https://doc.rust-lang.org/core/option/) list what `Option` is actually *for*. It is a better map of the topic than any tutorial ordering, so here it is with the page that covers each job:

| Job | Covered by |
|---|---|
| Initial values | [`Option` fields](option_fields/README.md) — `Default` gives `None` for every `T` |
| Return values for **partial functions** (undefined over part of their input range) | [Partial functions](partial_functions/README.md) |
| Reporting **simple errors**, returning `None` on failure | [`Option` vs `Result`](option_vs_result/README.md) — and when to stop doing this and return `Result` |
| Optional **struct fields** | [`Option` fields](option_fields/README.md) |
| Fields that can be **loaned or taken** | [one-item collection](option_as_collection/README.md) — `take()` / `replace()` |
| Optional **function arguments** | *not yet written* |
| **Nullable pointers** | [Nullable pointers](nullable_pointers/README.md) — `Option<Box<T>>`, the niche that makes it free, and the recursive type it unlocks |
| **Swapping** things out of difficult situations | [one-item collection](option_as_collection/README.md) — `take()` against the borrow checker |

"Partial function" is the phrase worth keeping, and it now has [its own page](partial_functions/README.md). A function undefined for some of its inputs — `first()` on an empty list, square root of a negative — is exactly the shape `Option` exists to make total: returning `Option<T>` turns a function that *sometimes has no answer* into one that *always* has an answer, where "no answer" is one of the answers.

## Planned

Rough order, not a promise. Each becomes a page once it has a runnable example worth reading:

- **Optional function arguments** — the last unwritten row in the table above
- **Ownership and moves** — why a value can only have one owner, and what "moved" means in an error message
- **Borrowing** — `&T` and `&mut T`, and the one-writer-or-many-readers rule
- **Lifetimes** — what `'a` is actually annotating, and why most of the time you write none
- **`String` vs `&str`** — the same split as `Vec<T>` vs `&[T]`, and why it exists
- **Traits** — shared behaviour without inheritance; `impl Trait` versus `dyn Trait`
- **Error handling in the large** — `thiserror` for libraries, `anyhow` for applications, building on [`Option` vs `Result`](option_vs_result/README.md)
- **Iterators** — laziness, and why the loop you would write by hand is usually slower
- **`Rc`, `RefCell`, and friends** — what to reach for when the borrow checker says no and you are sure you are right
