//! Kata solution: prove the reuse, then let the compiler stop you using it.
//!
//! The two refusals cannot be in this file -- an example that does not compile
//! has no answer key and would fail the gate. They are written as comments
//! beside the fixes, and their transcripts are on the lesson page, taken from
//! `dead_frame.rs` and `dead_frame_named.rs` compiled by hand.
//!
//!   rustc --edition 2024 a_stack_slot_is_reused_kata.rs -o /tmp/assirk && /tmp/assirk

#[derive(Debug)]
struct Ballot {
    precinct: u32,
    score: u32,
}

/// Part 1. Records where its own local sat, and hands back the address as a
/// plain number -- which is legal, and useless, and exactly the point.
#[inline(never)]
fn address_of_local(precinct: u32) -> usize {
    let b = Ballot { precinct, score: 0 };
    &b as *const Ballot as usize
}

/// Part 2. A different local, a different type, called from the same place.
#[inline(never)]
fn address_of_other_local() -> usize {
    let running_total: u64 = 0;
    &running_total as *const u64 as usize
}

// Part 3, the two refusals. Neither can be written here, so both are quoted.
//
//   fn make() -> &Ballot { let b = Ballot { .. }; &b }
//     error[E0106]: missing lifetime specifier
//     ...the SIGNATURE is wrong: a returned reference has to borrow from
//        something, and this function has nothing to borrow from.
//
//   fn make<'a>(seed: &'a u32) -> &'a Ballot { let b = Ballot { .. }; &b }
//     error[E0515]: cannot return reference to local variable `b`
//     ...the signature is now writable, so the BODY gets checked, and the
//        real bug is named: "returns a reference to data owned by the
//        current function". The first error asks where the lifetime comes
//        from; the second says this local is not it.

/// Part 4, fix one: return the VALUE. It moves into a slot the caller owns.
#[inline(never)]
fn built(precinct: u32) -> Ballot {
    Ballot { precinct, score: 5 }
}

/// Part 4, fix two: borrow from an argument, which outlives the call.
/// Elision fills the lifetime in, so no `'a` has to be written.
#[inline(never)]
fn highest(ballots: &[Ballot]) -> &Ballot {
    ballots.iter().max_by_key(|b| b.score).expect("non-empty")
}

fn main() {
    println!("Part 1 — the same call twice.\n");
    let first = address_of_local(7);
    let second = address_of_local(9);
    println!("    two calls, one address:  {}", first == second);
    println!("\n  Not a coincidence of this build. `address_of_local` returned, the");
    println!("  stack pointer was restored to where it had been, and the second");
    println!("  call started from the same offset. Same frame layout, same slot.");

    println!("\nPart 2 — a different function, from the same place.\n");
    let other = address_of_other_local();
    println!("    a u64 local, within 256 bytes of the Ballots:  {}",
             other.abs_diff(first) < 256);
    println!("\n  Checked as a distance, not an equality: a different function has a");
    println!("  different frame layout, so its local sits at a different offset");
    println!("  inside the SAME reissued region. Equality would be luck; overlap");
    println!("  is the claim worth making.");

    println!("\nPart 3 — what the two errors each tell you.\n");
    println!("    E0106 is about the SIGNATURE: no lifetime is available.");
    println!("    E0515 is about the BODY: the lifetime you supplied is not this");
    println!("    local's. Fixing the first only gets you as far as the second,");
    println!("    which is the useful part -- naming a lifetime never lengthens");
    println!("    one, so there was never a spelling that made the local survive.");

    println!("\nPart 4 — the two fixes, both compiling above.\n");
    let owned = built(7);
    println!("    returned by value:   {owned:?}");
    println!("    ...moved into a slot main provided, so no frame outlived it");

    let roster = vec![
        Ballot { precinct: 1, score: 3 },
        Ballot { precinct: 2, score: 9 },
        Ballot { precinct: 3, score: 4 },
    ];
    let top = highest(&roster);
    println!("    borrowed from an argument: {top:?}");
    println!("    ...precinct {} won it with {} points", top.precinct, top.score);
    println!("    ...the reference borrows `roster`, which is main's, so it is");
    println!("    alive for as long as the caller keeps it alive");

    println!("\n  Which would I write? Return the value. Borrowing from an argument");
    println!("  is right when the caller already owns the data and you are picking");
    println!("  something OUT of it -- `highest` is that shape and a lifetime would");
    println!("  be wrong there. When the function CREATED the value, the caller has");
    println!("  nothing for it to borrow from, and a reference is the wrong return");
    println!("  type rather than an annotation problem.");
}
