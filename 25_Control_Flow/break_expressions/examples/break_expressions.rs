//! `break` leaves the innermost enclosing loop, and nothing further out.
//!
//!   rustc --edition 2024 break_expressions.rs -o /tmp/brk && /tmp/brk

use std::ops::ControlFlow;

fn main() {
    println!("1. break leaves the innermost loop only");
    let grid = [[1, -2, 3], [4, 5, 6], [-7, 8, 9]];
    let mut found = None;
    let mut assignments = 0;
    let mut rows_visited = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        rows_visited.push(r);
        for (c, &n) in row.iter().enumerate() {
            if n < 0 {
                found = Some((r, c));
                assignments += 1;
                break; // leaves the column loop; the row loop carries on
            }
        }
    }
    println!("   rows visited: {rows_visited:?}");
    println!("   found = {found:?}, assigned {assignments} times");
    println!("   The first negative is at (0, 1). The row loop kept going after");
    println!("   the break, met -7 on row 2, and overwrote the answer.");

    let mut first = None;
    'rows: for (r, row) in grid.iter().enumerate() {
        for (c, &n) in row.iter().enumerate() {
            if n < 0 {
                first = Some((r, c));
                break 'rows;
            }
        }
    }
    println!("   with break 'rows: found = {first:?}");

    println!();
    println!("2. The book's endless zip, ended by break");
    let mut passes = 0;
    let mut stopped_at = None;
    for (x, y) in (0..).zip(0..) {
        passes += 1;
        if x + y > 100 {
            stopped_at = Some((x, y));
            break;
        }
    }
    println!("   stopped at (x, y) = {stopped_at:?} on pass {passes}");
    println!("   50 + 50 = 100 is not > 100, so (51, 51) is the first pair that is.");

    println!();
    println!("3. break in a match arm leaves the loop, and has type !");
    let tokens = ["3", "4", "x", "5"];
    let mut sum = 0;
    let mut read = Vec::new();
    for t in tokens {
        read.push(t);
        let n: i32 = match t.parse() {
            Ok(n) => n,
            Err(_) => break, // type !, so it fits in an arm that must be i32
        };
        sum += n;
    }
    println!("   read {read:?}, sum = {sum}");
    println!("   \"5\" was never read: the break left the for loop, not the match.");

    println!();
    println!("4. break with a value: loop and labelled blocks only");
    let mut n = 0;
    let first_square_over_20 = loop {
        n += 1;
        if n * n > 20 {
            break n;
        }
    };
    println!("   loop {{ .. break n; }}              -> {first_square_over_20}");
    let size = 'size: {
        if first_square_over_20 > 3 {
            break 'size "big";
        }
        "small"
    };
    println!("   'size: {{ .. break 'size \"big\"; }}  -> {size:?}");
    let bare = loop {
        break;
    };
    println!("   loop {{ break; }}                   -> {bare:?}   <- a bare break is break ()");
    let from_range = (1..).find(|n| n * n > 20);
    println!("   (1..).find(|n| n * n > 20)        -> {from_range:?}   <- the for-shaped answer is an Option");

    println!();
    println!("5. Stop at the first match as a value: position, any, find");
    let temps = [12, 15, 31, 18, 33];
    let mut index = None;
    let mut loop_passes = 0;
    for (i, &t) in temps.iter().enumerate() {
        loop_passes += 1;
        if t > 30 {
            index = Some(i);
            break;
        }
    }
    println!("   for + break            -> {index:?} after {loop_passes} passes");

    let mut calls = 0;
    let pos = temps.iter().position(|&t| {
        calls += 1;
        t > 30
    });
    println!("   position(t > 30)       -> {pos:?} after {calls} calls");

    let mut calls = 0;
    let any_hot = temps.iter().any(|&t| {
        calls += 1;
        t > 30
    });
    println!("   any(t > 30)            -> {any_hot} after {calls} calls");

    let mut calls = 0;
    let first_hot = temps.iter().find(|&&t| {
        calls += 1;
        t > 30
    });
    println!("   find(t > 30)           -> {first_hot:?} after {calls} calls");

    let mut calls = 0;
    let hot_count = temps
        .iter()
        .filter(|&&t| {
            calls += 1;
            t > 30
        })
        .count();
    println!("   filter(t > 30).count() -> {hot_count} after {calls} calls   <- counting has to see all five");

    println!();
    println!("6. A closure cannot break; try_for_each stops on ControlFlow::Break");
    let words = ["alpha", "beta", "stop", "gamma"];
    let mut seen = Vec::new();
    let result = words.iter().try_for_each(|&w| {
        if w == "stop" {
            return ControlFlow::Break(w);
        }
        seen.push(w);
        ControlFlow::Continue(())
    });
    println!("   seen = {seen:?}");
    println!("   result = {result:?}");
}
