//! What `&x` does to `x` — the lock, when it lifts, and what it covers.
//!
//! Nothing here fails to compile: the refusals live on the page as rustc
//! transcripts, because a program that does not build has no output to record.
//! What this shows is the *legal* half — each point where the lock has lifted,
//! proved by doing the very thing the lock would have forbidden.
//!
//!   rustc --edition 2024 borrowed_state.rs -o /tmp/bs && /tmp/bs

#[derive(Debug)]
struct S(u8);

struct Loud(&'static str);

impl Drop for Loud {
    fn drop(&mut self) {
        println!("      drop: {}", self.0);
    }
}

struct Ballot {
    voter: String,
    score: u8,
}

impl Ballot {
    fn voter(&self) -> &str {
        &self.voter
    }
}

fn main() {
    println!("──── 1. The lock sits on the PLACE, not on the reference");
    let mut b = S(0);
    println!("  b = {:?}, holding {}", b, b.0);
    let r = &b;
    println!("  through r: {:?}   <- r's last use, so the lock lifts HERE", r);
    b = S(4);
    println!("  b = {:?}   <- assigned after the borrow ended, no error", b);
    println!("  Move that read below the assignment and the same lines are E0506.");

    println!();
    println!("──── 2. Copying the reference extends the lock");
    let mut c = S(2);
    let r = &c;
    let s = r;
    println!("  through s: {:?}   <- `s` is a copy of `r`, so `c` stays locked", s);
    println!("     until this line — the last use of ANY reference that may point at c");
    c = S(9);
    println!("  c = {:?}   <- legal once `s` is done, not once `r` is", c);

    println!();
    println!("──── 3. Two fields of one struct are two places");
    let mut ballot = Ballot { voter: String::from("Ada"), score: 3 };
    let who = &ballot.voter;
    ballot.score = 5;
    println!("  &ballot.voter held, ballot.score written anyway: {who} {}", ballot.score);
    println!("  The checker split the struct: `voter` is locked, `score` is not.");
    let name = ballot.voter();
    println!("  ballot.voter() takes &self, so it locks ALL of ballot — and this");
    println!("  is the last line allowed to read `name`: {name}");
    ballot.score = 7;
    println!("  ballot.score = {}   <- written after `name`'s last use", ballot.score);

    println!();
    println!("──── 4. A `ref` binding keeps the WHOLE value alive");
    let pair = |a, b| (Loud(a), Loud(b));
    {
        println!("  (a) both halves bound normally — neither dies before the brace:");
        let (_first, _second) = pair("1 — bound normally", "4 — bound normally");
        println!("      ...both still alive on this line, and the brace is next:");
    }
    {
        println!("  (b) second half bound to `_` — it dies on its own line:");
        let (_first, _) = pair("2 — bound normally", "5 — bound to `_`");
        println!("      ^ that drop is already above this line. The brace is next:");
    }
    {
        println!("  (c) first half bound by `ref` — now NEITHER half can die:");
        let (ref _first, _) = pair("3 — bound by ref", "6 — bound to `_`");
        println!("      nothing dropped yet, though `_` bound the second half. Brace:");
    }
    println!("  (b) and (c) differ by one keyword. `ref` borrows into the tuple, so");
    println!("  the whole temporary is held open — and value 6 rides along with it.");
}
