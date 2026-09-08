//! Two streams and one number: what belongs on each, and what a caller can test.
//!
//! The program re-runs ITSELF in six roles and reports what a caller saw --
//! stdout, stderr and the exit status, kept apart. That is the only honest way
//! to show the split, because a program cannot observe its own streams once
//! they have left, and a line reading "this went to stderr" asserts exactly the
//! thing in question.
//!
//! Nothing machine-specific is printed: the rows are fixed and no path,
//! environment variable or clock reaches the output.
//!
//!   rustc --edition 2024 stderr_and_exit_status.rs -o /tmp/ses && /tmp/ses

use std::env;
use std::io::{BufWriter, Write};
use std::process::{exit, Command};

/// The rows the tool "reads". Row 3 is damaged, which is what gives the tool
/// something to say that is not part of its answer.
const ROWS: [&str; 4] = ["alpha beta", "gamma", "- torn -", "delta epsilon zeta"];

fn unreadable(row: &str) -> bool {
    row.starts_with('-')
}

/// The answer: one line per row, ending in the number the caller is after.
fn count_rows(warn_on_stdout: bool) {
    for (i, row) in ROWS.iter().enumerate() {
        if unreadable(row) {
            let complaint = format!("warning: skipping unreadable row {}", i + 1);
            if warn_on_stdout {
                println!("{complaint}");
            } else {
                eprintln!("{complaint}");
            }
            continue;
        }
        println!("{row} -> {}", row.split_whitespace().count());
    }
}

// ------------------------------------------------------------------ the child

/// One run of the tool, in the role named on the command line.
fn child(role: &str) {
    match role {
        // The warning goes where a warning goes.
        "good" => count_rows(false),

        // One word different: `println!` instead of `eprintln!`.
        "sloppy" => count_rows(true),

        // A failure announced to a person and hidden from every program.
        "silent" => {
            eprintln!("error: row 3 is unreadable, giving up");
            exit(0);
        }

        // The identical message, with the number that makes it true.
        "honest" => {
            eprintln!("error: row 3 is unreadable, giving up");
            exit(1);
        }

        // A buffer of your own, and an exit that does not run destructors.
        "lost" => {
            let mut w = BufWriter::new(std::io::stdout().lock());
            writeln!(w, "total 6").expect("the write only fills a buffer");
            exit(0);
        }

        // The same thing, flushed first.
        "flushed" => {
            let mut w = BufWriter::new(std::io::stdout().lock());
            writeln!(w, "total 6").expect("the write only fills a buffer");
            w.flush().expect("stdout is open");
            exit(0);
        }

        other => {
            eprintln!("error: no such role: {other}");
            exit(2);
        }
    }
}

// ----------------------------------------------------------------- the parent

/// What a caller gets back: the two streams, kept apart, and the number.
struct Seen {
    out: String,
    err: String,
    code: i32,
}

fn run(role: &str) -> Seen {
    let exe = env::current_exe().expect("current_exe");
    let done = Command::new(exe).arg(role).output().expect("spawn");
    Seen {
        out: String::from_utf8_lossy(&done.stdout).into_owned(),
        err: String::from_utf8_lossy(&done.stderr).into_owned(),
        code: done.status.code().expect("the child was not killed by a signal"),
    }
}

fn show(label: &str, text: &str) {
    if text.is_empty() {
        println!("   {label:<7} (nothing)");
        return;
    }
    for (n, line) in text.lines().enumerate() {
        let tag = if n == 0 { label } else { "" };
        println!("   {tag:<7} {line}");
    }
}

/// A caller reading the answer: one row per line, the count in the last column.
fn rows_and_total(text: &str) -> (usize, u32) {
    let total = text
        .lines()
        .filter_map(|l| l.split_whitespace().next_back())
        .filter_map(|t| t.parse::<u32>().ok())
        .sum();
    (text.lines().count(), total)
}

fn banner(n: u32, title: &str) {
    println!("\n──── {n}. {title}");
}

fn main() {
    if let Some(role) = env::args().nth(1) {
        child(&role);
        return;
    }

    banner(1, "One run, seen from outside");
    let good = run("good");
    show("stdout", &good.out);
    show("stderr", &good.err);
    println!("   status  {}", good.code);
    println!("   Three rows counted, one row complained about, and the complaint is");
    println!("   not one of the three. Nothing here is a convention the compiler");
    println!("   enforces: `println!` writes to the first, `eprintln!` to the second.");

    banner(2, "What a redirect keeps");
    println!("   prog > rows.txt");
    println!("   {:<17}{:>2} bytes  the {} answer lines", "rows.txt gets", good.out.len(), good.out.lines().count());
    println!("   {:<17}{:>2} bytes  the warning", "the screen keeps", good.err.len());
    println!("   Neither half is lost and neither is in the other's way. A tool that");
    println!("   puts its diagnostics on stdout offers you both or neither.");

    banner(3, "The same warning, one word different");
    let sloppy = run("sloppy");
    show("stdout", &sloppy.out);
    show("stderr", &sloppy.err);
    println!("   status  {}", sloppy.code);
    let (good_rows, good_total) = rows_and_total(&good.out);
    let (bad_rows, bad_total) = rows_and_total(&sloppy.out);
    println!();
    println!("   {:<34}{good_rows} -> {bad_rows}", "a caller counting rows");
    println!("   {:<34}{good_total} -> {bad_total}", "a caller summing the last column");
    println!("   Both runs exited 0 and neither printed anything a person would call");
    println!("   wrong. The warning became data because nothing distinguishes them");
    println!("   once they are on the same stream.");

    banner(4, "The number a script reads");
    let silent = run("silent");
    let honest = run("honest");
    println!("   silent  stderr: {:<40} status {}", silent.err.trim(), silent.code);
    println!("   honest  stderr: {:<40} status {}", honest.err.trim(), honest.code);
    println!("   The sentence is identical; the number is not. `prog && next` runs");
    println!("   `next` after the first and stops at the second, and a CI step");
    println!("   passes the first. Zero is success, non-zero is failure, and that is");
    println!("   nearly the whole convention.");

    banner(5, "process::exit does not run destructors");
    let lost = run("lost");
    let flushed = run("flushed");
    println!("   wrote \"total 6\" into a BufWriter, then exit(0)");
    println!("   without flush  stdout {:?}  status {}", lost.out, lost.code);
    println!("   with flush     stdout {:?}  status {}", flushed.out, flushed.code);
    println!("   The buffer is yours, so no runtime cleanup knows about it, and");
    println!("   `exit` does not unwind — the `Drop` that would have flushed it never");
    println!("   runs. A correct message, reported as success, printed by nobody.");
}
