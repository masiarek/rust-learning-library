fn main() {
    let mut s = String::from("hello");
    s.insert(0, '>');
    s.insert(3, '-');
    println!("{s:?}");

    // At the end, it is push.
    let n = s.len();
    s.insert(n, '!');
    println!("{s:?}");

    // Byte offsets, not character positions.
    let mut accented = String::from("héllo");
    println!("boundary at 2? {}", accented.is_char_boundary(2));
    let third = accented.char_indices().nth(2).unwrap().0;
    accented.insert(third, '-');
    println!("{accented:?}");

    // Everything after the offset shifts, so this is O(n).
    let mut reversed = String::new();
    for c in "abcd".chars() {
        reversed.insert(0, c);
    }
    println!("{reversed:?}");

    // The linear way to do the same thing.
    println!("{:?}", "abcd".chars().rev().collect::<String>());
}
