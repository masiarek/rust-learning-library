# `Vec::append`

[`Vec` methods](../README.md) · [Collections](../../README.md)

**Level:** reference · for working programmers

**One line:** Move every element of another vector into this one, leaving it empty.

```text
pub fn append(&mut self, other: &mut Vec<T, A>)
```

Stable since **1.4.0**.

The elements are **moved**, not cloned, so `T` need not be `Clone` — that is the difference from [`extend_from_slice`](../vec_extend_from_slice/README.md), which clones out of a slice you keep.

The source is left **empty but alive**, and it keeps its allocation. That makes it cheap to refill, and it is why `append` is the right call for draining one buffer into another in a loop.

It takes `&mut Self`, so both vectors must be reachable and distinct: `v.append(&mut v)` is two mutable borrows of one value and does not compile. ([`clippy::extend_with_drain` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#extend_with_drain), warn by default, pushes `v.extend(w.drain(..))` towards this method.)

Panics if the new capacity would exceed `isize::MAX` bytes.

**"Moved, not cloned" does not mean nothing is copied.** `append` reserves in the *destination* and does one [`ptr::copy_nonoverlapping` ↗](https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html) of the whole run, then sets the source's length to zero. No `Clone::clone` runs and no per-element allocation happens — but the bytes do move, the destination may reallocate to make room, and the source's buffer is never adopted. That last part is visible in the example below: the source still reports the capacity it had.

If you want the source *consumed* rather than emptied, `v.extend(other)` takes it by value. If you want the source untouched, [`extend_from_slice`](../vec_extend_from_slice/README.md) clones.

## `append` vs `drain`

They overlap on exactly one case — moving everything out of a vector you intend to keep — and `append` is the better spelling of that one:

| | `dst.append(&mut src)` | `src.drain(range)` |
|---|---|---|
| how much | all of `src`, always | any range, including `..` |
| where it goes | the end of another `Vec<T>` | an iterator — anywhere at all |
| on the way out | nothing; it is a bulk copy | `.map()`, `.filter()`, `.rev()`, `.collect()` |
| the source | emptied, buffer kept | range removed, buffer kept |

So `drain` earns its place by being the *general* one. It can take a slice out of the middle and leave the rest in order; it can hand the removed elements to a `String`, a `HashSet`, a channel, or a function that takes an iterator; it can transform them on the way. `append` can do none of that — it only ever concatenates one whole `Vec<T>` onto another.

Read the other direction, `append` is the fast path for the case it does cover. `dst.extend(src.drain(..))` is the same result routed through the iterator protocol, which is why [`clippy::extend_with_drain` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#extend_with_drain) (`perf`, **warn** by default) rewrites it back to `append`. Clippy keeps three more lints for reaching for `drain` when something simpler exists — [`drain_collect` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#drain_collect) (warn) for `.drain(..).collect()`, and the allow-by-default [`clear_with_drain` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#clear_with_drain) and [`iter_with_drain` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#iter_with_drain) — and between them they draw the boundary: `drain` is for a **range you want back**, not for emptying, clearing, or consuming.

## Example

<!-- source:vec_append -->
*[`vec_append.rs`](examples/vec_append.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
fn main() {
    // append MOVES the elements across and leaves the source empty but alive.
    let mut a = vec![1, 2, 3];
    let mut b = vec![4, 5];
    a.append(&mut b);
    println!("a {a:?}  b {b:?}  b.capacity() {}", b.capacity() > 0);

    // That is the difference from extend_from_slice, which clones. Here the
    // elements are not cloneable at all and it still works.
    struct NotClone(u8);
    let mut xs = vec![NotClone(1)];
    let mut ys = vec![NotClone(2), NotClone(3)];
    xs.append(&mut ys);
    let tags: Vec<u8> = xs.iter().map(|n| n.0).collect();
    println!("moved {tags:?} — non-Clone values — source now len {}", ys.len());

    // It takes &mut, so both vectors have to be reachable and distinct.
    // v.append(&mut v) does not compile: two mutable borrows of one value.
    let mut v = vec!["a"];
    let mut w = vec!["b"];
    v.append(&mut w);
    println!("{v:?}");

    // The source keeps its allocation, so it is cheap to refill.
    let mut src = vec![1u8, 2, 3, 4];
    let cap_before = src.capacity();
    let mut dst = vec![];
    dst.append(&mut src);
    println!("source kept its buffer: {}", src.capacity() == cap_before);
    src.push(9);
    println!("refilled without allocating: {src:?}");

    // Three vectors into one, in order.
    let mut all = vec![];
    for mut part in [vec![1, 2], vec![3], vec![4, 5]] {
        all.append(&mut part);
    }
    println!("{all:?}");

    // Why drain exists. append takes ALL of another Vec and puts it on the end
    // of this one. drain takes a RANGE, and hands the removed elements back as
    // an iterator — so they can be transformed on the way out, and can land
    // somewhere that is not the tail of a Vec.
    let mut ids = vec![10, 20, 30, 40, 50];
    let head: String = ids.drain(..2).map(|n| n.to_string()).collect::<Vec<_>>().join("-");
    println!("drain took a range, into a String: {head:?}  left {ids:?}");

    // Spelled with the full range and extended onto another vector, it is
    // append the slow way round — clippy::extend_with_drain (perf, warn by
    // default) rewrites it back.
    let mut from = vec![1, 2, 3];
    let mut onto = vec![0];
    onto.extend(from.drain(..));
    println!("extend + drain(..) does the same as append: {onto:?}  from {from:?}");
}
```
<!-- /source -->

<!-- output:vec_append -->
*Verified output of [`vec_append.rs`](examples/vec_append.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
a [1, 2, 3, 4, 5]  b []  b.capacity() true
moved [1, 2, 3] — non-Clone values — source now len 0
["a", "b"]
source kept its buffer: true
refilled without allocating: [9]
[1, 2, 3, 4, 5]
drain took a range, into a String: "10-20"  left [30, 40, 50]
extend + drain(..) does the same as append: [0, 1, 2, 3]  from []
```
<!-- /output -->

## See also

- [`Vec::extend_from_slice`](../vec_extend_from_slice/README.md) — the cloning version, source untouched
- [`Vec::drain`](../vec_drain/README.md) — moving out a range rather than everything
- [`Vec::split_off`](../vec_split_off/README.md) — the inverse: one vector into two
- [`Vec::into_iter`](../vec_into_iter/README.md) — the other way to move elements out

[`Vec::append` in the standard library ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append)
