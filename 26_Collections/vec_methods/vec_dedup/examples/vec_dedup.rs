fn main() {
    // CONSECUTIVE duplicates only. This is the whole trap.
    let mut v = vec![1, 2, 2, 3, 3, 3, 1];
    v.dedup();
    println!("{v:?}");        // the trailing 1 survives: it is not next to the first

    // To remove every duplicate, sort first — then equal values are adjacent.
    let mut v = vec![3, 1, 3, 2, 1, 3];
    v.sort();
    v.dedup();
    println!("sorted then deduped: {v:?}");

    // Or keep the original order with a set, which dedup cannot do.
    let mut seen = std::collections::HashSet::new();
    let mut v = vec![3, 1, 3, 2, 1, 3];
    v.retain(|n| seen.insert(*n));
    println!("first occurrence, original order: {v:?}");

    // It is O(n) and in place, which is why it is worth the "consecutive"
    // restriction: a run-length pass over already-grouped data.
    let mut log = vec!["open", "open", "read", "read", "read", "close"];
    log.dedup();
    println!("collapsed run: {log:?}");

    // The kept element of each run is the FIRST one.
    #[derive(Debug, PartialEq)]
    struct Entry { key: u8, note: &'static str }
    impl Entry { fn new(key: u8, note: &'static str) -> Self { Entry { key, note } } }
    let mut v = vec![Entry::new(1, "first"), Entry::new(1, "second")];
    // PartialEq here compares both fields, so nothing is equal and nothing goes.
    v.dedup();
    println!("full-struct equality keeps both: {}", v.len());
    println!("  keys {:?}", v.iter().map(|e| (e.key, e.note)).collect::<Vec<_>>());

    // Comparing on one field is what dedup_by_key is for.
    let mut v = vec![Entry::new(1, "first"), Entry::new(1, "second")];
    v.dedup_by_key(|e| e.key);
    println!("dedup_by_key keeps the first: {:?}", v[0]);

    // Empty and single-element vectors are no-ops.
    let mut e: Vec<u8> = vec![];
    e.dedup();
    let mut one = vec![7];
    one.dedup();
    println!("{e:?} {one:?}");
}
