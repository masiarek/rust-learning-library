//! Two error types, one return type — and the two ways to make them one.
//!
//!   rustc --edition 2024 not_every_error_is_io_error.rs -o /tmp/ne && /tmp/ne

use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

/// Stands in for a file read, so the run stays deterministic.
fn read_sheet(name: &str) -> Result<String, io::Error> {
    match name {
        "ballots.txt" => Ok("Ada=5".to_string()),
        "torn.txt" => Ok("Ada=oops".to_string()),
        _ => Err(io::Error::new(io::ErrorKind::NotFound, "no such file or directory")),
    }
}

// ------------------------------------------ answer 1: erase it (Box<dyn Error>)

fn tally_boxed(name: &str) -> Result<u32, Box<dyn Error>> {
    let sheet = read_sheet(name)?; // io::Error   -> Box<dyn Error>
    let (_who, count) = sheet.split_once('=').ok_or("sheet has no '='")?;
    let n = count.parse::<u32>()?; // ParseIntError -> Box<dyn Error>
    Ok(n)
}

// ------------------------------------------------- answer 2: name it (an enum)

#[derive(Debug)]
enum TallyError {
    Unreadable(io::Error),
    Malformed(String),
    NotANumber(ParseIntError),
}

impl fmt::Display for TallyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TallyError::Unreadable(_) => write!(f, "could not read the tally sheet"),
            TallyError::Malformed(row) => write!(f, "row has no '=': {row:?}"),
            TallyError::NotANumber(_) => write!(f, "the count is not a number"),
        }
    }
}

impl Error for TallyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TallyError::Unreadable(e) => Some(e),
            TallyError::NotANumber(e) => Some(e),
            TallyError::Malformed(_) => None, // nothing underneath: we made this one
        }
    }
}

// The boilerplate `thiserror`'s `#[from]` removes: one impl per source type.
impl From<io::Error> for TallyError {
    fn from(e: io::Error) -> Self {
        TallyError::Unreadable(e)
    }
}

impl From<ParseIntError> for TallyError {
    fn from(e: ParseIntError) -> Self {
        TallyError::NotANumber(e)
    }
}

fn tally(name: &str) -> Result<u32, TallyError> {
    let sheet = read_sheet(name)?; // From<io::Error>     fires here
    let (_who, count) = sheet
        .split_once('=')
        .ok_or_else(|| TallyError::Malformed(sheet.clone()))?;
    let n = count.parse::<u32>()?; // From<ParseIntError> fires here
    Ok(n)
}

/// What a caller can do once the failures have names: three different recoveries.
fn recover(name: &str) -> String {
    match tally(name) {
        Ok(n) => format!("counted {n}"),
        Err(TallyError::Unreadable(e)) if e.kind() == io::ErrorKind::NotFound => {
            "no sheet yet — starting a fresh tally at 0".to_string()
        }
        Err(TallyError::Unreadable(_)) => "sheet unreadable — retrying later".to_string(),
        Err(TallyError::NotANumber(_)) => "one bad count — skipping the row".to_string(),
        Err(TallyError::Malformed(_)) => "sheet is not a tally sheet — aborting".to_string(),
    }
}

fn chain(e: &dyn Error) -> String {
    let mut out = e.to_string();
    let mut cursor = e.source();
    while let Some(inner) = cursor {
        out.push_str(&format!(" <- {inner}"));
        cursor = inner.source();
    }
    out
}

fn main() {
    let sheets = ["ballots.txt", "torn.txt", "gone.txt"];

    println!("1. One function, two error types");
    println!("   read_sheet -> io::Error       parse -> ParseIntError");
    println!("   Declaring the function `-> Result<u32, ParseIntError>` and using `?`");
    println!("   on the read is E0277: \"`?` couldn't convert the error\", because the");
    println!("   conversion `?` performs is `From`, and nobody wrote that impl.");

    println!("\n2. Answer 1 — erase it: Box<dyn Error>");
    for s in sheets {
        println!("   {s:<12} {:?}", tally_boxed(s).map_err(|e| e.to_string()));
    }
    println!("   Compiles the moment you write it, and every error converts. What");
    println!("   the caller can now do is print. That is the whole list.");

    println!("\n3. ...and the escape hatch that proves the cost");
    let boxed = tally_boxed("gone.txt").unwrap_err();
    let as_io = boxed.downcast_ref::<io::Error>().map(|e| e.kind());
    let as_parse = boxed.downcast_ref::<ParseIntError>().map(|e| e.to_string());
    println!("   downcast_ref::<io::Error>()     = {as_io:?}");
    println!("   downcast_ref::<ParseIntError>() = {as_parse:?}");
    println!("   It works, and notice what it is: a run-time type test, one per");
    println!("   candidate, that the compiler cannot check you finished. Add a new");
    println!("   failure upstream and every caller silently falls through.");

    println!("\n4. Answer 2 — name it: an enum with one variant per decision");
    for s in sheets {
        match tally(s) {
            Ok(n) => println!("   {s:<12} Ok({n})"),
            Err(e) => println!("   {s:<12} {}", chain(&e)),
        }
    }
    println!("   Same three inputs. Now the failures have names, and each one still");
    println!("   carries the error it came from, so the chain survives.");

    println!("\n5. Which is the point: the caller can DECIDE");
    for s in sheets {
        println!("   {s:<12} {}", recover(s));
    }
    println!("   Four arms, four different recoveries — and the match is exhaustive,");
    println!("   so adding a variant makes the compiler visit every caller.");

    println!("\n6. What it cost to write");
    println!("   the enum                  5 lines   you write this either way");
    println!("   impl Display              9 lines   \\");
    println!("   impl Error (source)       9 lines    | derived from attributes");
    println!("   impl From<io::Error>      5 lines    | by `thiserror`");
    println!("   impl From<ParseIntError>  5 lines   /");
    println!("   -------------------------------");
    println!("   33 lines, 28 of them mechanical. That is what the crate removes —");
    println!("   and it does not change the model above by one byte.");

    println!("\n7. So which one");
    println!("   A library: name them. Its callers have decisions to make, and the");
    println!("   enum IS the API — mark it #[non_exhaustive] or adding a variant is");
    println!("   a breaking change.");
    println!("   A binary: erase them. The caller is a person, the decision is what");
    println!("   to print, and Box<dyn Error> (or anyhow) says exactly that.");
    println!("   The common shape is both: a library half that names, a main that");
    println!("   erases. Picking by habit is how a library ends up unable to tell");
    println!("   its callers 'file missing' from 'file malformed'.");
}
