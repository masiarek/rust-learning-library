//! `Result` aliases: how to read an error type that has been compressed away.
//!
//! `Result<T, E>` has two parameters, but almost every signature you meet in the
//! wild shows one, or none. The missing parameter has not gone anywhere — it has
//! been pinned by a type alias. Expanding the alias is how you find out what can
//! actually go wrong, which is the only question the `E` slot exists to answer.
//!
//!   rustc --edition 2024 result_aliases.rs -o /tmp/ra && /tmp/ra

use std::convert::Infallible;
use std::fmt;
use std::io::Write;
use std::mem::size_of;
use std::str::FromStr;

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() {
    banner(1, "io::Result<T> looks wrong until you expand it");

    // Writing into a Vec<u8> goes through the same std::io::Write trait a file
    // does, so it hands back the same type — no filesystem required.
    let mut buf: Vec<u8> = Vec::new();

    let short: std::io::Result<()> = writeln!(buf, "Ada,Ben,Cara");
    let long: Result<(), std::io::Error> = writeln!(buf, "5,2,0");

    println!("  io::Result<()>              -> {short:?}");
    println!("  Result<(), std::io::Error>  -> {long:?}");

    // The proof that they are one type: assign one to the other's annotation.
    let same: std::io::Result<()> = long;
    println!("  the two annotations are interchangeable -> {same:?}");
    println!("  buffer now holds: {:?}", String::from_utf8(buf).unwrap());
    println!("      type Result<T> = Result<T, Error>;   <- one line in std::io");
}

// ─────────────────────────────────────────────────────────── Step 2
struct Row(Vec<u8>);

impl fmt::Display for Row {
    // Not `Result<(), fmt::Error>` spelled out — everyone writes the alias.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

fn step2() {
    banner(2, "The aliases you will actually meet");

    println!("  std::io::Result<T>      = Result<T, std::io::Error>");
    println!("  std::fmt::Result        = Result<(), std::fmt::Error>   <- NO parameters at all");
    println!("  std::thread::Result<T>  = Result<T, Box<dyn Any + Send>>");
    println!("  a Display impl returning fmt::Result -> {}", Row(vec![5, 2, 0]));
    println!("      fmt::Result pins BOTH slots, which is why `Ok(())` ends every fmt impl.");
}

// ─────────────────────────────────────────────────────────── Step 3
mod row {
    use std::fmt;

    #[derive(Debug)]
    pub enum Error {
        Empty,
        NotANumber(String),
        OutOfRange { got: u32, max: u32 },
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::Empty => write!(f, "the input line was empty"),
                Error::NotANumber(tok) => write!(f, "{tok:?} is not a score"),
                Error::OutOfRange { got, max } => write!(f, "score {got} is above the {max} cap"),
            }
        }
    }

    impl std::error::Error for Error {}

    /// The crate-wide alias. Declared once, next to the error it pins.
    pub type Result<T> = std::result::Result<T, Error>;

    /// Reads as "gives back scores, or the crate's error".
    pub fn parse(line: &str) -> Result<Vec<u32>> {
        if line.trim().is_empty() {
            return Err(Error::Empty);
        }
        let mut out = Vec::new();
        for tok in line.split(',') {
            let n: u32 = tok
                .trim()
                .parse()
                .map_err(|_| Error::NotANumber(tok.trim().to_string()))?;
            if n > 5 {
                return Err(Error::OutOfRange { got: n, max: 5 });
            }
            out.push(n);
        }
        Ok(out)
    }
}

fn step3() {
    banner(3, "Writing your own alias");

    for line in ["5,2,0", "", "5,x,0", "5,9,0"] {
        match row::parse(line) {
            Ok(v) => println!("  parse({line:?}) -> Ok({v:?})"),
            Err(e) => println!("  parse({line:?}) -> Err: {e}"),
        }
    }
    println!("      pub type Result<T> = std::result::Result<T, Error>;");
    println!("      Inside the module `Result` now means YOUR Result. Outside it is");
    println!("      `row::Result<T>`, and the prelude's two-parameter one is untouched.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "Ok(()) — the other slot emptied out");

    let done: row::Result<()> = Ok(());
    println!("  a function that only succeeds or fails -> {done:?}");

    let e = row::Error::OutOfRange { got: 9, max: 5 };
    println!("  Display of the error  -> {e}");
    println!("  Debug of the error    -> {e:?}");
    println!("      A failing `fn main() -> Result<(), E>` prints the DEBUG form after");
    println!("      \"Error: \" and exits 1. Your careful Display impl is not used.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "Infallible — the error type that has no values");

    let s: Result<String, Infallible> = String::from_str("Ada");
    let n: Result<u64, Infallible> = u64::try_from(3u32);
    println!("  String::from_str(\"Ada\")  -> {s:?}    (FromStr::Err  = Infallible)");
    println!("  u64::try_from(3u32)      -> {n:?}        (TryFrom::Error = Infallible)");

    println!("  size_of  Infallible={}  ()={}", size_of::<Infallible>(), size_of::<()>());
    println!(
        "  size_of  u64={}  Result<u64, Infallible>={}  Result<u64, io::Error>={}",
        size_of::<u64>(),
        size_of::<Result<u64, Infallible>>(),
        size_of::<Result<u64, std::io::Error>>(),
    );
    println!("      Infallible is an enum with NO variants, so Err(_) cannot be built.");
    println!("      The compiler knows it, drops the tag, and Result<u64, Infallible>");
    println!("      costs exactly what a u64 costs. A promise with no runtime price.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn step6() {
    banner(6, "Reading the E slot back");

    let rows = [
        ("Infallible", "cannot fail; the Result is there for a trait's sake"),
        ("io::Error", "one concrete failure, ask it for .kind()"),
        ("RowError (your enum)", "a fixed menu; the caller can match on it"),
        ("Box<dyn Error>", "anything at all; nobody downstream will match"),
    ];
    for (e, meaning) in rows {
        println!("  Result<T, {e:<24}> {meaning}");
    }
    println!("      This is the only question the second parameter answers, and an");
    println!("      alias does not remove the answer — it just moves it one hop away.");
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
