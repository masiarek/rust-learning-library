//! Kata solution: a rolling window over a `VecDeque`, and the two silent bugs.
//!
//!   rustc --edition 2024 the_vecdeque_kata.rs -o /tmp/vdk && /tmp/vdk

use std::collections::VecDeque;

const LIMIT: usize = 3;
const READINGS: [i32; 5] = [30, 10, 50, 20, 40];

/// The whole window, in logical order — front is the oldest reading.
fn mean(window: &VecDeque<i32>) -> f64 {
    window.iter().sum::<i32>() as f64 / window.len() as f64
}

/// The same sum written the way that looks obviously right and is not.
fn mean_first_slice(window: &VecDeque<i32>) -> f64 {
    let (front, _back) = window.as_slices();
    front.iter().sum::<i32>() as f64 / window.len() as f64
}

fn main() {
    println!("1. The window: push_back, and pop_front once it is full");
    let mut window: VecDeque<i32> = VecDeque::with_capacity(LIMIT);
    for r in READINGS {
        if window.len() == LIMIT {
            window.pop_front();
        }
        window.push_back(r);
        let shown = format!("{window:?}");
        println!("   reading {r:>3}   window {shown:<16} mean {:>6.2}", mean(&window));
    }
    println!("   Two O(1) operations per reading, and no element ever moves: the");
    println!("   evicted one is forgotten by bumping `head`, and the new one is");
    println!("   written into the slot it just left.");

    println!();
    println!("2. The bug: reading the ring through as_slices().0");
    let mut window: VecDeque<i32> = VecDeque::with_capacity(LIMIT);
    for (step, r) in READINGS.into_iter().enumerate() {
        if window.len() == LIMIT {
            window.pop_front();
        }
        window.push_back(r);
        let (front, back) = window.as_slices();
        let good = mean(&window);
        let bad = mean_first_slice(&window);
        let flag = if (good - bad).abs() > f64::EPSILON { "  <- WRONG" } else { "" };
        println!("   step {}  as_slices ({front:?}, {back:?})", step + 1);
        println!("           iter() {good:>6.2}   as_slices().0 {bad:>6.2}{flag}");
    }
    println!("   The first eviction is where it starts: from then on the contents wrap,");
    println!("   `.0` is only the piece up to the end of the buffer, and the sum quietly");
    println!("   drops whatever came after it. Nothing panics, no index is out of range,");
    println!("   and the number is merely wrong. Use `iter()`, `range(..)` or the");
    println!("   `(front, back)` pair — never `.0` alone.");

    println!();
    println!("3. The second bug: sorting the window in place");
    println!("   window before {window:?}   oldest reading is {:?}", window.front());
    let mut copy: Vec<i32> = window.iter().copied().collect();
    copy.sort();
    println!("   median from a copy: {}   window untouched {window:?}", copy[LIMIT / 2]);
    let mut in_place = window.clone();
    in_place.make_contiguous().sort();
    println!("   median in place:    {}   window now      {in_place:?}",
             in_place[LIMIT / 2]);
    println!("   `make_contiguous()` hands back `&mut [T]`, so sorting through it");
    println!("   reorders the deque itself — and the front is no longer the oldest");
    println!("   reading. The next eviction then drops the wrong one:");
    let mut a = window.clone();
    let mut b = in_place.clone();
    a.pop_front();
    a.push_back(60);
    b.pop_front();
    b.push_back(60);
    println!("   after reading  60:  correct {a:?}   after sorting in place {b:?}");
    println!("   Sort a copy when the deque is a queue; sort in place only when the");
    println!("   order was never carrying meaning.");

    println!();
    println!("4. The same window on a Vec, and what it costs");
    let mut v: Vec<i32> = Vec::with_capacity(LIMIT);
    for r in READINGS {
        if v.len() == LIMIT {
            v.remove(0);
        }
        v.push(r);
    }
    println!("   Vec version      {v:?}   same answers, and it derefs to a slice, so");
    println!("   `v.iter()`, `v.sort()` and `&v[..]` all work with no ceremony.");
    let before = v.as_ptr();
    let first_was = v[0];
    v.remove(0);
    println!("   remove(0): buffer pointer unchanged ({}), but v[0] went {first_was} -> {}",
             std::ptr::eq(before, v.as_ptr()), v[0]);
    println!("   That is the difference in one line. The Vec keeps the same allocation");
    println!("   and shifts every surviving element down a slot — O(n) per reading. The");
    println!("   VecDeque moves nothing and bumps an index instead. For a window of 3");
    println!("   that is noise; for a queue of a million it is the whole cost.");
    println!("   Reach for a Vec anyway when only the BACK is busy: it is one word");
    println!("   smaller, and everything on the slice page applies to it directly.");
}
