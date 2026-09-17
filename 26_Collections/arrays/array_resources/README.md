# Helpful resources for arrays

[Arrays: the map](../README.md) › **Beside the lessons** · start at: [Writing an array down](../writing_an_array_down/README.md)

**Level:** reference · the reading list

**One line:** Where else fixed-size arrays are explained — The Book, the Reference, std, the Edition Guide, Rust by Example, Comprehensive Rust, Rustlings, two books by chapter and section name, articles and videos — with what each covers and what to read with care, because several say things that 1.98.0 no longer does.

Every entry was checked on 2026-09-16: each URL answered 200 (Stack Overflow and O'Reilly refuse scripted requests, so those two were checked through the Stack Exchange API and a copy of the book), and each anchor exists on its page. Print books give their chapter and section *titles*, because numbers move between editions. "Read with care" names a sentence that fails when run, and [the claims page](../array_claims_checked/README.md) has the run.

## If you read only four

- **[The Rust Programming Language, ch. 3.2 — *The Array Type* ↗](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)**, under "Compound Types", with *Array Element Access* and [*Invalid Array Element Access* ↗](https://doc.rust-lang.org/book/ch03-02-data-types.html#invalid-array-element-access). It says Rust has two primitive compound types, tuples and arrays. *Read with care:* "Arrays are useful when you want your data allocated on the stack" is about locals; [Where an array lives](../where_an_array_lives/README.md).
- **[`array` in std ↗](https://doc.rust-lang.org/std/primitive.array.html)**: the two ways to create one, which traits work at every length and which only up to 32, `From` for tuples of 1 to 12, and the [*Editions* ↗](https://doc.rust-lang.org/std/primitive.array.html#editions) section on `into_iter`.
- **[The Reference — array expressions ↗](https://doc.rust-lang.org/reference/expressions/array-expr.html#array-expressions)**: the binding rule for `[value; N]`, including that the `Copy` requirement applies only when the length is greater than 1, and that a const block or a path to a constant also qualifies. [Array and slice indexing expressions ↗](https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions) is on the same page.
- **[Edition Guide — *IntoIterator for arrays* ↗](https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html)**: why `array.into_iter()` means one thing before 2021 and another after, and how the migration lint works.

## Official documentation

- [The Reference — array types ↗](https://doc.rust-lang.org/reference/types/array.html): `N` is a constant `usize`, every element is always initialised, and indexing is bounds-checked. Its own example puts one array on the stack and one in a `Box`.
- [The Reference — constant promotion ↗](https://doc.rust-lang.org/reference/destructors.html#constant-promotion): why `&[1, 2, 3]` can be returned as `&'static [i32]`. [Arrays in and out of functions](../arrays_in_signatures/README.md#returning-static-i32-a-promoted-constant).
- [The Reference — const and static elision ↗](https://doc.rust-lang.org/reference/lifetime-elision.html#const-and-static-elision): a lifetime left out of a `static`'s type is `'static`. [An array in a `const` or a `static`](../static_arrays/README.md#where-the-lifetime-has-to-be-written).
- [`std::array` ↗](https://doc.rust-lang.org/std/array/index.html): the module, with [`from_fn` ↗](https://doc.rust-lang.org/std/array/fn.from_fn.html) (stable since 1.63.0) and [`repeat` ↗](https://doc.rust-lang.org/std/array/fn.repeat.html) (stable since 1.91.0). Arrays' own `map`, `as_slice`, `each_ref` and `each_mut` are on the primitive page.
- [The Book, ch. 4.3 — *Other Slices* ↗](https://doc.rust-lang.org/book/ch04-03-slices.html#other-slices): `&a[1..3]` is a `&[i32]`, a pointer to the first element and a length.
- [rust-lang/rust#25725 ↗](https://github.com/rust-lang/rust/issues/25725), *"IntoIterator should be implemented for [T; N] not just references"*: opened 2015-05-22, closed 2021-04-26. The history behind the edition change, in the order it happened:
  - [#65819 ↗](https://github.com/rust-lang/rust/pull/65819), *"Add `IntoIterator` impl for arrays by value (`for [T; N]`)"*, opened 2019-10-25, was **closed without merging** after a crater run found thousands of broken crates.
  - [#84147 ↗](https://github.com/rust-lang/rust/pull/84147), *"Cautiously add IntoIterator for arrays by value"*, merged 2021-04-25 for Rust 1.53.0. It cherry-picked the impl from #65819 and hid it from method-call dispatch on arrays, so `array.into_iter()` kept its old meaning on editions 2015 and 2018.
- [Rust 1.73.0 — *Cleaner panic messages* ↗](https://blog.rust-lang.org/2023/10/05/Rust-1.73.0/#cleaner-panic-messages): why a transcript with `panicked at 'index out of bounds…', src/main.rs` is from an older compiler.

## Tutorials and exercises

- [Rust by Example — *Arrays and Slices* ↗](https://doc.rust-lang.org/rust-by-example/primitives/array.html): one listing covering both. *Read with care:* "Arrays are stack allocated", and "Out of bound indexing on array with constant value causes compile time error", which is a lint and only applies to constant indexes. Its `.get` loop runs one past the end on purpose. [Claims 1, 2, 5 and 6](../array_claims_checked/README.md#the-claims).
- [Comprehensive Rust — 8.1 *Arrays* ↗](https://google.github.io/comprehensive-rust/tuples-and-arrays/arrays.html), [8.3 *Array Iteration* ↗](https://google.github.io/comprehensive-rust/tuples-and-arrays/iteration.html) and [8.5 *Exercise: Nested Arrays* ↗](https://google.github.io/comprehensive-rust/tuples-and-arrays/exercise.html), Google's course. The exercise transposes `[[1, 2, 3], [4, 5, 6], [7, 8, 9]]`, the same matrix as [this library's kata](../arrays_in_signatures/README.md#practice). *Read with care:* the speaker notes say arrays "go on the stack".
- [Rustlings — `04_primitive_types` ↗](https://github.com/rust-lang/rustlings/tree/main/exercises/04_primitive_types): `primitive_types3` builds an array of at least 100 elements, and `primitive_types4` takes a slice of one. There is no separate arrays chapter. *Read with care:* the hint credits coercion for `assert_eq!([2, 3, 4], nice_slice)` compiling. It is an impl, and [claim 7](../array_claims_checked/README.md#the-claims) calls it by name.

## Books

- ***Rust in Action*** (Tim McNamara, Manning 2021), ch. 2 "Language foundations" → **§2.10 "Making lists of things with arrays, slices, and vectors"** → 2.10.1 "Arrays", 2.10.2 "Slices", 2.10.3 "Vectors". Listing 2.21, [`ch2-3arrays.rs` ↗](https://github.com/rust-in-action/code/blob/1st-edition/ch2/ch2-3arrays.rs), is where [Writing an array down](../writing_an_array_down/README.md) starts. *Read with care:* slices give "fast, read-only access", but `&mut [T]` writes; and the remark that implementing traits for arrays becomes unwieldy predates const generics.
- ***Programming Rust***, 2nd ed. (Blandy, Orendorff & Tindall, O'Reilly 2021), ch. 3 "Fundamental Types" → **"Arrays, Vectors, and Slices"** → "Arrays", "Vectors", "Slices". The three types side by side, including that a length known only at run time cannot size an array. *Read with care:* the useful methods "are all provided as methods on slices, not arrays" is now mostly rather than entirely true.

## Articles and answers

- [Said van de Klundert, *Rust slice* (2021) ↗](http://saidvandeklundert.net/learn/2021-08-14-rust-slice/): arrays as the storage behind slices, writing through `&mut`, and measured sizes (`[i32; 500]` is 2,000 bytes, `&[i32]` 16, `&[i32; 500]` 8). *Read with care:* its panic output is in the pre-1.73 format, and one of its quoted definitions comes from std rather than The Book.
- [Stack Overflow — *How to iterate over and filter an array?* (2015) ↗](https://stackoverflow.com/questions/30467085/how-to-iterate-over-and-filter-an-array): the classic question. *Read with care:* every answer was last edited before Rust 1.53 and the 2021 edition. Its `|&&x|` pattern is now [`E0308`](../array_errors/README.md#23-x-from-an-answer-written-before-2021) over `into_iter()`, and `std::array::IntoIter::new` is deprecated.

## Videos

- [Let's Get Rusty — *Common Programming Concepts in Rust* ↗](https://www.youtube.com/watch?v=2V0JaMVjzws) (2021): follows The Book's chapter 3; the video's description puts *Data Types* at 3:52–7:45.
- [Code to the Moon — *Rust Pizza Slices, Arrays and Strings* ↗](https://www.youtube.com/watch?v=SHxiGLa6Btc) (2022): arrays at 0:15, slices at 0:36, in six minutes.
- [Rustfully — *10: Arrays are cool in Rust* ↗](https://www.youtube.com/watch?v=QaCrG3Bqnzk) (2025).
- [dcode — *Rust Programming Tutorial #20 - Arrays* ↗](https://www.youtube.com/watch?v=cH6Qv47MPwk) (2017). *Read with care:* it predates const generics and the 2021 edition.

The titles were checked through YouTube's oEmbed endpoint and the timestamps come from the video descriptions; the videos were not watched for this list.

## Checked and left out

- [*Automatically Detecting Lifetime Annotation Bugs in the Rust Language* ↗](https://research.redhat.com/wp-content/uploads/2022/09/7-RUST.pdf) (Red Hat Research, 2022) was suggested as an arrays source. It is a one-page research poster about lifetimes, borrowing and raw pointers, and says nothing about arrays.

## In this library

| Question | Page |
|---|---|
| how to write one | [Writing an array down](../writing_an_array_down/README.md) |
| the length in the type, slices, `&[T]` parameters | [Arrays and slices](../../arrays_and_slices/README.md) |
| array or `Vec` | [Array or `Vec`?](../../array_or_vec/README.md) |
| passing and returning | [Arrays in and out of functions](../arrays_in_signatures/README.md) |
| stack, heap, `Box<[T]>` | [Where an array lives](../where_an_array_lives/README.md) |
| tables in a `static` | [An array in a `const` or a `static`](../static_arrays/README.md) |
| iterating | [`iter`, `iter_mut` and `into_iter`](../../../24_Iterators/iter_iter_mut_into_iter/README.md) |

## See also

- [Arrays: the map](../README.md) — every page, and which question it answers
- [What array explanations get wrong, run](../array_claims_checked/README.md) — the "read with care" notes above, each run
- [Books](../../../10_Resources/books/README.md) — the library's shelf of whole books

## Po polsku

Lista źródeł o tablicach `[T; N]`, sprawdzona 2026-09-16: The Book (rozdział 3.2, „The Array Type”), Reference (typy tablicowe, wyrażenia tablicowe i reguła dla `[wartość; N]`), strona `array` w std, Edition Guide o `IntoIterator` dla tablic, Rust by Example, Comprehensive Rust, Rustlings, *Rust in Action* (§2.10) i *Programming Rust* (rozdział 3), artykuły i nagrania. Książki podano z tytułami rozdziałów i sekcji, bo numery zmieniają się między wydaniami.

„Czytaj ostrożnie” oznacza zdanie, które nie wytrzymuje uruchomienia na 1.98.0. Najczęściej chodzi o „tablice są na stosie”, o błąd kompilacji przy indeksie poza zakresem (tylko dla stałych) i o odpowiedzi sprzed edycji 2021, w których `into_iter()` na tablicy zwracało referencje.

**Szukaj po polsku:** tablice w Ruscie książki · `rust arrays tutorial` · `rust array into_iter edition 2021` · compound types
