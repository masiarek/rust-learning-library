//! `use` is a shortcut, not an import: nothing is loaded, a name is bound.
//!
//!   rustc --edition 2024 the_use_declaration.rs -o /tmp/use && /tmp/use

use std::collections::BTreeMap;
use std::collections::HashMap as Table;   // renamed, because the next line needs it
use std::fmt::Write as FmtWrite;

mod election {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Method {
        Star,
        Approval,
        Plurality,
    }

    pub mod ballots {
        pub fn count() -> u32 {
            12
        }
        pub fn spoiled() -> u32 {
            1
        }
    }
}

// A whole module's worth of names, once.
use election::ballots::{count, spoiled};
// And the variants of an enum, so `Star` needs no prefix in this file.
use election::Method::{self, Approval, Star};

fn describe(m: Method) -> &'static str {
    match m {
        Star => "score then automatic runoff",
        Approval => "approve as many as you like",
        // The unimported variant still needs its path.
        election::Method::Plurality => "pick one",
    }
}

fn main() {
    println!("1. `use` binds a name; the item was already there");
    let mut t: Table<&str, u32> = Table::new();
    t.insert("Ada", 3);
    println!("   `use std::collections::HashMap as Table` -> Table::new() = {t:?}");
    println!("   Deleting the `use` line does not remove HashMap from the program.");
    println!("   It only means you have to spell std::collections::HashMap in full.");
    println!("   Nothing is compiled, loaded or linked by a `use`.");

    println!();
    println!("2. `as` renames, and is the fix for a collision");
    println!("   `use std::fmt::Write` and `use std::io::Write` in one file is");
    println!("   E0252: \"the name `Write` is defined multiple times\". Rename one.");
    let mut s = String::new();
    write!(s, "{} ballots", count()).unwrap();
    println!("   with fmt::Write in scope as FmtWrite: {s:?}");
    println!("   rustc even offers the fix: \"you can use `as` to change the");
    println!("   binding name of the import\". Note what this `use` bought — a");
    println!("   TRAIT in scope is what makes `write!` on a String resolve at all,");
    println!("   even though the name FmtWrite never appears again.");

    println!();
    println!("3. Braces bring several names from one path");
    println!("   use election::ballots::{{count, spoiled}};");
    println!("   count() = {}, spoiled() = {}", count(), spoiled());
    println!("   `{{self, …}}` also brings the module itself, so you can write both");
    println!("   `ballots::count()` and `count()`.");

    println!();
    println!("4. Importing enum variants, and when not to");
    println!("   use election::Method::{{self, Approval, Star}};");
    println!("   describe(Star)     = {}", describe(Star));
    println!("   describe(Approval) = {}", describe(Approval));
    println!("   describe(Method::Plurality) = {}", describe(Method::Plurality));
    println!("   Bare `Star` reads well inside a match on one enum, and badly in a");
    println!("   file with three enums that each have a `Star`. `Option`'s Some and");
    println!("   None are in the prelude for exactly this reason — they are common");
    println!("   enough that the ambiguity never arises.");

    println!();
    println!("5. The glob, and why it is rare");
    println!("   `use election::Method::*;` compiles and is discouraged: a new");
    println!("   variant upstream can silently shadow a local name, and a reader");
    println!("   cannot tell where a bare `Star` came from. The two accepted uses");
    println!("   are a prelude module (`use my_crate::prelude::*`) and the inside");
    println!("   of a test module (`use super::*`), where the source is obvious.");

    println!();
    println!("6. What is already in scope without any `use` at all");
    let ordered: BTreeMap<&str, u32> = t.iter().map(|(k, v)| (*k, *v)).collect();
    println!("   Vec, String, Option, Result, Box, Some, None, Ok, Err and the");
    println!("   traits listed in std::prelude are injected into every module.");
    println!("   Everything else needs a path or a use: {ordered:?} needed one.");
}
