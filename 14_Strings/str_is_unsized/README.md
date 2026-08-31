# `str` is unsized

**Level:** 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** You never hold a `str`, only ever a pointer to one — and that single fact explains `?Sized`, the fat pointer, why `Clone` cannot help you, and why [`ToOwned`](../../12_Traits/to_owned/README.md) had to be invented.

---

## What this page has to answer

- What "unsized" means concretely: `size_of::<str>()` does not compile, because the size is a property of the value rather than the type.
- The fat pointer — `&str` is 16 bytes, a data pointer plus a length, which is why `size_of::<&str>()` and `size_of::<&String>()` differ.
- `Sized` is an implicit bound on every type parameter, so `?Sized` is a *relaxation* rather than a requirement — and the E0277 you get for forgetting it.
- Why `Clone: Sized` forces `ToOwned` to exist, and why `.clone()` on a `&str` gives back a `&str` and warns.
- The other unsized types you already use — `[T]`, `dyn Trait` — and the one rule they share: always behind a pointer.

## See also

- [`ToOwned`](../../12_Traits/to_owned/README.md)
- [`String` vs `&str`](../string_vs_str/README.md)
- [The anatomy of a `String`](../anatomy_of_a_string/README.md)
- [Marker traits](../../12_Traits/marker_traits/README.md)
- [STRINGS.md](../../STRINGS.md) — the map this page is a gap in
- [Strings: links, books and videos](../resources/README.md) — where to read about it in the meantime

## Po polsku

`str` jest typem o rozmiarze nieznanym w czasie kompilacji (*unsized*, w literaturze też *dynamically sized type*) — dlatego `size_of::<str>()` w ogóle się nie kompiluje, a w kodzie nigdy nie trzyma się samego `str`, tylko wycinek łańcucha `&str`: gruby wskaźnik (*fat pointer*) złożony z adresu i długości, czyli dwa razy szerszy niż `&String`. Polskiego czytelnika najczęściej zaskakuje to, że `Sized` jest **domyślnym**, niewidocznym ograniczeniem każdego parametru generycznego, więc `?Sized` niczego nie wymaga — ono to ograniczenie *rozluźnia*, a `E0277` przy jego braku mówi dokładnie o tym. Stąd bierze się też `ToOwned`: `Clone` wymaga `Sized`, więc `.clone()` na `&str` oddaje tylko kolejny `&str`, a żeby dostać wersję na własność, musiała powstać osobna cecha (*trait*).

**Szukaj po polsku:** typy o nieznanym rozmiarze · gruby wskaźnik · `rust unsized types` · `rust ?Sized bound` · `rust fat pointer str`
