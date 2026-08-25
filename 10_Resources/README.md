# Resources

**Level:** reference · the map

**One line:** A curated shelf rather than a link dump — every entry says *when it is the right one*, because the problem with Rust learning material is not that there is too little of it.

There is an enormous amount of good, free Rust material, almost all of it excellent, and that is precisely the difficulty: at any given moment roughly two of these are the right thing to read and the rest are a way of avoiding the one you need. So each page here is organised by **the moment you are in** rather than by topic — with one deliberate exception, [Structs](structs/README.md), because that is the topic where the material is thickest and the quality spread widest, so the choice is worth making once.

| Page | For |
|---|---|
| [Books](books/README.md) | reading — the long-form explanations, from first day to specialist |
| [Exercises](exercises/README.md) | typing — the practice tracks, and which one suits which stage |
| [Going deeper](going_deeper/README.md) | a specific domain — unsafe, embedded, async, macros, testing, performance |
| [Why Haskell](haskell/README.md) | curiosity — what Rust borrowed, renamed, and deliberately left behind |
| [Structs](structs/README.md) | one topic — the checked shelf for structs: the chapter, the normative sources, the videos worth the time |

## Where to start, honestly

| If you are… | Read |
|---|---|
| **starting from zero** | [**Start here**](../00_Start_Here/README.md) — the plan, and the three resources it is built on |
| starting from zero, and prefer slides to prose | [Comprehensive Rust ↗](https://google.github.io/comprehensive-rust/) — Google's four-day course, and unusually good |
| fighting the borrow checker | the [Brown University edition ↗](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html) of The Book — same text, with interactive quizzes that catch the misunderstanding |
| coming from C | [Rust for C Programmers ↗](https://rust-for-c-programmers.salewskis.de/ch1/chapter_1_rust_for_c_programmers.html), then [Learn Rust the Dangerous Way ↗](https://cliffle.com/p/dangerust/1/) |
| coming from Python or ABAP | this library — every lesson has an *"If you are coming from another language"* section for exactly that |
| wanting to type rather than read | [rustlings](../00_Start_Here/rustlings/README.md) first, then [100 Exercises ↗](https://github.com/mainmatter/100-exercises-to-learn-rust) |
| stuck on lifetimes specifically | [quinedot's guide ↗](https://quinedot.github.io/rust-learning/) — the best free treatment of the topic |
| looking something up mid-edit | [cheats.rs ↗](https://cheats.rs/) |

## Two notes on this shelf

**Everything listed is free and online.** No entry requires a purchase, and every link was checked and returned 200 on 2026-08-23. One well-known guide was dropped for being dead — Michael Bryan's Rust FFI guide, which now 404s; the [Nomicon's FFI chapter ↗](https://doc.rust-lang.org/nomicon/ffi.html) covers the same ground and is maintained.

**The Google Docs from the original list are not linked here.** They are private documents; a link to one is a 404 for every reader of this site and a leaked document id besides. Their *contents* — which is what mattered — are what this section is made of.

## And when a resource is not the answer

The failure mode this section exists to prevent: reading a fourth explanation of ownership instead of writing a program that owns something. The Book's chapter 4 plus one afternoon of the compiler saying no will teach you more than the other three combined.

If you have read the chapter and it has not landed, the fix is usually a *different kind* of resource rather than a better one — an exercise instead of a chapter, or a video instead of prose — which is why the table above sorts by situation.
