//! The braces take a name, not an expression.
//!
//! Since Rust 1.58 you can write `println!("n is {n}")` and the macro will find
//! the variable called `n`. That looks like a Python f-string and it is not one:
//! what goes between the braces is an IDENTIFIER, resolved at compile time by
//! the macro, and nothing else. `{n + 1}`, `{v.len()}` and `{self.voter}` are
//! each a compile error with its own message. The escape hatch is a named
//! argument — or a `let` on the line above, which usually reads better anyway.
//!
//!   rustc --edition 2024 braces_take_a_name.rs -o /tmp/btan && /tmp/btan

fn banner(title: &str) {
    println!("\n──── {title}");
}

struct Ballot {
    voter: &'static str,
    scores: [u8; 3],
}

impl Ballot {
    /// `{self.voter}` is refused, so bind the field first and capture the name.
    fn show(&self) {
        let voter = self.voter;
        let scores = self.scores;
        println!("  {voter} scored {scores:?}");
    }
}

fn main() {
    banner("A name in the braces is looked up like any other name");

    let n = 5;
    let voters = 9;
    println!("  n is {n}, voters is {voters}");
    println!("  the old way still works: n is {}, voters is {}", n, voters);

    banner("But ONLY a name. These three are compile errors:");

    println!("  println!(\"{{n + 1}}\");");
    println!("      error: invalid format string: expected `}}`, found `+`");
    println!("  println!(\"{{scores.len()}}\");");
    println!("      error: invalid format string: expected `}}`, found `.`");
    println!("  println!(\"{{scores[0]}}\");");
    println!("      error: invalid format string: expected `}}`, found `[`");
    println!("  Not one of them is a type error. The macro cannot even finish");
    println!("  reading the string — it wanted a name and found punctuation.");

    banner("Field access gets a message of its very own");

    println!("  println!(\"ballot from {{self.voter}}\");");
    println!("      error: invalid format string: field access isn't supported");
    println!("      help: consider using a positional formatting argument instead");
    println!("  rustc wrote that error for this exact mistake, which tells you");
    println!("  how often people make it. The fix it suggests:");
    let ballot = Ballot { voter: "Ada", scores: [5, 3, 0] };
    ballot.show();

    banner("The two escape hatches");

    let scores = [5, 3, 0];
    println!("  {}", scores.len()); //          positional: the argument list
    println!("  {count}", count = scores.len()); // a named argument
    let count = scores.len(); //                or a `let`, and then a capture
    println!("  {count}");
    println!("  All three print the same 3. The third names the value, which is");
    println!("  the only one of them a reader can still follow at ten lines.");

    banner("Width and precision take a name too — with a trailing $");

    let name = "Ada";
    let width = 8;
    let ratio = 1.0_f64 / 3.0;
    let prec = 2;
    println!("  |{name:>width$}|   <- right-aligned in `width` columns");
    println!("  {ratio:.prec$}        <- `prec` decimal places");

    banner("It reads whatever the name means AT THAT POINT");

    let n = 5;
    println!("  {n}");
    let n = "five";
    println!("  {n}   <- same four characters in the string, different variable");
    println!("  The capture is a name lookup, so a shadow changes what it finds.");

    banner("The format string itself must be a literal");

    println!("  let s = \"hello {{n}}\";");
    println!("  println!(s);");
    println!("      error: format argument must be a string literal");
    println!("  The braces are read at COMPILE time, so a string assembled at");
    println!("  run time has nothing left to read them.");

    banner("And a real brace is doubled");

    println!("  To print {{}} you write {{{{}}}} — and {name} still captures alongside it.");
    let json = format!("{{\"voter\": \"{name}\"}}");
    println!("  {json}");
}
