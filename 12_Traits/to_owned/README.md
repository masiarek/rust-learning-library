# `ToOwned`: `Clone` for types whose owned twin is a different type

**Level:** 201 · working knowledge

**One line:** `Clone` goes `&T → T`; `ToOwned` goes `&T →` whatever the owned version of `T` actually is — which for `str` is `String`, a different type entirely.

```rust
fn main() {
    let name: &str = "Adam";
    let owned: String = name.to_owned();          // &str  -> String
    let nums: &[i32] = &[3, 1, 2];
    let owned_nums: Vec<i32> = nums.to_owned();   // &[i32] -> Vec<i32>
    println!("{owned} {owned_nums:?}");           // Adam [3, 1, 2]
}
```

Two different owned types, one method name. `Clone` cannot express that: its signature is `fn clone(&self) -> Self`, so the answer is always the same type it started with.

## The trait, in full

```rust
pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;                 // required
    fn clone_into(&self, target: &mut Self::Owned) {   // provided
        *target = self.to_owned();                     // the default just allocates
    }
}
```

The associated type is the whole mechanism — `Owned` is what makes `str`'s answer `String` and `[i32]`'s answer `Vec<i32>`. The bound on it, `Borrow<Self>`, is the promise that you can always get back to the borrowed form, which is what lets [`Cow` ↗](https://doc.rust-lang.org/std/borrow/enum.Cow.html) hold either half and hand out a `&str` regardless.

## Why it has to exist

Because `Clone` requires `Sized`, and `str` is not sized.

That single sentence explains the behaviour everybody trips over. `str` has no `Clone` impl at all — it cannot have one. But `&str` does, because shared references are `Copy` and therefore `Clone`. So when you write `name.clone()` on a `&str`, method lookup goes looking for a `clone` whose receiver is `&str`, does not find one on `str`, falls through to the reference's own impl, and hands you back **another `&str`**.

rustc will tell you so itself:

```text
warning: call to `.clone()` on a reference in this situation does nothing
   |
35 |     let cloned: &str = name.clone();
   |                            ^^^^^^^^ help: remove this redundant call
   |
   = note: the type `str` does not implement `Clone`, so calling `clone` on `&str`
           copies the reference, which does not do anything and can be removed
   = note: `#[warn(noop_method_call)]` on by default
```

On a `&String` the same syntax does something different, and for the same reason read the other way: `String` **does** implement `Clone`, so lookup finds that impl first and `r.clone()` gives you a `String`. Same characters, different outcome, decided by whether the pointee is `Sized`.

## For everything else, they are the same call

```rust
impl<T: Clone> ToOwned for T { type Owned = T; ... }
```

That blanket impl means every `Clone` type gets `to_owned()` for free, doing exactly what `clone()` does. So outside the unsized cases the choice is **stylistic**, and the community genuinely disagrees about it — one camp finds `.to_string()` clearer, another finds `.to_owned()` more honest about what is being bought. The rule of thumb worth keeping: `clone()` reads as *"I have a `T` and want another"*, `to_owned()` as *"I have a borrow and want to own it"*.

### The argument you will meet, and its expiry date

Search this question and you will find the same advice everywhere, usually word for word: *use `to_owned()` on a string literal, because `to_string()` is generic, goes through `Display`, and may allocate more than once.* That was true when it was written, in 2015. It stopped being true in **April 2016**, when [`ToString` was specialized for `str` ↗](https://github.com/rust-lang/rust/pull/32586) — the case everybody was worried about no longer runs the formatting machinery at all.

Nine years later the advice is still being republished with the performance reasoning intact, sometimes quoting the original thread's own *"this may be fixed in the future with specialization"* caveat without noticing that it was. Measured on rustc 1.97.1 with `-O`, `String::from`, `.into()`, `.to_string()` and `.to_owned()` land within a couple of nanoseconds of each other on a 13-byte literal. The one real outlier is `format!("a literal")`, about 30% slower — which clippy flags anyway, as `useless_format`, for the unrelated reason that it is a formatting call with nothing to format.

**Check what a benchmark was built with before believing it.** The most-linked measurement of this question prints `target/debug` in its own transcript: at `-O0` the five spellings differ by about 2%, a spread far too small to carry the conclusion drawn from it. The conclusion — `format!` is the slow one — happens to be right, but the method could not have shown it.

So the performance argument is dead, and the argument that outlived it is about **documentation**. dtolnay's case, [made in that same thread in 2017 ↗](https://users.rust-lang.org/t/to-string-vs-to-owned-for-string-literals/1441/6), is that `&str` and `String` are both strings, so *"convert this string to a string"* names nothing; what actually differs is ownership, and `to_owned()` is the spelling that says so at the point where a reader is asking why the conversion is there at all. Same conclusion as the 2015 advice, reached for a reason that does not expire.

One naming note, because it is a common slip: `to_` methods do **not** consume `self` — `to_owned(&self)` borrows. `into_` is the prefix that means the value is consumed.

## The trap that blanket impl sets

On an `Rc` or `Arc`, `.to_owned()` clones the **pointer**, not the data:

```rust
let shared = Rc::new(String::from("ballot"));
let second = shared.to_owned();
// strong_count is now 2, and Rc::ptr_eq(&shared, &second) is true
```

`Rc<T>` is `Clone`, so the blanket impl applies and `to_owned` *is* `clone` — which for an `Rc` means bumping the reference count. If you wanted the `String` copied, you have to dereference first: `(*shared).clone()`. Reaching for `to_owned` because it sounds like it makes an independent copy is exactly the wrong instinct here.

## Can you implement it yourself?

Almost never, and the two refusals are worth meeting because between them they explain the shape of the whole trait.

**A type that is `Clone` cannot have one.** The blanket impl already covers it, so your impl is a second one:

```text
error[E0119]: conflicting implementations of trait `ToOwned` for type `DataRef<'_>`
   |
   = note: conflicting implementation in crate `alloc`:
           - impl<T> ToOwned for T
             where T: Clone;
```

Since essentially every ordinary type derives `Clone`, that rules out essentially every ordinary type.

**A reference-like type has nowhere to put one either**, even after you drop the `Clone`. `type Owned` is bound by `Borrow<Self>`, and a `DataOwned { text: String }` cannot hand out a `&DataRef<'_>` — there is no `DataRef` stored anywhere to lend:

```text
error[E0277]: the trait bound `DataOwned: Borrow<DataRef<'_>>` is not satisfied
   |
note: required by a bound in `std::borrow::ToOwned::Owned`
```

Be precise about what that error proves, because it is easy to over-read: the type checker is refusing the *missing bound*, not the design. Write `impl Borrow<DataRef<'_>> for DataOwned` with a `todo!()` body and the `ToOwned` impl compiles perfectly well — the wall is one step further on, at the moment you have to write a `borrow` that returns a reference to a `DataRef` the `DataOwned` never stored. So the blocker is semantic rather than syntactic, and no compiler error will state it for you.

Which is why every impl in the standard library is on an **unsized referent** rather than on a reference — `str`, `CStr`, `OsStr`, `Path`, `[T]`. Those are the types that genuinely have a separate owned form and are always met through a pointer, which is the situation the trait was shaped for.

What *does* compile is a `Sized` type that is not `Clone`, with `type Owned = Self` — the run below has one. The blanket `impl<T> Borrow<T> for T` satisfies the bound and nothing conflicts. It also buys nothing: it is `Clone` under a different name, and adding `#[derive(Clone)]` later turns it into the `E0119` above. If you want a `.to_owned()` method on your own type, write an inherent one and skip the trait.

The genuinely blocked case is a validated wrapper — an "ASCII-only string" newtype you want to use with [`Cow`](../../18_Ownership/clone_on_write/README.md). Doing it properly needs a `#[repr(transparent)]` wrapper around `str` and an `unsafe` pointer cast in `borrow`, because `Borrow`/`ToOwned` predate GATs and there is no safe way to make a `&MyNewtype` out of a `&str`. That limitation is still open: an [`IntoOwned` pre-RFC ↗](https://internals.rust-lang.org/t/pre-rfc-intoowned-trait-that-harmonizes-cow-and-toowned/23609) from late 2025 is one attempt at harmonizing the pair, and had not converged on a signature that survives the existing blanket impls.

## The trap in generic code

`<&str as ToOwned>::Owned` is **`&str`**, not `String`. The blanket impl applies to the reference itself, so a bound written on the reference resolves to the wrong side of the pair:

```rust
fn foo<S, T>(s: S) -> T where S: ToOwned<Owned = T>, T: Borrow<S> { s.to_owned() }
let _s: String = foo("hi");    // E0308: expected `String`, found `&str`

fn bar<S: ?Sized, T>(s: &S) -> T where S: ToOwned<Owned = T>, T: Borrow<S> { s.to_owned() }
let s: String = bar("hi");     // "hi"
```

`S` unifies with `&str`, so `T` is `&str` and the annotation is what breaks. The fix is to take `&S` rather than `S`, so `S` is `str` and `T` is `String` — and then **`S: ?Sized` is not optional**, because `str` is precisely the unsized case the trait exists for. Leave it off and the signature that was supposed to be the fix fails on its own bound instead: *"the size for values of type `str` cannot be known at compilation time … required by an implicit `Sized` bound in `foo`"*. Two errors, one cause, and the second is the page's whole thesis arriving as a compiler message. This was [filed as a diagnostics bug in 2016 ↗](https://github.com/rust-lang/rust/issues/31228) and closed in 2020 once the error learned to suggest a conversion method; the underlying surprise is unchanged, and it is the same one as section 2b above, one level of generics up.

## `clone_into`, the provided method

`clone_into(&self, target: &mut Self::Owned)` writes into a buffer you already have instead of allocating a new one. `str`'s impl overrides it to reuse the `String`'s existing capacity — the run below fills a 64-byte buffer and the capacity does not move. Same family as [`mem::take`](../../GLOSSARY.md): the standard library's habit of offering the in-place spelling beside the allocating one.

## The verified output

<!-- output:to_owned -->
*Verified output of [`to_owned.rs`](examples/to_owned.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The owned twin is a different TYPE — that is the whole point
   "Adam"      (&str)    .to_owned() -> String "Adam"
   [3, 1, 2]   (&[i32])  .to_owned() -> Vec    [3, 1, 2]

2. `.clone()` cannot do that: on a &str it hands back another &str
   name.clone()    is still a &str: "Adam"
   needs_string(name.to_owned()) = 4
   needs_string(name.clone())    would be E0308 — expected String, found &str
   size_of::<&str>()   = 16   pointer + length
   size_of::<String>() = 24   pointer + length + capacity

2b. WHY .clone() behaves differently on &str than on &String
   (&String).clone() -> String  "Ada"   <- String: Clone exists
   (&str).clone()    -> &str    "Adam"  <- str: Clone does NOT
   str is !Sized, and Clone requires Sized. That is the whole reason
   ToOwned exists at all.

3. For everything that is Clone, they are the SAME call
   s.clone()    = "Ada"
   s.to_owned() = "Ada"   <- blanket impl<T: Clone> ToOwned for T

4. The trap that blanket impl sets: on an Rc it clones the POINTER
   strong_count before      = 1
   strong_count after       = 2
   same allocation?           true
   (*shared).clone() is the real copy: "ballot"

5. `clone_into`: the provided method that reuses a buffer
   capacity 64 -> 64 , contents "reuse me"   (no new allocation)

6. Why the trait is shaped that way: Cow pays only when it must
   "one  two"   -> "one two"   owned — one allocation
   "one two"    -> "one two"   borrowed — nothing allocated

7. Implementing it yourself: legal, and pointless
   Tally { seats: 3, name: "Ada" }.to_owned() -> Tally { seats: 3, name: "Ada" }
   type Owned = Self, so this is Clone wearing a different name.
   Give Tally a #[derive(Clone)] and it is E0119 instead: the
   blanket impl<T: Clone> ToOwned for T already covers it.
```
<!-- /output -->

## Practice

**Predict the owned twin before you run it.** For a `&str`, a `&[i32]`, a `&Path`, a `&String`, a plain `i32` and an `Rc<String>`, write down what `.to_owned()` returns — the *type*, not the value — then check yourself with [`std::any::type_name_of_val` ↗](https://doc.rust-lang.org/std/any/fn.type_name_of_val.html).

Two of the six are the ones worth getting wrong. Say what `42.to_owned()` gives you and why that is not a bug but the blanket impl working as designed. Then say how many heap buffers exist after `Rc::new(String::from("ballot")).to_owned()`, and what you would have written instead to get two.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:to_owned_kata -->
*[`to_owned_kata.rs`](examples/to_owned_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: `.to_owned()` answers to the SOURCE, not to `String`.
//!
//!   rustc --edition 2024 to_owned_kata.rs -o /tmp/tok && /tmp/tok

use std::any::type_name_of_val;
use std::path::Path;
use std::rc::Rc;

fn main() {
    println!("1. Four receivers, four different owned twins");
    let text: &str = "Ada";
    let nums: &[i32] = &[5, 2, 0];
    let path: &Path = Path::new("/tmp/ballot");
    let already: String = String::from("Ben");

    let a = text.to_owned();
    let b = nums.to_owned();
    let c = path.to_owned();
    let d = (&already).to_owned();

    println!("   &str     .to_owned() -> {}", type_name_of_val(&a));
    println!("   &[i32]   .to_owned() -> {}", type_name_of_val(&b));
    println!("   &Path    .to_owned() -> {}", type_name_of_val(&c));
    println!("   &String  .to_owned() -> {}", type_name_of_val(&d));
    println!("   One method name, four answers: `type Owned` is chosen by the source.");

    println!();
    println!("2. So it is NOT a stringifying operation");
    // The prediction most people get wrong. `i32: Clone`, so the blanket
    // `impl<T: Clone> ToOwned for T` applies and the owned twin of an i32 is
    // an i32 — this does not produce text and never could.
    let n = 42.to_owned();
    println!("   42.to_owned()  -> {:<3}  ({})", n, type_name_of_val(&n));
    println!("   42.to_string() -> {:<5}({})", format!("{:?}", 42.to_string()), type_name_of_val(&42.to_string()));
    println!("   `to_string` is about TEXT. `to_owned` is about OWNERSHIP.");
    println!("   They coincide on a &str and nowhere else.");

    println!();
    println!("3. The trap: on an Rc it clones the POINTER");
    let shared = Rc::new(String::from("ballot"));
    let second = shared.to_owned();
    println!("   type          {}", type_name_of_val(&second));
    println!("   strong_count  {}", Rc::strong_count(&shared));
    println!("   same buffer?  {}", Rc::ptr_eq(&shared, &second));
    let deep: String = (*shared).clone();
    println!("   the real copy is (*shared).clone() -> {deep:?}");

    println!();
    println!("4. Why `Owned: Borrow<Self>` is in the trait");
    // Every owned twin above can lend its borrowed half back, which is what
    // lets one signature take either side of the pair.
    fn shout(s: &str) -> String { s.to_uppercase() }
    println!("   shout(&a) = {:?}   <- String lends a &str back", shout(&a));
    println!("   the bound is the round trip, and it is what makes Cow possible.");
}
```
<!-- /source -->

<!-- output:to_owned_kata -->
*Verified output of [`to_owned_kata.rs`](examples/to_owned_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Four receivers, four different owned twins
   &str     .to_owned() -> alloc::string::String
   &[i32]   .to_owned() -> alloc::vec::Vec<i32>
   &Path    .to_owned() -> std::path::PathBuf
   &String  .to_owned() -> alloc::string::String
   One method name, four answers: `type Owned` is chosen by the source.

2. So it is NOT a stringifying operation
   42.to_owned()  -> 42   (i32)
   42.to_string() -> "42" (alloc::string::String)
   `to_string` is about TEXT. `to_owned` is about OWNERSHIP.
   They coincide on a &str and nowhere else.

3. The trap: on an Rc it clones the POINTER
   type          alloc::rc::Rc<alloc::string::String>
   strong_count  2
   same buffer?  true
   the real copy is (*shared).clone() -> "ballot"

4. Why `Owned: Borrow<Self>` is in the trait
   shout(&a) = "ADA"   <- String lends a &str back
   the bound is the round trip, and it is what makes Cow possible.
```
<!-- /output -->

</details>

---

## See also

- [Making a `String`](../../14_Strings/making_a_string/README.md) — the five spellings that produce a `String`, and which to prefer; this page is the trait *behind* one of them
- [Concatenating strings](../../14_Strings/concatenating_strings/README.md) — where `s1.to_owned() + s2` comes from: `+` needs an owned left operand
- [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) — the owned/borrowed pair this trait converts between
- [Six kinds of string](../../14_Strings/six_kinds_of_string/README.md) — `OsString`/`Path`/`Cow` and the rest of the owned-borrowed pairs
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — the trait this one generalizes
- [What a trait is](../what_a_trait_is/README.md) — associated types, which are what make `Owned` possible

## Sources

The long-running style thread, worth reading for how much its *reasoning* changed while its conclusion did not: [`to_string()` vs `to_owned()` for string literals ↗](https://users.rust-lang.org/t/to-string-vs-to-owned-for-string-literals/1441) (2015–2021). On implementing the trait yourself: [Stack Overflow — implement `ToOwned` for user-defined types ↗](https://stackoverflow.com/questions/72105604/implement-toowned-for-user-defined-types), whose answer is the source of the referent-not-reference framing above.

The two threads this page settles, both worth reading for how much disagreement a "simple" question produced: [Stack Overflow — the difference between `clone` and `to_owned` ↗](https://stackoverflow.com/questions/22264502/in-rust-what-is-the-difference-between-clone-and-to-owned) (the accepted answer is right; the sharpest explanation is BallpointBen's comment underneath it, on deref coercion and `!Sized`) and [r/rust on the same question ↗](https://www.reddit.com/r/rust/comments/l5uih4/what_is_the_difference_between_clone_and_to_owned/) — where the top answer is correct, the "it doesn't matter" answer is nearly correct, and one upvoted reply claiming `ToOwned` is how you get the data out of an `Rc` is **wrong** in the exact way the trap section above demonstrates.
