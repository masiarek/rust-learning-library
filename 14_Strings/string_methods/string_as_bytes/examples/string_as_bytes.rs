fn main() {
    let s = String::from("héllo");
    println!("{:?}", s.as_bytes());
    println!("{} bytes, {} chars", s.as_bytes().len(), s.chars().count());

    // Identical to the str method, reached by deref.
    println!("{}", s.as_bytes() == s.as_str().as_bytes());

    // Borrowed: the String is still there afterwards.
    let bytes = s.as_bytes();
    println!("first {:?}, and s is still {:?}", bytes.first(), s);

    // The owned version, which consumes the String and copies nothing.
    let owned = String::from("héllo");
    let v: Vec<u8> = owned.into_bytes();
    println!("{v:?}");

    // Writing bytes out is the usual reason.
    println!("{}", String::from("GET /").as_bytes().starts_with(b"GET"));
}
