fn main() {
    let mut owned = String::from("héllo");

    for mid in [1, 2, 3, 99] {
        // The borrow is released again on None, so the loop can keep asking.
        match owned.as_mut_str().split_at_mut_checked(mid) {
            Some((a, b)) => println!("{mid:>2} -> {a:?} {b:?}"),
            None => println!("{mid:>2} -> refused"),
        }
    }

    // Edit one half in place once a legal offset is found.
    if let Some((left, _)) = owned.as_mut_str().split_at_mut_checked(3) {
        left.make_ascii_uppercase();
    }
    println!("{owned}");
}
