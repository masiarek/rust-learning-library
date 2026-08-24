//! `Copy` vs `Clone`: one changes what `=` MEANS, the other adds a method.
//!
//!   rustc --edition 2024 copy_vs_clone.rs -o /tmp/cvc && /tmp/cvc

// Clone only. Duplicating is possible, but you must ask for it by name.
#[derive(Debug, Clone)]
struct Ballot {
    voter: String,   // String is not Copy, so Ballot can never be Copy
    scores: Vec<u8>,
}

// Copy AND Clone. Every field is Copy, and we opted in.
#[derive(Debug, Clone, Copy)]
struct Precinct {
    id: u32,
    registered: u32,
}

// All-Copy fields, but NOT opted in. This is the case people trip over.
#[derive(Debug, Clone)]
struct Tally {
    counted: u32,
}

fn consume_ballot(b: Ballot) -> String { format!("{} cast {}", b.voter, b.scores.len()) }
fn consume_precinct(p: Precinct) -> u32 { p.id + p.registered }
fn consume_tally(t: Tally) -> u32 { t.counted }

fn main() {
    println!("1. The difference is what `let b = a;` MEANS");
    let p = Precinct { id: 7, registered: 431 };
    let _also_p = p; //          Copy   -> p is COPIED, and p is still alive
    println!("   Precinct is Copy: after `let _also = p;`, p is fine -> {p:?}");

    let b = Ballot { voter: "Ada".to_string(), scores: vec![5, 2, 0] };
    let moved = b; //            not Copy -> b is MOVED, and b is now dead
    println!("   Ballot is not:   after `let moved = b;`, `b` is E0382");
    println!("                    the value lives on as `moved` -> {moved:?}");
    println!("   Same syntax. Different meaning. The TYPE decides.");

    println!("\n2. Passing to a function is the same question");
    println!("   consume_precinct(p) = {} (id+registered), and p survives -> {p:?}",
             consume_precinct(p));
    println!("   consume_ballot(moved) = {:?} and `moved` does not survive",
             consume_ballot(moved.clone()));
    println!("   ...which is why `.clone()` is in that line at all.");

    println!("\n3. All-Copy fields is NOT enough — you have to opt in");
    let t = Tally { counted: 12 };
    println!("   Tally holds one u32 and still is not Copy, because it does");
    println!("   not derive Copy. consume_tally(t) MOVES it:");
    println!("   consume_tally(t) = {}", consume_tally(t));
    println!("   `t` is now dead. Opting in is deliberate: making a type Copy");
    println!("   is a promise to your callers that you cannot quietly take back.");

    println!("\n4. Clone is a method you call; Copy is something the compiler does");
    let original = Ballot { voter: "Ben".to_string(), scores: vec![4] };
    let duplicate = original.clone(); // explicit, and it allocates
    println!("   original  {original:?}");
    println!("   duplicate {duplicate:?}   <- a second heap allocation, on purpose");
    println!("   `Copy` never allocates: it is a bit-for-bit copy, nothing else.");

    println!("\n5. The three refusals, each with its own code");
    println!("   impl Copy for P {{}} without Clone");
    println!("     error[E0277]: the trait bound `P: Clone` is not satisfied");
    println!("     -> `Copy` requires `Clone`. Always derive both together.");
    println!("   #[derive(Copy)] on a struct holding a String");
    println!("     error[E0204]: the trait `Copy` cannot be implemented for this type");
    println!("       this field does not implement `Copy`");
    println!("   #[derive(Copy)] on a struct that also impls Drop");
    println!("     error[E0184]: `Copy` not allowed on types with destructors");
    println!("     -> a destructor runs once per value; copies would run it twice.");
}
