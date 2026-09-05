# `Vec` methods

**Level:** reference · for working programmers

**One line:** One page per method on `Vec<T>` — all 46 that stable Rust 1.98 exposes, plus the `IntoIterator` impls — each with the signature, what it actually does, the trap it is usually involved in, and a program whose printed output is checked by CI.

These pages are a **reference**, not a course. If you have not met `Vec` yet, read [The `Vec`](../the_vec/README.md) first — it explains the pointer/length/capacity triple that makes half of this list make sense. Come back when you know which method you want and need to know how it behaves at the edges.

Every page has the same shape: a one-line summary, the signature, the stability line, the prose, then a **complete runnable program and its verified output**. Nothing on any of these pages is hand-typed output — [`tools/run_examples.py`](../../CONTRIBUTING.md) compiles each example, runs it, and fails the build if what the page shows is not what the program printed.

**The fences without an answer key were compiled too.** `run_examples.py` reaches `examples/*.rs` and nothing else, so a hand-authored `rust` block on a page is a claim no gate was checking — and on a reference page that is a strong claim, because the reader's next move is to paste it. Every such block in this folder was therefore extracted back out of the finished page and compiled on its own:

```bash
rustc --edition 2024 --crate-type lib --emit=metadata fence.rs
```

Four failed. Three were loose statements with no enclosing item, which a reader pasting them would have met as a syntax error; the fourth was the three `impl IntoIterator` blocks quoted from std, which cannot compile standalone at all and is a `text` fence now, with a line saying why.

**That check belongs to this folder, not to the library.** Run over the whole repo it fires on 142 of the 178 pages that carry a Rust fence — four in five — and almost none of them are wrong: a lesson about `let` opens with `let x = 5;` on purpose, and wrapping that in an `fn main()` to satisfy a compiler would make it worse to read. The difference is not syntax but contract. Here a fence is a complete demonstration of one method; on a teaching page it is an illustration of one line. No compiler can see which a page promised.

## Most "`Vec` methods" are not on this list

`Vec<T>` implements `Deref<Target = [T]>`, so every slice method is reachable on a vector and the compiler inserts the conversion silently. That is where the ones you are most likely to be looking for actually live:

| you probably want | it is a **slice** method |
|---|---|
| sorting | [`sort`](../slice_methods/slice_sort/README.md), [`sort_unstable`](../slice_methods/slice_sort_unstable/README.md), [`sort_by`](../slice_methods/slice_sort_by/README.md), [`sort_by_key`](../slice_methods/slice_sort_by_key/README.md) |
| searching | [`contains`](../slice_methods/slice_contains/README.md), [`binary_search`](../slice_methods/slice_binary_search/README.md), [`iter().position(…)` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position) |
| the ends | [`first`](../slice_methods/slice_first/README.md), [`last`](../slice_methods/slice_last/README.md), [`first_mut`](../slice_methods/slice_first_mut/README.md), [`last_mut`](../slice_methods/slice_last_mut/README.md) |
| safe indexing | [`get`](../slice_methods/slice_get/README.md), [`get_mut`](../slice_methods/slice_get_mut/README.md) — and the index may be a **range**: `v.get(1..3)` is `Some(&v[1..3])`, `v.get(1..99)` is `None` rather than a clamp or a panic |
| iterating | [`iter`](../slice_methods/slice_iter/README.md), [`iter_mut`](../slice_methods/slice_iter_mut/README.md), [`chunks`](../slice_methods/slice_chunks/README.md), [`windows`](../slice_methods/slice_windows/README.md) |
| rearranging | [`reverse`](../slice_methods/slice_reverse/README.md), [`swap`](../slice_methods/slice_swap/README.md), [`rotate_left`](../slice_methods/slice_rotate_left/README.md), [`fill`](../slice_methods/slice_fill/README.md) |
| joining | [`concat`](../slice_methods/slice_concat/README.md), [`join`](../slice_methods/slice_join/README.md) |

