//! Exercise 2, solved: `pub mod m2`, the book's fix, so that `m3` can reach
//! `crate::m1::m2::D`. Both structs get a function that builds one.
//!
//!   rustc --edition 2024 pg6_02_module_paths.rs -o /tmp/pg602 && /tmp/pg602

mod m1 {
    struct A {
        d: m2::D,
    }

    pub mod m2 {
        #[derive(Debug)]
        pub enum D {
            B,
            C,
        }
    }

    pub fn build_a() -> String {
        let a = A { d: m2::D::B };
        format!("m1::A {{ d: {:?} }}", a.d)
    }
}

mod m3 {
    struct C {
        e: crate::m1::m2::D,
    }

    pub fn build_c() -> String {
        let c = C { e: crate::m1::m2::D::C };
        format!("m3::C {{ e: {:?} }}", c.e)
    }
}

fn main() {
    println!("{}", m1::build_a()); // m1::A { d: B }
    println!("{}", m3::build_c()); // m3::C { e: C }
    println!("m3 reaches D only because m2 is pub AND D is pub: two doors, both open");
}
