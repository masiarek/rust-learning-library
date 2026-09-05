# `Vec::extend_from_slice`

[`Vec` methods](../README.md) · [Collections](../../README.md)

**Level:** reference · for working programmers

**One line:** Clone every element of a slice onto the end.

```text
pub fn extend_from_slice(&mut self, other: &[T])
```

Stable since **1.6.0**.

`T: Clone`, not `Copy` — `String`, `Vec` and any type with a `Clone` impl all work.

The source is a `&[T]` and stays yours. A `&Vec<T>` coerces to `&[T]`, so this is how you concatenate two vectors you both want to keep.

Panics if the new capacity exceeds `isize::MAX` bytes. Reserving the room first with [`try_reserve`](../vec_try_reserve/README.md) makes that allocation fallible, since the call then has nothing left to allocate.

For bytes there is a shorthand worth knowing: `buf.extend_from_slice(b"hello")` and `buf.extend_from_slice(s.as_bytes())` are how a `Vec<u8>` gets built.

When you do **not** need the source afterwards, [`append`](../vec_append/README.md) moves instead of cloning and drops the `Clone` bound with it.

## `extend_from_slice` vs `extend`

std's own note is that this method *is* `extend`, "except that it also works with slice elements that are `Clone` but not `Copy`" — and that it may be deprecated if Rust ever gets specialization.

That exception is the reason it exists: `Vec` has two `Extend` impls, and the one taking references demands `Copy`.

```text
impl<'a, T: Copy + 'a, A: Allocator> Extend<&'a T> for Vec<T, A>
impl<T, A: Allocator> Extend<T> for Vec<T, A>
```

So `v.extend(&other)` compiles for a `Vec<i32>` and not for a `Vec<String>`:

```rust,compile_fail
fn main() {
    let src = vec![String::from("a")];
    let mut dst: Vec<String> = Vec::new();
    dst.extend(&src);
    // error[E0271]: type mismatch resolving
    //               `<&Vec<String> as IntoIterator>::Item == String`
    //               expected `String`, found `&String`
}
```

Swap in `dst.extend_from_slice(&src)` and it compiles: that call asks only for `Clone`.

`extend_from_slice` asks only for `Clone`, so it takes both. Spelling the iterator out — `dst.extend(src.iter().cloned())` — is the same work said the long way.

It also reserves **once** for the whole slice. `extend` can only pre-reserve when the iterator reports its length, and a `filter` chain does not.

### The element types must match exactly

`&[T]` means *that* `T`. A `Vec<String>` will not take a `&[&str]`, however convertible the elements look:

```rust,compile_fail
fn main() {
    let slice: &[&str] = &["apple", "banana", "cherry"];
    let mut v: Vec<String> = Vec::new();
    v.extend_from_slice(slice);
    // error[E0308]: mismatched types
    //               expected `&[String]`, found `&[&str]`
}
```

There is nowhere for a conversion to happen: cloning a `&str` gives a `&str`. Use `extend` and convert on the way in.

```rust
fn main() {
    let slice: &[&str] = &["apple", "banana", "cherry"];
    let mut v: Vec<String> = Vec::new();
    v.extend(slice.iter().map(|&s| s.to_string()));
    assert_eq!(v, ["apple", "banana", "cherry"]);
}
```

## Example

<!-- source:vec_extend_from_slice -->
*[`vec_extend_from_slice.rs`](examples/vec_extend_from_slice.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
fn main() {
    // Clones each element out of a slice you keep.
    let mut v = vec![1, 2];
    let more = [3, 4, 5];
    v.extend_from_slice(&more);
    println!("{v:?}  source still usable: {more:?}");

    // T: Clone is the requirement — not Copy. Strings work.
    let mut names = vec![String::from("Ada")];
    let rest = vec![String::from("Ben"), String::from("Cara")];
    names.extend_from_slice(&rest);
    println!("{names:?}  rest {rest:?}");

    // extend(&v) goes through Extend<&T>, which demands T: Copy — so it takes
    // numbers and refuses the Strings above:
    //     names.extend(&rest);   // error[E0271]: expected `String`, found `&String`
    // extend_from_slice asks only for Clone, which is why the method exists.
    let nums = vec![1, 2, 3];
    let mut copied: Vec<i32> = Vec::new();
    copied.extend(&nums);
    println!("extend(&v) needs Copy: {copied:?}");

    // The element types must match exactly. A Vec<String> will not take a
    // &[&str], because cloning a &str gives a &str, not a String:
    //     owned.extend_from_slice(words);   // error[E0308]: expected `&[String]`
    // Convert on the way in with extend instead.
    let words: &[&str] = &["apple", "banana", "cherry"];
    let mut owned: Vec<String> = Vec::new();
    owned.extend(words.iter().map(|&s| s.to_string()));
    println!("{owned:?} from {words:?}");

    // A &Vec<T> coerces to &[T], so this is how you concatenate two vectors
    // you both want to keep.
    let a = vec![1, 2];
    let b = vec![3, 4];
    let mut joined = Vec::with_capacity(a.len() + b.len());
    joined.extend_from_slice(&a);
    joined.extend_from_slice(&b);
    println!("{joined:?} from {a:?} and {b:?}");

    // It reserves once for the whole slice, which extend() from an iterator
    // can only do when the iterator knows its own length.
    let mut v: Vec<u8> = Vec::new();
    v.extend_from_slice(&[0; 100]);
    println!("100 bytes in one go: len {} cap {}", v.len(), v.capacity());

    // For bytes and &str there is a shorthand worth knowing.
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"hello ");
    buf.extend_from_slice("world".as_bytes());
    println!("{}", String::from_utf8(buf).unwrap());
}
```
<!-- /source -->

<!-- output:vec_extend_from_slice -->
*Verified output of [`vec_extend_from_slice.rs`](examples/vec_extend_from_slice.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
[1, 2, 3, 4, 5]  source still usable: [3, 4, 5]
["Ada", "Ben", "Cara"]  rest ["Ben", "Cara"]
extend(&v) needs Copy: [1, 2, 3]
["apple", "banana", "cherry"] from ["apple", "banana", "cherry"]
[1, 2, 3, 4] from [1, 2] and [3, 4]
100 bytes in one go: len 100 cap 100
hello world
```
<!-- /output -->

## See also

- [`Vec::append`](../vec_append/README.md) — the moving version, which empties the source
- [`Vec::extend_from_within`](../vec_extend_from_within/README.md) — the same, from this vector's own elements
- [`Vec::push`](../vec_push/README.md) — one element at a time
- [`Vec::reserve`](../vec_reserve/README.md) — what this does for you automatically

[`Vec::extend_from_slice` in the standard library ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend_from_slice)
