fn main() {
    let mut nums = vec![5, 3, 9, 1, 3];
    nums.sort_unstable();
    println!("{nums:?}");

    // "Unstable" is about the relative order of EQUAL elements. Sort by the
    // first field only, so the letters show whether the ties moved.
    let mut stable = vec![(1, "a"), (0, "b"), (1, "c"), (0, "d"), (1, "e"), (0, "f")];
    let mut unstable = stable.clone();
    stable.sort_by_key(|p| p.0);
    unstable.sort_unstable_by_key(|p| p.0);
    println!("stable:   {stable:?}");
    println!("unstable: {unstable:?}");
    println!("same order on this input: {}", stable == unstable);

    // With a total comparison there is nothing to distinguish, so prefer this one.
    let mut words = vec!["pear", "fig", "apple"];
    words.sort_unstable();
    println!("{words:?}");
}
