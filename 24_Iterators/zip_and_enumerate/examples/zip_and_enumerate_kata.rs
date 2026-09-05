//! Kata solution: five pairings over two vectors, and the row that vanishes.
//!
//!   rustc --edition 2024 zip_and_enumerate_kata.rs -o /tmp/zaek && /tmp/zaek

fn main() {
    println!("1. Print the index and both elements");
    let ints = vec![10, 20, 30, 40];
    let floats = vec![1.1, 2.2, 3.3, 4.4];
    for (i, (n, f)) in ints.iter().zip(floats.iter()).enumerate() {
        println!("   Index: {i}, Integer: {n}, Float: {f}");
    }

    println!();
    println!("2. Concatenate into a new Vec<String>");
    let left = vec!["hello", "rust", "world"];
    let right = vec!["goodbye", "c++", "earth"];
    let joined: Vec<String> = left
        .iter()
        .zip(right.iter())
        .enumerate()
        .map(|(i, (a, b))| format!("{i}: {a} and {b}"))
        .collect();
    println!("   {joined:?}");

    println!();
    println!("3. Sum each pair");
    let a = vec![5, 10, 15];
    let b = vec![3, 6, 9];
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        // x and y are &i32. `x + y` compiles anyway: std implements Add for
        // &i32, so the references add without a deref in sight.
        println!("   Index: {i}, Sum: {}", x + y);
    }

    println!();
    println!("4. Keep the pairs whose sum is even — three ways to get past the &");
    let p = vec![2, 2, 2, 3, 4, 5];
    let q = vec![4, 8, 1, 2, 2, 2];

    // (a) deref at the push site
    let mut deref_at_push: Vec<(i32, i32)> = Vec::new();
    for (x, y) in p.iter().zip(q.iter()) {
        if (x + y) % 2 == 0 {
            deref_at_push.push((*x, *y));
        }
    }

    // (b) deref in the pattern — the & goes on the left of the =
    let mut deref_in_pattern: Vec<(i32, i32)> = Vec::new();
    for (&x, &y) in p.iter().zip(q.iter()) {
        if (x + y) % 2 == 0 {
            deref_in_pattern.push((x, y));
        }
    }

    // (c) copied() upstream, so the chain never sees a reference at all
    let copied: Vec<(i32, i32)> = p
        .iter()
        .copied()
        .zip(q.iter().copied())
        .filter(|(x, y)| (x + y) % 2 == 0)
        .collect();

    println!("   (a) deref at push     {deref_at_push:?}");
    println!("   (b) deref in pattern  {deref_in_pattern:?}");
    println!("   (c) .copied() first   {copied:?}");
    println!("   Identical. (c) is the one to reach for in a chain: filter hands");
    println!("   its closure a reference to the item, so a chain over references");
    println!("   makes the pattern &(x, y) where (b) would have written (&x, &y).");

    println!();
    println!("5. Multiply each pair by its index");
    let u = vec![2, 4, 6];
    let v = vec![3, 5, 7];
    let transformed: Vec<i32> = u
        .iter()
        .zip(v.iter())
        .enumerate()
        .map(|(i, (x, y))| i as i32 * x * y)
        .collect();
    println!("   {transformed:?}");
    println!("   `i as i32` is not decoration: enumerate counts in usize, and");
    println!("   there is no silent widening to i32. Drop the cast and what");
    println!("   rustc says depends on whether the Vec was annotated — see the");
    println!("   lesson: unannotated, it does not mention the multiply at all.");

    println!();
    println!("6. The prediction: one vector is longer than the other");
    let long = vec![2, 2, 2, 3, 4, 5, 6, 8];
    let short = vec![4, 8, 1, 2, 2, 2];
    let even: Vec<(i32, i32)> = long
        .iter()
        .copied()
        .zip(short.iter().copied())
        .filter(|(x, y)| (x + y) % 2 == 0)
        .collect();
    println!("   long has {} items, short has {}", long.len(), short.len());
    println!("   pairs examined: {}", long.iter().zip(short.iter()).count());
    println!("   kept: {even:?}");
    println!("   6 and 8 were never tested. zip ended the sequence when the");
    println!("   shorter side ran out, and the only evidence is a count nobody");
    println!("   printed. If both sides are supposed to be the same length, say");
    println!("   so before the loop:");

    if long.len() != short.len() {
        println!("   assert_eq! would fail here: {} != {}", long.len(), short.len());
    }
}
