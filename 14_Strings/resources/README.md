# Strings: links, books and videos

**Level:** reference · the reading list

**One line:** Where the [strings section](../README.md) got its material, and where to go when a lesson here is not enough — the normative sources first, then the book chapters worth owning, then the essays, the videos, and the exercises to actually type.

**The route through the lessons is [STRINGS.md](../../STRINGS.md).** This page is the outside world; that one is the order to read this library in.

---

## Official

- [The Book, ch. 8.2 — Storing UTF-8 encoded text with strings ↗](https://doc.rust-lang.org/book/ch08-02-strings.html) — the canonical chapter. `String::new`, `push_str`, `+` and `format!`, why indexing is refused, and the bytes/scalars/graphemes split. If you read one thing, read this.
- [The Book, ch. 4.1 — What is ownership? ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) — `String` as the worked example for ownership itself, which is why the moves and the text arrive together. The `E0382` here is the same one [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) prints in full.
- [The Book, ch. 4.3 — The slice type ↗](https://doc.rust-lang.org/book/ch04-03-slices.html) — where `&str` stops being "the string parameter type" and becomes a view with a start and an end.
- [The Reference — textual types ↗](https://doc.rust-lang.org/reference/types/textual.html) — the normative definition of `char` and `str`. Terse, and the one that is *binding* when two explanations disagree.
- [`std::string::String` ↗](https://doc.rust-lang.org/std/string/struct.String.html) · [`str` ↗](https://doc.rust-lang.org/std/primitive.str.html) — the method lists. Skim them once end to end; half of what people write by hand is already there.
- [`std::fmt` ↗](https://doc.rust-lang.org/std/fmt/) — the format mini-language: fill, align, sign, width, precision, and the `$` that makes any of them dynamic. The one std page whose *contents* are a syntax nobody guesses.
- [Rust by Example — strings ↗](https://doc.rust-lang.org/rust-by-example/std/str.html) — the smallest runnable version of the idea, including the literal escapes and byte strings.

## Book chapters

- **Programming Rust**, 2nd ed. (Blandy, Orendorff & Tindall) — **ch. 17, "Strings and Text"**. The best single treatment in print: `char`, `String`/`str`, the formatting machinery and regex, with the Unicode explained rather than waved at. Read it after the Book's ch. 8.
- **Easy Rust** (Dave MacLeod) — [**ch. 14, "Strings"** ↗](https://dhghomon.github.io/easy_rust/Chapter_14.html). The gentlest version: why `&str` needs the `&` at all, and the four ways to build a `String` (`String::from`, `.to_string()`, `format!`, `.into()`) laid out side by side. Free, and short enough to read in a sitting — the [video below](#video) is the spoken companion. One thing to read carefully: its size demonstration measures `size_of_val` on the *text*, which varies with content; `size_of::<&str>()` is always 16 bytes, as [`ToOwned`](../../12_Traits/to_owned/README.md) prints.
- **Effective Rust** (David Drysdale) — [**Item 5, "Understand type conversions"** ↗](https://www.lurklurk.org/effective-rust/casts.html). Not a strings chapter, but the one that settles `From` / `Into` / `TryFrom`, which is the machinery under `String::from` and `.into()`. Free online.
- **Code Like a Pro in Rust** (Brenden Matthews, Manning 2024) — **no strings chapter.** Worth saying out loud so nobody goes looking: its arc is Cargo, tooling, data structures, memory, testing, async and optimization. Ch. 4–5 are the nearest thing, and they are about layout and allocation rather than text.

## Essays and write-ups

- [Working with strings in Rust ↗](https://fasterthanli.me/articles/working-with-strings-in-rust) — Amos on why `String` and `&str` are two types and what every other language quietly got away with. The piece to send someone who thinks Rust is being difficult on purpose.
- [Rust Language Cheat Sheet — string conversions ↗](https://cheats.rs/#string-conversions) — the whole conversion matrix on one screen: every source type, every target, the call that gets you there. Print it.
- [UTF-8 Everywhere ↗](https://utf8everywhere.org/) — the manifesto behind the encoding decision Rust made for you. Long, opinionated, and the reason `String` is a `Vec<u8>` with a promise rather than an array of characters.
- [`unicode-segmentation` ↗](https://docs.rs/unicode-segmentation/) — the crate for the third answer to "how long is it". Graphemes are not in `std`, deliberately, and this is where they live.

## Q&A

The one thread worth reading rather than searching:

- [What are the differences between Rust's `String` and `str`? ↗](https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str) — asked 2014, 240k views, and still the best single page on the question. huon's top answer is the one to read: `str` is bytes *somewhere*, and it enumerates the three somewheres (static storage, inside a `String`, a stack array) — which is where [`String` vs `&str`](../string_vs_str/README.md) gets its three-ways-to-have-text framing. Read the **footnotes and the comments**, not just the answers: the thread's most-repeated error is "`str` is immutable", corrected twice in the comments by Chayim Friedman — a `str` is fixed-**length**, and `&mut str` can be mutated in place. Two other answers are worth the scroll (snnsnn's long one on the data section and `struct String { vec: Vec<u8> }`; Zorf's on fat pointers and `Box<str>`), and one 42-vote answer says a `&str`'s length field counts *characters* — it counts **bytes**, as [Meet the `char`](../meet_the_char/README.md) prints.
- [Understanding when to use `String` vs `str` ↗](https://users.rust-lang.org/t/understanding-when-to-use-string-vs-str/103746) — the users forum, 2023, and much shorter. Skip it for the definitions and take the one refinement it adds to "parameters take `&str`": scottmcm's rule that a parameter should become `String` **when the function always calls `.to_owned()` on it anyway** — at which point taking the borrow only forces the caller to allocate a copy you were going to make regardless.
- [`String` vs `str`, why? ↗](https://users.rust-lang.org/t/string-vs-str-why/61334) — the users forum, 2021, a newcomer asking why there are two types at all and whether `&str` lives on the stack. Read it for two things the answers add to the definitions: steffahn's suggestion to read `String` as `StringBuf` (the name std already uses for `PathBuf`/`Path` and `OsString`/`OsStr`, which makes `&str` the same kind of thing as `&[T]`), and scottmcm's point that garbage-collected languages keep the same split — Python's `str` against `io.StringIO`, Java's `StringBuilder` — Rust just writes it into the type. Both are now on [`String` vs `&str`](../string_vs_str/README.md#the-names-hide-the-pattern). A forum thread, so weigh it as one; the memory claims in the question are the ones the answers correct.

## Video

- [Easy Rust 013: String and `&str` ↗](https://youtu.be/pSyaGzGg26o) — Dave MacLeod, the spoken companion to [ch. 14 ↗](https://dhghomon.github.io/easy_rust/Chapter_14.html) above. Short, and it takes the *why does this need a `&`* question seriously instead of asserting the answer.
- [When to use `String` and `&str`? ↗](https://youtu.be/GrIudQW_i_U) — Smart Contract Programmer, ~8½ min, and the one that walks **all six forms in order** instead of the usual two: [0:39 ↗](https://www.youtube.com/watch?v=GrIudQW_i_U&t=39s) `String` · [1:49 ↗](https://www.youtube.com/watch?v=GrIudQW_i_U&t=109s) `&String` · [3:01 ↗](https://www.youtube.com/watch?v=GrIudQW_i_U&t=181s) `&mut String` · [3:59 ↗](https://www.youtube.com/watch?v=GrIudQW_i_U&t=239s) `str` · [5:11 ↗](https://www.youtube.com/watch?v=GrIudQW_i_U&t=311s) `&str` · [6:03 ↗](https://www.youtube.com/watch?v=GrIudQW_i_U&t=363s) `&mut str` · [6:42 ↗](https://www.youtube.com/watch?v=GrIudQW_i_U&t=402s) why `str` works as neither a parameter nor a return type · [8:06 ↗](https://www.youtube.com/watch?v=GrIudQW_i_U&t=486s) the when-to-use-which summary. Its [code is published ↗](https://github.com/t4sk/hello-rust/blob/main/src/bin/borrow_string_str.rs), which is the reason to reach for it over a longer video: the unsized argument arrives as something you can compile rather than a sentence — `let a: str = "hello";` beside `let b: str = "hello rust";`, two lengths wanting one type, which is [`str` is unsized](../str_is_unsized/README.md) in two lines. **Two things it leaves half-done.** It reaches `&mut str`, calls it *“possible but uncommon”*, and never mutates one — so the half worth seeing goes missing: a `&mut str` really can rewrite bytes in place ([`make_ascii_uppercase`](../str_methods/str_make_ascii_uppercase/README.md), reached through [`as_mut_str`](../string_methods/string_as_mut_str/README.md)), it just cannot change how many there are. And the published code files `fn make_str() -> &str` under the comment *“reference outlives value s”*, which is the wrong diagnosis: `let s = "";` is a `&'static str` and outlives nothing, the error is **`E0106`, missing lifetime specifier**, and it is the *signature* at fault, not the body — write `-> &'static str` and that same body compiles and runs (checked on 1.98.0). `E0106` in full is in [`String` vs `&str`](../string_vs_str/README.md).

## Exercises worth typing

### Here

Every lesson in the strings arc carries one, and they are ordered to be attempted in the map's order — the full table with levels is [KATAS.md](../../KATAS.md).

| The lesson | What its kata makes you do |
|---|---|
| [`String` vs `&str`](../string_vs_str/README.md) | One `&str` parameter, three callers — then flip it to `String` and price what every call site now pays |
| [String slices](../string_slices/README.md) | Cut a name in half without panicking: `len()/2` on four names, and two ways to find a legal boundary |
| [The anatomy of a `String`](../anatomy_of_a_string/README.md) | Predict `len` and `capacity` through five pushes — which ones reallocate? |
| [Making a `String`](../making_a_string/README.md) | Implement `Display` once, collect four abilities; then let the source pick the spelling across six conversions |
| [Concatenating strings](../concatenating_strings/README.md) | One greeting three ways, then earn `E0369`, `E0308` and `E0368` on purpose |
| [Building a `String`](../building_a_string/README.md) | One line built four ways — how many buffers, and which one is wrong inside a loop |
| [Meet the `char`](../meet_the_char/README.md) | One name, three lengths — and the combining accent that makes two identical-looking strings unequal |
| [Walking a `String`](../walking_a_string/README.md) | An empty field is data: parse `"5,,0"`, then watch `split_whitespace()` shorten the row |
| [`&'static str`](../static_str/README.md) | Return a label three ways — read the `E0515`, then match, leak, or own it |
| [Six kinds of string](../six_kinds_of_string/README.md) | Three arrivals, three types — then break the UTF-8 and NUL promises on purpose |
| [`ToOwned`](../../12_Traits/to_owned/README.md) | Predict the owned twin for six receivers before you run it |
| [`Cow`](../../18_Ownership/clone_on_write/README.md) | Return a `Cow` and prove the untouched rows were never copied |

### Elsewhere

- [rustlings ↗](https://github.com/rust-lang/rustlings) — `strings1`/`strings2` are the smallest possible version of the `&str`-vs-`String` question, and the `conversions` set (`from_into`, `from_str`, `as_ref_mut`, `try_from_into`, `using_as`) is the machinery under [Making a `String`](../making_a_string/README.md). Do the `conversions` set right after that lesson; it is the only external exercise here that maps one-to-one onto a page.
- [Exercism — Rust track ↗](https://exercism.org/tracks/rust) — mentored, and heavy on text problems early on ("Reverse String" is the one that teaches graphemes the hard way).
- [practice.rs — strings ↗](https://practice.rs/compound-types/string.html) — fill-in-the-blank against a compiler, in the same house style as Rust by Example.
- [100 Exercises — String slices ↗](https://rust-exercises.com/100-exercises/04_traits/06_str_slice) — the chapter that draws it: stack-and-heap diagrams for `String`, `&String` and `&str` side by side, then `&s[1..]` as two words pointing into somebody else's buffer. The whole course is [in the exercises hub](../../10_Resources/exercises/README.md); this is the one chapter worth reading out of order, and the diagrams are the reason.

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order, plus the topics still missing
- [Six kinds of string](../six_kinds_of_string/README.md) — where to go when the text is a path, an OS handle, or bound for C
- [GLOSSARY.md](../../GLOSSARY.md) — string slice, deref coercion, capacity, Unicode scalar value, grapheme cluster
- [Traits: links and videos](../../12_Traits/resources/README.md) — the same page for the other section this arc leans on

## Po polsku

Ta strona jest listą lektur i dla polskiego czytelnika ma jedną niewygodną własność: wszystko na niej jest po angielsku, i nie jest to przeoczenie. Tour of Rust, jedyny obszerny polski kurs Rusta, kończy tłumaczenie na rozdziale 5 — czyli dokładnie przed rozdziałem o tekście. Nie istnieje więc polski odpowiednik rozdziału 8.2 Księgi ani rozdziału 17 *Programming Rust*: ten dział czyta się po angielsku, a polszczyzna służy tu do nazywania rzeczy, nie do ich tłumaczenia. Dlatego wiersz „Szukaj po polsku” poniżej zawiera głównie frazy angielskie — to nie kapitulacja, tylko wskazanie, gdzie naprawdę są odpowiedzi.

Zanim zaczniesz szukać, warto wiedzieć, że po polsku ta sama rzecz nazywa się na trzy sposoby. **Łańcuch znaków** jest formalny i tego trzyma się ta biblioteka. **Napis** to słowo ze starszej polskiej literatury algorytmicznej — jeśli szukasz „algorytmy na napisach”, trafisz na dobre teksty o przetwarzaniu tekstu, tyle że nie o Ruscie. **String** bez tłumaczenia jest w praktyce najczęstsze w polskich rozmowach programistów i to ono najlepiej działa w wyszukiwarce razem ze słowem `rust`. Termin `&str` nie ma polskiej nazwy w obiegu poza tą, której używa ta biblioteka: **wycinek łańcucha** (*string slice*).

Jest też powód, dla którego akurat te lektury polski czytelnik powinien czytać uważniej, a nie pobieżnie. Klasyczny przykład z Księgi — dlaczego indeksowanie `s[0]` jest zabronione — pokazywany jest na cyrylicy albo dewanagari i łatwo go odczytać jako ciekawostkę o egzotycznych alfabetach. Po polsku to przypadek domyślny: `"żółw".len()` daje **7**, `.chars().count()` daje **4**, a `&"żółw"[0..1]` panikuje w środku znaku. Dochodzi do tego rzecz, której angielskojęzyczne materiały prawie nie poruszają — normalizacja: `ó` może przyjechać jako jeden znak U+00F3 albo jako `o` plus łączący akut U+0301, i te dwa łańcuchy wyglądają identycznie, a `==` zwraca `false`. Nazwy plików z macOS przychodzą w tej drugiej postaci. To dlatego pozycje o Unicode z listy powyżej — rozdział 17 *Programming Rust* i crate `unicode-segmentation` — są dla nas mniej opcjonalne niż dla autora oryginału.

**Szukaj po polsku:** łańcuch znaków w Ruscie · napisy i kodowanie UTF-8 · normalizacja Unicode NFC NFD · `rust String vs &str` · `rust string index utf8` · `rust unicode-segmentation graphemes`