Each of those has a page of its own in the [`slice` methods](../slice_methods/README.md) reference, built to the same promise as this one; [arrays and slices](../arrays_and_slices/README.md) is the lesson, and [`slice` ↗](https://doc.rust-lang.org/std/primitive.slice.html) in std is the full list. **This list holds only what `Vec` itself defines** — which is, almost exactly, the operations that change the *length* or the *buffer*. Anything that works within a fixed length belongs to the slice.

That split is the single most useful thing to know about the type: `Vec` owns and resizes, `[T]` is a window on elements that already exist.

## The signatures

Taken from the rendered documentation of the **pinned toolchain**, Rust 1.98.0, so they match what you get when you build this repo.

Three notes on reading them.

**The allocator parameter.** `Vec` has a second, unstable generic parameter (`Vec<T, A>`), which appears in some return types here — ignore it; on stable there is only the global allocator.

**`const fn` rarely means what you want.** It marks a method that may be *called* in a const context, which for most of these is not usable in practice: a `Vec` cannot be dropped at compile time (`error[E0493]`), so `Vec::new()` in a `static` works and almost nothing else does.

**`T` must be `Sized`.** The bound is implicit — it is not written in any signature — so `Vec<dyn Debug>` fails with `error[E0277]`, and the note points at the struct definition: *"required by an implicit `Sized` bound in `Vec`"*. The fix is a layer of indirection, `Vec<Box<dyn Debug>>`, and it is the same bound behind `Vec<[T]>` and `Vec<str>` being rejected.

While you are there: the struct has **two** fields, `buf: RawVec<T, A>` and `len: usize` — the capacity lives inside `RawVec`, not beside the length. The familiar pointer/length/capacity triple is what a `Vec` *means*, not how it is spelled, and std says outright that *"the ABI is not stable and `Vec` makes no guarantees about its memory layout (including the order of fields)"*, so do not write code that assumes either.

## Making one

| method | what it does |
|---|---|
| [`new`](vec_new/README.md) | Empty, and it has not allocated |
| [`with_capacity`](vec_with_capacity/README.md) | Empty, but the buffer is already there |

## Adding elements

| method | what it does |
|---|---|
| [`push`](vec_push/README.md) | One on the end — amortised O(1) |
| [`push_mut`](vec_push_mut/README.md) | The same, returning a `&mut` to it |
| [`insert`](vec_insert/README.md) | One at an index, shifting the rest right |
| [`insert_mut`](vec_insert_mut/README.md) | The same, returning a `&mut` to it |
| [`append`](vec_append/README.md) | **Moves** every element of another vector in |
| [`extend_from_slice`](vec_extend_from_slice/README.md) | **Clones** every element of a slice in |
| [`extend_from_within`](vec_extend_from_within/README.md) | Clones a range of *this* vector onto its own end |

## Removing one element

| method | what it does |
|---|---|
| [`pop`](vec_pop/README.md) | The last one, as an `Option` |
| [`pop_if`](vec_pop_if/README.md) | The last one, only if it passes a test |
| [`remove`](vec_remove/README.md) | By index, preserving order — O(n) |
| [`swap_remove`](vec_swap_remove/README.md) | By index, **not** preserving order — O(1) |

## Removing many

| method | what it does |
|---|---|
| [`truncate`](vec_truncate/README.md) | Keep a prefix, drop the tail |
| [`clear`](vec_clear/README.md) | Drop everything, keep the buffer |
| [`drain`](vec_drain/README.md) | Remove a range and **hand it to you** |
| [`retain`](vec_retain/README.md) | Keep what a predicate approves, one pass |
| [`retain_mut`](vec_retain_mut/README.md) | The same, with a predicate that can also edit |
| [`extract_if`](vec_extract_if/README.md) | `retain`'s mirror: yields what it removes |
| [`splice`](vec_splice/README.md) | Replace a range with an iterator of any length |

## Removing duplicates

All three are **consecutive-only** — they collapse runs, not sets.

| method | what it does |
|---|---|
| [`dedup`](vec_dedup/README.md) | Runs of equal elements, by `PartialEq` |
| [`dedup_by`](vec_dedup_by/README.md) | Runs, by a comparison you write |
| [`dedup_by_key`](vec_dedup_by_key/README.md) | Runs, by a key you derive |

## Length and shape

| method | what it does |
|---|---|
| [`len`](vec_len/README.md) | How many elements — not bytes |
| [`is_empty`](vec_is_empty/README.md) | `len() == 0`, spelled so it reads |
| [`resize`](vec_resize/README.md) | Set the length, padding with clones of a value |
| [`resize_with`](vec_resize_with/README.md) | Set the length, padding from a closure |
| [`split_off`](vec_split_off/README.md) | Cut in two at an index, returning the tail |
| [`into_flattened`](vec_into_flattened/README.md) | `Vec<[T; N]>` → `Vec<T>`, free |

## Capacity

| method | what it does |
|---|---|
| [`capacity`](vec_capacity/README.md) | How many fit before the next reallocation |
| [`reserve`](vec_reserve/README.md) | Room for `additional` more, with slack |
| [`reserve_exact`](vec_reserve_exact/README.md) | Room for `additional` more, without slack |
| [`try_reserve`](vec_try_reserve/README.md) | `reserve`, returning `Err` instead of aborting |
| [`try_reserve_exact`](vec_try_reserve_exact/README.md) | `reserve_exact`, returning `Err` |
| [`shrink_to_fit`](vec_shrink_to_fit/README.md) | Give the unused tail back |
| [`shrink_to`](vec_shrink_to/README.md) | The same, but never below a floor |
| [`spare_capacity_mut`](vec_spare_capacity_mut/README.md) | The gap, as writable uninitialised memory |

## Views and conversions

| method | what it does |
|---|---|
| [`as_slice`](vec_as_slice/README.md) | Borrow the whole thing as `&[T]` — free |
| [`as_mut_slice`](vec_as_mut_slice/README.md) | The same, writable |
| [`into_boxed_slice`](vec_into_boxed_slice/README.md) | Consume it into an exactly-sized `Box<[T]>` |
| [`leak`](vec_leak/README.md) | Consume it into a `&'a mut [T]`, never freeing |
| [`into_iter`](vec_into_iter/README.md) | The three `IntoIterator` impls, and `iter` vs `into_iter` |

## Raw parts, and the unsafe corner

Five methods where the compiler stops helping. Each page says what the safe alternative is, because there almost always is one.

| method | what it does |
|---|---|
| [`as_ptr`](vec_as_ptr/README.md) | The buffer address, without the length |
| [`as_mut_ptr`](vec_as_mut_ptr/README.md) | The same, writable |
| [`set_len`](vec_set_len/README.md) | Set the length field — drops nothing, checks nothing |
| [`into_raw_parts`](vec_into_raw_parts/README.md) | Decompose into pointer, length, capacity |
| [`from_raw_parts`](vec_from_raw_parts/README.md) | Rebuild from those three numbers |

## What is not here

**24 unstable methods.** `Vec` carries a large nightly surface, most of it the allocator API (`new_in`, `with_capacity_in`, `from_raw_parts_in`, `allocator`, …) plus `push_within_capacity`, `pop_if`'s neighbours `try_remove` and `peek_mut`, `from_fn`, `into_chunks`, `into_array`, `try_with_capacity`, `split_at_spare_mut`, and the fallible shrinks. They are omitted because this reference is pinned to what stable 1.98 will actually compile.

**The trait impls**, except `IntoIterator`. `Vec` also gets `Clone`, `Debug`, `Hash`, `Ord`, `Extend`, `FromIterator` and a long list of `From` conversions. [`collect` into a `Vec`](../../24_Iterators/collect_into_a_vec/README.md) covers the one you will use most.

## See also

- [The `Vec`](../the_vec/README.md) — the type itself: three numbers, and how they grow
- [Arrays and slices](../arrays_and_slices/README.md) — where the other half of the methods live
- [Vec of Vecs](../vec_of_vecs/README.md) — the nested case, and when to flatten it
- [Collections](../README.md) — the six types a program is made of
- [`Vec` in the standard library ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html)
