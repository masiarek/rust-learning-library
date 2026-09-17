# Reading the `ToOwned` docs page, block by block

**Level:** 101 → 201 · reading std docs

**One line:** The standard library's page for `ToOwned` is eight blocks — header, declaration, description, associated type, required method, provided method, dyn compatibility, implementors — and each one answers a question the lesson pages spend paragraphs on; the run at the bottom executes every claim the page makes.

```bash
rustup doc std::borrow::ToOwned
```

That opens the page for the toolchain you are using, offline, so the page matches your compiler. The [online copy ↗](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html) is always the latest stable release. This library pins 1.98.0 with rustup's `minimal` profile, which does not include the docs; `rustup component add rust-docs` installs them.

## 1. The header: `std::borrow`, `1.0.0`, `Source`

**The path** is where you import it from — except that you never need to: `ToOwned` is in the prelude, and section 1 of the run calls `.to_owned()` with no `use` line. [`Borrow`](../borrow_trait/README.md), from the same module, is not in the prelude.

**`1.0.0`** is the release the item became stable in. Methods carry their own badges, and they can be later than the trait's; section 6 below has one.

**`Source`** jumps to the code the page was generated from, which shows attributes the page leaves out. `to_owned` is `#[must_use]`, with a reason attached:

```rust
fn main() {
    let name = "Ada";
    name.to_owned();
}
```

```text title="rustc 1.98.0 on must_use.rs — the snippet above"
warning: unused return value of `to_owned` that must be used
 --> must_use.rs:3:5
  |
3 |     name.to_owned();
  |     ^^^^^^^^^^^^^^^
  |
  = note: cloning is often expensive and is not expected to have side effects
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
3 |     let _ = name.to_owned();
  |     +++++++
```

The source file is `alloc/src/borrow.rs`, not a file in `std`. The trait is defined in `alloc`, the part of the standard library that needs a heap but not an operating system, and `std` re-exports it. Compiler notes that point into the trait name the `alloc` path, as the `E0038` in section 7 does.

## 2. The declaration

```rust
pub trait ToOwned {
    type Owned: Borrow<Self>;
    // Required method
    fn to_owned(&self) -> Self::Owned;
    // Provided method
    fn clone_into(&self, target: &mut Self::Owned) { ... }
}
```

Both comments are rustdoc's, not the source's. **Required** means every impl writes it; **provided** means the trait has a default body, which `{ ... }` hides. `Source` shows the body is one line:

```rust
fn clone_into(&self, target: &mut Self::Owned) {
    *target = self.to_owned();
}
```

Section 4 of the run implements only `to_owned` for a type, calls `clone_into`, and counts one call to `to_owned`.

## 3. The description

*"A generalization of `Clone` to borrowed data."* The next sentence says `Clone` works only for going from `&T` to `T`. The page does not say why: `Clone` requires `Sized`, and `str` is not. That is the first section of the [`ToOwned`](../to_owned/README.md) lesson.

## 4. Required associated type: `type Owned: Borrow<Self>`

