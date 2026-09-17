//! `for` loops: ranges and collections are one mechanism, because `for` only
//! ever calls `IntoIterator::into_iter` on what you hand it.
//!
//!   rustc --edition 2024 for_loops.rs -o /tmp/for_loops && /tmp/for_loops

use std::any::type_name;
use std::collections::BTreeMap;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("=== 1. n..m stops before m; n..=m includes it ===");
    let exclusive: Vec<i32> = (1..5).collect();
    let inclusive: Vec<i32> = (1..=5).collect();
    println!("  1..5               -> {exclusive:?}      {} turns, the last one is 4", exclusive.len());
    println!("  1..=5              -> {inclusive:?}   {} turns", inclusive.len());
    let mut turns = 0;
    for _ in 0..3 {
        turns += 1;
    }
    println!("  for _ in 0..3      -> {turns} turns, and no loop variable to name");
    let mut backwards = 0;
    for _ in 5..1 {
        backwards += 1;
    }
    println!("  for _ in 5..1      -> {backwards} turns  <- empty, not reversed; rustc says nothing");
    let reversed: Vec<i32> = (1..5).rev().collect();
    println!("  (1..5).rev()       -> {reversed:?}");

    println!();
    println!("=== 2. a range is a value, and for consumes it ===");
    let r = 1..5;
    println!("  let r = 1..5;      type {}", type_of(&r));
    println!("  r.contains(&5)     = {}", r.contains(&5));
    let mut walked = Vec::new();
    for i in r.clone() {
        walked.push(i);
    }
    println!("  for i in r.clone() -> {walked:?}   (Range is not Copy; a second `for i in r` is E0382)");
    let mut rest = 1..10;
    for i in &mut rest {
        if i == 3 {
            break;
        }
    }
    println!("  for i in &mut rest, break at 3 -> rest is now {rest:?}");

    println!();
    println!("=== 3. arrays, Vecs and slices go through the same door ===");
    let array = [2, 4, 8];
    let vector = vec![2, 4, 8];
    let mut seen = Vec::new();
    for elem in array {
        seen.push(elem);
    }
    println!("  for elem in array        -> {seen:?}");
    seen.clear();
    for elem in &vector {
        seen.push(*elem);
    }
    println!("  for elem in &vector      -> {seen:?}");
    seen.clear();
    for elem in &vector[1..] {
        seen.push(*elem);
    }
    println!("  for elem in &vector[1..] -> {seen:?}");

    println!();
    println!("=== 4. what you hand to for decides what each item is ===");
    let mut prices = vec![250, 1200, 99];
    let mut item = "";
    let mut total = 0;
    for p in &prices {
        item = type_of(&p);
        total += p;
    }
    println!("  for p in &prices      item: {item:<23} read  -> total = {total}, prices still {prices:?}");
    for p in &mut prices {
        item = type_of(&p);
        *p += 1;
    }
    println!("  for p in &mut prices  item: {item:<23} write -> *p += 1, prices now {prices:?}");
    let names = vec![String::from("Ada"), String::from("Ben")];
    let mut kept = Vec::new();
    for name in names {
        item = type_of(&name);
        kept.push(name);
    }
    println!("  for name in names     item: {item:<23} owned -> kept = {kept:?}, names is gone");

    println!();
    println!("=== 5. the move happens as the loop starts, and the name survives it ===");
    let small = [1, 2, 3];
    let mut sum = 0;
    for x in small {
        sum += x;
    }
    println!("  [i32; 3] is Copy: after `for x in small`, sum = {sum}, small = {small:?}");
    let mut v = vec![1, 2, 3];
    let mut count = 0;
    for _ in v {
        count += 1;
    }
    v = vec![9];
    println!("  Vec is not Copy: `for _ in v` ran {count} turns, then `v = vec![9]` -> v = {v:?}");

    println!();
    println!("=== 6. `for x in &c` calls IntoIterator on the REFERENCE, whatever that does ===");
    let words = vec!["alpha", "beta"];
    println!("  (&words).into_iter()  {}", type_of(&(&words).into_iter()));
    println!("  words.iter()          {}", type_of(&words.iter()));
    let mut stock = BTreeMap::new();
    stock.insert("apples", 3);
    stock.insert("pears", 0);
    for (fruit, n) in &stock {
        println!("  for (fruit, n) in &stock -> {fruit} = {n}   fruit: {}, n: {}", type_of(&fruit), type_of(&n));
    }
    let s = String::from("héllo");
    let chars: Vec<char> = s.chars().collect();
    println!("  for c in &s is E0277 (&String is not an iterator); s.chars() -> {chars:?}");
    println!("  s.bytes().count() = {}, s.chars().count() = {}", s.bytes().count(), s.chars().count());

    println!();
    println!("=== 7. enumerate, not a counter kept by hand ===");
    for (i, w) in words.iter().enumerate() {
        println!("  {i}: {w}");
    }

    println!();
    println!("=== 8. a for loop evaluates to (), so it cannot hand a value out ===");
    let unit = for _ in 0..0 {};
    println!("  let unit = for _ in 0..0 {{}};  unit = {unit:?}");
    let first_big = vector.iter().find(|&&x| x > 3);
    println!("  first element > 3, found without a loop: {first_big:?}");

    println!();
    println!("=== 9. many for loops are a chain in disguise ===");
    let mut even_squares = 0;
    for n in 1..=10 {
        if n % 2 == 0 {
            even_squares += n * n;
        }
    }
    let chained: i32 = (1..=10).filter(|n| n % 2 == 0).map(|n| n * n).sum();
    println!("  loop:  sum of even squares in 1..=10 = {even_squares}");
    println!("  chain: (1..=10).filter(even).map(square).sum() = {chained}");
    assert_eq!(even_squares, chained);

    println!();
    println!("=== 10. the loop variable is the item, not the loop's counter ===");
    let mut printed = Vec::new();
    for mut i in 0..5 {
        if i == 1 {
            i += 2; // C's `i += 2` skips ahead; this changes one copy and nothing else
        }
        printed.push(i);
    }
    println!("  for mut i in 0..5, i += 2 when i == 1 -> {printed:?}   <- 2 still arrives");
    let skipped: Vec<i32> = (0..5).filter(|&i| i != 1 && i != 2).collect();
    println!("  to skip, say so to the iterator: filter  -> {skipped:?}");
    let stride: Vec<i32> = (0..10).step_by(3).collect();
    println!("  a fixed stride (C's i += 3) is step_by: (0..10).step_by(3) -> {stride:?}");
}
