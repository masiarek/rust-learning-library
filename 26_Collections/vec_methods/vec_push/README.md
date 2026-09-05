# `Vec::push`

[`Vec` methods](../README.md) · [Collections](../../README.md)

**Level:** reference · for working programmers

**One line:** Add one element to the end. The commonest `Vec` call there is.

```text
pub fn push(&mut self, value: T)
```

Stable since **1.0.0**. Its `const` form is still unstable.

`T` is whatever the vector holds, and on a `Vec::new()` the **first push is what decides it** — the element type is inferred backwards from the value you put in, so deleting the pushes turns the `let` above them into `error[E0282]: type annotations needed for Vec<_>`. An integer, a `String`, a struct of your own, another `Vec`: same call, no special cases.

Amortised O(1): most pushes are a write into spare capacity, and the occasional one reallocates and copies everything stored so far. Doubling is what makes *n* pushes cost O(*n*) in total rather than O(*n*²).

It takes the value **by value**, so it moves. Pushing a `String` you still want afterwards is the `error[E0382]: borrow of moved value` that everybody meets once; [`extend_from_slice`](../vec_extend_from_slice/README.md) clones instead, and `push(x.clone())` says the cost out loud. A struct of your own moves for the same reason unless it derives `Copy`, which is where most people meet that error the second time.

Pushing a `Vec` into a `Vec` moves **three words** — pointer, length, capacity, 24 bytes on a 64-bit target. The row's heap buffer is not copied and does not move, which is what makes a `Vec<Vec<T>>` cheap to build a row at a time: the allocation happened at the `vec![...]`, not at the push. [Grids and nested `Vec`s](../../vec_of_vecs/README.md) counts them.

**You cannot push while holding a reference into the vector.** The borrow checker refuses it, and the reason is the reallocation above: the buffer may move, which would leave that reference dangling. This is the rule that makes `Vec` safe where a C++ `vector` merely documents the hazard.

Panics if the new capacity would exceed `isize::MAX` bytes.

[`push_mut`](../vec_push_mut/README.md) is the same call returning a `&mut` to what it just stored, and [`insert`](../vec_insert/README.md) is the same idea at an arbitrary position — but O(n), because everything after it shifts.

## Example

<!-- source:vec_push -->
*[`vec_push.rs`](examples/vec_push.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut v = Vec::new();
    v.push("Ada");
    v.push("Ben");
    v.push("Cara");
    println!("{v:?} len {}", v.len());

    // push takes the value by value: it moves, it does not borrow.
    let name = String::from("Dana");
    v.push(&name);              // a &String coerces to &str here
    println!("{v:?}");

    // Amortised O(1): most pushes are a write, some are a reallocation.
    let mut caps = vec![];
    let mut v: Vec<u8> = Vec::new();
    for n in 0..17 {
        let before = v.capacity();
        v.push(n);
        if v.capacity() != before { caps.push((n, before, v.capacity())); }
    }
    println!("reallocations during 17 pushes:");
    for (at, from, to) in &caps { println!("  on push #{at}: cap {from} -> {to}"); }
    println!("  {} reallocations for 17 pushes", caps.len());

    // The first capacity depends on the element SIZE, not on the count:
    // 8 for one-byte elements, 4 for anything up to 1 KiB, 1 above that.
    let mut bytes: Vec<u8> = Vec::new();      bytes.push(0);
    let mut words: Vec<u64> = Vec::new();     words.push(0);
    let mut big: Vec<[u8; 2048]> = Vec::new(); big.push([0; 2048]);
    println!("first capacity: u8 {} u64 {} [u8; 2048] {}",
             bytes.capacity(), words.capacity(), big.capacity());

    // The value is moved, so this is the E0382 everyone meets once.
    let owned = String::from("moved");
    let mut names: Vec<String> = Vec::new();
    names.push(owned);
    // println!("{owned}");   // error[E0382]: borrow of moved value: `owned`
    println!("{names:?}");

    // T is decided by the FIRST push — the element type is inferred backwards
    // from what goes in. Delete the pushes and the line below is
    // error[E0282]: type annotations needed for `Vec<_>`.
    let mut points = Vec::new();
    points.push(Point { x: 1, y: 2 });
    points.push(Point { x: 3, y: 4 });
    let corner = Point { x: 5, y: 6 };
    points.push(corner);
    // println!("{corner:?}");  // error[E0382] again — a struct moves like a String
    for point in &points { println!("point ({}, {})", point.x, point.y); }

    // Pushing a Vec moves three words. The row's heap buffer is not copied and
    // does not move, which is what makes a Vec<Vec<T>> cheap to build a row at
    // a time — the allocation happened at `vec![...]`, not at the push.
    let row = vec![1, 2, 3];
    let buffer = row.as_ptr();
    let mut rows: Vec<Vec<i32>> = Vec::new();
    rows.push(row);
    rows.push(vec![4, 5, 6]);
    println!("{} words moved per row; row buffer moved: {}",
             size_of::<Vec<i32>>() / size_of::<usize>(), buffer != rows[0].as_ptr());
    for r in &rows { println!("  {r:?}"); }

    // Pushing while holding a reference into the Vec is refused at compile
    // time, because a reallocation would leave that reference dangling.
    let mut v = vec![1, 2, 3];
    let first = v[0];           // a copy, not a borrow — this is fine
    v.push(4);
    println!("{v:?} first was {first}");
}
```
<!-- /source -->

<!-- output:vec_push -->
*Verified output of [`vec_push.rs`](examples/vec_push.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
["Ada", "Ben", "Cara"] len 3
["Ada", "Ben", "Cara", "Dana"]
reallocations during 17 pushes:
  on push #0: cap 0 -> 8
  on push #8: cap 8 -> 16
  on push #16: cap 16 -> 32
  3 reallocations for 17 pushes
first capacity: u8 8 u64 4 [u8; 2048] 1
["moved"]
point (1, 2)
point (3, 4)
point (5, 6)
3 words moved per row; row buffer moved: false
  [1, 2, 3]
  [4, 5, 6]
[1, 2, 3, 4] first was 1
```
<!-- /output -->

## See also

- [`Vec::push_mut`](../vec_push_mut/README.md) — push, and keep a handle on the element
- [`Vec::pop`](../vec_pop/README.md) — the other end of the stack
- [`Vec::insert`](../vec_insert/README.md) — the same, at a chosen index
- [`Vec::extend_from_slice`](../vec_extend_from_slice/README.md) — many at once, cloned
- [`Vec::capacity`](../vec_capacity/README.md) — what the reallocations are doing
- [Grids and nested `Vec`s](../../vec_of_vecs/README.md) — what a `Vec` of `Vec`s costs once you have built one

[`Vec::push` in the standard library ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
