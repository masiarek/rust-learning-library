# Complex numbers: `num::Complex` and Python's `complex`

[Other number types](../README.md) › **Complex numbers**

**Level:** 101 → 201 · for newcomers

**One line:** Python has a complex type built in, with its own literal (`2.1-1.2j`). Rust's std has none, so `num::complex::Complex` supplies a struct with two public fields — and `a + b` works on it only because the crate implements `Add`, which you can write yourself with std in one short file.

```rust title="src/main.rs — cargo 1.98.0, num 0.4.3"
use num::complex::Complex;

fn main() {
    let a = Complex::new(2.1, -1.2);
    let b = Complex { re: 11.1, im: 22.2 };
    println!("{}", a + b);                                    // 13.2+21i
    println!("{} {}", a.re, a.im);                            // 2.1 -1.2
    println!("{}", Complex::new(3.0_f64, 4.0).norm());        // 5
    println!("{}", Complex::<f64>::i() * Complex::i());       // -1+0i
    println!("{}", Complex::new(-1.0_f64, 0.0).sqrt());       // 0+1i
    println!("{}", (-1.0_f64).powf(0.5));                     // NaN
    println!("{}", Complex::new(1, 2) * Complex::new(3, 4));  // -5+10i
}
```

```python title="Python 3.14.7"
import cmath

a = 2.1-1.2j
b = complex(11.1, 22.2)
print(a + b)                          # (13.2+21j)
print(a.real, a.imag)                 # 2.1 -1.2
print(abs(3+4j))                      # 5.0
print(1j * 1j)                        # (-1+0j)
print(cmath.sqrt(-1))                 # 1j
print((-1) ** 0.5)                    # (6.123233995736766e-17+1j)
print(complex(1, 2) * complex(3, 4))  # (-5+10j)
```

A complex number is two numbers, a real part and an imaginary part, where the imaginary unit squared is −1. Both languages print `-1` for that square, in their own notation.

## Where they differ

