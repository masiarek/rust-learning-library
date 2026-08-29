fn main() {
    let s = "héllo";

    for mid in [0, 1, 2, 3, 5, 6, 99] {
        println!("{mid:>2} -> {:?}", s.split_at_checked(mid));
    }

    // Distinguishing the two refusals, when it matters.
    for mid in [2, 99] {
        let why = if mid > s.len() { "out of range" } else { "inside a character" };
        println!("{mid} refused: {why}");
    }

    // Truncating to a budget: floor the offset rather than refusing.
    let budget = 3;
    let cut = s.floor_char_boundary(budget);
    println!("{:?}", s.split_at(cut));
}
