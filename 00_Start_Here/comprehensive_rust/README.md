# Comprehensive Rust

**Level:** 101 → 201 · for newcomers

**One line:** [Comprehensive Rust ↗](https://google.github.io/comprehensive-rust/) is Google's four-day Rust class, published as a book — and it is the only one of the four resources here written to be **taught**, which is why every slide carries a duration and a block of speaker notes, and why it is the best *syllabus* on this page and the weakest thing to read alone.

## What it is

A course, not a tutorial. Four days of Rust Fundamentals (Days 1–4, three or four chapters each), plus four standalone classes that are the reason it exists inside Google: **Android**, **Chromium**, **Bare-metal**, and a full day of **Concurrency**. Apache-2.0 for the code and CC-BY-4.0 for the prose, translated into a dozen-odd languages, and it runs its own exercises in-page.

The shape gives it two properties nothing else on this page has. Every slide states **how long it should take**, which makes it a real plan rather than a reading list. And every slide has **speaker notes** — the questions students actually ask, the mistake to demonstrate live, the thing to say when someone asks about `while true`. Those notes are the most useful part of the book and the easiest to miss, because the default view collapses them.

The flip side is what a slide deck is: the visible half of a lesson. Read alone, "Blocks and Scopes" is four bullets and a five-line program. That is not a criticism of the course — it is what it is for.

## Where it fits beside the other three

| | Gives you | Fails at |
|---|---|---|
| [The Book](../the_book/README.md) | the model, at length | making you type |
| [Rust by Example](../rust_by_example/README.md) | the syntax, runnable | the why |
| [rustlings](../rustlings/README.md) | the reps | breadth |
| **Comprehensive Rust** | the **order**, timed, plus an instructor's notes on each idea | depth without the instructor |

Use it as the **spine** — its chapter order is better sequenced than most self-study paths, because it was tuned against real classes — and read The Book for the parts it moves past in five minutes.

## Chapter by chapter, against this library

The Fundamentals days, and where each chapter lands here. This is a map, not a claim of equivalence: where a row says *covered*, this library takes the same topic somewhere between one and five pages deeper.

| CR chapter | Here |
|---|---|
| **Day 1** Hello, World | [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) · [Benefits of Rust](../benefits_of_rust/README.md) |
| Types and Values | [Variables](../../15_First_Programs/variables/README.md) · [Values](../../15_First_Programs/values/README.md) · [Type inference](../../15_First_Programs/type_inference/README.md) · [What a type annotation does](../../15_First_Programs/what_an_annotation_does/README.md) |
| Control Flow Basics | [Control flow](../../25_Control_Flow/README.md) *(stubs)* · [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) |
| Tuples and Arrays | [Tuples](../../26_Collections/tuples/README.md) · [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) |
| References | [Borrowing](../../18_Ownership/borrowing/README.md) · [String slices](../../14_Strings/string_slices/README.md) · [`String` vs `&str`](../../14_Strings/string_vs_str/README.md) |
| User-Defined Types | [Structs](../../16_Structs/README.md) · [Enums](../../13_Enums/README.md) · [`const` and `static`](../../27_Modules/const_and_static/README.md) · [`Result` aliases](../../17_Option_and_Result/result_aliases/README.md) |
| **Day 2** Pattern Matching | [Pattern matching](../../30_Pattern_Matching/README.md) *(stubs)* · [`if let`](../../17_Option_and_Result/if_let/README.md) · [`while let`](../../17_Option_and_Result/while_let/README.md) |
| Methods and Traits | [`impl` blocks](../../16_Structs/impl_blocks/README.md) · [Traits](../../12_Traits/README.md) · [Supertraits](../../12_Traits/supertraits/README.md) |
| Generics | [Generics](../../22_Generics/README.md) |
| Closures | [Closures](../../23_Closures/README.md) |
| Standard Library Types | [`Option` and `Result`](../../17_Option_and_Result/README.md) · [Strings](../../14_Strings/README.md) · [`Vec`](../../26_Collections/the_vec/README.md) · [`HashMap`](../../26_Collections/the_hashmap/README.md) |
| Standard Library Traits | [Comparisons](../../12_Traits/comparison_traits/README.md) · [Operators are traits](../../12_Traits/operators_are_traits/README.md) · [`From` and `Into`](../../29_Conversion/from_and_into/README.md) · [Casting with `as`](../../29_Conversion/casting_with_as/README.md) · [`Read` and `Write`](../../12_Traits/read_and_write/README.md) · [The `Default` trait](../../03_Command_Line/the_default_trait/README.md) |
| **Day 3** Memory Management | [Ownership](../../18_Ownership/README.md) · [Stack and heap](../../18_Ownership/stack_and_heap/README.md) · [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) · [`Drop` and RAII](../../12_Traits/drop_and_raii/README.md) |
| Smart Pointers | [`Box`](../../26_Collections/the_box/README.md) · [`Rc`](../../18_Ownership/reference_counting/README.md) · [Returning a trait](../../12_Traits/returning_a_trait/README.md) |
| Borrowing | [Borrowing](../../18_Ownership/borrowing/README.md) · [Interior mutability](../../09_Advanced/interior_mutability/README.md) *(stub)* |
| Lifetimes | [Lifetime annotations](../../18_Ownership/lifetime_annotations/README.md) · [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) · [`&'static str`](../../14_Strings/static_str/README.md) |
| **Day 4** Iterators | [Iterators](../../24_Iterators/README.md) |
| Modules | [Modules](../../27_Modules/README.md) |
| Testing | [Testing](../../28_Testing/README.md) |
| Error Handling | [Errors](../../02_Errors/README.md) |
| Unsafe Rust | [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) · [What a union is](../../09_Advanced/what_a_union_is/README.md) |
| *Concurrency (extra day, morning)* | [Spawning a thread](../../09_Advanced/spawning_a_thread/README.md) · [Channels](../../09_Advanced/channels/README.md) · [`Send` and `Sync`](../../09_Advanced/send_and_sync/README.md) *(stub)* · [`RwLock` and atomics](../../09_Advanced/rwlock_and_atomics/README.md) *(stub)* · [Lock poisoning](../../09_Advanced/mutex_poisoning/README.md) · [`Arc`](../../18_Ownership/sharing_across_threads/README.md) |

## What each has that the other does not

**Only in Comprehensive Rust.** The four standalone classes — Android, Chromium, bare-metal `no_std`, and the **async** half of the concurrency day. The async gap is the significant one, and it is the same structural gap [Observability](../../21_Observability/README.md) has: every example in this library compiles with `rustc` alone and no crates, and async in practice means Tokio. A handful of smaller slides too — *Arithmetic*, and the *Playground*.

**Only here.** Everything past the fourth day: [tooling](../../05_Tooling/README.md), [the command line](../../03_Command_Line/README.md), [files](../../04_Files/README.md), [data](../../06_Data/README.md), [clients](../../07_Clients/README.md), [observability](../../21_Observability/README.md), [compilers](../../20_Compilers/README.md), and the [numbers-and-bytes](../../19_Numbers/README.md) layer under all of it. And a difference of kind rather than coverage: **every claim here is checked**. A number on a page came out of a program that CI recompiles, which a slide deck has no mechanism for and does not try to have.

## Reading the speaker notes

They are the reason to use the site rather than a PDF, and they are hidden by default. Open a slide and expand *Speaker Notes* at the bottom; there is also a separate notes window, opened with the icon in the top bar, meant for a second screen while presenting. Two examples of what is down there and not in the slide: that `1..5` stops at 4 and `1..=5` is how you say otherwise, and that an integer literal Rust has not pinned down yet prints as `{integer}` in error messages.

## See also

- [Start here](../README.md) — the three-resource plan this is a fourth door onto
- [Benefits of Rust](../benefits_of_rust/README.md) — the twenty selling points this course opens with, each weighed against its evidence
- [C and C++](../../31_C_and_Cpp/README.md) — the nine bugs those benefits are a reply to, compiled and run
- [Comprehensive Rust ↗](https://google.github.io/comprehensive-rust/) · [the repository ↗](https://github.com/google/comprehensive-rust) · [Course structure ↗](https://google.github.io/comprehensive-rust/running-the-course/course-structure.html)

## Po polsku

Kurs jest przetłumaczony na kilkanaście języków i **polska wersja istnieje**, tylko trzeba o niej wiedzieć: leży pod [`/pl/` ↗](https://google.github.io/comprehensive-rust/pl/), a w liście języków na stronie jej nie ma, więc nie da się do niej kliknąć — adres wpisuje się ręcznie. Powód jest widoczny po otwarciu: tłumaczenie jest w toku i miesza się zdanie po zdaniu. Na slajdzie „Własność” pierwsze dwa zdania są polskie, a trzecie — *We say that the variable owns the value* — już nie. Na slajdzie o przenoszeniu ten sam rysunek pamięci jest podpisany „Stack / Heap” przed przeniesieniem i „Stos / Sterta” po nim. Sprawdzone 30 sierpnia 2026; to się będzie zmieniać, ale kierunek jest jeden: tytuły i plan są po polsku dużo częściej niż treść slajdów.

Dla uczącego się to nie jest neutralne, bo strona w dwóch językach naraz jest **trudniejsza** niż strona po angielsku: przy każdym akapicie trzeba na nowo ustalać, czy „zakres” i `scope` to to samo pojęcie, czy dwa różne. Do tego terminologia polskiej wersji rozjeżdża się z tą, którą trzyma ta biblioteka i Tour of Rust — kurs mówi „zakres” tam, gdzie tutaj jest **zasięg**, „wiszące referencje” zamiast *zwisających*, i „przywiązania zmiennych” o wiązaniach. Praktycznie wychodzi najlepiej tak: polska wersja do **planu** — spisu treści, nazw bloków i czasów przy każdym slajdzie, gdzie tłumaczenie jest najgęstsze i najmniej ryzykowne — a same slajdy po angielsku.

Reszta wartości tego kursu jest po polsku nie do odtworzenia, i akurat to jest dobra wiadomość. Notatki prowadzącego (*speaker notes*) są tu najciekawszą częścią książki, a dla samouka są tym bliższe wykładowcy, im mniej ma kogo zapytać — i są domyślnie zwinięte, więc większość czytających nigdy ich nie otwiera. Czasy przy slajdach czytaj jako **proporcję**, nie obietnicę: pochodzą z prawdziwej sali, gdzie ktoś odpowiada na pytania na bieżąco, więc mówią, który temat jest na godzinę, a który na pięć minut — a nie ile zajmie ci samodzielne przejście przez niego w drugim języku.

**Szukaj po polsku:** kurs Rusta Google · Comprehensive Rust po polsku · notatki prowadzącego · `comprehensive rust speaker notes` · `comprehensive rust course structure`
