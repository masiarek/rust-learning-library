fn main() {
    let s = String::from("héllo");
    let cap = s.capacity();
    let v = s.into_bytes();
    println!("{v:?}");
    println!("capacity carried across: {}", v.capacity() == cap);

    // The round trip: one scan out, nothing copied either way.
    let back = String::from_utf8(v).unwrap();
    println!("{back:?}");

    // The Vec can now hold anything, valid UTF-8 or not.
    let mut bytes = String::from("hi").into_bytes();
    bytes.push(0xff);
    println!("{:?}", String::from_utf8(bytes.clone()).is_err());
    println!("{:?}", String::from_utf8_lossy(&bytes));

    // Borrowing instead, when the String should survive.
    let keep = String::from("hi");
    println!("{:?} and {keep:?}", keep.as_bytes());
}
