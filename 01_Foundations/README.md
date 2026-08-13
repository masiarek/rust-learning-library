# Foundations

The ideas you meet in your first week of Rust and keep using forever. Each page is one idea, with a program you can compile and run beside it.

| Lesson | Level | What it teaches |
|---|---|---|
| [`Option` vs `Result`](option_vs_result/README.md) | 101 | Absence versus failure — and the single question ("could the caller ask *why not?*") that decides which type you want |
| [`Option` fields](option_fields/README.md) | 101 | `Option` in a type definition: required-by-default fields, and when `Option<Vec<T>>` is right |
| [`Option` is a one-item collection](option_as_collection/README.md) | 201 | It iterates; `None` really is variant 0; and why the tag is often free |

## The eight jobs `Option` does

The [`std::option` module docs](https://doc.rust-lang.org/core/option/) list what `Option` is actually *for*. It is a better map of the topic than any tutorial ordering, so here it is with the page that covers each job:

| Job | Covered by |
|---|---|
| Initial values | [`Option` fields](option_fields/README.md) — `Default` gives `None` for every `T` |
| Return values for **partial functions** (undefined over part of their input range) | [`Option` vs `Result`](option_vs_result/README.md) |
| Reporting **simple errors**, returning `None` on failure | [`Option` vs `Result`](option_vs_result/README.md) — and when to stop doing this and return `Result` |
| Optional **struct fields** | [`Option` fields](option_fields/README.md) |
| Fields that can be **loaned or taken** | [one-item collection](option_as_collection/README.md) — `take()` / `replace()` |
| Optional **function arguments** | *not yet written* |
| **Nullable pointers** | [one-item collection](option_as_collection/README.md) — the niche that makes `Option<Box<T>>` free |
| **Swapping** things out of difficult situations | [one-item collection](option_as_collection/README.md) — `take()` against the borrow checker |

"Partial function" is the phrase worth keeping. A function that is undefined for some of its inputs — `first()` on an empty list, square root of a negative — is exactly the shape `Option` exists to make total. Returning `Option<T>` turns a function that *sometimes has no answer* into one that *always* has an answer, where "no answer" is one of the answers.

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
