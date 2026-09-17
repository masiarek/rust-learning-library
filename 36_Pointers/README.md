# Pointers

**Level:** 201 · working knowledge

**One line:** A memory address is a number, a pointer is an address with a type, and a reference is a pointer with promises the compiler checks — this section measures each layer, the second word some pointers carry, the alignment every reference promises, and the raw pointers that promise nothing.

The section follows the vocabulary of *Rust in Action*, chapter 6 ("Memory"), which is also the source whose claims it checks:

| Word | Say it when | Rust types |
|---|---|---|
| **reference** | the compiler's guarantees apply | `&T`, `&mut T` |
| **pointer** | talking about the primitive idea, where you are responsible | any of these |
| **raw pointer** | the unsafety should be explicit | `*const T`, `*mut T` |
| **smart pointer** | a type that acts like a pointer and manages what it points at | `Box<T>`, `Rc<T>`, `String`, … — [its own section](../41_Smart_Pointers/README.md) |

Rust's Reference groups them the same way under [Pointer types ↗](https://doc.rust-lang.org/reference/types/pointer.html). Where a pointer is two words, this section calls it **wide**, the Reference's word, rather than *fat*.

## Lessons

| Lesson | Level | What it answers |
|---|---|---|
| [Address, pointer, reference](address_pointer_reference/README.md) | 201 | What each of the three adds to the one before — a type, provenance, and five promises — and why a pointer made from a number is not the pointer it equals |
| [Wide pointers](wide_pointers/README.md) | 201 | When a pointer is two words, why the second is a length for slices and a vtable pointer for `dyn Trait`, and why *wide* rather than *fat* |
| [A reference is aligned to its referent, not to `usize`](aligned_to_the_referent/README.md) | 201 → 301 | What alignment belongs to, where padding comes from, why `repr(packed)` fields refuse a reference, and what a misaligned one does in debug, `-O` and Miri |
| [Raw pointers: `*const T` and `*mut T`](raw_pointers/README.md) | 201 → 301 | What safe code may do with one, what needs `unsafe`, the aliasing rules the borrow checker does not check, and reading from address 0 |
| [*Rust in Action*, chapter 6: pointer claims, run](rust_in_action_chapter_6/README.md) | 201 | Seventeen claims from the chapter and its listings, checked on rustc 1.98.0 — including a listing that is undefined behaviour |

Read them in that order: the first names the three layers, the next two measure what a pointer's value holds and where it may point, the fourth takes the promises away, and the last is the chapter the section was written against.

## Already elsewhere in the library

These pages were written before this section, and each answers one pointer question in place. They are linked, not repeated:

| Question | Page |
|---|---|
| Which address does `&x` give you, and why does no example print one? | [What an address shows](../18_Ownership/what_an_address_shows/README.md) |
| What does the lifetime in `&'a T` promise? | [What `&'a T` claims](../18_Ownership/what_a_reference_claims/README.md) |
| Why can there be many `&T` or one `&mut T`? | [Borrowing](../18_Ownership/borrowing/README.md) and [Reborrowing](../18_Ownership/reborrowing/README.md) |
| How does the compiler decide a borrow is refused? | [A borrow is a loan](../18_Ownership/references/a_borrow_is_a_loan/README.md), and every page on references in order: [References: the map](../18_Ownership/references/README.md) |
| `&T`, `&mut T`, `*const T`, `*mut T`, `Box<T>` and `Rc<T>` side by side? | [Six pointer types, one table](../18_Ownership/references/pointer_types_compared/README.md) |
| How is "no value" written without a null? | [Nullable pointers](../17_Option_and_Result/nullable_pointers/README.md) and [`Option` is a one-item collection](../17_Option_and_Result/option_as_collection/README.md) |
| Why is `&str` two words? | [`str` is unsized](../14_Strings/str_is_unsized/README.md) and [Step 2: Some types have no size](../12_Traits/how_to_learn_to_owned/types_with_no_size/README.md) |
| What does a `String`'s pointer point at? | [The anatomy of a `String`](../14_Strings/anatomy_of_a_string/README.md) |
| What does `unsafe` allow, and what does it not turn off? | [What `unsafe` turns off](../09_Advanced/what_unsafe_turns_off/README.md) |
| What is a pointer to a function? | [Function pointers](../23_Closures/function_pointers/README.md) |
| What does a `dyn Trait` pointer cost at a call? | [Static vs dynamic dispatch](../12_Traits/static_vs_dynamic_dispatch/README.md) |
| What do the C pointer bugs look like, and what does Rust refuse? | [Null dereference](../31_C_and_Cpp/null_dereference/README.md), [Use-after-free](../31_C_and_Cpp/use_after_free/README.md), [Double free](../31_C_and_Cpp/double_free/README.md), [Buffer overruns](../31_C_and_Cpp/buffer_overruns/README.md) |
| How do you build an owner from a raw pointer? | [`Vec::from_raw_parts`](../26_Collections/vec_methods/vec_from_raw_parts/README.md) and [`String::from_raw_parts`](../14_Strings/string_methods/string_from_raw_parts/README.md) |
| Where does heap memory come from? | [The global allocator](../09_Advanced/the_global_allocator/README.md) and [`Allocator::shrink`](../09_Advanced/allocator_shrink/README.md) |

## Words

Every term below has a [glossary](../GLOSSARY.md) entry: *memory address*, *raw pointer*, *provenance*, *wide pointer* (and *fat pointer*, the older name), *thin pointer*, *pointer alignment*, *padding*, *pointer cast*, *`NonNull<T>`*, *niche*.

## See also

- [Smart pointers](../41_Smart_Pointers/README.md) — the section after this one: `Deref`, `Drop`, and what `Box` and `Rc` cost
- [Ownership](../18_Ownership/README.md) — the rules references exist to follow
- [C and C++](../31_C_and_Cpp/README.md) — the bugs raw pointers make possible, run in C
- [Books](../10_Resources/books/README.md) — *Rust in Action* among the others

## Po polsku

Ten dział mierzy trzy warstwy: **adres** (liczba), **wskaźnik** (adres z typem i pochodzeniem — *provenance*) oraz **referencję** (wskaźnik z obietnicami, które sprawdza kompilator). Nazewnictwo idzie za *Rust in Action*: „referencja”, gdy działają gwarancje kompilatora, „wskaźnik”, gdy odpowiadasz sam, „surowy wskaźnik”, gdy tę niebezpieczność trzeba nazwać wprost. Wskaźnik złożony z dwóch słów nazywamy **szerokim** (*wide*), jak Reference, a nie „grubym” (*fat*).

Lekcje odpowiadają po kolei: co dodaje każda warstwa; kiedy wskaźnik ma drugie słowo i czym ono jest (długością albo wskaźnikiem do vtable); do czego wyrównana jest referencja (do typu, na który wskazuje, nie do `usize`); co wolno z surowym wskaźnikiem bez `unsafe`, a co nie; i które twierdzenia rozdziału 6 książki się nie sprawdzają. Wiele pokrewnych stron istniało już wcześniej w innych działach — tabela „Already elsewhere” prowadzi do nich zamiast je powtarzać.

**Szukaj po polsku:** wskaźniki w Ruście · referencja a wskaźnik · surowe wskaźniki · wyrównanie pamięci · `rust pointers references raw pointers`
