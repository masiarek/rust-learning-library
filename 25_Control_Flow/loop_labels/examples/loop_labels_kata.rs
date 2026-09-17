//! Kata solution: three ways out of a grid of shelves.
//!
//!   rustc --edition 2024 loop_labels_kata.rs -o /tmp/llk && /tmp/llk

fn main() {
    // shelves[s][slot] = how many items are in that slot; 0 means empty
    let shelves = [
        [3, 5, 2, 8],
        [4, 0, 6, 1],
        [7, 7, 7, 7],
        [0, 2, 9, 3],
    ];

    println!("1. The first empty slot, three ways");

    // (a) a flag variable, checked once in each loop
    let mut by_flag = None;
    let mut done = false;
    for (s, shelf) in shelves.iter().enumerate() {
        for (slot, &count) in shelf.iter().enumerate() {
            if count == 0 {
                by_flag = Some((s, slot));
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }

    // (b) a label, so one break leaves both loops
    let mut by_label = None;
    'shelves: for (s, shelf) in shelves.iter().enumerate() {
        for (slot, &count) in shelf.iter().enumerate() {
            if count == 0 {
                by_label = Some((s, slot));
                break 'shelves;
            }
        }
    }

    // (c) no loop at all: the search is a value
    let by_chain = shelves
        .iter()
        .enumerate()
        .find_map(|(s, shelf)| shelf.iter().position(|&count| count == 0).map(|slot| (s, slot)));

    println!("   (a) flag       {by_flag:?}");
    println!("   (b) label      {by_label:?}");
    println!("   (c) find_map   {by_chain:?}");

    println!();
    println!("2. Total stock on the shelves that have no empty slot");
    let mut full_shelves = Vec::new();
    let mut total = 0;
    'shelves: for (s, shelf) in shelves.iter().enumerate() {
        for &count in shelf {
            if count == 0 {
                continue 'shelves; // the rest of this shelf does not matter
            }
        }
        full_shelves.push(s);
        total += shelf.iter().sum::<i32>();
    }
    println!("   full shelves {full_shelves:?}, total stock {total}");

    let chain_total: i32 = shelves
        .iter()
        .filter(|shelf| shelf.iter().all(|&count| count > 0))
        .map(|shelf| shelf.iter().sum::<i32>())
        .sum();
    println!("   the same with filter(all > 0): {chain_total}");

    println!();
    println!("3. The prediction: (b) with the label taken off the break");
    let mut unlabelled = None;
    let mut assignments = 0;
    for (s, shelf) in shelves.iter().enumerate() {
        for (slot, &count) in shelf.iter().enumerate() {
            if count == 0 {
                unlabelled = Some((s, slot));
                assignments += 1;
                break;
            }
        }
    }
    println!("   answer {unlabelled:?}, written {assignments} times");
    println!("   The break ended each shelf's slot loop, and the shelf loop went");
    println!("   on to shelf 3, whose empty slot replaced the right answer.");
}
