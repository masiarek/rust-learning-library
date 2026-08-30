//! A `Vec` is three numbers on the stack and one allocation on the heap.
//!
//!   rustc --edition 2024 the_vec.rs -o /tmp/vec && /tmp/vec

fn total(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

fn main() {
    println!("1. Three numbers: pointer, length, capacity");
    println!("   size_of::<Vec<u32>>()  = {}  (three usize)", size_of::<Vec<u32>>());
    println!("   size_of::<Vec<[u8; 999]>>() = {}  — the same three, whatever T is",
             size_of::<Vec<[u8; 999]>>());
    println!("   The elements are not in the Vec. The Vec is a receipt for them.");

    println!();
    println!("2. Growth is amortised doubling, and you can watch it");
    let mut v: Vec<u32> = Vec::new();
    println!("   Vec::new()            len {} cap {}", v.len(), v.capacity());
    for n in 1..=9u32 {
        let before = v.capacity();
        v.push(n);
        let after = v.capacity();
        if after != before {
            println!("   push({n}) reallocated   len {} cap {before} -> {after}", v.len());
        }
    }
    println!("   after 9 pushes        len {} cap {}", v.len(), v.capacity());
    println!("   Nine pushes, three allocations. Doubling is why pushing n items");
    println!("   costs O(n) in total rather than O(n^2), and the exact sequence is");
    println!("   this std's choice, not a promise in the language.");

    println!();
    println!("3. If you know the size, say so");
    let mut sized: Vec<u32> = Vec::with_capacity(9);
    let start = sized.capacity();
    for n in 1..=9u32 {
        sized.push(n);
    }
    println!("   with_capacity(9): cap {start} -> {} — no reallocation at all", sized.capacity());
    println!("   Same nine values, one allocation instead of three, and no copying");
    println!("   of the old contents.");

    println!();
    println!("4. A Vec derefs to a slice, so slice methods just work");
    println!("   total(&v) = {} — `total` takes &[u32] and was handed a &Vec<u32>",
             total(&v));
    println!("   v.first() = {:?}, v.contains(&5) = {}", v.first(), v.contains(&5));
    println!("   v.iter().rev().take(3): {:?}", v.iter().rev().take(3).collect::<Vec<_>>());
    println!("   Write &[T] in a signature and both callers work. Write &Vec<T>");
    println!("   and you have refused arrays, slices and everything borrowed.");

    println!();
    println!("5. Removing: the one that keeps order, and the one that is fast");
    let mut a: Vec<char> = "abcdef".chars().collect();
    let mut b = a.clone();
    let removed = a.remove(1);
    let swapped = b.swap_remove(1);
    println!("   remove(1)      -> {removed}, left {a:?}   (everything after shifts down)");
    println!("   swap_remove(1) -> {swapped}, left {b:?}   (the last element fills the hole)");
    println!("   O(n) versus O(1). If you are about to sort anyway, take the O(1).");

    let mut scores = vec![5, 3, 0, 4, 0, 2];
    scores.retain(|&s| s > 0);
    println!("   retain(|s| s > 0) on [5, 3, 0, 4, 0, 2] -> {scores:?}");
    println!("   One pass, one shift per survivor — not one `remove` per zero.");
}
