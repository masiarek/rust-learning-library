# Books

**Level:** reference

**One line:** Every Rust book worth reading is free and online, which removes the buying decision and replaces it with a harder one — which of them is for *now*.

All links checked 2026-08-23.

## The one everyone means

**[The Rust Programming Language](https://doc.rust-lang.org/book/)** — "The Book", Klabnik and Nichols. Ships with the toolchain (`rustup doc --book`, offline). If you read one thing, this. Chapters 1–10 are the language; 11 onwards are increasingly optional.

Three variants worth knowing about, because they solve different problems with the same text:

- **[The Brown University edition](https://rust-book.cs.brown.edu/)** — the same book with interactive quizzes and ownership diagrams inserted. The quizzes are *diagnostic*: they are built around the misconceptions readers actually have. If ownership has not clicked, read chapter 4 here rather than again in the original.
- **[The abridged Book](https://jasonwalton.ca/rust-book-abridged/)** — same material, much shorter, aimed at people who already program. Good if The Book's pace is testing your patience.
- **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** — the official companion: runnable examples with commentary instead of prose with examples. A reference to jump around in, not a book to read front to back.

## First-day alternatives

**[Comprehensive Rust](https://google.github.io/comprehensive-rust/)** — Google's four-day course, used to teach their own Android and Chromium teams, and released free. Slide-shaped rather than prose-shaped, with speaker notes and exercises. Includes whole days on Android, bare-metal and concurrency that no other free course covers. Probably the best material here for someone who learns from a taught course.

**[A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)** — older and shorter, and its tone is exactly what the title says. Some of it predates current idiom, so treat it as an on-ramp rather than a reference.

**[teach-rs](https://github.com/trifectatechfoundation/teach-rs)** — a modular *university course* — slides, exercises, and a syllabus you can rearrange. Aimed at whoever is doing the teaching, which makes it the odd one out here and the right one if you ever have to run a session.

## Coming from another language

**[Rust for C Programmers](https://rust-for-c-programmers.salewskis.de/ch1/chapter_1_rust_for_c_programmers.html)** — assumes you know pointers, arenas and undefined behaviour, and spends its time on what Rust does differently instead of explaining what a loop is.

**[Learn Rust the Dangerous Way](https://cliffle.com/p/dangerust/1/)** — takes a real C program (an n-body simulation) and converts it to Rust in stages, starting with a literal `unsafe` transliteration and refactoring toward idiom. The single best thing on this page for understanding *why* the safe version is shaped the way it is.

## The one that teaches ownership properly

**[Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)** — builds a linked list six times, each attempt failing in an instructive way. It is genuinely funny, and it is the canonical answer to "I understand the rules and still cannot write anything."

**[quinedot's ownership, borrowing and lifetimes](https://quinedot.github.io/rust-learning/)** — the best free treatment of lifetimes specifically, including the parts The Book leaves out. Reach for this the day an error mentions a lifetime you did not write.

## Reference and lookup

- **[cheats.rs](https://cheats.rs/)** — one enormous page covering the whole language and standard library. The thing to have open in a tab.
- **[Rust Design Patterns](https://rust-unofficial.github.io/patterns/)** — idioms, patterns and anti-patterns, with the reasoning. Read once you can write Rust and are starting to wonder if you are writing it *well*.
- **[The Rust Performance Book](https://nnethercote.github.io/perf-book/)** — by one of the people who made rustc faster. Practical and specific.

## Lists of lists

When this page runs out:

- **[Awesome Rust](https://github.com/rust-unofficial/awesome-rust)** — crates and applications, not learning material. For "is there a crate for this".
- **[ctjhoa/rust-learning](https://github.com/ctjhoa/rust-learning)** — a very large curated list of articles, books and talks.
- **[Little Book of Rust Books](https://lborb.github.io/book/official.html)** — an index of essentially every Rust book that exists.
- **[rust-edu.org](https://rust-edu.org/)** — teaching materials and the academic side.

## One with a caveat

**[The Rust Anthology](https://brson.github.io/rust-anthology/1/intro.html)** — a collection of the best Rust blog posts of the mid-2010s. Genuinely good writing, and *old*: it predates async/await, the 2018 edition, and NLL. Read it for insight into why Rust is the way it is; do not copy code out of it.

## And to build your own

**[mdBook](https://rust-lang.github.io/mdBook/index.html)** — the tool almost every book on this page is built with. Markdown in, static site out, and it runs your code samples as tests, which is the same promise this library makes with [`run_examples.py`](https://github.com/masiarek/rust-learning-library/blob/master/tools/run_examples.py).

## See also

- [Exercises](../exercises/README.md) — when you have read enough
- [Going deeper](../going_deeper/README.md) — the domain-specific shelf
