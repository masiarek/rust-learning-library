fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s:?}");

    // A &String works, via deref.
    let tail = String::from("!");
    s.push_str(&tail);
    println!("{s:?}");

    // One allocation for the whole build.
    let words = ["alpha", "beta", "gamma"];
    let mut out = String::with_capacity(20);
    for (i, w) in words.iter().enumerate() {
        if i > 0 { out.push_str(", "); }
        out.push_str(w);
    }
    println!("{out:?} len {} capacity {}", out.len(), out.capacity());

    // join does the same job for a slice.
    println!("{:?}", words.join(", "));

    // It returns (), so it does not chain.
    let mut t = String::new();
    let unit = t.push_str("x");
    println!("{t:?} returned {unit:?}");
}
