# What a `Vec` guarantees

**Level:** 301 · deep dive

**One line:** `Vec` publishes an unusually long list of promises about its own representation — the pointer is never null, the capacity is exact, nothing is ever stored inline — and an equally important list of things it deliberately refuses to promise, of which the growth factor and the drop order are the two people most often assume.

Most types in std tell you what they *do*. `Vec` also tells you what it *is*, because unsafe code has to be able to manipulate one correctly and because so much else is built on top of it. That is what the [Guarantees ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees) section of its documentation is for, and it is the part of that page nobody reads until something surprising happens.

This page is that section, made runnable. [The `Vec`](../the_vec/README.md) is the lesson; this is the small print underneath it.

> Everything below applies to an unqualified `Vec<T>`. `Vec` has a second, unstable generic parameter for a custom allocator, and overriding it may change the behaviour described here.

## The triplet, and the layout that is not one

A `Vec` is a *(pointer, capacity, length)* triplet — "no more, no less", as std puts it. Three words, whatever `T` is.

What is **not** guaranteed is the order of those three fields, or anything else about the layout: *"the ABI is not stable and `Vec` makes no guarantees about its memory layout (including the order of fields)."* The struct as written actually has **two** fields, `buf: RawVec<T, A>` and `len`, with the capacity living inside `RawVec` — so even the count is a description of what a `Vec` means rather than how it is spelled. Read the three numbers with [`as_ptr`](../vec_methods/vec_as_ptr/README.md), [`capacity`](../vec_methods/vec_capacity/README.md) and [`len`](../vec_methods/vec_len/README.md), and if you need the triplet as data, [`into_raw_parts`](../vec_methods/vec_into_raw_parts/README.md) is the supported door.

## The pointer is never null — which is why `Option<Vec<T>>` is free

```rust
assert_eq!(size_of::<Option<Vec<u8>>>(), size_of::<Vec<u8>>());
```

`Vec` is null-pointer-optimized: because the pointer promises never to be null, `None` can be spelled with that one forbidden bit pattern and the wrapper costs nothing. The same trick is behind `Option<Box<T>>` and `Option<&T>` — see [nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md).

## ...but it may not point at anything

Never null is not the same as always allocated. **A `Vec` allocates if and only if `size_of::<T>() * capacity() > 0`**, which is worth reading twice, because it has two independent ways of being false.

**Capacity zero.** `Vec::new()`, `vec![]`, `Vec::with_capacity(0)` and [`shrink_to_fit`](../vec_methods/vec_shrink_to_fit/README.md) on an empty vector all leave you with no allocation at all. The pointer is parked at the element type's alignment — an implementation detail, not a promise; the promise is only that it is not null. This is why `Vec::new()` is cheap enough to write in a hot path and why it works in a `static`.

**A zero-sized element.** `Vec<()>` never allocates however much you push into it, and its capacity is reported as `usize::MAX`. That breaks the shortcut people reach for — *"capacity 0 means it has not allocated"* — while the `size_of` × `capacity` rule keeps working. [ZSTs](../../15_First_Programs/the_unit_type/README.md) are the reason the rule is phrased that way rather than as a comparison against zero.

The practical consequence is the one std spells out: if you allocate through a `Vec` and then use the memory for something else, you must rebuild the `Vec` with [`from_raw_parts`](../vec_methods/vec_from_raw_parts/README.md) and drop that to free it. There is no other supported route back.

## Nothing is ever stored inline

```rust
assert_eq!(size_of::<Vec<[u8; 4096]>>(), size_of::<Vec<u8>>());
```

`Vec` will never do the "small" optimization — a few elements kept in the struct itself, as `std::string` does in most C++ implementations and as `smallvec` does deliberately. std gives two reasons, and both are about someone else's code rather than its own: the contents would stop having a stable address when the `Vec` is merely **moved**, which unsafe code cannot work around; and every single access would pay for a branch, which penalises the general case to help a special one.

So if a `Vec` has allocated, its elements are on the heap, contiguous, in order: `len` initialised ones followed by `capacity - len` logically uninitialised ones.

```text
            ptr      len  capacity
       +--------+--------+--------+
       | 0x0123 |      2 |      4 |
       +--------+--------+--------+
            |
            v
Heap   +--------+--------+--------+--------+
       |    'a' |    'b' | uninit | uninit |
       +--------+--------+--------+--------+
```

If you want the small-vector behaviour, it is a crate. That it is not in std is a decision, not an omission.

## `capacity()` is exact, and that is what makes `with_capacity` a promise

> `push` and `insert` will never (re)allocate if the reported capacity is sufficient. `push` and `insert` will (re)allocate if `len == capacity`.

Both halves matter. The first is why [`with_capacity`](../vec_methods/vec_with_capacity/README.md) is worth reaching for at all — if it were a hint you could not build anything on it. The second is why you can *predict* the reallocation rather than measure it: it happens exactly when the two numbers meet, and the number `capacity()` reports is the real one.

