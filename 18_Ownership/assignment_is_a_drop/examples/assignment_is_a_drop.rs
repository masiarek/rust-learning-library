//! Assignment drops the old value — a death with no brace in sight.
//!
//!   rustc --edition 2024 assignment_is_a_drop.rs -o /tmp/aiad && /tmp/aiad

use std::mem;

/// A value that announces its own death, so the schedule is readable.
struct Tally(&'static str, u32);

impl Drop for Tally {
    fn drop(&mut self) {
        println!("     drop  {} ({})", self.0, self.1);
    }
}

/// Announces its own birth too, so the ORDER of the two is visible.
fn built(name: &'static str, n: u32) -> Tally {
    println!("     built {}", name);
    Tally(name, n)
}

fn main() {
    println!("1. Assignment drops the old value");
    {
        let mut round = built("round 1", 12);
        println!("     round 1 counted {} ballots", round.1);
        round = built("round 2", 30);
        println!("     still inside the block, one line to go");
        println!("     round is now {} ({})", round.0, round.1);
    }
    println!("   `round 1` died on the assignment line. No brace closed, nothing");
    println!("   went out of scope, and the statement reads like an edit.");
    println!("   Read the order too: BUILT round 2 comes before DROP round 1.");
    println!("   The right-hand side is evaluated first, then the old value is");
    println!("   dropped, then the new one is stored.\n");

    println!("2. Two assignments that drop nothing");
    {
        let mut owner = built("owner", 1);
        let elsewhere = owner;
        println!("     moved out of `owner`; now assigning to it again:");
        owner = built("replacement", 2);
        println!("     no drop line appeared — the location held nothing to drop");
        println!("     both are alive: {} and {}", elsewhere.0, owner.0);
    }
    {
        let later;
        println!("     `let later;` declares a name over an empty location");
        later = built("first value", 3);
        println!("     no drop line here either: {} ({})", later.0, later.1);
    }
    println!("   An assignment drops what was there. Twice above, nothing was.\n");

    println!("3. Writing through &mut T drops too");
    {
        let mut slot = built("old", 7);
        let r = &mut slot;
        *r = built("new", 8);
        println!("     the write through `r` dropped what `slot` held");
        println!("     slot is now {} ({})", slot.0, slot.1);
    }
    println!("   `*r = value` is an assignment, so the same rule applies — and");
    println!("   you cannot move the old value out through `&mut`, only drop it.\n");

    println!("4. Keeping the old value instead of dropping it");
    {
        let mut slot = built("outgoing", 5);
        let previous = mem::replace(&mut slot, built("incoming", 6));
        println!("     replace handed the old value back: {} ({})", previous.0, previous.1);
        println!("     no drop line — it is alive, and it is mine now");
        println!("     slot holds {} ({})", slot.0, slot.1);
        println!("     end of block:");
    }
    println!("   That is the whole difference between `*r = v` and");
    println!("   `mem::replace(r, v)`: one drops the old value, one returns it.\n");

    println!("5. The other two, on types that need no Drop impl to show it");
    {
        let mut name = String::from("Ada");
        let taken = mem::take(&mut name);
        println!("     take:  taken = {taken:?}, name = {name:?}   (name got Default)");

        let mut a = String::from("first");
        let mut b = String::from("second");
        mem::swap(&mut a, &mut b);
        println!("     swap:  a = {a:?}, b = {b:?}   (nothing dropped, nothing cloned)");
    }
    println!("   `take` needs Default, `swap` needs nothing, `replace` needs neither.\n");

    println!("6. Why it is worth knowing");
    {
        let mut buffer = String::with_capacity(64);
        for line in ["first", "second", "third"] {
            buffer = String::from(line);
        }
        println!("     the loop assigned 3 times, so 3 buffers were freed: {buffer:?}");

        let mut kept = String::with_capacity(64);
        for line in ["first", "second", "third"] {
            kept.clear();
            kept.push_str(line);
        }
        println!("     clear + push_str reuses one buffer:              {kept:?}");
    }
    println!("   Same result, and the first version does an allocation and a free");
    println!("   per round because every `=` freed what was there.");
}
