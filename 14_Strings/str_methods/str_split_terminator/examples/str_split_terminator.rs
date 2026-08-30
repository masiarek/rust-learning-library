fn main() {
    for input in ["a,b,", ",a,b", "a,,b", ",", ""] {
        println!("{:<7}  split {:<20}  terminator {:?}",
                 format!("{input:?}"),
                 format!("{:?}", input.split(',').collect::<Vec<&str>>()),
                 input.split_terminator(',').collect::<Vec<&str>>());
    }

    // The everyday case: a file that ends in a newline.
    let file = "alpha\nbeta\ngamma\n";
    println!("split {}  terminator {}  lines {}",
             file.split('\n').count(),
             file.split_terminator('\n').count(),
             file.lines().count());
}
