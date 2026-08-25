//! Eight struct refusals, and the fix each one is asking for.
//!
//!   rustc --edition 2024 when_a_struct_refuses.rs -o /tmp/wsr && /tmp/wsr

use std::fmt;

// --- 1. E0063: every field, every time -------------------------------------
#[derive(Debug, Default)]
struct Ballot {
    voter: String,
    score: u8,
}

// --- 2a. E0277 (Display): the human's format has to be written by hand ------
struct Receipt {
    voter: String,
    score: u8,
}
impl fmt::Display for Receipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} scored {}", self.voter, self.score)
    }
}

// --- 2c. E0277 (Sized): a `str` field has no size ---------------------------
struct Borrowed<'a> {
    voter: &'a str,
} // the reference has a size: 16 bytes
struct Owned {
    voter: Box<str>,
} // the box has a size: 16 bytes

// --- 2d. E0277 (Eq): Eq is a promise ABOUT PartialEq ------------------------
#[derive(PartialEq, Eq, Debug)]
struct Seat(u8);

// --- 3. E0119: pick derive OR the hand-written impl, never both -------------
#[derive(Debug)]
struct Quorum {
    needed: u32,
}
impl Default for Quorum {
    fn default() -> Self {
        Quorum { needed: 10 } // a domain default the derive could never guess
    }
}

// --- 5. E0282: a function that never mentions T still lives in impl<T> ------
struct Tally<T> {
    rows: Vec<T>,
}
impl<T> Tally<T> {
    fn quorum() -> u32 {
        10
    }
    fn rows(&self) -> usize {
        self.rows.len()
    }
}

fn main() {
    println!("1. E0063 — missing field in initializer");
    println!("   There is no partly-built struct in Rust. Name every field, or let");
    println!("   something else supply the rest:");
    let a = Ballot { voter: "Ada".into(), score: 5 };
    let b = Ballot { score: 5, ..Default::default() };
    println!("   named all:            {a:?}   (voter {:?}, score {})", a.voter, a.score);
    println!("   ..Default::default(): {b:?}   <- and this needs Default to exist");
    println!("   note the derived Default gave voter an EMPTY string, not a missing one:");
    println!("   the type's zero, which is rarely your domain's. b.voter.is_empty() = {}",
             b.voter.is_empty());

    println!("\n2. E0277 — one code, four different problems");
    println!("   E0277 is 'a trait bound was not satisfied'. For structs it shows up as:");

    println!("\n   2a. no Display, from `{{}}`");
    println!("       Rust will not guess how you want a human to read your type.");
    println!("       {}", Receipt { voter: "Ada".into(), score: 5 });

    println!("\n   2b. no Debug, from `{{:?}}`");
    println!("       Same code, opposite advice: this one it WILL generate.");
    println!("       {:?}", Seat(3));

    println!("\n   2c. `str` as a field type — not a missing impl, a missing SIZE");
    println!("       str is unsized, so it cannot be a field. Borrow it or box it:");
    let s = String::from("Ada");
    let borrowed = Borrowed { voter: &s };
    let owned = Owned { voter: "Ada".into() };
    println!("       &str  field: {}   Box<str> field: {}", borrowed.voter, owned.voter);
    println!("       The note is the part to read: 'only the LAST field of a struct");
    println!("       may have a dynamically sized type'.");

    println!("\n   2d. `#[derive(Eq)]` alone — Eq is a promise about PartialEq");
    println!("       Eq adds no methods. It says == is reflexive, so it needs the");
    println!("       PartialEq that defines ==. Always derive the pair:");
    println!("       Seat(3) == Seat(3) is {}", Seat(3) == Seat(3));

    println!("\n3. E0119 — conflicting implementations");
    println!("   #[derive(Default)] AND `impl Default` is two impls of one trait.");
    println!("   Keep the one that knows something the other cannot:");
    println!("   derived would be Quorum {{ needed: 0 }}; ours says needed = {}",
             Quorum::default().needed);

    println!("\n4. E0594 — cannot assign, not declared as mutable");
    println!("   `let b = b;` rebinds. mut belongs to the BINDING, so the new one");
    println!("   does not inherit it — and rustc's help points at line 5, not line 6.");
    println!("   The value never changed; the name did.");

    println!("\n5. E0282 — type annotations needed");
    println!("   `Tally::quorum()` names no T anywhere, but it lives in impl<T>, so");
    println!("   there is nothing to infer T from. Three fixes, in order of preference:");
    println!("     Tally::<i32>::quorum() = {}   name it at the call", Tally::<i32>::quorum());
    let t = Tally { rows: vec![1, 2, 3] };
    println!("     t.rows()               = {}   or have a receiver to infer from", t.rows());
    println!("     ...or move it out of impl<T> entirely, if it truly has no T in it.");

    println!("\nThe pattern across all eight: rustc is not withholding a fix.");
    println!("Seven of these print the exact edit, and the eighth (E0119) names both");
    println!("conflicting sites. The skill is reading the `note:` line, not the first one.");
}
