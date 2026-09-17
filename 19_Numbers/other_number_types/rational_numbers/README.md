# Rationals: `num::rational::Ratio` and Python's `Fraction`

[Other number types](../README.md) › **Rationals**

**Level:** 201 · working knowledge

**One line:** `Ratio` and `Fraction` are the same design — a numerator and a denominator, reduced by `gcd` after every operation — so 1/10 + 2/10 is exactly 3/10 in both. The differences are that `Ratio<i64>` has a ceiling (a panic in a debug build, a wrong answer in release), that it rounds a tie away from zero, and that it has no literal, because `new` has to reduce.

```rust title="src/main.rs — cargo 1.98.0, num 0.4.3"
use num::rational::{BigRational, Ratio};

fn main() {
    let total = Ratio::new(1, 10) + Ratio::new(2, 10);
    println!("{total}");                                    // 3/10
    println!("{}", total == Ratio::new(3, 10));             // true
    println!("{}", 0.1 + 0.2 == 0.3);                       // false
    println!("{}", Ratio::new(2, 4));                       // 1/2
    println!("{}", Ratio::new(3, -6));                      // -1/2
    println!("{}", Ratio::new(6, 3));                       // 2
    println!("{}", Ratio::new(5, 2).round());               // 3
    println!("{}", BigRational::from_float(0.1).unwrap());  // 3602879701896397/36028797018963968
}
```

```python title="Python 3.14.7"
from fractions import Fraction

total = Fraction(1, 10) + Fraction(2, 10)
print(total)                     # 3/10
print(total == Fraction(3, 10))  # True
print(0.1 + 0.2 == 0.3)          # False
print(Fraction(2, 4))            # 1/2
print(Fraction(3, -6))           # -1/2
print(Fraction(6, 3))            # 2
print(round(Fraction(5, 2)))     # 2
print(Fraction(0.1))             # 3602879701896397/36028797018963968
```

Seven of the eight lines agree. Both reduce `2/4` to `1/2`, both move a negative denominator's sign up to the numerator, both print a whole number with no `/1`, and both turn the float `0.1` into the exact binary fraction it stores: `3602879701896397/36028797018963968`, which is not 1/10 ([why](../../what_a_float_stores/README.md)). If you want the fraction the float was meant to be, `Rational64::approximate_float(0.1)` gives `Some(Ratio { numer: 1, denom: 10 })` and Python's `Fraction(0.1).limit_denominator()` gives `1/10`.

The line that differs is the tie, and it is one of three differences.

## A fixed-width `Ratio` has a ceiling

`Ratio<T>` stores two `T`s. With `T = i64`, adding two fractions multiplies denominators that can each be up to `i64::MAX`:

```rust title="src/main.rs — cargo 1.98.0, num 0.4.3"
use num::rational::Ratio;

fn main() {
    let a = Ratio::new(1_i64, i64::MAX);
    let b = Ratio::new(1_i64, i64::MAX - 1);
    println!("{}", a + b);
}
```

```text title="Real output — cargo run, abridged: the source path in the panic line is cut"
thread 'main' (28971451) panicked at .../num-integer-0.1.47/src/lib.rs:846:1:
attempt to multiply with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```text title="Real output — cargo run --release"
1/9223372036854775806
```

The debug build stops. The release build prints a fraction that is **half the true sum**, and exits 0. Integer overflow wraps silently in release, as [Meet the byte](../../meet_the_byte/README.md) measures for a `u8`, and a rational built on an integer inherits that. Here is the true sum in both languages:

```rust title="src/main.rs — cargo 1.98.0, num 0.4.3"
use num::BigInt;
use num::rational::BigRational;

fn main() {
    let max = BigInt::from(i64::MAX);
    let a = BigRational::new(1.into(), max.clone());
    let b = BigRational::new(1.into(), max - 1);
    println!("{}", a + b); // 18446744073709551613/85070591730234615838173535747377725442
}
```

```python title="Python 3.14.7"
from fractions import Fraction

