fn main() {
    let nums = [1, 2, 3];
    println!("{} {}", nums.contains(&2), nums.contains(&7));

    // It takes a reference even for a Copy type, so the argument is &2, not 2.
    let wanted = 3;
    println!("{}", nums.contains(&wanted));

    // The Vec<String> trap: contains wants a &String, and a literal is a &str.
    let names = vec![String::from("Ann"), String::from("Bob")];
    // names.contains("Ann");                     // error[E0308]: expected `&String`, found `&str`
    println!("{}", names.contains(&"Ann".to_string()));   // allocates just to ask
    println!("{}", names.iter().any(|n| n == "Ann"));    // no allocation

    // A slice of &str compares with a literal directly.
    let tags = ["red", "green"];
    println!("{}", tags.contains(&"green"));

    // Linear: it looks at every element until it finds one.
    let big: Vec<u32> = (0..1_000).collect();
    println!("{}", big.contains(&999));
}
