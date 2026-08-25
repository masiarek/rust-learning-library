//! Kata solution: predict which half of the base survives a `..base`.
//!
//!   rustc --edition 2024 struct_update_kata.rs -o /tmp/suk && /tmp/suk

#[derive(Debug, Default)]
struct Ballot {
    precinct: u32,       // Copy
    counted: bool,       // Copy
    voter: String,       // NOT Copy
    notes: String,       // NOT Copy
}

fn main() {
    let original = Ballot {
        precinct: 12,
        counted: false,
        voter: "Ada".to_string(),
        notes: "handed in late".to_string(),
    };

    // Only `notes` is named, so `..original` supplies the other THREE.
    let amended = Ballot { notes: "resolved".to_string(), ..original };

    println!("Predict, field by field, what is still readable on `original`:\n");
    println!("  precinct  Copy      -> copied   -> {} still readable", original.precinct);
    println!("  counted   Copy      -> copied   -> {} still readable", original.counted);
    println!("  voter     NOT Copy  -> MOVED    -> reading it is E0382");
    println!("  notes     NOT Copy  -> not taken (we named it) -> {:?} still readable", original.notes);
    println!("\n  amended = {amended:?}");

    println!("\nThe rule in one line:");
    println!("  `..base` moves exactly the fields you did NOT name,");
    println!("  and only the non-Copy ones among those actually go dead.");

    println!("\nThree ways to keep the base whole:");
    let base = Ballot { precinct: 3, counted: true,
                        voter: "Ben".to_string(), notes: "n/a".to_string() };

    // 1. Name every non-Copy field yourself.
    let a = Ballot { voter: "Cara".to_string(), notes: "n/a".to_string(), ..base };
    println!("  1. name every non-Copy field   -> base alive: {:?}", base.voter);

    // 2. Clone the base into the update position.
    let b = Ballot { precinct: 99, ..clone_ballot(&base) };
    println!("  2. clone into the base slot    -> base alive: {:?}", base.voter);

    // 3. Use a temporary nobody holds.
    let c = Ballot { voter: "Dan".to_string(), ..Default::default() };
    println!("  3. ..Default::default()        -> nothing to strand");

    println!("\n  {a:?}\n  {b:?}\n  {c:?}");

    println!("\nAnd the syntax trap, which is its own error:");
    println!("  Ballot {{ precinct: 1, ..base, }}");
    println!("    error: cannot use a comma after the base struct");
    println!("    note: the base struct must always be the last field");
}

// Ballot does not derive Clone here on purpose — this spells out that the
// "clone" in option 2 is ordinary code, not something `..` does for you.
fn clone_ballot(b: &Ballot) -> Ballot {
    Ballot {
        precinct: b.precinct,
        counted: b.counted,
        voter: b.voter.clone(),
        notes: b.notes.clone(),
    }
}
