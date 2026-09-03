//! The drop flag: the boolean the compiler adds to your stack frame.
//!
//!   rustc --edition 2024 the_drop_flag.rs -o /tmp/tdf && /tmp/tdf

use std::mem::size_of;

struct Named(&'static str);

impl Drop for Named {
    fn drop(&mut self) {
        println!("     drop {}", self.0);
    }
}

fn consume(n: Named) {
    println!("     consumed {} (and dropped it here)", n.0);
}

/// One function, one `let`, one closing brace — and two different schedules.
fn maybe_move(flag: bool) {
    let n = Named("value");
    if flag {
        consume(n);
    }
    println!("     end of maybe_move(flag = {flag})");
}

/// No `Drop` impl, so its fields can be moved out one at a time.
struct Pair {
    left: Named,
    right: Named,
}

fn main() {
    println!("1. The same source line, two different answers");
    maybe_move(true);
    println!("     ---");
    maybe_move(false);
    println!("   Identical code. In the first call the value was dropped inside");
    println!("   `consume`; in the second, at the closing brace. Something read at");
    println!("   run time decided which — and it is not part of the value.\n");

    println!("2. It is not in the type");
    println!("     size_of::<Named>() == size_of::<&'static str>() : {}",
             size_of::<Named>() == size_of::<&'static str>());
    println!("     a Drop impl adds nothing to the type's size, so the flag is");
    println!("     somewhere else: a slot in the stack frame of the function that");
    println!("     might or might not have moved the value out.\n");

    println!("3. Dropped exactly once, whichever way the branch goes");
    for flag in [true, false] {
        println!("     flag = {flag}");
        let n = Named("counted");
        if flag {
            consume(n);
        }
    }
    println!("   Once per iteration, never twice, never zero times. That is the");
    println!("   guarantee; the flag is how it is kept when the compiler cannot");
    println!("   settle the question by reading the source.\n");

    println!("4. Fields are tracked one at a time");
    {
        let p = Pair { left: Named("p.left"), right: Named("p.right") };
        let taken = p.left;
        println!("     moved out `p.left` -> {}, leaving {}", taken.0, p.right.0);
        drop(taken);
        println!("     end of block, and only `p.right` is left to drop:");
    }
    println!("   `p` is partly moved: the compiler tracks the two fields");
    println!("   separately, so the brace drops one of them and knows not to");
    println!("   touch the other.\n");

    println!("5. Which is why a type with its own Drop cannot be split");
    println!("     Give `Pair` a `Drop` impl and `let taken = p.left;` stops");
    println!("     compiling: error[E0509], cannot move out of type `Pair`,");
    println!("     which implements the `Drop` trait. There is no half-value to");
    println!("     hand `drop(&mut self)`, so the move is refused outright.\n");

    println!("6. The version where the emptiness is a value you can see");
    {
        let mut slot = Some(Named("in the Option"));
        for round in 0..3 {
            if round == 1 {
                if let Some(v) = slot.take() {
                    consume(v);
                }
            }
            println!("     round {round}: slot is_some = {}", slot.is_some());
        }
        println!("     end of block, and there is nothing left to drop:");
    }
    println!("   `Option::take` puts the same decision in the type, where you can");
    println!("   read it, test it, and return it. The flag does the identical job");
    println!("   invisibly — which is fine, until you want to ASK.");
}
