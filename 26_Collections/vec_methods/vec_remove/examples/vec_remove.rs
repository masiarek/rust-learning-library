fn main() {
    let mut v = vec!["a", "b", "c", "d"];
    println!("removed {:?}, left {v:?}", v.remove(1));

    // Order is preserved, which is what makes it O(n): everything after the
    // hole shifts left by one.
    let mut v: Vec<u8> = (0..8).collect();
    v.remove(0);
    println!("remove(0) keeps order: {v:?}");

    // Removing several by index means the later indices move under you.
    // Going back to front avoids that; retain avoids the whole problem.
    let mut v = vec![10, 11, 12, 13, 14];
    for i in [3, 1] { v.remove(i); }          // descending order on purpose
    println!("removed indices 1 and 3: {v:?}");

    let mut v = vec![10, 11, 12, 13, 14];
    let mut i = 0;
    v.retain(|_| { let keep = i != 1 && i != 3; i += 1; keep });
    println!("the same thing with retain: {v:?}");

    // Out of bounds panics; there is a non-panicking sibling only in nightly,
    // so the guard is yours to write.
    let mut v = vec![1, 2, 3];
    let idx = 7;
    if idx < v.len() { v.remove(idx); } else { println!("index {idx} is past len {}", v.len()); }

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| { let mut v = vec![1]; v.remove(3); });
    std::panic::set_hook(hook);
    println!("remove(3) from a len-1 vec panicked: {}", caught.is_err());
}
