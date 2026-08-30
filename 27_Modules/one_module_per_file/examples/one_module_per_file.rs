//! `mod name;` is a declaration, not an include — and the path is the same
//! whether the module is a block or a file.
//!
//!   rustc --edition 2024 one_module_per_file.rs -o /tmp/omf && /tmp/omf

// Written inline here, because this whole library compiles one file at a
// time. Everything below is true of the split version, unchanged.
mod election {
    pub mod ballots {
        pub fn count() -> u32 {
            12
        }
    }
    pub mod tally {
        pub fn total() -> u32 {
            super::ballots::count() * 5
        }
    }
    pub fn summary() -> String {
        format!("{} ballots, {} points", ballots::count(), tally::total())
    }
}

fn main() {
    println!("1. The tree above, as files");
    println!("   src/");
    println!("     main.rs              <- the crate root: `mod election;`");
    println!("     election.rs          <- `mod ballots;` and `mod tally;`");
    println!("     election/");
    println!("       ballots.rs");
    println!("       tally.rs");
    println!("   Or, in the older layout that still works:");
    println!("     election/mod.rs      instead of election.rs");
    println!("   Both are supported. `election.rs` beside `election/` is the");
    println!("   2018-edition form and the one to use; `mod.rs` is what you will");
    println!("   meet in older code, and a directory full of files called mod.rs");
    println!("   is why it was changed.");

    println!();
    println!("2. `mod election;` is a declaration");
    println!("   It says \"there is a module called election, its contents are in");
    println!("   the file next door\". It is NOT #include, and NOT an import: the");
    println!("   file is compiled as part of this crate exactly once, and a second");
    println!("   `mod election;` elsewhere is E0428, a duplicate definition.");
    println!("   A .rs file nobody declares is not compiled at all — the most");
    println!("   common way a new file appears to do nothing.");

    println!();
    println!("3. The paths do not change when you split");
    println!("   election::summary() = {}", election::summary());
    println!("   election::tally::total() = {}", election::tally::total());
    println!("   Those two lines are identical in both layouts. The module tree");
    println!("   is a property of the `mod` declarations, not of the directories,");
    println!("   which is why moving a module between file and block is a pure");
    println!("   refactor.");

    println!();
    println!("4. The crate root, and what `crate::` means");
    println!("   A binary's root is src/main.rs; a library's is src/lib.rs. That");
    println!("   file IS the crate's top module, so an item declared there is at");
    println!("   `crate::name` and needs no module of its own.");
    println!("   crate::election::ballots::count() = {}",
             crate::election::ballots::count());
    println!("   A package with both files is two crates — a library and a binary");
    println!("   that uses it by name, not by path. That is why `use my_crate::…`");
    println!("   appears in main.rs and `use crate::…` inside lib.rs.");

    println!();
    println!("5. What decides where an item lives");
    println!("   Not the file. `pub` and the module tree decide visibility, and");
    println!("   the file layout is a convenience for humans on top of it. So the");
    println!("   question when splitting is never \"which file is this in\" but");
    println!("   \"which module can see it\" — and a 900-line file with three");
    println!("   `mod` blocks has exactly the same answer as three files.");
}