MAX = 2**63 - 1
print(Fraction(1, MAX) + Fraction(1, MAX - 1))  # 18446744073709551613/85070591730234615838173535747377725442
```

`BigRational` is `Ratio<BigInt>`, and it is the Rust type that behaves like `Fraction`: no ceiling, a heap allocation for each value, and slower arithmetic as the numbers grow. [What `i128` is exact about](../../../09_Advanced/i128_exactness/README.md#how-this-differs-from-fractionsfraction) measures how far `Ratio<i128>` gets before it hits the ceiling. [When the denominators compound](../../../09_Advanced/compounding_weights/README.md) shows why a long computation reaches it sooner than you would guess.

## A tie rounds away from zero

`Ratio::new(5, 2).round()` is `3`, and Python's `round(Fraction(5, 2))` is `2`. `num-rational` rounds a half **away from zero**, like `f64::round`. Python's `round` rounds a half **to the even neighbour**, for `Fraction`, `float` and `Decimal` alike. Code ported from Python changes on exactly the ties, which is the data where nobody notices. [Making a float whole](../../rounding_a_float/README.md#three-tie-rules-and-they-disagree) covers the same split for `f64`.

## No literal: `new` has to reduce

`Complex { re: 2.1, im: -1.2 }` compiles. The same form for `Ratio` does not:

```rust
use num::rational::Ratio;
fn main() {
    let half = Ratio { numer: 1, denom: 2 };
    println!("{half}");
}
```

```text title="Real output — cargo build, num 0.4.3"
error[E0451]: fields `numer` and `denom` of struct `Ratio` are private
 --> src/main.rs:3:24
  |
3 |     let half = Ratio { numer: 1, denom: 2 };
  |                        ^^^^^     ^^^^^ private field
  |                        |
  |                        private field

For more information about this error, try `rustc --explain E0451`.
```

The fields are private because a `Ratio` has an **invariant**: it is always in lowest terms, with a positive denominator. `num-rational` 0.4.2's `new` is three lines — `new_raw`, then `reduce`. A literal would skip the second one. The escape hatch is public, and it shows what the invariant is for: `Ratio::new_raw(2, 4)` prints `2/4`, while `Ratio::new(2, 4)` prints `1/2`. Here is the same idea with std only:

<!-- source:ratio_new_reduces -->
*[`ratio_new_reduces.rs`](examples/ratio_new_reduces.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// Why a rational type has no literal: `new` is the one door, and it reduces.
mod ratio {
    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Ratio {
        numer: i64, // private, so no code outside this module can write 2/4
        denom: i64,
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a.abs()
    }

    impl Ratio {
        pub fn new(numer: i64, denom: i64) -> Option<Ratio> {
            if denom == 0 {
                return None;
            }
            let g = gcd(numer, denom);
            let sign = if denom < 0 { -1 } else { 1 };
            Some(Ratio {
                numer: sign * numer / g,
                denom: sign * denom / g,
            })
        }
    }

    impl fmt::Display for Ratio {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.denom == 1 {
                write!(f, "{}", self.numer)
            } else {
                write!(f, "{}/{}", self.numer, self.denom)
            }
        }
    }
}

use ratio::Ratio;

fn main() {
    println!("{}", Ratio::new(2, 4).unwrap()); // 1/2
    println!("{}", Ratio::new(3, -6).unwrap()); // -1/2
    println!("{}", Ratio::new(6, 3).unwrap()); // 2
    println!("{}", Ratio::new(0, 5).unwrap()); // 0
    println!("{:?}", Ratio::new(1, 0)); // None

    // Every value is reduced on the way in, so the derived == is right.
    println!("{}", Ratio::new(2, 4) == Ratio::new(1, 2)); // true

    // let half = Ratio { numer: 1, denom: 2 }; // E0451: the fields are private
}
```
<!-- /source -->

