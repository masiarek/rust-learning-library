fn main() {
    let s = "one,two,three,four";

    println!("{:?}", s.matches(',').collect::<Vec<&str>>());
    println!("{:?}", s.rmatches(',').count());

    // Order is reversed.
    let words = "a1b22c333";
    println!("{:?}", words.matches(char::is_numeric).collect::<Vec<&str>>());
    println!("{:?}", words.rmatches(char::is_numeric).collect::<Vec<&str>>());

    // Overlap resolution starts from the other end: the same number of
    // matches, but at different offsets, and a different leftover 'a'.
    println!("{:?}", "aaaaa".match_indices("aa").collect::<Vec<(usize, &str)>>());
    println!("{:?}", "aaaaa".rmatch_indices("aa").collect::<Vec<(usize, &str)>>());

    // The last two matches, without walking the whole string first.
    println!("{:?}", s.rmatches(char::is_alphabetic).take(2).collect::<Vec<&str>>());
}
