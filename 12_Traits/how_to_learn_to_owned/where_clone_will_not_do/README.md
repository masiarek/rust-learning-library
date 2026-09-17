# Where `Clone` will not do: code only

[How to learn `ToOwned`](../README.md) › **Beside the steps** · the question it answers: [Step 1](../clone_vs_to_owned/README.md)

**Level:** 101 → 201 · code, not prose

**One line:** Ten places where `.clone()` does not compile and `ToOwned` does. In each pair, the first fence must fail and the second must compile, and [`check_fences.py`](../../../tools/check_fences.py) holds both to that on every build. The errors in the comments are rustc 1.98.0's.

## 1. A `&str` into a `String`

```rust,compile_fail
pub fn keep(name: &str) -> String {
    name.clone() // E0308: expected `String`, found `&str`
}
```

```rust
pub fn keep(name: &str) -> String {
    name.to_owned()
}
```

## 2. Dereferencing first does not help

```rust,compile_fail
pub fn keep(name: &str) -> String {
    (*name).clone() // E0599: no method named `clone` found for type `str`
}
```

```rust
pub fn keep(name: &str) -> String {
    (*name).to_owned()
}
```

## 3. A slice into a `Vec`

```rust,compile_fail
pub fn keep(scores: &[i32]) -> Vec<i32> {
    scores.clone() // E0308: expected `Vec<i32>`, found `&[i32]`
}
```

```rust
pub fn keep(scores: &[i32]) -> Vec<i32> {
    scores.to_owned()
}
```

## 4. A `&Path` into a `PathBuf`

```rust,compile_fail
use std::path::{Path, PathBuf};

pub fn keep(file: &Path) -> PathBuf {
    file.clone() // E0308: expected `PathBuf`, found `&Path`
}
```

```rust
use std::path::{Path, PathBuf};

pub fn keep(file: &Path) -> PathBuf {
    file.to_owned()
}
```

## 5. Refilling a `String` you already have

```rust,compile_fail
pub fn refill(buf: &mut String, name: &str) {
    buf.clone_from(name); // E0308: expected `&String`, found `&str`
}
```

```rust
pub fn refill(buf: &mut String, name: &str) {
    name.clone_into(buf);
}
```

## 6. One function for every borrowed type

```rust,compile_fail
pub fn own_all<T: Clone>(items: &[&T]) -> Vec<T> {
    items.iter().map(|&item| item.clone()).collect()
}

pub fn names() -> Vec<String> {
    own_all::<str>(&["Ada", "Grace"]) // E0277: the trait bound `str: Clone` is not satisfied
}
```

```rust
pub fn own_all<B: ToOwned + ?Sized>(items: &[&B]) -> Vec<B::Owned> {
    items.iter().map(|&item| item.to_owned()).collect()
}

pub fn names() -> Vec<String> {
    own_all::<str>(&["Ada", "Grace"])
}

pub fn rows() -> Vec<Vec<i32>> {
    own_all::<[i32]>(&[&[1, 2][..], &[3][..]])
}
```

## 7. A struct that keeps the owned form of whatever it is lent

```rust,compile_fail
pub struct Keeper<T: Clone> {
    pub kept: T,
}

impl<T: Clone> Keeper<T> {
    pub fn new(borrowed: &T) -> Self {
        Keeper { kept: borrowed.clone() }
    }
}

pub fn keep_name() -> Keeper<str> { // E0277: the trait bound `str: Clone` is not satisfied
    Keeper::new("Ada")
}
```

```rust
pub struct Keeper<B: ToOwned + ?Sized> {
    pub kept: B::Owned,
}

impl<B: ToOwned + ?Sized> Keeper<B> {
    pub fn new(borrowed: &B) -> Self {
        Keeper { kept: borrowed.to_owned() }
    }
}

pub fn keep_name() -> Keeper<str> {
    Keeper::new("Ada")
}

pub fn keep_bytes() -> Keeper<[u8]> {
    Keeper::new(b"Ada")
}
```

## 8. Clone-on-write, built on `Clone`

```rust,compile_fail
pub enum MyCow<'a, T: Clone> {
    Borrowed(&'a T),
    Owned(T),
}

pub fn underscores(label: &str) -> MyCow<'_, str> { // E0277: the trait bound `str: Clone` is not satisfied
    if label.contains(' ') {
        MyCow::Owned(label.replace(' ', "_"))
    } else {
        MyCow::Borrowed(label)
    }
}
```

```rust
use std::borrow::Cow;

pub fn underscores(label: &str) -> Cow<'_, str> {
    if label.contains(' ') {
        Cow::Owned(label.replace(' ', "_"))
    } else {
        Cow::Borrowed(label)
    }
}
```

