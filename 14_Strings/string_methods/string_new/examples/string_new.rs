fn main() {
    let s = String::new();
    println!("len {} capacity {}", s.len(), s.capacity());

    // The heap is touched on the first push, not before.
    let mut grown = String::new();
    println!("before  len {} capacity {}", grown.len(), grown.capacity());
    grown.push('a');
    println!("after   len {} capacity {}", grown.len(), grown.capacity());

    // Three spellings of empty.
    println!("{} {}", String::new() == String::default(), String::new() == "".to_string());

    // const, so it can initialize a static.
    static EMPTY: String = String::new();
    println!("{:?} {}", EMPTY, EMPTY.is_empty());

    // Building up from nothing is the usual reason to start here.
    let mut out = String::new();
    for word in ["a", "b", "c"] {
        out.push_str(word);
    }
    println!("{out:?}");
}
