# Every returned-by-value error, and its fix

Beside [Returned by value](../returned_by_value/README.md) · the warnings, not the errors: [Lints around returning by value](../returned_by_value_lints/README.md)

**Level:** 201 · a reference, by symptom

**One line:** Seventeen compiler errors that turn up when a function returns something with no size, when returning a value would move it out of a borrow, or when a borrow comes back in its place. For each: the code that causes it, what rustc 1.98.0 prints, the mistake in a few sentences, and a fix. The broken code must fail and the fix must compile: [`check_fences.py`](../../tools/check_fences.py) holds both on every build.

The transcripts were recorded from rustc 1.98.0 with `--crate-type lib --emit=metadata`, the way the fences are checked, from a file named in each title, and stop before rustc's closing `aborting due to` summary. Every fix was built the same way and printed nothing, not even a warning. The transcripts are not regenerated on each commit the way an example's output is, so a later rustc can reword one. The fences are checked, so none of them can silently start compiling.

Seven of the seventeen are `E0277`, and its one message covers every slot that needs a size. Read the line under it, not the headline: *the return type of a function must have a statically known size*, *all local variables must have a statically known size*, *function arguments must have a statically known size* and *required by an implicit `Sized` bound in `Option`* each name a different slot.

## Find the error

| # | Code | What rustc says | The mistake |
|---|---|---|---|
| 1 | `E0277`, `E0507` | [the size for values of type `str` cannot be known at compilation time](#1-str) | `-> str` |
| 2 | `E0277`, `E0508` | [the size for values of type `[i32]` cannot be known at compilation time](#2-i32) | `-> [i32]` |
| 3 | `E0277`, `E0308` | [the size for values of type `str` cannot be known at compilation time](#3-optionstr) | `-> Option<str>` |
| 4 | `E0746` | [return type cannot be a trait object without pointer indirection](#4-dyn-display) | `-> dyn Display` |
| 5 | `E0782` | [expected a type, found a trait](#5-fni32-i32-with-no-dyn-or-impl) | `-> Fn(i32) -> i32`, with no `dyn` or `impl` |
| 6 | `E0308` | [`if` and `else` have incompatible types](#6-impl-display-with-two-types-behind-it) | `-> impl Display` with two types behind it |
| 7 | `E0277`, `E0507` | [the size for values of type `T` cannot be known at compilation time](#7-t-with-t-sized) | `-> T` with `T: ?Sized` |
| 8 | `E0277` | [the size for values of type `str` cannot be known at compilation time](#8-deriveclone-on-a-struct-holding-a-str) | `#[derive(Clone)]` on a struct holding a `str` |
| 9 | `E0038` | [the trait `Clone` is not dyn compatible](#9-boxdyn-clone) | `Box<dyn Clone>` |
| 10 | `E0038` | [the trait `Shape` is not dyn compatible](#10-a-trait-method-returning-self-used-as-dyn) | A trait method returning `Self`, used as `dyn` |
| 11 | `E0507` | [cannot move out of `self.nickname` which is behind a shared reference](#11-an-optionstring-field-returned-from-self) | An `Option<String>` field returned from `&self` |
| 12 | `E0508` | [cannot move out of type `[String]`, a non-copy slice](#12-the-first-string-of-a-slice) | The first `String` of a slice |
| 13 | `E0106` | [missing lifetime specifier](#13-str-with-nothing-to-borrow-from) | `-> &str` with nothing to borrow from |
| 14 | `E0515` | [cannot return reference to local variable `name`](#14-static-str-pointing-at-a-local) | `-> &'static str` pointing at a local |
| 15 | `E0515` | [cannot return reference to temporary value](#15-str-pointing-at-a-temporary) | `-> &str` pointing at a temporary |
| 16 | `E0277` | [the size for values of type `str` cannot be known at compilation time](#16-a-str-parameter) | A `str` parameter |
| 17 | `E0277` | [the size for values of type `str` cannot be known at compilation time](#17-a-let-of-type-str) | A `let` of type `str` |

## Returning something with no size

### 1. `-> str`

```rust,compile_fail
fn first_word(s: &str) -> str {
    *s
}
```

```text title="rustc 1.98.0 on return_str.rs"
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> return_str.rs:1:27
  |
1 | fn first_word(s: &str) -> str {
  |                           ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
  = note: the return type of a function must have a statically known size

error[E0507]: cannot move out of `*s` which is behind a shared reference
 --> return_str.rs:2:5
  |
2 |     *s
  |     ^^ move occurs because `*s` has type `str`, which does not implement the `Copy` trait
```

**The mistake.** Returned by value means the `str` itself comes back, so the caller has to set aside room for it before the call, and `str` has no one size: `"hi"` is two bytes and `"hello"` is five. The second error is the body's. `*s` is the text that `s` points at, and handing it back by value would move it out from behind a shared reference. A size would not fix that half: entry 11 is the same move, for a type that has one.

**The fix.** Return something with a size that leads to the text. `&str` lends the caller's text back; `String` and `Box<str>` own a copy on the heap.

```rust
pub fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

pub fn first_word_owned(s: &str) -> String {
    first_word(s).to_owned()
}

pub fn first_word_boxed(s: &str) -> Box<str> {
    Box::from(first_word(s))
}
```

**Read:** [Returned by value — so `-> str` has no signature](../returned_by_value/README.md#so-str-has-no-signature), [`str` is unsized](../../14_Strings/str_is_unsized/README.md)

### 2. `-> [i32]`

```rust,compile_fail
pub fn first_two(numbers: &[i32]) -> [i32] {
    numbers[..2]
}
```

```text title="rustc 1.98.0 on return_slice.rs"
error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
 --> return_slice.rs:1:38
  |
1 | pub fn first_two(numbers: &[i32]) -> [i32] {
  |                                      ^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `[i32]`
  = note: the return type of a function must have a statically known size

error[E0508]: cannot move out of type `[i32]`, a non-copy slice
 --> return_slice.rs:2:5
  |
2 |     numbers[..2]
  |     ^^^^^^^^^^^^
  |     |
  |     cannot move out of here
  |     move occurs because value has type `[i32]`, which does not implement the `Copy` trait
```

**The mistake.** Entry 1 for a slice: `[i32]` is any number of `i32`s. The second error is the body's. `numbers[..2]` names two elements where they sit, in the caller's slice, and returning them by value would move them out. *a non-copy slice* describes `[i32]`, which is not `Copy` although `i32` is.

**The fix.** Lend the slice, or copy it into a `Vec`. When the length is fixed, return an array: `[i32; 2]` has its length in the type, so it has a size and can come back by value.

```rust
pub fn first_two(numbers: &[i32]) -> &[i32] {
    &numbers[..2]
}

pub fn first_two_owned(numbers: &[i32]) -> Vec<i32> {
    numbers[..2].to_vec()
}

pub fn first_two_array(numbers: &[i32]) -> [i32; 2] {
    [numbers[0], numbers[1]]
}
```

**Read:** [Arrays in signatures](../../26_Collections/arrays/arrays_in_signatures/README.md), [Step 2: some types have no size](../../12_Traits/how_to_learn_to_owned/types_with_no_size/README.md)

### 3. `-> Option<str>`

```rust,compile_fail
pub fn middle_name(full_name: &str) -> Option<str> {
    let mut parts = full_name.split(' ');
    parts.next();
    parts.next()
}
```

```text title="rustc 1.98.0 on option_of_str.rs"
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> option_of_str.rs:1:40
  |
1 | pub fn middle_name(full_name: &str) -> Option<str> {
  |                                        ^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
note: required by an implicit `Sized` bound in `Option`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/option.rs:598:0

error[E0308]: mismatched types
 --> option_of_str.rs:4:5
  |
1 | pub fn middle_name(full_name: &str) -> Option<str> {
  |                                        ----------- expected `Option<str>` because of return type
...
4 |     parts.next()
  |     ^^^^^^^^^^^^ expected `Option<str>`, found `Option<&str>`
  |
  = note: expected enum `Option<_>`
             found enum `Option<&_>`
```

**The mistake.** Wrapping it does not help. `Option<T>` holds its `T` inline, so every type parameter carries an implicit `T: Sized` bound, and `Option<str>` breaks it before the return rule is reached. The second error spells out what `split` hands back: `Option<&str>`, a pointer that may be absent.

**The fix.** Return what `split` already gives you. Elision ties the `&str` to `full_name`.

```rust
pub fn middle_name(full_name: &str) -> Option<&str> {
    let mut parts = full_name.split(' ');
    parts.next();
    parts.next()
}
```

**Read:** [Step 2 — `Sized` is the bound you never wrote](../../12_Traits/how_to_learn_to_owned/types_with_no_size/README.md#sized-is-the-bound-you-never-wrote)

### 4. `-> dyn Display`

```rust,compile_fail
use std::fmt::Display;

pub fn label(n: i32) -> dyn Display {
    n
}
```

```text title="rustc 1.98.0 on return_dyn.rs"
error[E0746]: return type cannot be a trait object without pointer indirection
 --> return_dyn.rs:3:25
  |
3 | pub fn label(n: i32) -> dyn Display {
  |                         ^^^^^^^^^^^ doesn't have a size known at compile-time
  |
help: consider returning an `impl Trait` instead of a `dyn Trait`
  |
3 - pub fn label(n: i32) -> dyn Display {
3 + pub fn label(n: i32) -> impl Display {
  |
help: alternatively, box the return type, and wrap all of the returned values in `Box::new`
  |
3 ~ pub fn label(n: i32) -> Box<dyn Display> {
4 ~     Box::new(n)
  |
```

**The mistake.** `dyn Display` means any type that implements `Display`: an `i32` here, a `String` in the next function, each a different size. The caller cannot reserve room for *any*, so a trait object cannot come back by value, and rustc offers the two ways round it.

**The fix.** `impl Display` stands for one concrete type the compiler reads off the body, so the size is known after all; the caller just cannot name the type. `Box<dyn Display>` returns a pointer, and the value lives on the heap.

```rust
use std::fmt::Display;

pub fn label(n: i32) -> impl Display {
    n
}

pub fn boxed_label(n: i32) -> Box<dyn Display> {
    Box::new(n)
}
```

**Read:** [Returning a trait](../../12_Traits/returning_a_trait/README.md), [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md)

### 5. `-> Fn(i32) -> i32`, with no `dyn` or `impl`

```rust,compile_fail
pub fn make_adder(n: i32) -> Fn(i32) -> i32 {
    move |x| x + n
}
```

```text title="rustc 1.98.0 on return_bare_fn.rs"
error[E0782]: expected a type, found a trait
 --> return_bare_fn.rs:1:30
  |
1 | pub fn make_adder(n: i32) -> Fn(i32) -> i32 {
  |                              ^^^^^^^^^^^^^^
  |
help: use `impl Fn(i32) -> i32` to return an opaque type, as long as you return a single underlying type
  |
1 | pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
  |                              ++++
help: alternatively, you can return an owned trait object
  |
1 | pub fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
  |                              +++++++               +
```

**The mistake.** A closure's type has no name you can write, so the return type has to name a trait instead, and a trait written where a type goes needs `dyn` or `impl`. Answers from before edition 2021 write it bare: on editions 2015 and 2018 the same file gets a deprecation warning and entry 4's `E0746`, and adding only `dyn` gets `E0746` on every edition. Each closure is its own type, with its own size.

**The fix.** Both of rustc's suggestions compile. `impl Fn` returns the closure itself, by value; `Box<dyn Fn>` returns a pointer to it.

```rust
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

pub fn make_boxed_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)
}
```

**Read:** [What a closure is](../../23_Closures/what_a_closure_is/README.md), [Returning a trait](../../12_Traits/returning_a_trait/README.md)

### 6. `-> impl Display` with two types behind it

```rust,compile_fail
use std::fmt::Display;

pub fn label(n: i32) -> impl Display {
    if n < 0 {
        "negative"
    } else {
        n
    }
}
```

```text title="rustc 1.98.0 on impl_two_types.rs"
error[E0308]: `if` and `else` have incompatible types
 --> impl_two_types.rs:7:9
  |
4 | /     if n < 0 {
5 | |         "negative"
  | |         ---------- expected because of this
6 | |     } else {
7 | |         n
  | |         ^ expected `&str`, found `i32`
8 | |     }
  | |_____- `if` and `else` have incompatible types
  |
help: you could change the return type to be a boxed trait object
  |
3 - pub fn label(n: i32) -> impl Display {
3 + pub fn label(n: i32) -> Box<dyn Display> {
  |
help: if you change the return type to expect trait objects, box the returned expressions
  |
5 ~         Box::new("negative")
6 |     } else {
7 ~         Box::new(n)
  |
```

**The mistake.** `impl Display` is still one type, chosen once for the whole function, which is how its size is fixed before the call. A `&str` from one branch and an `i32` from the other are two types, so rustc's help switches to `Box<dyn Display>`, which can point at either.

**The fix.** Box both, as rustc suggests, or make both branches the same type.

```rust
use std::fmt::Display;

pub fn label(n: i32) -> Box<dyn Display> {
    if n < 0 {
        Box::new("negative")
    } else {
        Box::new(n)
    }
}

pub fn label_text(n: i32) -> impl Display {
    if n < 0 {
        String::from("negative")
    } else {
        n.to_string()
    }
}
```

**Read:** [Returning a trait — `impl Trait` when the answer is one type](../../12_Traits/returning_a_trait/README.md#impl-trait-when-the-answer-is-one-type)

### 7. `-> T` with `T: ?Sized`

```rust,compile_fail
pub fn copy_out<T: ?Sized>(x: &T) -> T {
    *x
}
```

```text title="rustc 1.98.0 on generic_unsized_return.rs"
error[E0277]: the size for values of type `T` cannot be known at compilation time
 --> generic_unsized_return.rs:1:38
  |
1 | pub fn copy_out<T: ?Sized>(x: &T) -> T {
  |                 -                    ^ doesn't have a size known at compile-time
  |                 |
  |                 this type parameter needs to be `Sized`
  |
  = note: the return type of a function must have a statically known size
help: consider removing the `?Sized` bound to make the type parameter `Sized`
  |
1 - pub fn copy_out<T: ?Sized>(x: &T) -> T {
1 + pub fn copy_out<T>(x: &T) -> T {
  |

error[E0507]: cannot move out of `*x` which is behind a shared reference
 --> generic_unsized_return.rs:2:5
  |
2 |     *x
  |     ^^ move occurs because `*x` has type `T`, which does not implement the `Copy` trait
  |
help: if `T` implemented `Clone`, you could clone the value
 --> generic_unsized_return.rs:1:17
  |
1 | pub fn copy_out<T: ?Sized>(x: &T) -> T {
  |                 ^ consider constraining this type parameter with `Clone`
2 |     *x
  |     -- you could clone this value
```

**The mistake.** `?Sized` removes the one bound that lets a `T` be returned, and rustc's first help puts it back. The second error is the body: `*x` would move the `T` out from behind a `&T`, and the second help reaches for `Clone`.

**The fix.** Take both helps: drop `?Sized` and clone. `T: ?Sized + Clone` also compiles and means the same, because `Clone` brings `Sized` back as its supertrait. To accept `str` and `[i32]` as well, return what `ToOwned` makes: `T::Owned` is `String` for a `str`, and an associated type is `Sized` unless it says otherwise.

```rust
pub fn copy_out<T: Clone>(x: &T) -> T {
    x.clone()
}

pub fn copy_out_unsized<T: ?Sized + ToOwned>(x: &T) -> T::Owned {
    x.to_owned()
}
```

**Read:** [Every `ToOwned` error — `B: ToOwned` without `?Sized`](../../12_Traits/how_to_learn_to_owned/to_owned_errors/README.md#7-b-toowned-without-sized), [Where the bound goes](../../22_Generics/where_the_bound_goes/README.md)

## `Clone`, and other methods that return `Self`

### 8. `#[derive(Clone)]` on a struct holding a `str`

```rust,compile_fail
#[derive(Clone)]
pub struct Name(pub str);
```

```text title="rustc 1.98.0 on derive_clone_unsized.rs"
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> derive_clone_unsized.rs:2:12
  |
1 | #[derive(Clone)]
  |          ----- in this derive macro expansion
2 | pub struct Name(pub str);
  |            ^^^^ doesn't have a size known at compile-time
  |
  = help: within `Name`, the trait `Sized` is not implemented for `str`
note: required because it appears within the type `Name`
 --> derive_clone_unsized.rs:2:12
  |
2 | pub struct Name(pub str);
  |            ^^^^
note: required by a bound in `Clone`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/core/src/clone.rs:194:0

error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> derive_clone_unsized.rs:1:10
  |
1 | #[derive(Clone)]
  |          ^^^^^ doesn't have a size known at compile-time
  |
  = help: within `Name`, the trait `Sized` is not implemented for `str`
note: required because it appears within the type `Name`
 --> derive_clone_unsized.rs:2:12
  |
2 | pub struct Name(pub str);
  |            ^^^^
  = note: the return type of a function must have a statically known size

error[E0277]: the trait bound `str: Clone` is not satisfied
 --> derive_clone_unsized.rs:2:17
  |
1 | #[derive(Clone)]
  |          ----- in this derive macro expansion
2 | pub struct Name(pub str);
  |                 ^^^^^^^ the trait `Clone` is not implemented for `str`
  |
help: the trait `Clone` is implemented for `String`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/string.rs:2377:0
```

**The mistake.** A struct whose last field is a `str` is legal, and unsized like its field. `Clone` is declared `pub trait Clone: Sized`, because `fn clone(&self) -> Self` returns `Self` by value, and the first two errors are those two facts: the bound on the trait, and the return type of the `clone` the derive writes. The third is the derive cloning each field, and `str` has no `clone`.

**The fix.** Hold the text through a pointer. `Box<str>` is a pointer and a length, 16 bytes on a 64-bit target; `String` adds a capacity; `Rc<str>` makes each clone a count bump instead of a copy.

```rust
#[derive(Clone)]
pub struct Name(pub Box<str>);
```

**Read:** [Returned by value — why `str` is not `Clone`](../returned_by_value/README.md#why-str-is-not-clone), [Step 4: `Clone` hands back `Self`](../../12_Traits/how_to_learn_to_owned/clone_returns_self/README.md#self-in-self-out), [What a clone costs](../what_a_clone_costs/README.md)

### 9. `Box<dyn Clone>`

```rust,compile_fail
pub struct Gallery {
    pub pictures: Vec<Box<dyn Clone>>,
}
```

```text title="rustc 1.98.0 on dyn_clone.rs"
error[E0038]: the trait `Clone` is not dyn compatible
 --> dyn_clone.rs:2:27
  |
2 |     pub pictures: Vec<Box<dyn Clone>>,
  |                           ^^^^^^^^^ `Clone` is not dyn compatible
  |
  = note: the trait is not dyn compatible because it requires `Self: Sized`
  = note: for a trait to be dyn compatible it needs to allow building a vtable
          for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
```

**The mistake.** A trait object is how you hold *some* type without knowing its size, and `Clone` requires `Self: Sized`, so there is no `dyn Clone`. The return type is the reason: a `clone` called through the object would hand back a value whose type, and so whose size, the caller cannot know.

**The fix.** Declare your own trait whose copy comes back through a pointer. `Box<dyn Picture>` has one size whatever it points at.

```rust
pub trait Picture {
    fn clone_box(&self) -> Box<dyn Picture>;
}

#[derive(Clone)]
pub struct Photo(pub String);

impl Picture for Photo {
    fn clone_box(&self) -> Box<dyn Picture> {
        Box::new(self.clone())
    }
}

pub struct Gallery {
    pub pictures: Vec<Box<dyn Picture>>,
}

impl Gallery {
    pub fn duplicate(&self) -> Gallery {
        Gallery { pictures: self.pictures.iter().map(|p| p.clone_box()).collect() }
    }
}
```

**Read:** [Dyn compatibility](../../39_API_Design/dyn_compatibility/README.md), [Step 4: `Clone` hands back `Self`](../../12_Traits/how_to_learn_to_owned/clone_returns_self/README.md)

### 10. A trait method returning `Self`, used as `dyn`

```rust,compile_fail
pub trait Shape {
    fn area(&self) -> f64;
    fn duplicate(&self) -> Self;
}

pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
}
```

```text title="rustc 1.98.0 on dyn_returns_self.rs"
error[E0038]: the trait `Shape` is not dyn compatible
 --> dyn_returns_self.rs:7:25
  |
7 |     pub shapes: Vec<Box<dyn Shape>>,
  |                         ^^^^^^^^^ `Shape` is not dyn compatible
  |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
 --> dyn_returns_self.rs:3:28
  |
1 | pub trait Shape {
  |           ----- this trait is not dyn compatible...
2 |     fn area(&self) -> f64;
3 |     fn duplicate(&self) -> Self;
  |                            ^^^^ ...because method `duplicate` references the `Self` type in its return type
  = help: consider moving `duplicate` to another trait
```

**The mistake.** Entry 9 with a trait of your own. `duplicate` returns `Self` by value, and called through a `dyn Shape` that `Self` could be any shape of any size, so the trait cannot be made into an object. rustc names the method and points at the `Self` in its return type.

**The fix.** `where Self: Sized` keeps `duplicate` for concrete types and leaves it out of the vtable, so `dyn Shape` is allowed; calling `duplicate` through one is refused with *the `duplicate` method cannot be invoked on a trait object*. When the copy has to go through the object, return `Box<dyn Shape>` instead, as in entry 9.

```rust
pub trait Shape {
    fn area(&self) -> f64;
    fn duplicate(&self) -> Self
    where
        Self: Sized;
}

pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
}
```

**Read:** [Dyn compatibility](../../39_API_Design/dyn_compatibility/README.md)

## Returning by value moves it out

### 11. An `Option<String>` field returned from `&self`

```rust,compile_fail
pub struct User {
    pub name: String,
    pub nickname: Option<String>,
}

impl User {
    pub fn nickname(&self) -> Option<String> {
        self.nickname
    }
}
```

```text title="rustc 1.98.0 on move_option_field.rs"
error[E0507]: cannot move out of `self.nickname` which is behind a shared reference
 --> move_option_field.rs:8:9
  |
8 |         self.nickname
  |         ^^^^^^^^^^^^^ move occurs because `self.nickname` has type `Option<String>`, which does not implement the `Copy` trait
  |
help: consider cloning the value if the performance cost is acceptable
  |
8 |         self.nickname.clone()
  |                      ++++++++
```

**The mistake.** Returning by value hands the caller the value itself. From `&self` that means moving `nickname` out of a `User` the method only borrowed, and a shared reference cannot give anything away. rustc's help clones, which copies the name into a new `String` on every call.

**The fix.** Lend it: `as_deref` turns a `&Option<String>` into an `Option<&str>`. Keep the clone for a caller that has to own the name.

```rust
pub struct User {
    pub name: String,
    pub nickname: Option<String>,
}

impl User {
    pub fn nickname(&self) -> Option<&str> {
        self.nickname.as_deref()
    }

    pub fn nickname_owned(&self) -> Option<String> {
        self.nickname.clone()
    }
}
```

**Read:** [Every reference error — returning a `String` field from a `&self` method](../references/reference_errors/README.md#11-returning-a-string-field-from-a-self-method), [Ownership and moves](../ownership_and_moves/README.md)

### 12. The first `String` of a slice

```rust,compile_fail
pub fn first(names: &[String]) -> String {
    names[0]
}
```

```text title="rustc 1.98.0 on move_out_of_slice.rs"
error[E0508]: cannot move out of type `[String]`, a non-copy slice
 --> move_out_of_slice.rs:2:5
  |
2 |     names[0]
  |     ^^^^^^^^
  |     |
  |     cannot move out of here
  |     move occurs because `names[_]` has type `String`, which does not implement the `Copy` trait
  |
help: consider cloning the value if the performance cost is acceptable
  |
2 |     names[0].clone()
  |             ++++++++
```

**The mistake.** `names[0]` names the element where it sits, inside the caller's slice, and returning it by value would move it out and leave a hole. An `i32` would be copied out instead; the second line of the error says why this one is not: `names[_]` has type `String`, which is not `Copy`.

**The fix.** Lend the element, or clone it, as rustc suggests.

```rust
pub fn first(names: &[String]) -> &str {
    &names[0]
}

pub fn first_owned(names: &[String]) -> String {
    names[0].clone()
}
```

**Read:** [Every array error — moving one `String` out of an array](../../26_Collections/arrays/array_errors/README.md#13-moving-one-string-out-of-an-array), [Copy or move](../copy_or_move/README.md)

## Returning a borrow instead

### 13. `-> &str` with nothing to borrow from

```rust,compile_fail
pub fn default_name() -> &str {
    let name = String::from("guest");
    &name
}
```

```text title="rustc 1.98.0 on return_ref_no_input.rs"
error[E0106]: missing lifetime specifier
 --> return_ref_no_input.rs:1:26
  |
1 | pub fn default_name() -> &str {
  |                          ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
1 | pub fn default_name() -> &'static str {
  |                           +++++++
help: instead, you are more likely to want to return an owned value
  |
1 - pub fn default_name() -> &str {
1 + pub fn default_name() -> String {
  |
```

**The mistake.** A reference that comes back has to point at something that outlives the call, and a function with no reference parameters has nothing to borrow from. Of rustc's two helps, the first leads to entry 14 and the second is the fix.

**The fix.** Return the `String` itself, by value. The caller's slot receives the pointer, length and capacity, and the text on the heap stays where it is.

```rust
pub fn default_name() -> String {
    String::from("guest")
}
```

**Read:** [Returned by value — who reserves the room](../returned_by_value/README.md#who-reserves-the-room), [Lifetime annotations](../lifetime_annotations/README.md)

### 14. `-> &'static str` pointing at a local

```rust,compile_fail
pub fn default_name() -> &'static str {
    let name = String::from("guest");
    &name
}
```

```text title="rustc 1.98.0 on return_ref_to_local.rs"
error[E0515]: cannot return reference to local variable `name`
 --> return_ref_to_local.rs:3:5
  |
3 |     &name
  |     ^^^^^ returns a reference to data owned by the current function
```

**The mistake.** `'static` satisfies the signature, and the body cannot keep the promise: `name` is dropped when the function returns, its frame is released for [the next call to reuse](../a_stack_slot_is_reused/README.md), and its heap bytes are freed. A `&'static str` has to point at a literal, a `static`, or text leaked on purpose.

**The fix.** A literal when the text never changes; the `String` itself when it is built.

```rust
pub fn default_name() -> &'static str {
    "guest"
}

pub fn greeting_for(user: &str) -> String {
    format!("hello, {user}")
}
```

**Read:** [A stack slot is reused — what Rust does instead](../a_stack_slot_is_reused/README.md#what-rust-does-instead), [Every reference error — returning a reference to a local](../references/reference_errors/README.md#18-returning-a-reference-to-a-local)

### 15. `-> &str` pointing at a temporary

```rust,compile_fail
pub fn shout(text: &str) -> &str {
    &text.to_uppercase()
}
```

```text title="rustc 1.98.0 on return_ref_to_temporary.rs"
error[E0515]: cannot return reference to temporary value
 --> return_ref_to_temporary.rs:2:5
  |
2 |     &text.to_uppercase()
  |     ^-------------------
  |     ||
  |     |temporary value created here
  |     returns a reference to data owned by the current function
```

**The mistake.** The signature is fine: elision ties the result to `text`. The body does not borrow from `text`. `to_uppercase` returns a new `String` by value, into a temporary that nothing owns once the function ends, and the reference would outlive it.

**The fix.** The `String` is already the answer. Return it.

```rust
pub fn shout(text: &str) -> String {
    text.to_uppercase()
}
```

**Read:** [Temporary lifetimes](../temporary_lifetimes/README.md)

## The same rule for the other slots

### 16. A `str` parameter

```rust,compile_fail
pub fn byte_count(text: str) -> usize {
    text.len()
}
```

```text title="rustc 1.98.0 on str_parameter.rs"
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> str_parameter.rs:1:25
  |
1 | pub fn byte_count(text: str) -> usize {
  |                         ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
help: function arguments must have a statically known size, borrowed types always have a known size
  |
1 | pub fn byte_count(text: &str) -> usize {
  |                         +
```

**The mistake.** A parameter is a slot the caller fills before the call, so it needs a size for the same reason a return value does. The `help:` says why the fix works: *borrowed types always have a known size*.

**The fix.** Take a `&str`.

```rust
pub fn byte_count(text: &str) -> usize {
    text.len()
}
```

**Read:** [The call stack — the argument becomes a local](../the_call_stack/README.md#the-argument-becomes-a-local)

### 17. A `let` of type `str`

```rust,compile_fail
pub fn byte_count(text: &str) -> usize {
    let copy: str = *text;
    copy.len()
}
```

```text title="rustc 1.98.0 on str_local.rs"
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> str_local.rs:2:9
  |
2 |     let copy: str = *text;
  |         ^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
  = note: all local variables must have a statically known size
help: consider borrowing here
  |
2 |     let copy: &str = *text;
  |               +
```

**The mistake.** A local is a slot in the frame, sized when the function is compiled, and `*text` is a `str` with no size to size it by. rustc's help adds a `&` to the type and not to the value, so `let copy: &str = *text;` fails in turn, with `E0308` *expected `&str`, found `str`*.

**The fix.** A copy of the text is a `String`. If you did not need a copy, use `text`.

```rust
pub fn byte_count(text: &str) -> usize {
    let copy: String = text.to_owned();
    copy.len()
}
```

**Read:** [`str` is unsized — so a `str` can only live behind a pointer](../../14_Strings/str_is_unsized/README.md#so-a-str-can-only-live-behind-a-pointer), [Stack and heap](../stack_and_heap/README.md)

## See also

- [Returned by value](../returned_by_value/README.md) — why the caller needs one size, fixed at compile time, before the call
- [Lints around returning by value](../returned_by_value_lints/README.md) — what the compiler only warns about, and what nothing flags
- [Returned by value: reading](../returned_by_value_resources/README.md) — where each error is explained at length
- [Step 2: some types have no size](../../12_Traits/how_to_learn_to_owned/types_with_no_size/README.md) — `str`, `[T]` and `Path`, and the pointer that carries their length
- [Step 4: `Clone` hands back `Self`](../../12_Traits/how_to_learn_to_owned/clone_returns_self/README.md) — why `str` can never be `Clone`, the cause of entries 8 and 9
- [`str` is unsized](../../14_Strings/str_is_unsized/README.md) — the same fact from the string side
- [The call stack](../the_call_stack/README.md) — the frame a return value, a parameter and a local all need room in
- [A stack slot is reused](../a_stack_slot_is_reused/README.md) — what entry 14's `E0515` protects you from
- [Returning a trait](../../12_Traits/returning_a_trait/README.md) — `impl Trait` and `Box<dyn Trait>`, entries 4 to 6 as a lesson
- [Every reference error, and its fix](../references/reference_errors/README.md) — the same kind of page for `&` and `&mut`
- [Every `ToOwned` error, and its fix](../../12_Traits/how_to_learn_to_owned/to_owned_errors/README.md) — the same kind of page for `ToOwned`, `Clone` and `Cow`
- [rustc's error index ↗](https://doc.rust-lang.org/error_codes/error-index.html) — the long explanation behind each code, also printed by `rustc --explain E0746`

## Po polsku

Siedemnaście błędów kompilatora, które pojawiają się, gdy funkcja ma zwrócić wartość bez znanego rozmiaru, gdy zwrócenie wartości wymagałoby przeniesienia jej spod pożyczki, albo gdy w miejsce wartości zwracana jest referencja do czegoś, co zaraz zniknie. Przy każdym: kod, który go wywołuje, dokładny komunikat rustc 1.98.0, na czym polega pomyłka i poprawka, która się kompiluje.

Zwrot „przez wartość” oznacza, że wraca sama wartość, a nie wskaźnik do niej, więc wywołujący musi przed wywołaniem zarezerwować na nią miejsce o jednym, znanym w czasie kompilacji rozmiarze. `str`, `[T]` i `dyn Trait` takiego rozmiaru nie mają, dlatego nie da się ich zwrócić, a `Clone` (`fn clone(&self) -> Self`) wymaga `Sized`. Poprawki to zawsze wskaźnik o stałym rozmiarze: `&str`, `String`, `Box<str>`, `impl Trait` albo `Box<dyn Trait>`. Siedem z siedemnastu to `E0277`; linia pod komunikatem mówi, którego miejsca dotyczy: typu zwracanego, zmiennej lokalnej, argumentu czy parametru typu w `Option`.

**Szukaj po polsku:** `rust E0277 the return type of a function must have a statically known size` · `rust E0746` · `rust E0038 dyn compatible Clone` · `rust E0515 zwracanie referencji`
