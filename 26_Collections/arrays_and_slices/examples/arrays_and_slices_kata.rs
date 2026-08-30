//! Kata solution: one function, four callers, and the signature that rejects three.
//!
//!   rustc --edition 2024 arrays_and_slices_kata.rs -o /tmp/aask && /tmp/aask

/// The signature that only ever serves one caller.
fn average_fixed(scores: &[u32; 5]) -> f64 {
    f64::from(scores.iter().sum::<u32>()) / 5.0
}

/// The signature to write instead.
fn average(scores: &[u32]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }
    Some(f64::from(scores.iter().sum::<u32>()) / scores.len() as f64)
}

/// Runs of equal values, without allocating a Vec per run.
fn longest_run(scores: &[u32]) -> usize {
    let mut best = 0;
    let mut current = 0;
    let mut previous: Option<u32> = None;
    for &s in scores {
        current = if Some(s) == previous { current + 1 } else { 1 };
        previous = Some(s);
        best = best.max(current);
    }
    best
}

fn main() {
    let ballot: [u32; 5] = [5, 3, 3, 3, 2];
    let short = [4u32, 4];
    let owned: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    let empty: [u32; 0] = [];

    println!("1. The fixed-length signature, and who it turns away");
    println!("   average_fixed(&ballot) = {:.2}", average_fixed(&ballot));
    println!("   average_fixed(&short)   does not compile:");
    println!("     expected `&[u32; 5]`, found `&[u32; 2]`   [E0308]");
    println!("   average_fixed(&owned)   does not compile either — a Vec is not");
    println!("     an array, however many elements it happens to hold.");

    println!();
    println!("2. The slice signature, and the same four callers");
    println!("   average(&ballot)     = {:?}", average(&ballot).map(|a| (a * 100.0).round() / 100.0));
    println!("   average(&short)      = {:?}", average(&short));
    println!("   average(&owned)      = {:?}", average(&owned));
    println!("   average(&ballot[1..]) = {:?}", average(&ballot[1..]).map(|a| (a * 100.0).round() / 100.0));
    println!("   average(&empty)      = {:?}   <- the length can be zero, so the", average(&empty));
    println!("   function has to say what it does about that. `&[u32; 5]` never");
    println!("   had to, which is the one thing it bought.");

    println!();
    println!("3. `len()` is a run-time value, so the empty case is a real case");
    println!("   The fixed version divides by the 5 in its own type and cannot");
    println!("   be handed nothing. The slice version divides by len(), so 0/0");
    println!("   is reachable: in floating point that is NaN, silently.");
    let bad = f64::from(empty.iter().sum::<u32>()) / empty.len() as f64;
    println!("   without the guard: {bad}   <- prints, compares false to itself");
    println!("   with the guard:    {:?}", average(&empty));

    println!();
    println!("4. What the slice methods give you for free");
    println!("   longest_run({ballot:?}) = {}", longest_run(&ballot));
    println!("   same, via windows(2): {}", 1 + ballot.windows(2).filter(|w| w[0] == w[1]).count());
    println!("   (that shortcut is only right because this array has ONE run of");
    println!("   repeats — count adjacent equal pairs and you have counted every");
    println!("   run at once, not the longest. windows is a tool, not an answer.)");
    println!("   ballot.split_at(2) = {:?}", ballot.split_at(2));
    println!("   ballot.iter().rev().collect::<Vec<_>>() = {:?}", ballot.iter().rev().collect::<Vec<_>>());
}
