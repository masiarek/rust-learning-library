//! `Drop` runs at a place you can point to in the source. That is RAII.
//!
//!   rustc --edition 2024 drop_and_raii.rs -o /tmp/draii && /tmp/draii

/// Anything that prints when it dies makes ownership visible.
struct Loud(&'static str);

impl Drop for Loud {
    fn drop(&mut self) {
        println!("     drop {}", self.0);
    }
}

/// The shape RAII actually takes in real code: acquire in `new`, release in
/// `drop`, and the caller cannot forget because there is nothing to call.
struct Tally {
    name: &'static str,
    total: u32,
}

impl Tally {
    fn open(name: &'static str) -> Self {
        println!("     open {name}");
        Tally { name, total: 0 }
    }
    fn add(&mut self, n: u32) {
        self.total += n;
    }
}

impl Drop for Tally {
    fn drop(&mut self) {
        println!("     close {} at {}", self.name, self.total);
    }
}

fn consume(l: Loud) {
    println!("     consume got {}", l.0);
}

fn main() {
    println!("1. Last declared, first dropped");
    {
        let _a = Loud("a");
        let _b = Loud("b");
        let _c = Loud("c");
        println!("     end of block");
    }
    println!("   Reverse declaration order, always — because a later value may");
    println!("   borrow an earlier one, and never the other way round.");

    println!();
    println!("2. Moving moves the drop with it");
    {
        let l = Loud("moved");
        consume(l);
        println!("     back in main, and it is already gone");
    }
    println!("   The drop happened at the end of `consume`, not here. Whoever owns");
    println!("   the value owns the free — which is what makes ownership readable:");
    println!("   find the last owner and you have found the deallocation.");

    println!();
    println!("3. Ending a value early");
    {
        let l = Loud("early");
        drop(l);
        println!("     still in the block, and it is gone");
    }
    println!("   `drop(x)` is a one-line function that takes x by value and does");
    println!("   nothing. The value dies because it was MOVED into a function that");
    println!("   ended — no magic, and it is why you cannot call `l.drop()`");
    println!("   yourself: that is E0040, \"explicit use of destructor method\".");

    println!();
    println!("4. RAII: the release you cannot forget");
    {
        let mut t = Tally::open("round 1");
        t.add(5);
        t.add(3);
        println!("     ...counting...");
    }
    println!("   There is no close() to call and no `finally` to write. The");
    println!("   compiler inserted the call at the end of the scope, and it runs");
    println!("   on the early-return path and on the panic path too.");

    println!();
    println!("5. Fields drop after the struct, in declaration order");
    struct Pair {
        first: Loud,
        second: Loud,
    }
    impl Drop for Pair {
        fn drop(&mut self) {
            println!("     drop Pair itself");
        }
    }
    {
        let p = Pair { first: Loud("field first"), second: Loud("field second") };
        println!("     built from {} and {}", p.first.0, p.second.0);
        println!("     end of block");
    }
    println!("   The struct's own Drop::drop runs FIRST, then its fields in");
    println!("   declaration order — the opposite of the local-variable rule, and");
    println!("   the one that surprises people. It has to be that way: your drop");
    println!("   body may still need the fields.");

    println!();
    println!("6. What Drop is not for");
    println!("   Not for returning a value: drop takes &mut self, so it cannot");
    println!("   move anything out, and it cannot fail — there is no Result.");
    println!("   A close() that can error still needs to be called by hand, with");
    println!("   Drop as the backstop that at least frees the handle.");
    println!("   And a value can leak: std::mem::forget, an Rc cycle, or a process");
    println!("   that aborts all skip it. Leaking is SAFE in Rust; unsoundness is");
    println!("   what the language promises to prevent, not garbage.");
}
