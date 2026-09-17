//! Kata solution: the grade table with a hole in it.
//!
//! One function, three spellings, and then the same mistake made in each: the
//! C band deleted. Two of the three still compile.
//!
//!   rustc --edition 2024 match_expressions_kata.rs -o /tmp/mxk && /tmp/mxk

/// Spelling 1: an `if` ladder. Order matters, and nothing checks the rungs.
fn grade_ladder(score: u8) -> char {
    if score > 100 {
        '?'
    } else if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}

/// Spelling 2: a `match` whose last arm is `_`.
fn grade_catch_all(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    }
}

/// Spelling 3: a `match` that names every `u8`, including the invalid ones.
fn grade_named(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        101..=u8::MAX => '?',
    }
}

/// Spelling 1 with the C rung deleted. Compiles.
fn grade_ladder_holed(score: u8) -> char {
    if score > 100 {
        '?'
    } else if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}

/// Spelling 2 with the C arm deleted. Compiles.
fn grade_catch_all_holed(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    }
}

fn main() {
    println!("1. Three spellings, checked against each other on every u8");
    let disagreements = (0..=u8::MAX)
        .filter(|&s| {
            let a = grade_ladder(s);
            a != grade_catch_all(s) || a != grade_named(s)
        })
        .count();
    println!("   scores checked: 256, disagreements: {disagreements}");
    for s in [95u8, 75, 42, 101, 255] {
        println!(
            "   {s:>3} -> ladder {}  catch-all {}  named {}",
            grade_ladder(s),
            grade_catch_all(s),
            grade_named(s)
        );
    }

    println!();
    println!("2. Delete the C band from each, and predict before compiling");
    for s in [75u8, 79] {
        println!(
            "   {s} -> ladder {}  catch-all {}  named: does not compile",
            grade_ladder_holed(s),
            grade_catch_all_holed(s)
        );
    }
    println!("   The ladder hands 70..=79 to the next rung down, so a C is a D.");
    println!("   The catch-all hands them to `_`, so a C is an invalid score.");
    println!("   Both compile without a warning, and both are wrong.");
    println!("   The named match is error[E0004]: non-exhaustive patterns:");
    println!("   `70_u8..=79_u8` not covered. It names the hole exactly, because");
    println!("   no arm was allowed to mean \"everything else\".");
}
