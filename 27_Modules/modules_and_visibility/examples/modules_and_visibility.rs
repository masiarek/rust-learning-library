//! Modules: a namespace, and a wall that is private by default.
//!
//!   rustc --edition 2024 modules_and_visibility.rs -o /tmp/mv && /tmp/mv

mod election {
    /// Visible outside this module.
    pub const SEATS: u32 = 3;

    /// Not visible outside it. Nothing said `pub`.
    const QUORUM: u32 = 10;

    pub fn seats_and_quorum() -> (u32, u32) {
        (SEATS, QUORUM)
    }

    pub mod ballots {
        /// A child can see everything its ancestors have, public or not.
        pub fn quorum_from_child() -> u32 {
            super::QUORUM
        }

        /// `pub` here means "public to whoever can see this module", not
        /// "public to the world" — `ballots` itself has to be pub as well.
        pub fn count() -> u32 {
            12
        }
    }

    pub mod tally {
        /// Visible to `election` and its descendants, and nobody else.
        pub(super) fn internal_total() -> u32 {
            super::ballots::count() * 5
        }

        pub fn total() -> u32 {
            internal_total()
        }
    }

    /// A pub struct with private fields: the type escapes, the fields do not.
    pub struct Result {
        pub winner: &'static str,
        margin: u32,
    }

    impl Result {
        pub fn new(winner: &'static str, margin: u32) -> Self {
            Result { winner, margin }
        }
        pub fn margin(&self) -> u32 {
            self.margin
        }
    }
}

use election::ballots;

fn main() {
    println!("1. Private by default, at every level");
    println!("   election::SEATS            = {}", election::SEATS);
    println!("   election::QUORUM           -> E0603: \"constant `QUORUM` is private\"");
    println!("   election::seats_and_quorum() = {:?}", election::seats_and_quorum());
    println!("   The module can read its own private items and hand the values");
    println!("   out. What is private is the NAME, not the data.");

    println!();
    println!("2. Privacy is one-way: children see up, parents do not see in");
    println!("   ballots::quorum_from_child() = {}", ballots::quorum_from_child());
    println!("   A child module reaches its parent's private items through");
    println!("   `super::`. The parent cannot reach a child's private items at");
    println!("   all — which is the whole reason a module is a useful boundary.");

    println!();
    println!("3. `pub` is relative, and `pub(super)` / `pub(crate)` say how far");
    println!("   election::tally::total()          = {}", election::tally::total());
    println!("   election::tally::internal_total() -> E0603: \"function");
    println!("   `internal_total` is private\" — same code as QUORUM above, because");
    println!("   pub(super) stops at `election` and main is outside it.");
    println!("   pub            as far as whoever can see the module");
    println!("   pub(crate)     anywhere in this crate, nowhere outside it");
    println!("   pub(super)     the parent module and its descendants");
    println!("   pub(in path)   one named ancestor module");

    println!();
    println!("4. A public struct with private fields");
    let r = election::Result::new("Ada", 7);
    println!("   r.winner   = {}   <- pub field", r.winner);
    println!("   r.margin   -> E0616: \"field `margin` of struct `Result` is private\"");
    println!("   r.margin() = {}   <- the accessor is the door that stayed open", r.margin());
    println!("   Fields are private individually, so `pub struct` says nothing");
    println!("   about them. This is what stops a caller building a Result with");
    println!("   a margin that does not match the ballots.");

    println!();
    println!("5. Paths: `crate::`, `super::`, `self::`");
    println!("   crate::election::SEATS  = {}   absolute, from the crate root",
             crate::election::SEATS);
    println!("   self::election::SEATS   = {}   relative to this module",
             self::election::SEATS);
    println!("   super::               one level up — used inside a module, and");
    println!("                         an error at the crate root, where there is");
    println!("                         nothing above.");
}
