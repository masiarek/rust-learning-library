# `str::as_str`: the method that was stabilized and taken back

**Level:** 201 → 301 · working knowledge

**One line:** On a `&str` there is nothing for `as_str` to do — you are already holding the view — but the method exists for `Box<str>`, `Rc<str>` and `Cow<str>`, it is `E0658` on stable, and the reason it is *still* unstable after reaching stabilization is a method-resolution rule this library already teaches.

Sooner or later you write `s.as_str()` on something that is not a `String`, and the compiler says something stranger than "no such method":

```text
error[E0658]: use of unstable library feature `str_as_str`
 --> as_str_probe.rs:5:15
  |
5 |     let u = s.as_str();
  |               ^^^^^^
  |
  = note: see issue #130366 <https://github.com/rust-lang/rust/issues/130366> for more information

For more information about this error, try `rustc --explain E0658`.
```

The method is real. It is written, it is tested, it is in the standard library you are linking against — and it is fenced off behind `#![feature(str_as_str)]`, which [only nightly will open](../../05_Tooling/nightly/README.md). That is a different situation from a missing import or a typo, and it deserves a different reflex.

## What you already have

On a `&str`, all three spellings are one borrow of one buffer:

```rust
let s: &str = "Hello";
let whole = &s[..];               // same pointer, same length
let deref = &*s;                  // same again
assert!(std::ptr::eq(s, whole));  // true — not equal, identical
```

Nothing is copied and nothing is converted, which is the real answer to why the method is not there: `&str → &str` is the identity, and you performed it by writing the variable's name. The [string slices](../string_slices/README.md) page is the long version of that; [`String` vs `&str`](../string_vs_str/README.md) is where the owner-and-view split comes from.

So on the type in the error message, the fix is to delete the call.

## Where the method is actually wanted

The proposal was never really about `&str`. It was about every *other* owner of some text — the ones [`String::as_str`](../string_methods/string_as_str/README.md) cannot help with, because they are not `String`:

| you have | stable spelling |
|---|---|
| `String` | `s.as_str()` — stable since 1.7, and unaffected by any of this |
| `Box<str>` | `&*b`, or `&b[..]` |
| `Rc<str>` / `Arc<str>` | `&*r` |
| `Cow<'_, str>` | `&*c` |
| `&str` | nothing — you have one |

`Rc<str>` and `Arc<str>` are the cases that bring people here: they are the right type for a string shared many ways and never edited, and until you need to hand one to a function taking `&str`, nothing tells you that the tidy spelling is missing. The feature's [tracking issue ↗](https://github.com/rust-lang/rust/issues/130366) opens with exactly that motivation — one spelling that reads the same on every owner of some text.

