fn main() {
    // insert_mut is insert plus a &mut to the element that landed there.
    let mut v = vec![String::from("Ada"), String::from("Cara")];
    let slot = v.insert_mut(1, String::new());
    slot.push_str("Ben");
    println!("{v:?}");

    // The same shape as push_mut, one position earlier in the vector.
    let mut v: Vec<Vec<u8>> = vec![vec![9]];
    let row = v.insert_mut(0, Vec::new());
    row.extend_from_slice(&[1, 2, 3]);
    println!("{v:?}");

    // It is #[must_use]: ignoring the reference means you wanted `insert`.
    let mut v = vec![1, 2];
    let r = v.insert_mut(1, 0);
    *r = 99;
    println!("{v:?}");

    // Same rules as insert: index <= len, and everything after shifts right.
    let mut v = vec!["a", "b"];
    let end = v.insert_mut(2, "c");
    println!("inserted at len: {end}");
    println!("{v:?}");

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| {
        let mut v = vec![1];
        let _ = v.insert_mut(5, 0);
    });
    std::panic::set_hook(hook);
    println!("insert_mut past the end panicked: {}", caught.is_err());
}
