fn main() {
    let s = "the rain in spain";

    println!("{:?}", s.matches("in").collect::<Vec<&str>>());
    println!("{} matches, {} pieces", s.matches("in").count(), s.split("in").count());

    // Matches do not overlap.
    println!("{:?}", "aaaa".matches("aa").collect::<Vec<&str>>());

    // A predicate pattern reports which character matched.
    println!("{:?}", s.matches(|c: char| "aeiou".contains(c)).collect::<Vec<&str>>());

    // A set of characters.
    println!("{:?}", "a1b2c3".matches(char::is_numeric).collect::<Vec<&str>>());

    // Counting occurrences is the everyday use.
    println!("{}", "banana".matches('a').count());
}
