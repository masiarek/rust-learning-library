# `Vec`

**Level:** 101 → 201 · for newcomers

**One line:** A `Vec<T>` is three numbers on the stack — pointer, length, capacity — and one allocation on the heap that it grows by doubling.

```rust
fn main() {
    let mut scores: Vec<u32> = Vec::new();
    scores.push(5);
    scores.push(3);
    scores.push(0);
    println!("{scores:?} len {} cap {}", scores.len(), scores.capacity());
    // [5, 3, 0] len 3 cap 4
}
```

`vec![5, 3, 0]` is the same thing in one line, and `(1..=100).collect()` is the same thing when the values come from somewhere.

## Three numbers, and none of them is the data

`size_of::<Vec<T>>()` is 24 on a 64-bit machine **whatever `T` is** — a `Vec<[u8; 999]>` is also 24 bytes. The elements are not in the `Vec`; the `Vec` is a receipt for them.

| field | says |
|---|---|
| pointer | where the elements are |
| **len** | how many are initialised |
| **capacity** | how many fit before the next allocation |

`len` is what you almost always want. `capacity` matters exactly once: when you are about to fill it.

## Growth is amortised doubling, and you can watch it

Nine pushes into a `Vec::new()` cause three reallocations — capacity goes 0 → 4 → 8 → 16 — and each one copies everything already stored into the new buffer. Doubling is what makes pushing *n* items cost O(*n*) in total instead of O(*n*²); the exact sequence is this std's choice, not a promise in the language.

If you know the size, say so:

```rust
fn main() {
    let mut sized: Vec<u32> = Vec::with_capacity(9);
    for n in 1..=9 { sized.push(n); }
    println!("cap {}", sized.capacity());   // 9
}
```

One allocation instead of three, and nothing copied. `collect()` does this for you when the iterator knows its own length — a range does, a `filter` does not.

## It derefs to a slice, so slice methods just work

Everything on [arrays and slices](../arrays_and_slices/README.md) applies here: `first`, `last`, `contains`, `sort`, `windows`, `iter`, and indexing that panics while `.get` returns `Option`. That is also the argument for `&[T]` over `&Vec<T>` in a signature — a `&Vec<u32>` coerces to `&[u32]` at the call site, so taking the slice costs the caller nothing and accepts three more kinds of argument.

## Removing: the one that keeps order, and the one that is fast

```rust
fn main() {
    let mut a: Vec<char> = "abcdef".chars().collect();
    let mut b = a.clone();
    a.remove(1);        // ['a', 'c', 'd', 'e', 'f']  — everything after shifts down
    b.swap_remove(1);   // ['a', 'f', 'c', 'd', 'e']  — the last element fills the hole
    println!("{a:?} {b:?}");
}
```

O(*n*) against O(1). `swap_remove` is the right answer surprisingly often — but if the `Vec` is a ranking, it has silently reordered your results.

To delete many, `retain` is one pass with one shift per survivor:

```rust
fn main() {
    let mut scores = vec![5, 3, 0, 4, 0, 2];
    scores.retain(|&s| s > 0);
    println!("{scores:?}");   // [5, 3, 4, 2]
}
```

## The trap: deleting inside an index loop

```rust
fn main() {
    let mut v = vec![5u32, 0, 3, 0, 4];
    let mut i = 0;
    while i < v.len() {
        if v[i] == 0 { v.remove(i); } else { i += 1; }   // note: no i += 1 after a removal
    }
    println!("{v:?}");   // [5, 3, 4]
}
```

Increment `i` after the `remove` and every element following a deleted one is skipped — so `[0, 0]` leaves one zero behind. Nothing warns, because every index used is valid. Writing `for i in 0..v.len()` is worse still: the bound is computed once, and the loop then indexes past the shrinking end.

Rust does stop you doing this *while iterating*: `for x in &v { v.remove(…) }` is a borrow-check error, not a run-time surprise. The index loop is the version that escapes that check, which is exactly why `retain` exists.

## If you are coming from another language

- **Python.** `Vec` is `list`, closely: `push`/`append`, `pop`/`pop`, `remove(i)`/`del xs[i]`, `retain`/a comprehension. The differences are the ones type and ownership bring. A `Vec<T>` holds one type, so there is no `[1, "two", 3.0]`; `xs[i]` panics rather than raising something you can catch, and `.get(i)` is the version that returns `Option`; and `v2 = v1` **moves** where Python aliases, so the double-mutation bug that Python's shared reference causes is a compile error. The one habit to unlearn is `for i in range(len(xs))` — Rust's `for x in &v` is both faster and impossible to get wrong, and the index form is where the deletion trap above lives. Python's `list.pop(0)` is O(*n*) for the same reason `Vec::remove(0)` is; `VecDeque` is Rust's `collections.deque`.
- **ABAP.** A `Vec<T>` is a `STANDARD TABLE OF ty` and the correspondence is close enough to be useful: `APPEND` is `push`, `DELETE itab INDEX i` is `remove`, `READ TABLE … INDEX` is `get`, `LOOP AT` is `for x in &v`. Two things transfer directly. `DELETE itab WHERE cond` is `retain` with the condition negated — one statement, one pass, and the same reason to prefer it over deleting in a loop. And the ABAP rule that you must not `DELETE` from the table you are looping over is Rust's borrow checker, enforced by convention there and by the compiler here. What ABAP has that `Vec` does not is the sorted and hashed table kinds with their key declarations; in Rust those are separate types — `BTreeMap` and `HashMap` — rather than a property of the table.
- **C++.** `std::vector` exactly, down to the growth strategy and `reserve` being `with_capacity`. `swap_remove` is the idiom C++ programmers write by hand as `std::swap(v[i], v.back()); v.pop_back();`. Iterator invalidation is the same hazard and Rust turns it into a compile error.
- **Java / C#.** `ArrayList<T>` / `List<T>`, with `ensureCapacity` / the capacity constructor. `ConcurrentModificationException` is thrown at run time for the case Rust rejects at compile time.

