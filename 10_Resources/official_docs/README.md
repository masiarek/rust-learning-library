# Official docs

**Level:** reference · the normative shelf

**One line:** A book argues, a reference *defines* — and when two blog posts disagree about what Rust does, exactly one of the documents below settles it.

**All links checked 2026-08-29.** Every one of these is maintained by the Rust project itself, versioned with the compiler, and installed on your machine already: `rustup doc` opens the whole set offline at the version you are compiling with.

## The difference this page exists to make

A tutorial tells you what to write. A reference tells you what the compiler is *obliged* to do. Most confusion about Rust is people quoting the first when they need the second — "structs are laid out in declaration order" is true of the C representation and [explicitly not guaranteed ↗](https://doc.rust-lang.org/nomicon/repr-rust.html) of the default one, and no amount of re-reading a chapter will tell you that.

Reach for this page when a question has a *right answer* rather than a good practice: what a keyword means, whether behaviour is defined, when a lint fires, why a feature is shaped this way.

## Which document answers which question

| The question | The document |
|---|---|
| What does this type or method do? | [`std` API docs ↗](https://doc.rust-lang.org/std/) |
| What does this syntax *mean* to the language? | [The Reference ↗](https://doc.rust-lang.org/reference/) |
| What am I promising when I write `unsafe`? | [The Rustonomicon ↗](https://doc.rust-lang.org/nomicon/) |
| What is error `E0502` telling me? | [The error index ↗](https://doc.rust-lang.org/error_codes/error-index.html) |
| How do I make Cargo do this? | [The Cargo Book ↗](https://doc.rust-lang.org/cargo/) |
| What changed in the 2024 edition? | [The Edition Guide ↗](https://doc.rust-lang.org/edition-guide/) |
| Why is the feature shaped this way? | [The RFC book ↗](https://rust-lang.github.io/rfcs/) |
| Is my public API idiomatic? | [The API Guidelines ↗](https://rust-lang.github.io/api-guidelines/) |
| What does this clippy lint want? | [The Clippy lint list ↗](https://rust-lang.github.io/rust-clippy/master/) |
| Where does the formatter's opinion come from? | [The Style Guide ↗](https://doc.rust-lang.org/style-guide/) |
| Is this nightly feature ever going to land? | [The Unstable Book ↗](https://doc.rust-lang.org/unstable-book/) |
| What is in a released version? | [Release notes ↗](https://github.com/rust-lang/rust/blob/master/RELEASES.md) |

## The five that matter most

**[The standard library docs ↗](https://doc.rust-lang.org/std/)** — the most-read document in Rust, and the one most people use least well. Three habits pay immediately:

- **The `source` link on every item** shows the actual implementation, which is often shorter than the prose describing it and always more precise.
- **The *Trait Implementations* section** is where the answer usually is. "Why can I compare these?" is `PartialOrd`; "why does `?` work here?" is a `From` impl listed on the error type.
- **`rustup doc --std`** opens your installed copy. That matters more than it sounds: the online copy is the current stable release, and if you are pinned to an older toolchain it will show you methods you cannot call. When a signature matters, read it from the version you are compiling with — the [rustup](../../05_Tooling/rustup/README.md) page has the commands.

For anything outside `std`, [docs.rs ↗](https://docs.rs/) builds the same documentation for every published crate, at every version — the version selector in the top-left is the whole point.

**[The Reference ↗](https://doc.rust-lang.org/reference/)** — the closest thing Rust has to a language specification. It is not a tutorial and does not pretend to be: it defines the grammar, the type system, name resolution, coercions, and what each keyword means. Read it in fragments, when a specific question needs a definitive answer. Its own front page notes it is not yet complete or fully normative, which is honest and worth knowing before quoting it as law.

**[The Rustonomicon ↗](https://doc.rust-lang.org/nomicon/)** — the rules you take responsibility for when `unsafe` turns the checks off. Aliasing, uninitialised memory, `repr` and layout, drop order, exception safety, FFI. Not a book to read early; the [Going deeper](../going_deeper/README.md) shelf says when.

**[The error index ↗](https://doc.rust-lang.org/error_codes/error-index.html)** — every `E0xxx` code with an explanation and a minimal example that triggers it. `rustc --explain E0502` prints the same text in the terminal, which is faster than searching for the message. Underused: the compiler's inline message tells you what happened, and the index tells you *why the rule exists*.

**[The Cargo Book ↗](https://doc.rust-lang.org/cargo/)** — every manifest key, every profile setting, workspaces, features, and the resolver. The [feature-resolution chapter ↗](https://doc.rust-lang.org/cargo/reference/features.html) is the one to read before debugging a dependency conflict, and this library's [dependency pages](../../05_Tooling/cargo_dependencies/README.md) cover the same ground for the common cases.

## The toolchain's own books

Each tool ships its manual, and the manual is better than any blog post about the tool.

| Tool | Book | Reach for it when |
|---|---|---|
| `rustup` | [The rustup Book ↗](https://rust-lang.github.io/rustup/) | toolchains, components, targets, overrides |
| `rustdoc` | [The rustdoc Book ↗](https://doc.rust-lang.org/rustdoc/) | doc comments, doctests, intra-doc links |
| `rustc` | [The rustc Book ↗](https://doc.rust-lang.org/rustc/) | lint levels, codegen options, target specs |
| `clippy` | [The Clippy Book ↗](https://doc.rust-lang.org/clippy/) | configuring lints, and [the searchable lint list ↗](https://rust-lang.github.io/rust-clippy/master/) |
| `rustfmt` | [The Style Guide ↗](https://doc.rust-lang.org/style-guide/) | arguing with the formatter, and losing correctly |
| `mdBook` | [The mdBook Book ↗](https://rust-lang.github.io/mdBook/) | building a book of your own — most of the [Books](../books/README.md) shelf uses it |

## Where the decisions are made

Not documentation of what Rust *is*, but of how it got that way and where it is going. Useful when the answer to "why can't I do this?" is historical rather than technical.

- **[The RFC book ↗](https://rust-lang.github.io/rfcs/)** — every accepted design proposal, with the discussion that shaped it. Search it by feature name when a design looks arbitrary; it usually is not.
- **[The Edition Guide ↗](https://doc.rust-lang.org/edition-guide/)** — what changed in 2018, 2021 and 2024, and what `cargo fix --edition` will do to your code. The first thing to read when an example from a blog post no longer compiles.
- **[The Unstable Book ↗](https://doc.rust-lang.org/unstable-book/)** — every nightly-only feature gate, and the tracking issue behind it.
- **[The API Guidelines ↗](https://rust-lang.github.io/api-guidelines/)** — the project's checklist for a public crate interface: naming, trait implementations, what to make `#[non_exhaustive]`. Short, and the single fastest way to make a library feel like Rust.
- **[Unsafe Code Guidelines ↗](https://rust-lang.github.io/unsafe-code-guidelines/)** — the ongoing work to pin down what `unsafe` code is actually allowed to assume. Explicitly *not* settled, which is itself the useful information.
- **[Rust Project Goals ↗](https://rust-lang.github.io/rust-project-goals/)** — what the project has committed to this cycle.

## For working on Rust itself

- **[Guide to Rustc Development ↗](https://rustc-dev-guide.rust-lang.org/)** — how the compiler is built and how a change gets through it. Also the best available explanation of what borrow checking actually does, if you want the mechanism rather than the rules.
- **[Standard library developers guide ↗](https://std-dev-guide.rust-lang.org/about.html)** — the bar a change to `std` has to clear, including stability.
- **[Rust Forge ↗](https://forge.rust-lang.org/)** — release process, infrastructure, team structure.

## Everything, offline

```bash
rustup doc            # the whole set, at your installed version
rustup doc --book     # The Book
rustup doc --std      # the standard library
rustup doc --reference
rustup doc --nomicon
rustc --explain E0502 # one error code, in the terminal
```

The offline copy is not a convenience feature. It is the *version-matched* copy: if `rustup show` says you are on a pinned toolchain, the online docs are describing a different compiler from the one rejecting your code.

## See also

- [Books](../books/README.md) — the shelf that argues, reviewed
- [Resources](../README.md) — the map of every shelf
- [Going deeper](../going_deeper/README.md) — the domain shelves, and when the Nomicon becomes relevant
- [Reading a compilation failure](../../20_Compilers/reading_a_compilation_failure/README.md) — this library on what the compiler is telling you, before you go looking it up

## Po polsku

Różnica, dla której ta strona istnieje, jest po polsku równie ostra: podręcznik **przekonuje**, dokumentacja referencyjna **definiuje**. Zanim zaczniesz szukać, rozstrzygnij, czy twoje pytanie ma dobrą odpowiedź, czy tylko dobrą praktykę — bo tylko to pierwsze da się rozstrzygnąć cytatem. Klasyczny przykład z tej strony: zdanie „pola struktury leżą w pamięci w kolejności deklaracji” jest prawdziwe dla reprezentacji C i **jawnie niegwarantowane** dla domyślnej, a żadna liczba powtórnych lektur rozdziału z podręcznika tego nie powie, bo podręcznik po prostu o tym nie mówi.

Nic z tej półki nie istnieje po polsku i to akurat przeszkadza mniej, niż się wydaje: to jest **najłatwiejszy angielski w całym ekosystemie**. Dokumentacja referencyjna jest gęsta, ale schematyczna — definicje, tabele, sygnatury, zero idiomów i żartów — więc czyta się ją znacznie łatwiej niż wpis blogowy. Ryzykowny jest dopiero zamiennik: polski artykuł, który przekonuje zamiast definiować i prawie nigdy nie podaje, której wersji Rusta dotyczy. Dwa nawyki załatwiają większość problemu. Pierwszy: `rustc --explain E0502` — kod błędu jest niezależny od języka i jest najlepszym kluczem wyszukiwania, jaki masz, a podział ról warto zapamiętać, bo jest wygodny: komunikat kompilatora mówi, **co się stało**, a indeks błędów mówi, **dlaczego ta reguła w ogóle istnieje**. Drugi: `rustup doc`, które otwiera kopię offline **zgodną z twoją wersją** — jeśli masz przypiętą starszą wersję narzędzi, dokumentacja w sieci opisuje inny kompilator niż ten, który właśnie odrzucił twój kod.

Przy `std` warte wyrobienia są jeszcze dwa odruchy, bo to dokument najczęściej czytany i najgorzej wykorzystywany. Odpowiedź zwykle nie leży w opisie metody, tylko w sekcji **Trait Implementations** — „dlaczego mogę to porównać?” to `PartialOrd`, „dlaczego działa tu `?`” to jakieś `impl From` wypisane przy typie błędu. Drugi odruch to link `source` przy każdej pozycji: implementacja bywa krótsza od prozy, która ją opisuje, i zawsze jest dokładniejsza — a dla kogoś, kto czyta w drugim języku, kod jest po prostu jednoznaczniejszy niż zdanie. Na koniec zastrzeżenie, które warto znać, zanim się kogoś zacytuje: The Reference to najbliższa rzecz, jaką Rust ma do specyfikacji języka, ale jej własna strona tytułowa przyznaje, że nie jest kompletna ani w pełni normatywna. To jest najlepszy dostępny dowód, a nie prawo.

**Szukaj po polsku:** dokumentacja Rusta · specyfikacja języka · `rustup doc` · `rustc --explain E0502` · `docs.rs`
