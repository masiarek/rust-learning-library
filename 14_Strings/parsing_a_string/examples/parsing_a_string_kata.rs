//! Kata solution: one config line handled four ways, the whitespace that breaks
//! three of them, and a FromStr whose error you can match on instead of read.
//!
//!   rustc --edition 2024 parsing_a_string_kata.rs -o /tmp/pask && /tmp/pask

use std::fmt;
use std::num::IntErrorKind;
use std::str::FromStr;

/// What went wrong, as a value — the shape `ParseIntError::kind()` has, and the
/// reason to write an enum here rather than a `String` you can only print.
#[derive(Debug, PartialEq)]
enum PortError {
    Missing,
    NotANumber(String),
    OutOfRange(String),
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortError::Missing => write!(f, "no port given"),
            PortError::NotANumber(text) => write!(f, "{text:?} is not a number"),
            PortError::OutOfRange(text) => write!(f, "{text} is not a usable port"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Port(u16);

impl FromStr for Port {
    type Err = PortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(PortError::Missing);
        }
        match s.trim().parse::<u32>() {
            Ok(n) if (1..=65535).contains(&n) => Ok(Port(n as u16)),
            Ok(n) => Err(PortError::OutOfRange(n.to_string())),
            // Too big for a u32, so there is no number to carry back — only the
            // text that was typed. That is why the variant holds a String.
            Err(e) if *e.kind() == IntErrorKind::PosOverflow => {
                Err(PortError::OutOfRange(s.trim().to_string()))
            }
            Err(_) => Err(PortError::NotANumber(s.trim().to_string())),
        }
    }
}

/// The `?` handling: one function, every failure propagated to the caller.
fn read_port(line: &str) -> Result<Port, PortError> {
    let value = line.split_once('=').map(|(_, v)| v).unwrap_or("");
    let port: Port = value.parse()?;
    Ok(port)
}

fn main() {
    println!("Round 1 -- one bad value, four handlings");
    let value = "80x";

    // expect: the message the user actually sees. Caught so the run can finish.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| value.parse::<u16>().expect("port must be a number"));
    std::panic::set_hook(hook);
    let panic_message = caught
        .unwrap_err()
        .downcast_ref::<String>()
        .map(|s| s.lines().next().unwrap_or("").to_string())
        .unwrap_or_default();
    println!("   expect     panics: {panic_message}");
    println!("              -- honest in a scratch program; in a tool someone else");
    println!("                 runs it is a stack trace where an error belonged.");

    let fallback = value.parse::<u16>().unwrap_or(8080);
    println!("   unwrap_or  {fallback}   <- no complaint, and no way to know it fired");

    let described = match value.parse::<u16>() {
        Ok(n) => format!("{n}"),
        Err(e) => format!("kept the default, because {e}"),
    };
    println!("   match      {described}");

    println!("   ?          {:?}", read_port("port=80x"));
    println!("              -- the only one that hands the caller the decision.");

    println!("\nRound 2 -- the whitespace that breaks a hand-rolled reader");
    for line in ["port=8080", "port= 8080", "port =8080 ", "port=", "port=99999", "port=0"] {
        let raw = line.split_once('=').map(|(_, v)| v).unwrap_or("");
        let naive = raw.parse::<u16>();
        let ours = read_port(line);
        println!(
            "   {:>14}  raw={:<8} -> {:<44} ours -> {}",
            format!("{line:?}"),
            format!("{raw:?}"),
            match &naive {
                Ok(n) => format!("Ok({n})"),
                Err(e) => format!("Err({e})"),
            },
            match &ours {
                Ok(p) => format!("Ok({})", p.0),
                Err(e) => format!("Err({e})"),
            }
        );
    }
    println!("   `parse` never trims, so the space a human left after `=` is a");
    println!("   parse error in every reader that forgets `.trim()`. Ours trims");
    println!("   once, in `from_str`, so no caller has to remember.");

    println!("\nRound 3 -- an error you can match on, not just print");
    for line in ["port=443", "port=", "port=http", "port=99999", "port=4294967296"] {
        let advice = match read_port(line) {
            Ok(p) => format!("listening on {}", p.0),
            Err(PortError::Missing) => "fill in the port".into(),
            Err(PortError::NotANumber(t)) => format!("{t:?} looks like a name, not a port"),
            Err(PortError::OutOfRange(text)) => format!("{text} is above 65535 — pick another"),
        };
        println!("   {:>20}  {advice}", format!("{line:?}"));
    }
    println!("   Same shape as IntErrorKind, for the same reason: the caller wants");
    println!("   to BRANCH on the failure, and a String can only be shown to");
    println!("   someone. Note the last row -- 4294967296 overflows u32 before any");
    println!("   range check could run -- there is no u32 left to report, so the");
    println!("   variant carries the TEXT and says what was actually typed.");
}
