fn main() {
    let mut nums = vec![5, 3, 9, 1, 3];
    nums.sort();
    println!("{nums:?}");

    // Any slice sorts, not just a whole Vec: an array, or a sub-range.
    let mut arr = [3, 1, 2];
    arr.sort();
    let mut part = vec![9, 4, 3, 2, 1, 0];
    part[1..4].sort();
    println!("{arr:?} {part:?}");

    // Strings sort by bytes: every uppercase letter precedes every lowercase one.
    let mut words = vec!["pear", "Apple", "fig", "apple", "Zoo"];
    words.sort();
    println!("{words:?}");

    // sort returns (), so binding or printing the result is the trap.
    let receipt = nums.sort();
    println!("{receipt:?}  <- the receipt; nums itself is {nums:?}");

    // f64 is not Ord, so `floats.sort()` is error[E0277]. total_cmp is the fix.
    let mut floats = vec![2.5, -1.0, 0.0];
    // floats.sort();
    floats.sort_by(f64::total_cmp);
    println!("{floats:?}");
}
