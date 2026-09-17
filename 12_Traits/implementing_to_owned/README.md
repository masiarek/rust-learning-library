# Implementing `ToOwned` for your own type

**Level:** 201 → 301 · working knowledge

**One line:** `ToOwned` goes on the type a reference points *at* — `str`, never `&str` — so a type of your own needs an unsized `#[repr(transparent)]` wrapper around `str` or `[T]`, one pointer cast to lend it out of its owned twin, and a check in front of the only public way in.

```rust
use std::borrow::Borrow;

#[repr(transparent)]
pub struct AsciiStr {
    text: str,                  // unsized, so AsciiStr is too: str's role
}

pub struct AsciiString {
    text: String,               // String's role
}

impl Borrow<AsciiStr> for AsciiString {
    fn borrow(&self) -> &AsciiStr {
        // SAFETY: AsciiStr is #[repr(transparent)] over str.
        unsafe { &*(self.text.as_str() as *const str as *const AsciiStr) }
    }
}

impl ToOwned for AsciiStr {
    type Owned = AsciiString;
    fn to_owned(&self) -> AsciiString {
        AsciiString { text: self.text.to_owned() }
    }
}
```

That is `str` and `String` again, and std builds `Path` and `PathBuf` the same way: `Path` is a `#[repr(transparent)]` struct around `OsStr`, and the body of `Path::new` is this cast:

```rust
// std::path::Path::new, 1.98.0
unsafe { &*(s.as_ref() as *const OsStr as *const Path) }
```

## The question this answers

