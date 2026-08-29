fn main() {
    for line in ["key=value", "key=a=b", "=v", "k=", "bare"] {
        println!("{line:<10?} -> {:?}", line.split_once('='));
    }

    // The miss is a None you have to handle.
    let parsed: Vec<(&str, &str)> = ["a=1", "bad", "b=2"]
        .iter()
        .filter_map(|l| l.split_once('='))
        .collect();
    println!("{parsed:?}");

    // No byte arithmetic, so a multi-byte delimiter is safe.
    println!("{:?}", "left→right".split_once('→'));

    // A &str delimiter works too.
    println!("{:?}", "one :: two :: three".split_once(" :: "));
}