---

## The verified output

<!-- output:the_vec -->
*Verified output of [`the_vec.rs`](examples/the_vec.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three numbers: pointer, length, capacity
   size_of::<Vec<u32>>()  = 24  (three usize)
   size_of::<Vec<[u8; 999]>>() = 24  — the same three, whatever T is
   The elements are not in the Vec. The Vec is a receipt for them.

2. Growth is amortised doubling, and you can watch it
   Vec::new()            len 0 cap 0
   push(1) reallocated   len 1 cap 0 -> 4
   push(5) reallocated   len 5 cap 4 -> 8
   push(9) reallocated   len 9 cap 8 -> 16
   after 9 pushes        len 9 cap 16
   Nine pushes, three allocations. Doubling is why pushing n items
   costs O(n) in total rather than O(n^2), and the exact sequence is
   this std's choice, not a promise in the language.

3. If you know the size, say so
   with_capacity(9): cap 9 -> 9 — no reallocation at all
   Same nine values, one allocation instead of three, and no copying
   of the old contents.

4. A Vec derefs to a slice, so slice methods just work
   total(&v) = 45 — `total` takes &[u32] and was handed a &Vec<u32>
   v.first() = Some(1), v.contains(&5) = true
   v.iter().rev().take(3): [9, 8, 7]
   Write &[T] in a signature and both callers work. Write &Vec<T>
   and you have refused arrays, slices and everything borrowed.

5. Removing: the one that keeps order, and the one that is fast
   remove(1)      -> b, left ['a', 'c', 'd', 'e', 'f']   (everything after shifts down)
   swap_remove(1) -> b, left ['a', 'f', 'c', 'd', 'e']   (the last element fills the hole)
   O(n) versus O(1). If you are about to sort anyway, take the O(1).
   retain(|s| s > 0) on [5, 3, 0, 4, 0, 2] -> [5, 3, 4, 2]
   One pass, one shift per survivor — not one `remove` per zero.
```
<!-- /output -->

## Practice

**Count the reallocations, then delete them.** Push a hundred numbers into a `Vec::new()`, and count how many times the capacity changed and how many elements were copied in total — measure it in the program, from `len()` just before each growth, rather than working it out on paper. Then do the same with `Vec::with_capacity(100)` and with `(1..=100).collect()`.

Two questions the numbers raise. `truncate(3)` on the hundred-element `Vec` — what does capacity become, and why is that the behaviour you want? And `remove(1)` versus `swap_remove(1)` on a list of names: both are one line and one is a bug, so say which and under what assumption about the caller.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:the_vec_kata -->
*[`the_vec_kata.rs`](examples/the_vec_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: count the reallocations, then delete them.
//!
//!   rustc --edition 2024 the_vec_kata.rs -o /tmp/vk && /tmp/vk

/// Reads a tally into a fresh Vec the naive way, reporting every reallocation.
fn grow_naively(rows: &[u32]) -> (Vec<u32>, usize, usize) {
    let mut out = Vec::new();
    let mut reallocations = 0;
    let mut copied = 0;
    for &r in rows {
        let before = out.capacity();
        let filled = out.len();
        out.push(r);
        if out.capacity() != before {
            reallocations += 1;
            copied += filled;   // everything already stored is moved to the new buffer
        }
    }
    (out, reallocations, copied)
}

/// The same, told how many rows are coming.
fn grow_sized(rows: &[u32]) -> (Vec<u32>, usize, usize) {
    let mut out = Vec::with_capacity(rows.len());
    let mut reallocations = 0;
    let mut copied = 0;
    for &r in rows {
        let before = out.capacity();
        let filled = out.len();
        out.push(r);
        if out.capacity() != before {
            reallocations += 1;
            copied += filled;
        }
    }
    (out, reallocations, copied)
}

fn main() {
    let rows: Vec<u32> = (1..=100).collect();

    println!("1. One hundred pushes");
    let (naive, n1, copied1) = grow_naively(&rows);
    let (sized, n2, copied2) = grow_sized(&rows);
    println!("   Vec::new()          -> {} items, {n1} reallocations, cap {}", naive.len(), naive.capacity());
    println!("   with_capacity(100)  -> {} items, {n2} reallocations, cap {}", sized.len(), sized.capacity());
    println!("   Every reallocation copies everything already stored: {copied1} u32s");
    println!("   moved, against {copied2} for the sized version. The counting is in");
    println!("   the program — `out.len()` just before each growth, summed.");

    println!();
    println!("2. collect() already knew");
    let collected: Vec<u32> = (1..=100).collect();
    println!("   (1..=100).collect() -> {} items, cap {}", collected.len(), collected.capacity());
    println!("   A range knows its own length, so collect asked once and got it");
    println!("   right. That is `size_hint`, and it is why collect usually beats");
    println!("   a hand-written push loop without you doing anything.");

    println!();
    println!("3. Shrinking does not give the memory back");
    let mut v = collected.clone();
    v.truncate(3);
    println!("   after truncate(3): len {} cap {}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("   after shrink_to_fit(): len {} cap {}", v.len(), v.capacity());
    println!("   `clear` and `truncate` drop the elements and keep the buffer —");
    println!("   which is the behaviour you want in a loop that refills it.");

    println!();
    println!("4. Two removals, one of which is a bug");
    let names = ["Ada", "Ben", "Cara", "Dan", "Eve"];
    let mut keep_order: Vec<&str> = names.to_vec();
    let mut fast: Vec<&str> = names.to_vec();
    keep_order.remove(1);
    fast.swap_remove(1);
    println!("   remove(1)      -> {keep_order:?}");
    println!("   swap_remove(1) -> {fast:?}");
    println!("   Identical cost story, opposite guarantees. If this Vec is a");
    println!("   ranking, swap_remove has silently reordered the results.");

    println!();
    println!("5. Deleting while iterating, the three ways");
    let start = vec![5u32, 0, 3, 0, 4];
    let mut by_retain = start.clone();
    by_retain.retain(|&s| s != 0);
    let by_filter: Vec<u32> = start.iter().copied().filter(|&s| s != 0).collect();
    let mut by_index = start.clone();
    let mut i = 0;
    while i < by_index.len() {
        if by_index[i] == 0 {
            by_index.remove(i);
        } else {
            i += 1;
        }
    }
    println!("   retain            -> {by_retain:?}   in place, one pass");
    println!("   filter().collect  -> {by_filter:?}   a new Vec, borrows the old");
    println!("   index loop        -> {by_index:?}   correct only because `i` is");
    println!("   not incremented after a removal — the classic off-by-one is to");
    println!("   write a `for i in 0..len` here and skip every element after a");
    println!("   deleted one. Rust will not stop you: the indices are all valid.");
}
```
<!-- /source -->

<!-- output:the_vec_kata -->
*Verified output of [`the_vec_kata.rs`](examples/the_vec_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One hundred pushes
   Vec::new()          -> 100 items, 6 reallocations, cap 128
   with_capacity(100)  -> 100 items, 0 reallocations, cap 100
   Every reallocation copies everything already stored: 124 u32s
   moved, against 0 for the sized version. The counting is in
   the program — `out.len()` just before each growth, summed.

2. collect() already knew
   (1..=100).collect() -> 100 items, cap 100
   A range knows its own length, so collect asked once and got it
   right. That is `size_hint`, and it is why collect usually beats
   a hand-written push loop without you doing anything.

3. Shrinking does not give the memory back
   after truncate(3): len 3 cap 100
   after shrink_to_fit(): len 3 cap 3
   `clear` and `truncate` drop the elements and keep the buffer —
   which is the behaviour you want in a loop that refills it.

4. Two removals, one of which is a bug
   remove(1)      -> ["Ada", "Cara", "Dan", "Eve"]
   swap_remove(1) -> ["Ada", "Eve", "Cara", "Dan"]
   Identical cost story, opposite guarantees. If this Vec is a
   ranking, swap_remove has silently reordered the results.

5. Deleting while iterating, the three ways
   retain            -> [5, 3, 4]   in place, one pass
   filter().collect  -> [5, 3, 4]   a new Vec, borrows the old
   index loop        -> [5, 3, 4]   correct only because `i` is
   not incremented after a removal — the classic off-by-one is to
   write a `for i in 0..len` here and skip every element after a
   deleted one. Rust will not stop you: the indices are all valid.
```
<!-- /output -->

</details>

---

## See also

- [Arrays and slices](../arrays_and_slices/README.md) — the type `Vec` derefs to, and where its methods actually live
- [`Box`](../the_box/README.md) — one value on the heap, where `Vec` is many
- [Stack and heap](../../18_Ownership/stack_and_heap/README.md) — what the pointer points at, and what it costs to follow
- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — why `let v2 = v1;` leaves `v1` unusable
- [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) — the three doors onto a `Vec`, and which one consumes it
- [Building a `String`](../../14_Strings/building_a_string/README.md) — the same capacity story, for text

## Sources

[Std library types: Vectors ↗](https://doc.rust-lang.org/rust-by-example/std/vec.html) in Rust by Example, and [`std::vec::Vec` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html), whose *Capacity and reallocation* section is the authority for everything this page says about growth.
