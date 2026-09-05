fn main() {
    let words = ["red", "green", "blue"];
    println!("{}", words.join(", "));
    println!("{}  <- join(\"\") is concat", words.join(""));
    let owned = vec![String::from("a"), String::from("b")];
    println!("{}", owned.join("-"));

    // Not only strings: slices of slices, with an element or a slice between.
    let rows = [vec![1, 2], vec![3, 4]];
    println!("{:?}", rows.join(&0));
    println!("{:?}", rows.join(&[0, 0][..]));

    // Numbers have no join — [1, 2, 3].join(",") is error[E0599]. Map to strings first.
    let nums = [1, 2, 3];
    let csv = nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
    println!("{csv}");

    // An empty slice joins to an empty string.
    let none: [&str; 0] = [];
    println!("{:?}", none.join(", "));
}