| | Python | Rust, with `num` |
|---|---|---|
| Writing one | a literal: `2.1-1.2j`, or `complex(2.1, -1.2)` | no literal: [`Complex::new(2.1, -1.2)` ↗](https://docs.rs/num-complex/0.4.6/num_complex/struct.Complex.html#method.new) or the struct literal `Complex { re: 2.1, im: -1.2 }` |
| The two parts | `.real`, `.imag` | `.re`, `.im` — public fields |
| Printed | `(13.2+21j)` | `13.2+21i` |
| Its size | `abs(z)` | [`z.norm()` ↗](https://docs.rs/num-complex/0.4.6/num_complex/struct.Complex.html#method.norm) |
| √−1 | `cmath.sqrt(-1)` is `1j` | `Complex::new(-1.0, 0.0).sqrt()` is `0+1i` |
| Type of the parts | always `float` | any `T`: `Complex<i32>` multiplies integers exactly |

**The trap is the power operator.** In Python, `(-1) ** 0.5` quietly returns a `complex`, with `6.123233995736766e-17` of rounding noise in the real part. In Rust, `(-1.0_f64).powf(0.5)` is `NaN`, because an `f64` operation returns an `f64` and no operation changes the type for you. You get a complex answer only by starting from a `Complex`. Python is not consistent here either: `math.sqrt(-1)` raises `ValueError: expected a nonnegative input, got -1.0`, and only `**` promotes.

## Listing 2.6 from *Rust in Action*

This is the program the book uses to introduce `num`, run as the book says to: `cargo new`, `num = "0.4"` under `[dependencies]`, `cargo run`.

```rust title="src/main.rs — cargo 1.98.0, num 0.4.3"
use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im); // 13.2 + 21i
}
```

```text title="Real output — cargo 1.98.0, abridged to the last three lines"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.47s
     Running `target/debug/ch2-complex`
13.2 + 21i
```

The book prints `13.2 + 21.02i`. The real line is `13.2 + 21i`: `-1.2 + 22.2` is exactly 21 as an `f64`, and `{}` prints a whole float with no `.0`. The `{:?}` in the next section keeps it. The rest of the section is checked claim by claim in [*Rust in Action* §2.3.4, run](../number_types_claims_checked/README.md).

## What `a + b` needs, with std only

Here is the same sum with no crate: a struct, a `new`, and the two trait implementations that `num` provides for you.

<!-- source:complex_by_hand -->
*[`complex_by_hand.rs`](examples/complex_by_hand.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
// What `num::complex::Complex` does for `a + b`, written out with std only.
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    // A function somebody wrote. It sets nothing the literal cannot.
    const fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }
}

// This impl is the whole reason `a + b` compiles.
impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0.0 {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

fn main() {
    let a = Complex { re: 2.1, im: -1.2 }; // the literal
    let b = Complex::new(11.1, 22.2); // the function
    let result = a + b; // Add::add(a, b)

    println!("{} + {}i", result.re, result.im); // 13.2 + 21i
    println!("{result}"); // 13.2+21i
    println!("{result:?}"); // Complex { re: 13.2, im: 21.0 }
    println!("{a}"); // 2.1-1.2i
    let same = Complex { re: 2.1, im: -1.2 } == Complex::new(2.1, -1.2);
    println!("{same}"); // true

    println!("{}", (-1.0_f64).sqrt()); // NaN
}
```
<!-- /source -->

<!-- output:complex_by_hand -->
*Verified output of [`complex_by_hand.rs`](examples/complex_by_hand.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
13.2 + 21i
13.2+21i
Complex { re: 13.2, im: 21.0 }
2.1-1.2i
true
NaN
```
<!-- /output -->

Reading it against the output:

- **`impl Add for Complex` is what makes `a + b` compile.** `+` calls `Add::add`, and a struct with no `Add` impl gets `E0369`. [Operators are traits](../../../12_Traits/operators_are_traits/README.md) covers the rest of the operators.
- **The literal and `new` build the same value**, which is why `==` prints `true`. This `new` only moves its arguments into the fields, and so does `num`'s: `num-complex` 0.4.6 defines it as `Complex { re, im }`. The literal works because both fields are visible. [Rationals](../rational_numbers/README.md#no-literal-new-has-to-reduce) shows a type whose fields are private, where `new` is the only way in. [A type is not a constructor](../../../16_Structs/a_type_is_not_a_constructor/README.md#new-is-a-convention) covers the general rule.
- **`Display` is a choice the type's author makes.** This one prints `2.1-1.2i` rather than `2.1+-1.2i`, like `num`. Python's choice was parentheses and `j`.
- **`{:?}` shows `21.0` where `{}` showed `21`**, which is the difference behind the book's output line.
- **An `f64` has no square root of −1.** It is `NaN`, with no error. The last line is the whole reason the type exists.

## If you are coming from Python

**What transfers:** the model. A complex value is a pair of floats with arithmetic defined on the pair, and both languages give you addition, multiplication, `sqrt`, `exp` and polar form (`cmath.polar` in Python, [`to_polar` ↗](https://docs.rs/num-complex/0.4.6/num_complex/struct.Complex.html#method.to_polar) in `num`). `complex(1, 2) * complex(3, 4)` and `Complex::new(1, 2) * Complex::new(3, 4)` give the same `-5+10`.

**What changes:**

- **No literal, and no `j`.** `2.0i` is *"invalid suffix `i` for float literal"* on rustc 1.98.0. You write `Complex::new`, or get the unit from `Complex::i()` and multiply.
- **Promotion happens only in Python.** Python moves a value up to `complex` when an operation needs it (`**`) and refuses where it does not (`math.sqrt`). Rust never changes a type on its own. A real computation that needs a complex result has to be written with `Complex` from the start, and an `f64` that meets a negative square root gives `NaN` rather than an exception, so check for it.
- **The parts can be integers.** Python's `complex` always holds two `float`s. `Complex<i32>` holds two `i32`s, so Gaussian-integer arithmetic stays exact. Operations that need a float, like `norm` and `sqrt`, exist only when `T` is a float type: `Complex::new(3, 4).norm()` is `E0599`.
- **It is a plain `Copy` struct with public fields.** `z.re = 0.0` is allowed on a `mut` binding, where Python's `z.real = 0` raises `AttributeError: readonly attribute`. Nothing in `Complex` can break, since any pair of numbers is a valid complex number. The next page is about a type where that is not true.

## See also

- [Other number types](../README.md) — the map: what std leaves out and which crate fills each gap
- [Operators are traits](../../../12_Traits/operators_are_traits/README.md) — `Add`, `Mul` and the rest, for your own types
- [What a float actually stores](../../what_a_float_stores/README.md) — why `NaN` is a value and not an error
- [*Rust in Action* §2.3.4, run](../number_types_claims_checked/README.md) — the book's claims about this listing, each checked
- [`num_complex::Complex` ↗](https://docs.rs/num-complex/0.4.6/num_complex/struct.Complex.html) · [Python: `cmath` ↗](https://docs.python.org/3/library/cmath.html)

## Po polsku

Liczba zespolona (*complex number*) to para liczb: część rzeczywista (*real part*) i urojona (*imaginary part*). Python ma ją w standardzie, razem z literałem `2.1-1.2j` — z literą `j`, jak u elektryków, bo `i` oznacza u nich prąd. Rust w bibliotece standardowej nie ma nic takiego, więc sięgamy po `num::complex::Complex`: zwykłą strukturę z dwoma publicznymi polami `re` i `im`.

Dwie różnice, które bolą przy przenoszeniu kodu. Po pierwsze, `a + b` działa tylko dlatego, że crate implementuje cechę `Add` — bez tego kompilator zgłasza `E0369`. Po drugie, Python sam zamienia typ: `(-1) ** 0.5` daje liczbę zespoloną. W Ruscie `(-1.0_f64).powf(0.5)` to po prostu `NaN`, bez żadnego błędu. Książka *Rust in Action* podaje wynik `13.2 + 21.02i`, a program naprawdę wypisuje `13.2 + 21i`.

**Szukaj po polsku:** liczby zespolone Rust · `rust num complex` · `rust impl Add` · `python complex j`

---

[Other number types](../README.md) › next: [Rationals](../rational_numbers/README.md)
