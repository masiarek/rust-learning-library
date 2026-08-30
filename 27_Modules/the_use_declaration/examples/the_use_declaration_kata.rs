//! Kata solution: two Writes, a glob that shadows, and `use super::*`.
//!
//!   rustc --edition 2024 the_use_declaration_kata.rs -o /tmp/udk && /tmp/udk

use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

mod election {
    #[derive(Debug, PartialEq)]
    pub enum Method {
        Star,
        Approval,
    }

    pub const SEATS: u32 = 3;

    pub fn describe() -> &'static str {
        "one race"
    }
}

mod other {
    /// A name that collides with election::SEATS, on purpose.
    pub const SEATS: u32 = 99;

    /// And one that does not, so the glob below is genuinely in use.
    pub fn tiebreak() -> &'static str {
        "lot"
    }
}

// Both globs. The compiler does NOT reject this; it defers.
use election::*;
use other::*;

fn main() {
    println!("1. Two traits called Write, both needed");
    let mut text = String::new();
    write!(text, "{} seats", election::SEATS).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    IoWrite::write_all(&mut bytes, text.as_bytes()).unwrap();
    println!("   fmt::Write on a String : {text:?}");
    println!("   io::Write on a Vec<u8> : {:?}", String::from_utf8(bytes).unwrap());
    println!("   Without the `as` renames this file is E0252, \"the name `Write` is");
    println!("   defined multiple times\". rustc even writes the fix for you:");
    println!("   \"you can use `as` to change the binding name of the import\".");
    println!("   Note that FmtWrite is never NAMED below — bringing the trait into");
    println!("   scope is the entire job, because `write!` on a String needs it.");

    println!();
    println!("2. Two globs, one ambiguous name");
    println!("   `use election::*;` and `use other::*;` both bring in SEATS.");
    println!("   This compiles. Writing a bare `SEATS` does not:");
    println!("   E0659: \"`SEATS` is ambiguous\", \"ambiguous because of multiple");
    println!("   glob imports of a name in the same module\".");
    println!("   election::SEATS = {}, other::SEATS = {}", election::SEATS, other::SEATS);
    println!("   The error arrives at the USE SITE, not at the import — so a glob");
    println!("   can sit there harmlessly for months and then break the day");
    println!("   somebody adds a name upstream.");

    println!();
    println!("3. What the glob did bring in unambiguously");
    println!("   describe() = {}   <- from election, via the glob", describe());
    println!("   tiebreak() = {}        <- from other, via the other glob", tiebreak());
    println!("   Method::Star == Method::Approval: {}", Method::Star == Method::Approval);
    println!("   Which is the glob's actual cost: a reader cannot tell where");
    println!("   `describe` or `Method` came from without grepping both modules.");

    println!();
    println!("4. The two places a glob is fine");
    println!("   a. `use my_crate::prelude::*` — a module that exists to be");
    println!("      glob-imported, whose contents are a documented API.");
    println!("   b. `use super::*` at the top of a #[cfg(test)] module — the");
    println!("      source is the file you are already reading, and the tests");
    println!("      want everything in it, private items included.");
    println!("   Both share one property: there is exactly one place the names");
    println!("   can have come from.");
}
