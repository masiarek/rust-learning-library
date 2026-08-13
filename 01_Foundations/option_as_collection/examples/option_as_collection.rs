//! `Option` is a collection that holds either zero items or one.
//!
//! Two questions that always arrive together:
//!   * is an Option iterable?              — yes, and that buys you the whole Iterator toolbox
//!   * are there numbers behind Some/None? — yes, but not ones you are allowed to use
//!
//!   rustc --edition 2024 option_as_collection.rs -o /tmp/oac && /tmp/oac

use std::mem::{discriminant, size_of};

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() {
    banner(1, "Yes — you can `for` over an Option");

    for i in Some(5) {
        println!("  for i in Some(5)       -> body ran, i = {i}");
    }
    for i in None::<i32> {
        println!("  for i in None          -> {i}");
    }
    println!("  for i in None::<i32>   -> body never ran");

    println!("  Some(5).iter().count()     = {}", Some(5).iter().count());
    println!("  None::<i32>.iter().count() = {}", None::<i32>.iter().count());
    println!("      An Option is a Vec that can never hold more than one thing.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "So every iterator adapter already works on it");

    let mixed = vec![Some(1), None, Some(3)];
    let kept: Vec<i32> = mixed.iter().copied().flatten().collect();
    println!("  flatten     [Some(1), None, Some(3)] -> {kept:?}");

    let mut v = vec![1, 2];
    v.extend(Some(3));
    v.extend(None::<i32>);
    println!("  extend      [1,2] + Some(3) + None   -> {v:?}");

    let words = ["1", "x", "3"];
    let nums: Vec<u32> = words.iter().filter_map(|s| s.parse().ok()).collect();
    println!("  filter_map  {words:?} -> {nums:?}");

    let chained: Vec<i32> = vec![1, 2].into_iter().chain(Some(9)).collect();
    println!("  chain       [1,2].chain(Some(9))     -> {chained:?}");
    println!("      filter_map is just map + flatten: the None rows drop out on their own.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "The reverse: collect a pile of Options into ONE Option");

    let all: Option<Vec<i32>> = vec![Some(1), Some(2), Some(3)].into_iter().collect();
    let any: Option<Vec<i32>> = vec![Some(1), None, Some(3)].into_iter().collect();
    println!("  every Some -> {all:?}");
    println!("  one None   -> {any:?}");
    println!("      All-or-nothing, and it short-circuits: nothing after the first None is visited.");
    println!("      Result collects the same way, which is how you validate a whole batch at once.");
}

// ─────────────────────────────────────────────────────────── Step 4
fn step4() {
    banner(4, "Are there numbers behind Some and None?");

    // `None` is declared FIRST in std's definition, and the derived ordering
    // follows declaration order. So the ordering is observable and guaranteed.
    println!("  None::<i32> < Some(0)  = {}", None::<i32> < Some(0));
    println!("  Some(1) < Some(2)      = {}", Some(1) < Some(2));

    // A discriminant is comparable but deliberately opaque — no number falls out.
    println!(
        "  discriminant(&None) == discriminant(&Some(1)) = {}",
        discriminant(&None::<i32>) == discriminant(&Some(1))
    );
    println!("  discriminant(&None::<i32>) prints as {:?}", discriminant(&None::<i32>));
    println!("  discriminant(&Some(1))     prints as {:?}", discriminant(&Some(1)));
    println!("      There ARE numbers: None is variant 0, Some is variant 1 — declaration order.");
    println!("      But you cannot get at them. `None as i32` does NOT compile: a cast is only");
    println!("      allowed for a fieldless enum, and Some(T) carries data. The Debug text above");
    println!("      is a courtesy, not an API — compare the sizes in the next step.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn step5() {
    banner(5, "…and the tag is not always even stored");

    println!("  i32                      {:>3} bytes", size_of::<i32>());
    println!("  Option<i32>              {:>3}", size_of::<Option<i32>>());
    println!("  Box<i32>                 {:>3}", size_of::<Box<i32>>());
    println!("  Option<Box<i32>>         {:>3}", size_of::<Option<Box<i32>>>());
    println!("  Option<Option<Box<i32>>> {:>3}", size_of::<Option<Option<Box<i32>>>>());
    println!("  bool                     {:>3}", size_of::<bool>());
    println!("  Option<bool>             {:>3}", size_of::<Option<bool>>());
    println!("      Where the inner type has an unused bit pattern (a 'niche'), None takes it");
    println!("      and the tag costs nothing. That is why the number is not part of the API.");
}

// ─────────────────────────────────────────────────────────── Step 6
fn step6() {
    banner(6, "Asking a question without unwrapping");

    let x: Option<u32> = Some(2);
    println!("  Some(2).is_some()              = {}", x.is_some());
    println!("  Some(2).is_none()              = {}", x.is_none());
    println!("  Some(2).is_some_and(|n| n > 1) = {}", x.is_some_and(|n| n > 1));
    println!("  Some(0).is_some_and(|n| n > 1) = {}", Some(0u32).is_some_and(|n| n > 1));
    println!("  None.is_some_and(|n| n > 1)    = {}", None::<u32>.is_some_and(|n| n > 1));
    println!("  Some(2).map_or(0, |n| n * 10)  = {}", Some(2).map_or(0, |n| n * 10));
    println!("  None.map_or(0, |n| n * 10)     = {}", None::<i32>.map_or(0, |n| n * 10));
    println!("      is_some_and replaces the `x.is_some() && x.unwrap() > 1` you were about to write.");
}

// ─────────────────────────────────────────────────────────── Step 7
fn step7() {
    banner(7, "take() and replace(): moving out of a field you only borrow");

    let mut slot = Some(String::from("first"));
    let got = slot.take();
    println!("  after take()     got = {got:?}, slot = {slot:?}");

    let old = slot.replace(String::from("second"));
    println!("  after replace()  old = {old:?}, slot = {slot:?}");

    println!("      take() swaps in None and hands you the value. It is the standard way to");
    println!("      move a non-Copy value out of a &mut, which the borrow checker otherwise refuses.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    step7();
    println!();
}
