//! Kata solution: count the reallocations, then delete them.
//!
//!   rustc --edition 2024 the_vec_kata.rs -o /tmp/vk && /tmp/vk

/// Reads a tally into a fresh Vec the naive way, reporting every reallocation.
fn grow_naively(rows: &[u32]) -> (Vec<u32>, usize, usize) {
    let mut out = Vec::new();
    let mut reallocations = 0;
    let mut copied = 0;
    for &r in rows {
        let before = out.capacity();
        let filled = out.len();
        out.push(r);
        if out.capacity() != before {
            reallocations += 1;
            copied += filled;   // everything already stored is moved to the new buffer
        }
    }
    (out, reallocations, copied)
}

/// The same, told how many rows are coming.
fn grow_sized(rows: &[u32]) -> (Vec<u32>, usize, usize) {
    let mut out = Vec::with_capacity(rows.len());
    let mut reallocations = 0;
    let mut copied = 0;
    for &r in rows {
        let before = out.capacity();
        let filled = out.len();
        out.push(r);
        if out.capacity() != before {
            reallocations += 1;
            copied += filled;
        }
    }
    (out, reallocations, copied)
}

fn main() {
    let rows: Vec<u32> = (1..=100).collect();

    println!("1. One hundred pushes");
    let (naive, n1, copied1) = grow_naively(&rows);
    let (sized, n2, copied2) = grow_sized(&rows);
    println!("   Vec::new()          -> {} items, {n1} reallocations, cap {}", naive.len(), naive.capacity());
    println!("   with_capacity(100)  -> {} items, {n2} reallocations, cap {}", sized.len(), sized.capacity());
    println!("   Every reallocation copies everything already stored: {copied1} u32s");
    println!("   moved, against {copied2} for the sized version. The counting is in");
    println!("   the program — `out.len()` just before each growth, summed.");

    println!();
    println!("2. collect() already knew");
    let collected: Vec<u32> = (1..=100).collect();
    println!("   (1..=100).collect() -> {} items, cap {}", collected.len(), collected.capacity());
    println!("   A range knows its own length, so collect asked once and got it");
    println!("   right. That is `size_hint`, and it is why collect usually beats");
    println!("   a hand-written push loop without you doing anything.");

    println!();
    println!("3. Shrinking does not give the memory back");
    let mut v = collected.clone();
    v.truncate(3);
    println!("   after truncate(3): len {} cap {}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("   after shrink_to_fit(): len {} cap {}", v.len(), v.capacity());
    println!("   `clear` and `truncate` drop the elements and keep the buffer —");
    println!("   which is the behaviour you want in a loop that refills it.");

    println!();
    println!("4. Two removals, one of which is a bug");
    let names = ["Ada", "Ben", "Cara", "Dan", "Eve"];
    let mut keep_order: Vec<&str> = names.to_vec();
    let mut fast: Vec<&str> = names.to_vec();
    keep_order.remove(1);
    fast.swap_remove(1);
    println!("   remove(1)      -> {keep_order:?}");
    println!("   swap_remove(1) -> {fast:?}");
    println!("   Identical cost story, opposite guarantees. If this Vec is a");
    println!("   ranking, swap_remove has silently reordered the results.");

    println!();
    println!("5. Deleting while iterating, the three ways");
    let start = vec![5u32, 0, 3, 0, 4];
    let mut by_retain = start.clone();
    by_retain.retain(|&s| s != 0);
    let by_filter: Vec<u32> = start.iter().copied().filter(|&s| s != 0).collect();
    let mut by_index = start.clone();
    let mut i = 0;
    while i < by_index.len() {
        if by_index[i] == 0 {
            by_index.remove(i);
        } else {
            i += 1;
        }
    }
    println!("   retain            -> {by_retain:?}   in place, one pass");
    println!("   filter().collect  -> {by_filter:?}   a new Vec, borrows the old");
    println!("   index loop        -> {by_index:?}   correct only because `i` is");
    println!("   not incremented after a removal — the classic off-by-one is to");
    println!("   write a `for i in 0..len` here and skip every element after a");
    println!("   deleted one. Rust will not stop you: the indices are all valid.");
}
