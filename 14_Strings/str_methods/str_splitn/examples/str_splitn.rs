fn main() {
    let line = "key = a = b";

    println!("{:?}", line.split('=').collect::<Vec<&str>>());
    println!("{:?}", line.splitn(2, '=').collect::<Vec<&str>>());
    println!("{:?}", line.splitn(3, '=').collect::<Vec<&str>>());

    // Edge values of n.
    println!("{:?}", line.splitn(0, '=').collect::<Vec<&str>>());
    println!("{:?}", line.splitn(1, '=').collect::<Vec<&str>>());

    // n=2 is nearly always better written as split_once.
    println!("{:?}", line.split_once('='));
    println!("{:?}", "no delimiter".split_once('='));
    println!("{:?}", "no delimiter".splitn(2, '=').collect::<Vec<&str>>());
}
