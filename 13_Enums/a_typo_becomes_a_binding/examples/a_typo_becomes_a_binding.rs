//! One lowercase letter turns a match arm into a catch-all, silently.
//! Both functions below compile without a single warning. One of them is wrong.

#[derive(Debug, Clone, Copy)]
enum Suit { Heart, Diamond, Spade, Club, Joker }

/// Written when `Suit` had four variants, with `Spade` mistyped as `spade`.
/// Because of the glob import, `spade` is not a variant here — it is a fresh
/// binding, so it matches *anything*. `Joker`, added later, lands in it.
fn describe_glob(s: &Suit) -> String {
    use Suit::*;
    match s {
        Heart => String::from("the red one you draw"),
        Diamond => String::from("the other red one"),
        Club => String::from("the clover"),
        spade => {
            let _ = spade; // a real arm body ignores it; this only keeps the
            String::from("the pointy one") // unused-variable warning away.
        }
    }
}

/// The same arms, qualified. `Suit::spade` would not resolve, so the typo
/// could never have been written; and adding `Joker` forced an arm for it.
fn describe_qualified(s: &Suit) -> String {
    match s {
        Suit::Heart => String::from("the red one you draw"),
        Suit::Diamond => String::from("the other red one"),
        Suit::Club => String::from("the clover"),
        Suit::Spade => String::from("the pointy one"),
        Suit::Joker => String::from("the wild card"),
    }
}

fn main() {
    let all = [Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club, Suit::Joker];

    println!("{:<9} {:<24} {}", "suit", "glob-imported (typo)", "qualified");
    println!("{}", "-".repeat(62));
    for s in &all {
        let glob = describe_glob(s);
        let qual = describe_qualified(s);
        let flag = if glob == qual { "" } else { "  <- wrong" };
        let line = format!("{:<9} {:<24} {:<20}{}", format!("{s:?}"), glob, qual, flag);
        println!("{}", line.trim_end());
    }
}
