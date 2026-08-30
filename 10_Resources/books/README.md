# Books

**Level:** reference · the shelf

**One line:** Around seventy Rust books are named below, roughly a dozen are worth anyone's time, and which dozen depends entirely on what you already know — so every entry carries a verdict and the reason for it.

**Publication facts checked 2026-08-29** against the publisher's own page, and every link on this page answered on the same date. Two publishers refuse automated requests: Packt's pages were confirmed in a browser instead, and O'Reilly's storefront could not be reached from here at all — so its two print-only titles link their **official code repository**, which resolves, states the edition, and is the more useful link for a programmer anyway. What was *not* checked, and cannot be, is the verdict — see the next section, which says exactly what a verdict here is worth.

## How to read a verdict

Two kinds of claim sit side by side below, and they are not equally solid.

**Facts are checked.** Publisher, date, page count, price, edition, and whether a free copy exists were fetched from the publisher on the date above and quoted as found. Where a book has a newer edition than the one people usually mean, both are named.

**Verdicts are judgment.** They come from the books' own text and sample chapters, from what each one is trying to do, and from how the Rust community has actually used them — not from a controlled reading of all seventy. Where the community is split, the page says so instead of picking a side. Where a verdict is thin, it says **Unvetted** rather than inventing confidence. This is the one page in the library that cannot be backed by [a program that prints the answer](../../CONTRIBUTING.md), and pretending otherwise would be worse than admitting it.

| Verdict | Means |
|---|---|
| **Read it** | the best thing that exists on its subject, with nothing close behind |
| **Read it for X** | very good at one job — know which job before you start |
| **Reference** | consulted, not read front to back |
| **Only if X** | narrow: right for a specific reader, wasted on everyone else |
| **Dated** | was good; the code has aged past the point of copying |
| **Unvetted** | listed so the shelf is complete; judgment withheld |

## If you read one thing

