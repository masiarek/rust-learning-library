# The Book

**Level:** 101 · for newcomers

**One line:** [*The Rust Programming Language* ↗](https://doc.rust-lang.org/book/) by Steve Klabnik and Carol Nichols is the one resource that teaches the **model** rather than the syntax — and the reason to read it twice is that chapter 4 does not land the first time for almost anybody.

## Why it is foundational

Most language books are reference manuals with a tutorial bolted on. The Book is the opposite: it is an argument, delivered in order, about *why Rust is shaped this way* — and Rust is a language where the shape is the whole difficulty. You can learn `match` from any source in five minutes. You cannot learn why the compiler will not let you use a value after you moved it from a syntax reference, because the answer is a design decision with consequences three chapters away.

Three things make it the default rather than merely good:

- **It ships with the toolchain.** `rustup doc --book` opens your installed copy, offline, at the version you are actually compiling with. No other resource here is version-matched to your compiler.
- **It is maintained by the project.** Editions change the language; The Book changes with them.
- **It is sequenced deliberately.** Ownership arrives at chapter 4, after you can write a loop and before you can write anything real — which is exactly when the rules stop being abstract.

## Read chapters 1–10, then stop

The Book is 21 chapters and the split is sharp:

| Chapters | What they are | When |
|---|---|---|
| **1–10** | the language: syntax, ownership, structs, enums, collections, errors, generics, traits, lifetimes | **now**, in order |
| 11–14 | testing, an I/O project, iterators and closures, cargo | soon after |
| 15–21 | smart pointers, concurrency, async, OOP patterns, advanced features, a web server | when you need one of them |

Read 1–10 in order and do not skip chapter 4 because it is hard — chapter 4 *is* the book. Chapters 15 onward are a reference you visit; treating them as required reading is the most common way people stall.

## Read it twice

This is the recommendation worth taking literally, and the second read is a different book.

Nobody understands ownership on first contact. What happens instead is that you understand the *sentences* in chapter 4, carry on, and discover at chapter 8 that you cannot actually write anything — because understanding a rule and having it in your fingers are different states, and the gap is invisible from inside.

So on the second pass, use **[the Brown University edition ↗](https://rust-book.cs.brown.edu/)**: the identical text, with interactive quizzes and ownership diagrams inserted throughout. The quizzes are the point, and they are not decoration — they were built by researchers from the misconceptions real readers demonstrably hold, so they are *diagnostic*. A question you get wrong tells you something specific about your model, which re-reading the same paragraph will not.

If you only have the patience for one re-read, re-read [chapter 4 there ↗](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html).

## Two other editions, for two other situations

- **[The abridged Book ↗](https://jasonwalton.ca/rust-book-abridged/)** — the same material, considerably shorter, written for people who already program in something. If The Book's pace is testing your patience rather than your understanding, this is the fix. It is not a substitute for a first read if Rust is your first systems language.
- **`rustup doc --book`** — your offline, version-matched copy. Worth knowing about for the train.

## Chapter numbers move

Worth knowing before you follow anyone's cross-reference, including slides and blog posts: **the chapter numbering changed with the 2024 edition.** Async is now chapter 17, which pushed OOP to 18, Patterns to 19 and Advanced Features to 20. A guide citing "§18.3" for pattern syntax was written against the old numbering and now points somewhere else.

The chapter *names* are stable; the numbers are not. That is why the [rustlings mapping](../rustlings/README.md#which-exercise-goes-with-which-chapter) in this section names both.

## If you are coming from another language

- **Python** — the shock is chapter 4, and it is not syntax. Python never asked who owns a list because the garbage collector owned everything; Rust makes ownership a property you declare and the compiler checks. Everything difficult about Rust for a Python programmer is downstream of that one change.
- **ABAP** — the closest existing instinct is the discipline around `sy-subrc` and explicit cleanup: the language expects you to state what happens when things go wrong. Rust makes that a type rather than a convention. The genuinely new idea is still ownership; nothing in ABAP prepares you for a value having exactly one owner.

## See also

- [rustlings](../rustlings/README.md) — the exercises to run alongside, chapter by chapter
- [Rust by Example](../rust_by_example/README.md) — for looking up syntax mid-chapter
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — this library on the chapter 6 and 9 material, with the output proved

## Po polsku

Polskie tłumaczenie The Book istnieje i jest dobre: [*Język Programowania Rust* ↗](https://github.com/beling/book-pl), wymienione w oficjalnym dodatku z tłumaczeniami. Rozdział 4 — ten, o którym ta strona mówi, że *jest* książką — jest przełożony w całości i trzyma to samo słownictwo, co reszta tej biblioteki: własność, stos i sterta, łańcuch znaków. Ma jednak granicę i pada ona dokładnie w najgorszym możliwym miejscu względem rady „przeczytaj rozdziały 1–10”: po polsku są rozdziały **1–9**, a rozdział 10 — typy generyczne, cechy (*traits*) i czasy życia — jest tam nadal po angielsku. Czyli z dziesięciu zalecanych dostajesz dziewięć, a przełączenie na angielski wypada na rozdziale, w którym polska terminologia i tak jest najsłabiej ustalona.

Ostrzeżenie z sekcji o numeracji rozdziałów dotyczy tego tłumaczenia podwójnie, bo stara numeracja jest w nim **zamrożona**. Polska wersja ma rozdział 17 = programowanie obiektowe, 18 = wzorce, 19 = zagadnienia zaawansowane — czyli układ sprzed edycji 2024, zanim `async` wszedł na siedemnastkę i przesunął resztę. W praktyce: „rozdział 19” w polskim tekście i *chapter 19* w dzisiejszej angielskiej książce to dwa różne rozdziały, a odsyłacz „§18.3 o składni wzorców” w polskim tłumaczeniu jest trafny, w dzisiejszym oryginale prowadzi już gdzie indziej. Nazwy rozdziałów są stabilne, numery nie — więc powołuj się na nazwę, po polsku tym bardziej.

Rada „przeczytaj dwa razy” układa się przez to naturalnie na dwa języki, i warto ją tak właśnie potraktować, zamiast uznać pierwsze (droższe) przejście za załatwienie sprawy. Pierwszy raz po polsku, dokąd tłumaczenie sięga; drugi raz po angielsku w wydaniu Browna, które **istnieje wyłącznie po angielsku** i którego quizy są diagnostyczne — sprawdzają twój model własności, a nie znajomość angielskiego, więc pomylona odpowiedź mówi coś konkretnego o tym, co dalej rozumiesz źle. Do tego `rustup doc --book` daje kopię offline w wersji zgodnej z twoim kompilatorem — po angielsku, ale zawsze tę właściwą.

**Szukaj po polsku:** Język Programowania Rust · własność rozdział 4 · numeracja rozdziałów edycja 2024 · `rust book brown edition quizzes` · `rustup doc --book`
