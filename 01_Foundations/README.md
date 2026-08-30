# Foundations

**One line:** The ideas you meet in your first week of Rust and keep using forever — seven sections' worth, which is why this page is a map rather than a list.

This folder used to hold all of them: sixty-odd lessons in one flat run, sorted by nothing a reader could see. They are now grouped by what they are about, and this page is the door to each group. Nothing was dropped, and every lesson kept its content — only its address changed.

Read them in this order. Each section assumes the ones above it and nothing else.

| Section | Lessons | What it covers |
|---|---|---|
| [First programs](../15_First_Programs/README.md) | 12 | Running a `.rs` file at all, reading what the compiler says back, the braces every later page uses without explaining, and getting a program to print — then `let`, the built-in types, and who decides which one you got |
| [Control flow](../25_Control_Flow/README.md) | 8 | `if`, `match` and the three loops — all of them expressions, so all of them have values. Stubs for now |
| [Structs](../16_Structs/README.md) | 8 | A type of your own: fields here, behaviour in a separate `impl` block, no constructor, and the eight errors one produces |
| [`Option` and `Result`](../17_Option_and_Result/README.md) | 24 | No null and no exceptions — two ordinary enums, and the dozen small methods between `match` and `.unwrap()` |
| [Ownership](../18_Ownership/README.md) | 15 | One owner per value, what a move actually transfers, borrowing, and the three questions the word "scope" collapses into one |
| [Strings](../14_Strings/README.md) | 20 | Text is an owner and a view — and the bytes underneath are why `len()` is not a character count; nine of the twenty are still outlines |
| [Numbers and bytes](../19_Numbers/README.md) | 5 | The unit everything else is measured in, how to write one down, and the type that cannot hold the value you typed |

Four of those topics are big enough to have a reading map of their own, which crosses section boundaries where the lessons do: [OPTION.md](../OPTION.md), [SHADOWING.md](../SHADOWING.md), [STRUCTS.md](../STRUCTS.md), [STRINGS.md](../STRINGS.md).

Two sections sit alongside these rather than after them, because between them they are what the rest of the language is made of: [Enums](../13_Enums/README.md) and [Traits](../12_Traits/README.md).

There is also a slow, optional thread running through several of the pages above — [the long way round to a STAR count](../ROADMAP.md), which sequences a handful of lessons so each one is the next thing Rust wants to teach. `newtype_score`, `representing_a_ballot` and `six_kinds_of_zero` are its first rungs. They stand alone as Rust lessons; the election is the excuse.

## The eight jobs `Option` does

The [`std::option` module docs ↗](https://doc.rust-lang.org/core/option/) list what `Option` is actually *for*. It is a better map of the topic than any tutorial ordering, so here it is with the page that covers each job:

| Job | Covered by |
|---|---|
| Initial values | [Initial values](../17_Option_and_Result/initial_values/README.md) — and why deferred initialization usually beats it |
| Return values for **partial functions** (undefined over part of their input range) | [Partial functions](../17_Option_and_Result/partial_functions/README.md) |
| Reporting **simple errors**, returning `None` on failure | [Returning `None` on error](../17_Option_and_Result/none_on_error/README.md) — where "simple" is the load-bearing word |
| Optional **struct fields** | [`Option` fields](../17_Option_and_Result/option_fields/README.md) |
| Fields that can be **loaned or taken** | [one-item collection](../17_Option_and_Result/option_as_collection/README.md) — `take()` / `replace()` |
| Optional **function arguments** | [Optional function arguments](../17_Option_and_Result/optional_arguments/README.md) — and the four alternatives that usually beat it |
| **Nullable pointers** | [Nullable pointers](../17_Option_and_Result/nullable_pointers/README.md) — `Option<Box<T>>`, the niche that makes it free, and the recursive type it unlocks |
| **Swapping** things out of difficult situations | [one-item collection](../17_Option_and_Result/option_as_collection/README.md) — `take()` against the borrow checker |

"Partial function" is the phrase worth keeping, and it now has [its own page](../17_Option_and_Result/partial_functions/README.md). A function undefined for some of its inputs — `first()` on an empty list, square root of a negative — is exactly the shape `Option` exists to make total: returning `Option<T>` turns a function that *sometimes has no answer* into one that *always* has an answer, where "no answer" is one of the answers.

**Every row in that table now has a page.**

## Planned

Rough order, not a promise. Each becomes a page once it has a runnable example worth reading:

- **Lifetimes** — what `'a` is actually annotating, and why most of the time you write none
- **Traits** — shared behaviour without inheritance; `impl Trait` versus `dyn Trait`
- **Error handling in the large** — `thiserror` for libraries, `anyhow` for applications, building on [`Option` vs `Result`](../17_Option_and_Result/option_vs_result/README.md)
- **Iterators** — laziness, and why the loop you would write by hand is usually slower
- **`Rc`, `RefCell`, and friends** — what to reach for when the borrow checker says no and you are sure you are right
