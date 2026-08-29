fn main() {
    let mut s = String::from("world");
    s.insert_str(0, "hello, ");
    println!("{s:?}");

    // Inserting in the middle, at an offset found by searching.
    let mut line = String::from("key=value");
    if let Some(i) = line.find('=') {
        line.insert_str(i, " ");
        line.insert_str(i + 2, " ");
    }
    println!("{line:?}");

    // Prepending always copies the rest -- there is no cheap version.
    let mut acc = String::new();
    for word in ["c", "b", "a"] {
        acc.insert_str(0, word);
    }
    println!("{acc:?}");

    // Replacing a range instead of inserting into one.
    let mut r = String::from("key=value");
    r.replace_range(3..4, " -> ");
    println!("{r:?}");
}
