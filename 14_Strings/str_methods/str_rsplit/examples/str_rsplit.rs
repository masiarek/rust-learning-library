fn main() {
    let s = "a,b,c";

    println!("{:?}", s.split(',').collect::<Vec<&str>>());
    println!("{:?}", s.rsplit(',').collect::<Vec<&str>>());

    // The everyday use: the last field, cheaply.
    println!("{:?}", s.rsplit(',').next());

    // Same pieces, so reversing gets reading order back.
    let mut back: Vec<&str> = s.rsplit(',').collect();
    back.reverse();
    println!("{:?}", back);
    println!("{}", back == s.split(',').collect::<Vec<&str>>());

    // Empty pieces behave exactly as they do forwards.
    println!("{:?}", "a,,c".rsplit(',').collect::<Vec<&str>>());
}