| If you are… | Read | Verdict |
|---|---|---|
| new to Rust | [The Book](../../00_Start_Here/the_book/README.md), chapters 1–10 | Read it |
| new to Rust and learn better from a taught course | [Comprehensive Rust ↗](https://google.github.io/comprehensive-rust/) | Read it |
| stuck on ownership after reading chapter 4 | [Too Many Linked Lists ↗](https://rust-unofficial.github.io/too-many-lists/) | Read it |
| stuck on lifetimes specifically | [quinedot's guide ↗](https://quinedot.github.io/rust-learning/) | Read it |
| writing Rust at work and want to write it *well* | [Rust for Rustaceans ↗](https://nostarch.com/rust-rustaceans) | Read it |
| shipping a Rust service | [Zero To Production ↗](https://www.zero2prod.com/) | Read it for the practice, not the framework |
| writing concurrent code | [Rust Atomics and Locks ↗](https://marabos.nl/atomics/) | Read it |
| coming from C or C++ | Programming Rust — [official code ↗](https://github.com/ProgrammingRust/examples) | Read it |

Everything in that table except the last two rows is free. That is the unusual thing about Rust: the standard first book, the standard ownership book, the standard concurrency book and the standard idioms book are all free, legal, and one click away.

---

# Free and online

## The first book, and its three editions

The Book is argued for properly in [**Start here**](../../00_Start_Here/the_book/README.md) — which chapters, in what order, and why to read chapter 4 twice. One owner per fact; this page owns the editions.

| Edition | Verdict | Why |
|---|---|---|
| [**The Rust Programming Language** ↗](https://doc.rust-lang.org/book/) | **Read it** | The default, maintained by the project, versioned with your compiler (`rustup doc --book`). No competitor. |
| [**Brown University edition** ↗](https://rust-book.cs.brown.edu/) | **Read it** — on the second pass | Identical text plus diagnostic quizzes built from the misconceptions real readers hold. Strictly better for chapter 4, at no cost. |
| [**The abridged Book** ↗](https://jasonwalton.ca/rust-book-abridged/) | **Read it for** speed | Same material, far shorter, for people who already program. Not a substitute for a first read if Rust is your first systems language. |
| [Print, 3rd ed ↗](https://nostarch.com/rust-programming-language-3rd-edition) | See [In print](#in-print) | No Starch, March 2026, 624 pp, $59.99. The online text is the same book, free. |

## The other first books

**[Comprehensive Rust ↗](https://google.github.io/comprehensive-rust/)** — **Read it.** Google's four-day course, written to teach their own Android and Chromium engineers and released free. Slide-shaped rather than prose-shaped, with speaker notes and exercises, and it carries whole days on Android, bare-metal and concurrency that nothing else free covers. The best material on this page for anyone who learns from a taught course rather than a chapter. The one cost of the format: slides assume a voice filling the gaps, so a solo reader has to supply the pacing themselves.

**[100 Exercises to Learn Rust ↗](https://rust-exercises.com/100-exercises/)** — **Read it for** typing rather than reading. Luca Palmieri's test-driven course: a hundred failing tests, each one a concept. The right thing after [rustlings](../../00_Start_Here/rustlings/README.md) and the right thing *instead of* a second explanation of ownership.

**[Rust By Practice ↗](https://practice.rs/)** — **Read it for** drilling a specific chapter. Exercises with in-browser answers, organised by topic rather than as a course. Thinner than the two above; useful when one idea has not stuck.

**[Easy Rust ↗](https://dhghomon.github.io/easy_rust/)** — **Read it for** simple English. Dave MacLeod wrote it in deliberately restricted vocabulary for readers whose first language is not English, and there is nothing else like it. Last substantially updated in 2021, so some idiom has moved; his [Month of Lunches](#the-friendly-paid-on-ramps) is the maintained descendant.

**[Rust 101 ↗](https://rust-lang.guide/)** — **Unvetted.** A guided path through the official material rather than new prose. Harmless; adds little if you already have The Book open.

**[A Gentle Introduction to Rust ↗](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)** — **Dated.** The tone is exactly what the title promises and the explanations are still clear, but it predates a great deal of current idiom. An on-ramp, not a reference.

## Ownership, which is the wall

**[Learning Rust With Entirely Too Many Linked Lists ↗](https://rust-unofficial.github.io/too-many-lists/)** — **Read it.** Builds a linked list six times, each attempt failing in an instructive way. It is funny, it is short, and it is the canonical answer to *"I understand the rules and still cannot write anything."* The single highest-value free book on this page after The Book itself.

**[quinedot — Learning Rust ↗](https://quinedot.github.io/rust-learning/)** — **Read it** the day an error mentions a lifetime you did not write. The best free treatment of lifetimes anywhere, including the parts The Book leaves out — variance, `dyn` and lifetime elision among them.

**[LifetimeKata ↗](https://tfpk.github.io/lifetimekata/)** — **Read it for** the fingers. Exercises rather than explanation; pairs with the page above.

This library's own [Ownership](../../18_Ownership/README.md) section covers the same ground with the output proved, and [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) says which of these to reach for when.

## Coming from another language

**[Learn Rust the Dangerous Way ↗](https://cliffle.com/p/dangerust/1/)** — **Read it.** Takes a real C program — an n-body simulation — and converts it to Rust in stages, opening with a literal `unsafe` transliteration and refactoring toward idiom. The best thing on this page for understanding *why* the safe version is shaped the way it is, and it is an afternoon rather than a book.

**[Rust for the Polyglot Programmer ↗](https://www.chiark.greenend.org.uk/~ianmdlvl/rust-polyglot/index.html)** — **Read it for** a fast orientation. Ian Jackson writes for someone who already knows three languages and wants the delta, not the tutorial. Opinionated, including about the ecosystem, and says so.

**[Rust for C Programmers ↗](https://rust-for-c-programmers.salewskis.de/ch1/chapter_1_rust_for_c_programmers.html)** — **Read it for** the C reader. Assumes pointers, arenas and undefined behaviour are familiar, and spends its pages on what Rust does differently.

**[py2rs ↗](https://rochacbruno.github.io/py2rs/)** — **Reference.** A lookup table from Python to Rust: this idiom becomes that one, this library becomes that crate. Not a course. Useful open in a tab for the first fortnight.

**[Comparing parallel Rust and C++ ↗](https://parallel-rust-cpp.github.io/introduction.html)** — **Only if** you benchmark. Eight implementations of one numerical kernel in both languages, measured. Narrow and unusually rigorous.

## After the basics

**[Effective Rust ↗](https://effective-rust.com/)** — **Read it for** a checklist once you can already write Rust. David Drysdale's thirty-five items in the *Effective C++* mould, free online and in print (O'Reilly, April 2024, 280 pp). The items on the type system and on the borrow checker are the strongest; the criticism it reliably draws is that some items are shallow — a paragraph where the topic wanted a chapter. Fair, and it is still the fastest way to find out what you have been doing wrong.

**[Rust Design Patterns ↗](https://rust-unofficial.github.io/patterns/)** — **Reference.** Idioms, patterns and — the useful half — anti-patterns, each with its reasoning. Community-maintained, so depth varies by entry. Read once you have started wondering whether you are writing Rust *well*.

**[The Rustonomicon ↗](https://doc.rust-lang.org/nomicon/)** — **Read it** before you write `unsafe`, and not before that. The official book on the rules you must uphold once the compiler stops checking. It opens by telling you not to read it, which is correct: the rules only make sense once you know what is being relaxed.

**[High Assurance Rust ↗](https://highassurance.rs/)** — **Read it for** the security angle. Builds a production-grade data structure while teaching threat modelling, fuzzing, static analysis and CI alongside the language. Free, long, and unusually serious; the closest thing to a graduate course on this page. Still marked as a work in progress by its authors, so later chapters are thinner than early ones.

**[Rust Atomics and Locks ↗](https://marabos.nl/atomics/)** — **Read it.** Mara Bos — a Rust library team lead — on memory ordering, atomics, and building a mutex, a condition variable and a reader-writer lock from scratch. Free online, and in print from O'Reilly (January 2023, 250 pp). Narrow on purpose and definitive within that scope: this is the book to reach for the first time `Ordering::Relaxed` appears in a code review. Nothing else free is close.

**[The Rust Performance Book ↗](https://nnethercote.github.io/perf-book/)** — **Reference.** By one of the people who made rustc itself faster. Short, specific, measured, and it starts by telling you to benchmark before optimising.

## On the desk

- **[cheats.rs ↗](https://cheats.rs/)** — **Reference.** One enormous page covering the language and the standard library, including the memory-layout diagrams nothing else has. The thing to keep in a tab.
- **[Rust Cookbook ↗](https://rust-lang-nursery.github.io/rust-cookbook/)** — **Dated.** Task-shaped recipes ("read a CSV", "make an HTTP request") with working code. The recipes still compile; several pin crate versions from a previous era, so treat the *shape* as current and the dependency list as history.
- **[The Little Book of Rust Macros ↗](https://veykril.github.io/tlborm/)** — **Reference.** The maintained fork of Daniel Keep's original, and the only complete free treatment of `macro_rules!`. Dense. Pair with [MacroKata ↗](https://tfpk.github.io/macrokata/) for the exercises.
- **[Error Handling in Rust ↗](https://nrc.github.io/error-docs/)** — **Reference.** Nick Cameron on the whole design space — `Result`, panics, error types, the crates. Short and clear, and it answers the "`anyhow` or `thiserror`?" question properly. This library's [`Option` and `Result`](../../17_Option_and_Result/README.md) section covers the same material with the output proved.

## A subject at a time

Books here are listed so the shelf is complete; [Going deeper](../going_deeper/README.md) is the page that says *when* to open one.

| Subject | Book | Verdict |
|---|---|---|
| Async | [The Async Book ↗](https://rust-lang.github.io/async-book/) | **Reference** — official, and under active rewrite; chapters vary in polish |
| Embedded | [The Embedded Rust Book ↗](https://docs.rust-embedded.org/book/) | **Read it** if the target has no operating system |
| Embedded, hands on | [Discovery ↗](https://docs.rust-embedded.org/discovery/) | **Read it for** real hardware on a desk |
| Embedded, the deep end | [The Embedonomicon ↗](https://docs.rust-embedded.org/embedonomicon/) | **Only if** you are writing the runtime yourself |
| CLI tools | [Command Line Applications in Rust ↗](https://rust-cli.github.io/book/index.html) | **Read it for** the conventions — exit codes, stderr, `--help` |
| WebAssembly | [Rust and WebAssembly ↗](https://rustwasm.github.io/docs/book/) | **Dated** in its tooling; the concepts hold |
| Fuzzing | [The Rust Fuzz Book ↗](https://rust-fuzz.github.io/book/) | **Read it for** `cargo-fuzz`, which is an afternoon well spent |
| Parsing | [The Nominomicon ↗](https://tfpk.github.io/nominomicon/) | **Read it for** `nom` specifically |
| Security review | [Secure Rust Guidelines ↗](https://anssi-fr.github.io/rust-guide/) | **Reference** — the French cybersecurity agency's rules, and the only formal checklist here |
| C++ interop | [CXX ↗](https://cxx.rs) | **Reference** — the crate's own book, and the best explanation of the problem |
| Serialization | [Serde ↗](https://serde.rs/) | **Reference** — the crate's own book, better than most paid chapters on the subject |

## Build something books

These teach by finishing a program. All free, all **Read it for** the project named — none is a language reference, and none should be your first book.

| Book | You end up with |
|---|---|
| [PNGme ↗](https://jrdngr.github.io/pngme_book/) | a CLI that hides messages in PNG chunks — the best first project here |
| [Writing Interpreters in Rust ↗](https://rust-hosted-langs.github.io/book/introduction.html) | an interpreter, with a garbage collector you wrote |
| [Build a Lua Interpreter in Rust ↗](https://wubingzheng.github.io/build-lua-in-rust/en/) | a working Lua |
| [Create Your Own Programming Language with Rust ↗](https://createlang.rs/) | a small language, gently |
| [Writing an NES Emulator in Rust ↗](https://bugzmanov.github.io/nes_ebook/index.html) | a 6502 emulator that runs real cartridges |
| [Rust Sokoban ↗](https://sokoban.iolivia.me/) | a game, via an entity-component system |
| [Roguelike Tutorial in Rust ↗](https://bfnightly.bracketproductions.com/) | a full roguelike — long, and the best-organised of the game books |
| [Triangle From Scratch ↗](https://rust-tutorials.github.io/triangle-from-scratch/) | one triangle on screen with no dependencies at all |

## Lists of lists

When this page runs out:

- **[The Little Book of Rust Books ↗](https://lborb.github.io/book/)** — the index of essentially every Rust book that exists, and the source used to check this page for gaps.
- **[ctjhoa/rust-learning ↗](https://github.com/ctjhoa/rust-learning)** — a very large curated list of articles, books and talks.
- **[Awesome Rust ↗](https://github.com/rust-unofficial/awesome-rust)** — crates and applications, not learning material. For *"is there a crate for this"*.
- **[rust-edu.org ↗](https://rust-edu.org/)** — teaching materials and the academic side.

---

# In print

Nothing below is required. The free shelf above covers first book, ownership, lifetimes, concurrency, idioms and performance, which is most of a curriculum — so treat this section as *what paper buys you*, and the honest answer is editing, sequencing, and someone's undivided attention on a subject.

## The four that earn their price

| Book | Facts | Verdict |
|---|---|---|
| [**Rust for Rustaceans** ↗](https://nostarch.com/rust-rustaceans) | Jon Gjengset · No Starch · Nov 2021 · 280 pp · $49.99 | **Read it** — the standard second book |
| **Programming Rust** — [official code ↗](https://github.com/ProgrammingRust/examples) | Blandy, Orendorff & Tindall · O'Reilly · 2nd ed 2021, ISBN 9781492052593 · 3rd ed announced for late 2026 | **Read it** if you come from C or C++ |
| [**Zero To Production In Rust** ↗](https://www.zero2prod.com/) | Luca Palmieri · self-published · continuously revised · £35 ebook | **Read it for** production practice |
| [**Rust Atomics and Locks** ↗](https://marabos.nl/atomics/) | Mara Bos · O'Reilly · Jan 2023 · 250 pp · **free online** | **Read it** — and you need not pay |

**Rust for Rustaceans** is the book people mean by "the second Rust book", and it deserves the position: type layout, trait coherence, object safety, `Pin` and `Waker` from underneath async/await, `no_std`, macros, FFI, API design. It is also the book most often described as hard to follow, and that criticism is fair rather than a failure of the reader — it is 280 pages carrying what another author would spend 600 on, with no worked-example scaffolding. Read it slowly, after a year of writing Rust, with an editor open. Still the 2021 first edition as of this check; nothing in the core material has gone wrong, but the async ecosystem around it has moved.

**Programming Rust** is the most complete systems treatment in print: what a value *is* in memory, what a move compiles to, how trait objects are laid out. It is not a first book and does not try to be. Note the edition carefully — the 2nd edition (2021) is what is on shelves now, and the 3rd, fully updated for the 2024 edition, is announced for late 2026. If you are about to buy, that is a reason to wait.

**Zero To Production In Rust** is not really a Rust book; it is a book about shipping software, written in Rust. Testing strategy, telemetry, CI, database migrations, deployment, and how to keep a codebase honest as it grows — material almost nobody else writes down. The recurring criticism is fair too: it is built on `actix-web`, and the centre of gravity in the ecosystem has moved to `axum`. The *practices* transfer entirely; the framework chapters are a translation exercise. Buy it for the discipline, not the router.

**Rust Atomics and Locks** is in this table because it is worth a paid book's attention, not because it costs anything — Mara Bos publishes the whole text free. Buy the paper copy if you want to support it.

## The friendly paid on-ramps

| Book | Facts | Verdict |
|---|---|---|
| [The Rust Programming Language, 3rd ed ↗](https://nostarch.com/rust-programming-language-3rd-edition) | Klabnik, Nichols & Krycho · No Starch · Mar 2026 · 624 pp · $59.99 | **Read it for** paper — the text is [free online ↗](https://doc.rust-lang.org/book/) |
| [Learn Rust in a Month of Lunches ↗](https://www.manning.com/books/learn-rust-in-a-month-of-lunches) | David MacLeod · Manning · Feb 2024 · 568 pp | **Read it for** the gentlest paid start |
| [Rust in Action ↗](https://www.manning.com/books/rust-in-action) | Tim McNamara · Manning · Jun 2021 · 456 pp | **Read it for** systems programming *through* Rust |
| Command-Line Rust — [official code ↗](https://github.com/kyclark/command-line-rust) | Ken Youens-Clark · O'Reilly · 2022 · ~396 pp · ISBN 9781098109431 | **Read it for** practice with tests |

The **3rd edition** of The Book is a genuine revision rather than a reprint: built on the Rust 2024 edition, with a complete async chapter and a section on Miri for analysing unsafe code, and Chris Krycho joins as a third author. The online text tracks the same content and remains free, so this is a purchase about paper and about supporting the project.

**Learn Rust in a Month of Lunches** is the descendant of [Easy Rust](#the-other-first-books) and inherits its virtue: it explains as if to a person, in short sittings, without assuming a C background. If The Book's pace has defeated you twice, this is the fix. At 568 pages the "month of lunches" framing is optimistic.

**Rust in Action** is the odd and enjoyable one — you build a CPU emulator, a key-value store, a network stack, a filesystem. It teaches *systems programming* using Rust as the vehicle, so someone who wants only to be productive in Rust will find it a detour, and someone curious about how computers work will find it the best book on the page. It is from 2021; some crate code has aged, and the concepts have not.

**Command-Line Rust** rebuilds a dozen coreutils — `head`, `cut`, `wc`, `find` — each with a test suite written first. As a bridge from "I have read The Book" to "I have shipped something", it is the most practical exercise book in print. The `clap` API moved after publication, so expect to translate the argument-parsing code; the author's code repository tracks a 2024 printing.

## One subject, one book

| Book | Facts | Verdict |
|---|---|---|
| [Write Powerful Rust Macros ↗](https://www.manning.com/books/write-powerful-rust-macros) | Sam Van Overmeire · Manning · May 2024 · 320 pp | **Read it for** procedural macros — the only book-length treatment |
| [Asynchronous Programming in Rust ↗](https://www.packtpub.com/en-us/product/asynchronous-programming-in-rust-9781805128137) | Carl Fredrik Samson · Packt · Feb 2024 · 306 pp | **Read it for** building a runtime yourself |
| [Refactoring to Rust ↗](https://www.manning.com/books/refactoring-to-rust) | Lily Mara & Joel Holmes · Manning · Jun 2025 · 304 pp | **Read it for** adding Rust to an existing codebase |
| [Effective Rust ↗](https://effective-rust.com/) | David Drysdale · O'Reilly · Apr 2024 · 280 pp · **free online** | **Read it for** a checklist |
| [Rust Brain Teasers ↗](https://pragprog.com/titles/hwrustbrain/rust-brain-teasers/) | Herbert Wolverson · Pragmatic · Mar 2022 · 138 pp | **Read it for** an evening of puzzles |
| [Hands-on Rust ↗](https://pragprog.com/titles/hwrust/hands-on-rust/) | Herbert Wolverson · Pragmatic · Jul 2021 · 342 pp | **Only if** you want to build a game |
| [Black Hat Rust ↗](https://kerkour.com/black-hat-rust) | Sylvain Kerkour · self-published | **Only if** offensive security is the job |

**Asynchronous Programming in Rust** earns its place by building green threads, a future, and a working executor from scratch before it lets you use `tokio`. That is the explanation the official async book never quite gives, and it is why this is the one Packt Rust title with a clear recommendation.

**Refactoring to Rust** is about the situation most working programmers are actually in — a large codebase in another language, and one component that would benefit. FFI, incremental migration, and how to keep the build honest across the boundary.

**Hands-on Rust** is well written and tightly coupled to `bracket-lib`, a teaching-oriented game library. If you want a roguelike, it is good; if you want Rust, the coupling costs you.

## Also on the shelf

Real books, published by real publishers, each with a narrower audience than the sections above — listed so the shelf is complete rather than because they are the next thing to read.

| Book | Facts | Verdict |
|---|---|---|
| [Idiomatic Rust ↗](https://www.manning.com/books/idiomatic-rust) | Brenden Matthews · Manning · Aug 2024 · 256 pp | **Unvetted** — overlaps [Rust Design Patterns ↗](https://rust-unofficial.github.io/patterns/), which is free |
| [Code Like a Pro in Rust ↗](https://www.manning.com/books/code-like-a-pro-in-rust) | Brenden Matthews · Manning · Feb 2024 · 264 pp | **Unvetted** — tooling, testing and async at a survey depth |
| [Rust Web Programming, 3rd ed ↗](https://www.packtpub.com/en-us/product/rust-web-programming-9781835887769) | Maxwell Flitton · Packt · Jan 2026 · 674 pp | **Only if** web is the job — the most current web title, and very long |
| [Rust Web Development ↗](https://www.manning.com/books/rust-web-development) | Bastian Gruber · Manning · Dec 2022 · 400 pp | **Dated** — built on `warp`, which the ecosystem has largely left |
| [The Secrets of Rust: Tools ↗](https://bitfieldconsulting.com/books/rust-tools) | John Arundel · self-published · $29.95 | **Unvetted** — see [the disclosure below](#where-the-second-opinions-come-from) |
| [The Rust Spellbook ↗](https://bitfieldconsulting.com/books/rust-spellbook) | John Arundel · self-published · $29.95 | **Unvetted** — same disclosure |
| [Programming WebAssembly with Rust ↗](https://pragprog.com/titles/khrust/programming-webassembly-with-rust/) | Kevin Hoffman · Pragmatic · 2019 | **Dated** — the wasm toolchain is unrecognisable since |

## Books that were good and are now dated

The Packt back catalogue of 2017–2019 — *Mastering Rust*, *Rust High Performance*, *Hands-On Concurrency with Rust*, *Network Programming with Rust*, *Rust Programming By Example*, *Beginning Rust*, *Practical Rust Projects*, *Creative Projects for Rust Programmers* — predates or barely postdates the 2018 edition. Their code frequently will not compile, their crate recommendations are archaeology, and the language has gained `async`/`await`, non-lexical lifetimes, const generics and two editions since. Secondhand copies are cheap for a reason. Nothing on that list is the *only* book on its subject any more.

[The Rust Anthology ↗](https://brson.github.io/rust-anthology/1/intro.html) is the honourable exception: a collection of the best mid-2010s Rust blog posts, kept for insight into *why* Rust is shaped the way it is. Read it for the arguments; do not copy code out of it.

## The Amazon problem

Search "Rust programming" on any large bookstore and most of the first page will be titles nobody in the Rust community has heard of, published in the last two years, by authors with no other work and no online presence. Many are machine-generated: correct-sounding prose over code that does not compile, and a five-star review count that arrived in one week.

Three checks that cost nothing:

- **Is there a publisher?** No Starch, O'Reilly, Manning, Pragmatic and Packt all employ technical reviewers. A self-published book can be excellent — Palmieri's and Kerkour's are — but then the *author* is the credential, and they will have a visible history: crates, talks, a blog, an ecosystem role.
- **Does the code live somewhere?** Every legitimate Rust book has a public repository that compiles against a stated toolchain.
- **Is it discussed anywhere real?** [users.rust-lang.org ↗](https://users.rust-lang.org/), [r/rust ↗](https://www.reddit.com/r/rust/), *This Week in Rust*. A book with no thread anywhere and four hundred reviews is telling you something.

## Where the second opinions come from

The verdicts above are this library's own. Where community opinion was consulted, it came from the Rust user forum, r/rust, and published reviews — and two things about the published reviews are worth knowing before you weigh them.

**The most-linked "best Rust books" review ranks its own author's books first.** [Bitfield Consulting's list ↗](https://bitfieldconsulting.com/posts/best-rust-books) names *The Secrets of Rust: Tools* as best for beginners and *The Rust Spellbook* as best for intermediates; both are written and sold by John Arundel, who runs Bitfield. That is not proof the books are bad — Arundel is an experienced technical author with a real backlist — but a ranking cannot be evidence about the ranker's own entries, and the page does not say so. They are listed above as **Unvetted** for exactly that reason: the reviews that exist are not independent, and this library has not read them.

**Affiliate lists are not reviews.** A large share of "top 10 Rust books" results are affiliate-linked and rank by commission and search traffic. The tell is that every book is recommended and none is placed against another.

The same rule this library applies to voting-method advocacy applies here: use a source for what it is good at, and disclose the lean.

## See also

- [Resources](../README.md) — the map of every shelf, and which one answers which question
- [Official docs](../official_docs/README.md) — the documents that *define* Rust, for when two books disagree
- [Exercises](../exercises/README.md) — when you have read enough
- [Going deeper](../going_deeper/README.md) — the domain shelf, and when to open one
- [Start here](../../00_Start_Here/README.md) — the three foundational resources, argued for properly
