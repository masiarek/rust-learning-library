//! One arm, many values: the two ways to widen a `match` pattern.
//!
//!     8 | 12 | 18   an OR-PATTERN — this arm accepts any of three values
//!     0..=7         a RANGE PATTERN — this arm accepts eight
//!     9..=11 | 13..=17   they compose, because `|` joins patterns of any shape
//!
//! Both exist to delete repetition, and both introduce the same hazard: once one
//! arm is wide, a later arm can be dead code. The compiler checks that for you —
//! per ALTERNATIVE, not per arm — which is the part worth reading this for.
//!
//!   rustc --edition 2024 one_arm_many_values.rs -o /tmp/oamv && /tmp/oamv

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
// The expanded version: one arm per hour, which is how this is usually first
// written and is not wrong — only long.
fn commercials_long(hour: u32) -> &'static str {
    match hour {
        0 => "Classic video bundle",
        1 => "Classic video bundle",
        2 => "Classic video bundle",
        3 => "Classic video bundle",
        4 => "Classic video bundle",
        5 => "Classic video bundle",
        6 => "Classic video bundle",
        7 => "Classic video bundle",
        8 => "Food",
        9 => "Clothing",
        10 => "Clothing",
        11 => "Clothing",
        12 => "Food",
        13 => "Clothing",
        14 => "Clothing",
        15 => "Clothing",
        16 => "Clothing",
        17 => "Clothing",
        18 => "Food",
        19 => "Season ticket",
        20 => "Season ticket",
        21 => "Season ticket",
        22 => "Season ticket",
        23 => "Season ticket",
        24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

// The same function, five arms.
fn commercials(hour: u32) -> &'static str {
    match hour {
        0..=7 => "Classic video bundle",
        8 | 12 | 18 => "Food",
        9..=11 | 13..=17 => "Clothing",
        19..=24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

fn step1() {
    banner(1, "Twenty-six arms become five");

    let mut disagreements = 0;
    for hour in 0..=30 {
        if commercials_long(hour) != commercials(hour) {
            disagreements += 1;
            println!("  hour {hour}: long says {:?}, short says {:?}", commercials_long(hour), commercials(hour));
        }
    }
    println!("  checked every hour 0..=30, disagreements: {disagreements}");
    println!("  hour 8 -> {:?}   hour 12 -> {:?}   hour 25 -> {:?}", commercials(8), commercials(12), commercials(25));
    println!("      `8 | 12 | 18` is ONE arm that accepts any of three values, and");
    println!("      `0..=7` is one arm that accepts eight. The two compose — `9..=11 |");
    println!("      13..=17` is a single pattern — because `|` joins patterns, not");
    println!("      numbers. Nothing about the behaviour changed; 21 lines went away.");
}

// ─────────────────────────────────────────────────────────── Step 2
// The halfway state: you write the wide arm, and have not yet deleted the
// single-value arms it now swallows. rustc warns; this file silences it so the
// demo can run, and the README quotes the warning in full.
#[allow(unreachable_patterns)]
fn halfway(hour: u32) -> &'static str {
    match hour {
        8 | 12 | 18 => "Food",
        12 => "Clothing", // dead — 12 was taken by the arm above
        _ => "other",
    }
}

fn step2() {
    banner(2, "The first arm that matches wins, and the rest are dead");

    println!("  halfway(12) -> {:?}", halfway(12));
    println!("      The `12 => \"Clothing\"` arm below it never runs. `match` takes the");
    println!("      FIRST arm that matches, so widening an arm silently retires every");
    println!("      later arm it now covers — and rustc says so:");
    println!("          warning: unreachable pattern");
    println!("          8 | 12 | 18 => ...   matches all the relevant values");
    println!("          12 => ...            ^^ no value can reach this");
    println!("      Here the dead arm agreed with the live one, so deleting it changes");
    println!("      nothing. That is the lucky case, and it is why the warning is worth");
    println!("      reading rather than clearing: it is the same warning either way.");
}

// ─────────────────────────────────────────────────────────── Step 3
// The unlucky case, and the reason this page exists.
#[allow(unreachable_patterns)]
fn too_wide(hour: u32) -> &'static str {
    match hour {
        0..=7 => "Classic video bundle",
        9..=17 => "Clothing",  // too wide: 12 belongs to Food
        8 | 12 | 18 => "Food", // 8 and 18 still reach this arm; 12 does not
        19..=24 => "Season ticket",
        _ => "NOT A VALID HOUR",
    }
}

