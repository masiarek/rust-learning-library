//! `[T; N]` is a type per length. `&[T]` is the one you put in a signature.
//!
//!   rustc --edition 2024 arrays_and_slices.rs -o /tmp/aas && /tmp/aas

/// Takes a slice, so it accepts an array of any length, a `Vec`, or part of either.
/// Takes the mutable half of the same view. It can reorder and overwrite every
/// element it can see; it can never add or remove one.
fn zero_the_lowest(scores: &mut [u32]) {
    if let Some(slot) = scores.iter_mut().min() {
        *slot = 0;
    }
}

fn total(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

fn main() {
    let five: [u32; 5] = [5, 3, 0, 4, 2];
    let three = [1u32, 2, 3];

    println!("1. The length is part of the type");
    println!("   five  : [u32; 5] = {five:?}");
    println!("   three : [u32; 3] = {three:?}");
    println!("   [u32; 5] and [u32; 3] are as different as u32 and String. A");
    println!("   function taking [u32; 5] rejects a four-element array.");
    println!("   size_of::<[u32; 5]>() = {}  — five values, no header", size_of::<[u32; 5]>());

    println!();
    println!("2. A slice is a view: pointer plus length, and no ownership");
    println!("   size_of::<&[u32; 5]>() = {}  — a plain pointer", size_of::<&[u32; 5]>());
    println!("   size_of::<&[u32]>()    = {} — pointer AND length", size_of::<&[u32]>());
    println!("   The length moved out of the type and into the value, which is");
    println!("   why one function can serve every length.");
    println!("   total(&five) = {}, total(&three) = {}", total(&five), total(&three));
    println!("   total(&five[1..3]) = {} — same function, part of the array", total(&five[1..3]));

    println!();
    println!("3. Ranges are half-open: the end is not included");
    println!("   five[1..3] = {:?}   (indices 1 and 2)", &five[1..3]);
    println!("   five[..2]  = {:?}      five[3..] = {:?}", &five[..2], &five[3..]);
    println!("   five[..]   = {:?}  — the whole thing, as a slice", &five[..]);

    println!();
    println!("4. Out of bounds is a panic, not a wrong answer");
    println!("   five[9] does not even compile — rustc constant-folds the index");
    println!("   and refuses: `error: this operation will panic at runtime`.");
    let i = std::hint::black_box(9usize);   // an index rustc cannot see
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let boom = std::panic::catch_unwind(|| five[i]);
    std::panic::set_hook(hook);
    println!("   five[i] where i = 9 -> {}", if boom.is_err() { "panicked" } else { "returned" });
    println!("   five.get(9) -> {:?}", five.get(9));
    println!("   five.get(1) -> {:?}", five.get(1));
    println!("   `[i]` asserts the index is in range; `.get(i)` asks. C reads");
    println!("   past the end and keeps going; Rust stops the program.");

    println!();
    println!("5. The methods live on the slice, so the array gets them free");
    println!("   five.first() = {:?}, five.last() = {:?}", five.first(), five.last());
    println!("   five.contains(&4) = {}", five.contains(&4));
    let mut sorted = five;
    sorted.sort();
    println!("   a copy, sorted: {sorted:?}   (the original is still {five:?})");
    println!("   `five` is Copy because u32 is, so `let mut sorted = five` copied it.");
    println!("   windows(2): {:?}", five.windows(2).collect::<Vec<_>>());
    println!("   chunks(2):  {:?}", five.chunks(2).collect::<Vec<_>>());

    println!();
    println!("6. A slice is not read-only — but its length is fixed");
    let mut arr = [5u32, 3, 9, 4, 2];
    let mut v = vec![5u32, 3, 9, 4, 2];
    zero_the_lowest(&mut arr);
    zero_the_lowest(&mut v);
    println!("   zero_the_lowest(&mut [u32]) on an array: {arr:?}");
    println!("   ...and the same function on a Vec:       {v:?}");
    println!("   ...and on part of one:");
    zero_the_lowest(&mut v[..2]);
    println!("   zero_the_lowest(&mut v[..2])             {v:?}");
    println!("   &mut [T] is a slice too, so \"slices are read-only\" is wrong:");
    println!("   sort, reverse, fill and iter_mut all write through one. The");
    println!("   line is LENGTH, not writing — a slice can change every element");
    println!("   it can see and can never add or remove one. Which is why a");
    println!("   function that sorts takes &mut [T] and only a function that");
    println!("   pushes needs &mut Vec<T>.");
}