## 9. Count words, allocating only for a word not seen before

```rust,compile_fail
use std::collections::HashMap;

pub fn count(tally: &mut HashMap<String, u32>, word: &str) {
    match tally.get_mut(word) {
        Some(n) => *n += 1,
        None => {
            tally.insert(word.clone(), 1); // E0308: expected `String`, found `&str`
        }
    }
}
```

```rust
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

pub fn count<K, Q>(tally: &mut HashMap<K, u32>, word: &Q)
where
    K: Borrow<Q> + Hash + Eq,
    Q: ToOwned<Owned = K> + Hash + Eq + ?Sized,
{
    match tally.get_mut(word) {
        Some(n) => *n += 1,
        None => {
            tally.insert(word.to_owned(), 1);
        }
    }
}
```

## 10. Owning a list of words — and the `&` that still gets in the way

```rust,compile_fail
pub fn own_words(words: &[&str]) -> Vec<String> {
    words.iter().cloned().collect() // E0277: `Vec<String>` cannot be built from an iterator over elements of type `&str`
}
```

```rust,compile_fail
pub fn own_words(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_owned()).collect() // E0277: the same — `word` is a `&&str`, so this is `&str`'s to_owned
}
```

```rust
pub fn own_words(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| word.to_owned()).collect()
}
```

## All of them, run

<!-- output:where_clone_will_not_do -->
*Verified output of [`where_clone_will_not_do.rs`](examples/where_clone_will_not_do.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
 1. keep("Ada")                -> "Ada", alloc::string::String
 2. (*name).to_owned()         -> "Ada", alloc::string::String
 3. keep(&[3, 1, 2])           -> [3, 1, 2], alloc::vec::Vec<i32>
 4. keep(Path::new(..))        -> "notes.txt", std::path::PathBuf
 5. refill(&mut buf, "Grace")  -> "Grace", same buffer: true
 6. own_all::<str>             -> ["Ada", "Grace"], alloc::vec::Vec<alloc::string::String>
    own_all::<[i32]>           -> [[1, 2], [3]], alloc::vec::Vec<alloc::vec::Vec<i32>>
 7. Keeper::<str>::new         -> kept "Ada", alloc::string::String
    Keeper::<[u8]>::new        -> kept [65, 100, 97], alloc::vec::Vec<u8>
 8. underscores("first name")  -> Owned "first_name"
 8. underscores("surname")     -> Borrowed "surname"
 9. count words, &str keys     -> [("ada", 2), ("grace", 1)]
    count chunks, &[u8] keys   -> [([71, 69, 84], 2), ([80, 85, 84], 1)]
10. own_words(&["Ada", ..])    -> ["Ada", "Grace"], alloc::vec::Vec<alloc::string::String>
```
<!-- /output -->

## Why each one fails

| § | The step that explains it |
|---|---|
| 1, 3, 4 | [Step 4](../clone_returns_self/README.md) — `clone` returns `Self`, and on a `&str` `Self` is `&str` |
| 2, 6, 7, 8 | [Step 2](../types_with_no_size/README.md) and [step 4](../clone_returns_self/README.md#self-in-self-out) — `str` has no size, so it can never be `Clone` |
| 5 | [Step 9](../clone_into_refills/README.md) — `clone_from` wants the same type; `clone_into` wants the owned twin |
| 8 | [Step 7](../borrow_the_way_back/README.md#cow-is-built-on-the-pair) — `Cow` is built on `ToOwned` for exactly this reason |
| 9 | [Step 7](../borrow_the_way_back/README.md#what-get-asks-for) — `Borrow` for the lookup, `ToOwned` for the insert |
| 10 | [Step 5](../the_dot_picks_first/README.md#walk-it-for-to_owned-and-see-where-a-deref-happens) and [step 6](../the_blanket_to_owned/README.md#one-str-two-impls) — on a `&&str`, `.to_owned()` finds `&str`'s impl first |

## Po polsku

Dziesięć par kodu: w każdej pierwsza wersja z `.clone()` się nie kompiluje, a druga z `ToOwned` — tak. Obie są sprawdzane przy każdym budowaniu biblioteki (`compile_fail` musi się nie skompilować). Komentarze w kodzie to komunikaty rustc 1.98.0, a sekcja „All of them, run” pokazuje zweryfikowany wynik działających wersji. Tabela na końcu wskazuje krok ścieżki, który wyjaśnia każdą porażkę — w sekcji 10 pułapką jest nawet samo `to_owned()`, wołane na `&&str`.

**Szukaj po polsku:** `rust clone vs to_owned` · `rust str Clone not satisfied` · `rust ToOwned generic ?Sized`
