//! Kata solution: the test that could not see it, and the one that passed
//! for the wrong panic.
//!
//!   rustc --edition 2024 where_a_test_goes_kata.rs -o /tmp/wtgk && /tmp/wtgk
//!   rustc --edition 2024 --test where_a_test_goes_kata.rs -o /tmp/wtgt && /tmp/wtgt

/// Public: an integration test in tests/ can call this.
pub fn tally(line: &str) -> Option<u32> {
    let cells: Vec<&str> = line.split(',').collect();
    let mut total = 0;
    for c in &cells {
        total += parse_cell(c)?;
    }
    Some(total)
}

/// Private: only a test inside this module can call it.
fn parse_cell(cell: &str) -> Option<u32> {
    match cell.trim().parse::<u32>() {
        Ok(n) if n <= 5 => Some(n),
        _ => None,
    }
}

fn panics_two_ways(mode: u32) -> u32 {
    let scores = [5u32, 3];
    if mode == 0 {
        scores[std::hint::black_box(9)]          // index out of bounds
    } else {
        panic!("the ballot was never counted")   // a completely different bug
    }
}

fn caught(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(f);
    std::panic::set_hook(hook);
    match r {
        Ok(()) => "(no panic)".into(),
        Err(e) => e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "(non-string panic)".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_a_good_line() {
        assert_eq!(tally("5,3,0"), Some(8));
    }

    /// Only reachable from in here. An integration test cannot name it.
    #[test]
    fn rejects_a_score_above_five() {
        assert_eq!(parse_cell("9"), None);
        assert_eq!(parse_cell(" 3 "), Some(3));
    }

    /// The careless version: passes on ANY panic.
    #[test]
    #[should_panic]
    fn out_of_bounds_careless() {
        panics_two_ways(1);   // not the panic this test is named after
    }

    /// The version that says which panic it means.
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn out_of_bounds_careful() {
        panics_two_ways(0);
    }
}

fn main() {
    println!("1. What each kind of test can reach");
    println!("   tally(\"5,3,0\")   = {:?}   pub — an integration test can call it",
             tally("5,3,0"));
    println!("   parse_cell(\"9\")  = {:?}        private — only a unit test can",
             parse_cell("9"));
    println!("   An integration test in tests/api.rs is a SEPARATE CRATE. It writes");
    println!("   `use my_crate::tally;` and there is no spelling of parse_cell that");
    println!("   works: E0603, private function. That is the boundary doing its");
    println!("   job, and the reason to put the parse_cell tests inline.");

    println!();
    println!("2. The should_panic that tests nothing");
    println!("   panics_two_ways(0) -> {}", caught(|| { panics_two_ways(0); }));
    println!("   panics_two_ways(1) -> {}", caught(|| { panics_two_ways(1); }));
    println!("   A bare #[should_panic] passes on BOTH. So a test named");
    println!("   `out_of_bounds` goes green when the function panics for an");
    println!("   entirely unrelated reason — including a panic introduced by the");
    println!("   very refactor the test was there to catch.");
    println!("   #[should_panic(expected = \"index out of bounds\")] passes on the");
    println!("   first only. The `expected` string is a SUBSTRING match, so it need");
    println!("   not be the whole message, and it should be the part that is about");
    println!("   the failure rather than the part about the data.");

    println!();
    println!("3. Running this file as a test binary");
    println!("   rustc --edition 2024 --test where_a_test_goes_kata.rs");
    println!("   ...gives the harness that `cargo test` would run for the unit");
    println!("   half. The integration half needs a package, because a separate");
    println!("   crate needs something to link against.");

    println!();
    println!("4. Where to put a test, decided in one question");
    println!("   Can the behaviour be observed through the public API?");
    println!("     yes -> tests/, as an integration test. It survives a refactor of");
    println!("            the internals, which is what makes it worth keeping.");
    println!("     no  -> #[cfg(test)] beside the code. And then ask whether the");
    println!("            thing being tested should be public, because sometimes");
    println!("            the honest answer is that the API is missing a method.");
}
