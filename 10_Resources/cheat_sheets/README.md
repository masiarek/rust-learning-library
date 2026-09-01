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

Its source is public — **[ralfbiedert/cheats.rs ↗](https://github.com/ralfbiedert/cheats.rs/)** — which is worth knowing for two reasons beyond filing a correction. **The licence is CC-BY** ([the legal page ↗](https://cheats.rs/legal) states it, with Rust snippets released CC-0 and a requirement that any reproduction change the *Operator* section and not appear endorsed by the author), so the diagrams may be reused with credit rather than only linked. And **there are no diagrams to download.** The whole page carries exactly two `<img>` tags — a GitHub ribbon and the Ferris logo — because every memory-layout figure on it, the byte boxes and the lifetime strips alike, is drawn in HTML and CSS. Reusing one means re-drawing it, not copying a file, which is why the figures in this library are [built as text](../../14_Strings/anatomy_of_a_string/README.md) instead: a box diagram in a fenced block diffs, greps, and reads the same in both themes.

## Chapter by chapter, against this library

cheats.rs is a lookup surface for the whole language, so it doubles as a map of what a Rust library ought to cover. Its chapters are on the left; where this library says the same thing at teaching length, the page is on the right. **The blanks are real** — they are the gaps, named rather than papered over, and they are the honest answer to "does this library cover Rust?".

| cheats.rs | Here |
|---|---|
| [Data Structures ↗](https://cheats.rs/#data-structures) | [Structs](../../16_Structs/README.md) · [Enums](../../13_Enums/README.md) · [Collections](../../26_Collections/README.md) |
| [References & Pointers ↗](https://cheats.rs/#references-pointers) | [Ownership](../../18_Ownership/README.md) |
| [Functions & Behavior ↗](https://cheats.rs/#functions-behavior) | [Traits](../../12_Traits/README.md) · [Closures](../../23_Closures/README.md) |
| [Control Flow ↗](https://cheats.rs/#control-flow) | [Control flow](../../25_Control_Flow/README.md) |
| [Organizing Code ↗](https://cheats.rs/#organizing-code) | [Modules](../../27_Modules/README.md) |
| [Type Aliases and Casts ↗](https://cheats.rs/#type-aliases-and-casts) | [Conversion](../../29_Conversion/README.md) |
| [Macros & Attributes ↗](https://cheats.rs/#macros-attributes) | [Macros](../../25_Control_Flow/macros/README.md) |
| [Pattern Matching ↗](https://cheats.rs/#pattern-matching) | [Pattern matching](../../30_Pattern_Matching/README.md) |
| [Generics & Constraints ↗](https://cheats.rs/#generics-constraints) | [Generics](../../22_Generics/README.md) |
| [Higher-Ranked Items ↗](https://cheats.rs/#higher-ranked-items) | — |
| [Strings & Chars ↗](https://cheats.rs/#strings-chars) | [Strings](../../14_Strings/README.md) |
| [Documentation ↗](https://cheats.rs/#documentation) | [Doc tests](../../28_Testing/doc_tests/README.md), which is the half that runs |
| [The Abstract Machine ↗](https://cheats.rs/#the-abstract-machine) | [Compilers](../../20_Compilers/README.md) |
| [Memory & Lifetimes ↗](https://cheats.rs/#memory-lifetimes) | [Borrowed state](../../18_Ownership/borrowed_state/README.md) · [What `&'a T` claims](../../18_Ownership/what_a_reference_claims/README.md) · [At the call site](../../18_Ownership/lifetimes_at_the_call_site/README.md) |
| [Basic Types ↗](https://cheats.rs/#basic-types) | [Numbers and bytes](../../19_Numbers/README.md) |
| [Custom Types ↗](https://cheats.rs/#custom-types) | [Structs](../../16_Structs/README.md) · [Enums](../../13_Enums/README.md) · [What a union is](../../09_Advanced/what_a_union_is/README.md) |
| [Pointer Meta ↗](https://cheats.rs/#pointer-meta) | [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) covers the vtable; the fat-pointer layout itself is not drawn here |
| [Standard Library Types ↗](https://cheats.rs/#standard-library-types) | [Collections](../../26_Collections/README.md) · [Strings](../../14_Strings/README.md) · [`Rc`](../../18_Ownership/reference_counting/README.md) |
| [One-Liners ↗](https://cheats.rs/#one-liners) | [row by row, below](#the-one-liners-row-by-row) — the one chapter small enough to check exhaustively |
| [Thread Safety ↗](https://cheats.rs/#thread-safety) | [`Send` and `Sync`](../../09_Advanced/send_and_sync/README.md) |
| [Atomics & Cache ↗](https://cheats.rs/#atomics-cache) | [`RwLock` and atomics](../../09_Advanced/rwlock_and_atomics/README.md) |
| [Iterators ↗](https://cheats.rs/#iterators) | [Iterators](../../24_Iterators/README.md) · [Adapters, by job](../../24_Iterators/adapters_by_job/README.md) |
| [Number Conversions ↗](https://cheats.rs/#number-conversions) | [Conversion](../../29_Conversion/README.md) · [Numbers and bytes](../../19_Numbers/README.md) |
| [String Conversions ↗](https://cheats.rs/#string-conversions) | [Making a `String`](../../14_Strings/making_a_string/README.md) |
| [String Output ↗](https://cheats.rs/#string-output) | [Strings](../../14_Strings/README.md) |
| [Project Anatomy ↗](https://cheats.rs/#project-anatomy) | [Scaffolding](../../05_Tooling/scaffolding/README.md) |
| [Cargo ↗](https://cheats.rs/#cargo) | [Tooling](../../05_Tooling/README.md) |
| [Cross Compilation ↗](https://cheats.rs/#cross-compilation) | [Targets and triples](../../20_Compilers/targets_and_triples/README.md) |
| [Tooling Directives ↗](https://cheats.rs/#tooling-directives) | [Strict lints](../../05_Tooling/strict_lints/README.md) |
| [Types, Traits, Generics ↗](https://cheats.rs/#types-traits-generics) | [Traits](../../12_Traits/README.md) · [Generics](../../22_Generics/README.md) |
| [Foreign Types and Traits ↗](https://cheats.rs/#foreign-types-and-traits) | [Traits](../../12_Traits/README.md), for the orphan rule |
| [Type Conversions ↗](https://cheats.rs/#type-conversions) | [Conversion](../../29_Conversion/README.md) |
| [Performance Tips ↗](https://cheats.rs/#performance-tips) | — |
| [Async-Await 101 ↗](https://cheats.rs/#async-await-101) | only at the edges: [`Iterator` vs `Stream`](../../24_Iterators/iterator_vs_stream/README.md) and [instrumenting async](../../21_Observability/instrumenting_async/README.md) |
| [Closures in APIs ↗](https://cheats.rs/#closures-in-apis) | [Function pointers](../../23_Closures/function_pointers/README.md) |
| [Unsafe, Unsound, Undefined ↗](https://cheats.rs/#unsafe-unsound-undefined) | [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) · [C and C++](../../31_C_and_Cpp/README.md) |
| [API Stability ↗](https://cheats.rs/#api-stability) | — |

Four blanks, and they are not the same kind of blank. **Higher-ranked items** (`for<'a>`) and **API stability** are missing because nothing here has needed them yet. **Performance tips** is missing on purpose so far: every claim on such a page wants a measurement, and this library has no benchmark harness, so writing one would mean asserting numbers it cannot check — the one thing [the answer-key rule](../../CONTRIBUTING.md) exists to prevent. **Async** is the largest, and it is a section rather than a page.


## The One-Liners, row by row

[One-Liners ↗](https://cheats.rs/#one-liners) is the odd chapter out on cheats.rs: five tabs of *snippets* rather than concepts, admitted under three rules its source states in a comment — `std` only, "most people" should have hit the problem, and it must not be a trivial method on an obvious struct. That makes it the nearest thing the sheet has to a syllabus for a library like this one, and at twenty-six rows it is small enough to check exhaustively rather than chapter by chapter.

**Fourteen rows land on a written lesson, four land on a stub, and eight have nothing.** The kata column is not decoration. A row with a page but no `## Practice` is one you can read and still not write from memory afterwards — which, for a chapter whose subject is *snippets you forget*, is the whole failure mode.

### Strings — eight of nine, two with a kata

| Snippet | Here | Kata |
|---|---|---|
| `format!("{x}{y}")` | [Concatenating strings](../../14_Strings/concatenating_strings/README.md), with the `{x}` capture itself in [The braces take a name](../../15_First_Programs/braces_take_a_name/README.md). [The format mini-language](../../14_Strings/the_format_language/README.md) — the `{:>8}` / `{:.3}` half — is still a stub | ✓ |
| `write!(x, "{y}")` | [Building a `String`](../../14_Strings/building_a_string/README.md); the [`Write` trait](../../12_Traits/read_and_write/README.md) under it is a stub | ✓ |
| `s.split(pattern)` | [`str::split`](../../14_Strings/str_methods/str_split/README.md), and [Inside a `Split`](../../14_Strings/inside_a_split/README.md) for why `{:?}` prints a struct instead of the pieces | — |
| … with `&str` | [`str` methods](../../14_Strings/str_methods/README.md) — one `Pattern` table for all four shapes, each with a compiled page behind it | — |
| … with `char` | *(same table)* | — |
| … with closure | *(same table)* | — |
| `s.split_whitespace()` | [`str::split_whitespace`](../../14_Strings/str_methods/str_split_whitespace/README.md) | — |
| `s.lines()` | [`str::lines`](../../14_Strings/str_methods/str_lines/README.md) | — |
| `Regex::new(r"\s")?.split(…)` | — | — |

The regex blank is structural rather than an oversight. [`run_examples.py`](../../tools/run_examples.py) builds each example with a bare `rustc --edition 2024` — no `--extern`, no cargo — so an example can use nothing outside `std`. That is the same rule cheats.rs applies to this chapter, and the reason it flags that one row as needing a crate.

The six blanks in the kata column are the honest finding here. The [`str`](../../14_Strings/str_methods/README.md) and [`String`](../../14_Strings/string_methods/README.md) method pages are 125 reference pages with a compiled program each and **no exercise anywhere in the set** — deliberate, in that a reference page answers a question you already have, but it does mean the split family is a set of pages you consult rather than a skill you own.

### I/O — one of three

| Snippet | Here | Kata |
|---|---|---|
| `File::create(PATH)?` | [Opening a file](../../04_Files/opening_a_file/README.md) — a **stub** | — |
| `OpenOptions::new()…open(PATH)?` | *(same stub)* | — |
| `read_to_string(path)?` | [Reading lines efficiently](../../04_Files/reading_lines_efficiently/README.md), which prices it against `lines()` and a reused buffer | — |

[Files](../../04_Files/README.md) is six pages, five of them stubs, and no kata in the section.

### Macros — none of two

Both rows are `macro_rules!` with variable arguments (`$($args:expr),*` and expanding it with `$( f($args); )*`), and [Macros](../../25_Control_Flow/macros/README.md) is a stub that explains what the `!` means and stops. **Nothing in this library writes a macro.** That is the largest single gap the chapter exposes; it belongs in [Control flow](../../25_Control_Flow/README.md), where eight of nine pages are stubs.

### Transforms — all five, and the strongest corner

These rows are not snippets; each points at another sheet for *"everything you can turn this type into"*. The destinations are where this library goes deepest.

| Starting type | Here |
|---|---|
| `Option<T> -> …` | [`Option` and `Result`](../../17_Option_and_Result/README.md) — 27 pages, 23 of them carrying a kata |
| `Result<T, R> -> …` | *(same section)*, plus [Errors](../../02_Errors/README.md) |
| `Iterator<Item=T> -> …` | [Adapters, by job](../../24_Iterators/adapters_by_job/README.md) |
| `&[T] -> …` | [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) · [`Vec` methods](../../26_Collections/vec_methods/README.md) |
| `Future<T> -> …` | [`Iterator` vs `Stream`](../../24_Iterators/iterator_vs_stream/README.md) alone — the async blank named above |

### Esoterics — none of seven

| Snippet | Where it would go |
|---|---|
| <code>wants_closure({ let c = outer.clone(); move &#124;&#124; use_clone(c) })</code> | [Closures](../../23_Closures/README.md) · [The `move` keyword](../../23_Closures/the_move_keyword/README.md) |
| <code>iter.try_for_each(&#124;x&#124; { Ok::&lt;(), Error&gt;(()) })?</code> | [Errors](../../02_Errors/README.md) · [Iterators](../../24_Iterators/README.md) |
| `Cell::from_mut(mut_slice).as_slice_of_cells()` | [Interior mutability](../../09_Advanced/interior_mutability/README.md) |
| `&original_slice[offset..][..length]` | [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) |
| `const _: Option<&dyn T> = None;` | [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md), which covers dyn compatibility but not this canary for it |
| the *semver trick* | [Cargo dependencies](../../05_Tooling/cargo_dependencies/README.md) |
| `pub(crate) use internal_macro;` | [The `use` declaration](../../27_Modules/the_use_declaration/README.md) |

Seven rows, seven pages that already exist, and a paragraph each would close the tab. It is the cheapest gap on this list, and — by the rule cheats.rs uses to admit a row at all — the one a reader is least likely to find anywhere else.

### And the Cookbook it points to

The chapter's own header sends you to the **[Rust Cookbook ↗](https://rust-lang-nursery.github.io/rust-cookbook/)** for more, and that link is worth following with the `std`-only rule in mind, because it is the mirror image of this library. Its recipes are organised by *task* — compression, cryptography, databases, images, HTTP, parsing with `nom` — and nearly every one is a crate call. The overlap with what can be built here is the handful of chapters that stay inside `std`: text processing, some of the file-system and command-line recipes, error handling, and the concurrency primitives. Everything else is a reason the sections that name a crate — [Data](../../06_Data/README.md), [Clients](../../07_Clients/README.md), [Observability](../../21_Observability/README.md) — are stubs here and cannot graduate the usual way: the mechanism gets modelled in `std`, where the check can see it, and the crate's real API stays a link to the crate's own versioned docs.

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
