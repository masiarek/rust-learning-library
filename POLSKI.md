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

**A linked term has a lesson in this library; a bare one does not yet.** That is the only thing the linking means — it is not a judgement about how important the word is.

| English | Polski | Note |
|---|---|---|
| [variable](15_First_Programs/variables/README.md) | zmienna | |
| [mutable](18_Ownership/a_name_is_not_a_place/README.md) | mutowalna | *zmienna mutowalna*; ToR titles the lesson "Zmienianie Zmiennych" |
| [constant](27_Modules/const_and_static/README.md) | stała | |
| [type](15_First_Programs/values/README.md) | typ | *podstawowe typy* for the primitives |
| [type conversion, cast](29_Conversion/casting_with_as/README.md) | konwersja typów | |
| [array](26_Collections/arrays_and_slices/README.md) | tablica | `[T; N]`, fixed and on the stack. Python's list and Java's `ArrayList` are a `Vec`, not a *tablica* — the commonest false friend here. |
| [function](25_Control_Flow/functions/README.md) | funkcja | |
| [control flow](25_Control_Flow/README.md) | kontrola przepływu sterowania | |
| [loop](25_Control_Flow/the_loop_keyword/README.md) | pętla | |
| [expression block](15_First_Programs/a_block_is_an_expression/README.md) | blok wyrażeniowy | |
| [struct](16_Structs/README.md) | struktura | |
| [tuple struct, unit struct](16_Structs/what_a_struct_is/README.md) | struktura krotkowa, pusta struktura | |
| [method](16_Structs/impl_blocks/README.md) | metoda | *wywoływanie metod* |
| [memory](18_Ownership/README.md) | pamięć | |
| [enum](13_Enums/README.md) | wyliczenie | |
| [generics](22_Generics/README.md) | typy generyczne | |
| [`Option`](17_Option_and_Result/some_and_none/README.md) | Opcja | keep `Option` in code voice |
| [`Result`](17_Option_and_Result/option_vs_result/README.md) | Rezultat | keep `Result` in code voice |
| [`Vec`, vector](26_Collections/README.md) | wektor | |
| [ownership](18_Ownership/ownership_and_moves/README.md) | własność | *posiadanie danych* for owning |
| [scope](18_Ownership/scope_is_about_names/README.md) | zasięg | |
| [drop, release](12_Traits/drop_and_raii/README.md) | wypuszczenie zasobu | ToR: *Wypuszczanie Zasobów* |
| [move](18_Ownership/no_move_trait/README.md) | przeniesienie własności | Responsibility moves; the bytes do not. |
| [borrowing](18_Ownership/borrowing/README.md) | pożyczanie | |
| [reference, mutable reference](18_Ownership/what_an_address_shows/README.md) | referencja, referencja mutowalna | |
| dereference | dereferencja | |
| [lifetime](18_Ownership/lifetime_annotations/README.md) | czas życia | *czas życia zmiennej* |
| [static lifetime](14_Strings/static_str/README.md) | statyczny czas życia | |
| [error handling](02_Errors/README.md) | obsługa błędów | |

### Chosen here — no Polish course covers these

