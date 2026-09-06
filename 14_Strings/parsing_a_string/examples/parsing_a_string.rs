//! `.parse()` is a call to `FromStr`, so nothing in the expression says what to
//! build — and the `Result` is there because text is input, and input lies.
//!
//!   rustc --edition 2024 parsing_a_string.rs -o /tmp/pas && /tmp/pas

use std::fmt;
use std::num::IntErrorKind;
use std::str::FromStr;

/// A three-part version, parsed and printed by the same pair of traits.
#[derive(Debug, PartialEq)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let mut next = |field: &str| -> Result<u32, String> {
            let text = parts.next().ok_or_else(|| format!("missing {field}"))?;
            text.parse::<u32>()
                .map_err(|e| format!("{field}: {e} (got {text:?})"))
        };
        let version = Version {
            major: next("major")?,
            minor: next("minor")?,
            patch: next("patch")?,
        };
        match parts.next() {
            Some(extra) => Err(format!("trailing {extra:?}")),
            None => Ok(version),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The whole point of `kind()`: a failure you can branch on.
fn explain(text: &str) -> String {
    match text.parse::<i32>() {
        Ok(n) => format!("{n}"),
        Err(e) => match e.kind() {
            IntErrorKind::Empty => "nothing was typed".into(),
            IntErrorKind::InvalidDigit => "that is not a number".into(),
            IntErrorKind::PosOverflow => "too big for an i32".into(),
            IntErrorKind::NegOverflow => "too small for an i32".into(),
            // IntErrorKind is #[non_exhaustive], so this arm is required.
            _ => "unparseable".into(),
        },
    }
}

fn main() {
    println!("1. Three ways to say which type to build");
    let annotated: i32 = "42".parse().unwrap();
    let turbofished = "42".parse::<i32>().unwrap();
    let inferred: Result<i32, _> = "42".parse();
    println!("   annotation  let n: i32 = \"42\".parse()   -> {annotated}"); // 42
    println!("   turbofish   \"42\".parse::<i32>()         -> {turbofished}"); // 42
    println!("   inferred    from the Result's own type   -> {:?}", inferred.unwrap()); // 42
    println!("   Nothing in `.parse()` itself names a type, so one of these is");
    println!("   always required. Without one it is E0284, not a default of i32.");

    println!("\n2. parse takes the WHOLE string, and it is not a tokenizer");
    for text in ["42", "+42", "-42", "042", " 42 ", "42 ", "4_2", "1,000", "42.0", ""] {
        let verdict = match text.parse::<i32>() {
            Ok(n) => format!("Ok({n})"),
            Err(e) => format!("Err — {e}"),
        };
        println!("   {:>7}  {verdict}", format!("{text:?}"));
    }
    println!("   `4_2` is the surprise: the underscore is LITERAL syntax, read by");
    println!("   the compiler, and parse never sees a literal. Trim before parsing.");

    println!("\n3. Its digits are ASCII, not Unicode");
    let arabic_indic = "\u{661}\u{662}\u{663}"; // ١٢٣ — ARABIC-INDIC DIGIT ONE, TWO, THREE
    println!("   {arabic_indic:?}.chars().all(char::is_numeric) = {}", arabic_indic.chars().all(char::is_numeric)); // true
    println!("   {arabic_indic:?}.parse::<i32>()                = {:?}", arabic_indic.parse::<i32>());
    println!("   Every char says it is a digit and parse still refuses: it reads");
    println!("   b'0'..=b'9' and nothing else. Python's int() accepts these.");

    println!("\n4. For an integer, the failure is structured");
    for text in ["", "forty-two", "999999999999", "-999999999999"] {
        let e = text.parse::<i32>().unwrap_err();
        println!("   {:>16}  kind={:?}", format!("{text:?}"), e.kind());
    }
    println!("   ParseIntError::kind() — stable since 1.55 — is a value you can");
    println!("   match on, so \"try again\" and \"pick a bigger type\" are");
    println!("   distinguishable without reading an error message:");
    for text in ["7", "", "x", "999999999999"] {
        println!("      {:>16} -> {}", format!("{text:?}"), explain(text));
    }

    println!("\n5. For a float, it is not");
    let empty = "".parse::<f64>().unwrap_err();
    let word = "forty".parse::<f64>().unwrap_err();
    println!("   \"\"      -> {empty}");
    println!("   \"forty\" -> {word}");
    println!("   different errors?  {}", empty != word); // true
    println!("   ParseFloatError HAS a kind field, and it is pub(super) — there is");
    println!("   no accessor on stable, so you get Display and PartialEq and no");
    println!("   match. Comparing against a known error is the only way to ask.");

    println!("\n6. Floats forgive more than integers do");
    for text in ["3.14", "3.", ".5", "1e10", "inf", "NaN", "-0", "3,14", " 1.0"] {
        let verdict = match text.parse::<f64>() {
            Ok(n) => format!("Ok({n:?})"),
            Err(e) => format!("Err — {e}"),
        };
        println!("   {:>7}  {verdict}", format!("{text:?}"));
    }
    println!("   The decimal separator is a POINT, always. Rust reads no locale, so");
    println!("   a CSV exported anywhere that writes 3,14 must be fixed before this.");

    println!("\n7. Other targets, and another base");
    println!("   \"true\".parse::<bool>()  = {:?}", "true".parse::<bool>());
    println!("   \"True\".parse::<bool>()  = {:?}   <- exact spelling only", "True".parse::<bool>());
    println!("   \"a\".parse::<char>()     = {:?}", "a".parse::<char>());
    println!("   i64::from_str_radix(\"1a2b\", 16) = {:?}", i64::from_str_radix("1a2b", 16));
    println!("   from_str_radix is the way in for hex, binary and octal — `parse`");
    println!("   is base 10 and has no argument for anything else.");

    println!("\n8. FromStr for your own type, and Display as its mirror");
    for text in ["1.2.3", "1.2", "1.2.3.4", "1.x.3"] {
        match text.parse::<Version>() {
            Ok(v) => println!("   {:>9} -> {v}   (round trip: {})", format!("{text:?}"), v.to_string() == text),
            Err(e) => println!("   {:>9} -> {e}", format!("{text:?}")),
        }
    }
    println!("   Implement FromStr and `.parse::<Version>()` works; implement");
    println!("   Display and `.to_string()` works. One pair of traits, both");
    println!("   directions, and a round trip you can assert on.");
}
