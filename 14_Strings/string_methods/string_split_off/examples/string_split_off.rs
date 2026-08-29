fn main() {
    let mut s = String::from("hello world");
    let tail = s.split_off(5);
    println!("head {s:?} tail {tail:?}");

    // Both halves are owned, unlike split_at's two borrows.
    println!("{:?}", "hello world".split_at(5));

    // The head keeps the buffer; the tail is a new allocation.
    let mut roomy = String::with_capacity(64);
    roomy.push_str("hello world");
    let cut = roomy.split_off(5);
    println!("head capacity {} / tail capacity {}", roomy.capacity(), cut.capacity());

    // split_off(0) takes the contents out through a &mut.
    let mut source = String::from("everything");
    let taken = source.split_off(0);
    println!("taken {taken:?}, source {source:?}");

    // Byte offsets, with the usual boundary rule.
    let wide = String::from("héllo");
    println!("boundary at 2? {}", wide.is_char_boundary(2));
    let mut copy = wide.clone();
    println!("{:?}", copy.split_off(3));
}
