fn main() {
    for input in ["a,b,c", "a,,c", ",a", "a,", ""] {
        println!("{input:<7?} -> {:?}", input.split(',').collect::<Vec<&str>>());
    }

    // n matches, n+1 pieces — always.
    let s = "the rain in spain";
    println!("{} matches, {} pieces", s.matches("in").count(), s.split("in").count());

    // The four pattern shapes.
    println!("{:?}", "a1b2c".split(char::is_numeric).collect::<Vec<&str>>());
    println!("{:?}", "a-b_c".split(&['-', '_'][..]).collect::<Vec<&str>>());

    // Dropping empties is your decision, not the method's.
    let row = "a,,c";
    println!("{:?}", row.split(',').filter(|p| !p.is_empty()).collect::<Vec<&str>>());
    println!("columns: {} kept, {} after filtering",
             row.split(',').count(),
             row.split(',').filter(|p| !p.is_empty()).count());
}
