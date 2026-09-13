//! Kata solution: a sum over standard input that survives blank lines, spaces,
//! a bad line, Windows line endings and a last line with no newline — and the
//! loop that looks right and never ends.
//!
//!   rustc --edition 2024 reading_stdin_kata.rs -o /tmp/rsk && /tmp/rsk

use std::io::{self, BufRead, Cursor};

struct Tally {
    sum: u64,
    counted: usize,
    rejected: Vec<(usize, String)>, // (line number, what was there)
}

/// One number per line. Blank lines are skipped, spaces around a number are
/// fine, a line that is not a number is reported with its line number and the
/// sum goes on. The reader is any `BufRead`, so `main` can test it on a script.
fn sum_lines(input: &mut impl BufRead) -> io::Result<Tally> {
    let mut tally = Tally { sum: 0, counted: 0, rejected: Vec::new() };
    let mut line = String::new();
    let mut lineno = 0;
    loop {
        line.clear(); //                       (a) the buffer is reused
        if input.read_line(&mut line)? == 0 {
            break; //                          (b) Ok(0): nothing left
        }
        lineno += 1;
        let text = line.trim(); //             (c) the newline, \r\n or not, and the spaces
        if text.is_empty() {
            continue;
        }
        match text.parse::<u64>() {
            Ok(v) => {
                tally.sum += v;
                tally.counted += 1;
            }
            Err(_) => tally.rejected.push((lineno, text.to_string())),
        }
    }
    Ok(tally)
}

fn report(label: &str, typed: &str) {
    let t = sum_lines(&mut Cursor::new(typed)).unwrap();
    println!("   {label}: {typed:?}");
    println!("      sum {}  from {} numbers", t.sum, t.counted);
    for (n, what) in &t.rejected {
        println!("      line {n}: {what:?} is not a number, skipped");
    }
}

fn main() {
    println!("THE SUM, OVER FOUR THINGS THAT ARRIVE ON A REAL STDIN");
    report("plain    ", "3\n4\n5\n");
    report("messy    ", "3\n4\n\nfive\n 5 \n6");
    report("windows  ", "1\r\n2\r\n");
    report("nothing  ", "");

    println!();
    println!("THE LOOP THAT LOOKS RIGHT AND NEVER ENDS");
    println!("   while let Ok(_) = input.read_line(&mut line) {{ ... }}");
    println!("   Ok(0) is a success, so this condition is true at the end of the input,");
    println!("   and at the end of the input, and at the end of the input. Bounded to five");
    println!("   turns here so the page can show it:");
    let mut input = Cursor::new("7\n");
    let mut line = String::new();
    let mut turns = 0;
    while let Ok(n) = input.read_line(&mut line) {
        turns += 1;
        println!("      turn {turns}: Ok({n})   buffer {line:?}");
        if turns == 5 {
            println!("      ...still Ok(0), still looping. The guard is `n == 0`, and it is yours to write.");
            break;
        }
    }
    println!("   And without clear() the buffer kept the 7: read_line appends, even nothing.");
}
