# `ToOwned` in real code

[How to learn `ToOwned`](../README.md) › **Beside the steps** · read after: [Step 10](../implementing_it_or_not/README.md)

**Level:** 201 → 301 · reading other people's code

**One line:** Fourteen places where `ToOwned`, `Borrow` and `Cow` do real work — eight in the standard library, six in widely used crates — each a short excerpt behind a permalink, with what it shows that the path only told you.

Every link is pinned to a release tag, so its line numbers stay right. The standard-library excerpts are from `rust-lang/rust` at **1.98.0**, the compiler this library pins; the crates are at the version current on crates.io on 2026-09-16. All of them are licensed MIT OR Apache-2.0. Excerpts are copied as they are; where lines were skipped, the fence says so with a `// …` line. Claims a program can check without a dependency are run at the bottom of the page.

## The standard library

### 1. The blanket impl hands `clone_into` to `clone_from`

[`library/alloc/src/borrow.rs#L72-L85` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/alloc/src/borrow.rs#L72-L85)

```rust
impl<T> ToOwned for T
where
    T: Clone,
{
    type Owned = T;
    fn to_owned(&self) -> T {
        self.clone()
    }

    fn clone_into(&self, target: &mut T) {
        target.clone_from(self);
    }
}
```

The impl from [step 6](../the_blanket_to_owned/README.md) is four lines of logic. The part the path only mentioned is the second method: every `Clone` type's `clone_into` is its `clone_from`, which is why `String` and `Vec` reuse their buffers without an `impl ToOwned` of their own — [step 9](../clone_into_refills/README.md).

### 2. `str` refills with `clear` and `push_str`

[`library/alloc/src/str.rs#L246-L259` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/alloc/src/str.rs#L246-L259)

```rust
impl ToOwned for str {
    type Owned = String;

    #[inline]
    fn to_owned(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.as_bytes().to_owned()) }
    }

    #[inline]
    fn clone_into(&self, target: &mut String) {
        target.clear();
        target.push_str(self);
    }
}
```

`to_owned` for `str` is `[u8]`'s `to_owned`, relabelled as UTF-8 without a check — the bytes came out of a `str`, so they already are. And `clone_into` is the whole of buffer reuse: `clear` keeps the capacity, `push_str` grows it only if it must.

### 3. `[T]` reuses the elements too

[`library/alloc/src/slice.rs#L813-L826` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/alloc/src/slice.rs#L813-L826) and [`#L838-L848` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/alloc/src/slice.rs#L838-L848)

```rust
impl<T: Clone, A: Allocator> SpecCloneIntoVec<T, A> for [T] {
    default fn clone_into(&self, target: &mut Vec<T, A>) {
        // drop anything in target that will not be overwritten
        target.truncate(self.len());

        // target.len <= self.len due to the truncate above, so the
        // slices here are always in-bounds.
        let (init, tail) = self.split_at(target.len());

        // reuse the contained values' allocations/resources.
        target.clone_from_slice(init);
        target.extend_from_slice(tail);
    }
}
// …
impl<T: Clone> ToOwned for [T] {
    type Owned = Vec<T>;

    fn to_owned(&self) -> Vec<T> {
        self.to_vec()
    }

    fn clone_into(&self, target: &mut Vec<T>) {
        SpecCloneIntoVec::clone_into(self, target);
    }
}
```

`clone_from_slice` calls each element's `clone_from`, so refilling a `Vec<String>` from a `&[String]` keeps every inner `String`'s buffer as well as the `Vec`'s. The run at the bottom checks it. The `default fn` is std using specialization internally, which it may and you may not.

### 4. `Path`: the pattern step 10 copies

[`library/std/src/path.rs#L2355-L2358` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/std/src/path.rs#L2355-L2358), [`#L2412-L2414` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/std/src/path.rs#L2412-L2414), [`#L2232-L2242` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/std/src/path.rs#L2232-L2242) and [`#L2109-L2114` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/std/src/path.rs#L2109-L2114)

```rust
#[repr(transparent)]
pub struct Path {
    inner: OsStr,
}
// …
    pub const fn new<S: [const] AsRef<OsStr> + ?Sized>(s: &S) -> &Path {
        unsafe { &*(s.as_ref() as *const OsStr as *const Path) }
    }
// …
impl ToOwned for Path {
    type Owned = PathBuf;
    #[inline]
    fn to_owned(&self) -> PathBuf {
        self.to_path_buf()
    }
    #[inline]
    fn clone_into(&self, target: &mut PathBuf) {
        self.inner.clone_into(&mut target.inner);
    }
}
// …
impl Borrow<Path> for PathBuf {
    #[inline]
    fn borrow(&self) -> &Path {
        self.deref()
    }
}
```

All four pieces of [step 10](../implementing_it_or_not/README.md#what-fits-one-buffer-wrapped-as-an-unsized-type) in one file: an unsized `#[repr(transparent)]` wrapper, the pointer cast it makes sound, `ToOwned` naming the owned twin, and `Borrow` lending it back. `clone_into` passes straight through to `OsStr`'s, so a `PathBuf` refills as cheaply as the bytes under it. (`[const]` is the unstable const-trait syntax; ignore it.)

### 5. `CStr` reuses a boxed slice

[`library/alloc/src/ffi/c_str.rs#L1072-L1092` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/alloc/src/ffi/c_str.rs#L1072-L1092)

```rust
impl ToOwned for CStr {
    type Owned = CString;

    fn to_owned(&self) -> CString {
        CString { inner: self.to_bytes_with_nul().into() }
    }

    fn clone_into(&self, target: &mut CString) {
        let src = self.to_bytes_with_nul();
        // If the lengths match, we can reuse the existing allocation without any overhead.
        if target.inner.len() == src.len() {
            target.inner.copy_from_slice(src);
        } else {
            // Reuse the existing allocation's capacity by converting to a Vec.
            // We temporarily replace `target` with a valid dummy to remain panic-safe.
            let mut b = mem::replace(&mut target.inner, Box::new([0])).into_vec();
            self.to_bytes_with_nul().clone_into(&mut b);
            target.inner = b.into_boxed_slice();
        }
    }
}
```

A `CString` holds a `Box<[u8]>`, which has no spare capacity to reuse — so this `clone_into` copies in place when the lengths match, and otherwise turns the box into a `Vec` to borrow `[u8]`'s refill. The `mem::replace` with a one-byte dummy is there so that a panic halfway cannot leave `target` holding a moved-out box.

### 6. `Cow::to_mut`: all of clone-on-write

[`library/alloc/src/borrow.rs#L283-L294` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/alloc/src/borrow.rs#L283-L294)

```rust
    pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
        match *self {
            Borrowed(borrowed) => {
                *self = Owned(borrowed.to_owned());
                match *self {
                    Borrowed(..) => unreachable!(),
                    Owned(ref mut owned) => owned,
                }
            }
            Owned(ref mut owned) => owned,
        }
    }
```

The return type is the answer to [claim 1 on the `Cow` page](../cow_claims_checked/README.md#eight-claims-run): `&mut <B as ToOwned>::Owned`, a `&mut String`. The inner `match` exists only to borrow the value just written; the `unreachable!()` can never run.

### 7. `+=` on an empty `Cow<str>` borrows instead of allocating

[`library/alloc/src/borrow.rs#L494-L507` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/alloc/src/borrow.rs#L494-L507)

```rust
impl<'a> AddAssign<&'a str> for Cow<'a, str> {
    fn add_assign(&mut self, rhs: &'a str) {
        if self.is_empty() {
            *self = Cow::Borrowed(rhs)
        } else if !rhs.is_empty() {
            if let Cow::Borrowed(lhs) = *self {
                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = Cow::Owned(s);
            }
            self.to_mut().push_str(rhs);
        }
    }
}
```

Nobody explains the first branch: an empty `Cow` that gets `+= "x"` simply *becomes* `Borrowed("x")` — even an `Owned` one with a 64-byte buffer, which is dropped. And when it does have to allocate, it sizes the `String` for both halves at once rather than going through [step 7's](../borrow_the_way_back/README.md#checkpoint) exact-fit `to_owned` and a growing `push`. The run below shows all three branches.

### 8. `String::from_utf8_lossy`: borrow unless something needs replacing

[`library/alloc/src/string.rs#L628-L654` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/alloc/src/string.rs#L628-L654)

```rust
    pub fn from_utf8_lossy(v: &[u8]) -> Cow<'_, str> {
        let mut iter = v.utf8_chunks();

        let Some(chunk) = iter.next() else {
            return Cow::Borrowed("");
        };
        let first_valid = chunk.valid();
        if chunk.invalid().is_empty() {
            debug_assert_eq!(first_valid.len(), v.len());
            return Cow::Borrowed(first_valid);
        }
        // … copies the valid chunks into a String, with U+FFFD for each invalid one …
        Cow::Owned(res)
    }
```

The shape every good `Cow`-returning function has: find out cheaply whether work is needed, return `Borrowed` if not, and allocate once, sized from the input, if so.

## Crates

### 9. `serde_json` 1.0.151: an owned type that is a `Box`

[`src/raw.rs#L116-L147` ↗](https://github.com/serde-rs/json/blob/v1.0.151/src/raw.rs#L116-L147)

```rust
#[repr(transparent)]
pub struct RawValue {
    json: str,
}

impl RawValue {
    const fn from_borrowed(json: &str) -> &Self {
        unsafe { mem::transmute::<&str, &RawValue>(json) }
    }
    // …
}

impl Clone for Box<RawValue> {
    fn clone(&self) -> Self {
        (**self).to_owned()
    }
}

impl ToOwned for RawValue {
    type Owned = Box<RawValue>;

    fn to_owned(&self) -> Self::Owned {
        RawValue::from_owned(self.json.to_owned().into_boxed_str())
    }
}
```

The owned twin does not have to be a named struct. `type Owned = Box<RawValue>` satisfies `Borrow<Self>` for free, because std has `impl<T: ?Sized> Borrow<T> for Box<T>`. And `Box<RawValue>`'s `Clone` is written *in terms of* `to_owned` — the blanket impl's direction, reversed.

### 10. `bstr` 1.13.1: one borrowed type, three owners that lend it

[`src/impls.rs#L191-L210` ↗](https://github.com/BurntSushi/bstr/blob/1.13.1/src/impls.rs#L191-L210) and [`#L233-L240` ↗](https://github.com/BurntSushi/bstr/blob/1.13.1/src/impls.rs#L233-L240)

```rust
    impl Borrow<BStr> for BString {
        #[inline]
        fn borrow(&self) -> &BStr {
            self.as_bstr()
        }
    }

    impl Borrow<BStr> for Vec<u8> {
        #[inline]
        fn borrow(&self) -> &BStr {
            self.as_slice().as_bstr()
        }
    }

    impl Borrow<BStr> for String {
        #[inline]
        fn borrow(&self) -> &BStr {
            self.as_bytes().as_bstr()
        }
    }
    // …
    impl ToOwned for BStr {
        type Owned = BString;

        #[inline]
        fn to_owned(&self) -> BString {
            BString::from(self)
        }
    }
```

`Borrow` and `ToOwned` are not mirror images. Many owned types may lend out a `&BStr` — `BString`, `Vec<u8>`, `String` — but `ToOwned` picks exactly one of them as *the* owned twin. [Step 7's](../borrow_the_way_back/README.md) `type Owned: Borrow<Self>` only requires that the one chosen is among the lenders.

### 11. `camino` 1.2.6: the `Path` pattern, without `clone_into`

[`src/lib.rs#L3343-L3357` ↗](https://github.com/camino-rs/camino/blob/camino-1.2.6/src/lib.rs#L3343-L3357)

```rust
impl Borrow<Utf8Path> for Utf8PathBuf {
    #[inline]
    fn borrow(&self) -> &Utf8Path {
        self.as_path()
    }
}

impl ToOwned for Utf8Path {
    type Owned = Utf8PathBuf;

    #[inline]
    fn to_owned(&self) -> Utf8PathBuf {
        self.to_path_buf()
    }
}
```

The same pair as std's `Path` (§4), minus one method: nothing under `src/` at this tag mentions `clone_into`, so `Utf8Path` gets the trait's default, `*target = self.to_owned()` — a new buffer on every refill, where std's `Path` reuses one. A small, real example of [step 9's](../clone_into_refills/README.md#the-pair) "the saving comes from the impls that override it".

### 12. `regex` 1.13.1: `replace_all` returns the haystack when nothing matches

[`src/regex/string.rs#L891-L897` ↗](https://github.com/rust-lang/regex/blob/1.13.1/src/regex/string.rs#L891-L897) and [`#L972-L987` ↗](https://github.com/rust-lang/regex/blob/1.13.1/src/regex/string.rs#L972-L987)

```rust
    pub fn replace_all<'h, R: Replacer>(
        &self,
        haystack: &'h str,
        rep: R,
    ) -> Cow<'h, str> {
        self.replacen(haystack, 0, rep)
    }
// …
            let mut it = self.find_iter(haystack).enumerate().peekable();
            if it.peek().is_none() {
                return Cow::Borrowed(haystack);
            }
            let mut new = String::with_capacity(haystack.len());
            // … pushes the text between matches and each replacement …
            return Cow::Owned(new);
```

The same shape as `from_utf8_lossy` (§8), in a crate: peek for the first match, hand the input back borrowed if there is none. The `'h` ties the result to the haystack, which is why a caller that wants to keep it past the haystack's life calls [`into_owned`](../to_owned_errors/README.md#18-keeping-a-short-lived-cow-in-a-static-cache).

### 13. `borrowme` 0.1.0: a `Borrow` that returns a value

[`crates/borrowme/src/borrow.rs#L82-L89` ↗](https://github.com/udoprog/borrowme/blob/0.1.0/crates/borrowme/src/borrow.rs#L82-L89) and [`crates/borrowme/src/to_owned.rs#L66-L84` ↗](https://github.com/udoprog/borrowme/blob/0.1.0/crates/borrowme/src/to_owned.rs#L66-L84)

```rust
pub trait Borrow {
    type Target<'a>
    where
        Self: 'a;

    /// Borrow from `self`.
    fn borrow(&self) -> Self::Target<'_>;
}
// …
pub trait ToOwned {
    /// The owned type this is being converted to.
    type Owned;

    /// Perform a covnersion from a reference to owned value.
    fn to_owned(&self) -> Self::Owned;
}

impl<T> ToOwned for &T
where
    T: ?Sized + ToOwned,
{
    type Owned = T::Owned;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        T::to_owned(*self)
    }
}
```

The crate that answers [step 10's](../implementing_it_or_not/README.md#why-a-view-struct-cannot-be-the-borrowed-half) "a view struct cannot be the borrowed half". Its `Borrow` returns `Self::Target<'_>` — a generic associated type, so `borrow` can hand back a `Word<'_>` *by value* instead of a reference to something that must already exist. And its `ToOwned` for `&T` forwards to `T`, the opposite of std, where `&T`'s owned twin is `&T` itself ([step 6](../the_blanket_to_owned/README.md#one-str-two-impls)). The typo in the doc comment is the crate's.

### 14. `beef` 0.5.2: a leaner `Cow`, and a size claim the compiler overtook

[`src/lib.rs#L20-L37` ↗](https://github.com/maciejhirsz/beef/blob/v0.5.2/src/lib.rs#L20-L37)

```rust
//!
//! + `beef::Cow` is 3 words wide: pointer, length, and capacity. It stores the ownership tag in capacity.
//! + `beef::lean::Cow` is 2 words wide, storing length, capacity, and the ownership tag all in one word.
//!
//! Both versions are leaner than the `std::borrow::Cow`:
//!
//! ```rust
//! use std::mem::size_of;
//!
//! const WORD: usize = size_of::<usize>();
//!
//! assert_eq!(size_of::<std::borrow::Cow<str>>(), 4 * WORD);
//! assert_eq!(size_of::<beef::Cow<str>>(), 3 * WORD);
//!
//! // Lean variant is two words on 64-bit architecture
//! #[cfg(target_pointer_width = "64")]
//! assert_eq!(size_of::<beef::lean::Cow<str>>(), 2 * WORD);
//! ```
```

`beef` stores the `Borrowed`/`Owned` tag inside the capacity word — the trick std's own `Cow<str>` now gets from the compiler for nothing. Its docs were right when written: `std::borrow::Cow<str>` *was* four words. On rustc 1.98.0 it is three ([the tag is free](../../../18_Ownership/clone_on_write/README.md#the-tag-is-free), and the run below measures it), so the first assertion no longer holds — and by the crate's own count, `beef::Cow<str>` is now the same size as std's. The crate's last release was in 2022.

## The claims above, run

<!-- output:to_owned_in_real_code -->
*Verified output of [`to_owned_in_real_code.rs`](examples/to_owned_in_real_code.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
§3  [T]::clone_into reuses the elements too, not just the Vec
    target ["ab", "cd"], first String kept its own buffer: true

§5  CStr::clone_into into a CString of the same length reuses its buffer
    name "Bob", same buffer: true

§7  += on an empty Cow<str> borrows the right-hand side instead of allocating
    Borrowed("") += "x"             -> Borrowed
    Owned(capacity 64, empty) += "x" -> Borrowed   (the buffer is dropped)
    Borrowed("a") += "x"            -> Owned "ax"

§8  from_utf8_lossy borrows unless a byte needs replacing
    []                       -> Borrowed ""
    [118, 97, 108, 105, 100] -> Borrowed "valid"
    [99, 97, 102, 233]       -> Owned "caf�"

§9  a Box lends what it holds, which is what lets `type Owned = Box<RawValue>` work
    Box<str>: Borrow<str> -> "{\"raw\": true}"

§14 std's Cow<str>, measured in words
    size_of::<Cow<str>>() = 3 words
```
<!-- /output -->

## See also

- [Implementing `ToOwned` for your own type](../../implementing_to_owned/README.md) — the `Path` pattern of §4, written from scratch and checked
- [Every `ToOwned` error, and its fix](../to_owned_errors/README.md) — what the compiler says when an impl like these is missing a piece
- [Helpful resources for the path](../to_owned_reading_list/README.md) — books and articles; this page is the source code

## Po polsku

Czternaście miejsc, w których `ToOwned`, `Borrow` i `Cow` wykonują prawdziwą pracę: osiem w bibliotece standardowej (rust-lang/rust, tag 1.98.0) i sześć w popularnych bibliotekach (serde_json, bstr, camino, regex, borrowme, beef). Każdy link wskazuje konkretny tag i zakres linii, więc się nie zdezaktualizuje.

Kilka rzeczy, których ścieżka nie mówi wprost: implementacja zbiorcza przekazuje `clone_into` do `clone_from`; `[T]::clone_into` ponownie używa buforów samych elementów (np. każdego `String` w `Vec<String>`); `+=` na pustym `Cow<str>` nie alokuje, tylko zamienia go w `Borrowed`; w serde_json posiadanym bliźniakiem jest `Box<RawValue>`; camino nie nadpisuje `clone_into`, więc każde uzupełnienie alokuje; a dokumentacja `beef` twierdzi, że `std::borrow::Cow<str>` ma cztery słowa — na rustc 1.98.0 ma trzy.

**Szukaj po polsku:** `rust impl ToOwned for` · `rust Cow AddAssign` · `rust clone_into reuse allocation`