Two edges. **Bulk insertion methods are exempt** — [`extend`](../vec_methods/vec_extend_from_slice/README.md), [`append`](../vec_methods/vec_append/README.md), [`splice`](../vec_methods/vec_splice/README.md) and friends *may* reallocate even when they need not. And **"reallocate" does not mean "move"**: an allocator is free to grow a block in place, so the address may well be unchanged afterwards. This page's example prints capacities rather than addresses for exactly that reason — the address is the allocator's business and differs between machines.

## It never shrinks itself

[`clear`](../vec_methods/vec_clear/README.md), [`truncate`](../vec_methods/vec_truncate/README.md), [`pop`](../vec_methods/vec_pop/README.md), [`drain`](../vec_methods/vec_drain/README.md) and [`retain`](../vec_methods/vec_retain/README.md) all drop elements and keep the buffer. This is a guarantee, not an accident: *"Emptying a `Vec` and then filling it back up to the same `len` should incur no calls to the allocator."* That is the behaviour a loop that refills a buffer wants, and it is why giving memory back has to be asked for — [`shrink_to_fit`](../vec_methods/vec_shrink_to_fit/README.md) or [`shrink_to`](../vec_methods/vec_shrink_to/README.md).

## `len == capacity` makes `Box<[T]>` free in both directions

`vec![a, b, c]`, `vec![x; n]` and `Vec::with_capacity(n)` all give you *at least* the capacity you asked for — and the `vec!` macro gives exactly it, because it builds a boxed array and converts it rather than growing. When `len == capacity`, the conversion to and from [`Box<[T]>`](../vec_methods/vec_into_boxed_slice/README.md) neither reallocates nor moves the elements.

With slack it is a different operation: [`into_boxed_slice`](../vec_methods/vec_into_boxed_slice/README.md) has to shrink first, and the spare capacity does not come back when you convert the other way. That is the point of the type — a `Box<[T]>` is a `Vec` with the third number given up, one word smaller and unable to grow.

## Three things that are *not* promises

**The growth factor.** Doubling is this standard library's current strategy. std says outright that it does not guarantee any particular one, "nor when `reserve` is called", and that a non-constant factor may prove desirable. What *is* promised is amortised O(1) `push`. Write against that.

**The drop order.** Today the elements are dropped front to back. std: *"Currently, `Vec` does not guarantee the order in which elements are dropped. The order has changed in the past and may change again."* A `Drop` impl whose correctness depends on the order needs the order made explicit — pop them, or drain them.

**That removed data is erased.** It is not, and neither is it preserved. A `Vec`'s uninitialised memory is scratch space it may reuse as it likes, and a dropped buffer may be handed straight to the next allocation with your bytes still in it. Zeroing it yourself is not a fix either: the optimizer does not treat that write as a side effect and is free to delete it. **If the data is sensitive, this is a job for a crate built for it** (`zeroize` and friends), not for a `clear()` and a hope.

The mirror image of that last one is the single guarantee std makes about the excess capacity, and it is a constructive one: *"using unsafe code to write to the excess capacity, and then increasing the length to match, is always valid."* That is the contract [`spare_capacity_mut`](../vec_methods/vec_spare_capacity_mut/README.md) and [`set_len`](../vec_methods/vec_set_len/README.md) exist to be used under.

---

## The verified output

