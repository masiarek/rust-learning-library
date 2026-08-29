fn main() {
    let s = "one,two,three";

    println!("{:?}", s.match_indices(',').collect::<Vec<(usize, &str)>>());
    println!("{:?}", s.rmatch_indices(',').collect::<Vec<(usize, &str)>>());

    // Editing from the back keeps the unused offsets valid.
    let mut owned = String::from(s);
    for (i, m) in s.rmatch_indices(',') {
        owned.replace_range(i..i + m.len(), " | ");
    }
    println!("{owned}");

    // The last comma, without collecting everything.
    println!("{:?}", s.rmatch_indices(',').next());
}
