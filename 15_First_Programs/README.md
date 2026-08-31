# First programs

**One line:** Before any of the language itself, four things have to work — getting a file to run at all, reading what the compiler says back about it, the punctuation every later page uses without explaining, and getting the program to tell you something. Then the language starts, with the three pages every line of Rust rests on: naming a value, what you may name, and who decided its type.

The first five pages are not Rust *features* at all. They are the loop you will be inside for every other page in the library: write it, run it, read the message, print something to find out what happened. The pages are short and the traps are the interesting half — a `///` that is not a comment, a `_name` and a `_` that mean different things, an extra semicolon that is `E0308`, and `{n + 1}` in a format string, which does not do what a Python f-string does.

The language itself begins at [variables](variables/README.md), and the three pages there answer one question between them: what is on each side of a `let`, and who chose the type in the middle.

## Never written any Rust at all?

These pages start at the compiler rather than at the language, and they assume you have at least *seen* Rust — so if you have not, an hour in one of the three below first will make everything after it read much less tersely. They are three different shapes and any one of them is enough; none of them is a course you have to finish.

| | What it is | Why this one |
|---|---|---|
| [Tour of Rust ↗](https://tourofrust.com/TOC_en.html) | ten short chapters, explanation on one side of the page and an editable, runnable pane on the other | nothing to install — you change the code in the page and watch what breaks |
| [Rust in Easy English ↗](https://dhghomon.github.io/easy_rust/Chapter_3.html) | sixty-odd concepts in deliberately restricted English, one example each | written for readers whose first language is not English, and the shortest route to *recognising* the syntax below |
| [Rust for the Polyglot Programmer ↗](https://www.chiark.greenend.org.uk/~ianmdlvl/rust-polyglot/index.html) | the delta, for someone who already knows three languages | spends no pages on what programming is and all of them on what is different here |

Then the three that stay open in a tab forever — [The Book](../00_Start_Here/the_book/README.md), [Rust by Example](../00_Start_Here/rust_by_example/README.md) and [the standard library ↗](https://doc.rust-lang.org/std/). [Start here](../00_Start_Here/README.md) is the argument for the order to read them in, [the shelf](../10_Resources/books/README.md) carries a verdict on each, and [ImplFerris's roadmap ↗](https://github.com/ImplFerris/LearnRust) is there for anyone who would rather browse a list than be told.

## The lessons

| Lesson | Level | What it teaches |
|---|---|---|
| [Running a scratch program](rustc_without_cargo/README.md) | 101 | `rustc` alone, `cargo new`, and `src/bin/` — rustc's edition-2015 default, and what Cargo was quietly doing for you |
| [Comments that compile](comments_that_compile/README.md) | 101 → 201 | Four forms, and two of them are not comments — `///` is `#[doc = "..."]`, a misplaced one is a *warning* rather than an error, and the examples inside are compiled and run as tests |
| [What a warning is asking](what_a_warning_is_asking/README.md) | 101 → 201 | A warning is the compiler asking whether you meant it — `rustc` raises it itself, `_name` and `_` are different answers, and only one of them leaves your value alive to the end of the scope |
| [A block is an expression](a_block_is_an_expression/README.md) | 101 → 201 | `{ }` does two jobs, and the second is the surprise: it *has a value* — its last line without a semicolon. Why a function body needs no `return`, and why one extra character is `E0308` |
| [The braces take a name](braces_take_a_name/README.md) | 101 → 201 | `{n}` in a format string is an **identifier**, not a Python f-string — `{n + 1}`, `{v.len()}` and `{self.voter}` are three different compile errors, and only one of them is diagnosed as what it is |
| [Variables](variables/README.md) | 101 | `let` binds a name and the binding is **immutable by default** — `mut` is a promise to the reader, `E0384` is what happens without it, and `let` twice is a different mechanism again |
| [Values](values/README.md) | 101 → 201 | The dozen-odd built-in types, their literal syntax and their widths — `usize` is a pointer, `char` is four bytes, `bool` is one, and a width is a promise the compiler checks |
| [Meet the `bool`](meet_the_bool/README.md) | 101 → 201 | One bit of information in one byte, and **not a number** — there is no truthiness anywhere, `1u8 as bool` is refused with a fix attached, and `&` compiles beside `&&` while quietly evaluating the side the guard was protecting you from |
| [The unit type `()`](the_unit_type/README.md) | 101 → 201 | One value, zero bytes: what a function with no `->` returns, what a `;` leaves behind, why `Ok(())` reads oddly and says something precise — and why `let x = v.sort();` gives you the receipt instead of the result |
| [Type inference](type_inference/README.md) | 101 → 201 | `let x = 10;` is not "any type" — it is one type, decided by a **later** line, compiling to a byte-for-byte identical binary; plus `{integer}`, `{float}`, and where inference stops |
| [What a type annotation does](what_an_annotation_does/README.md) | 101 → 201 | `let s = "a";` and `let s: &str = "a";` are the same program — but an annotation is an *input* to inference, not a comment: it picks `u8` over the `i32` fallback, coerces `&String` to `&str`, and is what `parse` and `collect` have no answer without |
| [Debug and Display](debug_vs_display/README.md) | 101 → 201 | Two printing traits, two audiences: `{:?}` can be derived because it is structural and `{}` never will be — plus the compiler note that spreads the habit, and the four default paths that print your error's `Debug` form instead of the sentence you wrote |
| [What `dbg!` does](what_dbg_does/README.md) | 101 → 201 | Why it is not a shorter `println!("{:?}")` — it returns your value, captures the expression source, writes to stderr, always pretty-prints, moves a non-`Copy` argument, and survives `--release` |
| [Randomness](randomness/README.md) | 101 → 201 | `std` has no generator, so the Rust Book's guessing game no longer compiles — `thread_rng`/`gen_range` are gone, the trait to import is `RngExt`, and `% n` is biased for a reason no better generator can fix |

## Where it goes next

[Control flow](../25_Control_Flow/README.md) is the direct sequel: `if`, `match` and the three loops, all of which are expressions for the reason [a block is an expression](a_block_is_an_expression/README.md) gives. After that comes [Structs](../16_Structs/README.md) — the first type you write yourself — where the compiler's *errors* get their own treatment: [when a struct refuses](../16_Structs/when_a_struct_refuses/README.md) reads eight of them the same way this section reads a warning.

## Po polsku

Ta sekcja nie uczy jeszcze języka — uczy **pętli**, w której siedzi się potem przy każdej następnej stronie tej biblioteki: napisz plik, uruchom go, przeczytaj, co odpowiedział kompilator, wypisz coś, żeby sprawdzić, co się właściwie stało. Pierwsze pięć stron dotyczy `rustc`, ostrzeżeń, komentarzy, wypisywania na ekran i tej składni, której wszystkie późniejsze strony używają bez wyjaśniania — samego języka jeszcze w nich nie ma. Warto od razu przyjąć do wiadomości jedno: `rustc` mówi po angielsku i nie ma przełącznika, który by to zmienił, więc czytanie jego komunikatów jest tu **umiejętnością do wyćwiczenia**, a nie przeszkodą do obejścia. Kod błędu (`E0308`) bywa przy tym najlepszym hasłem do wyszukiwarki, jakie dostaje się za darmo.

Dla polskiego czytelnika jest tu jeden skrót, którego angielska tabela powyżej nie pokazuje: [Tour of Rust ↗](https://tourofrust.com/TOC_pl.html) ma polskie rozdziały 1–5 i pokrywają one ten odcinek z zapasem — podstawowe typy, zmienne mutowalne, kontrola przepływu sterowania, struktury, a na koniec własność i pożyczanie. Polska wersja urywa się po rozdziale 5, więc dalej i tak wraca się do angielskiego, ale ten pierwszy kawałek da się przejść po polsku, a słownictwo tej biblioteki jest z nim celowo zgodne. Druga pozycja z tabeli, *Rust in Easy English*, została napisana w świadomie uproszczonej angielszczyźnie właśnie dla osób, dla których angielski nie jest pierwszym językiem — dla nas jest to zwykle szybsza droga niż The Book.

Cztery pułapki, na które ta sekcja poluje, wyglądają niewinnie i wszystkie są składniowe: `///` **nie jest komentarzem**, tylko zapisem atrybutu `#[doc = "..."]`, a przykłady w środku są kompilowane i uruchamiane jako testy; `_x` i `_` to dwie różne odpowiedzi na to samo ostrzeżenie (pierwsza zostawia wartość przy życiu do końca zasięgu, druga wypuszcza ją natychmiast); jeden średnik więcej na końcu bloku zamienia jego wartość na `()` i kończy się błędem `E0308`; a `{n}` w łańcuchu formatującym (*format string*) to **identyfikator**, a nie f-string z Pythona — `{n + 1}` ani `{v.len()}` się nie skompilują. Ta ostatnia myli najbardziej tych, którzy przychodzą z Pythona.

**Szukaj po polsku:** kurs Rusta po polsku · Tour of Rust po polsku · zmienne mutowalne · `rust E0308 mismatched types` · `rust doc comments ///` · `rust format string named arguments`