fn step3() {
    banner(3, "rustc checks each ALTERNATIVE, not each arm");

    for hour in [8u32, 12, 18] {
        let flag = if too_wide(hour) == commercials(hour) { "" } else { "   <- wrong" };
        println!("  too_wide({hour:>2}) -> {:<22} correct: {:?}{flag}", format!("{:?}", too_wide(hour)), commercials(hour));
    }
    println!("      One arm of the pair is still perfectly reachable — 8 and 18 arrive");
    println!("      there every day — so \"is this arm dead?\" is the wrong question, and");
    println!("      a compiler that asked it would have nothing to report. rustc asks it");
    println!("      of each alternative separately and puts the caret under the one that");
    println!("      lost:");
    println!("          9..=17 => ...          matches all the relevant values");
    println!("          8 | 12 | 18 => ...         ^^ no value can reach this");
    println!("      That is a real bug caught for free. The version of this mistake the");
    println!("      compiler CANNOT catch is a range that is too wide with no later arm");
    println!("      to contradict it — nothing is unreachable, and hour 12 just quietly");
    println!("      sells clothing. Step 1's agreement loop is how you find that one.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "Three tokens, two meanings: `8 | 12 | 18`");

    let mask = 8 | 12 | 18;
    println!("  as an expression:  let mask = 8 | 12 | 18;   -> {mask}");
    println!("  as a pattern:      matches!(12, 8 | 12 | 18) -> {}", matches!(12, 8 | 12 | 18));
    println!("  as a pattern:      matches!(30, 8 | 12 | 18) -> {}", matches!(30, 8 | 12 | 18));
    println!("      Identical characters, unrelated jobs. In an EXPRESSION `|` is bitwise");
    println!("      or, and 8|12|18 is the single number 30. In a PATTERN it is");
    println!("      alternation, and the same text means \"any one of these three\" — which");
    println!("      is why 30 does not match it. Position decides, and there is no way to");
    println!("      ask for the other one: a pattern cannot compute.");

    println!("  0..=7 includes 7:  matches!(7, 0..=7) -> {}", matches!(7, 0..=7));
    println!("  0..7  excludes 7:  matches!(7, 0..7)  -> {}", matches!(7, 0..7));
    println!("      `..=` is the inclusive form and the one you almost always want for");
    println!("      hours, scores and grades, because the top of the range is a real");
    println!("      value. `..` stops one short — the same off-by-one as everywhere else.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "Every alternative must bind the same names");

    // Both sides bind `n`, and both bind it as i32 — so this is legal, and it is
    // legal in a `let` too, because between them the two patterns are exhaustive.
    let outcome: Result<i32, i32> = Err(7);
    let (Ok(n) | Err(n)) = outcome;
    println!("  let (Ok(n) | Err(n)) = Err(7);   -> n = {n}");
    println!("      Useful when both arms carry the same thing and you do not care yet");
    println!("      which one you got — a line number, a parsed value, a ballot id.");

    println!("  Some(x) | None => x              -> does not compile");
    println!("      error[E0408]: variable `x` is not bound in all patterns");
    println!("      There is no value for `x` when the None side matched, and Rust will");
    println!("      not invent one. Same names, same types, on every alternative.");

    // Nesting: the `|` can sit inside the constructor rather than outside it.
    let picks = [Some(8u32), Some(9), None];
    for p in picks {
        println!(
            "  {:<8} Some(8 | 12 | 18) -> {:<5} Some(8) | Some(12) | Some(18) -> {}",
            format!("{p:?}"),
            matches!(p, Some(8 | 12 | 18)),
            matches!(p, Some(8) | Some(12) | Some(18)),
        );
    }
    println!("      Those two spellings mean the same thing. The nested form has been");
    println!("      allowed since Rust 1.53; before that `|` only worked at the top of an");
    println!("      arm, which is why older code repeats the constructor.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn step6() {
    banner(6, "A guard covers the whole arm");

    for (hour, fasting) in [(12u32, true), (12, false)] {
        let ad = match hour {
            8 | 12 | 18 if !fasting => "Food",
            8 | 12 | 18 => "Water",
            _ => "other",
        };
        println!("  hour {hour}, fasting {fasting:<5} -> {ad:?}");
    }
    println!("      The `if` is not attached to `18`; it is attached to the arm. Any");
    println!("      alternative may match, and then the guard decides whether the arm");
    println!("      runs at all — if it says no, matching continues at the NEXT arm,");
    println!("      which is how the same three values reach two different answers here.");
    println!("      Note also that a guard makes an arm invisible to the exhaustiveness");
    println!("      checker: the compiler cannot evaluate `!fasting`, so it assumes the");
    println!("      arm might not run and still demands a catch-all.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    println!();
}
