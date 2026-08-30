fn main() {
    // Collapse consecutive runs that map to the same key.
    let mut v = vec![10, 16, 20, 21, 30];
    v.dedup_by_key(|n| *n / 10);
    println!("{v:?}");

    // The element kept from each run is the FIRST one.
    #[derive(Debug)]
    struct Row { id: u8, note: &'static str }
    let mut rows = vec![
        Row { id: 1, note: "first" },
        Row { id: 1, note: "second" },
        Row { id: 2, note: "third" },
    ];
    rows.dedup_by_key(|r| r.id);
    println!("{:?}", rows.iter().map(|r| (r.id, r.note)).collect::<Vec<_>>());

    // Consecutive, still. Sort by the key first if the runs are not grouped.
    let mut v = vec![("b", 1), ("a", 2), ("b", 3)];
    v.dedup_by_key(|p| p.0);
    println!("ungrouped: {v:?}");
    let mut v = vec![("b", 1), ("a", 2), ("b", 3)];
    v.sort_by_key(|p| p.0);
    v.dedup_by_key(|p| p.0);
    println!("sorted first: {v:?}");

    // The closure gets &mut T, so it may normalise on the way past.
    let mut names = vec![String::from("Ada"), String::from("ADA"), String::from("Ben")];
    names.dedup_by_key(|s| { *s = s.to_lowercase(); s.clone() });
    println!("{names:?}");

    // The key type only needs PartialEq, not Ord or Hash.
    #[derive(PartialEq)]
    struct Key(bool);
    let mut flags = vec![0, 2, 4, 5, 7, 8];
    flags.dedup_by_key(|n| Key(*n % 2 == 0));
    println!("runs of even/odd: {flags:?}");
}
