//! Kata solution: swap, ignore, nest, and the arity where it breaks down.
//!
//!   rustc --edition 2024 tuples_kata.rs -o /tmp/tk && /tmp/tk

/// A ballot: voter, candidate, score, and whether it arrived by post.
type BallotTuple = (&'static str, &'static str, u8, bool);

#[derive(Debug)]
struct Ballot {
    voter: &'static str,
    candidate: &'static str,
    score: u8,
    postal: bool,
}

fn totals_tuple(rows: &[BallotTuple]) -> (u32, u32) {
    let mut postal = 0;
    let mut walk_in = 0;
    for &(_, _, score, is_postal) in rows {
        if is_postal {
            postal += u32::from(score);
        } else {
            walk_in += u32::from(score);
        }
    }
    (postal, walk_in)
}

fn totals_struct(rows: &[Ballot]) -> (u32, u32) {
    let mut postal = 0;
    let mut walk_in = 0;
    for b in rows {
        if b.postal {
            postal += u32::from(b.score);
        } else {
            walk_in += u32::from(b.score);
        }
    }
    (postal, walk_in)
}

fn main() {
    println!("1. Swap without a temporary");
    let (mut a, mut b) = (1, 2);
    (a, b) = (b, a);
    println!("   after (a, b) = (b, a): a = {a}, b = {b}");
    println!("   The right-hand tuple is built first, so no third name is needed.");

    println!();
    println!("2. `_` in a pattern discards without binding");
    let row: BallotTuple = ("Ada", "Cara", 5, true);
    let (voter, _, score, _) = row;
    println!("   let (voter, _, score, _) = row;  ->  {voter} scored {score}");
    println!("   An unused binding warns; an `_` does not, because it says so.");

    println!();
    println!("3. Nesting, and destructuring through it");
    let by_round = ((1, "Ada"), (2, "Ben"));
    let ((r1, w1), (r2, w2)) = by_round;
    println!("   round {r1}: {w1}, round {r2}: {w2}");
    println!("   by_round.1.0 = {} — legal, and the reason this is the last", by_round.1.0);
    println!("   arity anyone should write.");

    println!();
    println!("4. The four-field tuple, and the same data named");
    let tuple_rows: [BallotTuple; 4] = [
        ("Ada", "Cara", 5, true),
        ("Ben", "Cara", 3, false),
        ("Cara", "Ada", 4, true),
        ("Dan", "Ada", 2, false),
    ];
    let struct_rows: Vec<Ballot> = tuple_rows
        .iter()
        .map(|&(voter, candidate, score, postal)| Ballot { voter, candidate, score, postal })
        .collect();

    println!("   first row, named: {} -> {}", struct_rows[0].voter, struct_rows[0].candidate);
    let (p1, w1) = totals_tuple(&tuple_rows);
    let (p2, w2) = totals_struct(&struct_rows);
    println!("   totals_tuple  -> postal {p1}, walk-in {w1}");
    println!("   totals_struct -> postal {p2}, walk-in {w2}");
    println!("   Same answer. The difference is in the two function bodies:");
    println!("     for (_, _, score, is_postal) in rows       <- position");
    println!("     for b in rows: b.score, b.postal            <- name");
    println!("   Swap `score` and `postal` in the tuple's TYPE and the first");
    println!("   body still compiles if the types happen to line up. Here they");
    println!("   do not (u8 vs bool), so this one is caught — which is luck,");
    println!("   not design: two u8 fields swapped compile fine and count wrong.");
    println!("   Field names cannot be transposed by accident.");

    println!();
    println!("5. What the struct also bought");
    println!("   {:?}", struct_rows[0]);
    println!("   `#[derive(Debug)]` prints the field names. A tuple prints");
    println!("   (\"Ada\", \"Cara\", 5, true) and leaves the reader to guess.");
}
