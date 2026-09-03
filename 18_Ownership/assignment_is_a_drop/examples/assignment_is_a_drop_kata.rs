//! Kata solution: six statements, and only three of them drop anything.
//!
//!   rustc --edition 2024 assignment_is_a_drop_kata.rs -o /tmp/aiadk && /tmp/aiadk

use std::mem;

struct T(&'static str);

impl Drop for T {
    fn drop(&mut self) {
        println!("       drop {}", self.0);
    }
}

fn main() {
    println!("A. reassignment                     let mut a = T(1); a = T(2);");
    {
        let mut a = T("one");
        println!("       a holds {}", a.0);
        a = T("two");
        println!("       <- one drop, above, on the assignment line");
        println!("       a now holds {}", a.0);
        println!("       end of block:");
    }

    println!("\nB. first write to a deferred `let`  let b; b = T(3);");
    {
        let b;
        b = T("three");
        println!("       b holds {}   <- nothing dropped: the location was empty", b.0);
        println!("       end of block:");
    }

    println!("\nC. write to a moved-out binding     let m = c; c = T(5);");
    {
        let mut c = T("four");
        let m = c;
        c = T("five");
        println!("       m = {}, c = {}   <- nothing dropped: `c` had been emptied", m.0, c.0);
        println!("       end of block:");
    }

    println!("\nD. write through &mut               *r = T(7);");
    {
        let mut d = T("six");
        let r = &mut d;
        *r = T("seven");
        println!("       <- one drop, above: `*r = v` is an assignment");
        println!("       d = {}", d.0);
        println!("       end of block:");
    }

    println!("\nE. shadowing                        let e = T(8); let e = T(9);");
    {
        let e = T("eight");
        println!("       e = {}", e.0);
        let e = T("nine");
        println!("       e = {}   <- nothing dropped: `let` declares, it does not write", e.0);
        println!("       end of block, and BOTH are still alive:");
    }
    println!("       ^ reverse declaration order, so the shadow dies first and");
    println!("         `eight` outlives the name that hid it.");

    println!("\nF. mem::replace                     let old = replace(&mut f, T(11));");
    {
        let mut f = T("ten");
        let old = mem::replace(&mut f, T("eleven"));
        println!("       old = {}, f = {}   <- nothing dropped: it was returned", old.0, f.0);
        println!("       end of block:");
    }

    println!("\nTally: two of the six dropped anything where the statement sits,");
    println!("       A and D. B and C did not, because an assignment drops what");
    println!("       was THERE and both locations were empty; E did not, because");
    println!("       `let` declares rather than writes; F did not, because the old");
    println!("       value was handed back instead.");
}
