//! fold and reduce: the consumer every other consumer is a special case of.
//!
//!   rustc --edition 2024 fold_and_reduce.rs -o /tmp/far && /tmp/far

use std::cell::Cell;
use std::collections::HashMap;

const SCORES: [i32; 6] = [5, 3, 0, 4, 2, 1];

fn main() {
    println!("1. fold takes a starting value and a way to absorb one more item");
    let total = SCORES.iter().fold(0, |acc, s| acc + s);
    println!("   .fold(0, |acc, s| acc + s)   = {total}");
    println!("   .sum::<i32>()                = {}", SCORES.iter().sum::<i32>());
    println!("   same answer, and `sum` IS this fold — std writes it as one.");

    println!();
    println!("2. Which is why the others are folds too");
    let count = SCORES.iter().fold(0usize, |acc, _| acc + 1);
    let max = SCORES.iter().fold(i32::MIN, |acc, s| if *s > acc { *s } else { acc });
    let collected = SCORES.iter().fold(Vec::new(), |mut acc, s| {
        acc.push(*s);
        acc
    });
    println!("   count -> {count}   (std: {})", SCORES.iter().count());
    println!("   max   -> {max}   (std: {:?})", SCORES.iter().max());
    println!("   collect -> {collected:?}");
    println!("   Reach for the named one every time — this is what it is made of,");
    println!("   not a suggestion to write it yourself.");

    println!();
    println!("3. The accumulator does NOT have to be the item type");
    let line = SCORES.iter().fold(String::new(), |mut acc, s| {
        if !acc.is_empty() {
            acc.push('-');
        }
        acc.push_str(&s.to_string());
        acc
    });
    println!("   six i32 -> one String   {line:?}");

    let tally: HashMap<&str, i32> = [("Ada", 5), ("Ben", 3), ("Ada", 4)]
        .into_iter()
        .fold(HashMap::new(), |mut acc, (name, score)| {
            *acc.entry(name).or_insert(0) += score;
            acc
        });
    let mut rows: Vec<_> = tally.iter().collect();
    rows.sort();
    println!("   three pairs -> a tally  {rows:?}");
    println!("   That is the whole reason fold exists: `sum` can only build a number.");

    println!();
    println!("4. Two answers in one pass, with a tuple accumulator");
    let (lo, hi) = SCORES
        .iter()
        .fold((i32::MAX, i32::MIN), |(lo, hi), s| (lo.min(*s), hi.max(*s)));
    println!("   min and max together    ({lo}, {hi})");
    println!("   .min() then .max() walks the data twice; this walks it once.");

    println!();
    println!("5. reduce is fold with the first item as the starting value");
    let by_reduce = SCORES.iter().copied().reduce(|a, b| a + b);
    println!("   .reduce(|a, b| a + b)   -> {by_reduce:?}");
    println!("   Note the Option: with no items there is no starting value to");
    println!("   return, so the answer is None rather than a made-up zero.");
    let empty: [i32; 0] = [];
    println!("   on an empty iterator:   reduce -> {:?}", empty.iter().copied().reduce(|a, b| a + b));
    println!("                           fold   -> {}", empty.iter().fold(0, |a, b| a + b));
    println!("   fold's answer is the identity you supplied. reduce refuses to");
    println!("   invent one, which is the right answer for max, and the wrong");
    println!("   shape for sum — where 0 really is the answer.");

    println!();
    println!("6. try_fold stops at the first failure, and that is how any/all/find work");
    let calls = Cell::new(0);
    let parsed: Result<i32, std::num::ParseIntError> =
        ["5", "3", "no", "4"].into_iter().try_fold(0, |acc, s| {
            calls.set(calls.get() + 1);
            Ok(acc + s.parse::<i32>()?)
        });
    println!("   try_fold over [5, 3, \"no\", 4] -> {}", match &parsed {
        Ok(v) => format!("Ok({v})"),
        Err(e) => format!("Err({e})"),
    });
    println!("   the closure ran {} times, not 4 — it stopped at the bad row.", calls.get());
    println!("   A plain fold cannot do that: it has no way to say \"stop\", so it");
    println!("   would have to carry the error to the end in the accumulator.");

    println!();
    println!("7. The trap: an accumulator that is rebuilt rather than carried");
    let cheap = SCORES.iter().fold(Vec::new(), |mut acc, s| {
        acc.push(*s);
        acc
    });
    let costly = SCORES.iter().fold(Vec::new(), |acc, s| {
        let mut next = acc.clone();
        next.push(*s);
        next
    });
    println!("   carried:  {cheap:?}");
    println!("   cloned:   {costly:?}");
    println!("   Identical output. The second built six Vecs to produce one, which");
    println!("   is quadratic in the length. `mut acc` and returning it is the");
    println!("   shape to write; a `.clone()` inside a fold is nearly always this.");
}