<!-- output:ratio_new_reduces -->
*Verified output of [`ratio_new_reduces.rs`](examples/ratio_new_reduces.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1/2
-1/2
2
0
None
true
```
<!-- /output -->

- **The derived `==` is correct only because `new` reduced.** Two `Ratio`s with the same value always have the same fields, so comparing fields compares values. `num`'s `Ratio` does not rely on that: its `PartialEq` compares values, so `Ratio::new_raw(2, 4) == Ratio::new(1, 2)` is `true` there too.
- **A zero denominator has no value.** This `new` returns `None`. `num`'s panics with `denominator == 0`, and Python's `Fraction(1, 0)` raises `ZeroDivisionError: Fraction(1, 0)`. [Partial functions](../../../17_Option_and_Result/partial_functions/README.md) covers why `Option` is the honest return type.
- **Uncomment the last line of `main` and it is `E0451` again**, for the same reason: the fields are private to `mod ratio`. [Modules and visibility](../../../27_Modules/modules_and_visibility/README.md) covers why privacy is per module, and [What an invariant is](../../../09_Advanced/what_an_invariant_is/README.md) covers what the one door buys.

## If you are coming from Python

**What transfers:** everything about the arithmetic. `Fraction` and `Ratio` both store a reduced numerator and denominator, compare by value, reject a zero denominator, convert a float exactly, and approximate one on request. Your mental model of `Fraction` is correct for `BigRational`.

**What changes:**

- **You choose the width, and a narrow one has a ceiling.** Python's `Fraction` sits on `int`, which cannot overflow. `Ratio<i32>`, `Ratio<i64>` and `Ratio<i128>` can. A debug build panics and a release build gives a wrong answer with no warning. Use `BigRational` unless you have a bound you can argue for.
- **Ties round away from zero.** `round(Fraction(5, 2))` is `2`, and `Ratio::new(5, 2).round()` is `3`.
- **No mixing with floats.** `Fraction(1, 2) + 0.5` quietly becomes the float `1.0`. `Ratio::new(1, 2) + 0.5` does not compile. `Ratio + integer` does compile — see [the overview](../README.md#if-you-are-coming-from-python).
- **Parsing takes a fraction, not a decimal.** `"3/10".parse::<Rational64>()` is `Ok(Ratio { numer: 3, denom: 10 })`, like `Fraction('3/10')`. But `Fraction('0.3')` is also `3/10`, while `"0.3".parse::<Rational64>()` is an `Err`.

## See also

- [Other number types](../README.md) — the map
- [Scale the denominator away](../../../09_Advanced/scaled_integers/README.md) — when the denominators are known in advance, and you can skip the `gcd` and count in integers
- [What `i128` is exact about](../../../09_Advanced/i128_exactness/README.md) — `Ratio<i128>` against `Fraction`, measured
- [A type is not a constructor](../../../16_Structs/a_type_is_not_a_constructor/README.md) — the literal, `new`, and why they are not the same thing
- [`num_rational::Ratio` ↗](https://docs.rs/num-rational/0.4.2/num_rational/struct.Ratio.html) · [Python: `fractions` ↗](https://docs.python.org/3/library/fractions.html)

## Po polsku

Ułamek (*rational number*) w Ruscie to `num::rational::Ratio`, a w Pythonie `fractions.Fraction`. Pomysł jest ten sam: licznik (*numerator*) i mianownik (*denominator*), skracane przez NWD (*gcd*) po każdym działaniu. Dlatego 1/10 + 2/10 to dokładnie 3/10, a nie 0.30000000000000004.

Trzy różnice. **Sufit:** `Ratio<i64>` trzyma dwie liczby `i64`, więc może się przepełnić — w buildzie debug program panikuje, w release po cichu wypisuje połowę prawdziwego wyniku. Odpowiednikiem `Fraction` jest `BigRational`, bez sufitu. **Remis przy zaokrąglaniu:** `Ratio::new(5, 2).round()` daje 3 (od zera), a Pythonowe `round(Fraction(5, 2))` daje 2 (do parzystej, tzw. zaokrąglanie bankierskie). **Brak literału:** pola `Ratio` są prywatne (`E0451`), bo typ pilnuje niezmiennika (*invariant*) — ułamek jest zawsze skrócony. Jedyne wejście to `new`, które skraca.

**Szukaj po polsku:** ułamki w Ruscie · `rust num rational overflow` · `rust E0451 private field` · zaokrąglanie bankierskie

---

[Other number types](../README.md) › previous: [Complex numbers](../complex_numbers/README.md) · next: [Big integers](../big_integers/README.md)
