# Arrays: the map

**Level:** 101 → 301 · the map

**One line:** `[T; N]` — a fixed number of values of one type, with the length in the type — is taught across a dozen pages in this library: how to write one, how it differs from a slice and a `Vec`, how it goes in and out of functions, where it lives, and how to iterate over it. This page lists them in reading order and says which one answers your question.

Arrays are one of Rust's two built-in *compound types*, with [tuples](../tuples/README.md). A `Vec` is not one of them: it comes from the standard library, and [Array or `Vec`?](../array_or_vec/README.md) is the page for choosing.

## Every array page, in reading order

| # | Page | What it answers |
|---|---|---|
| 1 | [Writing an array down](writing_an_array_down/README.md) | `[1, 2, 3]`, `[u8; 3]` and `[0; 3]`; why every element must be the same type; how inference picks it from the neighbours; the first loop over one, and the `u8` sum that overflows |
| 2 | [Arrays and slices](../arrays_and_slices/README.md) | the length is part of the type; `&[T]` moves it into the value; out-of-bounds panics; the methods live on the slice; the `&Vec<T>` trap |
| 3 | [Array or `Vec`?](../array_or_vec/README.md) | the four things an array buys, and why `Vec` is still the default |
| 4 | [Arrays in and out of functions](arrays_in_signatures/README.md) | returning `[i32; 3]` and `&'static [i32]`; a parameter of exactly N or of any length; const generics |
| 5 | [Where an array lives](where_an_array_lives/README.md) | stack, heap or static data; `Box<[i32; 3]>` against `Box<[i32]>`; the megabyte array that overflows a thread's stack |
| 6 | [An array in a `const` or a `static`](static_arrays/README.md) | a table of records in a `static`; what `'static` in its type means; comparing a `const` array with a local |
| 7 | [A `Vec` of arrays](../vec_of_arrays/README.md) | `Vec<[T; N]>`: fixed-width rows in one growable block |
| 8 | [Grids and nested `Vec`s](../vec_of_vecs/README.md) | `[[T; N]; M]`, a flat `Vec` and `Vec<Vec<T>>` for a 2D matrix |
| 9 | [Slices of slices](../slice_of_slices/README.md) | why `&[&[T]]` will not take your grid, and the signatures that will |
| 10 | [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) | three ways to iterate over an array, and the edition-2021 change to `array.into_iter()` |
| 11 | [`DoubleEndedIterator` and `ExactSizeIterator`](../../24_Iterators/double_ended_and_exact_size/README.md) | `.rev()` and `.len()` on an array's iterator |
| 12 | [`Vec::into_iter` and the three `IntoIterator` impls](../vec_methods/vec_into_iter/README.md) | the same three impls arrays have, on `Vec` |
| 13 | [`slice::iter`](../slice_methods/slice_iter/README.md) and [the `slice` methods](../slice_methods/README.md) | a page per method an array reaches through the slice: `sort`, `windows`, `chunks`, `contains` … |
| 14 | [Values](../../15_First_Programs/values/README.md) | the widths behind `[u8; 3]` and `[i32; 3]` |

Beside the lessons:

- [What array explanations get wrong, run](array_claims_checked/README.md) — array questions: twenty-four claims from books, docs, answers and chat, each checked on 1.98.0
- [Every array error, and its fix](array_errors/README.md) — array errors: twenty-nine refusals, by code and by message
- [Lints around arrays](array_lints/README.md) — twenty-four rustc and clippy warnings, each with a bad and a good program
- [Helpful resources for arrays](array_resources/README.md) — The Book, the Reference, std, books by chapter, articles and videos, each checked

## Which page answers your question

