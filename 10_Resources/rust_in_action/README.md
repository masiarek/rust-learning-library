# *Rust in Action*, run

**Level:** reference · a companion to the book

**One line:** Tim McNamara's *Rust in Action* (Manning, 2021) teaches systems programming through Rust. These pages run its listings on rustc 1.98.0, check the claims in the prose around them, and point each idea to the lesson in this library that covers it.

The book's code is in [rust-in-action/code ↗](https://github.com/rust-in-action/code/tree/1st-edition), on the `1st-edition` branch. That repository has no licence, so the listings are linked here rather than copied. Each page records what the book's file prints on rustc 1.98.0 and runs a program of its own for every claim it checks. For what this library thinks of the book, and how many of the repository's 62 Cargo projects still compile on 1.98.0, see [Books](../books/README.md).

## Chapter 2 — Language foundations

The pages for sections with a lesson section of their own live there — Cargo in Tooling, comparison and other number types in Numbers, flow control in Control flow — and are listed here with the rest.

| Book | The book's file | Page here | What the page checks |
|---|---|---|---|
| before listing 2.2 | [ch2/ok.rs ↗](https://github.com/rust-in-action/code/blob/1st-edition/ch2/ok.rs) | [From one `.rs` file to a Cargo project](../../05_Tooling/from_rustc_to_cargo/README.md) | `rustc` on one file, then `cargo init` and what `cargo run -v` shows |
| §2.2.1, listing 2.2 | [ch2-first-steps.rs ↗](https://github.com/rust-in-action/code/blob/1st-edition/ch2/ch2-first-steps.rs) | [Listing 2.2, run: variables and functions](first_steps/README.md) | inference from use; `: i32`, `30i32` and `30_i32`; assigning later without `mut`; the value of a macro call; the semicolon note; what `main` may return |
| §2.3.1, listing 2.3 | [ch2-intro-to-numbers.rs ↗](https://github.com/rust-in-action/code/blob/1st-edition/ch2/ch2-intro-to-numbers.rs) | [Listing 2.3, run: integers and floats](intro_to_numbers/README.md) | the `: i64` that is needed; an array's element type; `{:02}`; `round`; bits versus bytes; explicit conversions; `+` as a trait |
| §2.3.2, listing 2.4 | [ch2-non-base2.rs ↗](https://github.com/rust-in-action/code/blob/1st-edition/ch2/ch2-non-base2.rs) | [Listing 2.4, run: base 2, 8 and 16](non_base2/README.md) | base prefixes; `036`; `0x1f32`; `{:#x}`; `from_str_radix`; two's complement |
| §2.3.2, tables 2.1 and 2.2 | — | [Tables 2.1 and 2.2, run: numeric types and their bits](scalar_number_types/README.md) | the bit patterns of 20; `i128` and `u128`; special float values; what decides the width of `usize` |
| §2.3.3 | — | [What *Rust in Action* §2.3.3 says about comparing numbers, run](../../19_Numbers/comparing_numbers_claims_checked/README.md) | fifteen claims: the tolerance example, comparing two number types, `NaN`, the CPU-exception footnote |
| §2.3.4, listing 2.6 | [ch2/ch2-complex ↗](https://github.com/rust-in-action/code/tree/1st-edition/ch2/ch2-complex) | [*Rust in Action* §2.3.4, run](../../19_Numbers/other_number_types/number_types_claims_checked/README.md) · [Other number types](../../19_Numbers/other_number_types/README.md) | complex numbers with the `num` crate, and the number types std does not have |
| §2.4, listings 2.7 and 2.8 | — | [What *Rust in Action* says about flow control, run](../../25_Control_Flow/flow_control_claims_checked/README.md) · [Flow control](../../25_Control_Flow/flow_control/README.md) | twenty-one claims from `if` to `loop` labels, one folder per construct |

## What the pages found

All three listings compile and run on rustc 1.98.0 without a warning. The outputs of listings 2.3 and 2.4 match the book exactly. These are the places where the text and the compiler disagree, or where the text stops short:

| Where | The book, paraphrased | rustc 1.98.0 |
|---|---|---|
| §2.2.1, before listing 2.2 | the listing prints `a + b = 30` | it prints [`( a + b ) + ( c + d ) = 90`](first_steps/README.md#what-the-listing-prints) |
| §2.2.1, the note | a semicolon after `i + j` makes `add` return `()` | [`E0308`](first_steps/README.md#a-semicolon-after-i-j-is-an-error-not-a-unit-return): `add` no longer compiles |
| §2.2.1, p. 36 | `let c = 30_i32;` is line 5; `fn add` is line 10 | [`let d = 30_i32;` is line 5; `fn add` is line 11](first_steps/README.md#line-numbers-and-names-in-the-prose) |
| §2.2.1, p. 36 | a macro returns code rather than values | [the code has a value](first_steps/README.md#eight-claims-run): `println!` is `()` |
| §2.2.1, footnote | `main` returns `()` or a `Result` | [any `Termination` type](first_steps/README.md#main-starts-a-binary-not-every-crate), `ExitCode` included |
| §2.3.1 | you declare a type's size in bytes | [the name counts bits](intro_to_numbers/README.md#nine-claims-run): an `i32` is 4 bytes |
| §2.3.1 | conversions between types are always explicit | [true of numbers](intro_to_numbers/README.md#nine-claims-run); `&String` to `&str` is a silent coercion |
| listing 2.3, line 9 | its note covers the underscores; nothing says the `: i64` matters | [needed](intro_to_numbers/README.md#with-no-type-pow-does-not-compile): `E0689` without it, [overflow](intro_to_numbers/README.md#as-an-i32-the-square-overflows) as `i32` |
| table 2.1 | integers from 8 to 64 bits | [`i128` and `u128` too](scalar_number_types/README.md#seven-claims-run), stable since 1.26.0 |
| table 2.1 | `isize` and `usize` take the CPU's native width | [the pointer width](scalar_number_types/README.md#isize-and-usize-follow-the-pointer-not-the-cpu): 32 bits on 64-bit `x86_64-unknown-linux-gnux32` |
| §2.3.2 | unsigned integers represent only positive numbers | [and zero](scalar_number_types/README.md#seven-claims-run) |
| §2.3.2 | floats represent real numbers | [a finite set of binary fractions](scalar_number_types/README.md#seven-claims-run): `0.1` is not among them |

## Concept index

Every concept chapter 2 has shown so far: where the book shows it, which page here checks it, and the lesson that teaches it.

| Concept or keyword | In the book | Checked on | The lesson |
|---|---|---|---|
| `let`, immutability | listing 2.2, lines 2–5 | [2.2, claim 4](first_steps/README.md#eight-claims-run) | [Variables](../../15_First_Programs/variables/README.md) · [The shadowing map](../../SHADOWING.md) |
| type inference, integer fallback | listing 2.2, line 2; listing 2.3, line 2 | [2.2, claim 2](first_steps/README.md#eight-claims-run) · [2.3, claim 1](intro_to_numbers/README.md#nine-claims-run) | [Type inference](../../15_First_Programs/type_inference/README.md) |
| type annotation | listing 2.2, line 3; listing 2.3, lines 3 and 9 | [2.2, claim 3](first_steps/README.md#eight-claims-run) · [`E0689`](intro_to_numbers/README.md#with-no-type-pow-does-not-compile) | [What a type annotation does](../../15_First_Programs/what_an_annotation_does/README.md) |
| literal suffix, `_` separator | listing 2.2, lines 4–5; listing 2.3 | [2.2, claim 3](first_steps/README.md#eight-claims-run) | [Writing a number down](../../19_Numbers/writing_a_number_down/README.md) |
| `fn`, parameters, `->` | listing 2.2, line 11 | [2.2, refusals](first_steps/README.md#parameters-need-types-the-return-type-does-not) | [Functions](../../25_Control_Flow/functions/README.md) |
| tail expression, `;`, `()` | listing 2.2, line 12, and the note | [2.2, the semicolon](first_steps/README.md#a-semicolon-after-i-j-is-an-error-not-a-unit-return) | [A block is an expression](../../15_First_Programs/a_block_is_an_expression/README.md) · [The unit type `()`](../../15_First_Programs/the_unit_type/README.md) |
| entry point, `main` | listing 2.2, line 1, and footnote 1 | [2.2, `main`](first_steps/README.md#main-starts-a-binary-not-every-crate) | [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) |
| macros, `println!`, `{}` | listing 2.2, line 8 | [2.2, claim 5](first_steps/README.md#eight-claims-run) · [`Display`](first_steps/README.md#is-checked-against-display-at-compile-time) | [Macros](../../25_Control_Flow/macros/README.md) · [Debug and Display](../../15_First_Programs/debug_vs_display/README.md) |
| `char` versus string | §2.2.1, p. 36 | [2.2, claim 8](first_steps/README.md#eight-claims-run) | [Meet the `char`](../../14_Strings/meet_the_char/README.md) |
| methods on numbers, overflow | listing 2.3, line 10 | [2.3, claim 2](intro_to_numbers/README.md#nine-claims-run) | [Meet the byte](../../19_Numbers/meet_the_byte/README.md) · [Signed overflow](../../31_C_and_Cpp/signed_overflow/README.md) |
| arrays, indexing | listing 2.3, lines 12–18 | [2.3, claims 3 and 9](intro_to_numbers/README.md#nine-claims-run) | [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) |
| format spec: width, fill, base | listing 2.3, line 18; listing 2.4 | [2.3, claim 4](intro_to_numbers/README.md#nine-claims-run) · [2.4, claim 5](non_base2/README.md#seven-claims-run) | [The format mini-language](../../14_Strings/the_format_language/README.md) |
| rounding a float | §2.3.1 | [2.3, claim 5](intro_to_numbers/README.md#nine-claims-run) | [Making a float whole](../../19_Numbers/rounding_a_float/README.md) |
| conversions, coercion | §2.3.1 | [2.3, claim 7](intro_to_numbers/README.md#nine-claims-run) | [`From` and `Into`](../../29_Conversion/from_and_into/README.md) · [Coercion](../../29_Conversion/coercion/README.md) · [Casting with `as`](../../29_Conversion/casting_with_as/README.md) |
| operator overloading | §2.3.1 | [2.3, claim 8](intro_to_numbers/README.md#nine-claims-run) | [Operators are traits](../../12_Traits/operators_are_traits/README.md) |
| base 2, 8 and 16 | listing 2.4 | [2.4, claims 1–6](non_base2/README.md#seven-claims-run) | [Why hexadecimal](../../19_Numbers/why_hexadecimal/README.md) |
| two's complement | table 2.2 | [2.4, claim 7](non_base2/README.md#seven-claims-run) · [tables, claim 4](scalar_number_types/README.md#seven-claims-run) | [Meet the byte](../../19_Numbers/meet_the_byte/README.md#two-readings-of-the-same-eight-bits) |
| integer widths, `i128` | table 2.1 | [tables, claims 2–3](scalar_number_types/README.md#seven-claims-run) | [Values](../../15_First_Programs/values/README.md) · [What `i128` is exact about](../../09_Advanced/i128_exactness/README.md) |
| `isize`, `usize` | table 2.1 | [tables, pointer width](scalar_number_types/README.md#isize-and-usize-follow-the-pointer-not-the-cpu) | [Values](../../15_First_Programs/values/README.md) |
| IEEE 754 floats, NaN, `-0.0` | table 2.2 | [tables, claims 1 and 5–7](scalar_number_types/README.md#seven-claims-run) | [What a float actually stores](../../19_Numbers/what_a_float_stores/README.md) |

The one-sentence definitions are in the [glossary](../../GLOSSARY.md), and each error code these pages meet (`E0277`, `E0284`, `E0308`, `E0384`, `E0601`, `E0689`) has its lesson in [the error index](../../ERRORS.md).

## Po polsku

*Rust in Action* Tima McNamary (Manning, 2021) uczy programowania systemowego na przykładzie Rusta. Te strony uruchamiają listingi z książki na rustc 1.98.0, sprawdzają twierdzenia z tekstu wokół nich i odsyłają każde pojęcie do lekcji w tej bibliotece. Kod z książki jest tylko linkowany, nie kopiowany — repozytorium nie ma licencji.

Z rozdziału 2 wynika na razie tyle: listing 2.2 wypisuje `= 90`, a nie zapowiadane `a + b = 30`; średnik po `i + j` nie sprawia, że funkcja zwraca `()`, tylko powoduje błąd `E0308`; adnotacja `: i64` w listingu 2.3 jest konieczna; nazwy typów liczą bity, nie bajty; tabela 2.1 pomija `i128` i `u128`; a `usize` ma szerokość wskaźnika, nie procesora.

**Szukaj po polsku:** Rust in Action po polsku · podstawy języka Rust · literały liczbowe · wnioskowanie typów
