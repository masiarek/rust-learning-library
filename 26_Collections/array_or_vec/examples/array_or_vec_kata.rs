//! Kata solution: one election has two lengths, and only one of them is a fact
//! about the problem rather than about this run.

const SCALE: usize = 6; // 0..=5 -- decided by the ballot design, not by the input

fn print_histogram(counts: &[u32]) -> String {
    counts.iter().enumerate().map(|(s, n)| format!("{s}={n}")).collect::<Vec<_>>().join("  ")
}

fn main() {
    println!("=== the two lengths ===");
    println!("  the score scale is 0..=5 in every STAR election ever held  -> [u32; {SCALE}]");
    println!("  the candidate list is different every time                 -> Vec<String>");

    let candidates: Vec<String> = ["Ada", "Ben", "Cara"].iter().map(|s| s.to_string()).collect();
    let ballots = ["5,3,0", "4,4,4", "0,0,5", "2,5,1"];
    println!("  this election: {} candidates, {} ballots", candidates.len(), ballots.len());

    println!("\n=== count into the fixed-length one ===");
    let mut histogram = [0u32; SCALE];
    for line in ballots {
        for field in line.split(',') {
            histogram[field.parse::<usize>().unwrap()] += 1;
        }
    }
    println!("  histogram = {}", print_histogram(&histogram));

    println!("\n=== consequence 1: the array is Copy, so a snapshot is free ===");
    let before = histogram;          // no .clone(), no move
    histogram[5] += 1000;            // the original goes on changing
    println!("  before    = {}", print_histogram(&before));
    println!("  after     = {}", print_histogram(&histogram));
    println!("  the same two lines over a Vec do not compile. rustc opens with");
    println!("    error[E0382]: borrow of moved value: `live`");
    println!("  and closes with");
    println!("    help: consider cloning the value if the performance cost is acceptable");
    println!("  between them it says the move happened because Vec<u32> is not Copy.");
    println!("  So the fix is `let before = live.clone();` -- a heap allocation you");
    println!("  now know you made, which is the difference the two types are for.");
    histogram[5] -= 1000;

    println!("\n=== consequence 2: a wrong length is a compile error, not a bad report ===");
    fn total(counts: [u32; SCALE]) -> u32 { counts.iter().sum() }
    println!("  total(histogram) = {}", total(histogram));
    println!("  total([0u32; 5]) -> error[E0308]: mismatched types");
    println!("                      expected an array with a size of 6, found one with a size of 5");
    println!("  a Vec of the wrong length would have produced a report that was merely wrong");

    println!("\n=== consequence 3: a run-time length cannot be an array length ===");
    let n = candidates.len();
    let mut per_candidate = vec![0u32; n];   // the only thing that works here
    for line in ballots {
        for (i, field) in line.split(',').enumerate() {
            per_candidate[i] += field.parse::<u32>().unwrap();
        }
    }
    for (name, total) in candidates.iter().zip(&per_candidate) {
        println!("  {name:<5} {total}");
    }
    println!("  `let c = [0u32; n];` -> error[E0435]: attempt to use a non-constant value in a constant");
    println!("  an array length is a constant; n came from the input, so it is vec![0; n]");

    println!("\n=== and both end up at the same signature ===");
    println!("  print_histogram(&histogram)     -> {}", print_histogram(&histogram));
    println!("  print_histogram(&per_candidate) -> {}", print_histogram(&per_candidate));
    println!("  one fn taking &[u32], two callers -- the decision was about storage, not API");
}
