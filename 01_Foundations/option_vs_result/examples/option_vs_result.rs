//! Option vs Result — a runnable, step-by-step walkthrough.
//!
//!   rustc option_vs_result.rs -o ovr && ./ovr
//!
//! Read the source top-to-bottom alongside the output; each step is one idea.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
// Both types are ordinary enums. You could have written them yourself:
//
//     enum Option<T>    { Some(T), None }
//     enum Result<T, E> { Ok(T),   Err(E) }
//
// The ONLY structural difference: Result's sad arm carries a payload.
fn step1() {
    banner(1, "Two enums; the difference is whether the sad arm carries a reason");

    let mut scores = HashMap::new();
    scores.insert("ada", 5);

    println!("  scores.get(\"ada\")   -> {:?}", scores.get("ada"));
    println!("  scores.get(\"zoe\")   -> {:?}", scores.get("zoe"));
    println!("      \"why not?\" has exactly one answer, so None needs no payload.");

    println!("  \"42\".parse::<u32>() -> {:?}", "42".parse::<u32>());
    println!("  \"4x\".parse::<u32>() -> {:?}", "4x".parse::<u32>());
    println!("      \"why not?\" has many answers, so Err carries one.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "match — the form that always works");

    let ballots: Option<u32> = Some(461);
    match ballots {
        Some(n) => println!("  Option: counted {n} ballots"),
        None => println!("  Option: no ballot file loaded"),
    }

    match "5,2,0".parse::<u32>() {
        Ok(n) => println!("  Result: parsed {n}"),
        Err(e) => println!("  Result: could not parse — {e}"),
    }
    println!("      The compiler rejects the match if you forget an arm. That is the whole safety story.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "Shortcuts for when you only care about one arm");

    let winner: Option<&str> = Some("Ada");
    if let Some(name) = winner {
        println!("  if let            -> winner is {name}");
    }

    // let-else: bind, or bail out. The idiomatic guard clause.
    let raw = Some("12");
    let Some(text) = raw else {
        println!("  let-else          -> nothing to parse");
        return;
    };
    println!("  let-else          -> bound {text:?} for the rest of the function");

    let quorum: Option<u32> = None;
    println!("  unwrap_or         -> {}", quorum.unwrap_or(0));

    let m: u32 = Some(7).unwrap_or_else(|| {
        println!("  (this never prints — unwrap_or_else is lazy)");
        999
    });
    println!("  unwrap_or_else    -> {m}");

    let d: u32 = None.unwrap_or_default();
    println!("  unwrap_or_default -> {d}");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "map vs and_then — the nesting trap");

    let raw: Option<&str> = Some("  12  ");
    let trimmed: Option<&str> = raw.map(|s| s.trim());
    println!("  map               -> {trimmed:?}");

    // If your closure itself returns an Option, map nests it:
    let nested: Option<Option<u32>> = trimmed.map(|s| s.parse::<u32>().ok());
    println!("  map (wrong tool)  -> {nested:?}   <- Option<Option<u32>>");

    // and_then flattens. (Same idea as flat_map / bind.)
    let flat: Option<u32> = trimmed.and_then(|s| s.parse::<u32>().ok());
    println!("  and_then          -> {flat:?}");

    // Result has the same pair, plus map_err for the sad arm.
    let ok: Result<u32, String> = "7".parse::<u32>().map_err(|e| format!("bad score: {e}"));
    let no: Result<u32, String> = "x".parse::<u32>().map_err(|e| format!("bad score: {e}"));
    println!("  map_err           -> {ok:?}");
    println!("  map_err           -> {no:?}");
}

// ─────────────────────────────────────────────────────────── Step 5
// `?` means: unwrap the happy value, or return the sad one from THIS function.

fn first_word_len(s: &str) -> Option<usize> {
    let first = s.split_whitespace().next()?; // None here -> the function returns None
    Some(first.len())
}

fn sum_scores(line: &str) -> Result<u32, ParseIntError> {
    let mut total = 0;
    for tok in line.split(',') {
        total += tok.trim().parse::<u32>()?; // Err here -> the function returns that Err
    }
    Ok(total)
}

fn step5() {
    banner(5, "`?` — early return in one character");
    println!("  first_word_len(\"hello world\") -> {:?}", first_word_len("hello world"));
    println!("  first_word_len(\"     \")       -> {:?}", first_word_len("     "));
    println!("  sum_scores(\"5, 2, 0\")         -> {:?}", sum_scores("5, 2, 0"));
    println!("  sum_scores(\"5, x, 0\")         -> {:?}", sum_scores("5, x, 0"));
    println!("      `?` on an Option needs an Option-returning fn; on a Result, a Result-returning fn.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn step6() {
    banner(6, "Crossing between the two");

    let maybe: Option<u32> = None;
    let up: Result<u32, &str> = maybe.ok_or("no quorum recorded");
    println!("  Option -> Result  .ok_or(reason) -> {up:?}");

    let res = "nope".parse::<u32>();
    let down: Option<u32> = res.ok();
    println!("  Result -> Option  .ok()          -> {down:?}   (the reason is thrown away)");
    println!("      Going up you must SUPPLY a reason; going down you DISCARD one. That asymmetry is the point.");
}

// ─────────────────────────────────────────────────────────── Step 7
fn step7() {
    banner(7, "Two facts worth knowing early");

    let a: Option<Result<u32, ParseIntError>> = Some("12".parse());
    let b: Result<Option<u32>, ParseIntError> = a.transpose();
    println!("  .transpose() flips the nesting -> {b:?}");

    println!(
        "  size_of  i32={}  Option<i32>={}  Box<i32>={}  Option<Box<i32>>={}",
        std::mem::size_of::<i32>(),
        std::mem::size_of::<Option<i32>>(),
        std::mem::size_of::<Box<i32>>(),
        std::mem::size_of::<Option<Box<i32>>>(),
    );
    println!("      Option<Box<T>> is free: None reuses the null pointer. Safety with no runtime cost.");
}

// ─────────────────────────────────────────────────────────── Step 8
// A real error type. Three variants, because a caller might act differently on each.

#[derive(Debug)]
enum BallotError {
    Empty,
    BadScore(ParseIntError),
    OutOfRange { got: u32, max: u32 },
}

impl fmt::Display for BallotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BallotError::Empty => write!(f, "the ballot line was empty"),
            BallotError::BadScore(e) => write!(f, "not a number: {e}"),
            BallotError::OutOfRange { got, max } => {
                write!(f, "score {got} is above the {max} cap")
            }
        }
    }
}

