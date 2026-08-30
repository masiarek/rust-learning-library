fn main() {
    println!("{:?}", "abbb".trim_end_matches('b'));
    println!("{:?}", "http://x/y///".trim_end_matches('/'));
    println!("{:?}", "a/b/".trim_end_matches("/"));

    // One occurrence, reported.
    println!("{:?}", "http://x/y///".strip_suffix('/'));

    // Trailing zeros: fine on a decimal, wrong on an integer.
    for n in ["1.500", "1.0", "10", "100"] {
        println!("{:<7} -> {:?}", format!("{n:?}"), n.trim_end_matches('0'));
    }

    // Which is why a decimal wants the dot handled too.
    let d = "1.500";
    println!("{:?}", d.trim_end_matches('0').trim_end_matches('.'));
}
