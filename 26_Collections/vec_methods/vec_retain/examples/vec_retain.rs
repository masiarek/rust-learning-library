fn main() {
    // Keeps the elements the predicate approves — note "retain", not "remove".
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|n| n % 2 == 0);
    println!("{v:?}");

    // Order is preserved and it is a single O(n) pass, which is what makes it
    // the right answer instead of a loop calling remove().
    let mut words = vec!["a", "", "bb", "", "ccc"];
    words.retain(|w| !w.is_empty());
    println!("{words:?}");

    // The predicate takes &T, so it can read but not change.
    let mut v = vec![String::from("keep"), String::from("drop this")];
    v.retain(|s| !s.contains(' '));
    println!("{v:?}");

    // Visited in order, exactly once each — so a counter works as an index.
    let mut seen = vec![];
    let mut v = vec![10, 20, 30];
    v.retain(|n| { seen.push(*n); true });
    println!("visit order: {seen:?}");

    // Deduplicating with a set the predicate closes over. dedup() only
    // removes CONSECUTIVE duplicates; this removes all of them.
    let mut seen = std::collections::HashSet::new();
    let mut v = vec![3, 1, 3, 2, 1, 3];
    v.retain(|n| seen.insert(*n));
    println!("first occurrence of each: {v:?}");

    // Dropped elements are dropped as it goes.
    struct Noisy(u8);
    impl Drop for Noisy {
        fn drop(&mut self) { println!("  dropping {}", self.0); }
    }
    let mut v = vec![Noisy(1), Noisy(2), Noisy(3)];
    println!("retain(even):");
    v.retain(|n| n.0 % 2 == 0);
    println!("  survivor: {}", v[0].0);
}
