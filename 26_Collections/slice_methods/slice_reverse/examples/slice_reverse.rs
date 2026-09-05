fn main() {
    let mut v = vec![1, 2, 3];
    v.reverse();
    println!("{v:?}");

    // For a reversed copy, iterate instead; v is untouched.
    let copy: Vec<i32> = v.iter().rev().copied().collect();
    println!("{copy:?} {v:?}");

    // Descending sort: sort, then reverse.
    let mut nums = vec![3, 1, 2];
    nums.sort();
    nums.reverse();
    println!("{nums:?}");

    // A sub-range reverses on its own.
    let mut part = vec![1, 2, 3, 4, 5];
    part[1..4].reverse();
    println!("{part:?}");

    // It returns (): binding or printing the call is the trap.
    let receipt = part.reverse();
    println!("{receipt:?} {part:?}");

    // Reversing a string is by chars, not by bytes.
    let back: String = "abc".chars().rev().collect();
    println!("{back}");
}
