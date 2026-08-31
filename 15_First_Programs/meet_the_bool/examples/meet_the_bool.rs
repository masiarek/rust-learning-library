//! `bool` is one byte holding one bit of information, and it is not a number.
//! Everything printed here is a consequence of those two facts.

use std::mem::size_of;

fn main() {
    println!("=== two values, one byte ===");
    println!("  size_of::<bool>()         = {}   <- one bit of information, one byte of space", size_of::<bool>());
    println!("  size_of::<Option<bool>>() = {}   <- None hides in one of the 254 unused patterns", size_of::<Option<bool>>());
    println!("  size_of::<[bool; 8]>()    = {}   <- eight bools are eight BYTES, not one", size_of::<[bool; 8]>());
    println!("  true as u8  = {}", true as u8);
    println!("  false as u8 = {}", false as u8);

    println!("\n=== bool is not a number, so you say the conversion out loud ===");
    let approvals = [true, false, true, true, false];
    println!("  approvals            = {:?}", approvals);
    let by_filter = approvals.iter().filter(|&&b| b).count();
    let by_sum: u32 = approvals.iter().map(|&b| b as u32).sum();
    let by_from: u32 = approvals.iter().copied().map(u32::from).sum();
    println!("  filter(..).count()   = {}", by_filter);
    println!("  map(|b| b as u32).sum() = {}", by_sum);
    println!("  map(u32::from).sum()    = {}   <- the same cast, spelled as a conversion", by_from);

    println!("\n=== false < true, so bools sort and max ===");
    let mut flags = [true, false, true, false];
    flags.sort();
    println!("  sorted               = {:?}   <- false first", flags);
    println!("  false < true         = {}", false < true);
    println!("  approvals.iter().any(|&b| b) = {}", approvals.iter().any(|&b| b));
    println!("  approvals.iter().all(|&b| b) = {}", approvals.iter().all(|&b| b));

    println!("\n=== && short-circuits; & does NOT, and both compile on bool ===");
    fn expensive(log: &mut Vec<&'static str>) -> bool {
        log.push("expensive() ran");
        true
    }
    let mut log = Vec::new();
    let _ = false && expensive(&mut log);
    println!("  false && expensive()  -> log {:?}", log);
    let mut log = Vec::new();
    let _ = false & expensive(&mut log);
    println!("  false &  expensive()  -> log {:?}   <- both sides always evaluated", log);
    println!("  the values agree; only the work differs: {} vs {}", false && true, false & true);

    println!("\n=== a bool can hand you a value without an if ===");
    let quorum_met = true;
    println!("  true.then(|| \"counted\")   = {:?}", quorum_met.then(|| "counted"));
    println!("  false.then(|| \"counted\")  = {:?}", false.then(|| "counted"));
    println!("  true.then_some(42)        = {:?}   <- no closure when the value is ready", true.then_some(42));
    println!("  false.then_some(42)       = {:?}", false.then_some(42));

    println!("\n=== parsing text into one ===");
    for text in ["true", "false", "TRUE", "1", "yes"] {
        match text.parse::<bool>() {
            Ok(b) => println!("  {:>7}.parse::<bool>() = Ok({b})", format!("{text:?}")),
            Err(e) => println!("  {:>7}.parse::<bool>() = Err({e})", format!("{text:?}")),
        }
    }
    println!("  only the exact lowercase words parse; there is no truthiness here either");
}
