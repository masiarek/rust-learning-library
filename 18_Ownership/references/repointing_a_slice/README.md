# Re-pointing the caller's slice: `&mut &mut [T]`

[References](../README.md) › **Re-pointing a slice**

**Level:** 201 · working knowledge

**One line:** `t: &mut &mut [i32]` is a `&mut` to the caller's slice *reference*, so the function can move where the caller's view starts and ends — which a plain `t: &mut [i32]` can only do to its own copy.

```rust
fn skip_first(t: &mut &mut [i32]) {
    let whole = std::mem::take(t);   // the caller's &mut [i32], by value; `&mut []` left behind
    *t = &mut whole[1..];            // hand back a shorter one
}

fn main() {
    let mut data = [1, 2, 3];
    let mut view: &mut [i32] = &mut data;
    skip_first(&mut view);
    println!("{view:?}");            // [2, 3]
}
```

`data` still holds three elements. What changed is `view`: a slice reference is two words, an address and a length ([the second word](../../../14_Strings/str_is_unsized/README.md#the-family-and-the-second-word)), and `skip_first` rewrote both in the caller's variable.

## Three parameters, three reaches

| parameter | writes elements | moves the caller's view | grows the collection |
|---|---|---|---|
| `t: &mut [i32]` | yes | no — only its own copy | no |
| `t: &mut &mut [i32]` | yes | **yes** | no |
| `v: &mut Vec<i32>` | yes | — | yes, `v.push(4)` |

A slice has no `push`: `t.push(4)` on a `&mut [i32]` is `E0599`, *no method named `push` found for mutable reference `&mut [i32]`*. Growing needs the `Vec`; re-slicing a slice only ever narrows it.

## A plain `&mut [i32]` re-points its own copy

```rust
fn rebind_local(mut t: &mut [i32]) {
    t[0] = 99;          // writes the caller's data
    t = &mut t[1..];    // re-points the local `t`
    t[0] = 42;          // writes data[1]
}

fn main() {
    let mut data = [1, 2, 3];
    let view: &mut [i32] = &mut data;
    rebind_local(view);
    println!("{view:?}");   // [99, 42, 3]
}
```

Both writes reach the caller. The re-point does not: `t` is a parameter, a local that received the caller's (address, length) pair [by reborrow](../../reborrowing/README.md), and assigning to it changes the local. To change the caller's pair, the function needs a `&mut` to it — one more layer.

## Why not `*t = &mut t[1..]`

```text title="Abridged — real rustc output for skip_first_naive.rs"
error: lifetime may not live long enough
 --> skip_first_naive.rs:2:5
  |
1 | fn skip_first(t: &mut &mut [i32]) {
  |                  -    - let's call the lifetime of this reference `'2`
  |                  |
  |                  let's call the lifetime of this reference `'1`
2 |     *t = &mut t[1..];
  |     ^^^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
  |
help: consider introducing a named lifetime parameter
  |
1 | fn skip_first<'a>(t: &'a mut &'a mut [i32]) {
  |              ++++     ++      ++
```

`'1` is the outer borrow, the `&mut view` at the call, which only has to cover the call; `'2` is the caller's slice reference, which lives on after it. `t[1..]` reaches the elements *through* `t`, so `&mut t[1..]` is a reborrow and lasts no longer than `'1` — and `*t` must hold something that lasts `'2`. A `&mut` reached through another `&mut` cannot outlive the outer one; [what `&'a T` claims](../../what_a_reference_claims/README.md#the-direction-reverses-behind-mut) has the reason.

[`std::mem::take` ↗](https://doc.rust-lang.org/std/mem/fn.take.html) is the way round. `pub fn take<T>(dest: &mut T) -> T where T: Default` moves the caller's `&'2 mut [i32]` out of `*t` and leaves the default in its place — `impl<T> Default for &mut [T]` is the empty slice. `whole` is then the function's own `&'2 mut [i32]`, not a reborrow through `t`, so `&mut whole[1..]` lasts `'2` as well. This is the [`mem::take` move](../../assignment_is_a_drop/README.md#through-a-mut-and-why-memreplace-exists): the hole is filled in the same call that empties it.

## rustc's `help:` compiles, and breaks the caller

Accept the suggestion — `fn skip_first<'a>(t: &'a mut &'a mut [i32])` — and the function compiles. The call site does not:

```text title="Abridged — real rustc output for skip_first_one_lifetime.rs"
error[E0502]: cannot borrow `view` as immutable because it is also borrowed as mutable
 --> skip_first_one_lifetime.rs:9:16
  |
8 |     skip_first(&mut view);
  |                --------- mutable borrow occurs here
9 |     println!("{view:?}");
  |                ^^^^
  |                |
  |                immutable borrow occurs here
  |                mutable borrow later used here
```

One `'a` for both layers says the borrow of `view` lasts as long as the slice reference inside it, so `&mut view` stays live for the rest of `view`'s life and nothing can read `view` again. The two lifetimes need to stay different, and `mem::take` is what lets them.

## Handing out an element: name the inner lifetime

```rust
fn pop_front<'a>(t: &mut &'a mut [i32]) -> Option<&'a mut i32> {
    let (first, rest) = std::mem::take(t).split_first_mut()?;
    *t = rest;
    Some(first)
}
```

`'a` is the caller's slice lifetime, so each popped element outlives the call and two can be held at once. The example pops `a` and `b` from `[1, 2, 3]`, adds to both, and ends with `view = [3]` and `data = [11, 22, 3]`.

Leave the lifetimes out and rustc will not guess:

```text title="Abridged — real rustc output for pop_front_elided.rs"
error[E0106]: missing lifetime specifier
 --> pop_front_elided.rs:1:44
  |
1 | fn pop_front(t: &mut &mut [i32]) -> Option<&mut i32> {
  |                 ---------------            ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say which one of `t`'s 2 lifetimes it is borrowed from
```

Its `help:` again offers `&'a mut &'a mut [i32]`, with the caller-side cost above. Naming the *outer* lifetime instead — `fn pop_front<'a, 'b>(t: &'b mut &'a mut [i32]) -> Option<&'b mut i32>` — compiles, and ties each element to a borrow of `view`:

```text title="Abridged — real rustc output for pop_front_outer.rs"
error[E0499]: cannot borrow `view` as mutable more than once at a time
  --> pop_front_outer.rs:11:23
   |
10 |     let a = pop_front(&mut view).unwrap();
   |                       --------- first mutable borrow occurs here
11 |     let b = pop_front(&mut view).unwrap();
   |                       ^^^^^^^^^ second mutable borrow occurs here
12 |     *a += 10;
   |     -------- first borrow later used here
```

## The shared version needs no `take`

With `&mut &[i32]` the naive line compiles as written:

```rust
fn skip_first_shared(t: &mut &[i32]) {
    *t = &t[1..];
}
```

`&[i32]` is `Copy`, so reading `*t` copies the caller's reference out with its full lifetime, and the re-slice lasts as long as that copy. The same holds for `&str`, which makes `&mut &str` the usual shape of a parser cursor:

```rust
fn next_word<'a>(input: &mut &'a str) -> Option<&'a str> {
    let s = input.trim_start();
    if s.is_empty() {
        *input = s;
        return None;
    }
    let end = s.find(' ').unwrap_or(s.len());
    let (word, rest) = s.split_at(end);
    *input = rest;   // the caller's cursor now starts after the word
    Some(word)       // borrowed from the original text, not from `input`
}
```

On `"GET /index.html HTTP/1.1"`, two calls return `Some("GET")` and `Some("/index.html")` and leave the cursor at `" HTTP/1.1"`; each word has lifetime `'a`, so it outlives the cursor moving on.

## std does this: `Read for &[u8]`, `Write for &mut [u8]`

`impl Read for &[u8]` makes `Self = &[u8]`, so `fn read(&mut self, buf: &mut [u8])` receives a `&mut &[u8]` — this page's shared shape — and advances it past what it copied:

```rust
use std::io::Read;

fn main() {
    let mut src: &[u8] = b"hello";
    let mut buf = [0u8; 2];
    let n = src.read(&mut buf).unwrap();
    println!("{n} {:?}", std::str::from_utf8(src).unwrap());   // 2 "llo"
}
```

`impl Write for &mut [u8]` is the `&mut &mut [u8]` shape exactly. Its body in [1.98.0's `library/std/src/io/impls.rs` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/std/src/io/impls.rs) is `let (a, b) = mem::take(self).split_at_mut(amt);`, then `*self = b;` — the `skip_first` above. Write `b"hi"` into a five-byte buffer through it and the slice left over is three bytes long; the buffer reads `"hi..."`. This is why a `&mut [u8]` can stand in for a file in tests of [`Read` and `Write`](../../../12_Traits/read_and_write/README.md): each call consumes the front of the slice.

## Checkpoint

**Predict before you open the answer.** `skip_first` runs twice on a `view` of `[1, 2, 3]`. What do `view` and `data` print?

<details markdown="1">
<summary><strong>The answer</strong></summary>

<!-- output:repointing_a_slice -->
*Verified output of [`repointing_a_slice.rs`](examples/repointing_a_slice.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. &mut &mut [i32]: the function moves the caller's view
   skip_first(&mut view)          -> view = [2, 3]

2. &mut [i32]: elements yes, the caller's view no
   rebind_local(view)             -> view = [99, 42, 3]
   `t = &mut t[1..]` re-pointed the function's copy; view is still 3 long

3. &mut Vec<i32>: the one that can grow
   grow(&mut v)                   -> v = [1, 2, 3, 4]

4. Handing out elements that outlive the call
   a, b popped, both still live   -> view = [3]
   after *a += 10 and *b += 20    -> data = [11, 22, 3]

5. The shared version: &mut &[i32], and &mut &str as a parser cursor
   skip_first_shared(&mut view)   -> view = [2, 3]
   two next_word calls            -> Some("GET"), Some("/index.html")
   cursor                         -> " HTTP/1.1"
   two more                       -> Some("HTTP/1.1"), None, cursor = ""

6. std: `impl Read for &[u8]` receives &mut &[u8]
   src.read(&mut buf)             -> read 2: "he"
   src afterwards                 -> "llo", len 3

7. std: `impl Write for &mut [u8]` receives &mut &mut [u8]
   dst.write(b"hi")               -> wrote 2, dst len now 3
   out                            -> "hi..."

Checkpoint. skip_first twice on [1, 2, 3]: what do view and data print?
   view = [3]
   data = [1, 2, 3]  (the view moved; nothing was removed from data)
```
<!-- /output -->

</details>

## If you are coming from another language

- **C.** `void skip_first(int **p, size_t *len)` is the same function: a pointer to the caller's pointer, plus a pointer to the caller's length, because C carries a slice as two loose variables. Rust's `&mut &mut [i32]` bundles both halves into one fat pointer, so they cannot drift apart, and advancing past the end panics — `&t[1..]` on an empty slice — where `++*p; --*len;` would walk off and wrap the length. The `int **` idiom transfers directly; the bookkeeping around it is now the compiler's.
- **C++.** `std::span<int>& s` passed by reference and reassigned with `s = s.subspan(1)` is this shape, and `std::string_view&` advanced with `remove_prefix` is the parser cursor. A `span` passed by value is the plain `&mut [i32]` case: the callee can write elements and re-point only its copy. What C++ does not track is whether the caller's `span` outlives the buffer, or whether a second `span` writes the same elements meanwhile. Rust answers both at compile time — `'a` in `&mut &'a mut [i32]` ties the view to the buffer, and `&mut` rules out a second writer — so getting either wrong is a compile error rather than a dangling `span`.
- **Python.** A list slice `xs[1:]` is a copy, and `memoryview(buf)[1:]` is a new view object; either way, a function cannot rebind the caller's name, so the idiom is to return the new view (`view = skip_first(view)`) or keep an index in an object. `io.BytesIO` is the object that advances: `read(2)` moves its position, like `Read for &[u8]`. Rust can hand the callee the caller's variable itself, which Python has no way to express — and the price is saying, in the signature, which of the two lifetimes each returned reference borrows from.
- **ABAP.** The nearest shape is a field symbol re-assigned by a subroutine to a different part of a table, or an index passed by reference that the caller then uses — `ASSIGN lt_tab[ lv_index ] TO <fs>`. Both leave it to the programmer that the table is not changed underneath while the pointer or index is kept. `&mut &mut [i32]` is the checked version: the caller's view moves, and the compiler rejects any other access to those elements while it is out on loan.

## See also

- [Reborrowing](../../reborrowing/README.md) — what `&mut t[1..]` is, and why it cannot outlive `t`
- [What `&'a T` claims](../../what_a_reference_claims/README.md) — [the direction reverses behind `&mut`](../../what_a_reference_claims/README.md#the-direction-reverses-behind-mut)
- [Assignment drops the old value](../../assignment_is_a_drop/README.md) — `mem::replace`, `mem::take` and `mem::swap` as the ways to get a value out from behind a `&mut`
- [`str` is unsized](../../../14_Strings/str_is_unsized/README.md) — the two words in a `&[T]` or `&str`
- [`slice` methods](../../../26_Collections/slice_methods/README.md) — the slice API the cursor functions are built from
- [`Read` and `Write`](../../../12_Traits/read_and_write/README.md) — the traits whose byte-slice impls advance a `&mut &[u8]`
- [`split_first_mut` ↗](https://doc.rust-lang.org/std/primitive.slice.html#method.split_first_mut) · [`impl Read for &[u8]` ↗](https://doc.rust-lang.org/std/io/trait.Read.html#impl-Read-for-%26%5Bu8%5D) · [`std::mem::take` ↗](https://doc.rust-lang.org/std/mem/fn.take.html)

## Po polsku

`fn foo(t: &mut &mut [i32])` to **referencja wyłączna do referencji na wycinek** (*slice*) należącej do wywołującego. Wycinek to dwa słowa — adres i długość — a zewnętrzne `&mut` pozwala funkcji nadpisać obie wartości w zmiennej wywołującego: przesunąć początek widoku, skrócić go. Zwykłe `t: &mut [i32]` pozwala zapisywać elementy, ale `t = &mut t[1..]` zmienia wtedy tylko lokalną kopię i wywołujący niczego nie zauważy. Wydłużyć się nie da ani tak, ani tak — do tego potrzebne jest `&mut Vec<i32>`.

Naiwne `*t = &mut t[1..]` się nie kompiluje (*lifetime may not live long enough*): nowe pożyczenie przechodzi przez `t`, więc żyje tylko tyle co zewnętrzne pożyczenie, a `*t` musi przechować coś, co żyje tyle co wycinek wywołującego. Rozwiązaniem jest `std::mem::take(t)` — wyjmuje referencję wywołującego na własność, zostawiając pusty wycinek (`&mut [T]` implementuje `Default`), i dopiero ją przycina. Podpowiedź kompilatora `&'a mut &'a mut [i32]` kompiluje funkcję, ale blokuje zmienną wywołującego do końca jej życia (`E0502`). Zwracając element, nazwij czas życia **wewnętrzny**: `fn pop_front<'a>(t: &mut &'a mut [i32]) -> Option<&'a mut i32>`.

Wersja współdzielona, `&mut &[u8]` albo `&mut &str`, jest prostsza, bo `&[u8]` jest `Copy` — tak wygląda kursor parsera. Biblioteka standardowa robi dokładnie to samo: `impl Read for &[u8]` przesuwa wycinek za przeczytane bajty, a `impl Write for &mut [u8]` używa w środku `mem::take(self)`.

**Szukaj po polsku:** referencja do referencji · przesuwanie wycinka · kursor parsera w Ruscie · `rust &mut &mut [T]` · `rust mem::take slice advance` · `rust impl Read for &[u8]`
