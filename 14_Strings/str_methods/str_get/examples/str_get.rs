fn main() {
    let s = "héllo";

    println!("{:?}", s.get(0..1));
    println!("{:?}", s.get(0..2));     // None: byte 2 is inside 'é'
    println!("{:?}", s.get(0..99));    // None: out of bounds
    println!("{:?}", s.get(..));
    println!("{:?}", s.get(3..));

    // Every range form works.
    println!("{:?} {:?} {:?}", s.get(..3), s.get(1..3), s.get(1..=2));

    // The panicking equivalents, side by side.
    println!("{:?}", &s[0..1]);
    println!("panics instead: {}", s.get(0..2).is_none());

    // There is no s.get(0) -> char. These are the two real questions.
    println!("{:?}", s.chars().next());
    println!("{:?}", s.as_bytes().first());
}
