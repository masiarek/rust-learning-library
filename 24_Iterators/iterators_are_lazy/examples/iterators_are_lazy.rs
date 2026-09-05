//! Iterators are lazy: an adapter builds a plan, a consumer runs it.
//!
//!   rustc --edition 2024 iterators_are_lazy.rs -o /tmp/ial && /tmp/ial

use std::cell::{Cell, RefCell};

const SCORES: [i32; 6] = [5, 3, 0, 4, 2, 1];

fn main() {
    let calls = Cell::new(0);
    let doubled = |s: &i32| {
        calls.set(calls.get() + 1);
        s * 2
    };

    println!("1. Building the chain runs nothing at all");
    calls.set(0);
    let _plan = SCORES.iter().map(doubled);
    println!("   SCORES.iter().map(doubled)        closure calls: {}", calls.get());
    println!("   `_plan` is a value of type Map<Iter<i32>, {{closure}}>. It holds the");
    println!("   source and the closure, and has not looked at a single score.");

    println!();
    println!("2. A consumer is what runs it — and how far it runs is the consumer's call");
    calls.set(0);
    let all: Vec<i32> = SCORES.iter().map(doubled).collect();
    println!("   .collect()  -> {:<22} closure calls: {}", format!("{all:?}"), calls.get());

    calls.set(0);
    let first = SCORES.iter().map(doubled).find(|d| *d >= 6);
    println!("   .find(>=6)  -> {:<22} closure calls: {}", format!("{first:?}"), calls.get());

    calls.set(0);
    let any = SCORES.iter().map(doubled).any(|d| d == 0);
    println!("   .any(==0)   -> {:<22} closure calls: {}", any, calls.get());

    calls.set(0);
    let n = SCORES.iter().map(doubled).count();
    println!("   .count()    -> {:<22} closure calls: {}", n, calls.get());
    println!("   find and any stop at the first answer. collect and count cannot.");

    println!();
    println!("3. The chain is one pass, element at a time — not one pass per adapter");
    let log = RefCell::new(Vec::new());
    let odds_doubled: Vec<i32> = SCORES
        .iter()
        .inspect(|s| log.borrow_mut().push(format!("see {s}")))
        .filter(|s| {
            log.borrow_mut().push(format!("test {s}"));
            **s % 2 == 1
        })
        .map(|s| {
            log.borrow_mut().push(format!("double {s}"));
            s * 2
        })
        .take(2)
        .collect();
    println!("   result: {odds_doubled:?}");
    for step in log.borrow().iter() {
        println!("     {step}");
    }
    println!("   Each score is carried all the way through before the next one starts,");
    println!("   and `take(2)` stopped the whole chain: scores 0, 4, 2 and 1 were never");
    println!("   read. An eager version would have built two intermediate Vecs and");
    println!("   visited all six.");

    println!();
    println!("4. Which is what makes an endless sequence usable");
    let first_three: Vec<u32> = (1u32..)
        .map(|n| n * n)
        .filter(|sq| sq % 3 == 1)
        .take(3)
        .collect();
    println!("   (1..).map(square).filter(%3==1).take(3) = {first_three:?}");
    println!("   `1..` has no end. Nothing hangs, because nothing is computed until");
    println!("   `collect` asks, and it stops asking after three.");

    println!();
    println!("5. The mistake the compiler warns about");
    println!("   SCORES.iter().map(|s| println!(\"{{s}}\"));   <- prints NOTHING");
    println!("   warning: unused `Map` that must be used");
    println!("   note: iterators are lazy and do nothing unless consumed");
    println!("   `map` is for producing values. To run a side effect per item, use");
    println!("   `for_each` or a plain `for` loop, both of which consume:");
    let mut seen = Vec::new();
    SCORES.iter().take(3).for_each(|s| seen.push(*s));
    println!("   .for_each(...)  seen = {seen:?}");

    println!();
    println!("6. Laziness is per-adapter, so where you put an expensive step matters");
    let cheap_first = Cell::new(0);
    let expensive = |s: &&i32| {
        cheap_first.set(cheap_first.get() + 1);
        **s > 3
    };
    let a = SCORES.iter().filter(|s| **s % 2 == 1).filter(expensive).count();
    let after = cheap_first.get();
    cheap_first.set(0);
    let b = SCORES.iter().filter(expensive).filter(|s| **s % 2 == 1).count();
    println!("   cheap test first:  {a} kept, expensive test ran {after} times");
    println!("   expensive first:   {b} kept, expensive test ran {} times", cheap_first.get());
    println!("   same answer, different amount of work. Order the chain so the");
    println!("   cheapest, most selective test runs first.");

    println!();
    println!("7. Observing an iterator SPENDS it");
    let mut it = vec!["a".to_string(), "b".to_string()].into_iter();
    println!("   {it:?}                <- Debug on the iterator: free, takes nothing");
    let first = it.next();
    println!("   let first = it.next();       first = {first:?}");
    println!("   println!(\"{{:?}}\", it.next())  prints {:?}", it.next());
    println!("   println!(\"{{:?}}\", it.next())  prints {:?}", it.next());
    println!("   first is STILL {first:?}: a stored value, not a repeated call.");
    println!("   But assert_eq!(it.next(), Some(\"b\")) would panic here — left None,");
    println!("   right Some(\"b\") — because the two prints above already took both.");
    println!("   Printing an iterator is free; printing what next() returns is not.");
}
