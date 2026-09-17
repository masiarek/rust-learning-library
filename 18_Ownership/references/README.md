# References: `&`, `&mut`, `ref`, `*` and the pointers beside them

[Ownership](../README.md) › **References**

**Level:** 101 → 301 · a learning path

**One line:** A Rust reference is a pointer carrying promises the compiler checks — never null, never outliving what it points at, and never usable while a `&mut` to the same place is live. The lessons that teach it are spread over five sections of the library; this folder lists them in the order they build on each other, and adds the pages that were missing.

Pages marked **new** live in this folder. The rest are existing lessons elsewhere, linked in place rather than repeated. Go top to bottom, or find what surprised you in the next table and start there.

## Find your page

| If this surprised you | Start at |
|---|---|
| `!flag` compiles on a `flag: &bool`, but `if flag` does not | [When you need the `*`](when_you_need_the_star/README.md#five-places-the-is-written-for-you) |
| `flag == true` is refused, and `n > 1` fails with a different error code | [When you need the `*`](when_you_need_the_star/README.md#wants-both-sides-at-the-same-depth) |
| `.filter(\|b\| **b)` needs two stars | [When you need the `*`](when_you_need_the_star/README.md#closures-count-the) |
| `let &x = r;` *removes* a reference, and the `*` in `*const T` dereferences nothing | [Where the `&` sits](../where_the_sigil_sits/README.md) |
| a borrow error's `^` points at the `push`, not at the later read that made it an error | [A borrow is a loan](a_borrow_is_a_loan/README.md#reading-a-borrow-error) |
| `&mut v[1]` borrowed the whole `Vec` | [A borrow is a loan](a_borrow_is_a_loan/README.md#reading-a-borrow-error) |
| an `i32`, which has no destructor, was "dropped here while still borrowed" | [A borrow is a loan](a_borrow_is_a_loan/README.md#going-out-of-scope-is-a-use-too) |
| while `&b` is held, the owner cannot assign to `b` or move it | [Borrowed state](../borrowed_state/README.md) |
| a `&mut`, which is not `Copy`, was passed to three calls in a row | [Reborrowing](../reborrowing/README.md) |
| `Some(_n)` moved the value although `_n` is never used | [The `ref` keyword](the_ref_keyword/README.md#a-name-binds-and-moves-even-when-you-never-use-it) |
| `match &x { Some(ref n) => … }` compiles on edition 2021 and is an error on 2024 | [The `ref` keyword](the_ref_keyword/README.md#edition-2024-ref-where-the-borrow-is-already-implied) |
| one pattern moved one field and borrowed another | [The `ref` keyword](the_ref_keyword/README.md#one-pattern-two-modes) |
| `*t = &mut t[1..]` does not compile, and rustc's suggested `&'a mut &'a mut` breaks the caller | [Re-pointing a slice](repointing_a_slice/README.md#why-not-t-mut-t1) |
| reading from a `&[u8]` moved the slice forward | [Re-pointing a slice](repointing_a_slice/README.md#std-does-this-read-for-u8-write-for-mut-u8) |
| `<'a>` did not make anything live longer | [Lifetime annotations](../lifetime_annotations/README.md) |
| `Box<str>` is two words, and moving out of `*boxed` compiles where `*rc` does not | [Six pointer types, one table](pointer_types_compared/README.md#boxt-is-neither-always-one-pointer-wide-nor-only-a-pointer) |
| a table said `&T` lives on the stack and has shared ownership | [Six pointer types, one table](pointer_types_compared/README.md#what-the-popular-table-gets-wrong) |
| `Option<&T>` is the same size as `&T` | [Six pointer types, one table](pointer_types_compared/README.md#a-reference-is-a-pointer-with-rules) |
| a `Cell` changed behind a `&` | [Interior mutability](../../09_Advanced/interior_mutability/README.md) |
| a snippet from Stack Overflow, The Book or a course did not compile as printed | [Reference claims, run](reference_claims_checked/README.md) |

## The path

**Before you start**, the memory the rules protect: [Stack and heap](../stack_and_heap/README.md) (where a value's bytes live), [The call stack](../the_call_stack/README.md) (what a call does to them) and [A stack slot is reused](../a_stack_slot_is_reused/README.md) (why a pointer into a finished frame is a bug and not a curiosity).

| # | Page | What clicks |
|---|---|---|
| 1 | [Where the `&` sits decides what it does](../where_the_sigil_sits/README.md) | `&` and `*` do different jobs in a type, an expression and a pattern, so `*r` names a place |
| 2 | [Borrowing](../borrowing/README.md) | many readers or one writer, and a borrow ends at its last use |
| 3 | **new** [A borrow is a loan](a_borrow_is_a_loan/README.md) | how the compiler decides: each reference records a loan, two conditions are checked, and every labelled line of a borrow error is one of them |
| 4 | [Borrowed state](../borrowed_state/README.md) | the same loan from the owner's side: `E0505` and `E0506` |
| 5 | [Reborrowing](../reborrowing/README.md) | a call site passes `&mut *r`, so a `&mut` is not moved into the call |
| 6 | **new** [When you need the `*`](when_you_need_the_star/README.md) | the five places Rust writes the `*` for you, and why `if flag` is not one of them |
| 7 | **new** [The `ref` keyword](the_ref_keyword/README.md), then [Match ergonomics](../../30_Pattern_Matching/match_ergonomics/README.md) | a binding that borrows instead of moving — and the default binding mode that means you rarely write it |
| 8 | **new** [Re-pointing a slice](repointing_a_slice/README.md) | `&mut &mut [T]`: a `&mut` to the caller's reference, `mem::take`, and `Read for &[u8]` |
| 9 | [How to learn lifetimes](../how_to_learn_lifetimes/README.md) → [Lifetime annotations](../lifetime_annotations/README.md) → [What `&'a T` claims](../what_a_reference_claims/README.md) → [Lifetimes at the call site](../lifetimes_at_the_call_site/README.md) → [Temporary lifetime extension](../temporary_lifetimes/README.md) | the loan's length written into a signature, and what it costs the caller |
| 10 | **new** [Six pointer types, one table](pointer_types_compared/README.md) | `&T`, `&mut T`, `*const T`, `*mut T`, `Box<T>` and `Rc<T>` on seven questions, each cell run |
| 11 | [`Rc`](../reference_counting/README.md) · [`Arc`](../sharing_across_threads/README.md) · [Interior mutability](../../09_Advanced/interior_mutability/README.md) · [`Cow`](../clone_on_write/README.md) | several owners, writing through a `&`, and borrowing until somebody writes |
| 12 | [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) · [Use-after-free](../../31_C_and_Cpp/use_after_free/README.md) · [Null dereference](../../31_C_and_Cpp/null_dereference/README.md) · [Iterator invalidation](../../31_C_and_Cpp/iterator_invalidation/README.md) · [Lifetime safety in Clang](../../31_C_and_Cpp/lifetime_safety_in_clang/README.md) | raw pointers, and the C and C++ bugs the rules above make unwritable |

Beside the path:

- **new** [What reference explanations get wrong, run](reference_claims_checked/README.md) — claims from a Stack Overflow thread, The Book, *Programming Rust*, Cliffle's *Learn Rust the Dangerous Way* and a chat answer, each checked on rustc 1.98.0.
- **new** [Every reference error, and its fix](reference_errors/README.md) — the compiler errors of this path by symptom: the code, rustc 1.98.0's words, the mistake and the fix, both halves checked on every build.
- **new** [Lints around references](reference_lints/README.md) — rustc and clippy warnings about borrows, `ref`, `*` and raw pointers, bad and good, and what no lint catches.
- **new** [Helpful resources](references_reading_list/README.md) — chapters, docs, a paper, articles and videos for each part of the path, with the ones to read with care.

## Katas along the path

The pages say what to understand; these make you write it. Each lives on its page, with a compiled solution folded under it.

| After step | Kata | On |
|---|---|---|
| 2 | [Many readers, or one writer](../borrowing/README.md#practice) — two shared borrows, then a mutable one, and the `println!` that moves the end of a borrow | [Borrowing](../borrowing/README.md) |
| 3 | [Reference to a local variable](a_borrow_is_a_loan/README.md#practice) — name the loan and the condition the `}` breaks, then fix `E0597` two ways | [A borrow is a loan](a_borrow_is_a_loan/README.md) |
| 6 | [Fewest stars](when_you_need_the_star/README.md#practice) — six lines without a `*` or `&`, each fixed with the fewest and the rule named | [When you need the `*`](when_you_need_the_star/README.md) |
| 7 | [Borrow one field, move the other](the_ref_keyword/README.md#practice) — one `let` pattern, and why `&upload` cannot do its job | [The `ref` keyword](the_ref_keyword/README.md) |
| 7 | [One character apart](../../30_Pattern_Matching/match_ergonomics/README.md#practice) — `Some(name)` against `&opt` and against `opt` | [Match ergonomics](../../30_Pattern_Matching/match_ergonomics/README.md) |
| 8 | [Split off the header](repointing_a_slice/README.md#practice) — advance a `&mut &[u8]` and a `&mut &mut [u8]` | [Re-pointing a slice](repointing_a_slice/README.md) |
| 10 | [Pick the pointer](pointer_types_compared/README.md#practice) — a config reader, a tree's children, a shared cache and C's `memchr` | [Six pointer types, one table](pointer_types_compared/README.md) |
| 11 | [Predict the count four times](../reference_counting/README.md#practice) — one roster shared by three tallies, and the edge that leaks | [`Rc`](../reference_counting/README.md) |

The full sequence, with every other kata in the library, is [KATAS.md](../../KATAS.md).

## Four symbols, four different kinds of thing

The most-voted Stack Overflow answer on this question opens by separating them, and the separation is the first thing to hold on to:

| You see | What it is | Where it is taught |
|---|---|---|
| `&x`, `&mut x` · `&T`, `&mut T` | an operator that borrows, and the sigil of a reference **type** | [Where the `&` sits](../where_the_sigil_sits/README.md), [Borrowing](../borrowing/README.md) |
| `*r` · `*const T`, `*mut T` | an operator that dereferences, and the sigil of a raw pointer **type**, where it dereferences nothing | [When you need the `*`](when_you_need_the_star/README.md), [Where the `&` sits](../where_the_sigil_sits/README.md#the-that-never-dereferences) |
| `ref x`, `ref mut x` | **pattern** syntax: bind by reference instead of by move | [The `ref` keyword](the_ref_keyword/README.md) |
| `Box<T>`, `Rc<T>` | **library types** that own what they point at | [Six pointer types, one table](pointer_types_compared/README.md) |

## Tests for anything else you read

Most explanations of references state the rule correctly and get one fact under it wrong. The tell for each:

- **"`ref` is the same as `&`."** In a pattern they are opposites: `ref x` matches the same values as `x` and binds a reference, while `&x` matches only a reference and binds what is behind it — [The `ref` keyword](the_ref_keyword/README.md#what-ref-changes-and-what-it-does-not).
- **"The compiler flags the line where you use the old reference."** The `^` is under the use of the *owner* that breaks the loan; the later read is the line labelled *later used here* — [A borrow is a loan](a_borrow_is_a_loan/README.md#reading-a-borrow-error).
- **"At run time a reference is just an address."** For `&u32`. A `&str` or `&[T]` is an address and a length — [claim 3](reference_claims_checked/README.md#eight-claims-run).
- **"References live on the stack."** A reference is a value like any other: in a `Vec<&str>` the references are on the heap, and the pointee can be anywhere — [Six pointer types, one table](pointer_types_compared/README.md#what-the-popular-table-gets-wrong).
- **"`Box<T>` is always one pointer wide."** Not when `T` is unsized: `Box<str>` and `Box<dyn Trait>` are two words — [Six pointer types, one table](pointer_types_compared/README.md#boxt-is-neither-always-one-pointer-wide-nor-only-a-pointer).
- **"`*const T` and `*mut T` differ only as a lint."** They cast freely, but they differ in variance — [Six pointer types, one table](pointer_types_compared/README.md#const-t-and-mut-t-differ-in-more-than-a-lint).
- **"`Rc<T>` is read-only."** Through `Deref`. `Rc::get_mut` writes while the count is one — [claim 5](reference_claims_checked/README.md#eight-claims-run).
- **"Rust auto-derefs, so `if flag` should work on a `&bool`."** Auto-deref happens at the dot and in coercions between reference types, never into a condition — [When you need the `*`](when_you_need_the_star/README.md#a-condition-wants-exactly-bool).
- **"An unused binding such as `Some(_n)` does not move anything."** It moves; only `_` binds nothing — [The `ref` keyword](the_ref_keyword/README.md#a-name-binds-and-moves-even-when-you-never-use-it).
- **"Apply rustc's `help:` and it compiles."** It does — and on `&mut &mut [T]` the suggested `'a` on both layers makes the *caller* the next error — [Re-pointing a slice](repointing_a_slice/README.md#rustcs-help-compiles-and-breaks-the-caller).

## If you are coming from another language

- **C** has no references, only pointers — `int &r = x;` does not parse as C ([the clang run](reference_claims_checked/README.md#c-has-no-reference-variables-c-does)). Every C pointer parameter leaves the same four questions to the caller: may it be null, how many elements does it cover, is the memory initialized, and who else is writing through it. A Rust reference answers all four in its type, which is Cliffle's argument in *Learn Rust the Dangerous Way* part 2 and the subject of [Six pointer types, one table](pointer_types_compared/README.md). The C library runs the length question twice: [A string is bytes up to a NUL ↗](https://masiarek.github.io/c-learning-library/03_Strings/a_string_is_bytes_up_to_a_nul/), where an array forgets its length the moment it is passed to a function, and [The functions that do not check ↗](https://masiarek.github.io/c-learning-library/03_Strings/the_functions_that_do_not_check/), where `strcpy` writes past a destination whose size it was never told.
- **C++** has `T&`, and it is the closer neighbour — a reference that cannot be null — but the similarity ends at the lifetime. Apple clang 21 builds an `int&` returned to a local with a warning, and an `int&` into a `std::vector` held across `push_back` with none, even under `-Wall -Wextra` ([the run](reference_claims_checked/README.md#c-has-no-reference-variables-c-does)). Rust refuses both, and [A borrow is a loan](a_borrow_is_a_loan/README.md) names the condition each one breaks. The heap version of the same bug is run in [Use-after-free](../../31_C_and_Cpp/use_after_free/README.md).
- **Python** gives every name a reference and checks nothing: two names can alias one dict, and growing it while a loop walks it raises `RuntimeError` at run time. Rust makes aliasing and mutation mutually exclusive before the program runs — [Borrowing](../borrowing/README.md#the-bug-the-rule-exists-to-prevent) names the Python error for the same bug. The Python library's [`bytearray` is the mutable one ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/bytearray_is_mutable/) makes the matching point from the other side: an immutable `bytes` can be handed to a function with no defensive copy because nobody can change it, which is what a `&T` promises for every type.

## See also

- [How to learn lifetimes](../how_to_learn_lifetimes/README.md) — the same kind of page, for the other wall
- [How to learn `ToOwned`](../../12_Traits/how_to_learn_to_owned/README.md) — where references meet `Clone`, method lookup and unsized types
- [Ownership](../README.md) — the section this folder sits in
- [Glossary](../../GLOSSARY.md) — *shared reference*, *exclusive reference*, *dangling reference*, *fat pointer*, *NLL*
- [Interior mutability ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/safety_in_languages/interior_mutability/) in the Concurrency library's concept map — the same `Cell`/`RefCell` exception, and `Mutex` and atomics as its threaded form

## Po polsku

Referencja w Ruście to wskaźnik z obietnicami, które sprawdza kompilator: nigdy nie jest pusta (*null*), nigdy nie przeżyje wartości, na którą wskazuje, i nie da się jej użyć, dopóki żyje `&mut` do tego samego miejsca. Lekcje o referencjach są rozrzucone po pięciu działach biblioteki; ten folder układa je w kolejności, w której jedna buduje na drugiej, i dodaje brakujące strony: model pożyczki (*loan*) z artykułu *Stacked Borrows*, kiedy trzeba pisać `*` (w tym `&bool` w `if`), słowo kluczowe `ref`, `&mut &mut [T]`, tabelę sześciu rodzajów wskaźników oraz stronę z twierdzeniami z internetu i książek sprawdzonymi na rustc 1.98.0.

Cztery symbole to cztery różne rzeczy: `&` pożycza (i oznacza typ referencji), `*` dereferencjonuje (i oznacza typ surowego wskaźnika), `ref` to składnia wzorca, a `Box` i `Rc` to typy z biblioteki, które są właścicielami tego, na co wskazują.

**Szukaj po polsku:** referencja współdzielona · referencja mutowalna · pożyczanie · dereferencja · surowy wskaźnik · `rust ref keyword` · `rust reference vs pointer`
