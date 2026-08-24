//! Kata solution: one E0382, three fixes, and the field that forbids one of them.
//!
//!   rustc --edition 2024 copy_vs_clone_kata.rs -o /tmp/cvck && /tmp/cvck

// ---- Round 1: all fields Copy, so all three fixes are available -----------
#[derive(Debug, Clone, Copy)]
struct Reading {
    precinct: u32,
    turnout: u32,
}

fn report(r: Reading) -> String { format!("p{} turnout {}", r.precinct, r.turnout) }
fn report_ref(r: &Reading) -> String { format!("p{} turnout {}", r.precinct, r.turnout) }

// ---- Round 2: one String field, and Copy is now impossible ----------------
#[derive(Debug, Clone)]
struct Named {
    precinct: String, // <- this one field forbids Copy for the whole struct
    turnout: u32,
}

fn name_it(n: Named) -> String { format!("{} turnout {}", n.precinct, n.turnout) }
fn name_it_ref(n: &Named) -> String { format!("{} turnout {}", n.precinct, n.turnout) }

fn main() {
    println!("The bug: calling a by-value function twice.");
    println!("    let r = Reading {{ .. }};");
    println!("    report(r); report(r);");
    println!("    error[E0382]: use of moved value: `r`\n");

    println!("Fix 1 — derive Copy. The call sites do not change at all.");
    let r = Reading { precinct: 7, turnout: 431 };
    println!("    {}", report(r));
    println!("    {}   <- same `r`, copied again", report(r));

    println!("\nFix 2 — clone at the call site. Visible, and it costs.");
    println!("    {}", report(r.clone()));

    println!("\nFix 3 — take a reference. Nothing is duplicated at all.");
    println!("    {}", report_ref(&r));

    println!("\nWhich to ship? Fix 3, then Fix 1, and Fix 2 last.");
    println!("  A reference asks for the least and copies nothing, so it is the");
    println!("  default. Copy is right for a small, plain-data value where the");
    println!("  `&` would be noise. Clone at a call site is the one to justify:");
    println!("  it is a real allocation, and it is often papering over a signature");
    println!("  that should have taken `&` in the first place.");

    println!("\nRound 2 — swap `precinct` to a String, and one fix disappears.");
    let n = Named { precinct: "Riverside".to_string(), turnout: 431 };
    println!("    #[derive(Copy)] is now:");
    println!("      error[E0204]: the trait `Copy` cannot be implemented for this type");
    println!("        this field does not implement `Copy`");
    println!("    ...because Copy is a bit-for-bit duplicate, and duplicating a");
    println!("    String's pointer would give two owners of one allocation.");
    println!("    That is the double-free `Copy` exists to make unrepresentable.");
    println!("\n    So only two fixes remain:");
    println!("      clone     {}", name_it(n.clone()));
    println!("      reference {}", name_it_ref(&n));
    println!("      and `n` is still alive: {n:?}");

    println!("\nThe rule to carry away:");
    println!("  Copy is not 'cheap to duplicate' — it is 'duplicating is JUST a");
    println!("  memcpy, with nobody owning anything afterwards'. Any field that");
    println!("  owns a resource makes that false for the whole struct.");
}
