//! `impl` blocks: where the functions went, and the three kinds of `self`.
//!
//!   rustc --edition 2024 impl_blocks.rs -o /tmp/ib && /tmp/ib

// ---------------------------------------------------------------------------
// The data. Nothing else is allowed in here — no `fn`, not one.
// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
struct Ballot {
    voter: String,
    scores: Vec<u8>,
}

// ---------------------------------------------------------------------------
// An INHERENT impl: functions that belong to Ballot and to nothing else.
// ---------------------------------------------------------------------------
impl Ballot {
    // No `self` => an ASSOCIATED FUNCTION. There is no instance to call it on,
    // so you name the type: Ballot::new(..). This is what "standalone" means.
    fn new(voter: &str) -> Self {
        // `Self` (capital) is the TYPE — here it is another way to write Ballot.
        Self { voter: voter.to_string(), scores: Vec::new() }
    }

    // `&self` => borrow it. Read-only, and the caller keeps the ballot.
    fn total(&self) -> u32 {
        self.scores.iter().map(|&s| s as u32).sum()
    }

    // `&mut self` => borrow it exclusively. May change it; caller still keeps it.
    fn add(&mut self, s: u8) {
        self.scores.push(s);
    }

    // `self` => CONSUME it. The ballot is moved in and the caller loses it.
    // Right when the operation logically ends the value's life.
    fn into_receipt(self) -> String {
        format!("{} cast {} scores", self.voter, self.scores.len())
    }
}

// A type may have MANY impl blocks. Nothing is nested, nothing is reopened —
// they simply add. Handy for grouping, and required once generics get involved.
impl Ballot {
    fn is_blank(&self) -> bool {
        self.scores.is_empty()
    }
}

// ---------------------------------------------------------------------------
// `impl` is NOT struct-only. Enums take methods exactly the same way.
// (`Option`'s hundred methods are just an `impl<T> Option<T>` in std.)
// ---------------------------------------------------------------------------
#[derive(Debug)]
enum Verdict {
    Elected(String),
    Tied(u8),
    NoContest,
}

impl Verdict {
    fn headline(&self) -> String {
        match self {
            Verdict::Elected(who) => format!("{who} wins"),
            Verdict::Tied(n) => format!("{n}-way tie"),
            Verdict::NoContest => "no contest".to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// A TRAIT impl: the same syntax plus `for`. The difference is who decides the
// signature — a trait impl fills in someone else's shape.
// ---------------------------------------------------------------------------
trait Summary {
    fn one_line(&self) -> String;

    // A DEFAULT method: implementors get this free unless they override it.
    fn shout(&self) -> String {
        self.one_line().to_uppercase()
    }
}

impl Summary for Ballot {
    fn one_line(&self) -> String {
        format!("{} scored {} candidates, total {}", self.voter, self.scores.len(), self.total())
    }
}

impl Summary for Verdict {
    fn one_line(&self) -> String {
        self.headline()
    }
    fn shout(&self) -> String {
        format!("*** {} ***", self.headline()) // overriding the default
    }
}

fn main() {
    println!("1. Associated function vs method — the only difference is `self`");
    let mut b = Ballot::new("Ada"); //        no self  -> name the TYPE
    b.add(5); //                              &mut self -> name the VALUE
    b.add(2);
    b.add(0);
    println!("   Ballot::new(\"Ada\")  associated function, no instance existed yet");
    println!("   b.add(5)            method, called on the value");
    println!("   b.total() = {}", b.total());

    println!("\n2. `b.total()` is sugar. This is the same call, spelled out:");
    println!("   Ballot::total(&b) = {}   equal: {}", Ballot::total(&b), Ballot::total(&b) == b.total());
    println!("   The dot inserts the `&` for you. That is the whole trick.");

    println!("\n3. The three kinds of self, and what each costs the caller");
    println!("   &self      total()        -> {}   caller keeps it", b.total());
    println!("   &mut self  add(4)         -> caller keeps it, changed");
    let mut c = b.clone();
    c.add(4);
    println!("              {:?}", c.scores);
    println!("   self       into_receipt() -> caller LOSES it");
    println!("              {}", c.into_receipt());
    println!("              `c` cannot be used again: E0382, borrow of moved value");

    println!("\n4. Several impl blocks are fine — they add up");
    println!("   b.is_blank() = {}  (from the second impl Ballot block)", b.is_blank());

    println!("\n5. `impl` is not struct-only — enums take methods identically");
    for v in [Verdict::Elected("Ada".into()), Verdict::Tied(3), Verdict::NoContest] {
        println!("   {:<24} -> {}", format!("{v:?}"), v.headline());
    }

    println!("\n6. Inherent impl vs trait impl");
    println!("   inherent: you choose the signature");
    println!("     b.total()      -> {}", b.total());
    println!("   trait:    the trait chose it, so many types can answer");
    println!("     b.one_line()   -> {}", b.one_line());
    println!("     Verdict.one_line() -> {}", Verdict::Tied(3).one_line());
    println!("   default method, inherited free by Ballot:");
    println!("     b.shout()      -> {}", b.shout());
    println!("   ...and overridden by Verdict:");
    println!("     Verdict.shout()    -> {}", Verdict::Tied(3).shout());
}
