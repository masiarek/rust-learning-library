//! Kata solution: the SOURCE decides the spelling, not taste.
//!
//!   rustc --edition 2024 choosing_a_spelling_kata.rs -o /tmp/cask && /tmp/cask

use std::fmt;

#[derive(Debug)]
struct Score(u8);

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} star{}", self.0, if self.0 == 1 { "" } else { "s" })
    }
}

// The one place `.into()` is plainly the right call: the signature names the
// destination, so the caller does not have to. Both a &str and a String go in.
fn cast(voter: impl Into<String>, score: Score) -> String {
    format!("{} scored {}", voter.into(), score)
}

fn main() {
    println!("1. Source is a &str -> ownership is the only thing changing");
    let borrowed: &str = "Ada";
    let owned = borrowed.to_owned();
    let built = String::from(borrowed);
    println!("   to_owned()     {owned:?}");
    println!("   String::from() {built:?}   <- same From impl, constructor-shaped");

    println!();
    println!("2. Source is not a string -> to_owned() is not a candidate");
    let n = 42;
    let flag = true;
    let ch = 'x';
    let score = Score(4);
    println!("   42.to_string()      {:?}", n.to_string());
    println!("   true.to_string()    {:?}", flag.to_string());
    println!("   'x'.to_string()     {:?}", ch.to_string());
    println!("   Score(4).to_string() {:?}  <- free, because Score implements Display", score.to_string());
    println!("   42.to_owned() compiles and gives back an i32. Not text, never was.");

    println!();
    println!("3. Source is already a String -> none of them");
    let have = String::from("Ben");
    let view: &str = &have;                 // free
    println!("   &have          {view:?}   <- a view costs nothing");
    println!("   have.to_string() would allocate a SECOND buffer for the same bytes.");
    println!("   If you want a copy, write .clone() so the reader can see you meant it.");

    println!();
    println!("4. Destination fixed by a signature -> .into()");
    println!("   cast(\"Ada\", Score(5))                {:?}", cast("Ada", Score(5)));
    println!("   cast(String::from(\"Ben\"), Score(1))  {:?}", cast(String::from("Ben"), Score(1)));
    // Without a destination, `.into()` has nothing to aim at:
    //     let x = "Ada".into();
    //     error[E0282]: type annotations needed
    println!("   Bare `let x = \"Ada\".into();` is E0282 — nothing names the target.");

    println!();
    println!("5. Several pieces -> format!");
    let a = "Ada";
    let b = "Ben";
    println!("   format!(\"{{a}} vs {{b}}\")  {:?}", format!("{a} vs {b}"));
    println!("   format!(\"{{a}}\") alone is clippy's useless_format: nothing to format.");
}