Every impl names the owned type, and generic code can read it back as `T::Owned`. Section 2 of the run prints it for three types. `str` gives `String`, a different type. `u8` gives `u8`, from the blanket impl for `Clone` types. And `&str` gives **`&str`** — a reference is `Clone`, so it is its own owned form. That last row is [the trap in generic code](../to_owned/README.md#the-trap-in-generic-code).

The bound, `Borrow<Self>`, is the promise that the owned form can lend the borrowed one back. [`Borrow`](../borrow_trait/README.md) has its own page, and its own promise about hashing.

## 5. Required method: `to_owned`

*"Creates owned data from borrowed data, usually by cloning."* For every `Clone` type the source is exactly `self.clone()`, which is the blanket impl in section 8.

The example under it is a doctest: rustdoc compiles and runs code blocks in doc comments, and the standard library's are part of Rust's own test suite. A doctest checks that the code compiles and does not panic, and prints nothing you can see. Section 3 of the run executes the same four lines and prints the results.

## 6. Provided method: `clone_into`, `1.63.0`

This badge is later than the trait's. `clone_into` was unstable, behind the feature `toowned_clone_into`, until 1.63.0. The description calls it the *"borrow-generalized version of [`Clone::clone_from` ↗](https://doc.rust-lang.org/std/clone/trait.Clone.html#method.clone_from)"*: it writes into a value you already own instead of returning a new one. Section 4 of the run executes the page's example.

What the page does not say is when that saves anything. The default body from section 2 allocates exactly as `to_owned` does. Only an override can reuse the buffer: `str`'s and `[T]`'s write into it, and the blanket impl for `Clone` types hands off to `clone_from`. That is counted on [`clone_into`](../clone_into/README.md).

## 7. Dyn compatibility: *"This trait is not dyn compatible."*

You cannot hold a `&dyn ToOwned`:

```rust
fn main() {
    let text: &dyn ToOwned<Owned = String> = "hi";
    let _ = text;
}
```

```text title="rustc 1.98.0 on dyn_to_owned.rs — the snippet above"
error[E0038]: the trait `ToOwned` is not dyn compatible
 --> dyn_to_owned.rs:2:20
  |
2 |     let text: &dyn ToOwned<Owned = String> = "hi";
  |                    ^^^^^^^^^^^^^^^^^^^^^^^ `ToOwned` is not dyn compatible
  |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/borrow.rs:30:16
  |
  = note: the trait is not dyn compatible because it uses `Self` as a type parameter
```

Line 30 of that file is `type Owned: Borrow<Self>;` — the bound from section 4 is what rules it out. `Borrow`'s page says the opposite, *"This trait is dyn compatible"*, and section 5 of the run keeps a `String` and a `&str` in one array of `&dyn Borrow<str>`.

The page adds that dyn compatibility used to be called *object safety*. Older books and answers use only the old name; [Static vs dynamic dispatch](../static_vs_dynamic_dispatch/README.md) covers what the rules are.

## 8. Implementors

The list, in the page's order:

| Impl | `type Owned` | Badge |
|---|---|---|
| `impl ToOwned for ByteStr` | `ByteString` | none |
| `impl ToOwned for CStr` | `CString` | 1.3.0 |
| `impl ToOwned for OsStr` | `OsString` | 1.0.0 |
| `impl ToOwned for Path` | [`PathBuf`](../../04_Files/path_and_pathbuf/README.md) | 1.0.0 |
| `impl ToOwned for str` | [`String`](../../14_Strings/string_vs_str/README.md) | 1.0.0 |
| `impl<T> ToOwned for T where T: Clone` | `T` | 1.0.0 |
| `impl<T> ToOwned for [T] where T: Clone` | `Vec<T>` | 1.0.0 |

Section 6 of the run calls every row but the first. **No badge here means unstable**: the std docs list unstable items too, and using `ByteStr` on stable is refused. Abridged to the first of three `E0658` errors, which differ only in the span:

```text title="Abridged — rustc 1.98.0 on bytestr.rs, a use of std::bstr::ByteStr and a call to to_owned"
error[E0658]: use of unstable library feature `bstr`
 --> bytestr.rs:1:5
  |
1 | use std::bstr::ByteStr;
  |     ^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #134915 <https://github.com/rust-lang/rust/issues/134915> for more information
```

Read the table by shape. Five rows are unsized borrowed types, each paired with its owned buffer — the case the trait was written for, and [the only kind of type you can usefully implement it for](../implementing_to_owned/README.md). One row, `impl<T> ToOwned for T where T: Clone`, covers everything else. That blanket row is where all of the surprises come from.

## What the page leaves out

The docs describe the trait accurately and mention none of these:

- `.to_owned()` on an `Rc<String>` copies the pointer, not the string, and on a `Cow::Borrowed` it returns a borrowed `Cow` — [the trap that blanket impl sets](../to_owned/README.md#the-trap-that-blanket-impl-sets)
- `.to_owned()` on a `&Foo` returns a `&Foo` when `Foo` is not `Clone` — [a reference is `Clone` even when its pointee is not](../to_owned/README.md#a-reference-is-clone-even-when-its-pointee-is-not)
- `impl ToOwned for MyType` is `E0119` whenever `MyType` is `Clone` — [can you implement it yourself?](../to_owned/README.md#can-you-implement-it-yourself)
- whether to write `to_owned()` or `to_string()` on a `&str` — [the argument you will meet, and its expiry date](../to_owned/README.md#the-argument-you-will-meet-and-its-expiry-date)

Each follows from the blanket impl row in section 8, which is why the [learning path](../how_to_learn_to_owned/README.md) spends five steps on the ideas underneath it.

## If you are coming from another language

- **Java** — rustdoc is closest to Javadoc: both are generated from comments in the source. The version badge is `@since`, *Implementors* is *All Known Implementing Classes*, and a required method is an abstract one. Javadoc has no counterpart to doctests: the `javadoc` tool does not compile or run the examples in a comment.
- **Python** — `help(str)` reads docstrings from the source, like rustdoc, but the library reference at docs.python.org is written separately. Its *"Added in version 3.x"* notes are the version badge. Python's `doctest` module is the closest thing to rustdoc's doctests, and you have to opt in to running it.
- **C++** — cppreference's *(since C++17)* markers work like the badges, but cppreference is written by volunteers from the standard, not generated from the library you link. Rust's docs are built from the same source as the `std` you compile against, which is why `rustup doc` gives a page that matches your compiler.

## The verified output

<!-- output:reading_the_to_owned_docs -->
*Verified output of [`reading_the_to_owned_docs.rs`](examples/reading_the_to_owned_docs.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The header: std::borrow, and no `use` line needed
   "Ada".to_owned() = "Ada", with ToOwned never imported: it is in the prelude

2. Required associated type: `type Owned: Borrow<Self>`
   <str  as ToOwned>::Owned = alloc::string::String   <- a different type
   <u8   as ToOwned>::Owned = u8                      <- the blanket impl: Self
   <&str as ToOwned>::Owned = &str                    <- a reference is Clone, so also Self

3. Required method: the page's own example, run
   ss = "a", vv = [1, 2]

4. Provided method: the page's own example, run
   s = "hello", v = [1, 2]
   an impl that wrote only to_owned: clone_into called to_owned 1 time, back.seat = 1

5. Dyn compatibility: Borrow is, ToOwned is not
   &dyn Borrow<str> lends "Ada"
   &dyn Borrow<str> lends "Ben"
   &dyn ToOwned<Owned = String> is E0038

6. Implementors: each stable impl, called
   str            "notes".to_owned()        -> alloc::string::String
   [T] (T: Clone) [1, 2][..].to_owned()     -> alloc::vec::Vec<i32>
   Path           Path::new(..).to_owned()  -> std::path::PathBuf
   OsStr          OsStr::new(..).to_owned() -> std::ffi::os_str::OsString
   CStr           c"notes".to_owned()       -> alloc::ffi::c_str::CString
   T (T: Clone)   42_u8.to_owned()          -> u8
```
<!-- /output -->

## See also

- [How to learn `ToOwned`](../how_to_learn_to_owned/README.md) — the ten steps, each now pointing at the documentation for its idea
- [`ToOwned`](../to_owned/README.md) — the lesson behind every block on this page
- [Reading the `Clone for &T` hover](../reading_the_clone_hover/README.md) — the same kind of reading for `Clone`, starting from the hover rather than the docs page
- [`Borrow`: look up an owned key with a borrowed one](../borrow_trait/README.md) — the trait in `type Owned`'s bound
- [`clone_into`](../clone_into/README.md) — the provided method, measured
- [Implementing `ToOwned` for your own type](../implementing_to_owned/README.md) — when a new row in section 8's table is worth writing
- [Static vs dynamic dispatch](../static_vs_dynamic_dispatch/README.md) — `dyn`, and what section 7's rule protects

## Sources

The page itself, [`ToOwned` in the standard library ↗](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html), read from the 1.98.0 copy `rustup doc` installs, and its [source, `alloc/src/borrow.rs` ↗](https://doc.rust-lang.org/src/alloc/borrow.rs.html). [Documentation tests ↗](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) in the rustdoc book. [Dyn compatibility ↗](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility) in the Reference.

## Po polsku

Strona dokumentacji `ToOwned` ma osiem bloków i każdy odpowiada na konkretne pytanie. Nagłówek podaje ścieżkę (`std::borrow`), wersję, w której element stał się stabilny (`1.0.0`), i link `Source`. Deklaracja rozróżnia metodę wymaganą (*required*), którą pisze każda implementacja, od dostarczonej (*provided*), która ma domyślne ciało — ukryte pod `{ ... }`. Blok *Implementors* to lista implementacji; wiersz bez numeru wersji oznacza element niestabilny, niedostępny w stabilnym Ruscie.

Najwięcej mówi to, co łatwo przeoczyć. `clone_into` ma znaczek `1.63.0`, późniejszy niż cała cecha (*trait*). Zdanie *„This trait is not dyn compatible”* znaczy, że nie ma `&dyn ToOwned` — winne jest ograniczenie `type Owned: Borrow<Self>`, co kompilator pokazuje, wskazując dokładnie tę linię źródła. Link `Source` odsłania atrybut, którego strona nie wyświetla: `#[must_use]` na `to_owned`, stąd ostrzeżenie przy nieużytym wyniku. Pułapek (`Rc`, `Cow`, `&Foo`) dokumentacja nie opisuje wcale — wszystkie wynikają z jednego wiersza tabeli, implementacji zbiorczej `impl<T: Clone> ToOwned for T`.

Dokumentację dla swojej wersji kompilatora otwiera `rustup doc std::borrow::ToOwned`, bez internetu. Przykłady na stronach biblioteki standardowej to testy dokumentacji (*doctests*): kompilują się i uruchamiają w testach samego Rusta, ale niczego nie wypisują, dlatego przykład na dole tej strony pokazuje ich wyniki.

**Szukaj po polsku:** dokumentacja biblioteki standardowej Rusta · czytanie rustdoc · `rust rustup doc offline` · `rust trait not dyn compatible E0038` · `rust doc stability badge unstable`
