//! Loops without an index: what `for i in 0..v.len()` risks, and the iterator
//! shapes that say what the index was for.
//!
//!   rustc --edition 2024 loops_without_an_index.rs -o /tmp/lwai && /tmp/lwai

use std::panic::{self, AssertUnwindSafe};

/// Run `f`, and return the panic message instead of printing it to stderr.
fn panic_message(f: impl FnOnce()) -> Option<String> {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(AssertUnwindSafe(f));
    panic::set_hook(hook);
    match result {
        Ok(()) => None,
        Err(payload) => Some(
            payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default(),
        ),
    }
}

fn sum_first_by_index(v: &[u64], n: usize) -> u64 {
    let mut s = 0;
    for i in 0..n {
        s += v[i];
    }
    s
}

fn sum_first_by_take(v: &[u64], n: usize) -> u64 {
    v.iter().take(n).sum()
}

fn main() {
    println!("=== 1. the same total, with and without an index ===");
    let prices = vec![250, 1200, 99, 480];
    let mut by_index = 0;
    for i in 0..prices.len() {
        by_index += prices[i];
    }
    let mut direct = 0;
    for price in &prices {
        direct += price;
    }
    println!("  for i in 0..prices.len() {{ prices[i] }} -> {by_index}");
    println!("  for price in &prices                   -> {direct}");
    for (i, price) in prices.iter().enumerate() {
        if *price > 1000 {
            println!("  and when the position IS wanted: enumerate -> index {i} holds {price}");
        }
    }

    println!();
    println!("=== 2. 0..v.len() is evaluated once, before the first turn ===");
    let mut queue = vec![1, 2, 3];
    let mut turns = 0;
    for i in 0..queue.len() {
        turns += 1;
        if queue[i] % 2 == 1 {
            queue.push(queue[i] * 10);
        }
    }
    println!("  pushing inside the loop compiles: {turns} turns, queue = {queue:?}");
    println!("  the pushed 10 and 30 were never visited -- the range was already 0..3");

    println!();
    println!("=== 3. and removing inside it skips one element, then runs off the end ===");
    let mut readings = vec![5, 0, 0, 4];
    let mut visited = Vec::new();
    let message = panic_message(|| {
        for i in 0..readings.len() {
            visited.push(i);
            if readings[i] == 0 {
                readings.remove(i);
            }
        }
    });
    println!("  start [5, 0, 0, 4], remove every 0 by index");
    println!("  visited indices {visited:?}, readings left as {readings:?}");
    println!("  panic: {}", message.unwrap_or_else(|| "none".to_string()));
    let mut kept = vec![5, 0, 0, 4];
    kept.retain(|&r| r != 0);
    println!("  retain(|&r| r != 0) -> {kept:?}");

    println!();
    println!("=== 4. an index needs the right bound, and take(n) is not the same fix ===");
    let v = [1, 2, 3];
    println!("  sum_first_by_index(&[1, 2, 3], 2) = {}", sum_first_by_index(&v, 2));
    println!("  sum_first_by_take (&[1, 2, 3], 2) = {}", sum_first_by_take(&v, 2));
    let mut by_index_5 = 0;
    let message = panic_message(|| by_index_5 = sum_first_by_index(&v, 5));
    println!(
        "  sum_first_by_index(&[1, 2, 3], 5) -> panic: {}",
        message.unwrap_or_else(|| format!("no panic, {by_index_5}"))
    );
    println!("  sum_first_by_take (&[1, 2, 3], 5) = {}   <- stops at the end, no panic", sum_first_by_take(&v, 5));

    println!();
    println!("=== 5. what the index was for, and the shape that says so ===");
    let temps = [12, 15, 11, 18, 20, 19];
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    println!("  temps = {temps:?}");

    let (warmest_at, warmest) = temps.iter().enumerate().max_by_key(|&(_, t)| t).unwrap();
    println!("  position     enumerate    warmest is index {warmest_at}, {warmest} degrees");

    let mut rises_by_index = Vec::new();
    for i in 1..temps.len() {
        rises_by_index.push(temps[i] - temps[i - 1]);
    }
    let rises: Vec<i32> = temps.windows(2).map(|w| w[1] - w[0]).collect();
    println!("  neighbours   windows(2)   {rises:?}   same as temps[i] - temps[i - 1]: {}", rises == rises_by_index);

    let mut pairs_by_index = Vec::new();
    for i in (0..temps.len()).step_by(2) {
        pairs_by_index.push(temps[i] + temps[i + 1]);
    }
    let pairs: Vec<i32> = temps.chunks(2).map(|c| c.iter().sum()).collect();
    println!("  groups       chunks(2)    {pairs:?}          same as temps[i] + temps[i + 1]: {}", pairs == pairs_by_index);
    let odd = [1, 2, 3];
    let odd_chunks: Vec<i32> = odd.chunks(2).map(|c| c.iter().sum()).collect();
    let mut odd_by_index = Vec::new();
    let message = panic_message(|| {
        for i in (0..odd.len()).step_by(2) {
            odd_by_index.push(odd[i] + odd[i + 1]);
        }
    });
    println!("    on [1, 2, 3]: chunks(2) -> {odd_chunks:?} (a short last chunk); the index form -> panic: {}", message.unwrap_or_default());

    let every_other: Vec<i32> = temps.iter().step_by(2).copied().collect();
    println!("  every k-th   step_by(2)   {every_other:?}");

    let backwards: Vec<i32> = temps.iter().rev().copied().collect();
    println!("  backwards    iter().rev() {backwards:?}");

    let mut labelled = Vec::new();
    for (day, t) in days.iter().zip(&temps) {
        if *t >= 18 {
            labelled.push(format!("{day} {t}"));
        }
    }
    println!("  two in step  zip          {labelled:?}");

    println!();
    println!("=== 6. where an index is honest: two positions at once ===");
    let mut sorted = temps;
    for i in 0..sorted.len() {
        for j in i + 1..sorted.len() {
            if sorted[j] < sorted[i] {
                sorted.swap(i, j);
            }
        }
    }
    println!("  swap(i, j) inside two index loops -> {sorted:?}");
    println!("  an iterator hands out one element at a time; swap needs two places in one call");
}
