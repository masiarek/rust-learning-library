# Functional Rust

**One line:** Rust is not a functional language, but its two most-used features are borrowed whole from that tradition — the **closure**, a function that carries values with it, and the **iterator**, a sequence that computes nothing until somebody asks.

Neither is exotic and you have already used both: `unwrap_or_else(|| 0)` is a closure, `for row in &rows` is an iterator. What this section does is explain the parts that are usually picked up by imitation — which of the three `Fn` traits your closure got and why the compiler cares, what `move` actually moves, why a chain of adapters does not allocate, and what happens when you write `next` yourself.

The thing that makes them *Rust* rather than a borrowed idea is that both are compiled away. A closure is a struct the compiler wrote, sized to exactly what it captured — often zero bytes — and a chain of adapters compiles to the loop you would have written by hand. There is no runtime, no boxing by default, and no garbage collector deciding when the captured `String` dies. That last one is why closures here need a `move` keyword at all, and why iterators come in three flavours instead of one.

| Lesson | Level | What it teaches |
|---|---|---|
| [What a closure is](what_a_closure_is/README.md) | 101 → 201 | The two-pipe syntax, the capture that separates a closure from a `fn`, and the measured claim that its size is exactly what it captured — zero bytes for one that captured nothing |
| [The three closure traits](three_closure_traits/README.md) | 201 | `Fn` / `FnMut` / `FnOnce` as a ladder rather than a menu; which bound to write; and the widely-repeated sentence about `move` and `FnOnce` that the run refutes in both directions |
| [The `move` keyword](the_move_keyword/README.md) | 201 | What it moves, the two errors that demand it, the `Copy` case that silently copies instead — and the field-granularity capture that edition 2021 introduced, measured in bytes |
| [Iterators are lazy](iterators_are_lazy/README.md) | 201 | Adapters build a plan and consumers run it, counted: 6 closure calls for `collect`, 1 for `find`, 0 for a chain nobody consumed |
| [`iter`, `iter_mut`, `into_iter`](iter_iter_mut_into_iter/README.md) | 101 → 201 | The three doors onto a collection, which one a `for` loop picked for you, and why the clone you added to make it compile was probably the wrong door |
| [Implementing `Iterator`](implementing_iterator/README.md) | 201 → 301 | One method and seventy-five arrive free; what does not (`rev`, `len`, `size_hint`); and why a collection must never *be* an iterator |

Read them in that order if you are reading the section rather than looking something up: the closure pages build the argument the iterator pages spend, since every adapter takes a closure and the bound it takes tells you what it is allowed to do with it.

## Where the neighbouring sections take over

- **[Ownership](../18_Ownership/README.md)** — capturing a value is a move or a borrow, so most closure errors are ownership errors in a costume. [Ownership and moves](../18_Ownership/ownership_and_moves/README.md) and [borrowing](../18_Ownership/borrowing/README.md) are the rules; [how to learn lifetimes](../18_Ownership/how_to_learn_lifetimes/README.md) is the `'static` on `thread::spawn`.
- **[Traits](../12_Traits/README.md)** — `Fn`, `FnMut`, `FnOnce`, `Iterator` and `IntoIterator` are ordinary traits with nothing built into the language. [What a trait is](../12_Traits/what_a_trait_is/README.md) covers required versus provided methods, which is the mechanism behind *"implement one method, get seventy-five"*; [returning a trait](../12_Traits/returning_a_trait/README.md) covers `impl Iterator<Item = T>` as a return type.
- **[Generics](../22_Generics/README.md)** — `fn apply<F: Fn(i32) -> i32>(f: F)` is a generic function with a trait bound, and it is stamped out once per closure type.
- **[`Option` and `Result`](../17_Option_and_Result/README.md)** — `map`, `and_then`, `unwrap_or_else` and friends are the same closure-taking shape on a container of at most one item. [`Option` is a one-item collection](../17_Option_and_Result/option_as_collection/README.md) makes that literal.

## Not yet written

The gaps, listed rather than stubbed so they are visible: **`fold` and `reduce`** (the consumer every other consumer is a special case of), **`collect` and `FromIterator`** — including `collect::<Result<Vec<_>, _>>()`, which turns a sequence of fallible rows into one fallible sequence — **iterator adapters by job** (`flat_map`, `partition`, `scan`, `windows`, `chunks`, `peekable`, `zip`/`unzip`), **`impl Iterator` as a return type** versus naming the concrete adapter chain, **`Iterator` versus `Stream`** once `async` arrives, and **when a `for` loop beats a chain**, which is more often than the fluent style suggests.