| English | Polski | Why this one |
|---|---|---|
| [trait](12_Traits/README.md) | cecha | Say **cecha (*trait*)** once, then use `trait`. The keyword is `trait`, and a reader who only learns *cecha* cannot read an error message. |
| [trait object](12_Traits/static_vs_dynamic_dispatch/README.md) | obiekt cechy | |
| [closure](23_Closures/README.md) | domknięcie | Widespread in Polish CS writing, unlike most of this half. |
| [iterator](24_Iterators/README.md) | iterator | Same word. |
| [slice](14_Strings/README.md) | wycinek | *plaster* also appears; *wycinek* is commoner and less odd. |
| [string slice, `&str`](14_Strings/string_vs_str/README.md) | wycinek łańcucha | |
| [`String`](14_Strings/making_a_string/README.md) | łańcuch znaków | Keep `String` when naming the type. |
| [`char`](14_Strings/meet_the_char/README.md) | znak | A Unicode code point — not a byte, and not a letter on screen. |
| [stack / heap](18_Ownership/stack_and_heap/README.md) | stos / sterta | **sterta** is not *kopiec*, the binary-heap data structure — English calls both "heap". |
| [smart pointer](26_Collections/the_box/README.md) | inteligentny wskaźnik | *sprytny wskaźnik* also seen. |
| [raw pointer](09_Advanced/what_unsafe_turns_off/README.md) | surowy wskaźnik | |
| [reference counting](18_Ownership/reference_counting/README.md) | zliczanie referencji | |
| borrow checker | borrow checker | **Not translated.** It is the compiler's name for itself and every search result is English. |
| [shadowing](SHADOWING.md) | przesłanianie | Not *overriding* — Polish OOP writing uses the same word for overriding a base-class method, which is a different thing entirely. |
| [pattern, pattern matching](30_Pattern_Matching/README.md) | wzorzec, dopasowanie wzorców | |
| [irrefutable / refutable](30_Pattern_Matching/irrefutable_patterns/README.md) | niepodważalny / podważalny | Polish usage is genuinely unsettled here; these are the transparent calques and the English is always glossed beside them. |
| [destructuring](30_Pattern_Matching/destructuring_structs/README.md) | destrukturyzacja | |
| [exhaustiveness](30_Pattern_Matching/destructuring_enums/README.md) | kompletność dopasowania | The property `E0004` is about. |
| [match arm](25_Control_Flow/match_expressions/README.md) | ramię | |
| [match guard](30_Pattern_Matching/match_guards/README.md) | strażnik | Same word as the RAII guard (`MutexGuard`); the two do not collide in practice. |
| [module](27_Modules/README.md) | moduł | |
| [crate](05_Tooling/cargo_dependencies/README.md) | crate | **Not translated.** *Skrzynka* is a literal rendering nobody uses; `crate` is a keyword and a command-line word. |
| [thread](09_Advanced/spawning_a_thread/README.md) | wątek | |
| [panic](17_Option_and_Result/what_a_panic_costs/README.md) | panika | The verb stays: *program panikuje*. |
| [compile](20_Compilers/what_a_compiler_does/README.md) | kompilować | |
| [undefined behaviour, UB](31_C_and_Cpp/README.md) | zachowanie niezdefiniowane | Not "a strange number comes out" — the compiler reasons backwards from the assumption that the case cannot arise. |
| [data race](31_C_and_Cpp/data_races/README.md) | wyścig danych | Keep distinct from *sytuacja wyścigu* (race condition) — Rust rules out the first, not the second. |
| [use-after-free](31_C_and_Cpp/use_after_free/README.md) | użycie po zwolnieniu | *wiszący wskaźnik* is the dangling pointer, same page. |
| [assertion](28_Testing/what_a_test_asserts/README.md) | asercja | |
| [orphan rule](29_Conversion/from_and_into/README.md) | reguła sieroty | |
| [capacity](26_Collections/the_vec/README.md) | pojemność | Keep distinct from **długość** (length); they grow independently, and the growth is *zamortyzowany* (amortised). |

### Words to leave in English

`unwrap` · `match` · `impl` · `derive` · `cargo` · `clippy` · `rustfmt` · `no_std` · every error code (`E0308`), every attribute (`#[derive]`), every keyword.

A Polish sentence containing an English keyword is normal Polish technical writing, and inflecting it is normal too — *w `match`u*, *przez `unwrap`a*. Do not invent a Polish keyword.


### One thing the terminology cannot fix

`Ord` on a `String` compares Unicode code points, not the Polish alphabet — `ą` sorts *after* `z`, not next to `a` — so a `BTreeMap` of surnames is not a Polish-sorted list. That is correct behaviour for a byte-wise comparison, not a bug, and the standard library has no locale-aware collation and does not pretend to. Any page that sorts human-readable Polish text should say so rather than imply the order is alphabetical.

## See also

- [GLOSSARY.md](GLOSSARY.md) — the English terms, defined
- [CONTRIBUTING.md](CONTRIBUTING.md) — how to add a lesson
- [Tour of Rust, po polsku ↗](https://tourofrust.com/TOC_pl.html) — chapters 1–5
