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