It is a papercut rather than a wall, though, and the [kata](#practice) below is about why: at a *call site* the coercion already does this for you, so `shout(&boxed)` and `shout(&counted)` compile with no help at all. What has no coercion to lean on is a closure, an iterator chain, or a `match` against a literal — the same three places [`String::as_str`](../string_methods/string_as_str/README.md) earns its keep.

## The workaround the feature's own author argues against

The advice you will find first is to write `.as_ref()` instead. It usually compiles, and the objection to it — raised by the person who proposed the feature — is worth more than the shortcut:

```rust
let arr: [u8; 3] = [1, 2, 3];
let r = arr.as_ref();   // &[u8] — NOT &[u8; 3]
```

[`AsRef` is not reflexive ↗](https://doc.rust-lang.org/std/convert/trait.AsRef.html). There is no blanket `impl<T: ?Sized> AsRef<T> for T`, because it would overlap with the auto-dereferencing impl that makes the trait convenient in the first place. So `.as_ref()` on an array finds nothing at its own rung, [walks down the deref chain](../../29_Conversion/coercion/README.md), and answers as a slice.

Which means `.as_ref()` does not mean *the same thing, viewed*. It means *whatever the deref chain offers first* — a different question, whose answer can change when the chain does. When you mean the identity, `&*x` says so and cannot drift.

## Stabilized, then reverted

This is the part worth the page. `str_as_str` completed its final comment period, was stabilized in [#151603 ↗](https://github.com/rust-lang/rust/pull/151603), and was then **reverted** in [#152963 ↗](https://github.com/rust-lang/rust/pull/152963) after the release broke a real crate: [`rgb` ↗](https://github.com/rust-lang/rust/issues/152961), via the `[T]::as_mut_slice` that had been added alongside it.

Nothing was removed, nothing conflicted, and no error was raised at the definition. Here is the whole mechanism, on a type small enough to read:

```rust
struct Pixel([u8; 3]);

trait ComponentSlice {                                  // what the crate provides
    fn as_mut_slice(&mut self) -> &mut [u8];
}
impl ComponentSlice for Pixel {
    fn as_mut_slice(&mut self) -> &mut [u8] { &mut self.0 }
}

impl Pixel {                                            // what the release added
    fn as_mut_slice(&mut self) -> &mut [u8; 3] { &mut self.0 }
}
```

`p.as_mut_slice()` compiled before and compiles after — and now returns `&mut [u8; 3]` instead of `&mut [u8]`, because **an inherent method beats a trait method** at the same rung. That rule is the subject of [Method resolution](../../12_Traits/method_resolution/README.md), and [A trait must be in scope](../../12_Traits/trait_in_scope/README.md) is the other half of it.

Two things follow that are easy to state and easy to forget:

**Adding an inherent method to a widely-extended type is a source-breaking change.** Not because it removes anything, but because every [extension trait](../../12_Traits/extension_traits/README.md) that guessed the same name silently loses the dot. The larger the type's ecosystem — and `str`, `[T]` and `Path` are as large as it gets — the more certain the collision.

**And the breakage lands on people who wrote neither side.** `rgb`'s author did not change anything, the std authors did not touch `rgb`, and the failing line belongs to a third party who called a method that has quietly changed type. That asymmetry is why this class of change goes through a crater run and why a stabilization can be reverted weeks after it shipped.

If it feels familiar, it is the same shape as arrays gaining a by-value `IntoIterator` — resolved there by making the new meaning [edition-dependent](../../24_Iterators/iter_iter_mut_into_iter/README.md) rather than by reverting, which is an option only when the migration is worth an edition's attention.

## The verified output

<!-- output:str_as_str -->
*Verified output of [`str_as_str.rs`](examples/str_as_str.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. On a &str you already have a &str
   s          "Hello"
   &s[..]     "Hello"     same pointer: true
   &*s        "Hello"     same pointer: true
   All three are one borrow of one buffer -- not copies that
   happen to be equal. There is no conversion left to perform,
   which is why the method you reached for is not there.

2. The error, on stable
   let u = s.as_str();
     error[E0658]: use of unstable library feature `str_as_str`
       |     let u = s.as_str();
       |               ^^^^^^
       = note: see issue #130366 for more information
   Note what it is NOT: not `no method named`, not a missing
   trait import. The method exists, is written, and is fenced
   off behind #![feature(str_as_str)] -- a nightly-only door.

3. Where the method would have earned its place
   Box<str>   &*b          -> &str
   Rc<str>    &*r          -> &str
   Arc<str>   &*a          -> &str
   Cow<str>   &*c          -> &str
   None of these is a String, so String::as_str is unavailable,
   and `&*` is doing real work rather than nothing. That is the
   gap the feature was proposed to fill: one spelling, `as_str`,
   that reads the same on every owner of some text.

4. Why the obvious workaround is the disputed one
   [u8; 3].as_ref()        -> &[u8]
   Not &[u8; 3]. AsRef is NOT reflexive: there is no blanket
   `impl<T> AsRef<T> for T`, so `.as_ref()` on an array finds
   nothing at that rung, derefs to [u8], and answers as a slice.
   So `.as_ref()` means 'whatever the deref chain offers first',
   which is a different question from 'the same thing, as a view'.
   Reach for `&*x` when you mean the second one.

5. And why the method was taken back
   p.as_mut_slice()                     -> &mut [u8; 3]
   ComponentSlice::as_mut_slice(&mut p) -> &mut [u8]
   One call site, two answers. An inherent method beats a trait
   method at the same rung, so adding one to a type thousands of
   crates already extend is a silent, source-breaking change --
   the calls still compile, and hand back a different type.
   That is exactly what happened when the feature grew from str
   to [T]: the new inherent [T]::as_mut_slice shadowed the rgb
   crate's trait method, and the stabilization was reverted.

6. So, today
   &str            nothing -- you already have one
   Box/Rc/Arc<str> &*x, or &x[..]
   Cow<str>        &*x
   String          x.as_str()   (stable since 1.7, and unaffected)
```
<!-- /output -->

## If you are coming from another language

- **Python.** `str(s)` on a `str` gives you `s` back — `str(s) is s` is `True` — so the reflexive conversion Rust declines to provide is one Python simply has, and `as_str` reads to a Python programmer like a method that obviously ought to exist. The second half of the page ports better than the first: adding a method to a base class silently changes what every subclass resolves through the MRO, and nobody finds out at definition time. Rust's version is stricter in one way and looser in another — the collision is caught at compile time in the *caller's* crate, not at run time, but it is still the caller who has to deal with it.
- **ABAP.** Adding a method to an interface breaks every implementing class immediately and loudly: the class no longer compiles until it implements the new method, and you find out in the transport. Rust's inherent-method version of the same change is the opposite temperament — every call site still compiles, and hands back a different type. The lesson to carry over is that "I only *added* something" is not a safety argument in either language; it is a claim about a name space you share with people you have never met.

## Practice

**The conversion you did not need, and the method that changed under you.** Write `fn shout(s: &str) -> String`, then call it with a `String`, a `Box<str>`, an `Rc<str>`, an `Arc<str>` and a `Cow<str>`. Count how many conversions you actually had to write, and then find the one place — a closure inside `.map()` — where the answer changes and you must say `&**r` out loud.

For the second half, give a struct a trait method named `as_mut_slice`, add an inherent method of the same name returning `&mut [u8; 3]`, and call a function that requires `&mut [u8]`. Read the `E0308`, then repair the call **without** deleting the inherent method — twice, two different ways — and say which of the two you would want a stranger's crate to rely on.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:str_as_str_kata -->
*[`str_as_str_kata.rs`](examples/str_as_str_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the conversion you did not need, and the method that changed
//! under you.
//!
//!   rustc --edition 2024 str_as_str_kata.rs -o /tmp/sask && /tmp/sask

use std::any::type_name_of_val as type_of;
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

fn shout(s: &str) -> String {
    s.to_uppercase()
}

/// Part two: a trait that hands out a view, on a type somebody else owns.
struct Reading([u8; 3]);

trait Samples {
    fn as_mut_slice(&mut self) -> &mut [u8];
}

impl Samples for Reading {
    fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// ...and the inherent method that arrives in a later release.
impl Reading {
    fn as_mut_slice(&mut self) -> &mut [u8; 3] {
        &mut self.0
    }
}

/// Only a `&mut [u8]` will do here -- an array is a different type.
fn zero_out(samples: &mut [u8]) {
    for s in samples.iter_mut() {
        *s = 0;
    }
}

fn main() {
    println!("Part 1: five owners, and how many conversions you actually write");
    let owned = String::from("hello");
    let boxed: Box<str> = "hello".into();
    let counted: Rc<str> = "hello".into();
    let shared: Arc<str> = "hello".into();
    let maybe: Cow<'_, str> = Cow::Borrowed("hello");

    println!("   shout(&owned)    {:?}", shout(&owned));
    println!("   shout(&boxed)    {:?}", shout(&boxed));
    println!("   shout(&counted)  {:?}", shout(&counted));
    println!("   shout(&shared)   {:?}", shout(&shared));
    println!("   shout(&maybe)    {:?}", shout(&maybe));
    println!("   Answer: none of them. Every one of these types derefs to str,");
    println!("   so a plain `&x` at a call site expecting &str coerces, and the");
    println!("   whole as_str question never arises. That is why the missing");
    println!("   method is a papercut rather than a wall.");

    println!();
    println!("Part 1b: where the coercion has nothing to aim at");
    let all: Vec<Rc<str>> = vec!["ada".into(), "bob".into()];
    let views: Vec<&str> = all.iter().map(|r| &**r).collect();
    println!("   .map(|r| &**r)   {views:?}");
    println!("   A closure's return type is inferred, not demanded, so there is");
    println!("   no &str for the compiler to coerce toward and you must say it.");
    println!("   This is the case String::as_str exists for, and the case the");
    println!("   str_as_str feature wanted to cover for every other owner.");

    println!();
    println!("Part 2: the method that changed under you");
    let mut r = Reading([7, 8, 9]);
    println!("   r.as_mut_slice()               -> {}", type_of(&r.as_mut_slice()));
    println!("   Samples::as_mut_slice(&mut r)  -> {}", type_of(&Samples::as_mut_slice(&mut r)));
    println!("   The inherent method wins, so the trait's is unreachable");
    println!("   through a dot -- and zero_out(r.as_mut_slice()) now fails:");
    println!("     error[E0308]: mismatched types");
    println!("       expected `&mut [u8]`, found `&mut [u8; 3]`");

    println!();
    println!("   Two repairs, and only one of them is yours to make:");
    zero_out(Samples::as_mut_slice(&mut r));
    println!("   Samples::as_mut_slice(&mut r)  {:?}   name the trait", r.0);
    let mut r2 = Reading([7, 8, 9]);
    zero_out(&mut r2.as_mut_slice()[..]);
    println!("   &mut ..[..]                    {:?}   reslice the array", r2.0);
    println!("   The second works because [u8; 3] derefs to [u8]. Neither is a");
    println!("   fix for the crate that broke: its users' call sites are the");
    println!("   ones that changed meaning, and they did not ask for either.");
}
```
<!-- /source -->

<!-- output:str_as_str_kata -->
*Verified output of [`str_as_str_kata.rs`](examples/str_as_str_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Part 1: five owners, and how many conversions you actually write
   shout(&owned)    "HELLO"
   shout(&boxed)    "HELLO"
   shout(&counted)  "HELLO"
   shout(&shared)   "HELLO"
   shout(&maybe)    "HELLO"
   Answer: none of them. Every one of these types derefs to str,
   so a plain `&x` at a call site expecting &str coerces, and the
   whole as_str question never arises. That is why the missing
   method is a papercut rather than a wall.

Part 1b: where the coercion has nothing to aim at
   .map(|r| &**r)   ["ada", "bob"]
   A closure's return type is inferred, not demanded, so there is
   no &str for the compiler to coerce toward and you must say it.
   This is the case String::as_str exists for, and the case the
   str_as_str feature wanted to cover for every other owner.

Part 2: the method that changed under you
   r.as_mut_slice()               -> &mut [u8; 3]
   Samples::as_mut_slice(&mut r)  -> &mut [u8]
   The inherent method wins, so the trait's is unreachable
   through a dot -- and zero_out(r.as_mut_slice()) now fails:
     error[E0308]: mismatched types
       expected `&mut [u8]`, found `&mut [u8; 3]`

   Two repairs, and only one of them is yours to make:
   Samples::as_mut_slice(&mut r)  [0, 0, 0]   name the trait
   &mut ..[..]                    [0, 0, 0]   reslice the array
   The second works because [u8; 3] derefs to [u8]. Neither is a
   fix for the crate that broke: its users' call sites are the
   ones that changed meaning, and they did not ask for either.
```
<!-- /output -->

</details>

## See also

- [`String::as_str`](../string_methods/string_as_str/README.md) — the stable one, and the three places it earns its keep
- [String slices](../string_slices/README.md) — what `&s[..]` is, and the one way it panics
- [Method resolution](../../12_Traits/method_resolution/README.md) — the ladder that makes inherent beat trait
- [Extension traits](../../12_Traits/extension_traits/README.md) — the pattern on the losing end of that rule
- [Coercion](../../29_Conversion/coercion/README.md) — why `shout(&boxed)` needed nothing
- [`rustup default nightly`](../../05_Tooling/nightly/README.md) — the door `#![feature(...)]` is behind, and why to open it per-project
- [`str` methods](../str_methods/README.md) — the 83 that are stable, one page each

## Sources

- [Tracking issue for `str_as_str` #130366 ↗](https://github.com/rust-lang/rust/issues/130366) — the public API, the expansion to `CStr`, `OsStr`, `Path` and `[T]`, and the full history
- [Stabilize `str_as_str` #151603 ↗](https://github.com/rust-lang/rust/pull/151603) and [the revert #152963 ↗](https://github.com/rust-lang/rust/pull/152963)
- [Regression in `rgb` with `as_mut_slice` #152961 ↗](https://github.com/rust-lang/rust/issues/152961) — the crate that broke
- [`AsRef` in the standard library ↗](https://doc.rust-lang.org/std/convert/trait.AsRef.html) — the note explaining why it is not reflexive

## Po polsku

Kiedy piszesz `s.as_str()` na czymś, co nie jest `String`iem, kompilator mówi coś dziwniejszego niż „nie ma takiej metody”: `error[E0658]: use of unstable library feature str_as_str`. Metoda **istnieje** — jest napisana i siedzi w bibliotece standardowej, do której właśnie linkujesz — tylko jest odgrodzona bramką `#![feature(str_as_str)]`, którą otwiera wyłącznie nightly. To zupełnie inna sytuacja niż brakujący import i zasługuje na inny odruch.

Na samym `&str` odpowiedź jest najprostsza z możliwych: **nic nie trzeba pisać**, bo już masz to, o co prosisz. `s`, `&s[..]` i `&*s` to jedno i to samo pożyczenie tego samego bufora — nie trzy równe kopie, tylko ten sam wskaźnik, co sprawdza `std::ptr::eq`. Propozycja nigdy zresztą nie dotyczyła `&str`, tylko **pozostałych właścicieli tekstu**, którym `String::as_str` nie pomoże, bo nie są `String`iem: `Box<str>`, `Rc<str>`, `Arc<str>` i `Cow<'_, str>`. Tam pisze się `&*x` — i to jest cała stabilna odpowiedź.

Podpowiedź, którą znajdziesz najszybciej — „użyj `.as_ref()`” — jest tą, przeciwko której protestuje **sam autor propozycji**, i warto wiedzieć dlaczego. `AsRef` **nie jest zwrotne**: nie ma uniwersalnej implementacji `impl<T> AsRef<T> for T`, bo nachodziłaby na tę, która daje temu typajowi automatyczne schodzenie po `Deref`. Dlatego `[u8; 3].as_ref()` oddaje `&[u8]`, a nie `&[u8; 3]` — metoda nie znajduje niczego na swoim szczeblu, schodzi niżej i odpowiada wycinkiem. Innymi słowy `.as_ref()` znaczy „cokolwiek łańcuch `Deref` zaproponuje najpierw”, a to jest inne pytanie niż „to samo, tylko jako widok”. Gdy chodzi ci o to drugie, `&*x` mówi to wprost i nie zmieni zdania.

Najciekawsze jest jednak to, **dlaczego funkcja wciąż jest niestabilna, choć już raz została ustabilizowana**. Przeszła okres ostatnich uwag, weszła jako stabilna w PR #151603 — i została **wycofana** w #152963, bo popsuła prawdziwy crate (`rgb`), i to nie przez `str`, tylko przez dodane przy okazji `[T]::as_mut_slice`. Mechanizm mieści się w trzech linijkach: **metoda własna (inherent) wygrywa z metodą z typaju** na tym samym szczeblu, więc dopisanie do typu metody o nazwie, której ktoś już używał w swoim typaju rozszerzającym, po cichu zabiera kropce dotychczasowe znaczenie. Wywołanie kompilowało się przedtem i kompiluje się nadal — tyle że zwraca `&mut [u8; 3]` zamiast `&mut [u8]`. Stąd dwa wnioski: **dodanie metody własnej do szeroko rozszerzanego typu jest zmianą łamiącą kod**, mimo że niczego nie usuwa; i **skutki spadają na osoby, które nie napisały żadnej ze stron** — ani autor `rgb`, ani autorzy biblioteki standardowej niczego u siebie nie zepsuli, a nie kompiluje się linijka trzeciej osoby. To ten sam kształt, co tablice i `IntoIterator` w edycji 2021, tylko tam rozwiązano go przez powiązanie nowego znaczenia z edycją, a nie przez wycofanie.

**Szukaj po polsku:** cecha niestabilna · bramka funkcjonalności · rozstrzyganie metod · metoda własna kontra metoda typaju · `rust E0658 unstable library feature` · `rust str_as_str` · `rust AsRef nie jest zwrotne`
