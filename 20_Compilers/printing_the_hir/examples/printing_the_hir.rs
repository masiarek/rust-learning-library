//! What `-Zunpretty=hir` prints for a `for` loop and a `?`, written back out
//! by hand in stable Rust and run beside the originals. The HIR itself calls
//! `Try::branch`, which is nightly-only, so the `?` half uses the form the
//! Reference gives for a `Result`: `Err(e)` returns `Err(From::from(e))`.
//!
//!   rustc --edition 2024 printing_the_hir.rs -o /tmp/pth && /tmp/pth

use std::num::ParseIntError;

fn sum(end: i32) -> i32 {
    let mut total = 0;
    for i in 0..end {
        total += i;
    }
    total
}

/// The loop as the HIR prints it: `0..end` is a `Range` literal, the loop is
/// a `match` on `IntoIterator::into_iter`, and each turn is a `match` on
/// `Iterator::next` whose `None` arm is the only way out.
#[allow(clippy::match_single_binding, clippy::while_let_loop)] // written the HIR's way on purpose
fn sum_desugared(end: i32) -> i32 {
    let mut total = 0;
    match IntoIterator::into_iter(std::ops::Range { start: 0, end }) {
        mut iter => loop {
            match Iterator::next(&mut iter) {
                None => break,
                Some(i) => {
                    total += i;
                }
            }
        },
    }
    total
}

fn double(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

/// The `?` as a `match`: carry on with the value, or return early with the
/// error passed through `From::from`.
#[allow(clippy::question_mark, clippy::useless_conversion)] // the `?` spelled out on purpose
fn double_desugared(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = match s.parse::<i32>() {
        Ok(val) => val,
        Err(residual) => return Err(From::from(residual)),
    };
    Ok(n * 2)
}

fn rule(title: &str) {
    println!("\n──── {title}");
}

fn main() {
    rule("for, as written and as the HIR prints it");
    for end in [0, 3, 5] {
        let (a, b) = (sum(end), sum_desugared(end));
        println!(
            "  end = {end}   sum = {a:<2}  sum_desugared = {b:<2}  same: {}",
            a == b
        );
    }

    rule("?, as written and as a match");
    for input in ["21", "x", ""] {
        let (a, b) = (double(input), double_desugared(input));
        println!("  {:<5} {a:?}", format!("{input:?}"));
        println!("  {:<5} {b:?}   same: {}", "", a == b);
    }
}
