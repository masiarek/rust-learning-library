# Foundations

The ideas you meet in your first week of Rust and keep using forever. Each page is one idea, with a program you can compile and run beside it.

| Lesson | Level | What it teaches |
|---|---|---|
| [`Option` vs `Result`](option_vs_result/README.md) | 101 | Absence versus failure — and the single question ("could the caller ask *why not?*") that decides which type you want |

## Planned

Rough order, not a promise. Each becomes a page once it has a runnable example worth reading:

- **Ownership and moves** — why a value can only have one owner, and what "moved" means in an error message
- **Borrowing** — `&T` and `&mut T`, and the one-writer-or-many-readers rule
- **Lifetimes** — what `'a` is actually annotating, and why most of the time you write none
- **`String` vs `&str`** — the same split as `Vec<T>` vs `&[T]`, and why it exists
- **Traits** — shared behaviour without inheritance; `impl Trait` versus `dyn Trait`
- **Error handling in the large** — `thiserror` for libraries, `anyhow` for applications, building on [`Option` vs `Result`](option_vs_result/README.md)
- **Iterators** — laziness, and why the loop you would write by hand is usually slower
- **`Rc`, `RefCell`, and friends** — what to reach for when the borrow checker says no and you are sure you are right
