//! "Return None on error" — the job on std's list to be most suspicious of.
//!
//! It is legitimate when a failure has exactly ONE cause. It is a downgrade when
//! you are throwing away a Result that already told you which cause it was.
//!
//!   rustc --edition 2024 none_on_error.rs -o /tmp/noe && /tmp/noe

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn parse_integer(input: &str) -> Option<i32> {
    match input.parse::<i32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn step1() {
    banner(1, "The function, and the one-liner it already is");

    for input in ["42", "abc"] {
        match parse_integer(input) {
            Some(value) => println!("  parse_integer({input:?}) -> Parsed value: {value}"),
            None => println!("  parse_integer({input:?}) -> Failed to parse integer"),
        }
    }

    // The whole match IS `.ok()`. Same behaviour, no hand-written arms.
    println!("  \"42\".parse::<i32>().ok()  -> {:?}", "42".parse::<i32>().ok());
    println!("  \"abc\".parse::<i32>().ok() -> {:?}", "abc".parse::<i32>().ok());
    println!(
        "  agree on every input?      {}",
        ["42", "abc", "", "7"]
            .iter()
            .all(|s| parse_integer(s) == s.parse::<i32>().ok())
    );
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "What the downgrade threw away");

    let inputs = ["abc", "", "99999999999999", "-99999999999999", "4.5"];
    println!("  {:<18} {:<46} {}", "input", "as Result", "as Option");
    for input in inputs {
        let as_result = input.parse::<i32>();
        let as_option = as_result.clone().ok();
        // Format to a String first: a Debug impl for &str writes straight to the
        // output and ignores the width, so `{input:>16?}` would not pad at all.
        println!(
            "  {:<18} {:<46} {}",
            format!("{input:?}"),
            format!("{as_result:?}"),
            format!("{as_option:?}")
        );
    }
    println!("      FIVE inputs, FOUR distinct causes — InvalidDigit, Empty, PosOverflow,");
    println!("      NegOverflow — and all of them arrive as the same None.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn message_from_result(input: &str) -> String {
    match input.parse::<i32>() {
        Ok(v) => format!("ok: {v}"),
        Err(e) => format!("that didn't work — {e}"),
    }
}

fn message_from_option(input: &str) -> String {
    match input.parse::<i32>().ok() {
        Some(v) => format!("ok: {v}"),
        None => "Failed to parse integer".to_string(),
    }
}

fn step3() {
    banner(3, "The cost lands on whoever reads the message");

    for input in ["99999999999999", ""] {
        println!("  input {input:?}");
        println!("    kept Result  -> {}", message_from_result(input));
        println!("    downgraded   -> {}", message_from_option(input));
    }
    println!("      A user who typed too many digits is told the same thing as one who");
    println!("      typed nothing. The information existed; the return type discarded it.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "When None-on-error is exactly right");

    println!("  '7'.to_digit(10)   -> {:?}", '7'.to_digit(10));
    println!("  'x'.to_digit(10)   -> {:?}", 'x'.to_digit(10));
    println!("  'f'.to_digit(16)   -> {:?}", 'f'.to_digit(16));

    let roster = [("ada", 1), ("ben", 2)];
    let find = |name: &str| roster.iter().find(|(n, _)| *n == name).map(|(_, id)| *id);
    println!("  find(\"ada\")        -> {:?}", find("ada"));
    println!("  find(\"zoe\")        -> {:?}", find("zoe"));
    println!("      One cause each: the char is not a digit in that radix; the name is not");
    println!("      on the roster. There is no second reason for a caller to distinguish.");
}

// ─────────────────────────────────────────────────────────── Step 5
#[derive(Debug)]
enum FieldError {
    Missing,
    NotANumber(std::num::ParseIntError),
}

fn read_field(raw: Option<&str>) -> Result<i32, FieldError> {
    let text = raw.ok_or(FieldError::Missing)?; // Option -> Result: supply the reason
    text.parse::<i32>().map_err(FieldError::NotANumber)
}

fn step5() {
    banner(5, "Going the other way when you need to");

    for raw in [Some("42"), None, Some("abc")] {
        let shown = match read_field(raw) {
            Ok(v) => format!("Ok({v})"),
            Err(FieldError::Missing) => "Err: the field was not supplied".to_string(),
            // The variant carries the original error, so we can still say WHICH
            // way the text was malformed — the whole point of not downgrading.
            Err(FieldError::NotANumber(e)) => format!("Err: not a number — {e}"),
        };
        println!("  read_field({raw:?}) -> {shown}");
    }
    println!("      ok_or supplies the reason Option could not carry. And note the default:");
    println!("      return the Result. A caller who does not care can always call .ok();");
    println!("      a caller who does care cannot invent what you already threw away.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    println!();
}
