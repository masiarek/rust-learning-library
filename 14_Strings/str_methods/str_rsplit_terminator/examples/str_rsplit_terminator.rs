fn main() {
    for input in ["a,b,", ",a,b", "a,,b"] {
        println!("{:<7}  fwd {:<20}  rev {:?}",
                 format!("{input:?}"),
                 format!("{:?}", input.split_terminator(',').collect::<Vec<&str>>()),
                 input.rsplit_terminator(',').collect::<Vec<&str>>());
    }

    // The dropped piece is the trailing one either way: here the LEADING
    // empty survives, and arrives last because the order is reversed.
    println!("{:?}", ",a".rsplit_terminator(',').collect::<Vec<&str>>());

    // The last real line of a newline-terminated file, cheaply.
    let file = "alpha\nbeta\ngamma\n";
    println!("{:?}", file.rsplit_terminator('\n').next());
}
