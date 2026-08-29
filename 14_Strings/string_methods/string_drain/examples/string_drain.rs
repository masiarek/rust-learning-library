fn main() {
    let mut s = String::from("hello world");
    let taken: String = s.drain(0..6).collect();
    println!("took {taken:?}, left {s:?}");

    // Dropping the Drain removes the range even if nothing is consumed.
    let mut t = String::from("prefix:value");
    t.drain(..7);
    println!("{t:?}");

    // drain(..) empties the string but keeps the buffer.
    let mut all = String::with_capacity(32);
    all.push_str("abc");
    let chars: Vec<char> = all.drain(..).collect();
    println!("{chars:?} left {all:?} capacity {}", all.capacity());

    // Byte offsets, with the usual boundary rule.
    let mut wide = String::from("héllo");
    println!("boundary at 2? {}", wide.is_char_boundary(2));
    let head: String = wide.drain(..3).collect();
    println!("{head:?} + {wide:?}");
}
