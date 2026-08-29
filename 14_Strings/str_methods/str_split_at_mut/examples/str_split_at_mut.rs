fn main() {
    let mut owned = String::from("hello world");

    let (left, right) = owned.as_mut_str().split_at_mut(5);
    left.make_ascii_uppercase();
    right.make_ascii_lowercase();
    println!("{owned}");

    // Two &mut into one string, which plain borrowing cannot express.
    let mut s = String::from("abcdef");
    let (a, b) = s.as_mut_str().split_at_mut(3);
    println!("{a:?} {b:?}");
    a.make_ascii_uppercase();
    println!("{s}");

    // The checked form refuses instead of panicking.
    let mut acc = String::from("héllo");
    println!("{:?}", acc.as_mut_str().split_at_mut_checked(2).is_none());
}