<!-- output:vec_guarantees -->
*Verified output of [`vec_guarantees.rs`](examples/vec_guarantees.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three numbers, and the order of them is not yours to know
   ptr/len/cap: len 3 cap 3
   `Vec` is and always will be a (pointer, capacity, length) triplet —
   no more, no less. The order of the fields is unspecified, and std
   says outright that the ABI is not stable. Read them with the
   methods; never assume a layout.

2. The pointer is never null, so Option<Vec<T>> is free
   size_of::<Option<Vec<u8>>>() == size_of::<Vec<u8>>(): true
   That is the null-pointer optimization: `None` is spelled with the
   one bit pattern the pointer promises never to hold, so the wrapper
   costs nothing.

3. ...but the pointer need not point at an allocation
   never allocated — new() true vec![] true with_capacity(0) true shrunk-empty true
   with_capacity(4) did allocate: true
   The rule is exact: a Vec allocates if and only if
   size_of::<T>() * capacity() > 0.

4. A zero-sized element never allocates, however many you push
   pushed 1000 units: still unallocated true, capacity is usize::MAX true
   Note what this breaks: "capacity() == 0 means it has not
   allocated" is false here. The size*capacity rule still holds.

5. Nothing is ever stored inline
   size_of::<Vec<[u8; 4096]>>() == size_of::<Vec<u8>>(): true
   `Vec` will never do the "small" optimization some C++ string and
   vector implementations do, where a few elements live in the struct
   itself. Two reasons std gives: the contents would stop having a
   stable address when the Vec is merely moved, and every access would
   pay for a branch. If you want that, it is a crate, not the std type.

6. capacity() is exact, so you can predict every reallocation
   push #0: len 0 of cap 3  ->  cap 3   (len == cap? false)
   push #1: len 1 of cap 3  ->  cap 3   (len == cap? false)
   push #2: len 2 of cap 3  ->  cap 3   (len == cap? false)
   push #3: len 3 of cap 3  ->  cap 6   (len == cap? true)
   `push` and `insert` reallocate when len == capacity and never
   otherwise. The reported capacity is completely accurate and can be
   relied on — which is what makes `with_capacity` a promise rather
   than a hint. Bulk insertions are the carve-out: std reserves the
   right to reallocate in those even when it need not.

7. It never shrinks itself
   after clear(): len 0 cap 64
   after shrink_to_fit(): len 0 cap 0
   Emptying a Vec and refilling it to the same length should cost the
   allocator nothing — which is the behaviour you want in a loop, and
   the reason handing memory back has to be asked for.

8. When len == capacity, Box<[T]> is free in both directions
   vec![1,2,3]: len 3 cap 3 — equal, so into_boxed_slice()
   neither reallocates nor moves the elements. Back again: len 3 cap 3
   with slack (len 3 of cap 10), the conversion has to shrink first,
   and the spare capacity is gone for good: len 3 cap 3

9. The one thing you may do to the spare capacity
   wrote into the excess capacity, then set_len: "abcd"

10. Three things that are NOT promises
   The growth factor. Doubling is this std's current choice; the only
   promise is amortised O(1) push.
   The drop order — today it is front to back: 1 2 3 
   ...but std says the order has changed before and may change again.
   And erasure. Removed elements are neither overwritten nor preserved;
   the freed buffer may be handed to the next allocation as it stands.
   Zeroing it yourself is not reliable either — the optimizer is free to
   delete a write nothing reads. Use a crate built for the job.
```
<!-- /output -->

---

## See also

- [The `Vec`](../the_vec/README.md) — the lesson this is the small print for
- [`Vec` methods](../vec_methods/README.md) — one page per method, including all five of the unsafe ones named above
- [`Vec::capacity`](../vec_methods/vec_capacity/README.md) — the first non-zero capacity depends on the element size, and `vec![]` does not follow the rule
- [Arrays and slices](../arrays_and_slices/README.md) — the borrowed view, which is this picture minus the capacity
- [`Box`](../the_box/README.md) — the one-element case, and `Box<[T]>` the exactly-sized one
- [Stack and heap](../../18_Ownership/stack_and_heap/README.md) — what the pointer points at, and what it costs to follow
- [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) — the four extra powers, and why this list exists for people using them
- [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) — the optimization that makes `Option<Vec<T>>` cost nothing

## Sources

[`std::vec::Vec` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html), whose *Guarantees* section is the authority for every promise quoted here; the layout diagram is std's own. Checked against the pinned toolchain, Rust 1.98.0.

## Po polsku

`Vec` jest jednym z niewielu typów w bibliotece standardowej, który obiecuje coś o **swojej własnej reprezentacji**, a nie tylko o zachowaniu — bo kod w `unsafe` musi umieć nim poprawnie manipulować. Trzy słowa: wskaźnik, pojemność, długość. Kolejność pól nie jest jednak żadną gwarancją i ABI nie jest stabilne, więc czyta się je metodami, nigdy przez rzutowanie.

Dwie rzeczy zaskakują najczęściej. **Wskaźnik nigdy nie jest pusty** (`null`) — i właśnie dlatego `Option<Vec<T>>` nie kosztuje ani jednego bajtu więcej niż sam `Vec` — ale „nigdy pusty" to nie to samo co „zawsze zaalokowany": `Vec` alokuje wtedy i tylko wtedy, gdy `size_of::<T>() * capacity() > 0`, więc ani `Vec::new()`, ani wektor typu zerowej wielkości nie mają za sobą żadnej alokacji. I **pojemność jest dokładna**: `push` przealokuje dokładnie wtedy, gdy `len == capacity`, i nigdy wcześniej — na tym opiera się sens `with_capacity`. Uwaga na jedno słowo: „przealokować" nie znaczy „przenieść", bo alokator może powiększyć blok w miejscu.

Trzy rzeczy natomiast **nie** są obiecane, a bywają zakładane: współczynnik wzrostu (podwajanie to dzisiejszy wybór tej implementacji, gwarancją jest tylko zamortyzowane O(1)), kolejność wywoływania `Drop` na elementach, oraz to, że usunięte dane zostaną wymazane. To ostatnie ma znaczenie przy danych wrażliwych — samo `clear()` niczego nie kasuje, a ręczne zerowanie optymalizator ma prawo usunąć, więc potrzebna jest biblioteka napisana do tego zadania.

**Szukaj po polsku:** gwarancje `Vec` · optymalizacja niepustego wskaźnika · typ zerowej wielkości · pojemność a długość · `rust Vec guarantees` · `rust null pointer optimization`
