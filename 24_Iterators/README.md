# Iterators

**One line:** An iterator is a sequence that computes nothing until somebody asks — one `next` method, seventy-five more that arrive free, and three different doors onto every collection depending on whether you mean to read it, change it, or take it apart.

`for row in &rows` is on half the pages in this library, and nothing before this section said what the `&` was choosing, why the same loop without it consumes `rows`, or why a chain of `map` and `filter` allocates nothing in between.

The property everything else follows from is **laziness**: `map` and `filter` build a value describing the work, and a consumer like `collect` or `find` is what runs it — element at a time, all the way through the chain, stopping the moment it has its answer. That is why an endless sequence is usable, why `take(2)` can stop a source six elements long after two, and why the fluent style costs nothing that a hand-written loop would not.

| Lesson | Level | What it teaches |
|---|---|---|
| [Iterators are lazy](iterators_are_lazy/README.md) | 201 | Adapters build a plan and consumers run it, counted: 6 closure calls for `collect`, 1 for `find`, 0 for a chain nobody consumed — plus the interleaving proof that a chain is one pass, not one per adapter |
| [`iter`, `iter_mut`, `into_iter`](iter_iter_mut_into_iter/README.md) | 101 → 201 | The three doors onto a collection, which one a `for` loop picked for you, the array `into_iter` that changed meaning in edition 2021, and why the clone you added to make it compile was probably the wrong door |
| [Implementing `Iterator`](implementing_iterator/README.md) | 201 → 301 | One method and seventy-five arrive free; what does not (`rev`, `len`, `size_hint`); and why a collection must never *be* an iterator |
| [`fold` and `reduce`](fold_and_reduce/README.md) | 201 | The consumer the others are made of — `sum` really is `fold(0, +)` in std — plus `try_fold`, and the `.clone()` inside a fold that makes it quadratic |
| [`collect` and `FromIterator`](collect_and_fromiterator/README.md) | 201 | The call with no behaviour of its own: the target type decides. Including `Result<Vec<_>, _>`, which flips a sequence of failures into one, and keeps only the first |
| [Adapters by job](adapters_by_job/README.md) | 201 | Which of the twenty to reach for, and the four traps — `take_while` is not `filter`, and `flat_map` over a `Result` deletes your errors |
| [Returning an iterator](returning_an_iterator/README.md) | 201 → 301 | `impl Iterator` for the chain you cannot name, the lifetime edition 2024 now captures for you, and the second branch that forces a box |
| [`DoubleEndedIterator` and `ExactSizeIterator`](double_ended_and_exact_size/README.md) | 201 → 301 | Where `rev()` and `len()` actually come from, what each promises, and why `.enumerate().rev()` and `.rev().enumerate()` number the same rows differently |
| [When a `for` loop beats a chain](when_a_loop_beats_a_chain/README.md) | 201 | Four jobs the fluent style loses — an error that names its row, a growing work list, a break out of two levels, three answers in one pass — and why speed is not one of them |
| [`Iterator` versus `Stream`](iterator_vs_stream/README.md) | 301 | The same trait with *"not yet"* added to the answer, polled by hand with a nine-line executor; and why an `Iterator` inside async code never yields |

The first three are the spine — read them in that order, since laziness explains what the other two are describing and the three doors have to be solid before you write the doors for a type of your own. After that the section stops being a sequence: `fold` and `collect` are the two consumers worth knowing properly, *adapters by job* is a lookup table, and the last three are each answers to a question you will not have until you hit it.

## They all take closures

[`23_Closures/`](../23_Closures/README.md) is the other half, and the two sections were one section until this one outgrew it. Every adapter here takes a closure, and the bound in its signature is a promise about what it will do with yours: `map` takes an [`FnMut`](../23_Closures/three_closure_traits/README.md) because it runs per item and may carry state, while a fallback that runs at most once can take an `FnOnce` and let you move an owned value out of it. If a chain is refusing your closure, that page is usually the answer.

## Iterator pages that live elsewhere

Iterators turn up long before a section about them, so their lessons stay where a reader will meet them:

- [`Option` is a one-item collection](../17_Option_and_Result/option_as_collection/README.md) — every adapter on this page, over a sequence of length 0 or 1
- [Walking a string](../14_Strings/walking_a_string/README.md) — `chars`, `bytes`, `char_indices` and the split family: the three-door question for text, where the answer is different
- [`str::split`](../14_Strings/str_methods/str_split/README.md) and its neighbours — one page per method, most of them returning an iterator
- [A generic recursive type](../22_Generics/a_generic_recursive_type/README.md) — the linked list, which is where writing `next` by hand stops being optional
- [What a trait is](../12_Traits/what_a_trait_is/README.md) — required versus provided methods, the mechanism behind *"write one, get seventy-five"*
- [Endless iteration](../02_Errors/endless_iteration/README.md) — the loop that never ends because "no more input" is not an error (a stub)

## Not yet written

The remaining gaps, listed rather than stubbed so they are visible: **`itertools`**, the crate everybody adds for `chunk_by`, `sorted`, `dedup_by` and `Either` — and where its adapters differ from the std ones they resemble; **iterating a `HashMap`** and why the order is deliberately not the insertion order; **`Extend` and `Iterator::by_ref`**, the two mechanisms that let a chain be consumed in stages; **parallel iterators** (`rayon`'s `par_iter`), which is a one-word change with a real contract behind it; and **writing an iterator over a tree**, where `next` has to hold an explicit stack because the recursion cannot.
