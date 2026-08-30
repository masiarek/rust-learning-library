fn main() {
    for input in ["a\nb\n", "a\nb", "", "\n"] {
        println!("{:<8} -> {:?}", format!("{input:?}"), input.split_inclusive('\n').collect::<Vec<&str>>());
    }

    // Nothing is discarded, so the pieces rebuild the original.
    let text = "alpha\r\nbeta\ngamma";
    let pieces: Vec<&str> = text.split_inclusive('\n').collect();
    println!("{pieces:?}");
    println!("round trip: {}", pieces.concat() == text);

    // lines() would have thrown the endings away.
    println!("{:?}", text.lines().collect::<Vec<&str>>());

    // Telling a terminated last line from an unterminated one.
    for input in ["a\nb\n", "a\nb"] {
        let last = input.split_inclusive('\n').last().unwrap();
        println!("{:<8} last piece {last:?} terminated={}", format!("{input:?}"), last.ends_with('\n'));
    }
}