A [Stack Overflow question ↗](https://stackoverflow.com/questions/72105604/implement-toowned-for-user-defined-types) wants `ToOwned` for this pair:

```rust
#[derive(Clone)]
struct DataRef<'a> {
    text: &'a str,
}

#[derive(Clone)]
struct DataOwned {
    text: String,
}
```

It meets two refusals, both reproduced on the [`ToOwned` page](../to_owned/README.md#can-you-implement-it-yourself). `E0119`, because `DataRef` is `Clone` and the blanket `impl<T: Clone> ToOwned for T` already covers it. And once the `Clone` is gone, a `Borrow<DataRef<'a>> for DataOwned` whose `borrow` has to return `&DataRef` — a reference to a `DataRef` that no `DataOwned` stores.

## Put the trait on the referent

Every `ToOwned` impl in std is on an unsized type: `str`, `[T]`, `CStr`, `OsStr`, `Path`. Line the pairs up as pointer types and the question's mistake is a misplaced column:

| | the borrowed pointer | what it points at — `ToOwned` goes here | the owned pointer |
|---|---|---|---|
| std | `&str` | `str` | `String` |
| std | `&Path` | [`Path` ↗](https://doc.rust-lang.org/std/path/struct.Path.html) | `PathBuf` |
| the question | `DataRef<'a>` | *no such type* | `DataOwned` |
| this page | `&AsciiStr` | `AsciiStr` | `AsciiString` |

`DataRef` is the borrowed *pointer*, so implementing the trait on it puts `Self` one level too high. [`Borrow::borrow` ↗](https://doc.rust-lang.org/std/borrow/trait.Borrow.html) returns `&Borrowed`, an address inside `self`, and nothing inside a `DataOwned` is a `DataRef` to take the address of. `AsciiStr` is the referent, and an `AsciiString`'s own bytes are one — section 2 of the run below prints that the lent `&AsciiStr` points into the `AsciiString`'s buffer.

## Why the cast is sound

The Reference gives a `#[repr(transparent)]` struct *"the same layout and ABI as the only non-size 0 non-alignment 1 field"* ([type layout ↗](https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation)). So a `*const str` and a `*const AsciiStr` agree on the address and on the length that travels with it — section 1 checks that `&AsciiStr` is the same size as `&str`, two words, like every pointer to an [unsized](../../14_Strings/str_is_unsized/README.md) value.

That attribute is the whole soundness argument. The ASCII check is not part of it: an `AsciiStr` holding `"café"` would still be valid UTF-8 inside a valid `str` — a wrong value, not undefined behaviour. The kata below measures the same thing on a sorted slice, where skipping the check makes `contains` report a present element missing. That is the split [What an invariant is](../../09_Advanced/what_an_invariant_is/README.md) draws between a safety invariant and a logical one, and it flips the moment `unsafe` code of yours *relies* on the property: a wrapper promising UTF-8 over `[u8]` that hands its bytes to `str::from_utf8_unchecked` turns the same skipped check into undefined behaviour.

## The check is the only way in

The comment thread under the answer asks the next question — how do you *make* one? — and the answer's author replies: the same cast as `borrow`, with the validation first:

```rust
impl<'a> TryFrom<&'a str> for &'a AsciiStr {
    type Error = NotAscii;

    fn try_from(s: &'a str) -> Result<Self, NotAscii> {
        match s.bytes().position(|b| !b.is_ascii()) {
            Some(at) => Err(NotAscii { at }),
            None => Ok(AsciiStr::from_str_unchecked(s)),   // the cast, private
        }
    }
}
```

Two things make that the only way in, and it takes both:

- **The fields are private and the pair lives in a `mod`.** Privacy in Rust is per module, not per type: with everything in a file's root module, `main` could write `AsciiString { text: String::from("café") }` and never meet the check. The example puts the pair in `mod ascii`.
- **The cast is a private function.** `from_str_unchecked` has two callers — `try_from` after the check, and `borrow` on bytes that were checked when the `AsciiString` was built.

`TryFrom` is std's and so is `&`, yet the impl is legal: `&` is one of the *fundamental* type constructors, so the orphan rule counts `&AsciiStr` as a local type. The everyday half of that rule is on [Extension traits](../extension_traits/README.md).

A method that cannot break the invariant skips the check. Uppercasing ASCII only produces ASCII, so the example's `to_uppercase` builds its `AsciiString` directly.

## What the pair buys

**`Borrow` buys lookups by the borrowed form.** [`HashMap::get` ↗](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get) takes a `&Q` wherever `K: Borrow<Q>`, so a `HashMap<AsciiString, u32>` accepts a `&AsciiStr` key (section 3). The price is a promise the compiler never checks — `Borrow`'s documentation requires `Eq`, `Ord` and `Hash` to be equivalent for the borrowed and the owned value. Deriving them on both halves keeps it here, since each hashes and compares exactly its text; had the two hashes disagreed, section 3 would have printed `None`.

**`ToOwned` buys `Cow`.** A [`Cow` ↗](https://doc.rust-lang.org/std/borrow/enum.Cow.html) needs `B: ToOwned`, so `Cow<'_, AsciiStr>` now exists. `shout` returns its input untouched when it is already uppercase and builds a new `AsciiString` only when it is not (section 4), and either arm derefs to `&AsciiStr`. The rest of that type is on [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md).

## Where it stops: one buffer

The cast lends out the bytes of **one** owned buffer. A `Name { first: String, last: String }` has two, and a `&` to a slice-like type is one address and one length, so no `&NameRef` can cover both. The answer notes that one string plus some fixed-size fields is possible in principle and hard in practice, because custom dynamically sized types are poorly supported.

## Usually, skip the trait

For the question's own `DataRef`, the answer's last paragraph is the practical one — if all you want is a `.to_owned()` that returns a `DataOwned`, write an inherent method:

```rust
impl DataRef<'_> {
    fn to_owned(&self) -> DataOwned {
        DataOwned { text: self.text.to_owned() }
    }
}
```

It wins at the call site, because [method lookup](../method_resolution/README.md) tries inherent methods before trait methods for each receiver type — and only there. `ToOwned::to_owned(&r)` still reaches the blanket impl and returns a `DataRef` (section 5), so generic code bounded on `T: ToOwned` gets the clone and never your method. A different name, such as `to_data_owned`, says the same thing without one spelling meaning two calls.

The other shape you will meet — `impl ToOwned for MyStruct { type Owned = MyStruct; … }` on a type that is not `Clone` — compiles, and buys nothing: it is `Clone` under a different name, and the day someone adds `#[derive(Clone)]` it becomes the `E0119` above. The [`ToOwned` page](../to_owned/README.md#can-you-implement-it-yourself) runs one.

## What std would need: a generic associated type

The question's third part asks what `ToOwned` could look like if std were free to change it. The obstacle is one return type: `borrow` returns `&Borrowed`. Make the borrowed form a generic associated type and the owned side can return it **by value** — and a struct of references is a value:

```rust
trait Lend {
    type Target<'a>
    where
        Self: 'a;

    fn lend(&self) -> Self::Target<'_>;
}

impl Lend for Name {
    type Target<'a> = NameRef<'a>;

    fn lend(&self) -> NameRef<'_> {
        NameRef { first: &self.first, last: &self.last }
    }
}
```

Two fields, no cast, no `unsafe` (section 6). What it gives up: a value is not a reference, so it cannot back [`Deref` ↗](https://doc.rust-lang.org/std/ops/trait.Deref.html), whose `deref` must return `&Self::Target`, and a `Cow` built on it could not deref either — you would call `.lend()` by hand. Nor can std swap it into `Borrow`, whose signature every existing impl already matches.

[`borrowme` ↗](https://docs.rs/borrowme/latest/borrowme/) is this idea shipped as a crate, by udoprog: first published in May 2023, 0.1.0 in April 2026. Its `Borrow` is the trait above — `type Target<'a> where Self: 'a;` and `fn borrow(&self) -> Self::Target<'_>` — its `ToOwned` drops std's `Owned: Borrow<Self>` bound, and a `#[borrowme]` attribute on a struct holding references generates the owned twin and both impls. Those are claims from its documentation; the example here builds the trait itself rather than depending on the crate. The `ToOwned` page links a 2025 `IntoOwned` pre-RFC, a separate attempt at the same gap.

## If you are coming from another language

- **C++** — `std::string_view` is the question's `DataRef`: a small value holding a pointer and a length, beside the owning `std::string`. C++ has no unsized types, so a view class is the only shape a borrowed form can take, and converting is an explicit constructor, `std::string(view)`. Rust's `&str` is the same view, but the type behind it, `str`, exists — which is what lets one trait describe "the thing behind the pointer" and lets `Cow` and `HashMap::get` work over it generically. A multi-field view has no referent in either language, and the idiomatic answer in both is a view struct plus a conversion function.
- **Python** — there is no borrowed/owned split to model: slicing a `str` makes a new object, and every name is already a reference the runtime counts. `memoryview` over `bytes` is the one view type, and nothing checks what you promise about its contents. In Rust the private field and the `TryFrom` door are how a promise about contents becomes a type.

## The verified output

<!-- output:implementing_to_owned -->
*Verified output of [`implementing_to_owned.rs`](examples/implementing_to_owned.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The only public way to an &AsciiStr is through the check
   "Ada"  -> Ok  AsciiStr { text: "Ada" }
   "café" -> Err, first non-ASCII byte at index 3
   size_of::<&AsciiStr>() == size_of::<&str>(): true   (address + length)

2. to_owned buys the owned twin; borrow lends it back without copying
   ada.to_owned()   -> AsciiString { text: "Ada" }
   owned.borrow()   -> AsciiStr { text: "Ada" }, pointing into owned's own buffer: true

3. Borrow buys lookups by the borrowed form
   seats: HashMap<AsciiString, u32>, key: &AsciiStr
   seats.get(key) = Some(3)

4. ToOwned buys Cow
   shout("HELLO") -> Cow::Borrowed("HELLO")   nothing allocated
   shout("hello") -> Cow::Owned("HELLO")      one new AsciiString
   a Cow<AsciiStr> derefs to &AsciiStr either way: "ADA"

5. The question's DataRef: an inherent method instead of the trait
   r.to_owned()          -> DataOwned { text: "x" }   inherent, found first
   ToOwned::to_owned(&r) -> DataRef   { text: "x" }   the blanket impl, still there

6. A GAT lends a struct of references by value: two fields, no cast
   name.lend() -> NameRef { first: "Ada", last: "Lovelace" }
   both fields point into name's own buffers: true
```
<!-- /output -->

## Practice

**A slice that promises its order.** Build `Sorted<T>` over `[T]` and `SortedVec<T>` over `Vec<T>`, the way this page builds `AsciiStr` and `AsciiString`: a private cast, a `TryFrom<&[T]> for &Sorted<T>` whose error is the index where the order first breaks, `Borrow`, `ToOwned`, and a `contains` that binary-searches — which it may, because the type promises order. Then write `ensure_zero(s: &Sorted<i32>) -> Cow<'_, Sorted<i32>>`, borrowed when `0` is already there.

Two questions before you look. `Borrow` and `ToOwned` need different bounds on `T` — which one needs `Clone`, and why does the other need nothing? And if the module grew a public constructor that skipped the check, what is the damage: undefined behaviour, or a wrong answer?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:implementing_to_owned_kata -->
*[`implementing_to_owned_kata.rs`](examples/implementing_to_owned_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a slice type that promises its order. `Sorted<T>` is the
//! borrowed half over `[T]`, `SortedVec<T>` the owned half over `Vec<T>` —
//! the `str`/`String` pattern again, one level more generic.
//!
//!   rustc --edition 2024 implementing_to_owned_kata.rs -o /tmp/itok && /tmp/itok

use std::borrow::{Borrow, Cow};

use sorted::{Sorted, SortedVec};

mod sorted {
    use std::borrow::Borrow;

    #[repr(transparent)]
    pub struct Sorted<T> {
        items: [T],
    }

    pub struct SortedVec<T> {
        items: Vec<T>,
    }

    impl<T> Sorted<T> {
        fn from_slice_unchecked(items: &[T]) -> &Sorted<T> {
            // SAFETY: `Sorted<T>` is `#[repr(transparent)]` over `[T]`, so the
            // pointers share a layout and the element count travels with them.
            unsafe { &*(items as *const [T] as *const Sorted<T>) }
        }

        pub fn as_slice(&self) -> &[T] {
            &self.items
        }

        /// Allowed to binary-search, because the type promises order.
        pub fn contains(&self, x: &T) -> bool
        where
            T: Ord,
        {
            self.items.binary_search(x).is_ok()
        }

        /// A copy with `x` inserted where it belongs.
        pub fn with(&self, x: T) -> SortedVec<T>
        where
            T: Ord + Clone,
        {
            let mut items = self.items.to_vec();
            let at = items.partition_point(|y| *y < x);
            items.insert(at, x);
            SortedVec { items }
        }
    }

    impl<'a, T: Ord> TryFrom<&'a [T]> for &'a Sorted<T> {
        /// The index of the first element smaller than the one before it.
        type Error = usize;

        fn try_from(items: &'a [T]) -> Result<Self, usize> {
            match items.windows(2).position(|w| w[0] > w[1]) {
                Some(i) => Err(i + 1),
                None => Ok(Sorted::from_slice_unchecked(items)),
            }
        }
    }

    // No bound on T: lending the slice copies nothing.
    impl<T> Borrow<Sorted<T>> for SortedVec<T> {
        fn borrow(&self) -> &Sorted<T> {
            Sorted::from_slice_unchecked(&self.items)
        }
    }

    // T: Clone, because making the owned twin copies every element.
    impl<T: Clone> ToOwned for Sorted<T> {
        type Owned = SortedVec<T>;

        fn to_owned(&self) -> SortedVec<T> {
            SortedVec { items: self.items.to_vec() }
        }
    }

    /// The door the second question is about: public here only so `main` can
    /// show what walks through it.
    pub fn from_vec_unchecked<T>(items: Vec<T>) -> SortedVec<T> {
        SortedVec { items }
    }
}

/// Borrowed when 0 is already there; one new `SortedVec` when it is not.
fn ensure_zero(s: &Sorted<i32>) -> Cow<'_, Sorted<i32>> {
    if s.contains(&0) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.with(0))
    }
}

fn main() {
    println!("1. TryFrom is the checked door");
    for items in [&[-2, 0, 5, 9][..], &[1, 4, 3, 8][..]] {
        match <&Sorted<i32>>::try_from(items) {
            Ok(s) => println!("   {items:?} -> Ok, contains(&5) = {}", s.contains(&5)),
            Err(i) => println!("   {items:?} -> Err, order breaks at index {i}"),
        }
    }

    println!();
    println!("2. ToOwned and Borrow, round trip");
    let s: &Sorted<i32> = (&[1, 2, 3][..]).try_into().unwrap();
    let owned: SortedVec<i32> = s.to_owned();
    let back: &Sorted<i32> = owned.borrow();
    println!("   s.to_owned().borrow() = {:?}", back.as_slice());

    println!();
    println!("3. Cow over Sorted<i32>");
    for items in [&[-1, 0, 4][..], &[-1, 4][..]] {
        let s: &Sorted<i32> = items.try_into().unwrap();
        match ensure_zero(s) {
            Cow::Borrowed(b) => println!("   ensure_zero({items:?}) -> Borrowed {:?}", b.as_slice()),
            Cow::Owned(o) => {
                let lent: &Sorted<i32> = o.borrow();
                println!("   ensure_zero({items:?}) -> Owned    {:?}", lent.as_slice());
            }
        }
    }

    println!();
    println!("4. Skip the check and the damage is a wrong answer, not undefined behaviour");
    let liar = sorted::from_vec_unchecked(vec![3, 2, 1]);
    let lent: &Sorted<i32> = liar.borrow();
    println!("   {:?}.contains(&3) = {}   (3 is right there)", lent.as_slice(), lent.contains(&3));
}
```
<!-- /source -->

<!-- output:implementing_to_owned_kata -->
*Verified output of [`implementing_to_owned_kata.rs`](examples/implementing_to_owned_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. TryFrom is the checked door
   [-2, 0, 5, 9] -> Ok, contains(&5) = true
   [1, 4, 3, 8] -> Err, order breaks at index 2

2. ToOwned and Borrow, round trip
   s.to_owned().borrow() = [1, 2, 3]

3. Cow over Sorted<i32>
   ensure_zero([-1, 0, 4]) -> Borrowed [-1, 0, 4]
   ensure_zero([-1, 4]) -> Owned    [-1, 0, 4]

4. Skip the check and the damage is a wrong answer, not undefined behaviour
   [3, 2, 1].contains(&3) = false   (3 is right there)
```
<!-- /output -->

`ToOwned` needs `T: Clone` because the owned twin is a copy of every element; `Borrow` lends the slice that is already there and copies nothing, so it takes any `T`. And the skipped check costs a wrong answer: the cast is sound for any `[T]`, so a `Sorted<i32>` over `[3, 2, 1]` is a valid slice carrying a false promise, and `binary_search`, which trusts the promise, reports `3` missing.

</details>

## See also

- [`ToOwned`](../to_owned/README.md) — the trait, the blanket impl, and the two refusals this page starts from
- [`clone_into`](../clone_into/README.md) — the provided method, which an impl like this one inherits as a plain `*target = self.to_owned()`
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md) — the type `ToOwned` exists to serve
- [`Borrow`: look up an owned key with a borrowed one](../borrow_trait/README.md) — the promise `AsciiString: Borrow<AsciiStr>` has to keep, and a key that breaks it
- [`str` is unsized](../../14_Strings/str_is_unsized/README.md) — why `AsciiStr` has no size and `&AsciiStr` is two words
- [Method resolution](../method_resolution/README.md) — why the inherent `to_owned` wins at the call site
- [What an invariant is](../../09_Advanced/what_an_invariant_is/README.md) — the one door, and a wrong value against undefined behaviour
- [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) — what the block around the cast switches off, and what it leaves on
- [Step 10 of the `ToOwned` path](../how_to_learn_to_owned/implementing_it_or_not/README.md) — `E0119`, the view struct `Borrow` cannot serve, and the inherent method
- [Three `ToOwned` katas, run](../how_to_learn_to_owned/to_owned_katas_checked/README.md#kata-3-a-toowned-that-changes-the-value) — the same wrapper with the check moved into `to_owned`, and the `Cow` and `HashMap` that break when it is

## Sources

[Stack Overflow — implement `ToOwned` for user-defined types ↗](https://stackoverflow.com/questions/72105604/implement-toowned-for-user-defined-types) (2022): the question; Kevin Reid's answer, which is the source of the referent-not-reference framing, the `repr(transparent)` wrapper, the one-buffer limit and the inherent-method advice; and the comment thread under it, which asks how to construct one. [The Reference on the transparent representation ↗](https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation). [`borrowme` ↗](https://docs.rs/borrowme/latest/borrowme/) for the generic-associated-type version.

## Po polsku

`ToOwned` implementuje się dla typu, **na który wskazuje** referencja — dla `str`, a nie dla `&str`. Stąd bierze się kłopot z pytania: `DataRef<'a> { text: &'a str }` sam jest widokiem, czyli „wskaźnikiem”, więc cecha (*trait*) trafia o poziom za wysoko, a `Borrow::borrow` musiałoby zwrócić `&DataRef` — adres czegoś, czego żaden `DataOwned` nie przechowuje. Wszystkie implementacje w bibliotece standardowej siedzą na typach bez znanego rozmiaru: `str`, `[T]`, `Path`, `OsStr`, `CStr`.

Własny typ w tej roli to struktura z jednym polem `str`, oznaczona `#[repr(transparent)]`. Atrybut gwarantuje ten sam układ w pamięci co pole, więc rzutowanie `*const str as *const AsciiStr` zachowuje adres i długość — tak samo zbudowany jest `Path` nad `OsStr`. Poprawność (*soundness*) tego `unsafe` zależy **wyłącznie** od atrybutu, a nie od sprawdzenia ASCII: `AsciiStr` z „café” w środku to zła wartość, a nie niezdefiniowane zachowanie. Sprawdzenie jest niezmiennikiem logicznym, a pilnują go prywatne pole w osobnym module i jedyne publiczne wejście, `TryFrom<&str> for &AsciiStr`. Prywatność w Ruście działa na poziomie modułu, nie typu.

W zamian para daje dwie rzeczy: `Borrow` pozwala szukać w `HashMap<AsciiString, _>` kluczem `&AsciiStr`, a `ToOwned` otwiera `Cow<'_, AsciiStr>`. Granica jest twarda — sztuczka działa tylko dla **jednego ciągłego bufora**. Struktura z dwoma polami `String` nie ma czego pożyczyć jako `&`, i wtedy lepsza jest zwykła metoda własna (*inherent method*). Ta wygrywa przy wywołaniu kropką, ale kod generyczny z ograniczeniem `T: ToOwned` dalej zobaczy implementację zbiorczą, więc lepiej nadać jej inną nazwę.

Na trzecie pytanie — co musiałaby zmienić biblioteka standardowa — odpowiada generyczny typ powiązany (GAT): `type Target<'a>` pozwala zwrócić formę pożyczoną **przez wartość**, więc struktura referencji przestaje być problemem. Kosztem jest utrata `Deref`. Tak działa crate `borrowme`.

**Szukaj po polsku:** typy o nieznanym rozmiarze · niezmiennik typu · `rust repr(transparent) newtype str` · `rust implement Borrow for custom type` · `rust custom DST` · `rust borrowme`
