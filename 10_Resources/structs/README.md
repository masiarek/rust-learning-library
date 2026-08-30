# Structs: the shelf

**Level:** reference

**One line:** The outside material on structs, checked and sorted — and the short answer to *"which one do I actually read?"* is the first row of the first table.

Structs are the topic where the free material is thickest and the quality spread is widest: the same hour buys you an interactive chapter with quizzes, or a fourth video re-explaining `impl` with different variable names. This page exists to make that choice once.

**All links checked 2026-08-24.** Everything that did not answer is [named at the bottom](#what-was-dropped) rather than quietly deleted — a shelf that only lists what works is a shelf you cannot trust about anything else.

---

## Read one thing

| Read | Why this one |
|---|---|
| **[The Book, ch. 5 — Brown edition ↗](https://rust-book.cs.brown.edu/ch05-01-defining-structs.html)** | The canonical chapter *plus* diagnostic quizzes that catch the misunderstanding while you still have it. Strictly better than the plain edition for this chapter, at no cost. |
| [The Book, ch. 5 — plain ↗](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) | Same text, offline with `rustup doc --book`, no JavaScript. |

If you read only one thing on this page, read the first row. The rest of this shelf is for after that, or for a specific question it did not answer.

## When you need the normative answer

Not tutorials — the documents that *define* the behaviour, for when two blog posts disagree.

| Source | Answers |
|---|---|
| [Reference — struct types ↗](https://doc.rust-lang.org/reference/types/struct.html) | What a struct *is* to the language: the three forms, and what a field means |
| [Reference — associated items ↗](https://doc.rust-lang.org/reference/items/associated-items.html) | The `self` receiver grammar, and the exact line between an associated function and a method |
| [Reference — visibility and privacy ↗](https://doc.rust-lang.org/reference/visibility-and-privacy.html) | Why `pub` on a struct says nothing about its fields, and why privacy is per *module* |
| [`std` — the `struct` keyword ↗](https://doc.rust-lang.org/std/keyword.struct.html) | The short official tour, with the unit-struct and tuple-struct cases spelled out |
| [Nomicon — `repr(Rust)` ↗](https://doc.rust-lang.org/nomicon/repr-rust.html) | That field order and padding are **not guaranteed**, which is the whole reason `repr(C)` exists |
| [The RFC index ↗](https://rust-lang.github.io/rfcs/introduction.html) | *Why* a feature is the shape it is — search it for `struct` when the answer is historical |

## Videos

Sorted by what they are for, not by length. Titles and channels are as published; the timestamps are the chapter marks worth jumping to.

| Video | Length & use |
|---|---|
| [**Structs** — Code of the Future ↗](https://www.youtube.com/watch?v=sQEIT603lPM) | ~14 min. The tightest single pass. [01:45 ↗](https://www.youtube.com/watch?v=sQEIT603lPM&t=105s) traditional structs · [03:20 ↗](https://www.youtube.com/watch?v=sQEIT603lPM&t=200s) creating instances · [05:52 ↗](https://www.youtube.com/watch?v=sQEIT603lPM&t=352s) changing fields · [07:49 ↗](https://www.youtube.com/watch?v=sQEIT603lPM&t=469s) tuple structs · [10:57 ↗](https://www.youtube.com/watch?v=sQEIT603lPM&t=657s) `impl` |
| [**Chapter 6 — Structures** — Vandad Nahavandipoor ↗](https://www.youtube.com/watch?v=YZy0eUigvKI) | ~30 min, and the most *complete* chapter list here. [05:38 ↗](https://www.youtube.com/watch?v=YZy0eUigvKI&t=338s) field init shorthand · [07:38 ↗](https://www.youtube.com/watch?v=YZy0eUigvKI&t=458s) struct update syntax · [10:44 ↗](https://www.youtube.com/watch?v=YZy0eUigvKI&t=644s) tuple structs · [14:05 ↗](https://www.youtube.com/watch?v=YZy0eUigvKI&t=845s) implementations · [18:34 ↗](https://www.youtube.com/watch?v=YZy0eUigvKI&t=1114s) a mutating method · [21:46 ↗](https://www.youtube.com/watch?v=YZy0eUigvKI&t=1306s) deriving `Debug` · [25:45 ↗](https://www.youtube.com/watch?v=YZy0eUigvKI&t=1545s) *"associated functions with `self` are methods"* · [26:27 ↗](https://www.youtube.com/watch?v=YZy0eUigvKI&t=1587s) non-method associated functions |
| [**Implement Methods on Rust Structs** — Trevor Sullivan ↗](https://www.youtube.com/watch?v=7EYSXQFRyKY) | Short, and only about `impl`. The one to send someone stuck on methods specifically. |
| [**05 Structs and Methods** — Jeff No Zhao ↗](https://www.youtube.com/watch?v=jASE2K1T8NM) | A second pass in a different voice, if the first one did not land. |
| [**Self-referential structs** — fasterthanlime ↗](https://www.youtube.com/watch?v=xNrglKGi-7o) | **The advanced one.** Why a struct holding a reference to its own field is the hard case, and what `Pin` is for. Watch it *after* lifetimes, not before. |
| [Rust Tutorial Full Course — Derek Banas ↗](https://www.youtube.com/watch?v=ygL_xcavzQ4&t=5090s) | Structs start at [1:24:50 ↗](https://www.youtube.com/watch?v=ygL_xcavzQ4&t=5090s) of a full course. Use the timestamp; the course is long. |
| [Rust 101 Crash Course — Zero To Mastery ↗](https://www.youtube.com/watch?v=lzKeecy4OmQ) | Six hours with 19 exercises. A course, not a lookup. |
| [Rust Demystified — Code to the Moon ↗](https://www.youtube.com/watch?v=TJTDTyNdJdY&t=714s) | Not a struct video, but [11:54 ↗](https://www.youtube.com/watch?v=TJTDTyNdJdY&t=714s) onward is good on the parts that make structs confusing. |

> Every YouTube link here has had its `si=` share-tracking parameter removed. They identify the sharer, not the video, and they do not survive being pasted into a page anybody else reads.

## Practice and articles

| Link | Use |
|---|---|
| [**Rust By Practice** — structs ↗](https://practice.course.rs/compound-types/struct.html) | Fill-in-the-blank exercises that compile, right beside the explanation. The closest thing here to the library's own kata format. |
| [Exercism — Rust track, structs ↗](https://exercism.org/tracks/rust/concepts/structs) | Exercises with human mentoring available. Requires an account. |
| [LogRocket — fundamentals for using structs ↗](https://blog.logrocket.com/fundamentals-for-using-structs-in-rust/) | A solid single-sitting article covering all three forms plus `impl`. |
| [jmmv — structs in tests ↗](https://jmmv.dev/2023/10/rust-test-structs.html) | Narrow and genuinely useful: using a struct to make a test suite readable. Nothing else here covers it. |

## What was dropped

Three links from the original list did not survive the check, and are recorded here so nobody re-adds them:

| Link | Status on 2026-08-24 |
|---|---|
| `marketsplash.com/tutorials/rust/rust-structs/` | **No response.** Connection failed outright. |
| `blog.knoldus.com/…/amp/` | **No response.** Also an AMP URL, which is the wrong thing to bookmark even while it works. |
| Google Docs working notes | **Not linked, by policy.** They are private documents — a link is a 404 for every reader of this site and a leaked document id besides. Same rule as the [shelf front page](../README.md). |

`exercism.org` answered `403` to an automated check but is live in a browser; Cloudflare refuses scripted requests. It is listed above on that basis, not on a green check.

**Rust By Practice moved rather than died.** The widely-circulated `practice.rs/compound-types/struct.html` is a hard 404; the live page is `practice.course.rs/compound-types/struct.html`. The old host still serves its root, which is what makes the dead deep link easy to miss — the domain looks healthy.

---

## The lessons this shelf supports

External material explains the language. The pages in this library are the ones with a program attached that CI compiles and runs, so their output cannot go stale:

[**STRUCTS.md**](../../STRUCTS.md) is the map — the reading order, and what is still missing. Start there rather than here if you want the library's own route through the topic.

## See also

[Books](../books/README.md) · [Exercises](../exercises/README.md) · [Going deeper](../going_deeper/README.md) · [the shelf front page](../README.md)

## Po polsku

Struktury (*structs*) to temat, w którym darmowych materiałów jest najwięcej, a rozrzut jakości największy: ta sama godzina kupuje albo jeden rozdział z quizami sprawdzającymi, czy naprawdę rozumiesz, albo czwarty film tłumaczący `impl` na innych nazwach zmiennych. Stąd reguła tej strony — **przeczytaj jedną rzecz** — i dla kogoś, kto czyta w drugim języku, jest ona ważniejsza, a nie mniej ważna, bo pokusa dalszego szukania „czegoś łatwiejszego” jest większa, a każdy znaleziony materiał po polsku wydaje się cenniejszy, niż jest. Akurat tutaj polski materiał naprawdę istnieje: rozdział 3 Tour of Rust jest przetłumaczony i daje całe słownictwo (struktura, struktura krotkowa, pusta struktura). Rozsądna trasa to wziąć stamtąd nazwy, a zrozumienie z rozdziału 5 *The Book* w wydaniu Browna — tym z quizami — i na tym poprzestać.

Trzy rzeczy zaskakują najbardziej, jeśli `struct` spotkało się wcześniej w C, C++ albo w Javie, a taka jest typowa droga na polskich studiach. Po pierwsze, **metody nie mieszkają w strukturze**: struktura ma wyłącznie pola, a metody stoją osobno, w bloku `impl`, którego może być więcej niż jeden. To nie jest klasa z inną składnią, to dwie osobne deklaracje o tym samym typie. Po drugie, `pub` na strukturze **nie mówi nic o jej polach**, a prywatność w Ruscie jest per **moduł**, nie per typ — inny typ z tego samego modułu widzi twoje prywatne pola, co jest dokładnie odwrotnie niż w Javie i C++. Po trzecie, kolejność pól i wyrównanie w zwykłej strukturze Rusta **nie są gwarantowane**; nawyk z C, że deklaracja opisuje układ bajtów, przenosi się tu po cichu i błędnie — od gwarancji jest `#[repr(C)]`. Wszystkie trzy to pytania z dobrą odpowiedzią, więc idź do dokumentacji referencyjnej, a nie do filmu.

Na koniec dwie uwagi o starzeniu się materiałów, bo w tym temacie działa ono inaczej niż gdzie indziej. Struktury nie zmieniły się od lat, więc starszy artykuł czy film **jest tu zwykle nadal poprawny** — inaczej niż przy iteratorach, gdzie milczenie o nowych metodach dyskwalifikuje ściągawkę. Za to same adresy potrafią umrzeć po cichu: krążący po sieci link `practice.rs/compound-types/struct.html` zwraca 404, choć strona główna tej domeny odpowiada normalnie, więc domena wygląda zdrowo, a link głęboki nie żyje. Żywy adres to `practice.course.rs`.

**Szukaj po polsku:** struktury w Ruscie · struktura krotkowa · `rust struct impl block` · `rust struct update syntax` · `rust repr(C)`
