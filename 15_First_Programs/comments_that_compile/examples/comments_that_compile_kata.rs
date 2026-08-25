//! Solution: the four comments, each moved to where it does its job.
//!
//! This inner doc comment is the first fix — in the broken version it was a
//! `///` sitting inside `main`, where it documented nothing and earned a
//! warning. A description of the whole file is what `//!` is for, and it has
//! to come before any item.

/// Prints the attributes an item actually carries, so a fix can be checked
/// rather than assumed. Same instrument as the lesson.
macro_rules! reveal_attrs {
    ($reveal:ident; $label:expr; $(#[$m:meta])* struct $name:ident { $field:ident : $ty:ty }) => {
        #[allow(dead_code)]
        struct $name { $field: $ty }
        fn $reveal() {
            println!("{} carries:", $label);
            $( println!("    #[{}]", stringify!($m)); )*
        }
    };
}

reveal_attrs! {
    reveal_ballot;
    "`Ballot`";
    /// One voter's filled-in paper.
    struct Ballot { score: u8 }
}

fn main() {
    reveal_ballot();

    // The second fix: this was a `///` before a `let`, which is a statement,
    // not an item — a warning, and documentation nobody would ever read. An
    // ordinary `//` is what a note-to-the-reader inside a function should be.
    let score: u8 = 3;

    println!("score = {score}");

    // The third fix: the trailing `///` with nothing after it was E0585.
    // Deleted, because it was a note about the function above it — and a note
    // about an item goes BEFORE the item, which is the whole rule.
}
