fn main() {
    println!("{:?}", "aaab".trim_start_matches('a'));
    println!("{:?}", "../../src".trim_start_matches("../"));
    println!("{:?}", "0042".trim_start_matches('0'));

    // Repetition vs one occurrence.
    println!("{:?}", "../../src".strip_prefix("../"));

    // No match is silent here, and reported there.
    println!("{:?}", "src".trim_start_matches("../"));
    println!("{:?}", "src".strip_prefix("../"));

    // The all-zeros edge case.
    for n in ["0042", "0", "000", "40"] {
        let t = n.trim_start_matches('0');
        println!("{n:<5?} -> {:?}  (as a number: {})", t, if t.is_empty() { "0" } else { t });
    }
}
