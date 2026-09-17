# Reading on borrowing forever and self-referential structs

Beside [Borrowing something forever](../borrowing_forever/README.md) and [A struct that points into itself](../self_referential_structs/README.md)

**Level:** reference · the reading list

**One line:** Where `&'a mut Thing<'a>`, variance and structs that borrow from themselves are explained at length: quinedot's three pages, the Nomicon and the Reference on variance, Jon Gjengset's `strtok` stream with timestamps, the Stack Overflow answer everyone is sent to, two book chapters and the crates, with what to read carefully in each.

Every entry was checked on 2026-09-16. Every link resolved and each title is the page's own. Book sections are copied from the publisher's table of contents (O'Reilly's, for both books), so "covers" means a section is titled for the topic, not that every paragraph was read. Video timestamps are the chapter marks in the video's own description. A video without them gets none here.

## If you read only three

- **[quinedot, *Learning Rust* ↗](https://quinedot.github.io/rust-learning/)** → *Practical suggestions for building intuition around borrow errors* → [*Borrowing something forever* ↗](https://quinedot.github.io/rust-learning/pf-borrow-forever.html) and [*Avoid self-referential structs* ↗](https://quinedot.github.io/rust-learning/pf-meta.html#avoid-self-referential-structs). Short, exact, and the source both lessons check claim by claim. One sentence does not survive running it (below).
- **[Jon Gjengset, *Crust of Rust: Subtyping and Variance* ↗](https://www.youtube.com/watch?v=iVYWDIW71jk)** (2021, 1 h 39 min). Going by its chapter list, its first hour builds a `strtok` whose test will not compile until its lifetimes are separated, the same bug as [Borrowing something forever](../borrowing_forever/README.md), found live. Chapters worth jumping to: 0:13:00 *Why can't we call strtok?*, 0:42:14 *Invariance*, 0:50:00 *&'a mut T covariance in 'a*, 0:57:57 *What went wrong in our strtok test?*, 1:02:24 *Fixing strtok*, 1:07:34 *Why is 'b: 'a not needed?*
- **[Stack Overflow: *Why can't I store a value and a reference to that value in the same struct?* ↗](https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct)** The question a self-referential struct gets closed as a duplicate of (question score 462, accepted answer 525, when checked). The accepted answer explains what a move does to the address, then has sections *How do I fix it?*, *A type with a reference to itself* and *What about Pin?*.

## For borrowing forever

- [quinedot — *Borrowing something forever* ↗](https://quinedot.github.io/rust-learning/pf-borrow-forever.html): `&'a mut Thing<'a>`, the four things you can no longer do, and the destructor case. Every claim holds on 1.98.0.
- [quinedot — *`&'a Struct<'a>` and covariance* ↗](https://quinedot.github.io/rust-learning/pf-shared-nested.html): the shared version, harmless when the struct is covariant. [The lesson](../borrowing_forever/README.md#what-still-works) runs both the covariant case and a `Cell` one that locks.
- [The Rustonomicon — *Subtyping and Variance* ↗](https://doc.rust-lang.org/nomicon/subtyping.html): why `&mut T` has to be invariant in `T`, with the use-after-free you would get otherwise.
- [The Reference — *Subtyping and variance* ↗](https://doc.rust-lang.org/reference/subtyping.html): the table of every type constructor's variance, the binding version.
- [rustc error index — `E0499` ↗](https://doc.rust-lang.org/error_codes/E0499.html), [`E0502` ↗](https://doc.rust-lang.org/error_codes/E0502.html), [`E0505` ↗](https://doc.rust-lang.org/error_codes/E0505.html), [`E0597` ↗](https://doc.rust-lang.org/error_codes/E0597.html): the long explanations of the errors this pattern produces, also printed by `rustc --explain`.

## For self-referential structs

- [quinedot — *Avoid self-referential structs* ↗](https://quinedot.github.io/rust-learning/pf-meta.html#avoid-self-referential-structs): the `Snek` and `bite(&'a mut self)` that [the lesson](../self_referential_structs/README.md) runs.
- [fasterthanlime — *Self-referential structs (in Rust)* ↗](https://www.youtube.com/watch?v=xNrglKGi-7o) (2021, 27 min). Also on the [structs resources page](../../10_Resources/structs/README.md), which says to watch it after lifetimes. Its description has no chapter marks, so there are no timestamps here.
- [`std::pin` — *A self-referential struct* ↗](https://doc.rust-lang.org/std/pin/index.html#a-self-referential-struct): self-referential structs as the simplest address-sensitive type, with an `Unmovable` whose `slice` points into its own `data`, and why compiler-generated futures need the same promise.
- [The Rust Programming Language, ch. 10.3 — *Lifetime Annotations in Struct Definitions* ↗](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions): the book's `struct ImportantExcerpt<'a>`, a struct borrowing from data *outside* it, which is the shape that works.
- ***Programming Rust*, 2nd ed.** (Blandy, Orendorff & Tindall, O'Reilly 2021): ch. 5 "References" → "Reference Safety" → **"Structs Containing References"** and **"Distinct Lifetime Parameters"**, which is the two-names fix. Also ch. 20 "Asynchronous Programming" → **"Pinning"** → "Pinned Pointers".
- ***Asynchronous Programming in Rust*** (Carl Fredrik Samson, Packt): ch. 9 **"Coroutines, Self-Referential Structs, and Pinning"** → "Discovering self-referential structs" → **"What is a move?"**, then "Pinning in Rust" (in theory, to the heap, to the stack, projections). This is where a self-referential struct stops being a newcomer's mistake and becomes something the compiler generates for every `async fn`.

## Crates that build one for you

Listed, not run on these pages. Each generates the `unsafe` code that stores an owner and a reference into its heap data together. Versions are from crates.io on 2026-09-16.

- [`ouroboros` ↗](https://docs.rs/ouroboros/latest/ouroboros/) (0.18.5): "A crate for creating safe self-referencing structs." An attribute macro, `#[self_referencing]`, on the struct.
- [`self_cell` ↗](https://docs.rs/self_cell/latest/self_cell/) (1.3.0): one `macro_rules!` macro, no procedural macro, and the dependent type must be marked `covariant` or `not_covariant`, the same variance these lessons are about.
- [`yoke` ↗](https://docs.rs/yoke/latest/yoke/) (0.8.3): attaches a zero-copy deserialized value, such as a `Cow<'a, str>`, to the buffer it came from, so the pair can be moved around together.

## Read with care

- **quinedot's *Avoid self-referential structs*, first claim**, that taking a `&'a mut Snek<'a>` is the only safe way to build one. Run, it is false: an in-place `snek.borrowed = &snek.owned` and a `Cell` with `&'a self` both build one ([claims, run](../self_referential_structs/README.md#quinedots-claims-run)). The guide's conclusion, that it is useless, still holds.
- **The same page's reborrow.** That the struct stays usable only through a reborrow returned from the method is true, but the reborrow must be shared. `-> &'a mut Self` does not compile ([entry 9](../borrowing_forever_errors/README.md#9-bite-handing-back-a-mut-self)).
- **"You can't do that in safe Rust."** The standard forum answer, and an approximation, as quinedot says. You can build one. You cannot return one from a constructor, move it or mutate it.
- **The crates have had soundness bugs.** [RUSTSEC-2023-0042 ↗](https://rustsec.org/advisories/RUSTSEC-2023-0042.html): "Ouroboros is Unsound", fixed in 0.16.0. [RUSTSEC-2023-0070 ↗](https://rustsec.org/advisories/RUSTSEC-2023-0070.html): "Insufficient covariance check makes self_cell unsound", fixed in 1.0.2 and 0.10.3. Both were fixed, and both show how hard the `unsafe` code behind a self-referential struct is to get right. That is the argument for a range whenever a range will do.

## Inside this library

- [Borrowing something forever](../borrowing_forever/README.md) and [A struct that points into itself](../self_referential_structs/README.md), the two lessons
- [Every borrowed-forever error, and its fix](../borrowing_forever_errors/README.md) and [Lints around borrowing forever](../borrowing_forever_lints/README.md), their companions
- [What `&'a T` claims](../what_a_reference_claims/README.md#the-direction-reverses-behind-mut) — variance, the idea underneath both lessons
- [How to learn lifetimes](../how_to_learn_lifetimes/README.md#practice) — the zero-copy log kata, where this wall first shows up
- [Books](../../10_Resources/books/README.md) — quinedot's guide in the library's general reading list

## Po polsku

Lista lektur do dwóch lekcji: o `&'a mut Thing<'a>` (pożyczanie na zawsze) i o strukturach samoreferencyjnych. Najkrócej: trzy strony quinedota, wykład Jona Gjengseta o wariancji (z konkretnymi znacznikami czasu), odpowiedź ze Stack Overflow, do której odsyła się każde takie pytanie, oraz rozdziały z *Programming Rust* i z książki Samsona o programowaniu asynchronicznym.

Czytaj uważnie: jedno zdanie quinedota, że jedyną bezpieczną drogą jest `&'a mut Snek<'a>`, po uruchomieniu okazuje się fałszywe. Crate'y `ouroboros` i `self_cell` miały poważne błędy (*unsoundness*), już poprawione, co dobrze pokazuje, jak trudny jest kod `unsafe` stojący za takimi strukturami, i przemawia za zakresem (`Range<usize>`) wszędzie, gdzie wystarczy.

**Szukaj po polsku:** wariancja w Ruscie · struktura samoreferencyjna · `Pin` w Ruscie · `rust variance` · `rust self-referential struct crate`