impl Error for BallotError {}

// THIS is what lets `?` turn a ParseIntError into a BallotError automatically.
impl From<ParseIntError> for BallotError {
    fn from(e: ParseIntError) -> Self {
        BallotError::BadScore(e)
    }
}

fn parse_ballot(line: &str) -> Result<Vec<u32>, BallotError> {
    if line.trim().is_empty() {
        return Err(BallotError::Empty);
    }
    let mut out = Vec::new();
    for tok in line.split(',') {
        let score: u32 = tok.trim().parse()?; // ParseIntError -> BallotError, via From
        if score > 5 {
            return Err(BallotError::OutOfRange { got: score, max: 5 });
        }
        out.push(score);
    }
    Ok(out)
}

fn step8() {
    banner(8, "Designing the E in Result<T, E>");
    for line in ["5,2,0", "", "5,x,0", "5,9,0"] {
        match parse_ballot(line) {
            Ok(v) => println!("  {line:?} -> ok {v:?}"),
            Err(e) => println!("  {line:?} -> error: {e}"),
        }
    }
    println!("      `?` did the ParseIntError -> BallotError conversion. That is `From`, not magic.");
}

// ─────────────────────────────────────────────────────────── Step 9
// When you don't want to enumerate every failure, erase the type.
fn load_and_total(line: &str) -> Result<u32, Box<dyn Error>> {
    let ballot = parse_ballot(line)?; // BallotError   -> Box<dyn Error>
    let weight: u32 = "3".parse()?; // ParseIntError -> Box<dyn Error>
    Ok(ballot.iter().sum::<u32>() * weight)
}

fn step9() {
    banner(9, "Box<dyn Error> — two unrelated error types in one function");
    println!("  load_and_total(\"5,2,0\") -> {:?}", load_and_total("5,2,0"));
    match load_and_total("5,9,0") {
        Ok(v) => println!("  -> {v}"),
        Err(e) => println!("  load_and_total(\"5,9,0\") -> Err: {e}"),
    }
    println!("      Libraries name their errors (Step 8). Applications usually erase them (Step 9).");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    step7();
    step8();
    step9();
    println!();
}
