# Traits: links and videos

**Level:** reference · the reading list

**One line:** The sources behind [this section](../README.md) — the normative ones first, then the essays worth the detour, then the videos.

**The same page for strings is [Strings: links, books and videos](../../14_Strings/resources/README.md)** — the two sections lean on each other, and `ToOwned` and `Display` are why.

## Official

- [The Reference — Items: traits ↗](https://doc.rust-lang.org/reference/items/traits.html) — the normative definition. Terse, and the only one of these that is *binding*: when two explanations disagree, this is the one that is right.
- [The Book, ch. 10 — Traits: defining shared behaviour ↗](https://doc.rust-lang.org/book/ch10-02-traits.html) — declaration, `impl … for`, default method bodies, trait bounds, `where`.
- [The Book, ch. 17 — Trait objects ↗](https://doc.rust-lang.org/book/ch17-02-trait-objects.html) — `dyn`, the vtable, and dynamic dispatch.
- [The Book, ch. 19 — the newtype pattern for external traits on external types ↗](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types) — the sanctioned way round the orphan rule.
- [Rust by Example — traits ↗](https://doc.rust-lang.org/rust-by-example/trait.html) — the smallest runnable version of the idea.
- [`std` keyword docs — `dyn` ↗](https://doc.rust-lang.org/std/keyword.dyn.html) — the two pointers a `dyn Trait` reference carries, the object-safety requirement, and the edition differences (before 2021 the keyword could be omitted; in 2015 a leading `::` was parsed as part of the path).
- [Edition guide — `dyn Trait` for trait objects ↗](https://doc.rust-lang.org/edition-guide/rust-2018/new-keywords.html#dyn-trait-for-trait-objects) — why the keyword exists at all: bare `Trait` as a type was ambiguous, and 2018 made the dynamic case say so.
- [Announcing `impl Trait` capture rules (2024) ↗](https://blog.rust-lang.org/2024/09/05/impl-trait-capture-rules.html) — what changed in edition 2024 about which lifetimes an `impl Trait` return captures.

## Essays and write-ups

- [Traits: a blog post by Aaron Turon (2015) ↗](https://blog.rust-lang.org/2015/05/11/traits.html) — the design rationale from the person who built it: why traits rather than interfaces, what coherence buys, and where the zero-cost claim comes from. Old, and still the best single explanation of *why*.
- [`dyn Trait` — quinedot's Rust learning notes ↗](https://quinedot.github.io/rust-learning/dyn-trait.html) — the long, careful treatment of trait objects: lifetimes, object safety, what a vtable actually contains, and the cases the Book skips. The best single page on `dyn` anywhere.
- [Rust traits: a deep dive ↗](https://blog.logrocket.com/rust-traits-a-deep-dive/) — long-form tour of the whole surface.
- [What are traits in Rust? ↗](https://www.educative.io/answers/what-are-traits-in-rust) — a short answer page, useful as a refresher.
- [A half-hour to learn Rust ↗](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) — not a traits article; the whirlwind tour that puts traits in context with everything around them.
- [Easy Rust — ch. 34, traits ↗](https://dhghomon.github.io/easy_rust/Chapter_34.html) — the written companion to the Easy Rust videos below.
- [When Rust hurts ↗](https://mmapped.blog/posts/15-when-rust-hurts.html) — the critical piece: where the trait system makes a design harder rather than easier. Worth reading precisely because it is not advocacy.
- [Felix Knorr — traits (2023) ↗](https://felix-knorr.net/posts/2023-04-17-traits.html)
- [My Rust journey: traits ↗](https://dev.to/henrybarreto/my-rust-journey-traits-4lpc) — one person's write-up.
- [Pythonista to Rustacean, part 1 ↗](https://algorithmicallysound.com/2020/06/18/pythonista-to-rustacean-1/) — flagged as unclear when it was collected; read it as a lead, not as an authority.
- [Advanced traits — practice.rs ↗](https://practice.rs/generics-traits/advanced-traits.html) — exercises rather than prose.

## The compiler's own test cases

- [`src/test/ui/traits` at `673d0db` ↗](https://github.com/rust-lang/rust/tree/673d0db5e393e9c64897005b470bfeb6d5aec61b/src/test/ui/traits) — every trait behaviour rustc guarantees, written as the smallest program that exercises it, each with its expected diagnostic beside it. The best answer to "does this compile, and what exactly does it say when it doesn't". Note the URL is pinned to a commit on purpose: the directory has since moved to `tests/ui/traits`, so the pin is what keeps the link alive.

## Video

One with a URL:

- [Generic traits, impls, and slices in Rust ↗](https://youtu.be/ykQbsTHqKFo)

The rest are offline copies, listed by title so they can be found again:

- *Rust traits vs C++ concepts*
- *Designing interfaces* — Rust for Rustaceans book club, ch. 3
- *Demo: traits* [34 of 35] — Rust for Beginners
- *Method overloading (kinda), and advanced trait usage* — Quick Rust
- *Easy Rust 063–070, 072–073* — the traits arc: introduction, reading an implementation, implementing `Display`, a longer trait, traits built on other traits, trait bounds, understanding `From`, implementing `From` (twice), and `AsRef`
- *Easy Rust 071 — type aliases and new types* — sits inside that run but is a different topic; the newtype half is what makes it belong ([a score is not a number](../../16_Structs/newtype_score/README.md) covers the same ground)
