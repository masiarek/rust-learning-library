//! Array or Vec: the differences you can measure, and the four cases where the
//! array is not merely adequate but the only one that does the job.

use std::mem::size_of;

const SCALE: usize = 6; // a STAR ballot is 0..=5 -- six buckets, forever.

fn report(counts: &[u32]) -> String {
    counts.iter().enumerate().map(|(s, n)| format!("{s}:{n}")).collect::<Vec<_>>().join(" ")
}

fn main() {
    println!("=== what is actually different ===");
    println!("  {:<34} {}", "size_of::<[u32; 5]>()", size_of::<[u32; 5]>());
    println!("  {:<34} {}   <- five values inline, no header", "  five u32 values", 5 * size_of::<u32>());
    println!("  {:<34} {}", "size_of::<Vec<u32>>() == 3 * usize", size_of::<Vec<u32>>() == 3 * size_of::<usize>());
    println!("     a Vec is pointer + length + capacity on the stack; the data is elsewhere");
    println!("  {:<34} {}", "size_of::<[u32; 5000]>()", size_of::<[u32; 5000]>());
    println!("     ...and that whole block is on the STACK when you make one");

    println!("\n=== case 1: the length is part of the type, so a wrong one cannot be passed ===");
    fn summarise(counts: [u32; SCALE]) -> u32 { counts.iter().sum() }
    let six = [4u32, 0, 2, 1, 7, 9];
    println!("  summarise([u32; 6]) = {}", summarise(six));
    println!("  summarise([u32; 5]) -> error[E0308]: mismatched types");
    println!("                         expected an array with a size of 6, found one with a size of 5");
    println!("  a Vec<u32> of the wrong length is a run-time surprise; this is a compile error");

    println!("\n=== case 2: an array of Copy types is Copy ===");
    let mut live = [4u32, 0, 2, 1, 7, 9];
    let snapshot = live;            // a real copy -- no .clone(), no move
    live[0] += 100;
    println!("  live     = {:?}", live);
    println!("  snapshot = {:?}   <- untouched, and `live` is still usable", snapshot);
    let live_v = vec![4u32, 0, 2];
    let snapshot_v = live_v.clone();  // a Vec needs this said out loud
    println!("  Vec needs .clone() for the same thing: {:?}", snapshot_v);

    println!("\n=== case 3: an array works in a const context; a Vec does not ===");
    const WEIGHTS: [u32; SCALE] = [0, 1, 2, 3, 4, 5];
    const TOP_WEIGHT: u32 = WEIGHTS[5];
    println!("  const WEIGHTS: [u32; 6]  = {:?}", WEIGHTS);
    println!("  const TOP_WEIGHT         = {}   <- indexed at compile time", TOP_WEIGHT);
    println!("  `const W: Vec<u32> = vec![..]` does not compile: allocation needs a run time");

    println!("\n=== case 4: no allocation at all ===");
    println!("  an array needs no allocator, so it is what no_std and embedded code use,");
    println!("  and what a hot loop uses to avoid touching the heap at all.");

    println!("\n=== where the Vec is simply right ===");
    let ballots = ["5,3,0", "4,4,4", "0,0,5", "2,5,1"];
    let mut counts = vec![0u32; SCALE];       // length from a value, not a literal
    for line in ballots {
        for field in line.split(',') {
            counts[field.parse::<usize>().unwrap()] += 1;
        }
    }
    println!("  parsed {} ballots -> counts = {:?}", ballots.len(), counts);
    println!("  the number of ballots is not known until the input is read, and");
    println!("  `let n = ballots.len(); let c = [0u32; n];` does not compile --");
    println!("    error[E0435]: attempt to use a non-constant value in a constant");
    println!("  an array length is a constant; `vec![0; n]` is the answer");

    println!("\n=== and the question mostly dissolves at the signature ===");
    let fixed: [u32; SCALE] = [4, 0, 2, 1, 7, 9];
    let grown: Vec<u32> = vec![4, 0, 2, 1, 7, 9];
    println!("  report(&fixed) = {}", report(&fixed));
    println!("  report(&grown) = {}", report(&grown));
    println!("  one function, `&[u32]`, and both callers work -- so the choice is about");
    println!("  how you STORE the data, not about what your functions accept");
}
