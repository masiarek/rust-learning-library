# Cheat sheets

**Level:** reference · the shelf

**One line:** One of these is maintained and the rest are frozen — which is harmless for memory layout, since the diagrams have not changed, and disqualifying for an iterator list, which has missed eight years of adapters.

**Every link checked 2026-08-30** with a full GET rather than a `HEAD`, because a bot-blocked `403` answers a `HEAD` as though it were fine. The one dead entry is named below rather than quietly dropped.

A cheat sheet earns its place by answering *"what is this called?"* faster than search does. That is a narrow job, and the failure mode is specific: a frozen sheet does not look frozen, so you go on trusting a table that stopped being complete years ago.

## The one to keep open

**[cheats.rs ↗](https://cheats.rs/)** — **Reference.** Maintained, enormous, and the only one on this page that is current. The language, the standard library, and the memory-layout diagrams nothing else has. Two deep links worth bookmarking on their own:

| Section | Why |
|---|---|
| [Standard library types ↗](https://cheats.rs/#standard-library-types) | every container drawn — the `ptr` / `len` / `capacity` boxes over a heap allocation, with subscripts giving the size on 16-, 32- and 64-bit targets |
| [String conversions ↗](https://cheats.rs/#string-conversions) | the whole matrix — every source type, every target, the call that gets you there |

It also ships as a printable PDF, which is the form most people actually want: **[A4 ↗](https://cheats.rs/dl/rust_cheat_sheet_a4.pdf)** · **[Letter ↗](https://cheats.rs/dl/rust_cheat_sheet_letter.pdf)**. The links inside the PDF still work.

## Memory layout, on one page

Both of these are frozen and both are still correct, because what they draw has not moved.

**[Rust container cheat sheet ↗](https://johnbsmith.github.io/Informatik/Rust/Dateien/Rust-container-cheat-sheet.pdf)** — **Reference.** `Box<T>`, `Vec<T>`, `Box<[T]>`, `Box<Trait>`, `&T`, `&[T]`, `Rc`, `Arc`, `Mutex`, `Cell`, `RefCell` and an enum, all drawn to the same scale on one sheet, with the conversions between them as arrows. Its own credit line reads *"by Raph Levien, Copyright 2017 Google Inc., released under Creative Commons BY, 2017-04-21, version 0.0.3"*; the [Google Slides original ↗](https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit) is still up. One caution that applies to every diagram of this kind: it draws `Vec` as `ptr, cap, len` and cheats.rs draws it as `ptr, len, capacity`, and **neither is wrong**, because the field order of `Vec` is [explicitly not guaranteed ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees). Read them as *what a value is made of*, never as a layout you can rely on.

**[Rust Memory Container Cheat-sheet ↗](https://github.com/usagi/rust-memory-container-cs)** — **Reference.** The same idea by Usagi Ito, MIT-licensed, and the one to print: SVG plus PNG at 3840×2160 down to 1280×720, in nine variants including dark, light and monochrome. Denser than Levien's and less good at the arrows; better on a wall.

## Frozen, and worth knowing why

**[Rust Iterator Cheat Sheet ↗](https://danielkeep.github.io/itercheat_baked.html)** — **Dated.** Its own header says *"Last updated: Rust 1.17.0, Itertools 0.6.0"* — that is 2017. The organising idea is still the best one going: it groups adapters by *what you want to do*, with a one-line summary each, and it covers `std` and `itertools` side by side. What it cannot do is tell you about anything added since, and the misses are ordinary methods you will reach for — `step_by`, `try_fold`, `copied`, `reduce`, `map_while` and `is_sorted` are all absent. Use it to find the *shape* of the method you want, then confirm the name in [`Iterator` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html). Or read [Adapters, by job](../../24_Iterators/adapters_by_job/README.md), which is the same idea kept current and with the output proved.

**[phaiax/rust-cheatsheet ↗](https://phaiax.github.io/rust-cheatsheet/)** — **Dated.** Method signatures for `Vec`, `HashMap`, `Option` and `Result` grouped by task — accessing, adding, removing — pulled from the std docs of its day. It states no version; the examples inside are marked 1.4 to 1.7, which places it in 2016. `std`'s own docs now do this job better and are current, so this is here for completeness rather than as a recommendation.

## The cheat sheets that ship with the language

Three official pages that are cheat sheets in everything but name, and the only ones that cannot go stale:

| Page | Answers |
|---|---|
| [`std::collections` — *"Which collection should I use?"* ↗](https://doc.rust-lang.org/std/collections/index.html#when-should-you-use-which-collection) | the choice between `Vec`, `VecDeque`, `HashMap`, `BTreeMap`, `HashSet` and the rest, with the cost table underneath it |
| [`std::fmt` ↗](https://doc.rust-lang.org/std/fmt/) | the formatting mini-language — `{:>8}`, `{:.3}`, `{:#x}`, `{:#?}` — which nobody remembers and everybody re-looks-up |
| [Rust API Guidelines checklist ↗](https://rust-lang.github.io/api-guidelines/checklist.html) | a page of one-line rules to read down before publishing a crate |

## Dead

**Periodic Table of Rust Types** (`cosmic.mearie.org`, Kang Seonghoon, 2014) — the host does not answer at all, over either scheme, as of the date at the top of this page. It is named here because a shelf that only lists what works cannot be trusted about anything else. It was a good sheet: values, references, raw pointers and smart pointers arranged as a grid by ownership and mutability.

## The ones already in this library

Not cheat sheets by name, but the same job with a program behind the numbers:

- [Adapters, by job](../../24_Iterators/adapters_by_job/README.md) — twenty-odd iterator adapters, grouped by what you are trying to do
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — the conversion table between the two, and the picture of it
- [Collections — *"Which one"*](../../26_Collections/README.md) — the same question `std` asks, answered in one table
- [Six kinds of string](../../14_Strings/six_kinds_of_string/README.md) — `String`, `&str`, `OsString`, `&OsStr`, `CString`, `&CStr`: three promises about the bytes, each owned and borrowed

---

## See also

- [Books](../books/README.md) — the reading shelf, where cheats.rs also appears under *On the desk*
- [Official docs](../official_docs/README.md) — when the question has a normative answer rather than a quick one
- [Resources](../README.md) — the map of all five shelves, organised by the moment you are in

## Po polsku

Ściągawka (*cheat sheet*) ma jedno zadanie: odpowiedzieć na pytanie „jak to się nazywa?” szybciej niż wyszukiwarka. Kłopot, o którym jest ta strona, polega na tym, że **zamrożona ściągawka nie wygląda na zamrożoną** — tabela sprzed ośmiu lat wygląda dokładnie tak samo jak tabela sprzed tygodnia. Dla rysunku pamięci (`Box`, `Vec`, `Rc`) to nie szkodzi, bo to, co rysuje, się nie ruszyło; dla listy adapterów iteratora to dyskwalifikacja — arkusz Daniela Keepa stanął na Ruscie 1.17 i nie zna ani `step_by`, ani `copied`, ani `reduce`, ani `map_while`. Stąd nawyk wart wyrobienia: najpierw poszukaj daty albo numeru wersji na górze arkusza, a nazwę metody potwierdź w dokumentacji `std`, która jako jedyna nie może się zdezaktualizować.

Polskiego czytelnika ta pułapka dotyczy mocniej niż angielskiego, i to nie z powodu języka, tylko objętości: materiałów o Ruscie po polsku jest niewiele i prawie żaden nie jest utrzymywany — Tour of Rust po polsku urywa się na rozdziale 5, a spora część polskich wpisów blogowych opisuje jeszcze stan sprzed edycji 2018. Dlatego dobry podział pracy wygląda tak: **po polsku szukaj wyjaśnień** (dlaczego coś działa właśnie tak), **po angielsku szukaj nazw** (jak się to nazywa i jak się to dziś pisze). Ściągawka jest ze swojej natury spisem nazw, więc niech będzie angielska i aktualna — `cheats.rs` plus `std`.

Jedna rzecz, której nie należy czytać dosłownie z żadnego rysunku: dwie ściągawki z tej strony rysują to samo `Vec` w różnej kolejności pól (`ptr, cap, len` u Leviena, `ptr, len, capacity` na cheats.rs) i **żadna nie jest błędna**, bo kolejność pól w `Vec` nie jest przez Rusta gwarantowana. Taki rysunek to odpowiedź na pytanie „z czego składa się ta wartość”, a nie układ bajtów, na którym można polegać — od gwarantowanego układu jest `#[repr(C)]`, którego `Vec` nie ma.

**Szukaj po polsku:** ściągawka Rust · rozmieszczenie w pamięci · `rust cheat sheet` · `cheats.rs` · `rust std collections which to use`
