//! Four jobs a `for` loop does better, and the one it does worse.
//!
//!   rustc --edition 2024 when_a_loop_beats_a_chain.rs -o /tmp/wlbc && /tmp/wlbc

/// Chain version: correct, and the error has lost the row number.
fn total_chain(rows: &[&str]) -> Result<i32, String> {
    rows.iter()
        .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
        .sum()
}

/// Loop version: `?` returns from THIS function, so the context is still in scope.
fn total_loop(rows: &[&str]) -> Result<i32, String> {
    let mut total = 0;
    for (n, s) in rows.iter().enumerate() {
        let value = s
            .parse::<i32>()
            .map_err(|e| format!("row {}: {e} (found {s:?})", n + 1))?;
        total += value;
    }
    Ok(total)
}

fn main() {
    let rows = ["5", "3", "no", "4"];

    println!("1. When the error needs to say WHERE");
    println!("   chain -> {:?}", total_chain(&rows));
    println!("   loop  -> {:?}", total_loop(&rows));
    println!("   Both stop at the same row. Only the loop still knows which row it");
    println!("   was, because `?` inside a closure returns from the CLOSURE — so a");
    println!("   chain that wants the index has to carry it through every stage.");
    println!("   `enumerate` plus a loop says it once.");

    println!();
    println!("2. When the work list grows while you drain it");
    let mut stack = vec![1u32];
    let mut visited = Vec::new();
    while let Some(n) = stack.pop() {
        visited.push(n);
        if n < 5 {
            stack.push(n * 2);
            stack.push(n * 2 + 1);
        }
    }
    println!("   visited {visited:?}");
    println!("   No iterator can express this: an iterator borrows the collection");
    println!("   for as long as it lives, so pushing while iterating is E0502.");
    println!("   `while let Some(x) = stack.pop()` takes one item and gives the");
    println!("   borrow straight back, which is why every worklist, BFS and");
    println!("   graph walk in Rust is a loop.");

    println!();
    println!("3. When you need out of TWO levels at once");
    let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut found = None;
    'outer: for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 5 {
                found = Some((r, c));
                break 'outer;
            }
        }
    }
    println!("   labelled break -> {found:?}");
    let by_chain = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|c| *c == 5).map(|c| (r, c)));
    println!("   find_map       -> {by_chain:?}");
    println!("   Same answer, and here the chain is arguably nicer. It stops being");
    println!("   nicer the moment the inner loop also has to update something");
    println!("   outside it — `find_map`'s closure cannot both search and mutate.");

    println!();
    println!("4. When one pass has to answer several questions");
    let scores = [5, 3, 0, 4, 2, 1];
    let mut sum = 0;
    let mut zeros = 0;
    let mut best = i32::MIN;
    for s in scores {
        sum += s;
        if s == 0 {
            zeros += 1;
        }
        if s > best {
            best = s;
        }
    }
    println!("   loop  -> sum {sum}, zeros {zeros}, best {best}");
    let (sum2, zeros2, best2) = scores.iter().fold((0, 0, i32::MIN), |(sum, zeros, best), s| {
        (sum + s, zeros + i32::from(*s == 0), best.max(*s))
    });
    println!("   fold  -> sum {sum2}, zeros {zeros2}, best {best2}");
    println!("   Identical answers. The fold is one expression and the loop is six");
    println!("   lines, and most readers get the loop faster — a tuple accumulator");
    println!("   with three fields is where the fluent style stops paying.");

    println!();
    println!("5. And the other direction, honestly");
    let names = ["Ada", "Bernadette", "Cara"];
    let shouted: Vec<String> = names.iter().filter(|n| n.len() > 3).map(|n| n.to_uppercase()).collect();
    let mut shouted_loop = Vec::new();
    for n in names {
        if n.len() > 3 {
            shouted_loop.push(n.to_uppercase());
        }
    }
    println!("   chain -> {shouted:?}");
    println!("   loop  -> {shouted_loop:?}");
    println!("   One line against five, no state to misread, and the `mut` and the");
    println!("   empty Vec never exist. When the body is transform-and-keep, the");
    println!("   chain wins outright — which is most of the time.");

    println!();
    println!("6. What is NOT a reason to pick either");
    println!("   Speed. Both compile to the same loop: the adapters are structs");
    println!("   with inlined `next` methods and the optimizer flattens them. If a");
    println!("   chain is slower it is because it allocates (an intermediate");
    println!("   collect) or clones, not because it is a chain.");
}
