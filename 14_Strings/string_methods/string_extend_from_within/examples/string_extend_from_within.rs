fn main() {
    let mut s = String::from("abc");
    s.extend_from_within(0..2);
    println!("{s:?}");

    // Doubling.
    let mut d = String::from("xy");
    d.extend_from_within(..);
    println!("{d:?}");

    // The version the borrow checker refuses, and its usual workaround:
    //     s.push_str(&s[0..2]);            // E0502
    let mut w = String::from("abc");
    let temp = w[0..2].to_string();
    w.push_str(&temp);
    println!("{w:?} (via a temporary)");

    // Byte offsets, with the usual boundary rule.
    let mut accented = String::from("héllo");
    println!("boundary at 2? {}", accented.is_char_boundary(2));
    accented.extend_from_within(0..3);
    println!("{accented:?}");
}
