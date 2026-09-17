# `AsRef` and `AsMut`

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** [`AsRef<T>` ↗](https://doc.rust-lang.org/std/convert/trait.AsRef.html) is the cheap conversion — a borrow of one type as a reference to another, with no allocation and no failure — which is why `File::open` takes `P: AsRef<Path>` and accepts a `&str`, a `String`, a `&Path` and a `PathBuf` alike.

## What it has to cover

- The trait's one method, `fn as_ref(&self) -> &T`, and `AsMut`'s mirror `fn as_mut(&mut self) -> &mut T`
- **The generic parameter it is for:** `fn open<P: AsRef<Path>>(path: P)`, and the four argument types that then work without a conversion at the call site
- The cost of that generic, and std's own fix: a thin generic wrapper that calls `as_ref()` once and hands off to a non-generic inner function
- **Against its neighbours:** `AsRef` is cheap and reference-to-reference; `From`/`Into` may allocate and produce an owned value; `Borrow` adds a promise that `Hash`, `Eq` and `Ord` agree, which `AsRef` does not make
- Why there is no blanket `impl AsRef<T> for T`, so `x.as_ref()` on some types is ambiguous or missing
- When not to use it: a function that takes `&str` is clearer if every caller already has one

## The trap it exists for

Calling `.as_ref()` and letting inference choose the target. A `String` implements `AsRef<str>`, `AsRef<[u8]>`, `AsRef<Path>` and `AsRef<OsStr>`, so an unannotated call fails with a "type annotations needed" error far from the reason.

## See also

- [`Path` and `PathBuf`](../../04_Files/path_and_pathbuf/README.md) — the `AsRef<Path>` parameter this page generalises
- [`Borrow`](../../12_Traits/borrow_trait/README.md) — the stricter trait, and the promise it adds
- [`From` and `Into`](../from_and_into/README.md) — the conversion that may allocate
- [Coercion](../coercion/README.md) — the conversion you never write, which often makes `AsRef` unnecessary

## Po polsku

`AsRef<T>` to najtańsza konwersja: pożyczenie jednej wartości jako referencji do innego typu, bez alokacji i bez możliwości błędu. Dlatego `File::open` przyjmuje `P: AsRef<Path>` i działa z `&str`, `String`, `&Path` oraz `PathBuf`. Od `Into` różni się tym, że nie tworzy nowej wartości, a od `Borrow` — tym, że nie obiecuje zgodności `Hash`, `Eq` i `Ord`.

**Szukaj po polsku:** konwersje w Ruscie · `rust asref vs borrow` · `rust asref path generic` · `rust asref vs into`
