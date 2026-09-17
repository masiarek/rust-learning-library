# Reading the `Clone for &T` hover, line by line

**Level:** 101 → 201 · reading std docs

**One line:** Hover `.clone()` on a `&str` and the editor shows `impl<T: PointeeSized> Clone for &T`, the impl for *every* shared reference, because `str` has no `clone` of its own. That clone copies an address and nothing else. The paragraphs underneath are `Clone::clone`'s general note for all types, not a description of this impl.

```rust
fn main() {
    let hello: &str = "Hello";
    let copy: &str = hello;                     // what hello.clone() does, minus the warning
    println!("{}", std::ptr::eq(hello, copy));  // true: one "Hello", two addresses of it
    let owned: String = hello.to_owned();       // this is the call that duplicates the text
    println!("{}", owned.as_ptr() == hello.as_ptr()); // false
}
```

The hover, as rust-analyzer shows it in Zed or VS Code (the lines above the doc text):

```text
core::clone::impls
impl<T: PointeeSized> Clone for &T
fn clone(&self) -> Self
```

| Line | Read it as |
|---|---|
| `core::clone::impls` | **where** the impl is written: a private module in `library/core/src/clone.rs`. Nothing to import. `Clone` is in the prelude, and `use core::clone::impls;` is `E0603`, *"module `impls` is private"* |
| `impl<T: PointeeSized> Clone for &T` | for **every** type `T` (any size, even none), a shared reference `&T` is `Clone` |
| `fn clone(&self) -> Self` | the trait's signature, with `Self` = `&T`. So `self` is a `&&T` |
| *"Returns a duplicate of the value…"* | the doc comment on `Clone::clone` in the trait, which covers all types. The method in this impl has no doc comment, so the hover borrows the trait's |

