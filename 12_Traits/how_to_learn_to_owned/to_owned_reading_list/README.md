# Helpful resources for the `ToOwned` path

[How to learn `ToOwned`](../README.md) › **Beside the steps** · start at: [Step 1](../clone_vs_to_owned/README.md)

**Level:** reference · the reading list

**One line:** For every step on the path, the chapters, sections and articles that cover the same idea — with the chapter *name* as well as its number, because numbers move between editions — plus the `Cow` material that sits beside steps 7 and 8, and three sources to read with care.

Every entry was checked on 2026-09-16. Free material links to the page itself. A print book gives its chapter and section title as the publisher's table of contents shows it — *Programming Rust* from O'Reilly's, *Rust for Rustaceans* from No Starch's "Contents in Detail" — so "covers" means the section is titled for it, not that every paragraph was read.

## If you read only four

- **[`std::borrow` ↗](https://doc.rust-lang.org/std/borrow/index.html)** — the module page for `Borrow`, `BorrowMut`, `ToOwned` and `Cow`. Short, normative, and the source every other entry paraphrases.
- ***Programming Rust*, 2nd ed.** (Blandy, Orendorff & Tindall, O'Reilly 2021) — **ch. 13 "Utility Traits"**, which has sections on `Sized`, `Clone`, `Copy`, `Deref`, `AsRef`, `Borrow` and `BorrowMut`, `From` and `Into`, `ToOwned`, and **"Borrow and ToOwned at Work: The Humble Cow"** (p. 323). The one book chapter that covers the whole path in order.
- **[The Rust Programming Language, ch. 20.3 — Dynamically Sized Types and the `Sized` Trait ↗](https://doc.rust-lang.org/book/ch20-03-advanced-types.html#dynamically-sized-types-and-the-sized-trait)** — step 2, from the book everyone has.
- **[The Reference — method call expressions ↗](https://doc.rust-lang.org/reference/expressions/method-call-expr.html#r-expr.method.candidate-receivers)** — step 5's candidate list, in the only text that is binding.

## Before step 1

- **[The Rust Programming Language, ch. 4.1 — What Is Ownership? ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)** — the layer under the whole path. [*Memory and Allocation* ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation) draws a `String` as a pointer, a length and a capacity with the bytes on the heap, and says a literal's text is "hardcoded directly into the final executable", which is where a `Cow::Borrowed` of a literal points. [*Variables and Data Interacting with Clone* ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone) is step 4's starting point, and its rule that Rust "will never automatically create 'deep' copies" is why every copy on this path is a call you can see. One simplification to carry forward carefully: [*Stack-Only Data: Copy* ↗](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy) introduces `Copy` for "types that are stored on the stack", but a `Copy` value inside a `Vec` lives on the heap and is still `Copy` — [`Copy` vs `Clone`](../../../16_Structs/copy_vs_clone/README.md) says what the trait actually promises.
- The library's [Ownership](../../../18_Ownership/README.md) section — especially [Stack and heap](../../../18_Ownership/stack_and_heap/README.md) and [Ownership and moves](../../../18_Ownership/ownership_and_moves/README.md).

## By step

### [1. `ToOwned` is `Clone` with a separate owned type](../clone_vs_to_owned/README.md)

- [`ToOwned` ↗](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html) — "a generalization of `Clone` to borrowed data", the associated type, and the implementors list
- [TRPL ch. 20.2 — Defining Traits with Associated Types ↗](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#defining-traits-with-associated-types)
- [The Reference — associated types ↗](https://doc.rust-lang.org/reference/items/associated-items.html#associated-types)
- [pretzelhammer, *Tour of Rust's Standard Library Traits* (2021) ↗](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md) — the "Associated Types", "Clone" and "ToOwned" sections
- *Programming Rust*, 2nd ed., ch. 13 → "Clone" and "ToOwned"; ch. 11 → "Associated Types (or How Iterators Work)"

### [2. Some types have no size](../types_with_no_size/README.md)

- [TRPL ch. 20.3 — Dynamically Sized Types and the `Sized` Trait ↗](https://doc.rust-lang.org/book/ch20-03-advanced-types.html#dynamically-sized-types-and-the-sized-trait)
- [The Reference — dynamically sized types ↗](https://doc.rust-lang.org/reference/dynamically-sized-types.html) and [pointer and reference layout ↗](https://doc.rust-lang.org/reference/type-layout.html#pointers-and-references-layout) — "twice the size"
- [Rustonomicon — exotically sized types ↗](https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts) — calls it a *wide pointer*, and builds a DST of its own
- [Effective Rust, Item 8 — fat pointer types ↗](https://effective-rust.com/references.html#fat-pointer-types)
- [pretzelhammer, *Sizedness in Rust* (2020) ↗](https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md)
- *Rust for Rustaceans* (Gjengset, No Starch 2021), ch. 2 "Types" → "Dynamically Sized Types and Wide Pointers" (p. 23)
- *Rust in Action* (McNamara, Manning 2021), ch. 6 "Memory" → §6.2 "Exploring Rust's reference and pointer types", whose note on p. 185 defines a fat pointer as "usually two `usize` wide" — the rest of that chapter's pointer claims are run in [*Rust in Action*, chapter 6](../../../36_Pointers/rust_in_action_chapter_6/README.md)
- Video: [Crust of Rust — Dispatch and Fat Pointers ↗](https://www.youtube.com/watch?v=xcygqF5LVmM) (Jon Gjengset, 2021) — *The Sized Trait* at 0:27:13, *Sizing Unsized Types* at 0:39:34, *Dynamically Sized Types* at 1:43:03

### [3. Owned and borrowed are two different types](../owned_and_borrowed_types/README.md)

- [TRPL ch. 15.2 — Using Deref Coercion in Functions and Methods ↗](https://doc.rust-lang.org/book/ch15-02-deref.html#using-deref-coercion-in-functions-and-methods), and ch. 4.3 on string slices as parameters
- [`Deref` — deref coercion ↗](https://doc.rust-lang.org/std/ops/trait.Deref.html#deref-coercion), and when to implement it at all
- [Rust Design Patterns — use borrowed types for arguments ↗](https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html) and [collections are smart pointers ↗](https://rust-unofficial.github.io/patterns/idioms/deref.html)
- [quinedot's learning notes — slice layout ↗](https://quinedot.github.io/rust-learning/misc-slice.html) — why `&[T]` rather than `&Vec<T>`, with diagrams
- *Programming Rust*, 2nd ed., ch. 13 → "Deref and DerefMut"; ch. 18 → "OsStr and Path"

### [4. `Clone` hands back `Self` — and every `&T` is `Clone`](../clone_returns_self/README.md)

- [`Copy` — what's the difference between `Copy` and `Clone`? ↗](https://doc.rust-lang.org/std/marker/trait.Copy.html) — including that `&T` is `Copy` whatever `T` is
- [`Clone::clone` ↗](https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone) — "for reference types like `&T`, this creates another reference to the same value"
- [The Reference — shared references are `Copy` ↗](https://doc.rust-lang.org/reference/types/pointer.html#shared-references-)
- [TRPL Appendix C — `Clone` and `Copy` for duplicating values ↗](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#clone-and-copy-for-duplicating-values)
- [Effective Rust, Item 10 — standard traits ↗](https://effective-rust.com/std-traits.html)
- *Programming Rust*, 2nd ed., ch. 4 → "Copy Types: The Exception to Moves"; ch. 13 → "Clone" and "Copy"

### [5. The dot takes the first receiver that fits](../the_dot_picks_first/README.md)

- [The Reference — candidate receiver types ↗](https://doc.rust-lang.org/reference/expressions/method-call-expr.html#r-expr.method.candidate-receivers), with a worked `Box<[i32; 2]>` example
- [Rustonomicon — the dot operator ↗](https://doc.rust-lang.org/nomicon/dot-operator.html) — including the `T: Clone` bound whose absence makes `.clone()` copy the reference
- [TRPL ch. 5.3 — where's the `->` operator? ↗](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator), automatic referencing in one paragraph
- [rustc dev guide — method lookup ↗](https://rustc-dev-guide.rust-lang.org/hir-typeck/method-lookup.html), the probe and confirm phases from inside the compiler
- [Stack Overflow — What are Rust's exact auto-dereferencing rules? ↗](https://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules)
- [dtolnay — autoref-based stable specialization ↗](https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md) — the lookup order used on purpose
- *Programming Rust*, 2nd ed., ch. 11 → "Fully Qualified Method Calls"

### [6. One blanket impl covers every `Clone` type](../the_blanket_to_owned/README.md)

- [TRPL ch. 10.2 — using trait bounds to conditionally implement methods ↗](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods) — blanket impls, and the orphan rule by name
- [The Reference — orphan rules ↗](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules)
- [Effective Rust, Item 6 — bypassing the orphan rule ↗](https://effective-rust.com/newtype.html#bypassing-the-orphan-rule-for-traits)
- [Niko Matsakis, *Little Orphan Impls* (2015) ↗](https://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls/) and [RFC 2451 ↗](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html) — why coherence is shaped the way it is
- [users.rust-lang.org — *ToOwned if and only if a value is a reference* (2023) ↗](https://users.rust-lang.org/t/toowned-if-and-only-if-a-value-is-a-reference/97697) — no bound can say "`T` is not a reference", so the answer is a trait with one impl for `&T where T: ToOwned` and one per owned type. Two things to take from it: `.into_iter().map(ToOwned::to_owned).collect()` is how a `Vec<&str>` becomes a `Vec<String>`, and the posted impl needs `T: ToOwned + ?Sized` to reach `&str` at all — without the `?Sized`, `"text".into_js_value()` is `E0599` on rustc 1.98.0.
- *Rust for Rustaceans*, ch. 2 → "Coherence and the Orphan Rule" (p. 28)

### [7. `Borrow` is the way back](../borrow_the_way_back/README.md)

- [`Borrow` ↗](https://doc.rust-lang.org/std/borrow/trait.Borrow.html) — the `HashMap` walkthrough, and why `Eq`, `Ord` and `Hash` must agree
- [`AsRef` — relation to `Borrow` ↗](https://doc.rust-lang.org/std/convert/trait.AsRef.html#relation-to-borrow)
- [Effective Rust, Item 8 — more pointer traits ↗](https://effective-rust.com/references.html#more-pointer-traits) — `Borrow`, `ToOwned` and `Cow` together
- [users.rust-lang.org — *Understanding Borrow and ToOwned* (2022) ↗](https://users.rust-lang.org/t/understanding-borrow-and-toowned/75951)
- *Programming Rust*, 2nd ed., ch. 13 → "AsRef and AsMut", "Borrow and BorrowMut", "Borrow and ToOwned at Work: The Humble Cow"

### [8. The traps are steps 5 and 6 together](../to_owned_traps/README.md)

- [TRPL ch. 15.4 — cloning to increase the reference count ↗](https://doc.rust-lang.org/book/ch15-04-rc.html#cloning-to-increase-the-reference-count) — "doesn't make a deep copy"
- [`std::rc` — cloning references ↗](https://doc.rust-lang.org/std/rc/index.html#cloning-references)
- [`Cow::into_owned` ↗](https://doc.rust-lang.org/std/borrow/enum.Cow.html#method.into_owned), and clippy's [`suspicious_to_owned` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#suspicious_to_owned)
- Video: [Crust of Rust — Smart Pointers and Interior Mutability ↗](https://www.youtube.com/watch?v=8O0Nt9qY_vo) (2020) — *Rc* at 1:06:27, *Copy-on-Write (Cow)* at 1:54:20
- *Programming Rust*, 2nd ed., ch. 4 → "Rc and Arc: Shared Ownership"
- *Rust in Action*, ch. 6 → §6.2.2 "Rust's pointer ecosystem", Figure 6.4 (p. 186) — the card game that gives `Rc<T>`, `Cow<T>` and the rest one card each

### [9. `clone_into` refills instead of allocating](../clone_into_refills/README.md)

- [`ToOwned::clone_into` ↗](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#method.clone_into) and [`Clone::clone_from` ↗](https://doc.rust-lang.org/std/clone/trait.Clone.html#method.clone_from)
- [Announcing Rust 1.63.0 ↗](https://blog.rust-lang.org/2022/08/11/Rust-1.63.0/), which stabilized `clone_into`
- Clippy's [`assigning_clones` ↗](https://rust-lang.github.io/rust-clippy/master/index.html#assigning_clones) — flags `a = b.clone()` and names both methods
- No book gives this a section of its own; the [`clone_into` page](../../clone_into/README.md) is the long treatment.

### [10. Implementing it — on the referent, or not at all](../implementing_it_or_not/README.md)

- [std's `path.rs` ↗](https://github.com/rust-lang/rust/blob/master/library/std/src/path.rs) — the real thing: `#[repr(transparent)] pub struct Path { inner: OsStr }`, `impl ToOwned for Path`, `impl Borrow<Path> for PathBuf`, and the pointer cast between them. Search it by item name; line numbers drift.
- [The Reference — the transparent representation ↗](https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation) and [the Rustonomicon on `repr(transparent)` ↗](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent)
- [`ref-cast` ↗](https://docs.rs/ref-cast) — dtolnay's safe derive for the `&T` → `&Wrapper` cast
- [Konstantin Grechishchev, *6 things you can do with the Cow in Rust* (2022) ↗](https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55) — its "keep your own type inside it" section builds an unsized `MyStr` with `Borrow` and `ToOwned`
- Video: *Crust of Rust — Dispatch and Fat Pointers*, above, *Q&A: Making Your Own DST* at 1:51:45

### [After the steps: `Clone`, `ToOwned` or `From`?](../clone_to_owned_or_from/README.md)

- [Rust API Guidelines — `as_`, `to_`, `into_` conventions (C-CONV) ↗](https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv), and [flexibility ↗](https://rust-lang.github.io/api-guidelines/flexibility.html) for who controls the allocation
- [Effective Rust, Item 5 — user-defined type conversions ↗](https://effective-rust.com/casts.html#user-defined-type-conversions)
- [`Into` — examples ↗](https://doc.rust-lang.org/std/convert/trait.Into.html#examples) and [`ToString` ↗](https://doc.rust-lang.org/std/string/trait.ToString.html) — implement `Display` instead
- [Herman Radtke, *Creating a Rust function that accepts String or &str* (2015) ↗](https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html/) — the `S: Into<String>` parameter
- *Rust for Rustaceans*, ch. 3 "Designing Interfaces" → "Generic Arguments" (p. 43) and "Borrowed vs. Owned" (p. 45)
- *Programming Rust*, 2nd ed., ch. 13 → "From and Into"; ch. 17 → "Converting Other Types to Strings"

## Source code worth reading

[`ToOwned` in real code](../to_owned_in_real_code/README.md) collects fourteen excerpts — std's `str`, `[T]`, `Path`, `CStr`, `Cow::to_mut`, `Cow`'s `+=` and `from_utf8_lossy`, and `serde_json`, `bstr`, `camino`, `regex`, `borrowme` and `beef` — each pinned to a release tag.

## `Cow`, specifically

The path meets `Cow` in [steps 7](../borrow_the_way_back/README.md) and [8](../to_owned_traps/README.md); [What `Cow` explanations get wrong](../cow_claims_checked/README.md) runs the claims the sources below repeat.

- ***Programming Rust*, 2nd ed., ch. 13 → "Borrow and ToOwned at Work: The Humble Cow" (p. 323), and ch. 17 → "Putting Off Allocation"** — the best book treatment: an error-describing function that returns `"…".into()` from most arms and a formatted `String` from one, `into_owned` for the caller who must keep it, `to_mut` and `+=` on a `get_name()` result. One sentence to correct as you read: `to_mut` returns `&mut <B as ToOwned>::Owned` — a `&mut String` — not the `&mut B` the text says.
- [Pascal Hertleif, *The Secret Life of Cows* (2018) ↗](https://deterministic.space/secret-life-of-cows.html) — the standard essay
- [Joe Wilm, *From &str to Cow* (2016) ↗](https://jwilm.io/blog/from-str-to-cow/) — `S: Into<Cow<'a, str>>` as a constructor argument
- [Herman Radtke, *Creating a Rust function that returns a &str or String* (2015) ↗](https://hermanradtke.com/2015/05/29/creating-a-rust-function-that-returns-string-or-str.html/)
- [Konstantin Grechishchev, *6 things you can do with the Cow in Rust* (2022) ↗](https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55)
- [`Rc::make_mut` ↗](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.make_mut) and [`Arc::make_mut` ↗](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.make_mut) — clone-on-write for shared pointers, which the `Cow` docs themselves point to
- [`regex::Regex::replace_all` ↗](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace_all) — a crate API returning `Cow<'h, str>`, so it can hand the haystack back borrowed when nothing matched

**Read with care** — each is useful, and each carries one of the errors [the claims page](../cow_claims_checked/README.md) runs:

- [Easy Rust, *Cow* ↗](https://dhghomon.github.io/easy_rust/Chapter_42.html) (headed "Chapter 43" although the URL says 42) — a gentle first telling, with a `modulo_3` example that shows both variants. It says `Cow` has "`into_owned` or `into_borrowed`"; there is no `into_borrowed`.
- [Thor, *Tipping Cows, a Primer on Rust's Most Bovine Data Structure* (2020, updated 2022) ↗](https://thork.net/posts/2020_cows_in_the_wild/) — a readable primer with a lazy `abs` over a `Cow<[i32]>`. Its `Cow<'a, Vec<T>>` wrapper compiles only through the blanket impl, and takes fewer inputs than `Cow<'a, [T]>`: a `Borrowed` there must point at a whole `Vec`.
- *Rust in Action*, ch. 6, listing 6.3 (p. 182) — `Cow` in code, as the return of `CStr::to_string_lossy`. Learn the `Cow` from it and not the rest: the listing builds a `String` with `String::from_raw_parts` over a `static` array, which `String` then tries to free. On rustc 1.98.0 it prints its line and aborts with `SIGABRT` when `main` returns. Miri names the undefined behaviour, and Linux prints `free(): invalid pointer`: [Listing 6.3, run](../../../36_Pointers/rust_in_action_chapter_6/README.md#listing-63-a-string-that-frees-a-static). Its p. 187 also names `core::ptr::Shared`, which no longer exists; `Rc` and `Arc` hold a `NonNull`.

Two more circulate widely and are not recommended: a LinkedIn article on `Cow<T>` in Rust, which says `String` implements `ToOwned`, that `as_ref()` returns a `Cow`, and that `Cow` costs dynamic dispatch; and chat-generated explanations that describe `Owned` as "the cloned value". Each of those is run on [the claims page](../cow_claims_checked/README.md).

Offline copies, listed by title so they can be found again:

- *SurrealDB Rust code discoveries Cow!*
- *COW Smart Pointer using RUST — Memory Optimizations — Clone on Write — Increase Performance*

## Edition notes

- **The Rust Programming Language renumbered its chapters** for the 2024-edition printing: advanced features are now ch. 20, trait objects ch. 18, and ch. 17 is async. Old `ch19-…` URLs redirect, but some anchors did not survive — link by the section title.
- **The Reference and the Rustonomicon renumber sections** whenever a page is added. The Reference's rule IDs, such as `r-expr.method.candidate-receivers`, are the stable citation.
- ***Programming Rust*, 3rd edition** is listed by O'Reilly for 6 October 2026. Chapter 13, "Utility Traits", keeps its number and all twelve section titles; the chapters after 20 shift by one.
- ***Rust for Rustaceans*** is still the first edition, from November 2021.

## See also

- [How to learn `ToOwned`](../README.md) — the path this list belongs to
- [Traits: links and videos](../../resources/README.md) and [Strings: links, books and videos](../../../14_Strings/resources/README.md) — the section-wide reading lists
- [Books](../../../10_Resources/books/README.md) — which Rust books to buy, and in what order

## Po polsku

Lista lektur do każdego kroku ścieżki `ToOwned`: dokumentacja standardowa, Reference, Rustonomicon, rozdziały książek i artykuły. Przy książkach drukowanych podano **nazwę** rozdziału i sekcji, nie tylko numer, bo numery zmieniają się między wydaniami — dotyczy to zwłaszcza *The Rust Programming Language* i nadchodzącego trzeciego wydania *Programming Rust*.

Jeśli czytać tylko jedno: rozdział 13 *Programming Rust* („Utility Traits”) przechodzi przez całą ścieżkę po kolei, łącznie z `Cow`. Przy materiałach o `Cow` zaznaczono, co w każdym jest błędne — na przykład nieistniejące `into_borrowed` albo listing z *Rust in Action*, który zwalnia pamięć, której nigdy nie przydzielił.

**Szukaj po polsku:** `rust ToOwned Borrow Cow książka` · `Programming Rust Utility Traits` · `rust Cow artykuł`
