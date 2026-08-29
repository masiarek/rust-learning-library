fn main() {
    println!("{:?}", "this is old, very old".replace("old", "new"));
    println!("{:?}", "a-b-c".replace('-', ""));
    println!("{:?}", "a b\tc".replace(char::is_whitespace, "_"));

    // The original is unchanged; a new String comes back.
    let original = "keep me";
    let changed = original.replace("keep", "drop");
    println!("{original:?} {changed:?}");

    // Chained replaces are sequential, not simultaneous.
    println!("{:?}", "ab".replace('a', "b").replace('b', "a"));

    // A single pass, which is what a swap actually needs.
    let swapped: String = "ab".chars()
        .map(|c| match c { 'a' => 'b', 'b' => 'a', other => other })
        .collect();
    println!("{swapped:?}");

    // It allocates even when nothing matches.
    let s = "no match here";
    println!("{}", s.replace("zzz", "!") == s);
}
