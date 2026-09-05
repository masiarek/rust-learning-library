# `slice` methods

**Level:** reference · for working programmers

**One line:** One page per slice method a `Vec` reader reaches for first — the 22 that the [`Vec` methods](../vec_methods/README.md) page names as *not on its list* — each with the signature, what it actually does, the trap it is usually involved in, and a program whose printed output is checked by CI.

`Vec<T>` implements `Deref<Target = [T]>`, and so does every array, so these methods are called on a `Vec` or an array as if they were its own — `v.sort()`, `arr.first()` — while living on the slice. That is why `sort` is not a `Vec` method, and why learning them once covers both types. [Arrays and slices](../arrays_and_slices/README.md) is the lesson; this folder is the per-method reference beside it, shaped like the [`Vec`](../vec_methods/README.md) and [`str`](../../14_Strings/str_methods/README.md) ones.

**This is not the whole slice API.** The pinned toolchain's documentation for `[T]` lists over two hundred methods, inherent and trait-provided together; the 22 here are the everyday set. Where a page's nearest relative has no page of its own — `chunks_exact`, `split_at_mut`, `partition_point`, `sort_by_cached_key`, `rotate_right`, `fill_with` — it is linked to std with a ↗.

Every page has the same shape: a one-line summary, the signature, the stability line, the prose, then a **complete runnable program and its verified output**. Nothing on any of these pages is hand-typed output — [`tools/run_examples.py`](../../CONTRIBUTING.md) compiles each example, runs it, and fails the build if what the page shows is not what the program printed. The signature block is a `text` fence on purpose: a bare `pub fn …` is not something you can paste, and the [house rule](../../CONTRIBUTING.md) is that the first `rust` block on a page must compile.

## The signatures

Taken from the rendered documentation of the **pinned toolchain**, Rust 1.98.0. Two things to know when reading them.

**`const fn` is common here, and it rarely matters.** `first`, `last`, `reverse`, `swap` and `rotate_left` are all `const` — they touch no allocator, so they can run in a `const` context — and the stability line on each page records when that landed. It changes nothing about calling them at run time.

**`SliceIndex` is one bound doing two jobs.** [`get`](slice_get/README.md) and [`get_mut`](slice_get_mut/README.md) take `I: SliceIndex<[T]>`, which is how a single method accepts both a `usize` (returning one element) and a range (returning a sub-slice). The same trait is behind `v[i]` and `v[a..b]`.

## Sorting

All four are **stable** — equal elements keep their order — except `sort_unstable`, whose name says so; all four return `()`.

| method | what it does |
|---|---|
| [`sort`](slice_sort/README.md) | Ascending by `Ord`; may allocate a temporary buffer |
| [`sort_unstable`](slice_sort_unstable/README.md) | The same, in place, without the equal-elements promise |
| [`sort_by`](slice_sort_by/README.md) | By a comparison you write — descending, floats, several keys |
| [`sort_by_key`](slice_sort_by_key/README.md) | By a key you extract, recomputed at every comparison |

## Searching

| method | what it does |
|---|---|
| [`contains`](slice_contains/README.md) | Linear scan by `==`; takes a `&T`, which is the `Vec<String>` trap |
| [`binary_search`](slice_binary_search/README.md) | O(log n) on a **sorted** slice; `Err` carries the insertion point |

`iter().position(…)` — the index of the first element passing a predicate — is an [`Iterator` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position) method, reached through [`iter`](slice_iter/README.md).

## The ends

| method | what it does |
|---|---|
| [`first`](slice_first/README.md) | `Option<&T>` — `None` where `v[0]` would panic |
| [`last`](slice_last/README.md) | `Option<&T>` — the safe spelling of `v[v.len() - 1]` |
| [`first_mut`](slice_first_mut/README.md) | The same, writable |
| [`last_mut`](slice_last_mut/README.md) | The same, writable |

## Safe indexing

| method | what it does |
|---|---|
| [`get`](slice_get/README.md) | An element or a range as an `Option`; an out-of-range range is `None`, not clamped |
| [`get_mut`](slice_get_mut/README.md) | The same, writable — and why two at once needs `get_disjoint_mut` |

## Iterating

| method | what it does |
|---|---|
| [`iter`](slice_iter/README.md) | Over `&T` — what `for x in &v` calls |
| [`iter_mut`](slice_iter_mut/README.md) | Over `&mut T` — what `for x in &mut v` calls |
| [`chunks`](slice_chunks/README.md) | Non-overlapping groups of `n`; the last may be short |
| [`windows`](slice_windows/README.md) | Overlapping runs of `n`; never short |

## Rearranging

In place, and each returns `()`.

| method | what it does |
|---|---|
| [`reverse`](slice_reverse/README.md) | Flip the order |
| [`swap`](slice_swap/README.md) | Exchange two indices — the thing `mem::swap` on two `&mut v[i]` cannot do |
| [`rotate_left`](slice_rotate_left/README.md) | Shift left by `mid`, wrapping; `mid > len` panics |
| [`fill`](slice_fill/README.md) | Every element becomes a clone of one value; the length never changes |

## Joining

| method | what it does |
|---|---|
| [`concat`](slice_concat/README.md) | Flatten a slice of slices or strings into one `Vec` or `String` |
| [`join`](slice_join/README.md) | The same, with a separator between the pieces |

## See also

- [Arrays and slices](../arrays_and_slices/README.md) — the lesson these are the reference for
- [`Vec` methods](../vec_methods/README.md) — the methods that *are* on `Vec`: the ones that change the length or the buffer
- [`str` methods](../../14_Strings/str_methods/README.md) — the same owner-and-view split for text
- [`slice` in the standard library ↗](https://doc.rust-lang.org/std/primitive.slice.html) — the full list
