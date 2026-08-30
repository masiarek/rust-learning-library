//! An attribute is metadata the compiler acts on. Five families cover most of it.
//!
//!   rustc --edition 2024 what_an_attribute_is.rs -o /tmp/attr && /tmp/attr

// An INNER attribute: `#!` applies to the thing it is inside — here, the
// whole crate. Must come before any item.
#![allow(clippy::needless_return)]

/// The one you will write most: derive asks the compiler to generate an impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Score(u8);

#[derive(Debug)]
#[non_exhaustive]
pub enum Method {
    Star,
    Approval,
}

/// An OUTER attribute: `#[...]` applies to the item after it.
#[allow(dead_code)]
fn never_called() -> u32 {
    0
}

#[cfg(test)]
mod tests {
    //! Compiled only when rustc is run with --test. Without the attribute
    //! this module would be built into every release binary.
    #[test]
    fn a_score_defaults_to_zero() {
        assert_eq!(super::Score::default(), super::Score(0));
    }
}

#[inline]
fn doubled(n: u32) -> u32 {
    n * 2
}

fn main() {
    println!("1. `derive` writes the impl you would have written");
    let a = Score(4);
    let b = a; // Copy, so this is not a move
    println!("   Debug:      {a:?}");
    println!("   Copy:       a is still usable after `let b = a`: {a:?} {b:?}");
    println!("   PartialEq:  a == b is {}", a == b);
    println!("   Ord:        sorted [4, 1, 3] -> {:?}", {
        let mut v = vec![Score(4), Score(1), Score(3)];
        v.sort();
        v
    });
    println!("   Default:    Score::default() = {:?}", Score::default());
    println!("   Eight traits, no bodies. Each one is a real impl block the");
    println!("   compiler generated, and each has a rule: derived Ord compares");
    println!("   fields in DECLARATION ORDER, so reordering a struct's fields is");
    println!("   a behaviour change.");

    println!();
    println!("2. Inner and outer, and how to tell them apart");
    println!("   #[attr]   applies to the item BELOW it       (outer)");
    println!("   #![attr]  applies to the item it is INSIDE   (inner)");
    println!("   The `!` is the whole difference, and an inner attribute has to be");
    println!("   the first thing in its file or block. `//!` and `///` follow the");
    println!("   same rule, because a doc comment IS an attribute — `#[doc = \"…\"]`.");

    println!();
    println!("3. The lint family: allow, warn, deny, forbid");
    println!("   #[allow(dead_code)] on never_called() is why this program has no");
    println!("   warnings. The four levels differ in what they do and in whether");
    println!("   an inner scope may override them:");
    println!("     allow   say nothing");
    println!("     warn    print, keep compiling");
    println!("     deny    error");
    println!("     forbid  error, and an inner allow is itself an error");
    println!("   They nest, so a crate-level `#![deny(warnings)]` can be relaxed");
    println!("   on one function — which is the whole reason to prefer deny over");
    println!("   forbid.");

    println!();
    println!("4. `cfg`: compiled or not compiled at all");
    println!("   #[cfg(test)] on the tests module above means it does not exist in");
    println!("   this binary — not that it is skipped at run time.");
    println!("   cfg!(test) here = {}   <- this binary was not built with --test",
             cfg!(test));
    println!("   `#[cfg(…)]` removes code; `cfg!(…)` is an expression that becomes");
    println!("   a literal true or false. The second still type-checks both sides,");
    println!("   which is why a `cfg!(windows)` branch cannot rot on a Mac.");
    println!("   (Nothing here prints the target OS on purpose: every example in");
    println!("   this library has a recorded answer key, and `env::consts::OS` says");
    println!("   \"macos\" on the author's machine and \"linux\" in CI. That is the");
    println!("   determinism rule, and it is easier to break than it looks.)");

    println!();
    println!("5. The rest, in one line each");
    println!("   #[test]           this fn is a test; rustc --test collects it");
    println!("   #[derive(…)]      generate these impls");
    println!("   #[non_exhaustive] downstream crates may not match it exhaustively");
    println!("                     or build it with a struct literal: {:?}", Method::Star);
    println!("   #[inline]         a hint, not an instruction: doubled(21) = {}",
             doubled(21));
    println!("   #[must_use]       warn if the return value is dropped");
    println!("   #[repr(C)]        lay this type out the way C would");
    println!("   #[deprecated]     warn at every use, with a message");
}
