//! Match ergonomics: matching a reference against a non-reference pattern makes every
//! binding a reference, so you stop writing `ref` and stop moving things by accident.
//!
//!   rustc --edition 2024 match_ergonomics.rs -o /tmp/me && /tmp/me

fn main() {
    println!("1. A `&` on the scrutinee makes every binding a `&`");
    let ballot: Option<String> = Some("Ada".to_string());
    if let Some(name) = &ballot {
        // `name` is a &String. Nothing moved, so `ballot` is intact below.
        println!("   name: &String         -> {name}, {} bytes", name.len());
    }
    println!("   ballot survived       -> {ballot:?}");

    println!();
    println!("2. The spelling it replaced, which still works and still means this");
    if let Some(ref name) = ballot {
        println!("   Some(ref name)        -> {name}");
    }

    println!();
    println!("3. `&mut` on the scrutinee gives `&mut` bindings, so you can write through them");
    let mut tally: Option<u32> = Some(7);
    if let Some(n) = &mut tally {
        *n += 1; // n: &mut u32
    }
    println!("   after `*n += 1`       -> {tally:?}");

    println!();
    println!("4. It goes all the way down a nested pattern");
    let rows: Vec<(String, u8)> = vec![("Ada".into(), 5), ("Ben".into(), 2)];
    let mut best: &str = "";
    let mut top = 0u8;
    for (name, score) in &rows {
        // name: &String, score: &u8 — one `&` on the iterator, none in the pattern
        if *score > top {
            top = *score;
            best = name;
        }
    }
    println!("   for (name, score)     -> {best} leads with {top}");
    println!("   rows survived         -> {} still owned", rows.len());

    println!();
    println!("5. Writing `&` in the PATTERN turns it back off — that destructures");
    let scores = [3u8, 9, 4];
    let total: u32 = scores.iter().map(|&n| u32::from(n)).sum();
    println!("   |&n| copies n out     -> total {total}");

    println!();
    println!("6. The trap: a binding you think is `u8` is `&&u8` inside a closure");
    let over: Vec<&u8> = scores.iter().filter(|n| **n > 3).collect();
    let same: Vec<&u8> = scores.iter().filter(|&&n| n > 3).collect();
    println!("   filter(|n| **n > 3)   -> {over:?}");
    println!("   filter(|&&n| n > 3)   -> {same:?}   (the same list, spelled the other way)");
}
