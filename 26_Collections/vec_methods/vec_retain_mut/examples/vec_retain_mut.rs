fn main() {
    // Same pass as retain, but the predicate gets &mut T — so it can filter
    // and edit in one traversal.
    let mut v = vec![1, 2, 3, 4, 5];
    v.retain_mut(|n| { *n *= 10; *n < 40 });
    println!("{v:?}");

    // Note what that means: the elements that get dropped were mutated first.
    // The closure runs on every element, keep or not.
    let mut touched = vec![];
    let mut v = vec![1, 2, 3];
    v.retain_mut(|n| { touched.push(*n); *n > 1 });
    println!("predicate saw {touched:?}, kept {v:?}");

    // Trimming and filtering a list of strings in one pass.
    let mut lines = vec![
        String::from("  alpha  "),
        String::from("   "),
        String::from(" beta"),
    ];
    lines.retain_mut(|s| { *s = s.trim().to_string(); !s.is_empty() });
    println!("{lines:?}");

    // Without retain_mut this is two passes (or a fold, or an index loop).
    let mut lines = vec![String::from(" a "), String::from("  ")];
    for s in lines.iter_mut() { *s = s.trim().to_string(); }
    lines.retain(|s| !s.is_empty());
    println!("the two-pass version: {lines:?}");

    // A running state in the closure: keep every element whose running total
    // is still under a budget, and record what it cost.
    let mut spent = 0;
    let mut costs = vec![3, 4, 10, 2, 5];
    costs.retain_mut(|c| { if spent + *c <= 9 { spent += *c; true } else { false } });
    println!("kept {costs:?} spending {spent}");
}
