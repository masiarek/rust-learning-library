//! Exercise 1, solved: `pub enum D`, which is the book's whole fix, plus a
//! function that builds an `A`, so the solution compiles without the two
//! dead_code warnings the book's version prints.
//!
//!   rustc --edition 2024 pg6_01_nested_visibility.rs -o /tmp/pg601 && /tmp/pg601

mod m1 {
    struct A {
        d: m2::D,
    }

    mod m2 {
        pub enum D {
            B,
            C,
        }
    }

    /// `A` and `m2` stay private to `m1`; this is the one door out.
    pub fn describe_both() -> String {
        let first = A { d: m2::D::B };
        let second = A { d: m2::D::C };
        format!("{} and {}", name(&first), name(&second))
    }

    fn name(a: &A) -> &'static str {
        match a.d {
            m2::D::B => "A holding D::B",
            m2::D::C => "A holding D::C",
        }
    }
}

fn main() {
    println!("{}", m1::describe_both()); // A holding D::B and A holding D::C
    println!("D is pub, so m1 can name m2::D; m2 itself stays private to m1");
}
