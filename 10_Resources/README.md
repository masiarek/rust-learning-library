# Resources

**Level:** reference · the map

**One line:** A curated shelf rather than a link dump — every entry says *when it is the right one*, because the problem with Rust learning material is not that there is too little of it.

There is an enormous amount of good Rust material, almost all of it free, and that is precisely the difficulty: at any given moment roughly two of these are the right thing to read and the rest are a way of avoiding the one you need. So the shelves below are organised by **the moment you are in** rather than by topic.

## Five kinds of material, and what each is for

Most wasted reading is a category error — opening a tutorial when the question has a right answer, or a specification when the question is *"how do people usually do this?"*

| Kind | Answers | Shelf |
|---|---|---|
| **Normative documents** | *What is Rust obliged to do?* | [Official docs](official_docs/README.md) |
| **Books** | *Why is it shaped this way?* | [Books](books/README.md) |
| **Exercises** | *Can I actually write it?* | [Exercises](exercises/README.md) |
| **Domain material** | *How does this work for async / embedded / macros?* | [Going deeper](going_deeper/README.md) |
| **Lookup** | *What is the name of the thing I want?* | [cheats.rs ↗](https://cheats.rs/) and [`std` ↗](https://doc.rust-lang.org/std/) |

The two that get confused are the first two. A book *argues*; a reference *defines*. "Struct fields are laid out in declaration order" is a reasonable-sounding sentence from a tutorial and [explicitly not guaranteed ↗](https://doc.rust-lang.org/nomicon/repr-rust.html) by the language — no third tutorial would have caught that, and one line of the Nomicon does.

## The shelves

| Page | For |
|---|---|
| [Books](books/README.md) | reading — every Rust book worth naming, with a verdict on each and what the verdict is based on |
| [Official docs](official_docs/README.md) | the definitive answer — `std`, the Reference, the Nomicon, the error index, the RFCs |
| [Exercises](exercises/README.md) | typing — the practice tracks, and which one suits which stage |
| [Going deeper](going_deeper/README.md) | a specific domain — unsafe, embedded, async, macros, testing, performance |
| [Why Haskell](haskell/README.md) | curiosity — what Rust borrowed, renamed, and deliberately left behind |
| [Structs](structs/README.md) | one topic — the checked shelf for structs: the chapter, the normative sources, the videos worth the time |

**Some sections keep their own reading list** next to the lessons, because the sources for one subject belong beside it: [Traits](../12_Traits/resources/README.md) and [Strings](../14_Strings/resources/README.md) both do. [Structs](structs/README.md) is the exception that sits here, because it is the topic where the free material is thickest and the quality spread widest.

## Where to start, honestly

| If you are… | Read |
|---|---|
| **starting from zero** | [**Start here**](../00_Start_Here/README.md) — the plan, and the three resources it is built on |
| starting from zero, and prefer slides to prose | [Comprehensive Rust ↗](https://google.github.io/comprehensive-rust/) — Google's four-day course, and unusually good |
| fighting the borrow checker | the [Brown University edition ↗](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html) of The Book — same text, with interactive quizzes that catch the misunderstanding |
| coming from C | [Rust for C Programmers ↗](https://rust-for-c-programmers.salewskis.de/ch1/chapter_1_rust_for_c_programmers.html), then [Learn Rust the Dangerous Way ↗](https://cliffle.com/p/dangerust/1/) |
| coming from Python or ABAP | this library — every lesson has an *"If you are coming from another language"* section for exactly that |
| wanting to type rather than read | [rustlings](../00_Start_Here/rustlings/README.md) first, then [100 Exercises ↗](https://rust-exercises.com/100-exercises/) |
| stuck on lifetimes specifically | [quinedot's guide ↗](https://quinedot.github.io/rust-learning/) — the best free treatment of the topic |
| deciding whether a book is worth buying | [Books](books/README.md) — and the short answer is usually *no, the free one is better* |
| looking something up mid-edit | [cheats.rs ↗](https://cheats.rs/) |

## Three notes on this shelf

**Nearly everything here is free, and the paid books are marked.** The whole free shelf — first book, ownership, lifetimes, concurrency, idioms, performance — is linked from [Books](books/README.md), which also reviews what is in print and says plainly which titles are worth paying for. Every link on every shelf page was checked on the date that page states. Dead links are named rather than quietly deleted: a shelf that only lists what works is a shelf you cannot trust about anything else.

**A verdict is not a checked fact, and the pages say which is which.** Every other page in this library is backed by [a program that prints the answer](../CONTRIBUTING.md). A book review cannot be. So the review pages separate the two explicitly — publication facts fetched from the publisher, verdicts marked as judgment, and *"unvetted"* used where the honest answer is that nobody here has read it.

**The Google Docs from the original list are not linked here.** They are private documents; a link to one is a 404 for every reader of this site and a leaked document id besides. Their *contents* — which is what mattered — are what this section is made of.

## And when a resource is not the answer

The failure mode this section exists to prevent: reading a fourth explanation of ownership instead of writing a program that owns something. The Book's chapter 4 plus one afternoon of the compiler saying no will teach you more than the other three combined.

If you have read the chapter and it has not landed, the fix is usually a *different kind* of resource rather than a better one — an exercise instead of a chapter, or a video instead of prose — which is why the tables above sort by situation rather than by subject.
