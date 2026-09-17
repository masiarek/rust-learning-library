//! `continue` ends this pass early. What runs next depends on the loop.
//!
//!   rustc --edition 2024 continue_expressions.rs -o /tmp/cont && /tmp/cont

fn main() {
    println!("1. continue skips the rest of this pass; for still advances");
    let mut visited = Vec::new();
    let mut sum = 0;
    for n in 0..10 {
        visited.push(n);
        if n % 2 == 0 {
            continue;
        }
        sum += n;
    }
    println!("   visited {visited:?}");
    println!("   sum of the odd ones = {sum}");
    println!("   Every n was visited: the range hands out the next number");
    println!("   whether or not the body finished.");

    println!();
    println!("2. while: a continue above the increment never reaches it");
    let readings = [4, 7, -1, 5, 6]; // -1 is a sensor error, to be skipped
    let mut i = 0;
    let mut total = 0;
    let mut passes = 0;
    let mut i_at = Vec::new();
    while i < readings.len() {
        passes += 1;
        if [3, 10, 20].contains(&passes) {
            i_at.push((passes, i));
        }
        if passes == 20 {
            println!("   stopped by a 20-pass safety cap, not by the loop condition");
            break;
        }
        if readings[i] < 0 {
            continue; // skips `i += 1` below, so i never moves again
        }
        total += readings[i];
        i += 1;
    }
    println!("   (pass, i): {i_at:?}");
    println!("   total = {total}   <- 5 and 6 were never added");

    let mut i = 0;
    let mut total = 0;
    while i < readings.len() {
        let r = readings[i];
        i += 1; // advance first, so nothing below can skip it
        if r < 0 {
            continue;
        }
        total += r;
    }
    println!("   increment moved above the continue: total = {total}");

    let mut total = 0;
    for &r in &readings {
        if r < 0 {
            continue;
        }
        total += r;
    }
    println!("   the same loop as a for:            total = {total}");

    println!();
    println!("3. continue in loop jumps back to the top; let-else makes the test");
    let lines = ["# settings", "", "width = 80", "oops", "height = 24"];
    let mut it = lines.iter();
    let mut settings = Vec::new();
    loop {
        let Some(line) = it.next() else { break };
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(" = ") else { continue };
        settings.push((key, value));
    }
    println!("   {settings:?}");

    println!();
    println!("4. continue has type !, so it fits in a match arm");
    let raw = ["12", "x", "30", "", "7"];
    let mut kept = Vec::new();
    for s in raw {
        let n: u32 = match s.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        kept.push(n);
    }
    println!("   {raw:?} -> {kept:?}");

    println!();
    println!("5. filter is `if not wanted {{ continue; }}` as an adapter");
    let odd_sum: i32 = (0..10).filter(|n| n % 2 != 0).sum();
    let good_total: i32 = readings.iter().filter(|&&r| r >= 0).sum();
    let parsed: Vec<u32> = raw.iter().filter_map(|s| s.parse().ok()).collect();
    println!("   (0..10).filter(odd).sum()          = {odd_sum}");
    println!("   readings.filter(r >= 0).sum()      = {good_total}");
    println!("   raw.filter_map(parse.ok()).collect = {parsed:?}");

    println!();
    println!("6. In for_each, return is the continue; in a for body it leaves the function");
    let xs = [1, 3, 4, 5];
    let mut odd = Vec::new();
    xs.iter().for_each(|&n| {
        if n % 2 == 0 {
            return; // ends this call of the closure; for_each calls it again for 5
        }
        odd.push(n);
    });
    println!("   for_each with return: {odd:?}");
    println!("   for with return:      {:?}", odd_until_even(&xs));
}

fn odd_until_even(xs: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    for &n in xs {
        if n % 2 == 0 {
            return out; // leaves odd_until_even, not just this pass
        }
        out.push(n);
    }
    out
}