| You want to know | Go to |
|---|---|
| how to write (initialize) array values | [Writing an array down](writing_an_array_down/README.md#three-spellings) |
| what the two halves of `[0; 3]` mean: the value, and how many | [`[value; how many]`](writing_an_array_down/README.md#value-how-many) |
| why the elements must be the same type, and how one suffix decides it | [Inference reads the neighbours](writing_an_array_down/README.md#inference-reads-the-neighbours), errors [6 to 8](array_errors/README.md#one-element-type-per-array) |
| why `["bb".to_string(); 3]` does not compile | [error 9](array_errors/README.md#9-a-string-in-a-repeat-expression), and `array::from_fn` |
| how to iterate over an array | [Iterating over it by reference](writing_an_array_down/README.md#iterating-over-it-by-reference), [`iter`, `iter_mut`, `into_iter`](../../24_Iterators/iter_iter_mut_into_iter/README.md) |
| how to filter an array | [claims 11 and 12](array_claims_checked/README.md#the-claims), errors [22 to 24](array_errors/README.md#ownership-iteration-and-filtering) |
| what *IntoIterator for arrays* changed in edition 2021 | [`array_into_iter`](array_lints/README.md#array_into_iter) |
| array or `Vec` | [Array or `Vec`?](../array_or_vec/README.md) |
| how to pass an array to a function, or return one | [Arrays in and out of functions](arrays_in_signatures/README.md) |
| whether an array is on the stack or the heap; a heap-allocated array | [Where an array lives](where_an_array_lives/README.md) |
| how to build a static array, or an array of structs in a `static` | [An array in a `const` or a `static`](static_arrays/README.md) |
| comparing arrays: a `const` against a local, or two lengths | [A `const` array, and a local](static_arrays/README.md#a-const-array-and-a-local-that-changes-its-contents), [error 3](array_errors/README.md#3-comparing-arrays-of-two-lengths) |
| what happens on an index past the end | [Arrays and slices](../arrays_and_slices/README.md#out-of-bounds-is-a-panic-not-a-wrong-answer), [error 10](array_errors/README.md#10-a-constant-index-past-the-end) |
| whether arrays stop at 32 elements | [claim 4](array_claims_checked/README.md#the-claims) |
| a 2D matrix, rows and columns | [Grids and nested `Vec`s](../vec_of_vecs/README.md), [the transpose kata](arrays_in_signatures/README.md#practice) |
| a list of rows that all have the same width | [A `Vec` of arrays](../vec_of_arrays/README.md) |

## Katas along the way

| After | Kata | On |
|---|---|---|
| 1 | [The first ten odd numbers, two ways](writing_an_array_down/README.md#practice) — a block that fills a zeroed array, and `array::from_fn` | [Writing an array down](writing_an_array_down/README.md) |
| 2 | [One function, four callers](../arrays_and_slices/README.md#practice) — which signatures turn which callers away — and the call that printed its own receipt | [Arrays and slices](../arrays_and_slices/README.md) |
| 3 | [Two lengths in one election](../array_or_vec/README.md#practice) — one is a fact about the problem, one about this run | [Array or `Vec`?](../array_or_vec/README.md) |
| 4 | [Transpose a matrix](arrays_in_signatures/README.md#practice) — two index loops, then const generics and `from_fn` | [Arrays in and out of functions](arrays_in_signatures/README.md) |
| 5 | [Nine sizes, then a megabyte on a small stack](where_an_array_lives/README.md#practice) | [Where an array lives](where_an_array_lives/README.md) |
| 6 | [A static lookup table, checked while it compiles](static_arrays/README.md#practice) | [An array in a `const` or a `static`](static_arrays/README.md) |
| 7 | [Read the wire, and make the short row impossible](../vec_of_arrays/README.md#practice) | [A `Vec` of arrays](../vec_of_arrays/README.md) |
| 8 | [Build the same grid twice](../vec_of_vecs/README.md#practice) | [Grids and nested `Vec`s](../vec_of_vecs/README.md) |
| 9 | [A signature you cannot change](../slice_of_slices/README.md#practice) | [Slices of slices](../slice_of_slices/README.md) |
| 11 | [Five references where you meant five numbers](../../24_Iterators/double_ended_and_exact_size/README.md#practice) | [`DoubleEndedIterator` and `ExactSizeIterator`](../../24_Iterators/double_ended_and_exact_size/README.md) |

The full sequence, with every other kata in the library, is [KATAS.md](../../KATAS.md).

## If you are coming from another language

- **Python.** Python has no fixed-length array in the language: `list` is Rust's `Vec`, and `tuple` is the nearest fixed-length thing, though it may mix types. The byte types come closest to `[u8; N]`: [`bytes(5)` is five zero bytes ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/making_a_bytes_object/index.html), and [`bytearray` is the mutable one ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/bytearray_is_mutable/index.html), which grows where a Rust array cannot. Slicing differs at the edges: [a Python slice clamps an out-of-range bound ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/slicing_is_not_indexing/index.html), so `a[1:100]` on five elements is the last four. In Rust `&a[1..100]` panics with *range end index 100 out of range for slice of length 5*, and `a.get(1..100)` is `None`.
- **C.** A C array passed to a function becomes a pointer, and its length is lost: [`sizeof` then gives the pointer's size ↗](https://masiarek.github.io/c-learning-library/03_Strings/a_string_is_bytes_up_to_a_nul/index.html). Rust keeps the length, in the type for `[T; N]` and beside the pointer for `&[T]`. The overruns that follow from that in C — [functions that write past a fixed buffer ↗](https://masiarek.github.io/c-learning-library/03_Strings/the_functions_that_do_not_check/index.html), and [a length prefix nobody checked ↗](https://masiarek.github.io/c-learning-library/05_Bytes_on_the_Wire/a_length_you_did_not_check/index.html) — are a panic or a compile error in Rust.
- **C++.** `std::array<T, N>` is `[T; N]`, stored inline, with the length in the type, and `std::vector<T>` is `Vec<T>`. The C++ library has no arrays page yet.
- **Go.** Go draws the same line: `[3]int` is a value with the length in its type, and `[]int` is a view with the length in the value. Go arrays are copied on assignment, as Rust's `Copy` arrays are. The Go library, a concurrency library, has no arrays page.
- **Java.** `int[]` carries its length at run time but not in its type, and it is always a reference to a heap object. [`split` returns a `String[]` with sharp edges ↗](https://masiarek.github.io/java-text-learning-library/04_Regex/split_has_sharp_edges/index.html) in the Java library.

## See also

- [Collections](../README.md) — the section this map belongs to: tuples, arrays, `Vec`, maps, sets and `Box`
- [How to learn `ToOwned`](../../12_Traits/how_to_learn_to_owned/README.md) — the same kind of map, for a trait
- [Glossary: array, slice, repeat expression](../../GLOSSARY.md) — one-paragraph definitions

## Po polsku

Wszystkie strony o tablicach `[T; N]` w jednym miejscu, w kolejności czytania. Kolejne strony uczą: jak zapisać tablicę (`[1, 2, 3]`, `[u8; 3]`, `[0; 3]`); czym różni się od wycinka `&[T]` i od wektora `Vec`; jak przekazać tablicę do funkcji i zwrócić ją; gdzie leży (stos, sterta, dane statyczne); jak zbudować tablicę statyczną z rekordami; jak po niej iterować. Obok są strony z pytaniami i odpowiedziami (twierdzenia sprawdzone na 1.98.0), z błędami kompilacji, z ostrzeżeniami clippy i ze źródłami.

Tablica i krotka to dwa wbudowane typy złożone (*compound types*). `Vec` do nich nie należy, bo pochodzi z biblioteki standardowej. Po polsku „tablica” to `[T; N]`, a to, co w Pythonie jest listą, to w Ruscie wektor.

**Szukaj po polsku:** tablice w Ruscie · array - main · array - errors · array - questions · array iterate · static array · filter an array · compound types · array - must be the same type · constants - comparing · heap-allocated array · IntoIterator for arrays · write (initialize) array values
