# `VecDeque`

**Level:** 201 · working knowledge

**One line:** A `VecDeque<T>` is a [`Vec`](../the_vec/README.md) with a fourth number — `head`, the slot the front element sits in — so taking from the front costs nothing, and the elements are allowed to wrap around the end of the buffer.

```rust
use std::collections::VecDeque;

fn main() {
    let mut jobs: VecDeque<&str> = VecDeque::from(["compile", "link", "test"]);
    jobs.push_back("package");
    while let Some(job) = jobs.pop_front() {
        println!("{job}");   // compile, then link, then test, then package
    }
}
```

`push_back` to add, `pop_front` to remove: that is the type's default job, and `extend`, `append` and `collect` all push onto the back in the same direction. Iterating goes front to back, `{:?}` prints front to back, and `VecDeque::from([…])` builds one from an array. Nothing on the outside of the type shows that it is a ring.

## Four numbers, and the fourth one is the whole idea

A `Vec` is three words on the stack. A `VecDeque` is four:

```text
        stack                                    heap
  ┌───────────┬────┐
  │ head      │  1 │ ── which slot is the front
  │ len       │  4 │
  │ ptr       │  ●─┼──────────▶ ┌─────┬─────┬─────┬─────┐
  │ capacity  │  4 │            │ 'e' │ 'b' │ 'c' │ 'd' │
  └───────────┴────┘            └─────┴─────┴─────┴─────┘
                                   ▲     ▲
                                   │     └ head — the front element
                                   └ the back element, having wrapped
```

`head` is the only addition, and it is what makes both ends cheap. Removing the front does not shift anything: it adds one to `head` and subtracts one from `len`. The freed slot is then the next one the back wraps into.

