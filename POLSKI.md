# Po polsku — the Polish side of this library

**Level:** reference · for anyone writing a Polish section

**One line:** Every lesson ends with a `## Po polsku` section written *in* Polish rather than translated *into* it — and it always keeps the English term in sight, because the compiler, the docs and every crate on crates.io speak English and the reader has to recognise the word when it appears in a terminal.

## Why a section and not a translation

A translated page is two pages that must be edited together, and the second one rots the first time somebody fixes a typo in the first. A Polish **section** at the bottom of the English page is one file, one URL, one edit.

It is also a different piece of writing, not the same writing in another language. The English page above it is the lesson. The Polish section answers the question a Polish reader actually arrives with: *what is this called in Polish, what does the Polish-language material I already know call it, and which English word do I type into a search box?* Same concept, different job — which is why translating the page above would not produce it.

This is the pattern the [biology library ↗](https://github.com/masiarek/biology-learning-library) already uses, adapted for one difference that matters a great deal here.

## The one rule: the English term stays visible

In biology the Polish term is the operative one — a student writes *przerzut* on a Polish exam, and *metastasis* is the foreign gloss. **Rust is the other way round.** The operative language is English:

- `rustc` prints `borrow of moved value`, never *pożyczenie przeniesionej wartości*
- [std ↗](https://doc.rust-lang.org/std/) is English, and so is every crate's documentation
- A search for *pożyczanie w Ruscie* returns a handful of blog posts; `rust borrow checker` returns the answer

So a Polish section that hides the English word has taught the reader something they cannot use. Write the Polish, and put the English beside it on first use:

```markdown
Pożyczanie (*borrowing*) to sposób na sięgnięcie po dane bez przejmowania ich na własność.
```

Never the reverse — the Polish carries the sentence, the English rides along in parentheses.

## The shape of the section

It is the **last section on the page**, after See also. Two parts, and the second is not optional:

```markdown
## Po polsku

<Two to five paragraphs. The same concept, framed the way Polish-language
material frames it. Not a translation of the page above — a reader who
speaks both should still get something from reading both.>

**Szukaj po polsku:** term · term · term
```

The **Szukaj po polsku** line is the part that earns its keep. It gives the exact phrases to type — including the English ones, when the English ones are what actually find something.

## Where the vocabulary comes from

[Tour of Rust ↗](https://tourofrust.com/TOC_pl.html) has the only substantial Polish Rust course on the web, and this library follows its choices wherever they exist. **They stop after chapter 5.** Chapters 1–5 (basics, control flow, structs, generics, ownership and borrowing) are translated; chapters 6–9 — text, traits and OOP, smart pointers, modules — are still in English on the Polish pages.

That is most of the vocabulary this library needs, and it means the table below has two halves with very different authority:

- **Anchored** — the term is Tour of Rust's, and a Polish reader coming from there will already have met it.
- **Chosen here** — no Polish course covers it, Polish usage is genuinely split, and this library picked one and uses it consistently. Marked so nobody mistakes consistency for consensus.

## The terminology

### Anchored in Tour of Rust's Polish chapters 1–5

| English | Polski | Note |
|---|---|---|
| variable | zmienna | |
| [mutable](18_Ownership/a_name_is_not_a_place/README.md) | mutowalna | *zmienna mutowalna*; ToR titles the lesson "Zmienianie Zmiennych" |
| constant | stała | |
| type | typ | *podstawowe typy* for the primitives |
| type conversion, cast | konwersja typów | |
| array | tablica | |
| function | funkcja | |
| control flow | kontrola przepływu sterowania | |
| loop | pętla | |
| expression block | blok wyrażeniowy | |
| [struct](16_Structs/README.md) | struktura | |
| tuple struct | struktura krotkowa | |
| unit struct | pusta struktura | |
| method | metoda | *wywoływanie metod* |
| memory | pamięć | |
| [enum](13_Enums/README.md) | wyliczenie | |
| [generics](22_Generics/README.md) | typy generyczne | |
| `Option` | Opcja | keep `Option` in code voice |
| `Result` | Rezultat | keep `Result` in code voice |
| `Vec`, vector | wektor | |
| [ownership](18_Ownership/ownership_and_moves/README.md) | własność | *posiadanie danych* for owning |
| [scope](18_Ownership/scope_is_about_names/README.md) | zasięg | |
| drop, release | wypuszczenie zasobu | ToR: *Wypuszczanie Zasobów* |
| move | przeniesienie własności | |
| [borrowing](18_Ownership/borrowing/README.md) | pożyczanie | |
| reference | referencja | |
| mutable reference | referencja mutowalna | |
| dereference | dereferencja | |
| [lifetime](18_Ownership/lifetime_annotations/README.md) | czas życia | *czas życia zmiennej* |
| static lifetime | statyczny czas życia | |
| error handling | obsługa błędów | |

### Chosen here — no Polish course covers these

| English | Polski | Why this one |
|---|---|---|
| [trait](12_Traits/README.md) | cecha | Say **cecha (*trait*)** once, then use `trait`. The keyword is `trait`, and a reader who only learns *cecha* cannot read an error message. |
| trait object | obiekt cechy | |
| [closure](23_Closures/README.md) | domknięcie | Widespread in Polish CS writing, unlike most of this half. |
| [iterator](24_Iterators/README.md) | iterator | Same word. |
| [slice](14_Strings/README.md) | wycinek | *plaster* also appears; *wycinek* is commoner and less odd. |
| string slice, `&str` | wycinek łańcucha | |
| `String` | łańcuch znaków | Keep `String` when naming the type. |
| `char` | znak | |
| [stack](18_Ownership/stack_and_heap/README.md) | stos | |
| heap | sterta | |
| [smart pointer](18_Ownership/reference_counting/README.md) | inteligentny wskaźnik | *sprytny wskaźnik* also seen. |
| raw pointer | surowy wskaźnik | |
| reference counting | zliczanie referencji | |
| borrow checker | borrow checker | **Not translated.** It is the compiler's name for itself and every search result is English. |
| shadowing | przesłanianie | |
| [pattern matching](30_Pattern_Matching/README.md) | dopasowanie wzorców | |
| [module](27_Modules/README.md) | moduł | |
| crate | crate | **Not translated.** *Skrzynka* is a literal rendering nobody uses; `crate` is a keyword and a command-line word. |
| [thread](09_Advanced/channels/README.md) | wątek | |
| panic | panika | The verb stays: *program panikuje*. |
| compile | kompilować | |

| [pattern](30_Pattern_Matching/README.md) | wzorzec | *dopasowanie wzorców* for pattern matching. |
| irrefutable / refutable | niepodważalny / podważalny | Polish usage is genuinely unsettled here; these are the transparent calques and the English is always glossed beside them. |
| destructuring | destrukturyzacja | |
| exhaustiveness | kompletność dopasowania | The property `E0004` is about. |
| match arm | ramię | |
| [match guard](30_Pattern_Matching/match_guards/README.md) | strażnik | Same word as the RAII guard (`MutexGuard`), and the two do not collide in practice. |
| undefined behaviour, UB | zachowanie niezdefiniowane | |
| [data race](31_C_and_Cpp/data_races/README.md) | wyścig danych | Keep distinct from *sytuacja wyścigu* (race condition) — Rust rules out the first, not the second. |
| [use-after-free](31_C_and_Cpp/use_after_free/README.md) | użycie po zwolnieniu | |
| dangling pointer | wiszący wskaźnik | |
| assertion | asercja | |
| orphan rule | reguła sieroty | |
| [capacity](26_Collections/the_vec/README.md) | pojemność | Keep distinct from **długość** (length); they grow independently. |
| amortised | zamortyzowany | |
| array vs `Vec` | tablica vs wektor | The one that misleads most: **tablica** is `[T; N]`, fixed and on the stack. Python's list and Java's `ArrayList` are `Vec`. |

### Words to leave in English

`unwrap` · `match` · `impl` · `derive` · `cargo` · `clippy` · `rustfmt` · `no_std` · every error code (`E0308`), every attribute (`#[derive]`), every keyword.

A Polish sentence containing an English keyword is normal Polish technical writing, and inflecting it is normal too — *w `match`u*, *przez `unwrap`a*. Do not invent a Polish keyword.


### One thing the terminology cannot fix

`Ord` on a `String` compares Unicode code points, not the Polish alphabet — `ą` sorts *after* `z`, not next to `a` — so a `BTreeMap` of surnames is not a Polish-sorted list. That is correct behaviour for a byte-wise comparison, not a bug, and the standard library has no locale-aware collation and does not pretend to. Any page that sorts human-readable Polish text should say so rather than imply the order is alphabetical.

## See also

- [GLOSSARY.md](GLOSSARY.md) — the English terms, defined
- [CONTRIBUTING.md](CONTRIBUTING.md) — how to add a lesson
- [Tour of Rust, po polsku ↗](https://tourofrust.com/TOC_pl.html) — chapters 1–5
