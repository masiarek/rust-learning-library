# Why Haskell, for Rust's sake

**Level:** reference · a side quest

**One line:** Rust borrowed a great deal from Haskell and then removed the vocabulary — so learning a little Haskell is not a detour into another language, it is reading the **design notes** for features you already use, with the names put back on.

Placed in [Resources](../README.md) rather than given a section of its own on purpose: this is optional, and a page that tells you to go learn a second language should be easy to walk past.

## The honest answer first

**You do not need Haskell to be good at Rust.** Plenty of excellent Rust programmers have never written a line of it, and nothing in this library requires it.

What Haskell buys is a *specific* kind of understanding: it makes explicit the ideas Rust deliberately hid. Rust's designers made a considered choice to ship the machinery without the terminology — no `Monad` trait, no `Functor`, `map` documented as "apply a function" rather than as anything categorical. That choice is why Rust is learnable. It is also why some Rust features feel arbitrary: you are using the answer to a question nobody told you was being asked.

A few weeks of Haskell tells you what the question was.

## What Rust took, and what it renamed

| Haskell | Rust | Renamed, removed, or restricted |
|---|---|---|
| `Maybe a` / `Just` / `Nothing` | `Option<T>` / `Some` / `None` | renamed |
| `Either e a` / `Right` / `Left` | `Result<T, E>` / `Ok` / `Err` | renamed, and the error side got a purpose |
| type classes | traits | renamed; Rust adds coherence rules Haskell does not have |
| `>>=` (bind) | `and_then` | renamed, and *not* abstracted — there is no `Monad` trait |
| `do` notation | `?` | restricted to `Try` types |
| `fmap` | `map` | renamed |
| algebraic data types + exhaustive `case` | `enum` + exhaustive `match` | taken almost unchanged |
| immutability by default | `let` without `mut` | taken, with an escape hatch |
| laziness by default | iterators, and nothing else | **deliberately rejected** |
| garbage collection | ownership and borrowing | **replaced** — this is where Rust leaves |

The top half is a rename. The bottom two rows are where Rust stops being Haskell-with-braces and becomes its own language, and knowing which is which is most of the value.

Look at the slide that prompted this page:

```haskell
divide :: Double -> Double -> Maybe Double
divide x 0 = Nothing
divide x y = Just (x / y)
```

```rust
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 { None } else { Some(x / y) }
}
```

Same program. Rust's is a translation, and the type signature is the same claim in both: *this function does not always have an answer, and the type says so.*

## The four things it actually teaches

**1. Why `Option` is a type and not a null.** In Haskell there is no alternative — there is no null to fall back to, so every absence is in a type and you spend a week doing nothing but composing `Maybe`s. That week makes `Option` obvious rather than annoying.

**2. What a monad is, with the name attached.** Rust gives you `and_then` on `Option`, on `Result`, on iterators, and never says these are the same thing. Haskell says it out loud, gives it a trait, and makes you write one. Afterwards, [`?`](../../01_Foundations/what_a_monad_is/README.md) reads as syntax for a thing you understand rather than a magic operator.

**3. Where traits came from, and what Rust changed.** Traits *are* type classes, with one significant addition: coherence — Rust's orphan rule, which Haskell lacks. Seeing type classes without that restriction is the fastest way to understand why Rust has it.

**4. What programming without mutation feels like.** Not as ideology — as calibration. Haskell makes mutation the awkward path and you find out how much of your usual state was habit. You come back writing more iterator chains and fewer `mut` accumulators, which is better Rust regardless of what you think of Haskell.

## And the one thing not to take from it

**Laziness.** Haskell evaluates by default only when forced; Rust is strict everywhere except iterator adaptors. Haskell's laziness makes some beautiful things possible (infinite lists) and makes performance genuinely hard to reason about — space leaks are a Haskell-specific art form. Rust looked at it and said no. Do not go looking for it here.

Similarly, do not import the point-free style. `f . g . h` is idiomatic Haskell; the Rust equivalent usually reads worse than the closure it replaced.

## How much is enough

**A few weekends.** The goal is not fluency, and you will get most of the value from the first third of one book:

- **[Learn You a Haskell for Great Good!](https://learnyouahaskell.github.io/)** (Miran Lipovača) — free online, famously friendly, and the standard first book. Chapters on types, type classes, `Maybe`, and the Functor/Applicative/Monad sequence are exactly the material that pays back in Rust. Stop after monads; you have what you came for. *(The [community-maintained edition](https://learnyouahaskell.github.io/) is the one to read — the original is old and unmaintained.)*
- **Real World Haskell** (O'Sullivan, Goerzen, Stewart) — the practical counterpart, and a cautionary tale about link rot: `realworldhaskell.org` **no longer resolves at all** (checked 2026-08-23; the domain is gone, not merely the page). What survives is the community's [up-to-date fork](https://github.com/tssm/up-to-date-real-world-haskell), which revises the 2008 text for modern GHC. Treat it as a second book, for the "how would you actually build something" question — and skip it entirely if the first one did its job.

You need `ghci` and nothing else — `brew install ghc` or [GHCup](https://www.haskell.org/ghcup/) — and most of the learning happens in the REPL rather than in projects.

## Is it worth it *for you*?

- **Yes, if** `?` and `and_then` feel like incantations you have memorised, or you enjoy knowing why a design is the way it is. This is a curiosity investment and it pays in understanding, not productivity.
- **Not yet, if** you are still fighting the borrow checker. Ownership is Rust's genuinely novel idea and Haskell has nothing to say about it — it solves the same memory problem with a garbage collector. Time spent on Haskell now is time not spent on the one part Haskell cannot help with.
- **Cheaper alternative:** read [What a monad is](../../01_Foundations/what_a_monad_is/README.md) here, which puts the names back on Rust's own machinery in one page. If that leaves you wanting the real thing, take the side quest.

## See also

- [What a monad is](../../01_Foundations/what_a_monad_is/README.md) — the same idea without leaving Rust
- [`Option` vs `Result`](../../01_Foundations/option_vs_result/README.md) — `Maybe` and `Either`, renamed and put to work
- [Books](../books/README.md) — the Rust shelf
