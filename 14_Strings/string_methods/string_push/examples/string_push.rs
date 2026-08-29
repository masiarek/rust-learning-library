fn main() {
    let mut s = String::from("hi");
    s.push('!');
    println!("{s:?}");

    // A char is 1..=4 bytes, so len grows by more than one.
    let mut wide = String::new();
    for c in ['a', 'é', '👋'] {
        wide.push(c);
        println!("pushed {c:?} -> len {} (grew by {})", wide.len(), c.len_utf8());
    }

    // push takes a char; push_str takes text.
    let mut t = String::new();
    t.push('a');
    t.push_str("bc");
    println!("{t:?}");

    // Growth doubles, so a run of pushes is cheap on average.
    let mut grown = String::new();
    let mut capacity = grown.capacity();
    for i in 1..=9 {
        grown.push('x');
        if grown.capacity() != capacity {
            capacity = grown.capacity();
            println!("push {i} reallocated: len {} capacity {capacity}", grown.len());
        }
    }

    // collect is usually the better spelling.
    let shouted: String = "hi".chars().map(|c| c.to_ascii_uppercase()).collect();
    println!("{shouted:?}");
}
