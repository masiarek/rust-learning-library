# Helpful resources for references

[References](../README.md) › **Reading list**

**Level:** reference · the reading list

**One line:** For each part of the references path, the book chapters, docs, papers and articles that cover the same idea — with chapter *names* as well as numbers, because numbers move between editions — and the sources to read with care, each with what is wrong in it.

Every link was checked on 2026-09-16. A print book is cited only by the chapter and section headings its own pages show, so "covers" means a section is titled for the idea, not that every paragraph was read.

## If you read only four

- **[The Rust Programming Language, ch. 4.2 — References and Borrowing ↗](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)** — `&`, `&mut`, the two rules, and the dangling reference the compiler refuses. The page everyone starts from.
- ***Programming Rust*, 2nd ed.** (Blandy, Orendorff & Tindall, O'Reilly 2021) — **ch. 5 "References"**, in particular **"Reference Safety"** and its first case, **"Borrowing a Local Variable"**: how the compiler assigns each reference a lifetime and works out the constraints between them. The deepest book treatment of *why* a reference program is refused.
- **[*Stacked Borrows: An Aliasing Model for Rust* ↗](https://plv.mpi-sws.org/rustbelt/stacked-borrows/)** (Jung, Dang, Kang & Dreyer, POPL 2020), **§2 "An Introduction to Rust"** — four pages that state the exclusion principle and explain borrow checking as loans with lifetimes. No formal background needed for §2; §3 onwards is the model for `unsafe` code. [A borrow is a loan](../a_borrow_is_a_loan/README.md) runs its programs.
- **[`reference` ↗](https://doc.rust-lang.org/std/primitive.reference.html)** and **[`pointer` ↗](https://doc.rust-lang.org/std/primitive.pointer.html)** — the std pages for `&T`/`&mut T` and `*const T`/`*mut T`: which traits every reference implements, and the list of things a raw pointer promises nothing about.

## By part of the path

### Borrowing, loans and lifetimes

- [TRPL ch. 4.2 — References and Borrowing ↗](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) — Listing 4-6 is the `E0596` of writing through a `&`
- *Programming Rust*, 2nd ed., ch. 5 "References" → "Reference Safety" → "Borrowing a Local Variable" — the `E0597` example [A borrow is a loan](../a_borrow_is_a_loan/README.md) takes as its kata
- [*Stacked Borrows* paper (PDF) ↗](https://plv.mpi-sws.org/rustbelt/stacked-borrows/paper.pdf) — §2 for loans, reborrowing and shared references; §5 "Supporting Interior Mutability" for why `Cell` is the exception
- [Rustonomicon — References ↗](https://doc.rust-lang.org/nomicon/references.html) and [Aliasing ↗](https://doc.rust-lang.org/nomicon/aliasing.html) — the rules again, from the side of someone writing `unsafe` code, and the optimization aliasing information makes sound
- [*Learning Rust With Entirely Too Many Linked Lists* — Stacked Borrows ↗](https://rust-unofficial.github.io/too-many-lists/fifth-stacked-borrows.html) — the same model met while writing a queue with raw pointers, with Miri reporting each violation
- [Ralf Jung, *Stacked Borrows Implemented* (2018) ↗](https://www.ralfj.de/blog/2018/11/16/stacked-borrows-implementation.html) — the blog post before the paper, with the rules tried out in Miri

### Pointer types, `Box` and `Rc`

- [The Reference — Pointer types ↗](https://doc.rust-lang.org/reference/types/pointer.html) — shared and mutable references, raw pointers, smart pointers: the normative list
- [TRPL ch. 15 — Smart Pointers ↗](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html), [15.2 — Treating Smart Pointers Like Regular References ↗](https://doc.rust-lang.org/book/ch15-02-deref.html) and [15.4 — `Rc<T>`, the Reference Counted Smart Pointer ↗](https://doc.rust-lang.org/book/ch15-04-rc.html) — the cons list shared by two lists, and the counts 1, 2, 3, 2
- [Effective Rust, Item 8 — Familiarize yourself with reference and pointer types ↗](https://effective-rust.com/references.html) — references, pointer traits, fat pointers, and smart pointer types, in that order
- [Stack Overflow — Understanding and relationship between `Box`, `ref`, `&` and `*` ↗](https://stackoverflow.com/questions/31949579/understanding-and-relationship-between-box-ref-and) — see [Read with care](#read-with-care) before trusting any snippet in it
- *Rust in Action* (Tim McNamara, Manning 2021), ch. 6 "Memory", §6.1–6.2 — pointers, references and raw pointers in the book's vocabulary; seventeen of its claims are run in [*Rust in Action*, chapter 6](../../../36_Pointers/rust_in_action_chapter_6/README.md): right on the three-way split, wrong on alignment, the second word of a wide pointer and the size of `Option`, and one listing is undefined behaviour
- [Ralf Jung, *Pointers Are Complicated II* (2020) ↗](https://www.ralfj.de/blog/2020/12/14/provenance.html) — provenance: why two pointers to the same address are not interchangeable to an optimizing compiler

### `ref` and patterns

- [`ref` keyword ↗](https://doc.rust-lang.org/std/keyword.ref.html) — two examples, one refused and one fixed, and the one-line difference between `ref` and `&` in a pattern
- [Rust By Example — The ref pattern ↗](https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html)
- [The Reference — Patterns, binding modes ↗](https://doc.rust-lang.org/reference/patterns.html#binding-modes) — the default binding mode that made `ref` rare
- [RFC 2005 — match ergonomics ↗](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html)

### Dereferencing

- [TRPL ch. 15.2 — Treating Smart Pointers Like Regular References ↗](https://doc.rust-lang.org/book/ch15-02-deref.html) — `*` on a reference, on a `Box`, and through `Deref`
- [`Deref` ↗](https://doc.rust-lang.org/std/ops/trait.Deref.html) — deref coercion, and its three-part test for when a type should implement `Deref` at all

### Raw pointers and `unsafe`

- [Cliffle, *Learn Rust the Dangerous Way*, part 2 — References available upon request ↗](https://cliffle.com/p/dangerust/2/) — a C program's raw-pointer parameter turned into `&mut [Body; N]`, and the list of bugs that change rules out; see [Read with care](#read-with-care) for the one line that no longer compiles
- [TRPL ch. 20.1 — Dereferencing a Raw Pointer ↗](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#dereferencing-a-raw-pointer)
- [`std::ptr` ↗](https://doc.rust-lang.org/std/ptr/index.html) — `ptr::read`, `ptr::write`, and the safety section every raw-pointer function points back to
- [Edition guide — Disallow references to `static mut` ↗](https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html)

### Writing through a shared reference

- [`std::cell` ↗](https://doc.rust-lang.org/std/cell/index.html) — "Shareable mutable containers": why `Cell` and `RefCell` can be written through a `&`
- [TRPL ch. 15.5 — `RefCell<T>` and the Interior Mutability Pattern ↗](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

## Videos

Titles and channels checked through YouTube's oEmbed endpoint on 2026-09-16; the timestamps are the authors' own chapter marks from each video's description, not a transcript read here.

- **[Jon Gjengset, *Crust of Rust: Lifetime Annotations* ↗](https://www.youtube.com/watch?v=rAl-9HwD858)** (April 2020, 93 min) — a string splitter that holds a `&'a str` and moves it forward, which is [Re-pointing a slice](../repointing_a_slice/README.md) built live. Chapters: [17:10 — missing lifetime specifier ↗](https://www.youtube.com/watch?v=rAl-9HwD858&t=1030s) · [48:07 — what is the `ref` keyword and why not `&` ↗](https://www.youtube.com/watch?v=rAl-9HwD858&t=2887s) · [51:36 — the `*` on the left of `remainder` ↗](https://www.youtube.com/watch?v=rAl-9HwD858&t=3096s) · [54:48 — mutable references are one level deep ↗](https://www.youtube.com/watch?v=rAl-9HwD858&t=3288s). Recorded on a 2020 toolchain; the edition-2024 rule about `ref` under a borrowing default mode is on [The `ref` keyword](../the_ref_keyword/README.md#edition-2024-ref-where-the-borrow-is-already-implied).
- **[Jon Gjengset, *Crust of Rust: Smart Pointers and Interior Mutability* ↗](https://www.youtube.com/watch?v=8O0Nt9qY_vo)** (June 2020, 123 min) — `Cell`, `RefCell` and `Rc` re-implemented from scratch. Chapters: [3:50 — interior mutability ↗](https://www.youtube.com/watch?v=8O0Nt9qY_vo&t=230s) · [41:21 — `RefCell` ↗](https://www.youtube.com/watch?v=8O0Nt9qY_vo&t=2481s) · [1:06:27 — `Rc` ↗](https://www.youtube.com/watch?v=8O0Nt9qY_vo&t=3987s) · [1:54:20 — `Cow` ↗](https://www.youtube.com/watch?v=8O0Nt9qY_vo&t=6860s). Beside [Six pointer types, one table](../pointer_types_compared/README.md) and [Interior mutability](../../../09_Advanced/interior_mutability/README.md).
- **[Ralf Jung, *Stacked Borrows: An Aliasing Model for Rust* ↗](https://www.youtube.com/watch?v=h9Fh4jRDGLo)** (ACM SIGPLAN, POPL 2020, 21 min, no chapters) — the conference talk for the paper [A borrow is a loan](../a_borrow_is_a_loan/README.md) takes its programs from.

## Read with care

Good sources, each with something that does not compile or no longer holds on rustc 1.98.0. The evidence for each is on [What reference explanations get wrong, run](../reference_claims_checked/README.md).

- **The Stack Overflow thread** (2015–2026). The accepted answer (2015) is still the best short overview of `Box`, `ref`, `&` and `*`, but its pointer snippet takes `&mut x` of a `let x` that is not `mut`, its `*mut SomeTrait` has been an error since edition 2021, and "`Box<T>` always takes a pointer's worth of bytes" is false for `Box<str>` and `Box<dyn Trait>`. The table answer (2022) mixes up where the pointer lives with where the data lives, and calls three non-owning pointers "shared ownership". Its last line, "`ref` is the same as `&`", equates two pattern forms that do opposite things.
- **The Book, ch. 4.2.** The `E0596` transcript for Listing 4-6 comes from an older rustc: 1.98.0 says *"so it cannot be borrowed as mutable"* and suggests the fix as a diff.
- **The Book, ch. 15.4.** "for reading only" describes `Rc<T>` through `Deref`; `Rc::get_mut` writes in place when the count is one, and `Rc::make_mut` clones first when it is not.
- ***Programming Rust*, 2nd ed.** "At run time, a reference is nothing but an address" holds for a reference to a sized type; a `&str` or `&[T]` also carries a length.
- **Cliffle, part 2** (written against rustc 1.39). `offset_Momentum(&mut solar_Bodies)` on a `static mut` is a warning in edition 2021 and an error in edition 2024.

## Po polsku

Lista lektur do ścieżki o referencjach. Najkrócej: rozdział 4.2 *The Rust Programming Language* (referencje i pożyczanie), rozdział 5 *Programming Rust* („Reference Safety”), sekcja 2 artykułu *Stacked Borrows* (zasada wykluczania i pożyczki z czasem życia) oraz strony `reference` i `pointer` w dokumentacji std. Sekcja „Read with care” wymienia źródła, które warto czytać, ale z poprawkami — wątek na Stack Overflow, fragmenty The Book, *Programming Rust* i kursu Cliffle'a — a dowody (wyniki kompilatora) są na stronie z twierdzeniami sprawdzonymi na rustc 1.98.0.

**Szukaj po polsku:** referencje w Ruście · pożyczanie · zasada wykluczania · `rust stacked borrows` · `rust ref keyword` · `rust raw pointer vs reference`
