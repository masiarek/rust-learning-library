# The comparison traits

**Level:** 201 · working knowledge

**One line:** `==` is `PartialEq` and `<` is `PartialOrd`, and the `Partial` is not decoration — it is the escape hatch that lets `f64` have both while having neither `Eq` nor `Ord`, because `NaN != NaN` breaks a promise those two make.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Four traits, two pairs: `PartialEq`/`Eq` and `PartialOrd`/`Ord`, and what each of `Eq` and `Ord` adds — reflexivity, and totality
- `Eq` and `Ord` have **no methods**. They are promises, which is why deriving one is a claim rather than an implementation
- What that buys: `sort()` needs `Ord`, `HashMap` keys need `Eq + Hash`, `BTreeMap` keys need `Ord` — so the missing derive shows up as `E0277` on a container, not on a comparison
- Deriving versus writing: a derived `PartialEq` compares every field, a derived `Ord` compares fields **in declaration order**, which means reordering a struct's fields silently changes its sort
- `PartialEq<Rhs>` is generic in the other operand, which is how `String == &str` works
- The `Ordering` enum, `cmp`, `partial_cmp`, and the `sort_by(|a, b| …)` idiom that follows
- Implementing `Ord` by hand for a domain rule (a candidate ranked by score, then by name) and keeping it consistent with `PartialEq`

## The trap it exists for

`derive(PartialOrd)` on a struct sorts by field declaration order, and nothing on the page you are reading says so. Move `name` above `score` in a refactor and every sort in the program changes meaning, with no error and no warning. Writing `Ord` by hand — or at minimum a test that pins the intended order — is what stops that.

## See also

- [What a float actually stores](../../19_Numbers/what_a_float_stores/README.md) — the `NaN` this page's `Partial` exists for, measured rather than asserted
- [What a trait is](../what_a_trait_is/README.md) — the mechanism underneath
- [Marker traits](../marker_traits/README.md) — `Eq` and `Ord` are nearly markers: promises with no code
- [Operators are traits](../operators_are_traits/README.md) — the other half of "an operator is a trait call"
- [`PartialEq` ↗](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) · [`Ord` ↗](https://doc.rust-lang.org/std/cmp/trait.Ord.html) · [Comprehensive Rust: Comparisons ↗](https://google.github.io/comprehensive-rust/std-traits/comparisons.html)

## Po polsku

Przedrostek `Partial` czyta się tu wprost przez słownik polskiej matematyki dyskretnej i to jest największa przewaga polskiego czytelnika na tej stronie: `Eq` żąda **zwrotności** (relacja równoważności), `Ord` żąda porządku **liniowego** (całkowitego), a `f64` nie spełnia żadnego z tych warunków, bo `NaN != NaN` — dlatego ma `PartialEq` i `PartialOrd`, ale ani `Eq`, ani `Ord`. Skutek praktyczny jest taki, że brak `Ord` zgłasza się jako `E0277` **na pojemniku, a nie na porównaniu**: `sort()` wymaga `Ord`, klucze `HashMap` wymagają `Eq + Hash`, klucze `BTreeMap` — `Ord`. Pułapka, dla której ta strona w ogóle powstała, jest zaś zupełnie przyziemna: `#[derive(PartialOrd)]` porównuje pola **w kolejności deklaracji**, więc przestawienie `name` przed `score` przy refaktorze po cichu zmienia sens każdego sortowania w programie — bez błędu i bez ostrzeżenia. Uwaga na koniec: strona jest na razie szkicem (*stub*), bez uruchamialnego przykładu, więc jej tezy nie przeszły jeszcze weryfikacji, której poddawana jest reszta biblioteki.

**Szukaj po polsku:** porządek częściowy a liniowy · relacja równoważności i zwrotność · `rust derive PartialOrd field order` · `rust f64 does not implement Ord` · `E0277 trait bound Ord is not satisfied`