std spells those four as three fields, because one of them is a pair: [`head`, `len`, and a `RawVec` that is itself a pointer and a capacity ↗](https://doc.rust-lang.org/src/alloc/collections/vec_deque/mod.rs.html). Exactly like a `Vec`, none of them is the data — `size_of::<VecDeque<T>>()` is four words whatever `T` is.

## The elements wrap, and `as_slices` is where you see it

Three pushes, one `pop_front`, two more pushes, one more `pop_front` — into a buffer with room for four:

```text title="Excerpt from the verified run below"
   push_back a, b, c        ['a', 'b', 'c']  as_slices (['a', 'b', 'c'], [])
   pop_front, push_back d,e ['b', 'c', 'd', 'e']  as_slices (['b', 'c', 'd'], ['e'])

      physical slot    0      1      2      3
                      'e'    'b'    'c'    'd'
                               ^ head = 1, len = 4
      logical order     b      c      d      e

   One more pop_front       ['c', 'd', 'e']       as_slices (['c', 'd'], ['e'])

      physical slot    0      1      2      3
                      'e'           'c'    'd'
                                      ^ head = 2, len = 3
      logical order     c      d      e
```

Capacity is still 4 and nothing was copied. The `'e'` went into the slot `'a'` vacated, which is why the contents are in **two pieces** — and [`as_slices` ↗](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.as_slices) is the method that hands you both of them, in order, as `(&[T], &[T])`. Either piece may be empty; the split is an implementation detail of where the pushes happened, not of your data. One more `pop_front` and slot 1 falls out of use while the contents still wrap around it: a ring's elements can be non-contiguous *and* have a gap in the middle, and neither the `{:?}` output nor any index you write ever shows it.

[`make_contiguous` ↗](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.make_contiguous) is the way out. It rotates the ring so the contents no longer wrap and returns one `&mut [T]` over the lot:

```rust
use std::collections::VecDeque;

fn main() {
    let mut ring: VecDeque<char> = VecDeque::from(['d', 'a', 'c', 'b']);
    ring.make_contiguous().sort();
    println!("{ring:?}");   // ['a', 'b', 'c', 'd']
}
```

It costs O(*n*) and needs `&mut`, so it is a repair rather than an accessor — do not reach for it once per loop iteration.

## It is not a slice

A `Vec` [derefs to `[T]`](../arrays_and_slices/README.md) and inherits every slice method for free. A `VecDeque` cannot, because a slice is one contiguous run and a ring buffer is not — there is no `Deref` impl on it at all, and this is the first thing you trip over.

```rust
use std::collections::VecDeque;

fn main() {
    let mut d: VecDeque<i32> = VecDeque::from([3, 1, 2]);
    // d.sort();            // E0599: no method named `sort` found
    // let s: &[i32] = &d[..];  // E0308: expected `usize`, found `RangeFull`
    d.make_contiguous().sort();      // the one that works
    println!("{d:?}");   // [1, 2, 3]
}
```

```text title="Abridged — real rustc output for the two commented lines, whose numbers are the ones above"
error[E0599]: no method named `sort` found for struct `VecDeque<T, A>` in the current scope
  |
5 |     d.sort();
  |       ^^^^ method not found in `VecDeque<i32>`

error[E0308]: mismatched types
  |
6 |     let s: &[i32] = &d[..];
  |                        ^^ expected `usize`, found `RangeFull`
```

The second one is worth reading twice: `VecDeque` implements `Index<usize>` and `IndexMut<usize>` and no range form at all, so `d[..]` is not a slicing operation returning an empty result — it is a type error at the index itself, which is why rustc names `RangeFull` where you expected `usize`. [`range` ↗](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.range) is the method that takes a range, and it returns an iterator.

What you keep without `make_contiguous`: `iter`, `len`, `contains`, `front`, `back`, `get`, `binary_search`, `rotate_left` / `rotate_right`, `retain`, `drain`, and indexing. What you lose until you call it: everything that hands out a `&[T]` — `sort`, `windows`, `chunks`, `concat`, `join`.

## Indexing is logical, never physical

`d[0]` is the front element, wherever it physically lives. At the end of the picture above `d[0]` is `'c'`, sitting in slot 2, and `d[2]` is `'e'`, sitting in slot 0. The index you write is an offset from `head`, wrapped — you never see a slot number, and no arithmetic you do on an index needs to know the capacity.

## The trap: `as_slices().0` is not the contents

`as_slices` returns a pair, and the pair is easy to half-read. `.0` is the run from `head` to the end of the buffer, which is the *whole* contents right up until the first wrap and a **prefix** of them ever after:

```text
   step 3  as_slices ([30, 10, 50], [])
           iter()  30.00   as_slices().0  30.00
   step 4  as_slices ([10, 50], [20])
           iter()  26.67   as_slices().0  20.00  <- WRONG
```

Nothing panics, no index goes out of range, and no element is ever read twice — the sum is merely missing an element, and it starts being wrong at whichever push happens to wrap. Sum through `iter()`, or use both halves of the pair.

The mirror image is sorting *through* `make_contiguous()`: it hands back `&mut [T]` into the deque itself, so `d.make_contiguous().sort()` reorders the queue. If the front meant *oldest*, it does not any more, and the next `pop_front` evicts the wrong element. Sort a copy when the order is carrying meaning. The practice below builds both bugs.

## Growth is the same doubling, and the power-of-two buffer is gone

```text
   cap 0 -> 4 -> 4 -> 4 -> 4 -> 8 -> 8 -> 8 -> 8 -> 16
   with_capacity(9).capacity() = 9
```

Nine `push_back`s cause three reallocations, exactly as they would on a `Vec`, and [`with_capacity` ↗](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.with_capacity) gives you the number you asked for.

Older material says otherwise, and used to be right. Before Rust 1.67 the buffer was `max(capacity + 1, MINIMUM_CAPACITY + 1).next_power_of_two()` slots and `capacity()` returned `cap - 1`, because the implementation stored `head` and `tail` and had to keep one slot empty so that `head == tail` could mean *empty* rather than *full*. `with_capacity(9)` therefore allocated 16 and reported 15. [rust#102991 ↗](https://github.com/rust-lang/rust/pull/102991) replaced `head + tail` with `head + len`, which frees the last slot and drops the power-of-two rule with it.

## `Vec` or `VecDeque`?

| You are doing this | Reach for |
|---|---|
| pushing and popping the **back** only | [`Vec`](../the_vec/README.md) — one word smaller, and it derefs to a slice |
| a **queue**: add at one end, take from the other | `VecDeque` |
| a **sliding window** over a stream | `VecDeque` |
| breadth-first search, a work list, an undo ring | `VecDeque` |
| `remove(0)` in a loop | `VecDeque` — that is the O(*n*) this type deletes |
| sorting, slicing, `windows`, passing `&[T]` around | [`Vec`](../the_vec/README.md), or `make_contiguous` first |
| a **stack** | [`Vec`](../the_vec/README.md) — `push`/`pop` are already the right end |

The two convert into each other, and one direction is free: [`VecDeque::from(vec)` is guaranteed O(1) and reallocates nothing ↗](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#impl-From%3CVec%3CT,+A%3E%3E-for-VecDeque%3CT,+A%3E), because a `Vec`'s buffer is a ring whose `head` is 0. Going back, `Vec::from(deque)` never reallocates either, but rotates first if the contents wrapped — the same O(*n*) `make_contiguous` does. A `VecDeque` also compares equal to a `Vec` and to an array of the same contents, so a test does not have to convert one to assert on it.

## If you are coming from another language

- **Python.** `collections.deque`, closely: `append`/`push_back`, `appendleft`/`push_front`, `pop`/`pop_back`, `popleft`/`pop_front`. The reason both exist is the same in both languages — `list.pop(0)` and `Vec::remove(0)` are both O(*n*), and both are the mistake the deque is there to delete. Three things do not carry over. Python's `deque(maxlen=n)` has no Rust counterpart: a bounded ring is yours to write, and it is two lines (`if d.len() == n { d.pop_front(); }`), which is what the practice below does. Python lets you index a deque with `d[i]` and so does Rust, but Python also lets you *slice* one via `itertools.islice`, where Rust wants [`range` ↗](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.range) and gives you an iterator rather than a copy. And `deque.rotate(n)` is `rotate_right(n)` — note the direction: Python's positive argument rotates right, and Rust makes you name which.
- **ABAP.** There is no ring buffer and no deque; a `STANDARD TABLE` is the only sequence, so a queue is written as `APPEND` plus `DELETE itab INDEX 1`, and that `DELETE` shifts every remaining row down exactly the way `Vec::remove(0)` does. The cost is usually invisible because the table is small and the loop is I/O-bound, which is why the habit survives — and it is the habit to drop when you move a batch job to Rust. The nearest thing ABAP has to `head` is reading with an index into a table you never delete from and carrying the index yourself, which is what a ring buffer automates. `SORTED TABLE` is not the analogue either: that is [`BTreeMap`](../sorted_collections/README.md), a different question.
- **C++.** `std::deque` is the same *interface* and a different *implementation*: it is a chunked array — a table of fixed-size blocks — not one ring, which is why `std::deque` guarantees references stay valid across a `push_front` or `push_back` and `VecDeque` does not. Rust's answer to "it must not move" is not the container, it is [`Box`](../the_box/README.md) or an index. `std::queue` is an adaptor over `std::deque`; Rust has no adaptor, you just do not call `push_front`.
- **Java / C#.** `ArrayDeque<E>` is the closest match there is — also a growable ring buffer, also the recommended queue and stack, and the Javadoc's own recommendation over `Stack` as a stack and `LinkedList` as a queue. `addLast`/`pollFirst` are `push_back`/`pop_front`. The difference is what you get for free: `ArrayDeque` is a `Collection`, so the whole `Collections`/streams surface applies to it, where `VecDeque` gives you iterators and asks for `make_contiguous` before anything slice-shaped. C#'s `Queue<T>` is a ring buffer too, but one-ended by design: there is no `AddFirst`, so a double-ended queue in .NET means `LinkedList<T>` or writing the ring yourself.

---

## The verified output

<!-- output:the_vecdeque -->
*Verified output of [`the_vecdeque.rs`](examples/the_vecdeque.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The default usage: push_back to add, pop_front to remove
   run compile  still queued: ["link", "test", "package"]
   run link     still queued: ["test", "package"]
   run test     still queued: ["package"]
   run package  still queued: []
   Debug prints front to back, exactly like a Vec, and so does iter().
   Nothing on the outside of the type shows that it is a ring.

2. Four numbers on the stack, not three
   Vec<u32>            3 words   ptr, len, capacity
   VecDeque<u32>       4 words   head, len, ptr, capacity
   VecDeque<[u8; 999]> 4 words   same four — the elements are never in it
   `head` is the physical slot the front element sits in. That one extra
   number is the whole difference, and it buys an O(1) pop_front.

3. The elements are allowed to wrap, and as_slices shows it
   push_back a, b, c        ['a', 'b', 'c']  as_slices (['a', 'b', 'c'], [])
   pop_front, push_back d,e ['b', 'c', 'd', 'e']  as_slices (['b', 'c', 'd'], ['e'])
   capacity is still 4, and nothing was copied or reallocated.

      physical slot    0      1      2      3
                      'e'    'b'    'c'    'd'
                               ^ head = 1, len = 4
      logical order     b      c      d      e

   The contents are in TWO pieces, and as_slices hands you both of them:
   (['b', 'c', 'd'], ['e']). Reading only the first is the bug this page is about.

   One more pop_front       ['c', 'd', 'e']       as_slices (['c', 'd'], ['e'])

      physical slot    0      1      2      3
                      'e'           'c'    'd'
                                      ^ head = 2, len = 3
      logical order     c      d      e

   Slot 1 is free now, and the contents still wrap around it: the
   elements can be non-contiguous AND have a gap in the middle. Neither
   the Debug output nor any index you write ever shows that.

4. Indexing is logical, never physical
   ring[0] = 'c'  <- the front, which physically sits in slot 2
   ring[2] = 'e'  <- the back, which physically sits in slot 0
   front Some('c')  back Some('e')  get(9) None
   contains(&'e') = true, and iter() walks c, d, e in that order.

5. make_contiguous rotates the ring and hands back a real slice
   make_contiguous() -> ['c', 'd', 'e']   the ring rotated, one piece now
   ...and slice methods apply to it: sort_by descending -> ['e', 'd', 'c']
   as_slices is now (['e', 'd', 'c'], []) — the second piece is empty.
   It costs O(n) and needs &mut, so it is not a free `as_slice()`.

6. Growth is the same amortised doubling a Vec uses
   cap 0 -> 4 -> 4 -> 4 -> 4 -> 8 -> 8 -> 8 -> 8 -> 16
   with_capacity(9).capacity() = 9  <- exact, not rounded up
   Before Rust 1.67 that answer was 15: the buffer was rounded up to a
   power of two (16) and one slot was always left empty, so that head ==
   tail could mean "empty". Material written before 2023 describes that
   layout, and it is gone.

7. Vec and VecDeque convert into each other
   Vec -> VecDeque   [1, 2, 3, 4]   O(1), guaranteed, no reallocation
   VecDeque -> Vec   [1, 2, 3, 4]   never reallocates, but rotates first if wrapped
   a wrapped one     ['b', 'c', 'd', 'e'] in slots (['b', 'c', 'd'], ['e'])
                     -> ['b', 'c', 'd', 'e']   the O(n) rotation, done for you
   A VecDeque compares equal to a Vec and to an array of the same
   contents, so a test never has to convert one to assert on it:
   d == vec![1, 2, 3] is true, d == [1, 2, 3] is true
```
<!-- /output -->

## Practice

**The window that wrapped.** Keep a rolling window of the last three readings from a sensor — `push_back` each reading, and `pop_front` once the window is full — and print the mean after each one.

Then break it twice, on purpose. First compute the mean by summing `as_slices().0` instead of `iter()`, and find the exact reading at which the two answers part company; say why that reading and not an earlier one. Second, take the median by sorting through `make_contiguous()`, then push one more reading and compare the window against the one you would have had. Both bugs produce a plausible number and no error, which is the only reason they are worth the exercise.

Finish with the same window on a `Vec` using `remove(0)`, and say what changed and what did not — one of the two is the reason `VecDeque` exists, and the other is the reason `Vec` is still the default.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:the_vecdeque_kata -->
*[`the_vecdeque_kata.rs`](examples/the_vecdeque_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a rolling window over a `VecDeque`, and the two silent bugs.
//!
//!   rustc --edition 2024 the_vecdeque_kata.rs -o /tmp/vdk && /tmp/vdk

use std::collections::VecDeque;

const LIMIT: usize = 3;
const READINGS: [i32; 5] = [30, 10, 50, 20, 40];

/// The whole window, in logical order — front is the oldest reading.
fn mean(window: &VecDeque<i32>) -> f64 {
    window.iter().sum::<i32>() as f64 / window.len() as f64
}

/// The same sum written the way that looks obviously right and is not.
fn mean_first_slice(window: &VecDeque<i32>) -> f64 {
    let (front, _back) = window.as_slices();
    front.iter().sum::<i32>() as f64 / window.len() as f64
}

fn main() {
    println!("1. The window: push_back, and pop_front once it is full");
    let mut window: VecDeque<i32> = VecDeque::with_capacity(LIMIT);
    for r in READINGS {
        if window.len() == LIMIT {
            window.pop_front();
        }
        window.push_back(r);
        let shown = format!("{window:?}");
        println!("   reading {r:>3}   window {shown:<16} mean {:>6.2}", mean(&window));
    }
    println!("   Two O(1) operations per reading, and no element ever moves: the");
    println!("   evicted one is forgotten by bumping `head`, and the new one is");
    println!("   written into the slot it just left.");

    println!();
    println!("2. The bug: reading the ring through as_slices().0");
    let mut window: VecDeque<i32> = VecDeque::with_capacity(LIMIT);
    for (step, r) in READINGS.into_iter().enumerate() {
        if window.len() == LIMIT {
            window.pop_front();
        }
        window.push_back(r);
        let (front, back) = window.as_slices();
        let good = mean(&window);
        let bad = mean_first_slice(&window);
        let flag = if (good - bad).abs() > f64::EPSILON { "  <- WRONG" } else { "" };
        println!("   step {}  as_slices ({front:?}, {back:?})", step + 1);
        println!("           iter() {good:>6.2}   as_slices().0 {bad:>6.2}{flag}");
    }
    println!("   The first eviction is where it starts: from then on the contents wrap,");
    println!("   `.0` is only the piece up to the end of the buffer, and the sum quietly");
    println!("   drops whatever came after it. Nothing panics, no index is out of range,");
    println!("   and the number is merely wrong. Use `iter()`, `range(..)` or the");
    println!("   `(front, back)` pair — never `.0` alone.");

    println!();
    println!("3. The second bug: sorting the window in place");
    println!("   window before {window:?}   oldest reading is {:?}", window.front());
    let mut copy: Vec<i32> = window.iter().copied().collect();
    copy.sort();
    println!("   median from a copy: {}   window untouched {window:?}", copy[LIMIT / 2]);
    let mut in_place = window.clone();
    in_place.make_contiguous().sort();
    println!("   median in place:    {}   window now      {in_place:?}",
             in_place[LIMIT / 2]);
    println!("   `make_contiguous()` hands back `&mut [T]`, so sorting through it");
    println!("   reorders the deque itself — and the front is no longer the oldest");
    println!("   reading. The next eviction then drops the wrong one:");
    let mut a = window.clone();
    let mut b = in_place.clone();
    a.pop_front();
    a.push_back(60);
    b.pop_front();
    b.push_back(60);
    println!("   after reading  60:  correct {a:?}   after sorting in place {b:?}");
    println!("   Sort a copy when the deque is a queue; sort in place only when the");
    println!("   order was never carrying meaning.");

    println!();
    println!("4. The same window on a Vec, and what it costs");
    let mut v: Vec<i32> = Vec::with_capacity(LIMIT);
    for r in READINGS {
        if v.len() == LIMIT {
            v.remove(0);
        }
        v.push(r);
    }
    println!("   Vec version      {v:?}   same answers, and it derefs to a slice, so");
    println!("   `v.iter()`, `v.sort()` and `&v[..]` all work with no ceremony.");
    let before = v.as_ptr();
    let first_was = v[0];
    v.remove(0);
    println!("   remove(0): buffer pointer unchanged ({}), but v[0] went {first_was} -> {}",
             std::ptr::eq(before, v.as_ptr()), v[0]);
    println!("   That is the difference in one line. The Vec keeps the same allocation");
    println!("   and shifts every surviving element down a slot — O(n) per reading. The");
    println!("   VecDeque moves nothing and bumps an index instead. For a window of 3");
    println!("   that is noise; for a queue of a million it is the whole cost.");
    println!("   Reach for a Vec anyway when only the BACK is busy: it is one word");
    println!("   smaller, and everything on the slice page applies to it directly.");
}
```
<!-- /source -->

<!-- output:the_vecdeque_kata -->
*Verified output of [`the_vecdeque_kata.rs`](examples/the_vecdeque_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The window: push_back, and pop_front once it is full
   reading  30   window [30]             mean  30.00
   reading  10   window [30, 10]         mean  20.00
   reading  50   window [30, 10, 50]     mean  30.00
   reading  20   window [10, 50, 20]     mean  26.67
   reading  40   window [50, 20, 40]     mean  36.67
   Two O(1) operations per reading, and no element ever moves: the
   evicted one is forgotten by bumping `head`, and the new one is
   written into the slot it just left.

2. The bug: reading the ring through as_slices().0
   step 1  as_slices ([30], [])
           iter()  30.00   as_slices().0  30.00
   step 2  as_slices ([30, 10], [])
           iter()  20.00   as_slices().0  20.00
   step 3  as_slices ([30, 10, 50], [])
           iter()  30.00   as_slices().0  30.00
   step 4  as_slices ([10, 50], [20])
           iter()  26.67   as_slices().0  20.00  <- WRONG
   step 5  as_slices ([50], [20, 40])
           iter()  36.67   as_slices().0  16.67  <- WRONG
   The first eviction is where it starts: from then on the contents wrap,
   `.0` is only the piece up to the end of the buffer, and the sum quietly
   drops whatever came after it. Nothing panics, no index is out of range,
   and the number is merely wrong. Use `iter()`, `range(..)` or the
   `(front, back)` pair — never `.0` alone.

3. The second bug: sorting the window in place
   window before [50, 20, 40]   oldest reading is Some(50)
   median from a copy: 40   window untouched [50, 20, 40]
   median in place:    40   window now      [20, 40, 50]
   `make_contiguous()` hands back `&mut [T]`, so sorting through it
   reorders the deque itself — and the front is no longer the oldest
   reading. The next eviction then drops the wrong one:
   after reading  60:  correct [20, 40, 60]   after sorting in place [40, 50, 60]
   Sort a copy when the deque is a queue; sort in place only when the
   order was never carrying meaning.

4. The same window on a Vec, and what it costs
   Vec version      [50, 20, 40]   same answers, and it derefs to a slice, so
   `v.iter()`, `v.sort()` and `&v[..]` all work with no ceremony.
   remove(0): buffer pointer unchanged (true), but v[0] went 50 -> 20
   That is the difference in one line. The Vec keeps the same allocation
   and shifts every surviving element down a slot — O(n) per reading. The
   VecDeque moves nothing and bumps an index instead. For a window of 3
   that is noise; for a queue of a million it is the whole cost.
   Reach for a Vec anyway when only the BACK is busy: it is one word
   smaller, and everything on the slice page applies to it directly.
```
<!-- /output -->

</details>

---

## See also

- [`Vec`](../the_vec/README.md) — the three-number version, and where the doubling is explained
- [`Vec::remove`](../vec_methods/vec_remove/README.md) and [`swap_remove`](../vec_methods/vec_swap_remove/README.md) — the two `Vec` answers to deleting from the middle, and why neither helps at the front
- [Arrays and slices](../arrays_and_slices/README.md) — what a `&[T]` is, and therefore what `make_contiguous` has to produce
- [`slice::sort`](../slice_methods/slice_sort/README.md) — the method the deque reaches only through that call
- [`BTreeMap` and `BTreeSet`](../sorted_collections/README.md) — when what you wanted was not a queue but an order

## Sources

[`std::collections::VecDeque` ↗](https://doc.rust-lang.org/std/collections/struct.VecDeque.html), whose opening paragraph is the "`push_back` to add, `pop_front` to remove" rule this page starts from, and the [`std::collections` decision table ↗](https://doc.rust-lang.org/std/collections/index.html#when-should-you-use-which-collection), which is where the `Vec`-or-`VecDeque` question is answered in std's own words. The pre-1.67 layout is quoted from the [1.66 source ↗](https://doc.rust-lang.org/1.66.0/src/alloc/collections/vec_deque/mod.rs.html) and the change is [rust#102991 ↗](https://github.com/rust-lang/rust/pull/102991).

## Po polsku

`VecDeque<T>` to kolejka dwustronna — po polsku najczęściej „dwustronna kolejka" albo wprost „deque" — zbudowana na **buforze cyklicznym** (ang. *ring buffer*, po polsku też „bufor pierścieniowy"). Różnica wobec `Vec` mieści się w jednej liczbie: obok wskaźnika, długości i pojemności dochodzi `head`, czyli numer komórki, w której siedzi pierwszy element. Dzięki niemu zdjęcie elementu z przodu nie przesuwa niczego — wystarczy zwiększyć `head` o jeden. To jest cała różnica i cały zysk: `Vec::remove(0)` przepisuje wszystkie pozostałe elementy o jedno miejsce w dół, `VecDeque::pop_front` nie przepisuje żadnego.

Cena jest jedna i trzeba ją znać od razu: **`VecDeque` nie jest wycinkiem** (`&[T]`). `Vec` dzięki `Deref` dostaje za darmo wszystkie metody wycinka, a bufor cykliczny nie jest ciągłym obszarem pamięci, więc nie może. `d.sort()` to `E0599`, a `&d[..]` to `E0308` — `VecDeque` implementuje `Index<usize>` i nic poza tym. Metodą wyjścia jest `make_contiguous()`, która obraca pierścień tak, żeby elementy przestały się „zawijać", i oddaje jeden `&mut [T]`. Kosztuje O(*n*) i wymaga `&mut`, więc jest naprawą, a nie darmowym dostępem.

Dwie pułapki, obie ciche — nic się nie wywala, wynik jest po prostu zły. Pierwsza: `as_slices()` zwraca **parę** wycinków, bo zawartość bywa w dwóch kawałkach, a `.0` to tylko kawałek od `head` do końca bufora. Suma po `.0` jest poprawna dokładnie do pierwszego zawinięcia i błędna od tego momentu, i nikt o tym nie ostrzeże — sumuj po `iter()`. Druga: sortowanie przez `make_contiguous().sort()` przestawia **samą kolejkę**, więc jeśli „przód" znaczył „najstarszy", to po sortowaniu już nie znaczy, a następne `pop_front` usunie nie ten element, co trzeba. Sortuj kopię, jeśli kolejność coś znaczy.

I jedna uwaga historyczna, bo polskie (i nie tylko) materiały sprzed 2023 roku mówią co innego: kiedyś bufor musiał mieć rozmiar będący potęgą dwójki i jedna komórka zawsze zostawała pusta, żeby `head == tail` mogło znaczyć „pusto" — `with_capacity(9)` dawało wtedy pojemność 15. Od Rusta 1.67 implementacja trzyma `head` i `len` zamiast `head` i `tail`, więc pojemność jest dokładnie taka, o jaką prosisz.

**Szukaj po polsku:** bufor cykliczny · kolejka dwustronna · `rust VecDeque vs Vec` · `rust VecDeque make_contiguous` · `rust pop_front O(1)`
