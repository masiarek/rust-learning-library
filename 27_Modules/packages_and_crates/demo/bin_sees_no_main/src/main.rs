//! Shared code put in main.rs, where the other binary cannot see it.

mod shared {
    pub fn greeting() -> &'static str {
        "hello"
    }
}

fn main() {
    println!("{}", shared::greeting());
}
