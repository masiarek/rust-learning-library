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
