//! Write `From`, get `Into` free — and `?` starts converting your errors.
//!
//!   rustc --edition 2024 from_and_into.rs -o /tmp/fi && /tmp/fi

use std::fmt;

/// A score on the 0-5 ballot.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Score(u8);

/// One direction, written once.
impl From<u8> for Score {
    fn from(n: u8) -> Self {
        Score(n.min(5))
    }
}

/// And the other, so a Score can go back to a number.
impl From<Score> for u32 {
    fn from(s: Score) -> Self {
        u32::from(s.0)
    }
}

/// Two ways a tally can fail, each with its own type.
#[derive(Debug)]
struct BadNumber(String);

#[derive(Debug)]
struct OutOfRange(u8);

/// The one error the caller sees.
#[derive(Debug)]
enum TallyError {
    NotANumber(String),
    TooLarge(u8),
}

impl From<BadNumber> for TallyError {
    fn from(e: BadNumber) -> Self {
        TallyError::NotANumber(e.0)
    }
}

impl From<OutOfRange> for TallyError {
    fn from(e: OutOfRange) -> Self {
        TallyError::TooLarge(e.0)
    }
}

impl fmt::Display for TallyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TallyError::NotANumber(s) => write!(f, "not a number: {s:?}"),
            TallyError::TooLarge(n) => write!(f, "{n} is above the 0-5 range"),
        }
    }
}

fn digits(cell: &str) -> Result<u8, BadNumber> {
    cell.parse::<u8>().map_err(|_| BadNumber(cell.to_string()))
}

fn in_range(n: u8) -> Result<u8, OutOfRange> {
    if n <= 5 { Ok(n) } else { Err(OutOfRange(n)) }
}

/// Two different error types, one return type, and no map_err in sight.
fn read_cell(cell: &str) -> Result<Score, TallyError> {
    let n = digits(cell)?;      // BadNumber  -> TallyError, via From
    let n = in_range(n)?;       // OutOfRange -> TallyError, via From
    Ok(Score::from(n))
}

fn main() {
    println!("1. One impl, two call syntaxes");
    let a = Score::from(4u8);
    let b: Score = 4u8.into();
    println!("   Score::from(4u8) = {a:?}");
    println!("   let b: Score = 4u8.into() = {b:?}   same value: {}", a == b);
    println!("   `Into` is never implemented by hand. std has a blanket impl —");
    println!("   `impl<T, U: From<T>> Into<U> for T` — so writing From gives you");
    println!("   both, and writing Into gives you one.");

    println!();
    println!("2. Which one to write in a signature");
    fn takes_score(s: Score) -> u32 {
        u32::from(s)
    }
    fn takes_anything<S: Into<Score>>(s: S) -> u32 {
        u32::from(s.into())
    }
    println!("   takes_score(Score::from(3)) = {}", takes_score(Score::from(3u8)));
    println!("   takes_anything(3u8)         = {}", takes_anything(3u8));
    println!("   takes_anything(Score(3))    = {}", takes_anything(Score(3)));
    println!("   `impl Into<T>` in an argument accepts the converted type too,");
    println!("   because From<T> for T is implemented for every T. That is the");
    println!("   reflexive impl, and it is why `.into()` sometimes converts nothing.");

    println!();
    println!("3. The conversion clamps, which is a decision this impl made");
    println!("   Score::from(9u8) = {:?}   <- 9 became 5, silently", Score::from(9u8));
    println!("   `From` must not fail: the trait has no error type, so any input");
    println!("   the conversion cannot honour has to be forced into range or");
    println!("   panicked on. When neither is acceptable, the trait you want is");
    println!("   TryFrom.");

    println!();
    println!("4. Where From earns its place: the `?` operator");
    for cell in ["4", "x", "9"] {
        match read_cell(cell) {
            Ok(s) => println!("   read_cell({cell:?}) = Ok({s:?})"),
            Err(e) => println!("   read_cell({cell:?}) = Err({e})"),
        }
    }
    println!("   Two helper functions, two unrelated error types, and `?` converted");
    println!("   each one on the way out — because `?` calls From::from on the error");
    println!("   whenever the types differ. Delete the two impls and both `?` lines");
    println!("   stop compiling with E0277, whose first line is exactly the sentence");
    println!("   you want: \"`?` couldn't convert the error to `TallyError`\", followed");
    println!("   by \"the trait `From<BadNumber>` is not implemented for `TallyError`\".");

    println!();
    println!("5. The conversions you already use");
    let owned: String = String::from("Ada");
    let also: String = "Ada".into();
    let n: u64 = u64::from(42u32);
    println!("   String::from(\"Ada\") = {owned:?}, \"Ada\".into() = {also:?}");
    println!("   u64::from(42u32) = {n} — every widening integer conversion is a From");
    println!("   impl, which is why `as` is never needed for one that cannot lose data.");
}
