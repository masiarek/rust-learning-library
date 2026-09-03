//! One character that replaces a `match`, a `return`, and a type conversion.
//!
//!   rustc --edition 2024 the_question_mark_operator.rs -o /tmp/qm && /tmp/qm

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// A row of a tally sheet, as it arrives: "Ada=5".
const SHEET: [&str; 4] = ["Ada=5", "Ben=2", "Cara=oops", "Dev"];

// ---------------------------------------------------------------- the long way

fn score_by_match(row: &str) -> Result<u32, ParseIntError> {
    let (_name, count) = match row.split_once('=') {
        Some(pair) => pair,
        None => return Ok(0), // no '=' at all: treat as a blank row
    };
    let n = match count.parse::<u32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(n)
}

// ----------------------------------------------------------------- with `?`

fn score(row: &str) -> Result<u32, ParseIntError> {
    let Some((_name, count)) = row.split_once('=') else {
        return Ok(0);
    };
    Ok(count.parse::<u32>()?)
}

// -------------------------------------------- `?` converting the error type

#[derive(Debug)]
enum RowError {
    Malformed(String),
    NotANumber(ParseIntError),
}

impl fmt::Display for RowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RowError::Malformed(row) => write!(f, "row has no '=': {row:?}"),
            RowError::NotANumber(e) => write!(f, "count is not a number: {e}"),
        }
    }
}

impl Error for RowError {}

/// This is the whole mechanism: `?` calls `From::from` on the way out, so the
/// `ParseIntError` becomes a `RowError` with nothing written at the call site.
impl From<ParseIntError> for RowError {
    fn from(e: ParseIntError) -> Self {
        RowError::NotANumber(e)
    }
}

fn strict_score(row: &str) -> Result<u32, RowError> {
    let (_name, count) = row
        .split_once('=')
        .ok_or_else(|| RowError::Malformed(row.to_string()))?;
    Ok(count.parse::<u32>()?) //  <-- ParseIntError in, RowError out
}

// ------------------------------------------------------------ `?` on `Option`

fn initial(row: &str) -> Option<char> {
    let (name, _) = row.split_once('=')?; // None here returns None from `initial`
    name.chars().next()
}

// ------------------------------------------------- the universal receiver

fn total(rows: &[&str]) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;
    for row in rows {
        sum += strict_score(row)?; // RowError -> Box<dyn Error>, also via From
    }
    Ok(sum)
}

fn main() {
    println!("1. The same function, written twice");
    for row in SHEET {
        println!(
            "   {row:<12} by match {:?}   with ? {:?}",
            score_by_match(row).map_err(|_| "err"),
            score(row).map_err(|_| "err")
        );
    }
    println!("   Identical answers. What `?` removed was nine lines of arms that");
    println!("   did nothing but hand the value on or hand the error back.");

    println!("\n2. What `?` expands to");
    println!("   count.parse::<u32>()?");
    println!("   ==  match count.parse::<u32>() {{");
    println!("           Ok(v)  => v,");
    println!("           Err(e) => return Err(From::from(e)),");
    println!("       }}");
    println!("   Three things, not one: unwrap the happy path, RETURN on the sad");
    println!("   one, and convert the error while leaving.");

    println!("\n3. The conversion is the half people miss");
    for row in SHEET {
        match strict_score(row) {
            Ok(n) => println!("   {row:<12} -> {n}"),
            Err(e) => println!("   {row:<12} -> {e}"),
        }
    }
    println!("   `strict_score` returns RowError, but its last `?` was applied to");
    println!("   a ParseIntError, and nobody wrote a conversion at that spot — the");
    println!("   `impl From<ParseIntError> for RowError` did it, silently.");

    println!("\n4. `?` works on Option too, and means something different");
    for row in SHEET {
        println!("   initial({row:?}) = {:?}", initial(row));
    }
    println!("   Here `?` returns None early. Same punctuation, same shape, but");
    println!("   there is no conversion step: `None` carries nothing to convert.");
    println!("   You cannot mix them in one function — `?` on an Option inside a");
    println!("   fn returning Result is E0277, not an automatic Ok/Err guess.");

    println!("\n5. Box<dyn Error>: one return type that accepts every `?`");
    println!("   total(all four rows)   = {:?}", total(&SHEET).map_err(|e| e.to_string()));
    println!("   total(the good two)    = {:?}", total(&SHEET[..2]).map_err(|e| e.to_string()));
    println!("   Every error type implementing Error converts into Box<dyn Error>,");
    println!("   so `?` accepts all of them. Convenient at the top of a program,");
    println!("   and lossy in a library: the caller gets a message, not a type.");

    println!("\n6. What `?` is not");
    println!("   It is not `unwrap`. It never panics; it returns.");
    println!("   It does not catch anything — a panic inside the call still unwinds.");
    println!("   And it is not free of design: every `?` you write is a decision that");
    println!("   THIS function does not handle that error, and the caller must.");
}
