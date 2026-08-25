//! Which defence actually catches a silent catch-all binding?
//! Two lints are denied at the crate level below. Neither one fires.
#![deny(unreachable_patterns, bindings_with_variant_name, unused_variables)]

#[derive(Debug, Clone, Copy)]
enum Mark { Scored(u8), Blank, Spoiled, Reissued }

/// The trap: `use Mark::*` plus a lowercase `spoiled` in the last arm.
/// `Reissued` was added afterwards and is silently reported as spoiled.
fn tally_glob(m: &Mark) -> (u8, &'static str) {
    use Mark::*;
    match m {
        Scored(n) => (*n, "counted"),
        Blank => (0, "left blank"),
        spoiled => {
            let _ = spoiled;
            (0, "spoiled")
        }
    }
}

/// The fix, and the only one of the three that works: qualify every path.
/// `Mark::spoiled` is `E0599`, so the typo cannot be written at all — and the
/// exhaustiveness check comes back, so `Reissued` had to be given an arm.
fn tally_qualified(m: &Mark) -> (u8, &'static str) {
    match m {
        Mark::Scored(n) => (*n, "counted"),
        Mark::Blank => (0, "left blank"),
        Mark::Spoiled => (0, "spoiled"),
        Mark::Reissued => (0, "spoiled, replacement issued"),
    }
}

fn main() {
    let ballot = [Mark::Scored(5), Mark::Blank, Mark::Spoiled, Mark::Reissued];

    println!("two lints denied at the top of this file, and it still compiled.\n");
    println!("{:<12} {:<22} {}", "mark", "glob-imported (typo)", "qualified");
    println!("{}", "-".repeat(70));
    for m in &ballot {
        let (gv, gr) = tally_glob(m);
        let (qv, qr) = tally_qualified(m);
        let flag = if gr == qr { "" } else { "  <- report is wrong" };
        println!("{:<12} {:<22} {}{}", format!("{m:?}"), format!("{gv} / {gr}"), format!("{qv} / {qr}"), flag);
    }
}
