//! Kata solution: the first ten odd numbers, as a block and as `from_fn`.
//!
//!   rustc --edition 2024 writing_an_array_down_kata.rs -o /tmp/wadk && /tmp/wadk

use std::any::type_name_of_val;

fn main() {
    println!("1. A block that fills a zeroed array, then hands it out");
    let odd_numbers: [i32; 10] = {
        let mut arr = [0; 10];
        for (i, num) in (0..10).map(|x| 2 * x + 1).enumerate() {
            arr[i] = num;
        }
        arr // the block's value: no `;`, so the array leaves the block
    };
    println!("   {odd_numbers:?}");
    println!("   `arr` is mutable only inside the block; odd_numbers is not mut at all");

    println!();
    println!("2. from_fn: the closure is asked for element i, N comes from the type");
    let from_fn: [i32; 10] = std::array::from_fn(|i| 2 * i as i32 + 1);
    println!("   {from_fn:?}");
    println!("   same array? {}", odd_numbers == from_fn);

    println!();
    println!("3. `2 * i as i32 + 1` reads as `(2 * (i as i32)) + 1`");
    let i: usize = 4;
    println!("   i = {i}: 2 * i as i32 + 1 = {}", 2 * i as i32 + 1);
    println!("   `as` binds tighter than `*`, so only i is cast; the 2 and the 1");
    println!("   are then inferred as i32 from it. The closure's i is a usize");
    println!("   because from_fn passes an index: {}", type_name_of_val(&i));

    println!();
    println!("4. Twelve instead of ten: count the edits");
    let twelve: [i32; 12] = std::array::from_fn(|i| 2 * i as i32 + 1);
    println!("   from_fn, one edit (the type):          {twelve:?}");
    let twelve_block: [i32; 12] = {
        let mut arr = [0; 12];
        for (i, num) in (0..12).map(|x| 2 * x + 1).enumerate() {
            arr[i] = num;
        }
        arr
    };
    println!("   block, three edits ([i32; _], [0; _], 0.._): {twelve_block:?}");
    println!("   Miss the [0; 12] and it is E0308 at `arr`: \"expected an array with a");
    println!("   size of 12, found one with a size of 10\".");
    println!("   Miss the 0..12 instead and it compiles, leaving [.., 0, 0] at the end:");
    let forgot_range: [i32; 12] = {
        let mut arr = [0; 12];
        for (i, num) in (0..10).map(|x| 2 * x + 1).enumerate() {
            arr[i] = num;
        }
        arr
    };
    println!("   {forgot_range:?}");
}
