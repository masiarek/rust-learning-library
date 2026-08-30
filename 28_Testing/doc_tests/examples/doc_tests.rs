//! The example in the documentation is a test, and it runs on every `cargo test`.
//!
//!   rustc --edition 2024 doc_tests.rs -o /tmp/dt && /tmp/dt
//!   rustdoc --edition 2024 --test doc_tests.rs      <- runs the examples below

/// Totals a ballot.
///
/// ```
/// assert_eq!(doc_tests::tally(&[5, 3, 0]), 8);
/// ```
///
/// An empty ballot totals zero:
///
/// ```
/// assert_eq!(doc_tests::tally(&[]), 0);
/// ```
pub fn tally(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

/// Reads one cell of a ballot.
///
/// The setup nobody needs to see is hidden with `#`, so the rendered example
/// is two lines while the compiled one is four:
///
/// ```
/// # use doc_tests::read_cell;
/// # let line = "5,3,0";
/// let cells: Vec<&str> = line.split(',').collect();
/// assert_eq!(read_cell(&cells, 1), Some(3));
/// ```
///
/// A score above 5 is not a score:
///
/// ```
/// # use doc_tests::read_cell;
/// assert_eq!(read_cell(&["9"], 0), None);
/// ```
///
/// And an example that is expected to fail:
///
/// ```should_panic
/// # use doc_tests::read_cell;
/// read_cell(&["5"], 0).expect("cell 0");
/// read_cell(&[], 0).expect("there is no cell 0");
/// ```
pub fn read_cell(cells: &[&str], i: usize) -> Option<u32> {
    match cells.get(i)?.trim().parse::<u32>() {
        Ok(n) if n <= 5 => Some(n),
        _ => None,
    }
}

fn main() {
    println!("1. A doc test is an example that has to keep working");
    println!("   tally(&[5, 3, 0]) = {}", tally(&[5, 3, 0]));
    println!("   read_cell(&[\"5\", \"3\", \"0\"], 1) = {:?}",
             read_cell(&["5", "3", "0"], 1));
    println!("   Every ``` block in a /// comment is compiled and run by");
    println!("   `cargo test`. That is the answer to the oldest problem in");
    println!("   documentation: the example that stopped compiling two releases");
    println!("   ago and nobody noticed.");

    println!();
    println!("2. Each block is a whole program");
    println!("   A doc test is wrapped in `fn main()` for you and compiled as its");
    println!("   own crate, which is why it must `use` your crate by name rather");
    println!("   than by `crate::`. It sees your PUBLIC API only — so a doc test");
    println!("   is an integration test that happens to be printed in the docs.");

    println!();
    println!("3. `#` hides a line from the reader, not from the compiler");
    println!("   A line starting with `# ` is compiled and not rendered. Use it");
    println!("   for the `use` line and the fixture, so the example on the page is");
    println!("   the two lines that matter. Do not use it to hide the part that");
    println!("   makes the example work — a reader who copies what is rendered");
    println!("   should get something that runs.");

    println!();
    println!("4. The four annotations on the fence");
    println!("   ```              compile and run          (the default)");
    println!("   ```no_run        compile, do not run      (network, long jobs)");
    println!("   ```should_panic  run, and expect a panic");
    println!("   ```ignore        do not even compile      (almost always wrong:");
    println!("                    use `text` if it is not Rust)");
    println!("   `ignore` is the one to be suspicious of — it turns a test into a");
    println!("   comment while leaving it looking like a test.");

    println!();
    println!("5. What they are good for, and what they are not");
    println!("   Good: the two-line example a reader needs, kept honest. A doc");
    println!("   test that fails is a documentation bug, which is exactly the");
    println!("   right thing to be told.");
    println!("   Not: exhaustive coverage. They are slower than unit tests (one");
    println!("   compilation each), they only reach the public API, and a doc");
    println!("   comment full of edge cases is a bad doc comment. Put the third");
    println!("   through twentieth case in #[cfg(test)] and leave the first one");
    println!("   on the page.");
}