Every claim below is printed by [the run at the bottom](#the-verified-output), section by section.

## 1. Why the hover landed on `&T` and not on `str`

`Clone` requires `Sized`, and [`str` has no size](../../14_Strings/str_is_unsized/README.md), so there is no `impl Clone for str`. The dot then tries receivers in order ([the dot takes the first receiver that fits](../how_to_learn_to_owned/the_dot_picks_first/README.md)). The first that fits is the reference's own impl, `<&str as Clone>::clone`, which takes a `&&str`. The hover shows the impl the dot picked.

On a `&String` the dot stops one step earlier. `String` **is** `Clone`, so `borrowed.clone()` resolves to `impl Clone for String`, the hover says so, and a new `String` comes back (run, section 6). Same spelling, different impl: the type of the thing behind the reference decides.

## 2. `impl<T: PointeeSized> Clone for &T`

Read `impl<T: Bound> Trait for Type` as *"for every `T` that meets `Bound`, `Type` implements `Trait`"*. `PointeeSized` is the loosest bound Rust has, so the impl covers every reference there is: `&str`, `&[i32]`, `&dyn Display`, and `&Ticket` for a `Ticket` that is not `Clone` (run, section 1).

Inside the compiler, types now sit on three levels by what is known about their size:

| Bound | Says | Also admits | Write it on 1.98 stable? |
|---|---|---|---|
| `T` (an implicit `Sized`) | size known at compile time | `i32`, `String`, `&str` | yes, it is the default |
| `T: ?Sized` (called `MetaSized` inside the compiler) | size readable from the pointer at run time | `str`, `[i32]`, `dyn Display` | yes, spelled `?Sized` |
| `T: PointeeSized` | may have no size at all | extern types, which are nightly-only | **no** |

**On stable, read `PointeeSized` as `?Sized`.** Every type you can write on stable meets both. The docs show the new name because std's source switched to it for these impls. Other impls on the same 1.98.0 docs still say `?Sized`, for example `Rc`'s: `impl<T: ?Sized, A: Allocator + Clone> Clone for Rc<T, A>`. The older spelling is what [`&T` is `Copy` for every `T`](../how_to_learn_to_owned/clone_returns_self/README.md#t-is-copy-for-every-t) quotes.

Try to write the new name yourself and stable refuses it:

```rust,ignore
use std::marker::PointeeSized;

fn describe<T: PointeeSized>(_x: &T) {}

fn main() {
    describe("Hello");
}
```

```text title="rustc 1.98.0 on pointee_sized.rs — the snippet above"
error[E0658]: use of unstable library feature `sized_hierarchy`
 --> pointee_sized.rs:1:5
  |
1 | use std::marker::PointeeSized;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #144404 <https://github.com/rust-lang/rust/issues/144404> for more information

error[E0658]: use of unstable library feature `sized_hierarchy`
 --> pointee_sized.rs:3:16
  |
3 | fn describe<T: PointeeSized>(_x: &T) {}
  |                ^^^^^^^^^^^^
  |
  = note: see issue #144404 <https://github.com/rust-lang/rust/issues/144404> for more information

error: aborting due to 2 previous errors
```

The gap between the bottom two rows only shows on nightly, where an extern type is a type with no size even at run time. Tried on `rustc 1.100.0-nightly (e7769602a 2026-08-24)` with `#![feature(sized_hierarchy, extern_types)]`: `o.clone()` on a `&Opaque` compiles, and so does passing it to a `T: PointeeSized` function. The same `o` passed to a `T: ?Sized` function is refused:

```text title="Abridged — rustc 1.100.0-nightly on opaque.rs"
error[E0277]: the size for values of type `Opaque` cannot be known
  --> opaque.rs:14:19
   |
14 |     maybe_unsized(o);               // E0277
   |     ------------- ^ the trait `MetaSized` is not implemented for `Opaque`
```

The same docs list the impl's opposite right beside it: `impl<T: PointeeSized> !Clone for &mut T`. Two live `&mut` to one value is what the borrow checker exists to prevent, so a `&mut` can never be cloned. [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md#t-is-copy-mut-t-is-not) has the matching `Copy` pair.

## 3. `fn clone(&self) -> Self`: the whole body is one `*`

With `Self` = `&T`, the signature becomes `fn clone(self: &&T) -> &T`. One dereference gets from the argument to the result, and that is std's entire body:

```rust,ignore
impl<T: PointeeSized> const Clone for &T {
    fn clone(&self) -> Self {
        *self
    }
}
```

`*self` reads a `&T` out from behind the outer `&`. A reference is `Copy`, so reading it copies it: the address, plus the length for a `&str` or `&[i32]`. No bytes of the `T` are touched. Section 2 of the run writes the same body as an ordinary function, `clone_like_std`, and gets the same address back. (`const Clone` means the impl may one day be called in a `const`, which is unstable on 1.98.0; the rendered docs leave the word out.)

## 4. *"Returns a duplicate of the value"*: the three bullets, measured

| The doc says | The run printed | What to add |
|---|---|---|
| *"For most types, this creates a deep, independent copy"* | `String::clone`: new buffer `true` | "Most" is doing work. A `Vec<Rc<String>>` clone makes a new `Vec` buffer but shares the `String` inside it. A clone goes exactly as deep as each field's own `clone`, as [what a clone costs](../../18_Ownership/what_a_clone_costs/README.md) adds up field by field |
| *"For reference types like `&T`, this creates another reference to the same value"* | `<&String>::clone`: same `String` `true` | This impl. Nothing is duplicated |
| *"For smart pointers like `Arc` or `Rc`, this increments the reference count but still points to the same underlying data"* | `Arc::clone`: same `String` `true`, `strong_count` 2 | One counter goes up. [`Rc`](../../18_Ownership/reference_counting/README.md) is the same without the atomic count |

So "duplicate" does not mean one thing. Look at the type to see which of the three rows you are in. The method name will not tell you. [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md#copy-is-shallow-clone-is-deep-both-halves-are-false) makes that argument in full.

## 5. The first doc example hides a line

What the rendered docs show, wrapped in a `main` so it can be pasted:

```rust
fn main() {
    let hello = "Hello"; // &str implements Clone

    assert_eq!("Hello", hello.clone());
}
```

What the source says, in [`clone.rs` ↗](https://doc.rust-lang.org/src/core/clone.rs.html) on 1.98.0:

```text
/// # #![allow(noop_method_call)]
/// let hello = "Hello"; // &str implements Clone
///
/// assert_eq!("Hello", hello.clone());
```

A doc-test line starting with `# ` runs but is [hidden from the page ↗](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#hiding-portions-of-the-example). This one switches off the warning you get when you paste the visible part:

```text title="rustc 1.98.0 on doc_example.rs — the four-line block above"
warning: call to `.clone()` on a reference in this situation does nothing
 --> doc_example.rs:4:30
  |
4 |     assert_eq!("Hello", hello.clone());
  |                              ^^^^^^^^ help: remove this redundant call
  |
  = note: the type `str` does not implement `Clone`, so calling `clone` on `&str` copies the reference, which does not do anything and can be removed
  = note: `#[warn(noop_method_call)]` on by default

warning: 1 warning emitted
```

Nothing in the example is false. *"`&str` implements Clone"* is true, through this impl with `T` = `str`. The `assert_eq!` passes because both sides point at the same five bytes, not because a copy was made (run, section 4). If you wanted your own copy of the text, the call is [`to_owned()`](../to_owned/README.md) or `String::from`, and the result is a `String`.

## 6. The second doc example: two handles, one `Mutex`

`data.clone()` in the doc calls `Arc::clone`. The library writes it `Arc::clone(&data)`, the spelling [the Book recommends ↗](https://doc.rust-lang.org/book/ch15-04-rc.html) so a reader can tell a count bump from a deep copy at a glance ([what a clone costs](../../18_Ownership/what_a_clone_costs/README.md#the-call-site-cannot-tell-you-which-one-you-got)). The run adds two lines the doc leaves out:

- after the clone, `Arc::strong_count(&data)` is **2**: two handles, one allocation;
- after `drop(data)` the count is **1** and the vector is still `[1, 2, 3, 4]`. The data lives until the last handle goes.

The `push` through one handle is visible through the other because there is only one `Vec`. `Arc` hands out only `&` access, so the `Mutex` is what makes writing through a shared handle possible ([interior mutability](../../09_Advanced/interior_mutability/README.md)). One handle per thread is how this is normally used ([sharing across threads](../../18_Ownership/sharing_across_threads/README.md)).

## If you are coming from another language

**Python.** `b = a` binds a second name to the same object. That is what `<&T>::clone` does, and the Python library's [`bytearray` page ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/bytearray_is_mutable/) prints `alias is the same object: True` for it. `copy.copy(list_of_lists)` is the `Vec<Rc<String>>` row: a new outer list, the same inner objects. `copy.deepcopy` is the all-the-way-down copy, which a derived `Clone` gives only when every field's own `clone` goes all the way down too. Two things change. In Rust the type says which one you get, so `hello.clone()` on a `&str` can be seen to copy nothing (and rustc says so). And you cannot mutate through a shared `&T`, so the alias it creates cannot cause the "I changed `b` and `a` changed too" bug that Python aliases cause. To change shared data you need something like the `Mutex` in section 6, which you have to write out.

**C.** `const char *q = p;` copies an address, and that is `<&str>::clone`, except that a `&str` carries its length too, so two words are copied. `strdup(p)` is the owned copy, `hello.to_owned()`. What the compiler now enforces is the part C leaves to comments: `q` owns nothing, so nothing can free it twice, and it cannot outlive the bytes it points at.

**C++.** Copying a `std::shared_ptr` bumps `use_count()` and shares the object, which is exactly `Arc::clone`. Copying a `const T*` copies the address. A C++ copy constructor can do either, and so can `Clone`, so the rule is the same: read the type. The closest C++ counterpart of `&T` is `const T*` rather than `const T&`, since a C++ reference cannot be copied as a value of its own, while a Rust `&T` can be copied and can never be null.

**Java.** Every object variable already behaves like a reference: `String b = a;` copies the reference, never the object. `Object.clone()` is shallow by default for an array of objects, like the `Vec<Rc<String>>` row. Java needs the `Cloneable` marker plus an override; Rust needs `impl Clone` or a derive. What Java cannot express is a reference you may not write through. Rust's `&T` is exactly that.

**ABAP.** `lo_b = lo_a.` on two object references copies the reference, and both point at the same instance, like `<&T>::clone` or `Arc::clone`. `ls_b = ls_a.` on two structures copies the field values, like cloning an owned value. The difference is that an ABAP reference can change the instance it points at, and a Rust `&T` cannot.

## The verified output

<!-- output:reading_the_clone_hover -->
*Verified output of [`reading_the_clone_hover.rs`](examples/reading_the_clone_hover.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. impl<T: PointeeSized> Clone for &T: every T, even with no size, even not Clone
   T = str         -> &str                                 same address: true
   T = [i32]       -> &[i32]                               same address: true
   T = dyn Display -> &dyn core::fmt::Display              same address: true
   T = Ticket      -> &reading_the_clone_hover::Ticket     same address: true
   Ticket is not Clone, and &Ticket still is: seat 12

2. fn clone(&self) -> Self, with Self = &str
   self is &&str, the body is *self, the result is &str   same address: true

3. The three kinds of "duplicate" the doc lists, measured
   String::clone     new buffer: true
   <&String>::clone  same String: true
   Arc::clone        same String: true   strong_count: 2
   Vec<Rc<String>>   new Vec buffer: true   but same String inside: true   ("most types" is not all)

4. The first doc example: its source starts with a hidden allow line
   hello.clone() is &str, same address: true   nothing was duplicated

5. The second doc example: two Arc handles, one Mutex
   strong_count after the clone: 2
   pushed 4 through data, read through data_clone: [1, 2, 3, 4]
   after drop(data): strong_count 1, the vector still there: [1, 2, 3, 4]

6. The hover only lands on &T when T has no clone of its own
   (&String).clone() is alloc::string::String   new buffer: true   the hover shows impl Clone for String
```
<!-- /output -->

## Practice

**Which impl does the hover land on?** Five `.clone()` calls, on `&str`, `&String`, `&&String`, `Rc<str>` and `&[i32]`. For each one, say which impl the dot picks and what type comes back. Then say which three calls rustc warns about, and why one of the three warnings has a different name from the other two.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:reading_the_clone_hover_kata -->
*[`reading_the_clone_hover_kata.rs`](examples/reading_the_clone_hover_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata: say which impl the hover lands on for each `.clone()`, and what type
//! comes back, before running this.
//!
//!   rustc --edition 2024 reading_the_clone_hover_kata.rs -o /tmp/rtchk && /tmp/rtchk

use std::any::type_name_of_val;
use std::rc::Rc;

// Three of the five calls draw a warning, which is part of the answer: two
// noop_method_call and one suspicious_double_ref_op. std's own doc example
// hides an allow line for the first of them.
#[allow(noop_method_call, suspicious_double_ref_op)]
fn main() {
    let owned = String::from("Hello");
    let text: &str = "Hello";
    let words: &String = &owned;
    let twice: &&String = &words;
    let counted: Rc<str> = Rc::from("Hello");
    let numbers: &[i32] = &[1, 2, 3];

    let a = text.clone();
    let b = words.clone();
    let c = twice.clone();
    let d = counted.clone();
    let e = numbers.clone();

    println!("text.clone()    {:<24} impl Clone for &T, T = str", type_name_of_val(&a));
    println!("words.clone()   {:<24} impl Clone for String", type_name_of_val(&b));
    println!("twice.clone()   {:<24} impl Clone for &T, T = String", type_name_of_val(&c));
    println!("counted.clone() {:<24} impl Clone for Rc<T, A>, strong_count now {}", type_name_of_val(&d), Rc::strong_count(&counted));
    println!("numbers.clone() {:<24} impl Clone for &T, T = [i32]", type_name_of_val(&e));
}
```
<!-- /source -->

<!-- output:reading_the_clone_hover_kata -->
*Verified output of [`reading_the_clone_hover_kata.rs`](examples/reading_the_clone_hover_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
text.clone()    &str                     impl Clone for &T, T = str
words.clone()   alloc::string::String    impl Clone for String
twice.clone()   &alloc::string::String   impl Clone for &T, T = String
counted.clone() alloc::rc::Rc<str>       impl Clone for Rc<T, A>, strong_count now 2
numbers.clone() &[i32]                   impl Clone for &T, T = [i32]
```
<!-- /output -->

Without the `allow` line, rustc 1.98.0 warns three times. `text.clone()` and `numbers.clone()` get [`noop_method_call` ↗](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#noop-method-call), because `str` and `[i32]` have no `Clone` of their own and the call can only copy the reference. `twice.clone()` gets [`suspicious_double_ref_op` ↗](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#suspicious-double-ref-op): *"using `.clone()` on a double reference, which returns `&String` instead of cloning the inner type"*. `String` *is* `Clone`, so the call is not a no-op, but the reader probably expected a `String`.

</details>

## See also

- [Step 4: `Clone` hands back `Self`, and every `&T` is `Clone`](../how_to_learn_to_owned/clone_returns_self/README.md), the same impl met on the `ToOwned` path
- [Step 5: the dot takes the first receiver that fits](../how_to_learn_to_owned/the_dot_picks_first/README.md), which decides the impl the hover shows
- [Method resolution](../method_resolution/README.md), the full lookup rule behind step 5
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md), for "shallow vs deep" and `&T` vs `&mut T`
- [`ToOwned`](../to_owned/README.md), the call to use when you wanted the text copied
- [Reading the `ToOwned` docs](../reading_the_to_owned_docs/README.md), the same kind of page for the trait next door
- [What a clone costs](../../18_Ownership/what_a_clone_costs/README.md), with allocations counted field by field
- [`str` is unsized](../../14_Strings/str_is_unsized/README.md) and [marker traits](../marker_traits/README.md), for `Sized` and `?Sized`
- [`Rc`](../../18_Ownership/reference_counting/README.md) and [sharing across threads](../../18_Ownership/sharing_across_threads/README.md), the smart-pointer row in section 4
- [Returned by value](../../18_Ownership/returned_by_value/README.md), why `Clone: Sized` rules out `str`: `-> Self` hands the value back into room the caller reserved, and a `str` has no one size for that room

## Sources

[`Clone` in the standard library ↗](https://doc.rust-lang.org/std/clone/trait.Clone.html), read from the 1.98.0 copy `rustup doc` installs, and its [source, `core/src/clone.rs` ↗](https://doc.rust-lang.org/src/core/clone.rs.html). The `Sized` / `MetaSized` / `PointeeSized` definitions in [`core/src/marker.rs` ↗](https://doc.rust-lang.org/src/core/marker.rs.html), and the [`sized_hierarchy` tracking issue ↗](https://github.com/rust-lang/rust/issues/144404) that rustc links. The [reference primitive ↗](https://doc.rust-lang.org/std/primitive.reference.html), whose trait list covers every `&T`.

## Po polsku

Po najechaniu kursorem na `.clone()` wywołane na `&str` edytor pokazuje `impl<T: PointeeSized> Clone for &T`, czyli implementację dla **każdej** referencji współdzielonej. Dzieje się tak, bo `str` nie ma własnego `clone`: `Clone` wymaga `Sized`, a `str` rozmiaru nie ma. Pierwsza linia, `core::clone::impls`, mówi tylko, **gdzie** ta implementacja jest zapisana (w prywatnym module), a nie co importować. `fn clone(&self) -> Self` z `Self` = `&T` oznacza, że `self` jest typu `&&T`, a całe ciało metody to `*self`: kopiuje się adres (dla `&str` także długość), a sam tekst nie jest ruszany. Opis pod spodem („Returns a duplicate of the value”) to ogólny komentarz metody `Clone::clone` z definicji cechy (*trait*), a nie opis tej konkretnej implementacji.

`PointeeSized` na stabilnym Ruście czytaj jak `?Sized`. To najluźniejszy z trzech poziomów „rozmiarowości” (`Sized`, `?Sized`, `PointeeSized`), a sama nazwa jest niestabilna (`E0658`). Różnicę widać tylko na nightly, przy typach `extern` bez żadnego rozmiaru. Trzy punkty z dokumentacji sprawdzone w programie: `String::clone` alokuje nowy bufor, `clone` na `&T` daje ten sam adres, `Arc::clone` podbija licznik do 2. Słowo „most” jest tu ważne, bo `Vec<Rc<String>>` po sklonowaniu ma nowy bufor, ale te same napisy w środku. Pierwszy przykład z dokumentacji ma ukrytą linię `# #![allow(noop_method_call)]`. Bez niej kompilator ostrzega, że to wywołanie nic nie robi. Jeśli chcesz własną kopię tekstu, użyj `to_owned()`.

**Szukaj po polsku:** klonowanie referencji · kopia płytka a kopia głęboka · zliczanie referencji · `rust clone on &str does nothing` · `rust PointeeSized ?Sized` · `rust noop_method_call`
