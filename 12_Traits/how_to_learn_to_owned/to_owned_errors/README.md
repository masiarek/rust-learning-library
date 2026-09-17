# Every `ToOwned` error, and its fix

[How to learn `ToOwned`](../README.md) › **Beside the steps** · related: [Where `Clone` will not do](../where_clone_will_not_do/README.md)

**Level:** 101 → 201 · a reference, by symptom

**One line:** Twenty compiler errors that turn up around `ToOwned`, `Clone`, `Borrow` and `Cow` — for each, the code that causes it, what rustc 1.98.0 prints, the mistake in one or two sentences, and the fix. The broken code must fail and the fix must compile: [`check_fences.py`](../../../tools/check_fences.py) holds both on every build.

The error transcripts are recorded from rustc 1.98.0 with `--crate-type lib`, as the fences are checked. They are not regenerated on each commit the way an example's output is, so an upgrade can reword one; the fences, which are checked, cannot silently stop failing.

## Find the error

| # | Code | What rustc says | The mistake |
|---|---|---|---|
| 1 | `E0308` | [mismatched types: expected `String`, found `&str`](#1-clone-on-a-str-where-a-string-is-wanted) | `.clone()` on a `&str` where a `String` is wanted |
| 2 | `E0599` | [no method named `clone` found for type `str` in the current scope](#2-nameclone-dereferencing-first) | `(*name).clone()` — dereferencing first |
| 3 | `E0507` | [cannot move out of `*name` which is behind a shared reference](#3-name-to-take-the-string-out-of-a-string) | `*name` to take the `String` out of a `&String` |
| 4 | `E0308` | [mismatched types: expected `Ticket`, found `&Ticket`](#4-to_owned-on-a-ticket-hands-back-a-ticket) | `.to_owned()` on a `&Ticket` hands back a `&Ticket` |
| 5 | `E0308` | [mismatched types: expected type parameter `T`, found `&T`](#5-to_owned-on-a-t-with-no-bound-on-t) | `to_owned()` on a `&T` with no bound on `T` |
| 6 | `E0277` | [the trait bound `str: Clone` is not satisfied](#6-t-clone-on-a-function-meant-to-take-a-str) | `T: Clone` on a function meant to take a `str` |
| 7 | `E0277` | [the size for values of type `str` cannot be known at compilation time](#7-b-toowned-without-sized) | `B: ToOwned` without `?Sized` |
| 8 | `E0271` | [type mismatch resolving `<str as ToOwned>::Owned == Vec<u8>`](#8-toownedowned-vecu8-called-with-a-str) | `ToOwned<Owned = Vec<u8>>`, called with a `&str` |
| 9 | `E0277` | [a value of type `Vec<String>` cannot be built from an iterator over elements of type `&str`](#9-itermapword-wordto_owned-over-a-str) | `.iter().map(\|word\| word.to_owned())` over a `&[&str]` |
| 10 | `E0308` | [mismatched types: expected `&String`, found `&str`](#10-stringclone_from-with-a-str) | `String::clone_from` with a `&str` |
| 11 | `E0119` | [conflicting implementations of trait `ToOwned` for type `View`](#11-impl-toowned-for-a-type-that-is-clone) | `impl ToOwned` for a type that is `Clone` |
| 12 | `E0053`, `E0046` | [method `to_owned` has an incompatible type for trait](#12-an-impl-without-type-owned) | An impl without `type Owned` |
| 13 | `E0277` | [the trait bound `TinyBuf: Borrow<Tiny>` is not satisfied](#13-an-owned-type-that-cannot-lend-the-borrowed-one-back) | An owned type that cannot lend the borrowed one back |
| 14 | `E0277` | [the trait bound `dyn std::fmt::Display: Clone` is not satisfied](#14-a-cow-of-a-trait-object) | A `Cow` of a trait object |
| 15 | `E0599` | [no method named `into_borrowed` found for enum `Cow<'_, str>` in the current scope](#15-into_borrowed) | `into_borrowed()` |
| 16 | `E0515` | [cannot return value referencing local variable `text`](#16-returning-cowborrowed-of-a-local) | Returning `Cow::Borrowed` of a local |
| 17 | `E0106` | [missing lifetime specifier](#17-a-cowstr-field-with-no-lifetime) | A `Cow<str>` field with no lifetime |
| 18 | `E0521` | [borrowed data escapes outside of function](#18-keeping-a-short-lived-cow-in-a-static-cache) | Keeping a short-lived `Cow` in a `'static` cache |
| 19 | `E0382` | [borrow of moved value: `label`](#19-using-a-cow-after-into_owned) | Using a `Cow` after `into_owned()` |
| 20 | `E0596` | [cannot borrow `cow` as mutable, as it is not declared as mutable](#20-to_mut-on-a-cow-that-is-not-mut) | `to_mut()` on a `Cow` that is not `mut` |

## Borrowed in, owned out

### 1. `.clone()` on a `&str` where a `String` is wanted

```rust,compile_fail
pub fn keep(name: &str) -> String {
    name.clone()
}
```

```text title="rustc 1.98.0 on clone_on_str.rs"
error[E0308]: mismatched types
 --> clone_on_str.rs:2:5
  |
1 | pub fn keep(name: &str) -> String {
  |                            ------ expected `String` because of return type
2 |     name.clone()
  |     ^^^^^^^^^^^^ expected `String`, found `&str`
  |
help: try using a conversion method
  |
2 -     name.clone()
2 +     name.to_string()
  |
```

**The mistake.** `clone` hands back `Self`, and on a `&str` `Self` is `&str` — the reference, copied. `str` itself can never be `Clone`.

**The fix.** `to_owned()` reaches `str`'s impl, whose `Owned` is `String`. rustc's own suggestion, `.to_string()`, makes the same `String`.

```rust
pub fn keep(name: &str) -> String {
    name.to_owned()
}
```

**Read:** [Step 4](../clone_returns_self/README.md), [step 1](../clone_vs_to_owned/README.md)

### 2. `(*name).clone()` — dereferencing first

```rust,compile_fail
pub fn keep(name: &str) -> String {
    (*name).clone()
}
```

```text title="rustc 1.98.0 on clone_behind_deref.rs"
error[E0599]: no method named `clone` found for type `str` in the current scope
 --> clone_behind_deref.rs:2:13
  |
2 |     (*name).clone()
  |             ^^^^^
  |
help: there is a method `clone_into` with a similar name, but with different arguments
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/borrow.rs:66:4
```

**The mistake.** `*name` is a `str`, and `str` has no `clone` at all. The `help:` about `clone_into` is a match on the name, not a fix.

**The fix.** Call `to_owned()` on the reference itself.

```rust
pub fn keep(name: &str) -> String {
    name.to_owned()
}
```

**Read:** [Step 2](../types_with_no_size/README.md), [step 4](../clone_returns_self/README.md#self-in-self-out)

### 3. `*name` to take the `String` out of a `&String`

```rust,compile_fail
pub fn keep(name: &String) -> String {
    *name
}
```

```text title="rustc 1.98.0 on move_out_of_ref.rs"
error[E0507]: cannot move out of `*name` which is behind a shared reference
 --> move_out_of_ref.rs:2:5
  |
2 |     *name
  |     ^^^^^ move occurs because `*name` has type `String`, which does not implement the `Copy` trait
  |
help: consider cloning the value if the performance cost is acceptable
  |
2 -     *name
2 +     name.clone()
  |
```

**The mistake.** A shared reference lends the value; it cannot give it away.

**The fix.** Make a copy — `to_owned()`, or the `clone()` rustc suggests; on a `&String` both build a new `String`. Or take `name: String` by value, if the caller can give it up.

```rust
pub fn keep(name: &String) -> String {
    name.to_owned()
}
```

**Read:** [Ownership and moves](../../../18_Ownership/ownership_and_moves/README.md), [step 5](../the_dot_picks_first/README.md#walk-it-for-clone)

### 4. `.to_owned()` on a `&Ticket` hands back a `&Ticket`

```rust,compile_fail
pub struct Ticket {
    pub seat: u32,
}

pub fn keep(ticket: &Ticket) -> Ticket {
    ticket.to_owned()
}
```

```text title="rustc 1.98.0 on to_owned_not_clone.rs"
error[E0308]: mismatched types
 --> to_owned_not_clone.rs:6:5
  |
5 | pub fn keep(ticket: &Ticket) -> Ticket {
  |                                 ------ expected `Ticket` because of return type
6 |     ticket.to_owned()
  |     ^^^^^^^^^^^^^^^^^ expected `Ticket`, found `&Ticket`
```

**The mistake.** `Ticket` is not `Clone`, so the blanket impl passes it by and lands on `&Ticket`, whose `Owned` is `&Ticket`. Nothing in the message mentions `Clone`.

**The fix.** Derive `Clone` — then `clone()` or `to_owned()` makes a new `Ticket`.

```rust
#[derive(Clone)]
pub struct Ticket {
    pub seat: u32,
}

pub fn keep(ticket: &Ticket) -> Ticket {
    ticket.clone()
}
```

**Read:** [Step 6](../the_blanket_to_owned/README.md#steps-4-and-5-meet-here)

## Generic code

### 5. `to_owned()` on a `&T` with no bound on `T`

```rust,compile_fail
pub fn keep<T>(value: &T) -> T {
    value.to_owned()
}
```

```text title="rustc 1.98.0 on generic_no_bound.rs"
error[E0308]: mismatched types
 --> generic_no_bound.rs:2:5
  |
1 | pub fn keep<T>(value: &T) -> T {
  |             -                - expected `T` because of return type
  |             |
  |             expected this type parameter
2 |     value.to_owned()
  |     ^^^^^^^^^^^^^^^^ expected type parameter `T`, found `&T`
  |
  = note: expected type parameter `_`
                  found reference `&_`
```

**The mistake.** With no bound, the only `ToOwned` within reach is the blanket impl for `&T` (every `&T` is `Clone`), so the call returns `&T`.

**The fix.** Say what `T` must be able to do: `T: Clone`.

```rust
pub fn keep<T: Clone>(value: &T) -> T {
    value.clone()
}
```

**Read:** [Step 6](../the_blanket_to_owned/README.md), [`ToOwned` — the trap in generic code](../../to_owned/README.md#the-trap-in-generic-code)

### 6. `T: Clone` on a function meant to take a `str`

```rust,compile_fail
pub fn keep<T: Clone>(value: &T) -> T {
    value.clone()
}

pub fn name() -> String {
    keep::<str>("Ada")
}
```

```text title="rustc 1.98.0 on generic_clone_str.rs — the first of 3 errors"
error[E0277]: the trait bound `str: Clone` is not satisfied
 --> generic_clone_str.rs:6:12
  |
6 |     keep::<str>("Ada")
  |            ^^^ the trait `Clone` is not implemented for `str`
  |
help: the trait `Clone` is implemented for `String`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/string.rs:2377:0
note: required by a bound in `keep`
 --> generic_clone_str.rs:1:16
  |
1 | pub fn keep<T: Clone>(value: &T) -> T {
  |                ^^^^^ required by this bound in `keep`
```

**The mistake.** `Clone` requires `Sized`, and `str` is neither. rustc reports three errors for the one mistake; the first is shown.

**The fix.** Bound on the borrowed type instead: `B: ToOwned + ?Sized`, returning `B::Owned`.

```rust
pub fn keep<B: ToOwned + ?Sized>(value: &B) -> B::Owned {
    value.to_owned()
}

pub fn name() -> String {
    keep::<str>("Ada")
}
```

**Read:** [Step 2](../types_with_no_size/README.md#sized-is-the-bound-you-never-wrote), [Where `Clone` will not do, §6](../where_clone_will_not_do/README.md#6-one-function-for-every-borrowed-type)

### 7. `B: ToOwned` without `?Sized`

```rust,compile_fail
pub fn owner<B: ToOwned>(borrowed: &B) -> B::Owned {
    borrowed.to_owned()
}

pub fn name() -> String {
    owner("Ada")
}
```

```text title="rustc 1.98.0 on generic_no_unsized.rs"
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> generic_no_unsized.rs:6:11
  |
6 |     owner("Ada")
  |     ----- ^^^^^ doesn't have a size known at compile-time
  |     |
  |     required by a bound introduced by this call
  |
  = help: the trait `Sized` is not implemented for `str`
note: required by an implicit `Sized` bound in `owner`
 --> generic_no_unsized.rs:1:14
  |
1 | pub fn owner<B: ToOwned>(borrowed: &B) -> B::Owned {
  |              ^ required by the implicit `Sized` requirement on this type parameter in `owner`
help: consider relaxing the implicit `Sized` restriction
  |
1 | pub fn owner<B: ToOwned + ?Sized>(borrowed: &B) -> B::Owned {
  |                         ++++++++
```

**The mistake.** A type parameter is `Sized` unless it says otherwise, so `B = str` is refused before `ToOwned` is even considered.

**The fix.** rustc's `help:` is exactly the fix: `B: ToOwned + ?Sized`.

```rust
pub fn owner<B: ToOwned + ?Sized>(borrowed: &B) -> B::Owned {
    borrowed.to_owned()
}

pub fn name() -> String {
    owner("Ada")
}
```

**Read:** [Step 2](../types_with_no_size/README.md), [step 1's kata](../clone_vs_to_owned/README.md#practice)

### 8. `ToOwned<Owned = Vec<u8>>`, called with a `&str`

```rust,compile_fail
pub fn bytes<B: ToOwned<Owned = Vec<u8>> + ?Sized>(borrowed: &B) -> Vec<u8> {
    borrowed.to_owned()
}

pub fn from_text() -> Vec<u8> {
    bytes("Ada")
}
```

```text title="rustc 1.98.0 on owned_mismatch.rs"
error[E0271]: type mismatch resolving `<str as ToOwned>::Owned == Vec<u8>`
 --> owned_mismatch.rs:6:11
  |
6 |     bytes("Ada")
  |     ----- ^^^^^ expected `Vec<u8>`, found `String`
  |     |
  |     required by a bound introduced by this call
  |
  = note: expected struct `Vec<u8>`
             found struct `String`
note: required by a bound in `bytes`
 --> owned_mismatch.rs:1:25
  |
1 | pub fn bytes<B: ToOwned<Owned = Vec<u8>> + ?Sized>(borrowed: &B) -> Vec<u8> {
  |                         ^^^^^^^^^^^^^^^ required by this bound in `bytes`
```

**The mistake.** The bound names the owned type, and `str`'s owned twin is `String`, not `Vec<u8>`.

**The fix.** Pass the bytes — `"Ada".as_bytes()` is a `&[u8]`, whose owned twin is `Vec<u8>`. Or loosen the bound to any `B::Owned`.

```rust
pub fn bytes<B: ToOwned<Owned = Vec<u8>> + ?Sized>(borrowed: &B) -> Vec<u8> {
    borrowed.to_owned()
}

pub fn from_text() -> Vec<u8> {
    bytes("Ada".as_bytes())
}
```

**Read:** [Step 1](../clone_vs_to_owned/README.md#the-impl-is-on-the-type-behind-the)

### 9. `.iter().map(|word| word.to_owned())` over a `&[&str]`

```rust,compile_fail
pub fn own_words(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_owned()).collect()
}
```

```text title="rustc 1.98.0 on iter_double_ref.rs"
error[E0277]: a value of type `Vec<String>` cannot be built from an iterator over elements of type `&str`
 --> iter_double_ref.rs:2:46
  |
2 |     words.iter().map(|word| word.to_owned()).collect()
  |                                              ^^^^^^^ value of type `Vec<String>` cannot be built from `std::iter::Iterator<Item=&str>`
  |
help: the trait `FromIterator<&str>` is not implemented for `Vec<String>`
      but trait `FromIterator<String>` is implemented for it
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/vec/mod.rs:3938:0
  = help: for that trait implementation, expected `String`, found `&str`
note: the method call chain might not have had the expected associated types
 --> iter_double_ref.rs:2:18
  |
2 |     words.iter().map(|word| word.to_owned()).collect()
  |     ----- ------ ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Iterator::Item` changed to `&str` here
  |     |     |
  |     |     `Iterator::Item` is `&&str` here
  |     this expression has type `&[&str]`
note: required by a bound in `collect`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/iter/traits/iterator.rs:2077:4
```

**The mistake.** `iter()` yields `&&str`, and `.to_owned()` on a `&&str` finds `&str`'s impl first, so the items stay `&str`. rustc's note shows exactly where: `Iterator::Item changed to &str here`.

**The fix.** Destructure one `&` in the closure: `|&word| word.to_owned()`.

```rust
pub fn own_words(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| word.to_owned()).collect()
}
```

**Read:** [Step 5](../the_dot_picks_first/README.md#walk-it-for-to_owned-and-see-where-a-deref-happens), [step 6](../the_blanket_to_owned/README.md#one-str-two-impls)

### 10. `String::clone_from` with a `&str`

```rust,compile_fail
pub fn refill(buf: &mut String, name: &str) {
    buf.clone_from(name);
}
```

```text title="rustc 1.98.0 on clone_from_str.rs"
error[E0308]: mismatched types
 --> clone_from_str.rs:2:20
  |
2 |     buf.clone_from(name);
  |         ---------- ^^^^ expected `&String`, found `&str`
  |         |
  |         arguments to this method are incorrect
  |
  = note: expected reference `&String`
             found reference `&str`
note: method defined here
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/clone.rs:245:7
```

**The mistake.** `clone_from` takes `&Self` — another `String`.

**The fix.** `clone_into` is the `ToOwned` version, and takes the borrowed form: `name.clone_into(buf)`.

```rust
pub fn refill(buf: &mut String, name: &str) {
    name.clone_into(buf);
}
```

**Read:** [Step 9](../clone_into_refills/README.md)

## Implementing `ToOwned`

### 11. `impl ToOwned` for a type that is `Clone`

```rust,compile_fail
use std::borrow::Borrow;

#[derive(Clone)]
pub struct View;
pub struct Owned(View);

impl Borrow<View> for Owned {
    fn borrow(&self) -> &View {
        &self.0
    }
}

impl ToOwned for View {
    type Owned = Owned;
    fn to_owned(&self) -> Owned {
        Owned(View)
    }
}
```

```text title="rustc 1.98.0 on impl_on_clone_type.rs"
error[E0119]: conflicting implementations of trait `ToOwned` for type `View`
  --> impl_on_clone_type.rs:13:1
   |
13 | impl ToOwned for View {
   | ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: conflicting implementation in crate `alloc`:
           - impl<T> ToOwned for T
             where T: Clone;
```

**The mistake.** `View` is `Clone`, so std's blanket impl already implements `ToOwned` for it, and a second impl conflicts.

**The fix.** Write an inherent method instead — or, if the type should not be `Clone`, remove the derive and keep the impl.

```rust
#[derive(Clone)]
pub struct View;
pub struct Owned(pub View);

impl View {
    pub fn to_owned(&self) -> Owned {
        Owned(self.clone())
    }
}
```

**Read:** [Step 10](../implementing_it_or_not/README.md), [step 6](../the_blanket_to_owned/README.md#why-it-also-blocks-your-own-impl)

### 12. An impl without `type Owned`

```rust,compile_fail
#[repr(transparent)]
pub struct Tiny([u8]);
pub struct TinyBuf(Vec<u8>);

impl ToOwned for Tiny {
    fn to_owned(&self) -> TinyBuf {
        TinyBuf(self.0.to_vec())
    }
}
```

```text title="rustc 1.98.0 on missing_owned.rs — both errors"
error[E0053]: method `to_owned` has an incompatible type for trait
 --> missing_owned.rs:6:27
  |
6 |     fn to_owned(&self) -> TinyBuf {
  |                           ^^^^^^^ expected associated type, found `TinyBuf`
  |
  = note: expected signature `fn(&Tiny) -> <Tiny as ToOwned>::Owned`
             found signature `fn(&Tiny) -> TinyBuf`
help: change the output type to match the trait
  |
6 -     fn to_owned(&self) -> TinyBuf {
6 +     fn to_owned(&self) -> <Tiny as ToOwned>::Owned {
  |

error[E0046]: not all trait items implemented, missing: `Owned`
 --> missing_owned.rs:5:1
  |
5 | impl ToOwned for Tiny {
  | ^^^^^^^^^^^^^^^^^^^^^ missing `Owned` in implementation
  |
  = help: implement the missing item: `type Owned = /* Type */;`
```

**The mistake.** The impl has to name its owned type. rustc reports both halves: the missing item, and the return type that therefore cannot match.

**The fix.** Add `type Owned = TinyBuf;` — and then the next error on this page appears, until `Borrow` is implemented too.

```rust
use std::borrow::Borrow;

#[repr(transparent)]
pub struct Tiny([u8]);
pub struct TinyBuf(Vec<u8>);

impl Borrow<Tiny> for TinyBuf {
    fn borrow(&self) -> &Tiny {
        let bytes: &[u8] = &self.0;
        // SAFETY: `Tiny` is `#[repr(transparent)]` over `[u8]`.
        unsafe { &*(bytes as *const [u8] as *const Tiny) }
    }
}

impl ToOwned for Tiny {
    type Owned = TinyBuf;
    fn to_owned(&self) -> TinyBuf {
        TinyBuf(self.0.to_vec())
    }
}
```

**Read:** [Step 1](../clone_vs_to_owned/README.md#two-signatures), [Implementing `ToOwned`](../../implementing_to_owned/README.md#put-the-trait-on-the-referent)

### 13. An owned type that cannot lend the borrowed one back

```rust,compile_fail
#[repr(transparent)]
pub struct Tiny([u8]);
pub struct TinyBuf(Vec<u8>);

impl ToOwned for Tiny {
    type Owned = TinyBuf;
    fn to_owned(&self) -> TinyBuf {
        TinyBuf(self.0.to_vec())
    }
}
```

```text title="rustc 1.98.0 on missing_borrow.rs"
error[E0277]: the trait bound `TinyBuf: Borrow<Tiny>` is not satisfied
 --> missing_borrow.rs:6:18
  |
6 |     type Owned = TinyBuf;
  |                  ^^^^^^^ unsatisfied trait bound
  |
help: the trait `Borrow<Tiny>` is not implemented for `TinyBuf`
 --> missing_borrow.rs:3:1
  |
3 | pub struct TinyBuf(Vec<u8>);
  | ^^^^^^^^^^^^^^^^^^
note: required by a bound in `std::borrow::ToOwned::Owned`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/borrow.rs:30:4
```

**The mistake.** `type Owned: Borrow<Self>` — the owned type must lend out the borrowed one, and `TinyBuf` does not.

**The fix.** Implement `Borrow<Tiny> for TinyBuf`. `#[repr(transparent)]` is what makes the cast from `&[u8]` to `&Tiny` sound.

```rust
use std::borrow::Borrow;

#[repr(transparent)]
pub struct Tiny([u8]);
pub struct TinyBuf(Vec<u8>);

impl Borrow<Tiny> for TinyBuf {
    fn borrow(&self) -> &Tiny {
        let bytes: &[u8] = &self.0;
        // SAFETY: `Tiny` is `#[repr(transparent)]` over `[u8]`.
        unsafe { &*(bytes as *const [u8] as *const Tiny) }
    }
}

impl ToOwned for Tiny {
    type Owned = TinyBuf;
    fn to_owned(&self) -> TinyBuf {
        TinyBuf(self.0.to_vec())
    }
}
```

**Read:** [Step 7](../borrow_the_way_back/README.md#the-bound-in-the-trait), [Implementing `ToOwned` — why the cast is sound](../../implementing_to_owned/README.md#why-the-cast-is-sound)

## `Cow`

### 14. A `Cow` of a trait object

```rust,compile_fail
use std::borrow::Cow;
use std::fmt::Display;

pub fn show(value: &dyn Display) -> Cow<'_, dyn Display> {
    Cow::Borrowed(value)
}
```

```text title="rustc 1.98.0 on cow_dyn.rs — the first of 3 errors"
error[E0277]: the trait bound `dyn std::fmt::Display: Clone` is not satisfied
 --> cow_dyn.rs:4:37
  |
4 | pub fn show(value: &dyn Display) -> Cow<'_, dyn Display> {
  |                                     ^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `dyn std::fmt::Display`
  |
  = note: required for `dyn std::fmt::Display` to implement `ToOwned`
note: required by a bound in `Cow`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/borrow.rs:169:0
```

**The mistake.** `Cow<'_, B>` needs `B: ToOwned`. For a trait object the only candidate is the blanket impl, which needs `Clone` — so the first error is about `Clone`, not `ToOwned`. The other two say the same thing again.

**The fix.** A trait object has no owned twin. Own something that has one — here the rendered text, as a `Cow<'static, str>` — or keep the `&dyn Display` without a `Cow`.

```rust
use std::borrow::Cow;
use std::fmt::Display;

pub fn show(value: &dyn Display) -> Cow<'static, str> {
    Cow::Owned(value.to_string())
}
```

**Read:** [What `Cow` explanations get wrong](../cow_claims_checked/README.md#two-claims-the-compiler-refuses)

### 15. `into_borrowed()`

```rust,compile_fail
use std::borrow::Cow;

pub fn peek(label: Cow<'_, str>) -> &str {
    label.into_borrowed()
}
```

```text title="rustc 1.98.0 on into_borrowed.rs"
error[E0599]: no method named `into_borrowed` found for enum `Cow<'_, str>` in the current scope
 --> into_borrowed.rs:4:11
  |
4 |     label.into_borrowed()
  |           ^^^^^^^^^^^^^
  |
help: there is a method `into_owned` with a similar name
  |
4 -     label.into_borrowed()
4 +     label.into_owned()
  |
```

**The mistake.** There is no such method. And a function that takes the `Cow` by value could not return a `&str` into it anyway.

**The fix.** Take `&Cow` and let `Deref` hand out the `&str`.

```rust
use std::borrow::Cow;

pub fn peek<'a>(label: &'a Cow<'_, str>) -> &'a str {
    label
}
```

**Read:** [What `Cow` explanations get wrong](../cow_claims_checked/README.md#two-claims-the-compiler-refuses)

### 16. Returning `Cow::Borrowed` of a local

```rust,compile_fail
use std::borrow::Cow;

pub fn label(n: u32) -> Cow<'static, str> {
    let text = format!("item {n}");
    Cow::Borrowed(&text)
}
```

```text title="rustc 1.98.0 on return_borrowed_local.rs"
error[E0515]: cannot return value referencing local variable `text`
 --> return_borrowed_local.rs:5:5
  |
5 |     Cow::Borrowed(&text)
  |     ^^^^^^^^^^^^^^-----^
  |     |             |
  |     |             `text` is borrowed here
  |     returns a value referencing data owned by the current function
```

**The mistake.** `text` is dropped when the function returns, so nothing may keep pointing at it.

**The fix.** Text you built is owned text: `Cow::Owned(text)`.

```rust
use std::borrow::Cow;

pub fn label(n: u32) -> Cow<'static, str> {
    let text = format!("item {n}");
    Cow::Owned(text)
}
```

**Read:** [Step 7](../borrow_the_way_back/README.md#cow-is-built-on-the-pair)

### 17. A `Cow<str>` field with no lifetime

```rust,compile_fail
use std::borrow::Cow;

pub struct Token {
    pub text: Cow<str>,
}
```

```text title="rustc 1.98.0 on struct_no_lifetime.rs"
error[E0106]: missing lifetime specifier
 --> struct_no_lifetime.rs:4:18
  |
4 |     pub text: Cow<str>,
  |                  ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
3 ~ pub struct Token<'a> {
4 ~     pub text: Cow<'a, str>,
  |
```

**The mistake.** A `Cow` may borrow, and a struct has to say for how long.

**The fix.** Name the lifetime — `Token<'a>` holding `Cow<'a, str>` — or use `Cow<'static, str>` if it only ever borrows literals.

```rust
use std::borrow::Cow;

pub struct Token<'a> {
    pub text: Cow<'a, str>,
}
```

**Read:** [Lifetime annotations — on a struct it becomes part of the type](../../../18_Ownership/lifetime_annotations/README.md#on-a-struct-it-becomes-part-of-the-type)

### 18. Keeping a short-lived `Cow` in a `'static` cache

```rust,compile_fail
use std::borrow::Cow;

pub fn remember(cache: &mut Vec<Cow<'static, str>>, text: Cow<'_, str>) {
    cache.push(text);
}
```

```text title="rustc 1.98.0 on store_short_cow.rs"
error[E0521]: borrowed data escapes outside of function
 --> store_short_cow.rs:4:5
  |
3 | pub fn remember(cache: &mut Vec<Cow<'static, str>>, text: Cow<'_, str>) {
  |                 -----                               ----
  |                 |                                   |
  |                 |                                   `text` is a reference that is only valid in the function body
  |                 |                                   has type `Cow<'1, str>`
  |                 `cache` declared here, outside of the function body
4 |     cache.push(text);
  |     ^^^^^^^^^^^^^^^^
  |     |
  |     `text` escapes the function body here
  |     argument requires that `'1` must outlive `'static`
  |
  = note: requirement occurs because of a mutable reference to `Vec<Cow<'_, str>>`
  = note: mutable references are invariant over their type parameter
  = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
```

**The mistake.** The cache holds `Cow<'static, str>`, and `text` may borrow something that dies sooner. The lifetime is part of the type, so it stays even when `text` happens to be `Owned`.

**The fix.** `into_owned()` is how a `Cow` leaves its lifetime behind — it clones only if the `Cow` was `Borrowed` — and `Cow::Owned` puts the result back in the cache's type.

```rust
use std::borrow::Cow;

pub fn remember(cache: &mut Vec<Cow<'static, str>>, text: Cow<'_, str>) {
    cache.push(Cow::Owned(text.into_owned()));
}
```

**Read:** [Step 7](../borrow_the_way_back/README.md#cow-is-built-on-the-pair), [step 8](../to_owned_traps/README.md#cow-to_owned-is-not-into_owned)

### 19. Using a `Cow` after `into_owned()`

```rust,compile_fail
use std::borrow::Cow;

pub fn shout(label: Cow<'_, str>) -> String {
    let mut loud = label.into_owned();
    loud.push('!');
    println!("{label} became {loud}");
    loud
}
```

```text title="rustc 1.98.0 on use_after_into_owned.rs"
error[E0382]: borrow of moved value: `label`
 --> use_after_into_owned.rs:6:16
  |
3 | pub fn shout(label: Cow<'_, str>) -> String {
  |              ----- move occurs because `label` has type `Cow<'_, str>`, which does not implement the `Copy` trait
4 |     let mut loud = label.into_owned();
  |                          ------------ `label` moved due to this method call
5 |     loud.push('!');
6 |     println!("{label} became {loud}");
  |                ^^^^^ value borrowed here after move
  |
note: `Cow::<'_, B>::into_owned` takes ownership of the receiver `self`, which moves `label`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/borrow.rs:331:22
help: you can `clone` the value and consume it, but this might not be your desired behavior
  |
4 |     let mut loud = label.clone().into_owned();
  |                         ++++++++
```

**The mistake.** `into_owned` takes `self`: afterwards there is no `Cow` left to read.

**The fix.** Read it first — or keep the `Cow` and write through `to_mut()` instead.

```rust
use std::borrow::Cow;

pub fn shout(label: Cow<'_, str>) -> String {
    println!("{label} is about to become loud");
    let mut loud = label.into_owned();
    loud.push('!');
    loud
}
```

**Read:** [Step 7](../borrow_the_way_back/README.md#cow-is-built-on-the-pair)

### 20. `to_mut()` on a `Cow` that is not `mut`

```rust,compile_fail
use std::borrow::Cow;

pub fn shout(label: &str) -> Cow<'_, str> {
    let cow = Cow::Borrowed(label);
    cow.to_mut().push('!');
    cow
}
```

```text title="rustc 1.98.0 on to_mut_not_mut.rs"
error[E0596]: cannot borrow `cow` as mutable, as it is not declared as mutable
 --> to_mut_not_mut.rs:5:5
  |
5 |     cow.to_mut().push('!');
  |     ^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
4 |     let mut cow = Cow::Borrowed(label);
  |         +++
```

**The mistake.** `to_mut` takes `&mut self`, because it may switch the `Cow` from `Borrowed` to `Owned` in place.

**The fix.** Declare it `let mut`, as rustc suggests.

```rust
use std::borrow::Cow;

pub fn shout(label: &str) -> Cow<'_, str> {
    let mut cow = Cow::Borrowed(label);
    cow.to_mut().push('!');
    cow
}
```

**Read:** [Step 7](../borrow_the_way_back/README.md#checkpoint)

## See also

- [Where `Clone` will not do: code only](../where_clone_will_not_do/README.md) — the same failures as ten pairs, without the transcripts
- [What `Cow` explanations get wrong, run](../cow_claims_checked/README.md) — the claims behind errors 14 and 15
- [ERRORS.md](../../../ERRORS.md) — every error code the library explains, by code
- [How to learn `ToOwned`](../README.md) — the path these errors are stations on

## Po polsku

Dwadzieścia błędów kompilatora, które pojawiają się wokół `ToOwned`, `Clone`, `Borrow` i `Cow`: dla każdego kod, który go wywołuje, dokładny komunikat rustc 1.98.0, na czym polega pomyłka i poprawka. Kod z błędem musi się nie kompilować, a poprawka musi się kompilować — sprawdza to `check_fences.py` przy każdym budowaniu. Tabela na górze pozwala znaleźć błąd po kodzie (`E0308`, `E0277`…) albo po treści komunikatu.

Najczęstsze pułapki: `.clone()` na `&str` (dostajesz `&str`, nie `String`), `.to_owned()` na `&T`, gdy `T` nie jest `Clone` (dostajesz `&T`), brak `?Sized` w funkcji generycznej, `.iter().map(|w| w.to_owned())` na `&[&str]` (elementy zostają `&str`) oraz przechowywanie krótko żyjącego `Cow` tam, gdzie potrzebny jest `'static` — wtedy pomaga `into_owned()`.

**Szukaj po polsku:** `rust E0308 expected String found &str` · `rust E0277 str Clone` · `rust Cow lifetime into_owned`
