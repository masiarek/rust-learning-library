//! The four lines that wrap a unit test, seen from a normal (non-test) build.
//! The page is 28_Testing/a_first_test_step_by_step/the_test_comes_first/README.md.
//!
//!   rustc --edition 2024 the_test_comes_first.rs -o /tmp/t && /tmp/t
//!   rustc --edition 2024 --test the_test_comes_first.rs -o /tmp/t && /tmp/t

// Exactly one of these two constants exists in any build. `rustc --test` (and
// so `cargo test`) sets the `test` cfg; a normal build does not.
#[cfg(test)]
const BUILD: &str = "a test build";
#[cfg(not(test))]
const BUILD: &str = "a normal build";

mod data {
    // Private: nothing outside `data` can call it -- except a child module.
    fn row_count() -> usize {
        4
    }

    pub mod checks {
        use super::*; // `super` is `data`, the parent module, not "the file"

        pub fn report() {
            println!("  running in    {}", module_path!());
            println!("  row_count()   = {}   <- private to data, visible here", row_count());
        }
    }
}

fn main() {
    println!("This is {BUILD}; cfg!(test) = {}", cfg!(test));
    println!();
    println!("A child module reaches its parent's private items through use super::*:");
    data::checks::report();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_is_a_test_build() {
        assert_eq!(BUILD, "a test build");
    }
}
