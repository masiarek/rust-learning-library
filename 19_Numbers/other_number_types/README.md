# Other number types: what Rust sends you to a crate for

**Level:** 101 → 201 · working knowledge

**One line:** Rust's standard library stops at fixed-width integers and binary floats. Complex numbers, fractions, integers with no ceiling and decimals all come from crates, where Python ships all four itself — and `num`, the crate most books name first, covers three of them, not four.

| You want | Python ships | Rust's std has | Rust crate | Page |
|---|---|---|---|---|
| A complex number | `complex`, with a literal: `2.1-1.2j` | nothing | [`num::complex::Complex` ↗](https://docs.rs/num-complex/0.4.6/num_complex/struct.Complex.html) | [Complex numbers](complex_numbers/README.md) |
| An exact fraction | `fractions.Fraction` | nothing | [`num::rational::Ratio` ↗](https://docs.rs/num-rational/0.4.2/num_rational/struct.Ratio.html) | [Rationals](rational_numbers/README.md) |
| An integer with no ceiling | `int` — every one of them | `i128` and `u128`, 39 digits | [`num::BigInt` ↗](https://docs.rs/num-bigint/0.4.8/num_bigint/struct.BigInt.html) | [Big integers](big_integers/README.md) |
| A decimal, for money | `decimal.Decimal` | nothing — count whole cents in an integer | [`rust_decimal` ↗](https://docs.rs/rust_decimal/1.43.0/rust_decimal/) | [Decimals](decimal_numbers/README.md) |
| A float with 50 digits | `decimal`, after `getcontext().prec = 50` | nothing | [`bigdecimal` ↗](https://docs.rs/bigdecimal/0.4.10/bigdecimal/), [`dashu-float` ↗](https://docs.rs/dashu-float/0.6.0/dashu_float/) | [Decimals — when 28 digits is not enough](decimal_numbers/README.md#when-28-digits-is-not-enough) |

The *nothing* is measured: the item index of the rendered std docs for 1.98.0 (`share/doc/rust/html/std/all.html`) has no item named `Complex`, `Ratio`, `Rational`, `BigInt` or `Decimal`.

## What `num` actually contains

`num` 0.4.3 writes almost no code of its own. Its `src/lib.rs` re-exports six smaller crates, and `cargo add num` locks these versions:

| Module | Crate | What you get |
|---|---|---|
| `num::bigint` | `num-bigint` 0.4.8 | `BigInt`, `BigUint` |
| `num::complex` | `num-complex` 0.4.6 | `Complex`, `Complex32`, `Complex64` |
| `num::rational` | `num-rational` 0.4.2 | `Ratio`, `Rational32`, `Rational64`, `BigRational` |
| `num::integer` | `num-integer` 0.1.47 | the `Integer` trait — `gcd`, `lcm`, `div_floor` |
| `num::iter` | `num-iter` 0.1.46 | `range_step` and friends |
| `num::traits` | `num-traits` 0.2.19 | `Zero`, `One`, `Num`, `Float` — for code generic over number types |

There is no decimal type and no arbitrary-precision float in it. Those are separate crates with separate authors: `rust_decimal`, `bigdecimal`, `dashu-float`, and `rug`, which wraps the C libraries GMP and MPFR. You can also depend on one piece directly. `cargo add num-rational` still pulls in `num-bigint`, `num-integer` and `num-traits` (`cargo tree` lists them), and leaves out `num-complex` and `num-iter`.

## Getting a crate into a program

Every crate program on these pages is `src/main.rs` in a Cargo project:

```sh
cargo new numbers && cd numbers
cargo add num
cargo run
```

```text title="Real output — cargo 1.98.0, cargo add num"
    Updating crates.io index
      Adding num v0.4.3 to dependencies
             Features:
             + std
             - alloc
             - libm
             - num-bigint
             - rand
             - serde
    Updating crates.io index
     Locking 8 packages to latest Rust 1.98.0 compatible versions
```

That writes `num = "0.4.3"` under `[dependencies]` — a range, not a pin, as [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) explains. What happens when you skip those commands and compile a loose file instead is [A throwaway that needs a crate](../../05_Tooling/scratch_with_a_crate/README.md).

**Which parts are machine-checked.** The runner behind this library compiles loose `.rs` files with `rustc` and cannot link a crate. So each page has two kinds of program. The std-only one sits in `examples/` and is checked like every other example here. The crate programs and the Python ones were run by hand — Cargo on rustc 1.98.0, Python 3.14.7 — and each trailing comment is checked against what the program printed.

## The pages

| Lesson | Level | What it teaches |
|---|---|---|
| [Complex numbers](complex_numbers/README.md) | 101 → 201 | Python has a `complex` literal and Rust has a two-field struct from `num`. `a + b` works only because the crate implements `Add`, and `(-1) ** 0.5` changes type in Python where Rust hands back `NaN` |
| [Rationals](rational_numbers/README.md) | 201 | `Ratio` and `Fraction` are one design. The differences are a ceiling that panics in debug and gives a wrong answer in release, a tie that rounds away from zero, and no literal, because `new` has to reduce |
| [Big integers](big_integers/README.md) | 101 → 201 | `u128` holds 34! and not 35!. `BigInt` grows like Python's `int` but divides like Rust: `/` truncates where Python's `//` floors, and floor division on a primitive is still unstable |
| [Decimals](decimal_numbers/README.md) | 201 | Whole cents first. Then `rust_decimal` next to `Decimal`: both keep 28 digits, but one overflows where the other rounds, and they build `0.1` from a float differently. Plus 50 digits of √2 three ways |
| [*Rust in Action* §2.3.4, run](number_types_claims_checked/README.md) | 101 → 201 | The book section that introduces `num`, checked claim by claim: the listing's printed output, what `num` covers, `use`, literals, `new`, and `cargo-edit` |

## If you are coming from Python

Python has a **numeric tower**: `int`, `Fraction`, `float` and `complex` mix freely, and an expression is promoted to the widest type in it. Rust has no tower. Every operator is a trait implementation between two named types, so a crate supports exactly the pairs its author wrote:

```python title="Python 3.14.7"
from decimal import Decimal
from fractions import Fraction

print(1 + Fraction(1, 2))         # 3/2
print(Fraction(1, 2) + 0.5)       # 1.0
print(1 + 2j + Fraction(1, 2))    # (1.5+2j)
Decimal('0.1') + 0.1              # TypeError: unsupported operand type(s) for +: 'decimal.Decimal' and 'float'
```

```rust title="src/main.rs — cargo 1.98.0, num 0.4.3"
use num::rational::Ratio;

fn main() {
    println!("{}", Ratio::new(1, 2) + 1); // 3/2
    // Ratio::new(1, 2) + 0.5            // E0277: cannot add `{float}` to `Ratio<{integer}>`
}
```

Two things transfer. `Decimal` refuses a float in both languages, for the same reason: the float is already inexact. And `Ratio + integer` works, because `num-rational` implements it. What changes is that Python's quiet promotion from `Fraction` to `float` has no Rust version. The line that would lose exactness does not compile, so you convert on purpose or not at all. See [Operators are traits](../../12_Traits/operators_are_traits/README.md) for why every pairing is a separate `impl`.

The Python library's [crosswalk ↗](https://masiarek.github.io/python-learning-library/CROSSWALK.html#numbers-the-standard-library-leaves-out) has the same map as one table, starting from the Python name.

## See also

- [What a float actually stores](../what_a_float_stores/README.md) — the one type std does give you, and why `0.1` is not 0.1
- [Making a float whole](../rounding_a_float/README.md) — the tie rules that come back in `Ratio::round` and `Decimal::round`
- [Writing a number down](../writing_a_number_down/README.md) — the range each built-in width promises
- [Comparing two numbers of different types](../comparing_two_number_types/README.md) — *Rust in Action* §2.3.3, the section before this one: the conversion Rust makes you write before `==`, where Python compares exact values ([Comparing an `int` with a `float` ↗](https://masiarek.github.io/python-learning-library/03_Numbers/comparing_int_and_float/index.html))
- [The Advanced exactness cluster](../../09_Advanced/README.md) — [scaled integers](../../09_Advanced/scaled_integers/README.md), [what `i128` is exact about](../../09_Advanced/i128_exactness/README.md) and [when the denominators compound](../../09_Advanced/compounding_weights/README.md): when a crate is more than the problem needs
- [Machine numbers ↗](https://masiarek.github.io/math-learning-library/01_Precision/machine_numbers/index.html) — the math library on what radix and precision decide, which is the difference between a binary float and a decimal one

## Po polsku

Biblioteka standardowa Rusta (*std*) zna tylko liczby całkowite o stałej szerokości (`i8`…`i128`) i binarne liczby zmiennoprzecinkowe (`f32`, `f64`). Liczby zespolone (*complex numbers*), ułamki (*rationals*), liczby całkowite bez górnej granicy (*big integers*, *arbitrary precision*) i liczby dziesiętne (*decimals*) trzeba dociągnąć z crate'a. Python ma wszystkie cztery w standardzie: `complex`, `fractions.Fraction`, zwykły `int` i `decimal.Decimal`.

Pułapka z książek: crate `num` daje tylko trzy z nich — `Complex`, `Ratio` i `BigInt`. Liczb dziesiętnych i zmiennoprzecinkowych o dowolnej precyzji w nim nie ma; to osobne crate'y (`rust_decimal`, `bigdecimal`, `dashu-float`, `rug`). Druga różnica: Python sam awansuje typy w wyrażeniu (`Fraction + float` daje `float`), a Rust nie — każda para typów to osobna implementacja cechy (*trait*), więc linia, która zgubiłaby dokładność, po prostu się nie kompiluje.

**Szukaj po polsku:** liczby zespolone w Ruscie · ułamki `rust num rational` · `rust bigint` · `rust_decimal` pieniądze · typy liczbowe Python a Rust
